//! RBAC (Role-Based Access Control) middleware
//!
//! Provides role-based access control for tenant resources.
//! Role hierarchy: owner > admin > member

use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    error::InternalError,
    http::header,
    middleware::Next,
    web, Error, HttpMessage,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use domain::api_key::{ApiKey, ApiKeyStatus};
use domain::repository::ApiKeyRepository;
use domain::user::TenantRole;
use service_auth::jwt::Claims;
use sha2::{Digest, Sha256};

/// JWT cookie name
pub const JWT_COOKIE_NAME: &str = "evolith_token";

/// Extract token from request (Bearer, Basic, X-API-Key, or cookie).
/// For Basic-Auth, returns Some(("basic", api_key)) to signal API key resolution needed.
/// For other methods, returns Some(("token", token_string)).
pub fn extract_token(req: &ServiceRequest) -> Option<(&'static str, String)> {
    if let Some(auth_header) = req.headers().get(header::AUTHORIZATION) {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(stripped) = auth_str.strip_prefix("Bearer ") {
                return Some(("token", stripped.to_string()));
            }
            if let Some(encoded) = auth_str.strip_prefix("Basic ") {
                if let Ok(decoded) = base64_decode(encoded.trim()) {
                    if let Some(colon_pos) = decoded.find(':') {
                        let api_key = decoded[colon_pos + 1..].to_string();
                        return Some(("basic", api_key));
                    }
                }
            }
        }
    }

    if let Some(api_key) = req.headers().get("X-API-Key") {
        if let Ok(key_str) = api_key.to_str() {
            return Some(("basic", key_str.to_string()));
        }
    }

    if let Some(cookie) = req.cookie(JWT_COOKIE_NAME) {
        return Some(("token", cookie.value().to_string()));
    }

    None
}

fn base64_decode(input: &str) -> Result<String, ()> {
    use base64::Engine;
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(input)
        .map_err(|_| ())?;
    String::from_utf8(decoded).map_err(|_| ())
}

pub async fn resolve_api_key(key: &str, repo: &dyn ApiKeyRepository) -> Result<ApiKey, RbacError> {
    let parts: Vec<&str> = key.split('_').collect();
    if parts.len() < 3 || parts[0] != "evo" || parts[1] != "sk" {
        return Err(RbacError::Unauthorized);
    }

    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    let key_hash = hex::encode(hasher.finalize());

    let api_key = repo
        .find_by_key(&key_hash)
        .await
        .map_err(|_| RbacError::Unauthorized)?
        .ok_or(RbacError::Unauthorized)?;

    if api_key.status != ApiKeyStatus::Active {
        return Err(RbacError::Unauthorized);
    }

    if let Some(expires_at) = api_key.expires_at {
        use chrono::Utc;
        if expires_at < Utc::now() {
            return Err(RbacError::Unauthorized);
        }
    }

    Ok(api_key)
}

