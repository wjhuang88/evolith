//! Auth end-to-end integration tests
//!
//! Tests the full HTTP auth flow through actix-web's test server, including RbacMiddleware JWT validation.

use actix_web::{test, web, App};
use chrono::Utc;
use serde::Deserialize;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::sync::Arc;

use api::middleware::rbac::RbacMiddleware;
use api::routes;
use api::routes::auth;
use api::state::AppState;
use common::execution::{CompositeProvider, ExecutionProvider};
use domain::repository::{
    ApiKeyRepository, AuditRepository, GitRepoRepository, InvitationRepository, NewInvitation,
    SkillRepository, SnippetRepository, TenantRepository, ToolRepository, UserRepository,
};
use infra::config::{
    AppConfig, AppMetaConfig, DatabaseConfig, JwtConfig, LogConfig, RateLimitConfig, RedisConfig,
    SandboxConfig, ServerConfig, SmtpConfig, StorageConfig, StripeConfig,
};
use infra::db::{
    SqliteApiKeyRepository, SqliteAuditRepository, SqliteGitRepoRepository,
    SqliteInvitationRepository, SqliteSkillRepository, SqliteSnippetRepository,
    SqliteTenantRepository, SqliteToolRepository, SqliteUserRepository,
};
use service_auth::{Argon2Hasher, JwtHandler};
use service_skill::executor::{DefaultSkillExecutor, SkillExecutor};
use service_tool::executor::ToolExecutorAdapter;
use service_tool::HttpProxyProvider;
use uuid::Uuid;

const MIGRATION_001: &str = include_str!("../../../migrations/sqlite/001_initial_schema.sql");
const MIGRATION_003: &str = include_str!("../../../migrations/sqlite/003_multi_tenant.sql");
const MIGRATION_004: &str = include_str!("../../../migrations/sqlite/004_user_permissions.sql");
const MIGRATION_005: &str = include_str!("../../../migrations/sqlite/005_payment_integration.sql");
const MIGRATION_006: &str = include_str!("../../../migrations/sqlite/006_skill_cli_data_model.sql");
const MIGRATION_007: &str = include_str!("../../../migrations/sqlite/007_git_repos.sql");
const MIGRATION_008: &str = include_str!("../../../migrations/sqlite/008_git_centric_quotas.sql");
const MIGRATION_009: &str =
    include_str!("../../../migrations/sqlite/009_safe_repo_policy_defaults.sql");

#[derive(Debug, Deserialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<ErrorInfo>,
}

#[derive(Debug, Deserialize)]
struct ErrorInfo {
    code: String,
}

#[derive(Debug, Deserialize)]
struct AuthResponseData {
    token: String,
    user: UserInfo,
    tenant: TenantInfo,
}

#[derive(Debug, Deserialize)]
struct UserInfo {
    id: String,
    email: String,
    username: String,
    role: String,
    tenant_id: String,
    tenant_role: String,
    email_verified: bool,
}

#[derive(Debug, Deserialize)]
struct TenantInfo {
    id: String,
    name: String,
    slug: String,
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    token: String,
    expires_at: i64,
}

#[derive(Debug, Deserialize)]
struct MessageResponse {
    message: String,
}

fn strip_leading_comments(sql: &str) -> &str {
    let mut result = sql;
    for line in sql.lines() {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() || trimmed_line.starts_with("--") {
            let offset = line.len() + 1;
            if result.len() > offset {
                result = &result[offset..];
            } else {
                return "";
            }
        } else {
            break;
        }
    }
    result.trim()
}

async fn setup_test_db() -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .expect("Failed to create in-memory SQLite pool");

    run_migration_sql(&pool, MIGRATION_001).await;
    run_migration_sql(&pool, MIGRATION_003).await;
    run_migration_sql(&pool, MIGRATION_004).await;
    run_migration_sql(&pool, MIGRATION_005).await;
    run_migration_sql(&pool, MIGRATION_006).await;
    run_migration_sql(&pool, MIGRATION_007).await;
    run_migration_sql(&pool, MIGRATION_008).await;
    run_migration_sql(&pool, MIGRATION_009).await;

    pool
}

