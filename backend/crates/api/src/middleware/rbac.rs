//! RBAC (Role-Based Access Control) middleware
//!
//! Provides role-based access control for tenant resources.
//! Role hierarchy: owner > admin > member

use actix_web::{dev::ServiceRequest, http::header, Error, HttpMessage};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};

use domain::user::TenantRole;
use service_auth::jwt::Claims;

/// JWT token extractor and validator
pub fn extract_token(req: &ServiceRequest) -> Option<String> {
    // Try Authorization header first
    if let Some(auth_header) = req.headers().get(header::AUTHORIZATION) {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                return Some(auth_str[7..].to_string());
            }
        }
    }

    // Try X-API-Key header for API access
    if let Some(api_key) = req.headers().get("X-API-Key") {
        if let Ok(key_str) = api_key.to_str() {
            return Some(key_str.to_string());
        }
    }

    None
}

/// Extract and validate JWT claims from request
pub fn extract_claims(req: &ServiceRequest, secret: &str) -> Option<Claims> {
    let token = extract_token(req)?;

    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .ok()?;

    Some(token_data.claims)
}

/// Current user context extracted from JWT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentUser {
    pub user_id: uuid::Uuid,
    pub role: String,
    pub tenant_id: uuid::Uuid,
    pub tenant_role: TenantRole,
}

impl CurrentUser {
    /// Check if user has required role or higher
    /// Role hierarchy: owner > admin > member
    pub fn has_role(&self, required: TenantRole) -> bool {
        let role_level = match self.tenant_role {
            TenantRole::Owner => 3,
            TenantRole::Admin => 2,
            TenantRole::Member => 1,
        };

        let required_level = match required {
            TenantRole::Owner => 3,
            TenantRole::Admin => 2,
            TenantRole::Member => 1,
        };

        role_level >= required_level
    }

    /// Check if user is owner
    pub fn is_owner(&self) -> bool {
        matches!(self.tenant_role, TenantRole::Owner)
    }

    /// Check if user is admin or higher
    pub fn is_admin(&self) -> bool {
        matches!(self.tenant_role, TenantRole::Owner | TenantRole::Admin)
    }
}

/// RBAC error types
#[derive(Debug, thiserror::Error)]
pub enum RbacError {
    #[error("Authentication required")]
    Unauthorized,
    #[error("Insufficient permissions")]
    Forbidden,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Tenant mismatch")]
    TenantMismatch,
}

impl actix_web::ResponseError for RbacError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        use crate::dto::common::ApiResponse;
        use actix_web::http::StatusCode;

        let (code, status) = match self {
            RbacError::Unauthorized => ("UNAUTHORIZED", StatusCode::UNAUTHORIZED),
            RbacError::Forbidden => ("FORBIDDEN", StatusCode::FORBIDDEN),
            RbacError::InvalidToken => ("INVALID_TOKEN", StatusCode::UNAUTHORIZED),
            RbacError::TenantMismatch => ("TENANT_MISMATCH", StatusCode::FORBIDDEN),
        };

        actix_web::HttpResponse::build(status)
            .json(ApiResponse::<()>::error(code, &self.to_string()))
    }
}

/// Extension trait to get current user from request
pub trait CurrentUserExt {
    fn get_current_user(&self) -> Option<CurrentUser>;
    fn require_auth(&self) -> Result<CurrentUser, RbacError>;
}

impl CurrentUserExt for actix_web::HttpRequest {
    fn get_current_user(&self) -> Option<CurrentUser> {
        self.extensions().get::<CurrentUser>().cloned()
    }

    fn require_auth(&self) -> Result<CurrentUser, RbacError> {
        self.get_current_user().ok_or(RbacError::Unauthorized)
    }
}

/// Check if user has access to tenant resource
pub fn check_tenant_access(
    user: &CurrentUser,
    tenant_id: uuid::Uuid,
    required_role: TenantRole,
) -> Result<(), RbacError> {
    // Check tenant match
    if user.tenant_id != tenant_id {
        return Err(RbacError::TenantMismatch);
    }

    // Check role
    if !user.has_role(required_role) {
        return Err(RbacError::Forbidden);
    }

    Ok(())
}

/// RBAC middleware configuration
#[derive(Clone)]
pub struct RbacMiddleware {
    pub jwt_secret: String,
    pub required_role: Option<TenantRole>,
}

impl RbacMiddleware {
    pub fn new(jwt_secret: String) -> Self {
        Self {
            jwt_secret,
            required_role: None,
        }
    }

    pub fn require_role(mut self, role: TenantRole) -> Self {
        self.required_role = Some(role);
        self
    }
}

impl<S> actix_web::dev::Transform<S, actix_web::dev::ServiceRequest> for RbacMiddleware
where
    S: actix_web::dev::Service<
        actix_web::dev::ServiceRequest,
        Response = actix_web::dev::ServiceResponse<actix_web::body::BoxBody>,
        Error = Error,
    >,
    S::Future: 'static,
{
    type Response = actix_web::dev::ServiceResponse<actix_web::body::BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = RbacMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RbacMiddlewareService {
            service,
            jwt_secret: self.jwt_secret.clone(),
            required_role: self.required_role.clone(),
        }))
    }
}

pub struct RbacMiddlewareService<S> {
    service: S,
    jwt_secret: String,
    required_role: Option<TenantRole>,
}

