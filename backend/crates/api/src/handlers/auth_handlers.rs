//! Auth handlers

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::dto::common::ApiResponse;

/// Login request
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 8, max = 128, message = "Password must be 8-128 characters"))]
    pub password: String,
}

/// Register request
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 3, max = 50, message = "Username must be 3-50 characters"))]
    pub username: String,

    #[validate(length(min = 8, max = 128, message = "Password must be 8-128 characters"))]
    pub password: String,
}

/// Auth response
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub expires_at: i64,
    pub user: UserInfo,
}

/// User info
#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: String,
    pub email: String,
    pub username: String,
    pub role: String,
}

/// Login handler
pub async fn login(body: web::Json<LoginRequest>) -> impl Responder {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "VALIDATION_ERROR",
            &errors.to_string(),
        ));
    }

    HttpResponse::Ok().json(ApiResponse::<()>::error(
        "NOT_IMPLEMENTED",
        "Login not implemented yet",
    ))
}

/// Register handler
pub async fn register(body: web::Json<RegisterRequest>) -> impl Responder {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "VALIDATION_ERROR",
            &errors.to_string(),
        ));
    }

    HttpResponse::Ok().json(ApiResponse::<()>::error(
        "NOT_IMPLEMENTED",
        "Registration not implemented yet",
    ))
}

/// Logout handler
pub async fn logout() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::<serde_json::Value>::success(
        serde_json::json!({ "message": "Logged out successfully" }),
    ))
}

/// Refresh token handler
pub async fn refresh_token() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::<()>::error(
        "NOT_IMPLEMENTED",
        "Token refresh not implemented yet",
    ))
}

/// Get current user handler
pub async fn get_current_user() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::<()>::error(
        "NOT_IMPLEMENTED",
        "Get current user not implemented yet",
    ))
}
