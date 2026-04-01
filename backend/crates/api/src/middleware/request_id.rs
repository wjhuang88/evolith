//! Request ID middleware
//!
//! Extracts or generates a unique request ID for request tracing.
//! - Extracts `X-Request-ID` from incoming request header if present
//! - Generates a new UUID v4 if not present
//! - Stores the request ID in request extensions
//! - Adds `X-Request-ID` to response headers

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use std::future::{ready, Ready};
use std::pin::Pin;

/// Header name for request ID
pub const X_REQUEST_ID_HEADER: &str = "X-Request-ID";

/// Request ID stored in request extensions
#[derive(Debug, Clone)]
pub struct RequestId(pub String);

impl std::ops::Deref for RequestId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Request ID middleware configuration
#[derive(Clone, Default)]
pub struct RequestIdMiddleware;

impl RequestIdMiddleware {
    pub fn new() -> Self {
        Self
    }
}

impl<S, B> Transform<S, ServiceRequest> for RequestIdMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequestIdMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestIdMiddlewareService { service }))
    }
}

pub struct RequestIdMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RequestIdMiddlewareService<S>
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
        let request_id = req
            .headers()
            .get(X_REQUEST_ID_HEADER)
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string())
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        req.extensions_mut().insert(RequestId(request_id.clone()));

        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;

            res.headers_mut().insert(
                actix_web::http::header::HeaderName::try_from(X_REQUEST_ID_HEADER).unwrap_or_else(
                    |_| actix_web::http::header::HeaderName::from_static("x-request-id"),
                ),
                actix_web::http::header::HeaderValue::from_str(&request_id).unwrap_or_else(|_| {
                    actix_web::http::header::HeaderValue::from_static("invalid")
                }),
            );

            Ok(res)
        })
    }
}

/// Extension trait to get request ID from request
pub trait RequestIdExt {
    fn get_request_id(&self) -> Option<String>;
}

impl RequestIdExt for actix_web::HttpRequest {
    fn get_request_id(&self) -> Option<String> {
        self.extensions().get::<RequestId>().map(|id| id.0.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_id_deref() {
        let id = RequestId("test-id-123".to_string());
        assert_eq!(&*id, "test-id-123");
    }

    #[test]
    fn test_request_id_clone() {
        let id = RequestId("test-id-456".to_string());
        let cloned = id.clone();
        assert_eq!(id.0, cloned.0);
    }
}
