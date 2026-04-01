//! User registration handler

use actix_web::cookie::{time::Duration, Cookie, SameSite};
use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use validator::Validate;

use crate::dto::auth_dto::*;
use crate::dto::common::ApiResponse;
use crate::middleware::csrf::CSRF_COOKIE_NAME;
use crate::middleware::rbac::JWT_COOKIE_NAME;
use crate::state::AppState;
use domain::user::{NewUser, TenantRole};
use domain::CreateTenantRequest;

/// Generate a URL-safe slug from a username
fn slug_from_username(username: &str) -> String {
    username
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

/// User registration
pub async fn register(
    body: web::Json<RegisterRequest>,
    state: web::Data<AppState>,
) -> impl Responder {
    // Validate request
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "VALIDATION_ERROR",
            &format!("{}", errors),
        ));
    }

    // Validate password strength
    if let Err(e) = state.hasher.validate_strength(&body.password) {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "WEAK_PASSWORD",
            &format!("Password does not meet requirements: {}", e),
        ));
    }

    // Check if email already exists
    match state.user_repo.find_by_email(&body.email).await {
        Ok(Some(_)) => {
            return HttpResponse::Conflict().json(ApiResponse::<()>::error(
                "EMAIL_EXISTS",
                "A user with this email already exists",
            ));
        }
        Ok(None) => {} // Good, email is available
        Err(e) => {
            tracing::error!("Failed to check existing email: {}", e);
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Registration failed",
            ));
        }
    }

    // Hash password with Argon2 BEFORE creating user
    let password_hash = match state.hasher.hash_password(&body.password) {
        Ok(hash) => hash,
        Err(e) => {
            tracing::error!("Failed to hash password: {}", e);
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Registration failed",
            ));
        }
    };

    // Create a new user struct with hashed password
    let new_user = NewUser {
        username: body.username.clone(),
        email: body.email.clone(),
        password: password_hash,
    };

    // Generate a temporary user ID for tenant creation
    let temp_user_id = Uuid::new_v4();

    // Derive tenant name and slug
    let tenant_name = body
        .tenant_name
        .clone()
        .unwrap_or_else(|| format!("{}'s Team", body.username));
    let tenant_slug = body
        .tenant_slug
        .clone()
        .unwrap_or_else(|| slug_from_username(&body.username));

    // Create default tenant for the new user
    let tenant = match state
        .tenant_repo
        .create(
            CreateTenantRequest {
                name: tenant_name,
                slug: tenant_slug,
                owner_email: body.email.clone(),
                owner_username: body.username.clone(),
                owner_password: body.password.clone(), // Not used for actual auth (password already hashed separately)
            },
            temp_user_id,
        )
        .await
    {
        Ok(tenant) => tenant,
        Err(e) => {
            tracing::error!("Failed to create tenant: {}", e);
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Registration failed",
            ));
        }
    };

    // Create user with tenant association and Owner role
    let user = match state
        .user_repo
        .create(new_user, tenant.id, TenantRole::Owner)
        .await
    {
        Ok(user) => user,
        Err(e) => {
            tracing::error!("Failed to create user: {}", e);
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Registration failed",
            ));
        }
    };

    // Serialize role strings for JWT
    let role_str = serde_json::to_value(&user.role)
        .ok()
        .and_then(|v| v.as_str().map(String::from))
        .unwrap_or_else(|| "user".to_string());
    let tenant_role_str = serde_json::to_value(&user.tenant_role)
        .ok()
        .and_then(|v| v.as_str().map(String::from))
        .unwrap_or_else(|| "owner".to_string());

    // Generate JWT token
    let (token, expires_at) =
        match state
            .jwt
            .generate_token(user.id, &role_str, tenant.id, &tenant_role_str)
        {
            Ok(result) => result,
            Err(e) => {
                tracing::error!("Failed to generate token: {}", e);
                return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                    "INTERNAL_ERROR",
                    "Registration succeeded but token generation failed",
                ));
            }
        };

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

    HttpResponse::Created()
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
                tenant_id: tenant.id.to_string(),
                tenant_role: tenant_role_str,
                email_verified: user.email_verified,
            },
            tenant: TenantInfo {
                id: tenant.id.to_string(),
                name: tenant.name,
                slug: tenant.slug,
                plan: format!("{}", tenant.plan),
            },
        }))
}
