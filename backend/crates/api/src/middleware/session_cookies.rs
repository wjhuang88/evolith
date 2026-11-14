//! HTTP-only session cookie construction shared by REST and GraphQL transports.
//! Password verification/JWT issuance do not belong in this module.

use actix_web::cookie::{time::Duration, Cookie, SameSite};
use uuid::Uuid;

use super::csrf::CSRF_COOKIE_NAME;
use super::rbac::JWT_COOKIE_NAME;

/// Create the same JWT/httpOnly + double-submit CSRF cookie pair for both transports.
///
/// The caller owns attaching these to the final HTTP response on successful authentication.
pub fn issue_login_cookies(
    token: &str,
    expires_at: i64,
    is_production: bool,
) -> [Cookie<'static>; 2] {
    let lifetime = Duration::seconds(expires_at - chrono::Utc::now().timestamp());
    let jwt_cookie = Cookie::build(JWT_COOKIE_NAME, token.to_owned())
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .secure(is_production)
        .max_age(lifetime)
        .finish();

    let csrf_cookie = Cookie::build(CSRF_COOKIE_NAME, Uuid::new_v4().to_string())
        .path("/")
        .http_only(false)
        .same_site(SameSite::Lax)
        .secure(is_production)
        .max_age(lifetime)
        .finish();

    [jwt_cookie, csrf_cookie]
}
