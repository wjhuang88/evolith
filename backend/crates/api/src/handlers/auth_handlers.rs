//! Auth handlers

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::dto::common::ApiResponse;
use infra::config::JwtConfig;
use service_auth::JwtHandler;

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

    /// Optional tenant name for new registration
    pub tenant_name: Option<String>,

    /// Optional tenant slug for new registration
    pub tenant_slug: Option<String>,
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
}

/// In-memory user store for development
pub struct UserStore {
    users: Mutex<HashMap<String, StoredUser>>,
    tenants: Mutex<HashMap<String, StoredTenant>>,
}

#[derive(Clone)]
pub struct StoredUser {
    pub id: String,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub role: String,
    pub tenant_id: String,
    pub tenant_role: String,
    pub created_at: i64,
}

#[derive(Clone)]
pub struct StoredTenant {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub plan: String,
    pub owner_id: String,
    pub created_at: i64,
}

impl Default for UserStore {
    fn default() -> Self {
        let mut store = Self {
            users: Mutex::new(HashMap::new()),
            tenants: Mutex::new(HashMap::new()),
        };

        // Create default tenant
        let default_tenant = StoredTenant {
            id: "00000000-0000-0000-0000-000000000001".to_string(),
            name: "Default Tenant".to_string(),
            slug: "default".to_string(),
            plan: "pro".to_string(),
            owner_id: "00000000-0000-0000-0000-000000000001".to_string(),
            created_at: Utc::now().timestamp(),
        };

        store
            .tenants
            .lock()
            .unwrap()
            .insert(default_tenant.slug.clone(), default_tenant);

        // Create demo user (password: demo123!)
        // Password hash for "demo123!" using simple hash for demo
        let demo_user = StoredUser {
            id: "00000000-0000-0000-0000-000000000001".to_string(),
            email: "demo@evolith.io".to_string(),
            username: "demo".to_string(),
            // Simple hash for demo - in production use proper hashing
            password_hash: "demo123!".to_string(),
            role: "admin".to_string(),
            tenant_id: "00000000-0000-0000-0000-000000000001".to_string(),
            tenant_role: "owner".to_string(),
            created_at: Utc::now().timestamp(),
        };

        store
            .users
            .lock()
            .unwrap()
            .insert(demo_user.email.clone(), demo_user);

        store
    }
}

/// App state for auth
#[derive(Clone)]
pub struct AuthState {
    pub jwt: JwtHandler,
    pub user_store: Arc<UserStore>,
}

impl AuthState {
    pub fn new(jwt_config: &JwtConfig) -> Self {
        Self {
            jwt: JwtHandler::new(jwt_config),
            user_store: Arc::new(UserStore::default()),
        }
    }
}

/// Simple password verification (for demo purposes)
fn verify_password(password: &str, hash: &str) -> bool {
    // For demo accounts, accept the plaintext password
    if hash == "demo123!" {
        return password == "demo123!";
    }
    // In production, use proper Argon2 verification
    // For now, simple comparison for demo
    password == hash
}

/// Login handler
pub async fn login(body: web::Json<LoginRequest>, state: web::Data<AuthState>) -> impl Responder {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "VALIDATION_ERROR",
            &errors.to_string(),
        ));
    }

    let email = body.email.to_lowercase();
    let password = body.password.clone();

    // Find user
    let user = {
        let users = state.user_store.users.lock().unwrap();
        users.get(&email).cloned()
    };

    // Verify password
    let valid = if let Some(ref user) = user {
        verify_password(&password, &user.password_hash)
    } else {
        false
    };

    if !valid {
        return HttpResponse::Unauthorized().json(ApiResponse::<()>::error(
            "INVALID_CREDENTIALS",
            "Invalid email or password",
        ));
    }

    let user = user.unwrap();

    // Get tenant info
    let tenant = {
        let tenants = state.user_store.tenants.lock().unwrap();
        tenants.get("default").cloned().unwrap_or(StoredTenant {
            id: "00000000-0000-0000-0000-000000000001".to_string(),
            name: "Default Tenant".to_string(),
            slug: "default".to_string(),
            plan: "pro".to_string(),
            owner_id: user.id.clone(),
            created_at: Utc::now().timestamp(),
        })
    };

    // Generate token
    let tenant_id = Uuid::parse_str(&tenant.id).unwrap_or(Uuid::nil());
    let (token, expires_at) = match state.jwt.generate_token(
        Uuid::parse_str(&user.id).unwrap_or(Uuid::nil()),
        &user.role,
        tenant_id,
        &user.tenant_role,
    ) {
        Ok(result) => result,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "TOKEN_ERROR",
                &format!("Failed to generate token: {}", e),
            ));
        }
    };

    let response = AuthResponseData {
        token,
        expires_at,
        user: UserInfo {
            id: user.id,
            email: user.email,
            username: user.username,
            role: user.role,
            tenant_id: user.tenant_id,
            tenant_role: user.tenant_role,
        },
        tenant: TenantInfo {
            id: tenant.id,
            name: tenant.name,
            slug: tenant.slug,
            plan: tenant.plan,
        },
    };

    HttpResponse::Ok().json(ApiResponse::<AuthResponseData>::success(response))
}

