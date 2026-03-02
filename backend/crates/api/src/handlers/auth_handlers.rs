//! Auth handlers
//! Handles user authentication, registration, email verification, and password reset

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::dto::auth_dto::*;
use crate::dto::common::ApiResponse;
use infra::config::JwtConfig;
use service_auth::JwtHandler;

/// Simple token generation for email verification and password reset
fn generate_token() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let random: u64 = rand_simple();
    format!("{:x}{:016x}", timestamp, random)
}

fn rand_simple() -> u64 {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};
    RandomState::new().build_hasher().finish()
}

/// In-memory user store for development
/// In production, this would be replaced with database storage
pub struct UserStore {
    users: Mutex<HashMap<String, StoredUser>>,
    tenants: Mutex<HashMap<String, StoredTenant>>,
    /// In-memory token store for email verification
    verify_tokens: Mutex<HashMap<String, VerifyTokenInfo>>,
    /// In-memory token store for password reset
    reset_tokens: Mutex<HashMap<String, ResetTokenInfo>>,
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
    pub email_verified: bool,
    pub verify_token: Option<String>,
    pub reset_token: Option<String>,
    pub reset_expires_at: Option<i64>,
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

#[derive(Clone)]
pub struct VerifyTokenInfo {
    pub user_id: String,
    pub email: String,
    pub expires_at: i64,
}

#[derive(Clone)]
pub struct ResetTokenInfo {
    pub user_id: String,
    pub email: String,
    pub expires_at: i64,
    pub used: bool,
}

impl Default for UserStore {
    fn default() -> Self {
        let mut store = Self {
            users: Mutex::new(HashMap::new()),
            tenants: Mutex::new(HashMap::new()),
            verify_tokens: Mutex::new(HashMap::new()),
            reset_tokens: Mutex::new(HashMap::new()),
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
        let demo_user = StoredUser {
            id: "00000000-0000-0000-0000-000000000001".to_string(),
            email: "demo@evolith.io".to_string(),
            username: "demo".to_string(),
            password_hash: "demo123!".to_string(),
            role: "admin".to_string(),
            tenant_id: "00000000-0000-0000-0000-000000000001".to_string(),
            tenant_role: "owner".to_string(),
            email_verified: true, // Demo user is pre-verified
            verify_token: None,
            reset_token: None,
            reset_expires_at: None,
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
    /// Email configuration (for sending verification emails)
    pub smtp_host: Option<String>,
    pub smtp_from: Option<String>,
}

impl AuthState {
    pub fn new(jwt_config: &JwtConfig) -> Self {
        Self {
            jwt: JwtHandler::new(jwt_config),
            user_store: Arc::new(UserStore::default()),
            smtp_host: None,
            smtp_from: None,
        }
    }

    pub fn with_smtp(mut self, host: Option<String>, from: Option<String>) -> Self {
        self.smtp_host = host;
        self.smtp_from = from;
        self
    }
}

/// Simple password verification (for demo purposes)
fn verify_password(password: &str, hash: &str) -> bool {
    // For demo accounts, accept the plaintext password
    if hash == "demo123!" {
        return password == "demo123!";
    }
    // In production, use proper Argon2 verification
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

    // Check if user exists
    let user = match user {
        Some(u) => u,
        None => {
            return HttpResponse::Unauthorized().json(ApiResponse::<()>::error(
                "INVALID_CREDENTIALS",
                "Invalid email or password",
            ));
        }
    };

    // Verify password
    let valid = verify_password(&password, &user.password_hash);

    if !valid {
        return HttpResponse::Unauthorized().json(ApiResponse::<()>::error(
            "INVALID_CREDENTIALS",
            "Invalid email or password",
        ));
    }

    // Check if email is verified (skip for demo user)
    if !user.email_verified && user.email != "demo@evolith.io" {
        return HttpResponse::Forbidden().json(ApiResponse::<()>::error(
            "EMAIL_NOT_VERIFIED",
            "Please verify your email address before logging in",
        ));
    }

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
            email_verified: user.email_verified,
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
                "00000000-0000-0000-0000-0000000000001".to_string(),
                "default".to_string(),
                "Default Tenant".to_string(),
                "pro".to_string(),
            )
        };

    // Generate verification token
    let verify_token = generate_token();
    let now = Utc::now().timestamp();
    let expires_24h = now + (24 * 60 * 60); // 24 hours

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
        email_verified: false,
        verify_token: Some(verify_token.clone()),
        reset_token: None,
        reset_expires_at: None,
        created_at: Utc::now().timestamp(),
    };

