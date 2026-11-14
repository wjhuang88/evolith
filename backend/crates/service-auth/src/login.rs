//! Transport-independent login orchestration shared by REST and GraphQL.
//!
//! Password checks and JWT issuance are owned here. HTTP cookies and response codes
//! remain the responsibility of each transport adapter.

use std::sync::Arc;

use domain::repository::{TenantRepository, UserRepository};
use domain::user::{TenantRole, UserRole};
use thiserror::Error;

use crate::{Argon2Hasher, JwtHandler};

#[derive(Debug, Error)]
pub enum LoginError {
    #[error("Invalid email or password")]
    InvalidCredentials,
    #[error("Login failed")]
    Internal,
}

/// Never expose the domain User entity: it includes password and recovery secrets.
#[derive(Debug, Clone)]
pub struct LoginUser {
    pub id: String,
    pub email: String,
    pub username: String,
    pub role: String,
    pub tenant_id: String,
    pub tenant_role: String,
    pub email_verified: bool,
}

#[derive(Debug, Clone)]
pub struct LoginTenant {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub plan: String,
}

#[derive(Debug, Clone)]
pub struct LoginOutcome {
    pub token: String,
    pub expires_at: i64,
    pub user: LoginUser,
    pub tenant: LoginTenant,
}

/// Auth business boundary; no Actix types, headers, cookies or GraphQL types.
#[derive(Clone)]
pub struct LoginService {
    user_repo: Arc<dyn UserRepository>,
    tenant_repo: Arc<dyn TenantRepository>,
    hasher: Argon2Hasher,
    jwt: JwtHandler,
}

impl LoginService {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        tenant_repo: Arc<dyn TenantRepository>,
        hasher: Argon2Hasher,
        jwt: JwtHandler,
    ) -> Self {
        Self {
            user_repo,
            tenant_repo,
            hasher,
            jwt,
        }
    }

    pub async fn authenticate(
        &self,
        email: &str,
        password: &str,
    ) -> Result<LoginOutcome, LoginError> {
        let user = match self.user_repo.find_by_email(email).await {
            Ok(Some(user)) => user,
            Ok(None) => return Err(LoginError::InvalidCredentials),
            Err(error) => {
                tracing::error!("Failed to find user by email: {}", error);
                return Err(LoginError::Internal);
            }
        };

        match self.hasher.verify_password(password, &user.password_hash) {
            Ok(true) => {}
            Ok(false) => return Err(LoginError::InvalidCredentials),
            Err(error) => {
                tracing::error!("Password verification error: {}", error);
                return Err(LoginError::Internal);
            }
        }

        let role = match user.role {
            UserRole::Admin => "admin",
            UserRole::User => "user",
        }
        .to_owned();
        let tenant_role = match user.tenant_role {
            TenantRole::Owner => "owner",
            TenantRole::Admin => "admin",
            TenantRole::Member => "member",
        }
        .to_owned();

        let (token, expires_at) = self
            .jwt
            .generate_token(user.id, &role, user.tenant_id, &tenant_role)
            .map_err(|error| {
                tracing::error!("Failed to generate token: {}", error);
                LoginError::Internal
            })?;

        // Keep the established REST fallback for an absent or unreadable tenant row.
        let tenant = match self.tenant_repo.find_by_id(user.tenant_id).await {
            Ok(Some(tenant)) => LoginTenant {
                id: tenant.id.to_string(),
                name: tenant.name,
                slug: tenant.slug,
                plan: format!("{}", tenant.plan),
            },
            _ => LoginTenant {
                id: user.tenant_id.to_string(),
                name: "Unknown".to_owned(),
                slug: "unknown".to_owned(),
                plan: "free".to_owned(),
            },
        };

        Ok(LoginOutcome {
            token,
            expires_at,
            user: LoginUser {
                id: user.id.to_string(),
                email: user.email,
                username: user.username,
                role,
                tenant_id: user.tenant_id.to_string(),
                tenant_role,
                email_verified: user.email_verified,
            },
            tenant,
        })
    }
}
