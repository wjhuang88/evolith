//! Middleware

pub mod auth;
pub mod csrf;
pub mod error;
pub mod rate_limit;
pub mod rbac;
pub mod request_id;
pub mod security_headers;
pub mod tenant;

pub use csrf::{CsrfMiddleware, CSRF_COOKIE_NAME, CSRF_HEADER_NAME};
pub use rate_limit::{
    create_api_key_limiter, create_authenticated_limiter, create_default_limiter,
    create_governor_config, create_unauthenticated_limiter,
};
pub use rbac::{CurrentUser, CurrentUserExt, RbacError, RbacMiddleware, JWT_COOKIE_NAME};
pub use request_id::{RequestId, RequestIdExt, RequestIdMiddleware, X_REQUEST_ID_HEADER};
pub use security_headers::SecurityHeadersMiddleware;
pub use tenant::{TenantContextExt, TenantMiddleware};