async fn run_migration_sql(pool: &SqlitePool, sql: &str) {
    for statement in sql.split(';') {
        let trimmed = statement.trim();
        if trimmed.is_empty() {
            continue;
        }
        let without_comments = strip_leading_comments(trimmed);
        if without_comments.is_empty() {
            continue;
        }
        sqlx::query(sqlx::AssertSqlSafe(without_comments))
            .execute(pool)
            .await
            .unwrap_or_else(|e| {
                panic!(
                    "Failed to execute migration statement: {}\nSQL: {}",
                    e, without_comments
                )
            });
    }
}

fn create_test_config() -> AppConfig {
    AppConfig {
        app: AppMetaConfig {
            public_url: "http://localhost:3001".to_string(),
        },
        server: ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
        },
        database: DatabaseConfig {
            database_type: "sqlite".to_string(),
            url: ":memory:".to_string(),
            max_connections: 5,
            seed_database: false,
        },
        redis: RedisConfig {
            url: "redis://localhost:6379".to_string(),
        },
        jwt: JwtConfig {
            secret: "test_secret_key_for_auth_e2e_testing_32chars!!".to_string(),
            expiration: "1h".to_string(),
        },
        storage: StorageConfig {
            endpoint: "localhost:9000".to_string(),
            access_key: "test".to_string(),
            secret_key: "test".to_string(),
            use_ssl: false,
            bucket: "test".to_string(),
        },
        sandbox: SandboxConfig {
            enabled: false,
            timeout_seconds: 30,
            memory_mb: 256,
            cpu_shares: 512,
            pids_limit: 256,
            network_enabled: false,
            max_output_bytes: 10485760,
        },
        log: LogConfig {
            level: "error".to_string(),
        },
        stripe: StripeConfig {
            secret_key: "sk_test_placeholder".to_string(),
            webhook_secret: "whsec_placeholder".to_string(),
            api_version: None,
            test_mode: true,
        },
        smtp: SmtpConfig {
            host: "localhost".to_string(),
            port: 1025,
            username: String::new(),
            password: String::new(),
            from_address: "noreply@evolith.io".to_string(),
            from_name: "Evolith".to_string(),
            enabled: false,
        },
        rate_limit: RateLimitConfig {
            unauthenticated_rpm: 30,
            authenticated_rpm: 300,
            api_key_rpm: 1000,
        },
    }
}

fn build_app_state(pool: SqlitePool) -> AppState {
    let config = create_test_config();
    let http_proxy: Arc<dyn ExecutionProvider> = Arc::new(HttpProxyProvider::new());
    let execution_provider: Arc<dyn ExecutionProvider> =
        Arc::new(CompositeProvider::new(None, Some(http_proxy.clone())));
    AppState {
        config: config.clone(),
        jwt: JwtHandler::new(&config.jwt),
        hasher: Argon2Hasher::new(),
        user_repo: Arc::new(SqliteUserRepository::new(pool.clone())) as Arc<dyn UserRepository>,
        tenant_repo: Arc::new(SqliteTenantRepository::new(pool.clone()))
            as Arc<dyn TenantRepository>,
        tool_repo: Arc::new(SqliteToolRepository::new(pool.clone())) as Arc<dyn ToolRepository>,
        skill_repo: Arc::new(SqliteSkillRepository::new(pool.clone())) as Arc<dyn SkillRepository>,
        snippet_repo: Arc::new(SqliteSnippetRepository::new(pool.clone()))
            as Arc<dyn SnippetRepository>,
        audit_repo: Arc::new(SqliteAuditRepository::new(pool.clone())) as Arc<dyn AuditRepository>,
        invitation_repo: Arc::new(SqliteInvitationRepository::new(pool.clone()))
            as Arc<dyn InvitationRepository>,
        api_key_repo: Arc::new(SqliteApiKeyRepository::new(pool.clone()))
            as Arc<dyn ApiKeyRepository>,
        git_repo_repo: Arc::new(SqliteGitRepoRepository::new(pool)) as Arc<dyn GitRepoRepository>,
        cache: Arc::new(infra::cache::InMemoryCache::new()),
        mailer: Arc::new(infra::mailer::ConsoleMailer),
        execution_provider: execution_provider.clone(),
        skill_executor: Arc::new(DefaultSkillExecutor::new()) as Arc<dyn SkillExecutor>,
        tool_executor: Arc::new(ToolExecutorAdapter::new(execution_provider)),
    }
}

