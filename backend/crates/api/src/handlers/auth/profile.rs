//! User profile handlers

use actix_web::{web, HttpResponse, Responder};

use crate::dto::auth_dto::*;
use crate::dto::common::ApiResponse;
use crate::middleware::auth::AuthenticatedUser;
use crate::state::AppState;
use domain::user::UpdateUser;
use validator::Validate;

/// DTO for update profile request
#[derive(Debug, serde::Deserialize)]
pub struct UpdateProfileRequest {
    pub username: Option<String>,
    pub email: Option<String>,
}

/// Get current user info
pub async fn get_current_user(
    user: AuthenticatedUser,
    state: web::Data<AppState>,
) -> impl Responder {
    // Fetch full user data from repository
    match state.user_repo.find_by_id(user.user_id).await {
        Ok(Some(full_user)) => {
            let role_str = serde_json::to_value(&full_user.role)
                .ok()
                .and_then(|v| v.as_str().map(String::from))
                .unwrap_or_else(|| "user".to_string());
            let tenant_role_str = serde_json::to_value(&full_user.tenant_role)
                .ok()
                .and_then(|v| v.as_str().map(String::from))
                .unwrap_or_else(|| "member".to_string());

            HttpResponse::Ok().json(ApiResponse::success(UserInfo {
                id: full_user.id.to_string(),
                email: full_user.email,
                username: full_user.username,
                role: role_str,
                tenant_id: full_user.tenant_id.to_string(),
                tenant_role: tenant_role_str,
                email_verified: full_user.email_verified,
            }))
        }
        Ok(None) => HttpResponse::NotFound()
            .json(ApiResponse::<()>::error("USER_NOT_FOUND", "User not found")),
        Err(e) => {
            tracing::error!("Failed to fetch user: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to fetch user information",
            ))
        }
    }
}

/// Update user profile
pub async fn update_profile(
    user: AuthenticatedUser,
    body: web::Json<UpdateProfileRequest>,
    state: web::Data<AppState>,
) -> impl Responder {
    let update = UpdateUser {
        username: body.username.clone(),
        email: body.email.clone(),
    };

    if let Err(errors) = update.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "VALIDATION_ERROR",
            &format!("{}", errors),
        ));
    }

    match state.user_repo.update(user.user_id, update).await {
        Ok(updated_user) => {
            let role_str = serde_json::to_value(&updated_user.role)
                .ok()
                .and_then(|v| v.as_str().map(String::from))
                .unwrap_or_else(|| "user".to_string());
            let tenant_role_str = serde_json::to_value(&updated_user.tenant_role)
                .ok()
                .and_then(|v| v.as_str().map(String::from))
                .unwrap_or_else(|| "member".to_string());

            HttpResponse::Ok().json(ApiResponse::success(UserInfo {
                id: updated_user.id.to_string(),
                email: updated_user.email,
                username: updated_user.username,
                role: role_str,
                tenant_id: updated_user.tenant_id.to_string(),
                tenant_role: tenant_role_str,
                email_verified: updated_user.email_verified,
            }))
        }
        Err(e) => {
            tracing::error!("Failed to update profile: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to update profile",
            ))
        }
    }
}
