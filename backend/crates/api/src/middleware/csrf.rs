//! CSRF (Cross-Site Request Forgery) protection middleware
//!
//! Implements double-submit cookie pattern for CSRF protection.
//! - A non-httpOnly `csrf_token` cookie is set on login/register/refresh
//! - State-changing requests (POST/PUT/PATCH/DELETE) must include
//!   an `X-CSRF-Token` header matching the cookie value

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse},
    http::{header, Method},
    Error,
};
use std::future::{ready, Ready};
use std::pin::Pin;

/// CSRF cookie name
pub const CSRF_COOKIE_NAME: &str = "csrf_token";

/// CSRF header name
pub const CSRF_HEADER_NAME: &str = "X-CSRF-Token";

/// CSRF error types
#[derive(Debug, thiserror::Error)]
pub enum CsrfError {
    #[error("CSRF token missing from cookie")]
    MissingCookie,
    #[error("CSRF token missing from header")]
    MissingHeader,
    #[error("CSRF token mismatch")]
    TokenMismatch,
}

impl actix_web::ResponseError for CsrfError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        use crate::dto::common::ApiResponse;
        use actix_web::http::StatusCode;

        actix_web::HttpResponse::build(StatusCode::FORBIDDEN)
            .json(ApiResponse::<()>::error("CSRF_ERROR", &self.to_string()))
    }
}

/// CSRF middleware configuration
#[derive(Clone)]
pub struct CsrfMiddleware {
    /// Paths that don't require CSRF validation (public paths and GET-only)
    pub exempt_paths: Vec<String>,
}

impl CsrfMiddleware {
    pub fn new() -> Self {
        Self {
            exempt_paths: vec![
                "/health".to_string(),
                "/health/live".to_string(),
                "/health/ready".to_string(),
                "/mcp".to_string(),
                "/api/v1/auth/login".to_string(),
                "/api/v1/auth/register".to_string(),
                "/api/v1/auth/forgot-password".to_string(),
                "/api/v1/auth/reset-password".to_string(),
                "/api/v1/auth/send-verify".to_string(),
                "/api/v1/auth/verify-email".to_string(),
                "/api/v1/invitations/accept".to_string(),
            ],
        }
    }

    fn requires_csrf(method: &Method) -> bool {
        matches!(
            *method,
            Method::POST | Method::PUT | Method::PATCH | Method::DELETE
        )
    }

    fn is_exempt_path(path: &str, exempt_paths: &[String]) -> bool {
        exempt_paths.iter().any(|p| path.starts_with(p))
            || (path.starts_with("/api/v1/tenant/") && path.ends_with("/members/join"))
    }
}

impl Default for CsrfMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> actix_web::dev::Transform<S, ServiceRequest> for CsrfMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<actix_web::body::BoxBody>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = CsrfMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CsrfMiddlewareService {
            service,
            exempt_paths: self.exempt_paths.clone(),
        }))
    }
}

pub struct CsrfMiddlewareService<S> {
    service: S,
    exempt_paths: Vec<String>,
}

impl<S> Service<ServiceRequest> for CsrfMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<actix_web::body::BoxBody>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = Error;
    type Future = Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let path = req.path();

        if CsrfMiddleware::is_exempt_path(path, &self.exempt_paths) {
            let fut = self.service.call(req);
            return Box::pin(fut);
        }

        if !CsrfMiddleware::requires_csrf(req.method()) {
            let fut = self.service.call(req);
            return Box::pin(fut);
        }

        let cookie_value = match req.cookie(CSRF_COOKIE_NAME) {
            Some(cookie) => cookie.value().to_string(),
            None => {
                return Box::pin(
                    async move { Err(actix_web::Error::from(CsrfError::MissingCookie)) },
                );
            }
        };

        let header_value = match req
            .headers()
            .get(header::HeaderName::from_static("x-csrf-token"))
        {
            Some(value) => match value.to_str() {
                Ok(s) => s.to_string(),
                Err(_) => {
                    return Box::pin(async move {
                        Err(actix_web::Error::from(CsrfError::MissingHeader))
                    });
                }
            },
            None => {
                return Box::pin(
                    async move { Err(actix_web::Error::from(CsrfError::MissingHeader)) },
                );
            }
        };

        if cookie_value != header_value {
            return Box::pin(async move { Err(actix_web::Error::from(CsrfError::TokenMismatch)) });
        }

        let fut = self.service.call(req);
        Box::pin(fut)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_requires_csrf() {
        assert!(CsrfMiddleware::requires_csrf(&Method::POST));
        assert!(CsrfMiddleware::requires_csrf(&Method::PUT));
        assert!(CsrfMiddleware::requires_csrf(&Method::PATCH));
        assert!(CsrfMiddleware::requires_csrf(&Method::DELETE));
        assert!(!CsrfMiddleware::requires_csrf(&Method::GET));
        assert!(!CsrfMiddleware::requires_csrf(&Method::HEAD));
        assert!(!CsrfMiddleware::requires_csrf(&Method::OPTIONS));
    }

    #[test]
    fn test_exempt_paths() {
        let middleware = CsrfMiddleware::new();
        assert!(CsrfMiddleware::is_exempt_path(
            "/health",
            &middleware.exempt_paths
        ));
        assert!(CsrfMiddleware::is_exempt_path(
            "/health/live",
            &middleware.exempt_paths
        ));
        assert!(CsrfMiddleware::is_exempt_path(
            "/mcp",
            &middleware.exempt_paths
        ));
        assert!(CsrfMiddleware::is_exempt_path(
            "/api/v1/auth/login",
            &middleware.exempt_paths
        ));
        assert!(CsrfMiddleware::is_exempt_path(
            "/api/v1/invitations/accept",
            &middleware.exempt_paths
        ));
        assert!(CsrfMiddleware::is_exempt_path(
            "/api/v1/tenant/11111111-1111-1111-1111-111111111111/members/join",
            &middleware.exempt_paths
        ));
        assert!(!CsrfMiddleware::is_exempt_path(
            "/api/v1/tools",
            &middleware.exempt_paths
        ));
        assert!(!CsrfMiddleware::is_exempt_path(
            "/api/v1/skills",
            &middleware.exempt_paths
        ));
        assert!(!CsrfMiddleware::is_exempt_path(
            "/api/v1/tenant/11111111-1111-1111-1111-111111111111/members",
            &middleware.exempt_paths
        ));
    }
}