#[actix_rt::test]
async fn test_register_success() {
    let pool = setup_test_db().await;
    let app_state = build_app_state(pool);
    let jwt_secret = app_state.config.jwt.secret.clone();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(RbacMiddleware::new(jwt_secret))
            .service(web::scope("/api/v1").configure(auth::configure)),
    )
    .await;

    let payload = serde_json::json!({
        "email": "test@example.com",
        "username": "testuser",
        "password": "TestPassword123!",
    });

    let req = test::TestRequest::post()
        .uri("/api/v1/auth/register")
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::CREATED);

    let body = test::read_body(resp).await;
    let result: ApiResponse<AuthResponseData> =
        serde_json::from_slice(&body).expect("Failed to parse response");

    assert!(result.success);
    let data = result.data.expect("Expected data in response");

    assert!(!data.token.is_empty());
    assert_eq!(data.user.email, "test@example.com");
    assert_eq!(data.user.username, "testuser");
    assert_eq!(data.user.role, "user");
    assert_eq!(data.user.tenant_role, "owner");
    assert!(!data.user.email_verified);

    assert!(!data.tenant.id.is_empty());
    assert!(!data.tenant.name.is_empty());
    assert!(!data.tenant.slug.is_empty());
}

#[actix_rt::test]
async fn test_register_duplicate_email() {
    let pool = setup_test_db().await;
    let app_state = build_app_state(pool);
    let jwt_secret = app_state.config.jwt.secret.clone();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(RbacMiddleware::new(jwt_secret))
            .service(web::scope("/api/v1").configure(auth::configure)),
    )
    .await;

    let payload = serde_json::json!({
        "email": "duplicate@example.com",
        "username": "firstuser",
        "password": "TestPassword123!",
    });

    let req = test::TestRequest::post()
        .uri("/api/v1/auth/register")
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::CREATED);

    let payload2 = serde_json::json!({
        "email": "duplicate@example.com",
        "username": "seconduser",
        "password": "TestPassword456!",
    });

    let req2 = test::TestRequest::post()
        .uri("/api/v1/auth/register")
        .set_json(&payload2)
        .to_request();

    let resp2 = test::call_service(&app, req2).await;
    assert_eq!(resp2.status(), actix_web::http::StatusCode::CONFLICT);

    let body = test::read_body(resp2).await;
    let result: ApiResponse<AuthResponseData> =
        serde_json::from_slice(&body).expect("Failed to parse response");

    assert!(!result.success);
    let error = result.error.expect("Expected error in response");
    assert_eq!(error.code, "EMAIL_EXISTS");
}

#[actix_rt::test]
async fn test_accept_invitation_existing_email_returns_conflict() {
    let pool = setup_test_db().await;
    let app_state = build_app_state(pool.clone());
    let jwt_secret = app_state.config.jwt.secret.clone();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(RbacMiddleware::new(jwt_secret))
            .configure(routes::configure_routes),
    )
    .await;

    let register_req = test::TestRequest::post()
        .uri("/api/v1/auth/register")
        .set_json(serde_json::json!({
            "email": "invited-existing@example.com",
            "username": "existing",
            "password": "TestPassword123!",
        }))
        .to_request();
    let register_resp = test::call_service(&app, register_req).await;
    assert_eq!(register_resp.status(), actix_web::http::StatusCode::CREATED);
    let register_body = test::read_body(register_resp).await;
    let register_result: ApiResponse<AuthResponseData> =
        serde_json::from_slice(&register_body).expect("Failed to parse register response");
    let auth_data = register_result.data.expect("Expected auth data");

    let invite_token = "inv_existing_email_token";
    let invitation_repo = SqliteInvitationRepository::new(pool);
    invitation_repo
        .create(NewInvitation::new(
            Uuid::parse_str(&auth_data.tenant.id).expect("Invalid tenant UUID"),
            "invited-existing@example.com".to_string(),
            "member".to_string(),
            invite_token.to_string(),
            Utc::now() + chrono::Duration::days(1),
            Uuid::parse_str(&auth_data.user.id).expect("Invalid user UUID"),
        ))
        .await
        .expect("Failed to create invitation");

    let accept_req = test::TestRequest::post()
        .uri("/api/v1/invitations/accept")
        .set_json(serde_json::json!({
            "token": invite_token,
            "username": "duplicate",
            "password": "TestPassword123!",
        }))
        .to_request();
    let accept_resp = test::call_service(&app, accept_req).await;
    assert_eq!(accept_resp.status(), actix_web::http::StatusCode::CONFLICT);
    let accept_body = test::read_body(accept_resp).await;
    let accept_result: ApiResponse<AuthResponseData> =
        serde_json::from_slice(&accept_body).expect("Failed to parse accept response");
    assert_eq!(
        accept_result.error.expect("Expected error").code,
        "EMAIL_EXISTS"
    );
}