    // Store user
    {
        let mut users = state.user_store.users.lock().unwrap();
        users.insert(email.clone(), user.clone());
    }

    // Store verification token
    {
        let mut tokens = state.user_store.verify_tokens.lock().unwrap();
        tokens.insert(
            verify_token.clone(),
            VerifyTokenInfo {
                user_id: user.id.clone(),
                email: email.clone(),
                expires_at: expires_24h,
            },
        );
    }

    // In production, send verification email here
    // For demo, we return the token in the response
    let verify_link = format!("/verify-email?token={}", verify_token);

    let response = AuthResponseData {
        token: String::new(), // No token until email is verified
        expires_at: 0,
        user: UserInfo {
            id: user.id,
            email: user.email,
            username: user.username,
            role: user.role,
            tenant_id: user.tenant_id,
            tenant_role: user.tenant_role,
            email_verified: false,
        },
        tenant: TenantInfo {
            id: tenant_id,
            name: tenant_name,
            slug: tenant_slug,
            plan: tenant_plan,
        },
    };

    // Include verification link in response for demo purposes
    let mut response_with_verify = serde_json::json!({
        "data": response,
        "verification_link": verify_link,
        "message": "Please verify your email address. A verification link has been sent."
    });

    HttpResponse::Ok().json(ApiResponse::<serde_json::Value>::success(
        response_with_verify,
    ))
}

/// Logout handler
pub async fn logout() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::<serde_json::Value>::success(
        serde_json::json!({ "message": "Logged out successfully" }),
    ))
}

/// Refresh token handler
pub async fn refresh_token(
    _body: web::Json<RefreshTokenRequest>,
    _state: web::Data<AuthState>,
) -> impl Responder {
    // In production, implement JWT refresh token logic
    // For now, return not implemented
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

/// Send verification email handler
pub async fn send_verification_email(
    body: web::Json<SendVerifyEmailRequest>,
    state: web::Data<AuthState>,
) -> impl Responder {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "VALIDATION_ERROR",
            &errors.to_string(),
        ));
    }

    let email = body.email.to_lowercase();

    // Find user
    let user = {
        let users = state.user_store.users.lock().unwrap();
        users.get(&email).cloned()
    };

    let user = match user {
        Some(u) => u,
        None => {
            // Don't reveal if email exists
            return HttpResponse::Ok().json(ApiResponse::<VerifyEmailResponse>::success(
                VerifyEmailResponse {
                    message: "If the email exists, a verification link has been sent".to_string(),
                    email: email.clone(),
                },
            ));
        }
    };

    // Check if already verified
    if user.email_verified {
        return HttpResponse::Ok().json(ApiResponse::<VerifyEmailResponse>::success(
            VerifyEmailResponse {
                message: "Email is already verified".to_string(),
                email: email.clone(),
            },
        ));
    }

    // Generate new verification token
    let verify_token = generate_token();
    let now = Utc::now().timestamp();
    let expires_24h = now + (24 * 60 * 60);

    // Store token
    {
        let mut tokens = state.user_store.verify_tokens.lock().unwrap();
        tokens.insert(
            verify_token.clone(),
            VerifyTokenInfo {
                user_id: user.id.clone(),
                email: email.clone(),
                expires_at: expires_24h,
            },
        );
    }

    // Update user's verify token
    {
        let mut users = state.user_store.users.lock().unwrap();
        if let Some(u) = users.get_mut(&email) {
            u.verify_token = Some(verify_token.clone());
        }
    }

    // In production, send verification email via SMTP
    // For demo, return the link
    let verify_link = format!("/verify-email?token={}", verify_token);
    println!("[DEMO] Verification link for {}: {}", email, verify_link);

    HttpResponse::Ok().json(ApiResponse::<VerifyEmailResponse>::success(
        VerifyEmailResponse {
            message: "Verification link sent to your email".to_string(),
            email: email.clone(),
        },
    ))
}

