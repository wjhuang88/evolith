//! Login, logout, and token refresh handlers

use actix_web::cookie::{time::Duration, Cookie, SameSite};
use actix_web::{web, HttpResponse, Responder};
use service_auth::{LoginError, LoginOutcome, LoginService};
use uuid::Uuid;

use crate::dto::auth_dto::*;
use crate::dto::common::ApiResponse;
use crate::middleware::auth::AuthenticatedUser;
use crate::middleware::csrf::CSRF_COOKIE_NAME;
use crate::middleware::rbac::JWT_COOKIE_NAME;
use crate::state::AppState;
use validator::Validate;

/// User login. Password validation/JWT issuance is shared with future GraphQL.
pub async fn login(body: web::Json<LoginRequest>, state: web::Data<AppState>) -> impl Responder {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "VALIDATION_ERROR",
            &format!("{}", errors),
        ));
    }

    let service = LoginService::new(
        state.user_repo.clone(),
        state.tenant_repo.clone(),
        state.hasher.clone(),
        state.jwt.clone(),
    );
    let LoginOutcome {
        token,
        expires_at,
        user,
        tenant,
    } = match service.authenticate(&body.email, &body.password).await {
        Ok(outcome) => outcome,
        Err(LoginError::InvalidCredentials) => {
            return HttpResponse::Unauthorized().json(ApiResponse::<()>::error(
                "INVALID_CREDENTIALS",
                "Invalid email or password",
            ));
        }
        Err(LoginError::Internal) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Login failed",
            ));
        }
    };

    // HTTP transport retains the existing cookie semantics unchanged.
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
        .json(ApiResponse::success(AuthResponseData {
            token,
            expires_at,
            user: UserInfo {
                id: user.id,
                email: user.email,
                username: user.username,
                role: user.role,
                tenant_id: user.tenant_id,
                tenant_role: user.tenant_role,
                email_verified: user.email_verified,
            },
            tenant: TenantInfo {
                id: tenant.id,
                name: tenant.name,
                slug: tenant.slug,
                plan: tenant.plan,
            },
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