#[actix_rt::test]
async fn test_register_weak_password() {
    let pool = setup_test_db().await;
    let app_state = build_app_state(pool);
    let jwt_secret = app_state.config.jwt.secret.clone();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(RbacMiddleware::new(jwt_secret))
            .service(web::scope("/api/v1").configure(auth::configure)),
    )
    .await;

    let payload = serde_json::json!({
        "email": "weak@example.com",
        "username": "weakuser",
        "password": "short",
    });

    let req = test::TestRequest::post()
        .uri("/api/v1/auth/register")
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::BAD_REQUEST);

    let body = test::read_body(resp).await;
    let result: ApiResponse<AuthResponseData> =
        serde_json::from_slice(&body).expect("Failed to parse response");

    assert!(!result.success);
    let error = result.error.expect("Expected error in response");
    assert_eq!(error.code, "VALIDATION_ERROR");
}

#[actix_rt::test]
async fn test_register_invalid_email() {
    let pool = setup_test_db().await;
    let app_state = build_app_state(pool);
    let jwt_secret = app_state.config.jwt.secret.clone();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(RbacMiddleware::new(jwt_secret))
            .service(web::scope("/api/v1").configure(auth::configure)),
    )
    .await;

    let payload = serde_json::json!({
        "email": "notanemail",
        "username": "invaliduser",
        "password": "TestPassword123!",
    });

    let req = test::TestRequest::post()
        .uri("/api/v1/auth/register")
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::BAD_REQUEST);

    let body = test::read_body(resp).await;
    let result: ApiResponse<AuthResponseData> =
        serde_json::from_slice(&body).expect("Failed to parse response");

    assert!(!result.success);
    let error = result.error.expect("Expected error in response");
    assert_eq!(error.code, "VALIDATION_ERROR");
}

#[actix_rt::test]
async fn test_login_success() {
    let pool = setup_test_db().await;
    let app_state = build_app_state(pool);
    let jwt_secret = app_state.config.jwt.secret.clone();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(RbacMiddleware::new(jwt_secret))
            .service(web::scope("/api/v1").configure(auth::configure)),
    )
    .await;

    let register_payload = serde_json::json!({
        "email": "login@example.com",
        "username": "loginuser",
        "password": "TestPassword123!",
    });

    let register_req = test::TestRequest::post()
        .uri("/api/v1/auth/register")
        .set_json(&register_payload)
        .to_request();

    let register_resp = test::call_service(&app, register_req).await;
    assert!(register_resp.status().is_success());
    let register_body = test::read_body(register_resp).await;
    let register_result: ApiResponse<AuthResponseData> =
        serde_json::from_slice(&register_body).expect("Failed to parse register response");
    let auth_data = register_result
        .data
        .expect("Expected data in register response");

    let login_payload = serde_json::json!({
        "email": "login@example.com",
        "password": "TestPassword123!",
    });

    let req = test::TestRequest::post()
        .uri("/api/v1/auth/login")
        .set_json(&login_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

    let body = test::read_body(resp).await;
    let result: ApiResponse<AuthResponseData> =
        serde_json::from_slice(&body).expect("Failed to parse response");

    assert!(result.success);
    let data = result.data.expect("Expected data in response");
    assert!(!data.token.is_empty());
    assert_eq!(data.user.email, "login@example.com");
    assert_eq!(data.user.username, "loginuser");
    assert_eq!(data.user.tenant_id, auth_data.user.tenant_id);
}

