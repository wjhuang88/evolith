//! Security headers middleware
//!
//! Adds security-related HTTP headers to all responses:
//! - X-Content-Type-Options: nosniff
//! - X-Frame-Options: DENY
//! - X-XSS-Protection: 1; mode=block
//! - Referrer-Policy: strict-origin-when-cross-origin
//! - Permissions-Policy: restrictive defaults
//! - Content-Security-Policy: configurable for dev/prod

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http::header::{HeaderName, HeaderValue},
    Error,
};
use std::future::{ready, Ready};
use std::pin::Pin;

#[derive(Clone)]
pub struct SecurityHeadersMiddleware {
    is_dev: bool,
}

impl SecurityHeadersMiddleware {
    pub fn new(is_dev: bool) -> Self {
        Self { is_dev }
    }
}

impl<S, B> Transform<S, ServiceRequest> for SecurityHeadersMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SecurityHeadersMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SecurityHeadersMiddlewareService {
            service,
            is_dev: self.is_dev,
        }))
    }
}

pub struct SecurityHeadersMiddlewareService<S> {
    service: S,
    is_dev: bool,
}

impl<S, B> Service<ServiceRequest> for SecurityHeadersMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let is_dev = self.is_dev;
        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;
            let headers = res.headers_mut();

            headers.insert(
                HeaderName::from_static("x-content-type-options"),
                HeaderValue::from_static("nosniff"),
            );
            headers.insert(
                HeaderName::from_static("x-frame-options"),
                HeaderValue::from_static("DENY"),
            );
            headers.insert(
                HeaderName::from_static("x-xss-protection"),
                HeaderValue::from_static("1; mode=block"),
            );
            headers.insert(
                HeaderName::from_static("referrer-policy"),
                HeaderValue::from_static("strict-origin-when-cross-origin"),
            );
            headers.insert(
                HeaderName::from_static("permissions-policy"),
                HeaderValue::from_static("camera=(), microphone=(), geolocation=(), payment=()"),
            );

            let csp = if is_dev {
                "default-src 'self'; script-src 'self' 'unsafe-inline' 'unsafe-eval'; style-src 'self' 'unsafe-inline'; img-src 'self' data: blob:; connect-src 'self' ws: wss: http://localhost:*; font-src 'self' data:"
            } else {
                "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; connect-src 'self'; font-src 'self'; frame-ancestors 'none'; base-uri 'self'; form-action 'self'"
            };

            if let Ok(csp_value) = HeaderValue::from_str(csp) {
                headers.insert(
                    HeaderName::from_static("content-security-policy"),
                    csp_value,
                );
            }

            Ok(res)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_headers_middleware_new() {
        let middleware = SecurityHeadersMiddleware::new(true);
        assert!(middleware.is_dev);

        let middleware = SecurityHeadersMiddleware::new(false);
        assert!(!middleware.is_dev);
    }
}