/// Register handler
pub async fn register(
    body: web::Json<RegisterRequest>,
    state: web::Data<AuthState>,
) -> impl Responder {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "VALIDATION_ERROR",
            &errors.to_string(),
        ));
    }

    let email = body.email.to_lowercase();

    // Check if user exists
    {
        let users = state.user_store.users.lock().unwrap();
        if users.contains_key(&email) {
            return HttpResponse::Conflict().json(ApiResponse::<()>::error(
                "USER_EXISTS",
                "Email already registered",
            ));
        }
    }

    // Create tenant if provided
    let (tenant_id, tenant_slug, tenant_name, tenant_plan) =
        if let Some(ref slug) = body.tenant_slug {
            let tid = Uuid::new_v4().to_string();
            let name = body.tenant_name.clone().unwrap_or_else(|| slug.clone());

            let tenant = StoredTenant {
                id: tid.clone(),
                name: name.clone(),
                slug: slug.clone(),
                plan: "free".to_string(),
                owner_id: String::new(),
                created_at: Utc::now().timestamp(),
            };

            let mut tenants = state.user_store.tenants.lock().unwrap();
            tenants.insert(slug.clone(), tenant);

            (tid, slug.clone(), name, "free".to_string())
        } else {
            (
                "00000000-0000-0000-0000-000000000001".to_string(),
                "default".to_string(),
                "Default Tenant".to_string(),
                "pro".to_string(),
            )
        };

    // For demo, store password as-is (in production, use proper hashing)
    let password_hash = body.password.clone();

    // Create user
    let user_id = Uuid::new_v4().to_string();
    let user = StoredUser {
        id: user_id.clone(),
        email: email.clone(),
        username: body.username.clone(),
        password_hash,
        role: "user".to_string(),
        tenant_id: tenant_id.clone(),
        tenant_role: if tenant_slug == "default" {
            "member".to_string()
        } else {
            "owner".to_string()
        },
        created_at: Utc::now().timestamp(),
    };

    // Store user
    {
        let mut users = state.user_store.users.lock().unwrap();
        users.insert(email.clone(), user.clone());
    }

    // Generate token
    let tenant_uuid = Uuid::parse_str(&tenant_id).unwrap_or(Uuid::nil());
    let (token, expires_at) = match state.jwt.generate_token(
        Uuid::parse_str(&user.id).unwrap_or(Uuid::nil()),
        &user.role,
        tenant_uuid,
        &user.tenant_role,
    ) {
        Ok(result) => result,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "TOKEN_ERROR",
                &format!("Failed to generate token: {}", e),
            ));
        }
    };

    let response = AuthResponseData {
        token,
        expires_at,
        user: UserInfo {
            id: user.id,
            email: user.email,
            username: user.username,
            role: user.role,
            tenant_id: user.tenant_id,
            tenant_role: user.tenant_role,
        },
        tenant: TenantInfo {
            id: tenant_id,
            name: tenant_name,
            slug: tenant_slug,
            plan: tenant_plan,
        },
    };

    HttpResponse::Ok().json(ApiResponse::<AuthResponseData>::success(response))
}

/// Logout handler
pub async fn logout() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::<serde_json::Value>::success(
        serde_json::json!({ "message": "Logged out successfully" }),
    ))
}

/// Refresh token handler
pub async fn refresh_token(_state: web::Data<AuthState>) -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::<()>::error(
        "NOT_IMPLEMENTED",
        "Token refresh not implemented yet",
    ))
}

/// Get current user handler
pub async fn get_current_user(_state: web::Data<AuthState>) -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::<()>::error(
        "NOT_IMPLEMENTED",
        "Get current user not implemented yet",
    ))
}