#[actix_rt::test]
async fn test_login_wrong_password() {
    let pool = setup_test_db().await;
    let app_state = build_app_state(pool);
    let jwt_secret = app_state.config.jwt.secret.clone();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(RbacMiddleware::new(jwt_secret))
            .service(web::scope("/api/v1").configure(auth::configure)),
    )
    .await;

    let register_payload = serde_json::json!({
        "email": "wrongpass@example.com",
        "username": "wrongpassuser",
        "password": "TestPassword123!",
    });

    let register_req = test::TestRequest::post()
        .uri("/api/v1/auth/register")
        .set_json(&register_payload)
        .to_request();

    let register_resp = test::call_service(&app, register_req).await;
    assert!(register_resp.status().is_success());

    let login_payload = serde_json::json!({
        "email": "wrongpass@example.com",
        "password": "WrongPassword123!",
    });

    let req = test::TestRequest::post()
        .uri("/api/v1/auth/login")
        .set_json(&login_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::UNAUTHORIZED);

    let body = test::read_body(resp).await;
    let result: ApiResponse<AuthResponseData> =
        serde_json::from_slice(&body).expect("Failed to parse response");

    assert!(!result.success);
    let error = result.error.expect("Expected error in response");
    assert_eq!(error.code, "INVALID_CREDENTIALS");
}

#[actix_rt::test]
async fn test_login_nonexistent_user() {
    let pool = setup_test_db().await;
    let app_state = build_app_state(pool);
    let jwt_secret = app_state.config.jwt.secret.clone();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(RbacMiddleware::new(jwt_secret))
            .service(web::scope("/api/v1").configure(auth::configure)),
    )
    .await;

    let payload = serde_json::json!({
        "email": "nonexistent@example.com",
        "password": "TestPassword123!",
    });

    let req = test::TestRequest::post()
        .uri("/api/v1/auth/login")
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::UNAUTHORIZED);

    let body = test::read_body(resp).await;
    let result: ApiResponse<AuthResponseData> =
        serde_json::from_slice(&body).expect("Failed to parse response");

    assert!(!result.success);
    let error = result.error.expect("Expected error in response");
    assert_eq!(error.code, "INVALID_CREDENTIALS");
}

#[actix_rt::test]
async fn test_get_current_user_authenticated() {
    let pool = setup_test_db().await;
    let app_state = build_app_state(pool);
    let jwt_secret = app_state.config.jwt.secret.clone();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(RbacMiddleware::new(jwt_secret))
            .service(web::scope("/api/v1").configure(auth::configure)),
    )
    .await;

    let register_payload = serde_json::json!({
        "email": "me@example.com",
        "username": "meuser",
        "password": "TestPassword123!",
    });

    let register_req = test::TestRequest::post()
        .uri("/api/v1/auth/register")
        .set_json(&register_payload)
        .to_request();

    let register_resp = test::call_service(&app, register_req).await;
    assert!(register_resp.status().is_success());
    let register_body = test::read_body(register_resp).await;
    let register_result: ApiResponse<AuthResponseData> =
        serde_json::from_slice(&register_body).expect("Failed to parse register response");
    let auth_data = register_result
        .data
        .expect("Expected data in register response");
    let token = auth_data.token;

    let req = test::TestRequest::get()
        .uri("/api/v1/auth/me")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

    let body = test::read_body(resp).await;
    let result: ApiResponse<UserInfo> =
        serde_json::from_slice(&body).expect("Failed to parse response");

    assert!(result.success);
    let user = result.data.expect("Expected user data in response");
    assert_eq!(user.email, "me@example.com");
    assert_eq!(user.username, "meuser");
    assert_eq!(user.role, "user");
    assert_eq!(user.tenant_role, "owner");
}

