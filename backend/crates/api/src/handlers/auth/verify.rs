//! Email verification handlers

use actix_web::{web, HttpResponse, Responder};

use crate::dto::auth_dto::*;
use crate::dto::common::ApiResponse;

/// Send verification email (stub - needs email service)
pub async fn send_verification_email(_body: web::Json<SendVerifyEmailRequest>) -> impl Responder {
    HttpResponse::NotImplemented().json(ApiResponse::<()>::error(
        "NOT_IMPLEMENTED",
        "Email verification is not yet implemented. Requires email service integration.",
    ))
}

/// Verify email with token (stub - needs email service)
pub async fn verify_email(_body: web::Json<VerifyEmailRequest>) -> impl Responder {
    HttpResponse::NotImplemented().json(ApiResponse::<()>::error(
        "NOT_IMPLEMENTED",
        "Email verification is not yet implemented. Requires email service integration.",
    ))
}
