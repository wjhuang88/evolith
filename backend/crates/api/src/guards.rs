//! Permission guards for handlers

use actix_web::{dev::ServiceRequest, http::header, Error, HttpRequest, HttpResponse};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

pub use crate::middleware::rbac::{check_tenant_access, CurrentUser, CurrentUserExt, RbacError};
pub use actix_web::ResponseError;
use domain::user::TenantRole;

// Re-export Claims from jwt service
use service_auth::jwt::Claims as JwtClaims;

/// JWT secret - would typically come from config
fn get_jwt_secret() -> String {
    std::env::var("JWT__SECRET").unwrap_or_else(|_| "dev_secret_key_for_testing_only".to_string())
}

/// Extract CurrentUser from request - tries extensions first, then extracts from JWT
fn extract_current_user(req: &HttpRequest) -> Option<CurrentUser> {
    // First try to get from request extensions (set by middleware)
    if let Some(user) = req.get_current_user() {
        return Some(user);
    }

    // If not in extensions, extract from JWT directly
    extract_jwt_user(req).ok()
}

/// Extract user from JWT token in Authorization header
fn extract_jwt_user(req: &HttpRequest) -> Result<CurrentUser, RbacError> {
    // Get token from Authorization header
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or(RbacError::InvalidToken)?;

    let jwt_secret = get_jwt_secret();
    let validation = Validation::new(Algorithm::HS256);

    let token_data = decode::<JwtClaims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &validation,
    )
    .map_err(|_| RbacError::InvalidToken)?;

    let claims = token_data.claims;

    let tenant_role = match claims.tenant_role.as_str() {
        "owner" => TenantRole::Owner,
        "admin" => TenantRole::Admin,
        _ => TenantRole::Member,
    };

    let user_id = uuid::Uuid::parse_str(&claims.sub).map_err(|_| RbacError::InvalidToken)?;
    let tenant_id =
        uuid::Uuid::parse_str(&claims.tenant_id).map_err(|_| RbacError::InvalidToken)?;

    Ok(CurrentUser {
        user_id,
        role: claims.role,
        tenant_id,
        tenant_role,
    })
}

/// Require authentication - returns current user or error
pub fn require_auth(req: &HttpRequest) -> Result<CurrentUser, RbacError> {
    // First try the extension method (from middleware)
    if let Some(user) = req.get_current_user() {
        return Ok(user);
    }

    // Fall back to extracting from JWT directly
    extract_jwt_user(req)
}

/// Require specific tenant role
pub fn require_role(user: &CurrentUser, role: TenantRole) -> Result<(), RbacError> {
    if user.has_role(role) {
        Ok(())
    } else {
        Err(RbacError::Forbidden)
    }
}

/// Require admin or higher role
pub fn require_admin(user: &CurrentUser) -> Result<(), RbacError> {
    if user.is_admin() {
        Ok(())
    } else {
        Err(RbacError::Forbidden)
    }
}

/// Require owner role
pub fn require_owner(user: &CurrentUser) -> Result<(), RbacError> {
    if user.is_owner() {
        Ok(())
    } else {
        Err(RbacError::Forbidden)
    }
}

/// Require access to specific tenant
pub fn require_tenant_access(user: &CurrentUser, tenant_id: uuid::Uuid) -> Result<(), RbacError> {
    if user.tenant_id == tenant_id {
        Ok(())
    } else {
        Err(RbacError::TenantMismatch)
    }
}

/// Combined guard: require auth + tenant access + optional role
pub fn require_access(
    req: &HttpRequest,
    tenant_id: uuid::Uuid,
    min_role: Option<TenantRole>,
) -> Result<CurrentUser, RbacError> {
    let user = require_auth(req)?;
    require_tenant_access(&user, tenant_id)?;

    if let Some(role) = min_role {
        require_role(&user, role)?;
    }

    Ok(user)
}

/// Guard for tenant-scoped routes
pub fn guard_tenant_route(
    req: &HttpRequest,
    tenant_id: &uuid::Uuid,
    min_role: Option<TenantRole>,
) -> Result<CurrentUser, RbacError> {
    require_access(req, *tenant_id, min_role)
}
