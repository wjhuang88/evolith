//! Email verification handlers

use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use validator::Validate;

use crate::dto::auth_dto::*;
use crate::dto::common::ApiResponse;
use crate::state::AppState;

/// Send verification email — generate token and send email
pub async fn send_verification_email(
    body: web::Json<SendVerifyEmailRequest>,
    state: web::Data<AppState>,
) -> impl Responder {
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest()
            .json(ApiResponse::<()>::error("VALIDATION_ERROR", &e.to_string()));
    }

    // Always return success to prevent email enumeration
    let user = match state.user_repo.find_by_email(&body.email).await {
        Ok(Some(u)) => u,
        Ok(None) | Err(_) => {
            return HttpResponse::Ok().json(ApiResponse::success(MessageResponse {
                message: "If the email exists, a verification link has been sent.".to_string(),
            }));
        }
    };

    // If already verified, don't resend
    if user.email_verified {
        return HttpResponse::Ok().json(ApiResponse::success(MessageResponse {
            message: "If the email exists, a verification link has been sent.".to_string(),
        }));
    }

    let token = format!("evt_{}", Uuid::new_v4().simple());

    if let Err(e) = state.user_repo.set_verify_token(user.id, &token).await {
        tracing::error!("Failed to save verification token: {}", e);
        return HttpResponse::Ok().json(ApiResponse::success(MessageResponse {
            message: "If the email exists, a verification link has been sent.".to_string(),
        }));
    }

    // Send verification email (best effort)
    let base_url = state.config.app.public_url.trim_end_matches('/');
    if let Err(e) = state
        .mailer
        .send_verification_email(&user.email, &user.username, &token, base_url)
        .await
    {
        tracing::warn!("Failed to send verification email: {}", e);
    }

    HttpResponse::Ok().json(ApiResponse::success(MessageResponse {
        message: "If the email exists, a verification link has been sent.".to_string(),
    }))
}

/// Verify email with token
pub async fn verify_email(
    body: web::Json<VerifyEmailRequest>,
    state: web::Data<AppState>,
) -> impl Responder {
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest()
            .json(ApiResponse::<()>::error("VALIDATION_ERROR", &e.to_string()));
    }

    let user = match state.user_repo.find_by_verify_token(&body.token).await {
        Ok(Some(u)) => u,
        Ok(None) => {
            return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                "INVALID_TOKEN",
                "Invalid or expired verification token",
            ));
        }
        Err(e) => {
            tracing::error!("Failed to find verification token: {}", e);
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to verify email",
            ));
        }
    };

    if let Err(e) = state.user_repo.verify_email(user.id).await {
        tracing::error!("Failed to verify email: {}", e);
        return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            "INTERNAL_ERROR",
            "Failed to verify email",
        ));
    }

    HttpResponse::Ok().json(ApiResponse::success(MessageResponse {
        message: "Email verified successfully.".to_string(),
    }))
}
