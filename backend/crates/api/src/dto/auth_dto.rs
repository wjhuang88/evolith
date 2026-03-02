//! Auth DTOs
//! Defines request/response structures for authentication endpoints

use serde::{Deserialize, Serialize};
use validator::Validate;

/// Login request
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 1, message = "Password is required"))]
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

    /// Optional tenant name for new registration
    pub tenant_name: Option<String>,

    /// Optional tenant slug for new registration
    pub tenant_slug: Option<String>,
}

/// Send verification email request
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct SendVerifyEmailRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

/// Verify email request
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct VerifyEmailRequest {
    pub token: String,
}

/// Forgot password request
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct ForgotPasswordRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

/// Reset password request
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct ResetPasswordRequest {
    pub token: String,

    #[validate(length(min = 8, max = 128, message = "Password must be 8-128 characters"))]
    pub password: String,
}

/// Refresh token request
#[derive(Debug, Deserialize, Serialize)]
pub struct RefreshTokenRequest {
    pub token: Option<String>,
}

/// Auth response data
#[derive(Debug, Serialize)]
pub struct AuthResponseData {
    pub token: String,
    pub expires_at: i64,
    pub user: UserInfo,
    pub tenant: TenantInfo,
}

/// Tenant info in auth response
#[derive(Debug, Serialize, Clone)]
pub struct TenantInfo {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub plan: String,
}

/// User info
#[derive(Debug, Serialize, Clone)]
pub struct UserInfo {
    pub id: String,
    pub email: String,
    pub username: String,
    pub role: String,
    pub tenant_id: String,
    pub tenant_role: String,
    pub email_verified: bool,
}

/// Token response (for refresh)
#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub token: String,
    pub expires_at: i64,
}

/// Simple success message response
#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message: String,
}

/// Email verification sent response
#[derive(Debug, Serialize)]
pub struct VerifyEmailResponse {
    pub message: String,
    pub email: String,
}

/// Password reset sent response
#[derive(Debug, Serialize)]
pub struct PasswordResetResponse {
    pub message: String,
    pub email: String,
}
