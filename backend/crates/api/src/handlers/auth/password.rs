//! Password management handlers

use actix_web::{web, HttpResponse, Responder};
use chrono::{Duration, Utc};
use uuid::Uuid;
use validator::Validate;

use crate::dto::auth_dto::*;
use crate::dto::common::ApiResponse;
use crate::middleware::auth::AuthenticatedUser;
use crate::state::AppState;

/// DTO for change password request
#[derive(Debug, serde::Deserialize, validator::Validate)]
pub struct ChangePasswordRequest {
    #[validate(length(min = 1, message = "Current password is required"))]
    pub old_password: String,

    #[validate(length(min = 8, max = 128, message = "New password must be 8-128 characters"))]
    pub new_password: String,
}

/// Change password (authenticated)
pub async fn change_password(
    user: AuthenticatedUser,
    body: web::Json<ChangePasswordRequest>,
    state: web::Data<AppState>,
) -> impl Responder {
    // Fetch current user to get password hash
    let current_user = match state.user_repo.find_by_id(user.user_id).await {
        Ok(Some(u)) => u,
        Ok(None) => {
            return HttpResponse::NotFound()
                .json(ApiResponse::<()>::error("USER_NOT_FOUND", "User not found"));
        }
        Err(e) => {
            tracing::error!("Failed to find user: {}", e);
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to change password",
            ));
        }
    };

    // Verify old password
    match state
        .hasher
        .verify_password(&body.old_password, &current_user.password_hash)
    {
        Ok(true) => {}
        Ok(false) => {
            return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                "INVALID_PASSWORD",
                "Current password is incorrect",
            ));
        }
        Err(e) => {
            tracing::error!("Password verification error: {}", e);
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to change password",
            ));
        }
    }

    // Validate new password strength
    if let Err(e) = state.hasher.validate_strength(&body.new_password) {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "WEAK_PASSWORD",
            &format!("New password does not meet requirements: {}", e),
        ));
    }

    // Hash new password
    let new_hash = match state.hasher.hash_password(&body.new_password) {
        Ok(hash) => hash,
        Err(e) => {
            tracing::error!("Failed to hash new password: {}", e);
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to change password",
            ));
        }
    };

    // Update password in repository
    match state
        .user_repo
        .update_password(user.user_id, &new_hash)
        .await
    {
        Ok(()) => HttpResponse::Ok().json(ApiResponse::success(MessageResponse {
            message: "Password changed successfully".to_string(),
        })),
        Err(e) => {
            tracing::error!("Failed to update password: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to change password",
            ))
        }
    }
}

/// Forgot password — generate reset token and send email
pub async fn forgot_password(
    body: web::Json<ForgotPasswordRequest>,
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
                message: "If the email exists, a reset link has been sent.".to_string(),
            }));
        }
    };

    let token = format!("prt_{}", Uuid::new_v4().simple());
    let expires_at = Utc::now() + Duration::hours(1);

    if let Err(e) = state
        .user_repo
        .set_reset_token(user.id, &token, expires_at)
        .await
    {
        tracing::error!("Failed to save reset token: {}", e);
        return HttpResponse::Ok().json(ApiResponse::success(MessageResponse {
            message: "If the email exists, a reset link has been sent.".to_string(),
        }));
    }

    // Send reset email (best effort)
    let base_url = state.config.app.public_url.trim_end_matches('/');
    if let Err(e) = state
        .mailer
        .send_password_reset_email(&user.email, &user.username, &token, base_url)
        .await
    {
        tracing::warn!("Failed to send reset email: {}", e);
    }

    HttpResponse::Ok().json(ApiResponse::success(MessageResponse {
        message: "If the email exists, a reset link has been sent.".to_string(),
    }))
}

/// Reset password with token
pub async fn reset_password(
    body: web::Json<ResetPasswordRequest>,
    state: web::Data<AppState>,
) -> impl Responder {
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest()
            .json(ApiResponse::<()>::error("VALIDATION_ERROR", &e.to_string()));
    }

    let user = match state.user_repo.find_by_reset_token(&body.token).await {
        Ok(Some(u)) => u,
        Ok(None) => {
            return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                "INVALID_TOKEN",
                "Invalid or expired reset token",
            ));
        }
        Err(e) => {
            tracing::error!("Failed to find reset token: {}", e);
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to reset password",
            ));
        }
    };

    // Check token expiry
    if let Some(expires_at) = user.reset_expires_at {
        if Utc::now() > expires_at {
            let _ = state.user_repo.clear_reset_token(user.id).await;
            return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                "TOKEN_EXPIRED",
                "Reset token has expired. Please request a new one.",
            ));
        }
    } else {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "INVALID_TOKEN",
            "Invalid reset token",
        ));
    }

    // Validate new password strength
    if let Err(e) = state.hasher.validate_strength(&body.password) {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "WEAK_PASSWORD",
            &format!("Password does not meet requirements: {}", e),
        ));
    }

    // Hash new password
    let new_hash = match state.hasher.hash_password(&body.password) {
        Ok(hash) => hash,
        Err(e) => {
            tracing::error!("Failed to hash password: {}", e);
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to reset password",
            ));
        }
    };

    if let Err(e) = state.user_repo.update_password(user.id, &new_hash).await {
        tracing::error!("Failed to update password: {}", e);
        return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            "INTERNAL_ERROR",
            "Failed to reset password",
        ));
    }

    if let Err(e) = state.user_repo.clear_reset_token(user.id).await {
        tracing::warn!("Failed to clear reset token: {}", e);
    }

    HttpResponse::Ok().json(ApiResponse::success(MessageResponse {
        message: "Password has been reset successfully.".to_string(),
    }))
}