#[actix_rt::test]
async fn test_get_current_user_unauthenticated() {
    let pool = setup_test_db().await;
    let app_state = build_app_state(pool);
    let jwt_secret = app_state.config.jwt.secret.clone();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(RbacMiddleware::new(jwt_secret))
            .service(web::scope("/api/v1").configure(auth::configure)),
    )
    .await;

    let req = test::TestRequest::get().uri("/api/v1/auth/me").to_request();

    let resp = test::try_call_service(&app, req).await;
    assert!(resp.is_err(), "Expected error for unauthenticated request");
}

#[actix_rt::test]
async fn test_refresh_token() {
    let pool = setup_test_db().await;
    let app_state = build_app_state(pool);
    let jwt_secret = app_state.config.jwt.secret.clone();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(RbacMiddleware::new(jwt_secret))
            .service(web::scope("/api/v1").configure(auth::configure)),
    )
    .await;

    let register_payload = serde_json::json!({
        "email": "refresh@example.com",
        "username": "refreshuser",
        "password": "TestPassword123!",
    });

    let register_req = test::TestRequest::post()
        .uri("/api/v1/auth/register")
        .set_json(&register_payload)
        .to_request();

    let register_resp = test::call_service(&app, register_req).await;
    assert!(register_resp.status().is_success());
    let register_body = test::read_body(register_resp).await;
    let register_result: ApiResponse<AuthResponseData> =
        serde_json::from_slice(&register_body).expect("Failed to parse register response");
    let auth_data = register_result
        .data
        .expect("Expected data in register response");
    let token = auth_data.token;

    let req = test::TestRequest::post()
        .uri("/api/v1/auth/refresh")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

    let body = test::read_body(resp).await;
    let result: ApiResponse<TokenResponse> =
        serde_json::from_slice(&body).expect("Failed to parse response");

    assert!(result.success);
    let data = result.data.expect("Expected token data in response");
    assert!(!data.token.is_empty());
    assert!(data.expires_at > 0);
}

#[actix_rt::test]
async fn test_update_profile() {
    let pool = setup_test_db().await;
    let app_state = build_app_state(pool);
    let jwt_secret = app_state.config.jwt.secret.clone();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(RbacMiddleware::new(jwt_secret))
            .service(web::scope("/api/v1").configure(auth::configure)),
    )
    .await;

    let register_payload = serde_json::json!({
        "email": "profile@example.com",
        "username": "profileuser",
        "password": "TestPassword123!",
    });

    let register_req = test::TestRequest::post()
        .uri("/api/v1/auth/register")
        .set_json(&register_payload)
        .to_request();

    let register_resp = test::call_service(&app, register_req).await;
    assert!(register_resp.status().is_success());
    let register_body = test::read_body(register_resp).await;
    let register_result: ApiResponse<AuthResponseData> =
        serde_json::from_slice(&register_body).expect("Failed to parse register response");
    let auth_data = register_result
        .data
        .expect("Expected data in register response");
    let token = auth_data.token;

    let payload = serde_json::json!({
        "username": "newusername",
    });

    let req = test::TestRequest::patch()
        .uri("/api/v1/auth/profile")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

    let body = test::read_body(resp).await;
    let result: ApiResponse<UserInfo> =
        serde_json::from_slice(&body).expect("Failed to parse response");

    assert!(result.success);
    let user = result.data.expect("Expected user data in response");
    assert_eq!(user.username, "newusername");
    assert_eq!(user.email, "profile@example.com");
}

