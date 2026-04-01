//! Login, logout, and token refresh handlers

use actix_web::cookie::{time::Duration, Cookie, SameSite};
use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

use crate::dto::auth_dto::*;
use crate::dto::common::ApiResponse;
use crate::middleware::auth::AuthenticatedUser;
use crate::middleware::csrf::CSRF_COOKIE_NAME;
use crate::middleware::rbac::JWT_COOKIE_NAME;
use crate::state::AppState;
use validator::Validate;

/// User login
pub async fn login(body: web::Json<LoginRequest>, state: web::Data<AppState>) -> impl Responder {
    // Validate request
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "VALIDATION_ERROR",
            &format!("{}", errors),
        ));
    }

    // Find user by email
    let user = match state.user_repo.find_by_email(&body.email).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return HttpResponse::Unauthorized().json(ApiResponse::<()>::error(
                "INVALID_CREDENTIALS",
                "Invalid email or password",
            ));
        }
        Err(e) => {
            tracing::error!("Failed to find user by email: {}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::<()>::error("INTERNAL_ERROR", "Login failed"));
        }
    };

    // Verify password
    match state
        .hasher
        .verify_password(&body.password, &user.password_hash)
    {
        Ok(true) => {}
        Ok(false) => {
            return HttpResponse::Unauthorized().json(ApiResponse::<()>::error(
                "INVALID_CREDENTIALS",
                "Invalid email or password",
            ));
        }
        Err(e) => {
            tracing::error!("Password verification error: {}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::<()>::error("INTERNAL_ERROR", "Login failed"));
        }
    }

    // Serialize role and tenant_role to string for JWT
    let role_str = serde_json::to_value(&user.role)
        .ok()
        .and_then(|v| v.as_str().map(String::from))
        .unwrap_or_else(|| "user".to_string());
    let tenant_role_str = serde_json::to_value(&user.tenant_role)
        .ok()
        .and_then(|v| v.as_str().map(String::from))
        .unwrap_or_else(|| "member".to_string());

    // Generate JWT token
    let (token, expires_at) =
        match state
            .jwt
            .generate_token(user.id, &role_str, user.tenant_id, &tenant_role_str)
        {
            Ok(result) => result,
            Err(e) => {
                tracing::error!("Failed to generate token: {}", e);
                return HttpResponse::InternalServerError()
                    .json(ApiResponse::<()>::error("INTERNAL_ERROR", "Login failed"));
            }
        };

    // Get tenant info
    let tenant_info = match state.tenant_repo.find_by_id(user.tenant_id).await {
        Ok(Some(tenant)) => TenantInfo {
            id: tenant.id.to_string(),
            name: tenant.name,
            slug: tenant.slug,
            plan: format!("{}", tenant.plan),
        },
        _ => TenantInfo {
            id: user.tenant_id.to_string(),
            name: "Unknown".to_string(),
            slug: "unknown".to_string(),
            plan: "free".to_string(),
        },
    };

    let is_production = state.config.is_production();
    let csrf_token = Uuid::new_v4().to_string();

    // Build httpOnly JWT cookie
    let jwt_cookie = Cookie::build(JWT_COOKIE_NAME, &token)
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .secure(is_production)
        .max_age(Duration::seconds(
            expires_at - chrono::Utc::now().timestamp(),
        ))
        .finish();

    // Build non-httpOnly CSRF cookie (readable by JS)
    let csrf_cookie = Cookie::build(CSRF_COOKIE_NAME, &csrf_token)
        .path("/")
        .http_only(false)
        .same_site(SameSite::Lax)
        .secure(is_production)
        .max_age(Duration::seconds(
            expires_at - chrono::Utc::now().timestamp(),
        ))
        .finish();

    HttpResponse::Ok()
        .cookie(jwt_cookie)
        .cookie(csrf_cookie)
        .json(ApiResponse::success(AuthResponseData {
            token,
            expires_at,
            user: UserInfo {
                id: user.id.to_string(),
                email: user.email,
                username: user.username,
                role: role_str,
                tenant_id: user.tenant_id.to_string(),
                tenant_role: tenant_role_str,
                email_verified: user.email_verified,
            },
            tenant: tenant_info,
        }))
}

/// Logout (invalidate token - stub, requires token blacklist in future)
pub async fn logout(_user: AuthenticatedUser) -> impl Responder {
    // Clear JWT cookie by setting max_age to 0
    let jwt_cookie = Cookie::build(JWT_COOKIE_NAME, "")
        .path("/")
        .http_only(true)
        .max_age(Duration::ZERO)
        .finish();

    // Clear CSRF cookie
    let csrf_cookie = Cookie::build(CSRF_COOKIE_NAME, "")
        .path("/")
        .http_only(false)
        .max_age(Duration::ZERO)
        .finish();

    HttpResponse::Ok()
        .cookie(jwt_cookie)
        .cookie(csrf_cookie)
        .json(ApiResponse::success(MessageResponse {
            message: "Logged out successfully".to_string(),
        }))
}

/// Refresh JWT token
pub async fn refresh_token(user: AuthenticatedUser, state: web::Data<AppState>) -> impl Responder {
    // Re-generate a new token with current user data
    let role_str = &user.role;
    let tenant_role_str = serde_json::to_value(&user.tenant_role)
        .ok()
        .and_then(|v| v.as_str().map(String::from))
        .unwrap_or_else(|| "member".to_string());

    match state
        .jwt
        .generate_token(user.user_id, role_str, user.tenant_id, &tenant_role_str)
    {
        Ok((token, expires_at)) => {
            let is_production = state.config.is_production();
            let csrf_token = Uuid::new_v4().to_string();

            let jwt_cookie = Cookie::build(JWT_COOKIE_NAME, &token)
                .path("/")
                .http_only(true)
                .same_site(SameSite::Lax)
                .secure(is_production)
                .max_age(Duration::seconds(
                    expires_at - chrono::Utc::now().timestamp(),
                ))
                .finish();

            let csrf_cookie = Cookie::build(CSRF_COOKIE_NAME, &csrf_token)
                .path("/")
                .http_only(false)
                .same_site(SameSite::Lax)
                .secure(is_production)
                .max_age(Duration::seconds(
                    expires_at - chrono::Utc::now().timestamp(),
                ))
                .finish();

            HttpResponse::Ok()
                .cookie(jwt_cookie)
                .cookie(csrf_cookie)
                .json(ApiResponse::success(TokenResponse { token, expires_at }))
        }
        Err(e) => {
            tracing::error!("Failed to refresh token: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to refresh token",
            ))
        }
    }
}
