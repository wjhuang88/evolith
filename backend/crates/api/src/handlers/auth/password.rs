//! Password management handlers

use actix_web::{web, HttpResponse, Responder};
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

/// Forgot password (stub - needs email service)
pub async fn forgot_password(_body: web::Json<ForgotPasswordRequest>) -> impl Responder {
    HttpResponse::NotImplemented().json(ApiResponse::<()>::error(
        "NOT_IMPLEMENTED",
        "Password reset is not yet implemented. Requires email service integration.",
    ))
}

/// Reset password with token (stub - needs email service)
pub async fn reset_password(_body: web::Json<ResetPasswordRequest>) -> impl Responder {
    HttpResponse::NotImplemented().json(ApiResponse::<()>::error(
        "NOT_IMPLEMENTED",
        "Password reset is not yet implemented. Requires email service integration.",
    ))
}