#[actix_rt::test]
async fn test_logout() {
    let pool = setup_test_db().await;
    let app_state = build_app_state(pool);
    let jwt_secret = app_state.config.jwt.secret.clone();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(RbacMiddleware::new(jwt_secret))
            .service(web::scope("/api/v1").configure(auth::configure)),
    )
    .await;

    let register_payload = serde_json::json!({
        "email": "logout@example.com",
        "username": "logoutuser",
        "password": "TestPassword123!",
    });

    let register_req = test::TestRequest::post()
        .uri("/api/v1/auth/register")
        .set_json(&register_payload)
        .to_request();

    let register_resp = test::call_service(&app, register_req).await;
    assert!(register_resp.status().is_success());
    let register_body = test::read_body(register_resp).await;
    let register_result: ApiResponse<AuthResponseData> =
        serde_json::from_slice(&register_body).expect("Failed to parse register response");
    let auth_data = register_result
        .data
        .expect("Expected data in register response");
    let token = auth_data.token;

    let req = test::TestRequest::post()
        .uri("/api/v1/auth/logout")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

    let body = test::read_body(resp).await;
    let result: ApiResponse<MessageResponse> =
        serde_json::from_slice(&body).expect("Failed to parse response");

    assert!(result.success);
    let data = result.data.expect("Expected message data in response");
    assert!(data.message.contains("Logged out") || data.message.contains("success"));
}

#[actix_rt::test]
async fn test_update_skill_success() {
    let pool = setup_test_db().await;
    let app_state = build_app_state(pool);
    let jwt_secret = app_state.config.jwt.secret.clone();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(RbacMiddleware::new(jwt_secret))
            .configure(routes::configure_routes),
    )
    .await;

    let register_req = test::TestRequest::post()
        .uri("/api/v1/auth/register")
        .set_json(serde_json::json!({
            "email": "skill-update@example.com",
            "username": "skillupdater",
            "password": "TestPassword123!",
        }))
        .to_request();
    let register_resp = test::call_service(&app, register_req).await;
    assert_eq!(register_resp.status(), actix_web::http::StatusCode::CREATED);
    let register_body = test::read_body(register_resp).await;
    let register_result: ApiResponse<AuthResponseData> =
        serde_json::from_slice(&register_body).expect("Failed to parse register response");
    let auth_data = register_result.data.expect("Expected auth data");
    let token = auth_data.token;

    let create_req = test::TestRequest::post()
        .uri("/api/v1/skills")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "name": "Test Skill",
            "version": "1.0.0",
            "description": "Original description",
            "content": "# Original content",
            "runtime": "python311",
            "dependencies": [],
            "is_public": false,
        }))
        .to_request();
    let create_resp = test::call_service(&app, create_req).await;
    assert_eq!(create_resp.status(), actix_web::http::StatusCode::CREATED);
    let create_body = test::read_body(create_resp).await;
    let create_result: ApiResponse<serde_json::Value> =
        serde_json::from_slice(&create_body).expect("Failed to parse create response");
    let skill_data = create_result.data.expect("Expected skill data");
    let skill_id = skill_data["id"].as_str().expect("Expected skill id");

    let update_req = test::TestRequest::put()
        .uri(&format!("/api/v1/skills/{}", skill_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "description": "Updated description",
            "content": "# Updated content",
        }))
        .to_request();
    let update_resp = test::call_service(&app, update_req).await;
    assert_eq!(update_resp.status(), actix_web::http::StatusCode::OK);

    let update_body = test::read_body(update_resp).await;
    let update_result: ApiResponse<serde_json::Value> =
        serde_json::from_slice(&update_body).expect("Failed to parse update response");
    assert!(update_result.success);
    let updated_skill = update_result.data.expect("Expected updated skill data");
    assert_eq!(updated_skill["description"], "Updated description");
    assert_eq!(updated_skill["content"], "# Updated content");
    assert_eq!(updated_skill["name"], "Test Skill");
}

#[actix_rt::test]
async fn test_update_skill_not_found() {
    let pool = setup_test_db().await;
    let app_state = build_app_state(pool);
    let jwt_secret = app_state.config.jwt.secret.clone();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(RbacMiddleware::new(jwt_secret))
            .configure(routes::configure_routes),
    )
    .await;

    let register_req = test::TestRequest::post()
        .uri("/api/v1/auth/register")
        .set_json(serde_json::json!({
            "email": "skill-404@example.com",
            "username": "skill404user",
            "password": "TestPassword123!",
        }))
        .to_request();
    let register_resp = test::call_service(&app, register_req).await;
    assert_eq!(register_resp.status(), actix_web::http::StatusCode::CREATED);
    let register_body = test::read_body(register_resp).await;
    let register_result: ApiResponse<AuthResponseData> =
        serde_json::from_slice(&register_body).expect("Failed to parse register response");
    let auth_data = register_result.data.expect("Expected auth data");
    let token = auth_data.token;

    let fake_id = Uuid::new_v4();
    let update_req = test::TestRequest::put()
        .uri(&format!("/api/v1/skills/{}", fake_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "description": "Should fail",
        }))
        .to_request();
    let update_resp = test::call_service(&app, update_req).await;
    assert_eq!(update_resp.status(), actix_web::http::StatusCode::NOT_FOUND);
}