pub fn extract_claims(req: &ServiceRequest, secret: &str) -> Option<Claims> {
    let (kind, token) = extract_token(req)?;
    if kind != "token" {
        return None;
    }

    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .ok()?;

    Some(token_data.claims)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentUser {
    pub user_id: uuid::Uuid,
    pub role: String,
    pub tenant_id: uuid::Uuid,
    pub tenant_role: TenantRole,
}

impl CurrentUser {
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

    pub fn is_owner(&self) -> bool {
        matches!(self.tenant_role, TenantRole::Owner)
    }

    pub fn is_admin(&self) -> bool {
        matches!(self.tenant_role, TenantRole::Owner | TenantRole::Admin)
    }
}

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

pub fn check_tenant_access(
    user: &CurrentUser,
    tenant_id: uuid::Uuid,
    required_role: TenantRole,
) -> Result<(), RbacError> {
    if user.tenant_id != tenant_id {
        return Err(RbacError::TenantMismatch);
    }
    if !user.has_role(required_role) {
        return Err(RbacError::Forbidden);
    }
    Ok(())
}

#[derive(Clone)]
pub struct RbacMiddleware {
    pub jwt_secret: String,
    pub required_role: Option<TenantRole>,
    pub public_paths: Vec<String>,
    pub api_key_repo: Option<Arc<dyn ApiKeyRepository>>,
}

impl RbacMiddleware {
    pub fn new(jwt_secret: String) -> Self {
        Self {
            jwt_secret,
            required_role: None,
            public_paths: vec![
                "/health".to_string(),
                "/mcp".to_string(),
                "/api/v1/auth/register".to_string(),
                "/api/v1/auth/login".to_string(),
                "/api/v1/auth/forgot-password".to_string(),
                "/api/v1/auth/reset-password".to_string(),
                "/api/v1/auth/send-verify".to_string(),
                "/api/v1/auth/verify-email".to_string(),
                "/api/v1/invitations/accept".to_string(),
            ],
            api_key_repo: None,
        }
    }

    pub fn with_api_key_repo(mut self, repo: Arc<dyn ApiKeyRepository>) -> Self {
        self.api_key_repo = Some(repo);
        self
    }

    pub fn require_role(mut self, role: TenantRole) -> Self {
        self.required_role = Some(role);
        self
    }

    pub fn add_public_path(mut self, path: &str) -> Self {
        self.public_paths.push(path.to_string());
        self
    }

    pub fn is_public_path(path: &str, public_paths: &[String]) -> bool {
        public_paths.iter().any(|p| path.starts_with(p))
            || (path.starts_with("/api/v1/tenant/") && path.ends_with("/members/join"))
            || (!path.starts_with("/api/")
                && !path.starts_with("/mcp")
                && !path.starts_with("/repos/"))
    }
}

pub fn default_public_paths() -> Vec<String> {
    vec![
        "/health".to_string(),
        "/mcp".to_string(),
        "/api/v1/auth/register".to_string(),
        "/api/v1/auth/login".to_string(),
        "/api/v1/auth/forgot-password".to_string(),
        "/api/v1/auth/reset-password".to_string(),
        "/api/v1/auth/send-verify".to_string(),
        "/api/v1/auth/verify-email".to_string(),
        "/api/v1/invitations/accept".to_string(),
    ]
}

/// Build an Unauthorized error. For git Smart HTTP paths (`/repos/`), the 401 carries
/// `WWW-Authenticate: Basic` so standard git clients retry with the URL credentials
/// (git does not send Basic creds proactively without a challenge).
fn unauthorized_response(path: &str) -> Error {
    if path.starts_with("/repos/") {
        Error::from(InternalError::from_response(
            RbacError::Unauthorized,
            actix_web::HttpResponse::Unauthorized()
                .append_header(("WWW-Authenticate", "Basic realm=\"evolith\""))
                .finish(),
        ))
    } else {
        Error::from(RbacError::Unauthorized)
    }
}

/// Global RBAC auth middleware.
///
/// `from_fn` passes the inner service as `next: Next<B>` (in scope across `.await`), so the
/// async API-key lookup can run before `next.call(req)` with NO `Mutex` and NO `unsafe`.
/// Do not regress to `UnsafeCell` (UB under concurrency) or `Mutex` (serializes the server).
pub async fn rbac_middleware<B>(
    req: ServiceRequest,
    next: Next<B>,
) -> Result<ServiceResponse<B>, Error> {
    let path = req.path().to_string();
    if RbacMiddleware::is_public_path(&path, &default_public_paths()) {
        return next.call(req).await;
    }

    let (jwt_secret, api_key_repo) = match req.app_data::<web::Data<crate::state::AppState>>() {
        Some(state) => (state.config.jwt.secret.clone(), state.api_key_repo.clone()),
        None => return Err(unauthorized_response(&path)),
    };

    let (kind, token) = match extract_token(&req) {
        Some(t) => t,
        None => return Err(unauthorized_response(&path)),
    };

    if kind == "token" {
        let validation = Validation::new(Algorithm::HS256);
        let current_user = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &validation,
        )
        .ok()
        .and_then(|token_data| {
            let claims = token_data.claims;
            let tenant_role = match claims.tenant_role.as_str() {
                "owner" => TenantRole::Owner,
                "admin" => TenantRole::Admin,
                _ => TenantRole::Member,
            };
            let user_id = uuid::Uuid::parse_str(&claims.sub).ok()?;
            let tenant_id = uuid::Uuid::parse_str(&claims.tenant_id).ok()?;
            Some(CurrentUser {
                user_id,
                role: claims.role,
                tenant_id,
                tenant_role,
            })
        });

        if let Some(user) = current_user {
            req.extensions_mut().insert(user);
            return next.call(req).await;
        }
        return Err(unauthorized_response(&path));
    }

    let api_key = resolve_api_key(&token, &*api_key_repo)
        .await
        .map_err(|_| unauthorized_response(&path))?;
    let user = CurrentUser {
        user_id: api_key.user_id,
        role: "api_key".to_string(),
        tenant_id: api_key.tenant_id,
        tenant_role: TenantRole::Member,
    };
    req.extensions_mut().insert(api_key);
    req.extensions_mut().insert(user);
    next.call(req).await
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

        assert!(owner.has_role(TenantRole::Owner));
        assert!(owner.has_role(TenantRole::Admin));
        assert!(owner.has_role(TenantRole::Member));
        assert!(!admin.has_role(TenantRole::Owner));
        assert!(admin.has_role(TenantRole::Admin));
        assert!(admin.has_role(TenantRole::Member));
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
        assert!(check_tenant_access(&user, user.tenant_id, TenantRole::Owner).is_ok());
        assert!(check_tenant_access(&user, user.tenant_id, TenantRole::Admin).is_ok());
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
        assert!(check_tenant_access(&user, other_tenant, TenantRole::Member).is_err());
    }

    #[test]
    fn test_check_tenant_access_denied_for_insufficient_role() {
        let user = CurrentUser {
            user_id: uuid::Uuid::new_v4(),
            role: "user".to_string(),
            tenant_id: uuid::Uuid::new_v4(),
            tenant_role: TenantRole::Member,
        };
        assert!(check_tenant_access(&user, user.tenant_id, TenantRole::Owner).is_err());
        assert!(check_tenant_access(&user, user.tenant_id, TenantRole::Admin).is_err());
    }

    #[test]
    fn test_public_invitation_accept_paths() {
        let middleware = RbacMiddleware::new("test-secret".to_string());
        assert!(RbacMiddleware::is_public_path(
            "/api/v1/invitations/accept",
            &middleware.public_paths
        ));
        assert!(RbacMiddleware::is_public_path(
            "/api/v1/tenant/11111111-1111-1111-1111-111111111111/members/join",
            &middleware.public_paths
        ));
        assert!(!RbacMiddleware::is_public_path(
            "/api/v1/tenant/11111111-1111-1111-1111-111111111111/members",
            &middleware.public_paths
        ));
    }

    #[test]
    fn test_repos_paths_require_auth() {
        let middleware = RbacMiddleware::new("test-secret".to_string());
        assert!(!RbacMiddleware::is_public_path(
            "/repos/abc123/info/refs",
            &middleware.public_paths
        ));
        assert!(!RbacMiddleware::is_public_path(
            "/repos/abc123/git-upload-pack",
            &middleware.public_paths
        ));
        assert!(!RbacMiddleware::is_public_path(
            "/repos/abc123/git-receive-pack",
            &middleware.public_paths
        ));
        assert!(!RbacMiddleware::is_public_path(
            "/repos/abc123",
            &middleware.public_paths
        ));
    }

    #[test]
    fn test_spa_routes_still_public() {
        let middleware = RbacMiddleware::new("test-secret".to_string());
        assert!(RbacMiddleware::is_public_path(
            "/login",
            &middleware.public_paths
        ));
        assert!(RbacMiddleware::is_public_path(
            "/dashboard",
            &middleware.public_paths
        ));
        assert!(RbacMiddleware::is_public_path(
            "/assets/main.js",
            &middleware.public_paths
        ));
        assert!(RbacMiddleware::is_public_path(
            "/profile",
            &middleware.public_paths
        ));
    }

    #[test]
    fn test_base64_decode_valid() {
        let result = base64_decode("dXNlcjpldm9fc2tfdGVzdA==");
        assert_eq!(result.ok().as_deref(), Some("user:evo_sk_test"));
    }

    #[test]
    fn test_base64_decode_invalid() {
        assert!(base64_decode("!!!not-base64!!!").is_err());
    }

    #[actix_rt::test]
    async fn test_extract_token_bearer() {
        let req = actix_web::test::TestRequest::default()
            .insert_header(("Authorization", "Bearer my-jwt-token"))
            .to_srv_request();
        let result = extract_token(&req);
        assert!(result.is_some());
        let (kind, token) = result.unwrap();
        assert_eq!(kind, "token");
        assert_eq!(token, "my-jwt-token");
    }

    #[actix_rt::test]
    async fn test_extract_token_basic() {
        let req = actix_web::test::TestRequest::default()
            .insert_header(("Authorization", "Basic dXNlcjpldm9fc2tfdGVzdA=="))
            .to_srv_request();
        let result = extract_token(&req);
        assert!(result.is_some());
        let (kind, token) = result.unwrap();
        assert_eq!(kind, "basic");
        assert_eq!(token, "evo_sk_test");
    }

    #[actix_rt::test]
    async fn test_extract_token_x_api_key() {
        let req = actix_web::test::TestRequest::default()
            .insert_header(("X-API-Key", "evo_sk_mykey123"))
            .to_srv_request();
        let result = extract_token(&req);
        assert!(result.is_some());
        let (kind, token) = result.unwrap();
        assert_eq!(kind, "basic");
        assert_eq!(token, "evo_sk_mykey123");
    }

    #[actix_rt::test]
    async fn test_extract_token_none() {
        let req = actix_web::test::TestRequest::default().to_srv_request();
        assert!(extract_token(&req).is_none());
    }
}
