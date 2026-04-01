//! Auth middleware - JWT authentication via Actix-web extractor
//!
//! Provides AuthenticatedUser extractor that pulls CurrentUser from request extensions.
//! The actual JWT validation is done by RbacMiddleware (see rbac.rs).

use std::future::{ready, Ready};

use actix_web::{dev::Payload, Error, FromRequest, HttpMessage, HttpRequest};

use crate::dto::common::ApiResponse;
use crate::middleware::rbac::CurrentUser;

/// Extractor that requires a valid authenticated user.
/// Use as handler parameter: `async fn handler(user: AuthenticatedUser) -> impl Responder`
///
/// Returns 401 if no valid JWT token / CurrentUser in request extensions.
pub struct AuthenticatedUser(pub CurrentUser);

impl std::ops::Deref for AuthenticatedUser {
    type Target = CurrentUser;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let user = req.extensions().get::<CurrentUser>().cloned();
        match user {
            Some(u) => ready(Ok(AuthenticatedUser(u))),
            None => ready(Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
                ApiResponse::<()>::error("UNAUTHORIZED", "Authentication required")
            )))),
        }
    }
}