#[actix_rt::test]
async fn test_send_verification_email_returns_success() {
    let pool = setup_test_db().await;
    let app_state = build_app_state(pool);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(RbacMiddleware::new("test_secret".to_string()))
            .service(web::scope("/api/v1").configure(auth::configure)),
    )
    .await;

    let register_payload = serde_json::json!({
        "email": "verify-send@example.com",
        "username": "verifysenduser",
        "password": "TestPassword123!",
    });
    let register_req = test::TestRequest::post()
        .uri("/api/v1/auth/register")
        .set_json(&register_payload)
        .to_request();
    let register_resp = test::call_service(&app, register_req).await;
    assert_eq!(register_resp.status(), actix_web::http::StatusCode::CREATED);

    let req = test::TestRequest::post()
        .uri("/api/v1/auth/send-verify")
        .set_json(serde_json::json!({
            "email": "verify-send@example.com",
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

    let body = test::read_body(resp).await;
    let result: ApiResponse<MessageResponse> =
        serde_json::from_slice(&body).expect("Failed to parse response");
    assert!(result.success);
}

#[actix_rt::test]
async fn test_verify_email_with_valid_token() {
    let pool = setup_test_db().await;
    let state = build_app_state(pool.clone());

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .wrap(RbacMiddleware::new("test_secret".to_string()))
            .service(web::scope("/api/v1").configure(auth::configure)),
    )
    .await;

    let register_payload = serde_json::json!({
        "email": "verify-valid@example.com",
        "username": "verifyvaliduser",
        "password": "TestPassword123!",
    });
    let register_req = test::TestRequest::post()
        .uri("/api/v1/auth/register")
        .set_json(&register_payload)
        .to_request();
    let register_resp = test::call_service(&app, register_req).await;
    assert_eq!(register_resp.status(), actix_web::http::StatusCode::CREATED);
    let register_body = test::read_body(register_resp).await;
    let register_result: ApiResponse<AuthResponseData> =
        serde_json::from_slice(&register_body).expect("Failed to parse register response");
    let auth_data = register_result.data.expect("Expected auth data");
    let user_id = Uuid::parse_str(&auth_data.user.id).expect("Invalid user UUID");

    let test_token = "evt_test_valid_token_12345";
    let user_repo = SqliteUserRepository::new(pool);
    user_repo
        .set_verify_token(user_id, test_token)
        .await
        .expect("Failed to set verify token");

    let req = test::TestRequest::post()
        .uri("/api/v1/auth/verify-email")
        .set_json(serde_json::json!({
            "token": test_token,
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

    let body = test::read_body(resp).await;
    let result: ApiResponse<MessageResponse> =
        serde_json::from_slice(&body).expect("Failed to parse response");
    assert!(result.success);
}

#[actix_rt::test]
async fn test_verify_email_with_invalid_token() {
    let pool = setup_test_db().await;
    let app_state = build_app_state(pool);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(RbacMiddleware::new("test_secret".to_string()))
            .service(web::scope("/api/v1").configure(auth::configure)),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/v1/auth/verify-email")
        .set_json(serde_json::json!({
            "token": "evt_bogus_token_that_does_not_exist",
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::BAD_REQUEST);

    let body = test::read_body(resp).await;
    let result: ApiResponse<MessageResponse> =
        serde_json::from_slice(&body).expect("Failed to parse response");
    assert!(!result.success);
    let error = result.error.expect("Expected error in response");
    assert_eq!(error.code, "INVALID_TOKEN");
}