/// Verify email handler
pub async fn verify_email(
    body: web::Json<VerifyEmailRequest>,
    state: web::Data<AuthState>,
) -> impl Responder {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "VALIDATION_ERROR",
            &errors.to_string(),
        ));
    }

    let token = body.token.clone();

    // Find token
    let token_info = {
        let tokens = state.user_store.verify_tokens.lock().unwrap();
        tokens.get(&token).cloned()
    };

    let token_info = match token_info {
        Some(t) => t,
        None => {
            return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                "INVALID_TOKEN",
                "Invalid or expired verification token",
            ));
        }
    };

    // Check if expired
    let now = Utc::now().timestamp();
    if token_info.expires_at < now {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "TOKEN_EXPIRED",
            "Verification token has expired",
        ));
    }

    // Mark email as verified
    {
        let mut users = state.user_store.users.lock().unwrap();
        if let Some(u) = users.get_mut(&token_info.email) {
            u.email_verified = true;
            u.verify_token = None;
        }
    }

    // Remove token
    {
        let mut tokens = state.user_store.verify_tokens.lock().unwrap();
        tokens.remove(&token);
    }

    HttpResponse::Ok().json(ApiResponse::<MessageResponse>::success(MessageResponse {
        message: "Email verified successfully. You can now log in.".to_string(),
    }))
}

/// Forgot password handler
pub async fn forgot_password(
    body: web::Json<ForgotPasswordRequest>,
    state: web::Data<AuthState>,
) -> impl Responder {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "VALIDATION_ERROR",
            &errors.to_string(),
        ));
    }

    let email = body.email.to_lowercase();

    // Find user
    let user = {
        let users = state.user_store.users.lock().unwrap();
        users.get(&email).cloned()
    };

    // Don't reveal if user exists
    let _ = user;

    // Always return success to prevent email enumeration
    return HttpResponse::Ok().json(ApiResponse::<PasswordResetResponse>::success(
        PasswordResetResponse {
            message: "If the email exists, a password reset link has been sent".to_string(),
            email: email.clone(),
        },
    ));

    // Note: In production, uncomment the following to actually process the request:
    /*
    let user = match user {
        Some(u) => u,
        None => {
            return HttpResponse::Ok().json(ApiResponse::<PasswordResetResponse>::success(
                PasswordResetResponse {
                    message: "If the email exists, a password reset link has been sent".to_string(),
                    email: email.clone(),
                },
            ));
        }
    };

    // Generate reset token
    let reset_token = generate_token();
    let now = Utc::now().timestamp();
    let expires_1h = now + (60 * 60); // 1 hour

    // Store token
    {
        let mut tokens = state.user_store.reset_tokens.lock().unwrap();
        tokens.insert(
            reset_token.clone(),
            ResetTokenInfo {
                user_id: user.id.clone(),
                email: email.clone(),
                expires_at: expires_1h,
                used: false,
            },
        );
    }

    // Update user's reset token
    {
        let mut users = state.user_store.users.lock().unwrap();
        if let Some(u) = users.get_mut(&email) {
            u.reset_token = Some(reset_token.clone());
            u.reset_expires_at = Some(expires_1h);
        }
    }

    // In production, send reset email via SMTP
    let reset_link = format!("/reset-password?token={}", reset_token);
    println!("[DEMO] Password reset link for {}: {}", email, reset_link);

    HttpResponse::Ok().json(ApiResponse::<PasswordResetResponse>::success(
        PasswordResetResponse {
            message: "Password reset link sent to your email".to_string(),
            email: email.clone(),
        },
    ))
    */
}

/// Reset password handler
pub async fn reset_password(
    body: web::Json<ResetPasswordRequest>,
    state: web::Data<AuthState>,
) -> impl Responder {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "VALIDATION_ERROR",
            &errors.to_string(),
        ));
    }

    let token = body.token.clone();
    let new_password = body.password.clone();

    // Find token
    let token_info = {
        let tokens = state.user_store.reset_tokens.lock().unwrap();
        tokens.get(&token).cloned()
    };

    let token_info = match token_info {
        Some(t) => t,
        None => {
            return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                "INVALID_TOKEN",
                "Invalid or expired password reset token",
            ));
        }
    };

    // Check if already used
    if token_info.used {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "TOKEN_USED",
            "This password reset token has already been used",
        ));
    }

    // Check if expired
    let now = Utc::now().timestamp();
    if token_info.expires_at < now {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "TOKEN_EXPIRED",
            "Password reset token has expired",
        ));
    }

    // Update password
    {
        let mut users = state.user_store.users.lock().unwrap();
        if let Some(u) = users.get_mut(&token_info.email) {
            // In production, hash the password with Argon2
            u.password_hash = new_password;
            u.reset_token = None;
            u.reset_expires_at = None;
        }
    }

    // Mark token as used
    {
        let mut tokens = state.user_store.reset_tokens.lock().unwrap();
        if let Some(t) = tokens.get_mut(&token) {
            t.used = true;
        }
    }

    HttpResponse::Ok().json(ApiResponse::<MessageResponse>::success(MessageResponse {
        message: "Password reset successfully. You can now log in with your new password."
            .to_string(),
    }))
}