impl<S> actix_web::dev::Service<actix_web::dev::ServiceRequest> for RbacMiddlewareService<S>
where
    S: actix_web::dev::Service<
        actix_web::dev::ServiceRequest,
        Response = actix_web::dev::ServiceResponse<actix_web::body::BoxBody>,
        Error = Error,
    >,
    S::Future: 'static,
{
    type Response = actix_web::dev::ServiceResponse<actix_web::body::BoxBody>;
    type Error = Error;
    type Future =
        std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: actix_web::dev::ServiceRequest) -> Self::Future {
        let jwt_secret = self.jwt_secret.clone();
        let required_role = self.required_role.clone();

        // Extract claims from token
        let current_user = extract_claims(&req, &jwt_secret).and_then(|claims| {
            let tenant_role = match claims.tenant_role.as_str() {
                "owner" => TenantRole::Owner,
                "admin" => TenantRole::Admin,
                _ => TenantRole::Member,
            };

            // Parse UUIDs from string
            let user_id = uuid::Uuid::parse_str(&claims.sub).ok()?;
            let tenant_id = uuid::Uuid::parse_str(&claims.tenant_id).ok()?;

            Some(CurrentUser {
                user_id,
                role: claims.role,
                tenant_id,
                tenant_role,
            })
        });

        // Check if authentication is required
        if let Some(user) = current_user {
            // Check role requirement
            if let Some(required) = required_role {
                if !user.has_role(required) {
                    return Box::pin(
                        async move { Err(actix_web::Error::from(RbacError::Forbidden)) },
                    );
                }
            }

            // Insert user context into request extensions
            req.extensions_mut().insert(user);

            let fut = self.service.call(req);
            Box::pin(async move { fut.await })
        } else {
            // No valid token found
            Box::pin(async move { Err(actix_web::Error::from(RbacError::Unauthorized)) })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_hierarchy() {
        let owner = CurrentUser {
            user_id: uuid::Uuid::new_v4(),
            role: "user".to_string(),
            tenant_id: uuid::Uuid::new_v4(),
            tenant_role: TenantRole::Owner,
        };

        let admin = CurrentUser {
            user_id: uuid::Uuid::new_v4(),
            role: "user".to_string(),
            tenant_id: uuid::Uuid::new_v4(),
            tenant_role: TenantRole::Admin,
        };

        let member = CurrentUser {
            user_id: uuid::Uuid::new_v4(),
            role: "user".to_string(),
            tenant_id: uuid::Uuid::new_v4(),
            tenant_role: TenantRole::Member,
        };

        // Owner can do everything
        assert!(owner.has_role(TenantRole::Owner));
        assert!(owner.has_role(TenantRole::Admin));
        assert!(owner.has_role(TenantRole::Member));

        // Admin can do admin and member things
        assert!(!admin.has_role(TenantRole::Owner));
        assert!(admin.has_role(TenantRole::Admin));
        assert!(admin.has_role(TenantRole::Member));

        // Member can only do member things
        assert!(!member.has_role(TenantRole::Owner));
        assert!(!member.has_role(TenantRole::Admin));
        assert!(member.has_role(TenantRole::Member));
    }

    #[test]
    fn test_current_user_is_owner() {
        let user = CurrentUser {
            user_id: uuid::Uuid::new_v4(),
            role: "user".to_string(),
            tenant_id: uuid::Uuid::new_v4(),
            tenant_role: TenantRole::Owner,
        };

        assert!(user.is_owner());
        assert!(user.is_admin());
    }

    #[test]
    fn test_current_user_is_admin() {
        let user = CurrentUser {
            user_id: uuid::Uuid::new_v4(),
            role: "user".to_string(),
            tenant_id: uuid::Uuid::new_v4(),
            tenant_role: TenantRole::Admin,
        };

        assert!(!user.is_owner());
        assert!(user.is_admin());
    }

    #[test]
    fn test_current_user_is_member() {
        let user = CurrentUser {
            user_id: uuid::Uuid::new_v4(),
            role: "user".to_string(),
            tenant_id: uuid::Uuid::new_v4(),
            tenant_role: TenantRole::Member,
        };

        assert!(!user.is_owner());
        assert!(!user.is_admin());
    }

    #[test]
    fn test_check_tenant_access_owner() {
        let user = CurrentUser {
            user_id: uuid::Uuid::new_v4(),
            role: "user".to_string(),
            tenant_id: uuid::Uuid::new_v4(),
            tenant_role: TenantRole::Owner,
        };

        // Owner can access owner-level resources
        let result = check_tenant_access(&user, user.tenant_id, TenantRole::Owner);
        assert!(result.is_ok());

        // Owner can access admin-level resources
        let result = check_tenant_access(&user, user.tenant_id, TenantRole::Admin);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_tenant_access_denied_for_wrong_tenant() {
        let user = CurrentUser {
            user_id: uuid::Uuid::new_v4(),
            role: "user".to_string(),
            tenant_id: uuid::Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap(),
            tenant_role: TenantRole::Owner,
        };

        let other_tenant = uuid::Uuid::parse_str("22222222-2222-2222-2222-222222222222").unwrap();
        let result = check_tenant_access(&user, other_tenant, TenantRole::Member);
        assert!(result.is_err());
    }

    #[test]
    fn test_check_tenant_access_denied_for_insufficient_role() {
        let user = CurrentUser {
            user_id: uuid::Uuid::new_v4(),
            role: "user".to_string(),
            tenant_id: uuid::Uuid::new_v4(),
            tenant_role: TenantRole::Member,
        };

        // Member cannot access owner-level resources
        let result = check_tenant_access(&user, user.tenant_id, TenantRole::Owner);
        assert!(result.is_err());

        // Member cannot access admin-level resources
        let result = check_tenant_access(&user, user.tenant_id, TenantRole::Admin);
        assert!(result.is_err());
    }
}
