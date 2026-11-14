//! Auth end-to-end integration tests
//!
//! Tests the full HTTP auth flow through actix-web's test server, including RbacMiddleware JWT validation.

use actix_web::{middleware::from_fn, test, web, App};
use chrono::Utc;
use serde::Deserialize;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::sync::Arc;

use api::routes;
use api::routes::auth;
use api::state::AppState;
use common::execution::{CompositeProvider, ExecutionProvider};
use domain::repository::{
    ApiKeyRepository, AuditRepository, GitRepoRepository, InvitationRepository, NewInvitation,
    SkillRepository, SnippetRepository, TenantRepository, ToolRepository, UserRepository,
};
use infra::config::{
    AppConfig, AppMetaConfig, DatabaseConfig, GitStorageConfig, JwtConfig, LogConfig,
    RateLimitConfig, RedisConfig, SandboxConfig, ServerConfig, SmtpConfig, StorageConfig,
    StripeConfig,
};
use infra::db::{
    SqliteApiKeyRepository, SqliteAuditRepository, SqliteGitRepoRepository,
    SqliteInvitationRepository, SqliteOutboxRepository, SqliteSkillRepository,
    SqliteSnippetRepository, SqliteTenantRepository, SqliteToolRepository, SqliteUserRepository,
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
const MIGRATION_010: &str = include_str!("../../../migrations/sqlite/010_repo_lifecycle.sql");

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
    run_migration_sql(&pool, MIGRATION_010).await;

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
        git_storage: GitStorageConfig {
            base_path: "/tmp/evolith-test-repos".to_string(),
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
        git_repo_repo: Arc::new(SqliteGitRepoRepository::new(pool.clone()))
            as Arc<dyn GitRepoRepository>,
        outbox_repo: Arc::new(SqliteOutboxRepository::new(pool)),
        cache: Arc::new(infra::cache::InMemoryCache::new()),
        mailer: Arc::new(infra::mailer::ConsoleMailer),
        execution_provider: execution_provider.clone(),
        skill_executor: Arc::new(DefaultSkillExecutor::new()) as Arc<dyn SkillExecutor>,
        tool_executor: Arc::new(ToolExecutorAdapter::new(execution_provider)),
        git_storage_base_path: "/tmp/evolith-test-repos".to_string(),
    }
}

#[actix_rt::test]
async fn test_register_success() {
    let pool = setup_test_db().await;
    let app_state = build_app_state(pool);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
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

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
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

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
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

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
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

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
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

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
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

    // Guard the REST cookie contract during login business-logic extraction.
    let set_cookies = resp
        .headers()
        .get_all(actix_web::http::header::SET_COOKIE)
        .into_iter()
        .map(|value| value.to_str().expect("valid Set-Cookie header").to_owned())
        .collect::<Vec<_>>();
    assert_eq!(set_cookies.len(), 2);
    let session_cookie = set_cookies
        .iter()
        .find(|value| value.starts_with("evolith_token="))
        .expect("httpOnly JWT cookie");
    assert!(session_cookie.contains("HttpOnly"));
    assert!(session_cookie.contains("SameSite=Lax"));
    assert!(session_cookie.contains("Path=/"));
    assert!(session_cookie.contains("Max-Age="));
    let csrf_cookie = set_cookies
        .iter()
        .find(|value| value.starts_with("csrf_token="))
        .expect("JS-readable double-submit CSRF cookie");
    assert!(!csrf_cookie.contains("HttpOnly"));
    assert!(csrf_cookie.contains("SameSite=Lax"));
    assert!(csrf_cookie.contains("Path=/"));
    assert!(csrf_cookie.contains("Max-Age="));

    let body = test::read_body(resp).await;
    let result: ApiResponse<AuthResponseData> =
        serde_json::from_slice(&body).expect("Failed to parse response");

    assert!(result.success);
    let data = result.data.expect("Expected data in response");
    assert!(!data.token.is_empty());
    assert_eq!(data.user.email, "login@example.com");
    assert_eq!(data.user.username, "loginuser");
    assert_eq!(data.user.tenant_id, auth_data.user.tenant_id);
    assert!(session_cookie.contains(&data.token));
}

#[actix_rt::test]
async fn test_login_wrong_password() {
    let pool = setup_test_db().await;
    let app_state = build_app_state(pool);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
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
    assert!(
        resp.headers()
            .get_all(actix_web::http::header::SET_COOKIE)
            .into_iter()
            .next()
            .is_none(),
        "failed login must not overwrite browser session cookies"
    );

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

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
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

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
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

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
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

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
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

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
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

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
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

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
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

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
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
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
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
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
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
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
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

// ─── Git Client Auth (EVO-103-B-1) ───

use api::middleware::csrf::CsrfMiddleware;
use base64::Engine;
use domain::api_key::NewApiKey;
use sha2::{Digest, Sha256};

fn hash_api_key(key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    hex::encode(hasher.finalize())
}

async fn create_test_api_key(
    repo: &dyn ApiKeyRepository,
    tenant_id: Uuid,
    user_id: Uuid,
    key_value: &str,
) {
    repo.create(NewApiKey {
        tenant_id,
        user_id,
        name: "test-git-key".to_string(),
        key_hash: hash_api_key(key_value),
        key_prefix: "evo_sk".to_string(),
        permissions: vec!["repo:read".to_string(), "repo:write".to_string()],
        rate_limit: Some(1000),
        expires_at: None,
    })
    .await
    .expect("Failed to create test API key");
}

#[actix_rt::test]
#[allow(clippy::unwrap_used)]
async fn test_basic_auth_valid_api_key_authenticates() {
    let pool = setup_test_db().await;

    let register_payload = serde_json::json!({
        "email": "gituser@example.com",
        "username": "gituser",
        "password": "TestPassword123!",
    });
    let app_for_register = test::init_service(
        App::new()
            .app_data(web::Data::new(build_app_state(pool.clone())))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .service(web::scope("/api/v1").configure(routes::auth::configure)),
    )
    .await;
    let register_req = test::TestRequest::post()
        .uri("/api/v1/auth/register")
        .set_json(&register_payload)
        .to_request();
    let register_resp = test::call_service(&app_for_register, register_req).await;
    assert_eq!(register_resp.status(), actix_web::http::StatusCode::CREATED);
    let register_body = test::read_body(register_resp).await;
    let register_result: ApiResponse<AuthResponseData> =
        serde_json::from_slice(&register_body).expect("Failed to parse register response");
    let auth_data = register_result.data.expect("Expected auth data");
    let user_id = Uuid::parse_str(&auth_data.user.id).expect("Invalid user UUID");
    let tenant_id = Uuid::parse_str(&auth_data.tenant.id).expect("Invalid tenant UUID");

    let api_key_repo: Arc<dyn ApiKeyRepository> =
        Arc::new(SqliteApiKeyRepository::new(pool.clone()));
    let test_api_key = "evo_sk_git_test_key_12345";
    create_test_api_key(&*api_key_repo, tenant_id, user_id, test_api_key).await;

    let mut state = build_app_state(pool);
    state.api_key_repo = api_key_repo;

    async fn echo_user(req: actix_web::HttpRequest) -> actix_web::HttpResponse {
        use api::middleware::rbac::CurrentUserExt;
        match req.get_current_user() {
            Some(user) => actix_web::HttpResponse::Ok().json(serde_json::json!({
                "user_id": user.user_id.to_string(),
                "tenant_id": user.tenant_id.to_string(),
                "role": user.role,
            })),
            None => actix_web::HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "unauthenticated"
            })),
        }
    }

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .route("/repos/test-repo/info/refs", web::get().to(echo_user)),
    )
    .await;

    let credentials = format!("gituser:{}", test_api_key);
    let encoded = base64::engine::general_purpose::STANDARD.encode(credentials);

    let req = test::TestRequest::get()
        .uri("/repos/test-repo/info/refs")
        .insert_header(("Authorization", format!("Basic {}", encoded)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).expect("Failed to parse response");
    assert_eq!(json["user_id"], auth_data.user.id);
    assert_eq!(json["role"], "api_key");
}

#[actix_rt::test]
#[allow(clippy::unwrap_used)]
async fn test_basic_auth_invalid_api_key_returns_401() {
    let pool = setup_test_db().await;

    let register_payload = serde_json::json!({
        "email": "gituser2@example.com",
        "username": "gituser2",
        "password": "TestPassword123!",
    });
    let app_for_register = test::init_service(
        App::new()
            .app_data(web::Data::new(build_app_state(pool.clone())))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .service(web::scope("/api/v1").configure(routes::auth::configure)),
    )
    .await;
    let register_req = test::TestRequest::post()
        .uri("/api/v1/auth/register")
        .set_json(&register_payload)
        .to_request();
    let register_resp = test::call_service(&app_for_register, register_req).await;
    assert_eq!(register_resp.status(), actix_web::http::StatusCode::CREATED);
    let register_body = test::read_body(register_resp).await;
    let register_result: ApiResponse<AuthResponseData> =
        serde_json::from_slice(&register_body).expect("Failed to parse register response");
    let auth_data = register_result.data.expect("Expected auth data");
    let user_id = Uuid::parse_str(&auth_data.user.id).expect("Invalid user UUID");
    let tenant_id = Uuid::parse_str(&auth_data.tenant.id).expect("Invalid tenant UUID");

    let api_key_repo: Arc<dyn ApiKeyRepository> =
        Arc::new(SqliteApiKeyRepository::new(pool.clone()));
    create_test_api_key(
        &*api_key_repo,
        tenant_id,
        user_id,
        "evo_sk_valid_key_wont_be_used",
    )
    .await;

    let mut state = build_app_state(pool);
    state.api_key_repo = api_key_repo;

    async fn echo_user(req: actix_web::HttpRequest) -> actix_web::HttpResponse {
        use api::middleware::rbac::CurrentUserExt;
        match req.get_current_user() {
            Some(user) => actix_web::HttpResponse::Ok().json(serde_json::json!({
                "user_id": user.user_id.to_string(),
            })),
            None => actix_web::HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "unauthenticated"
            })),
        }
    }

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .route("/repos/test-repo/info/refs", web::get().to(echo_user)),
    )
    .await;

    let credentials = "gituser:evo_sk_invalid_key_that_does_not_exist";
    let encoded = base64::engine::general_purpose::STANDARD.encode(credentials);

    let req = test::TestRequest::get()
        .uri("/repos/test-repo/info/refs")
        .insert_header(("Authorization", format!("Basic {}", encoded)))
        .to_request();

    let resp = test::try_call_service(&app, req).await;
    assert!(resp.is_err(), "Expected error for invalid API key");
}

#[actix_rt::test]
async fn test_repos_path_without_auth_returns_401() {
    let pool = setup_test_db().await;
    let app_state = build_app_state(pool);

    async fn echo_user(_req: actix_web::HttpRequest) -> actix_web::HttpResponse {
        actix_web::HttpResponse::Ok().json(serde_json::json!({ "ok": true }))
    }

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .route("/repos/test-repo/info/refs", web::get().to(echo_user)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/repos/test-repo/info/refs")
        .to_request();

    let resp = test::try_call_service(&app, req).await;
    assert!(
        resp.is_err(),
        "Expected error for unauthenticated /repos/ request"
    );
}

#[actix_rt::test]
#[allow(clippy::unwrap_used)]
async fn test_csrf_exempt_git_post_passes_without_token() {
    let pool = setup_test_db().await;

    // Create user and API key for auth
    let register_payload = serde_json::json!({
        "email": "csrf-git-user@example.com",
        "username": "csrfgituser",
        "password": "TestPassword123!",
    });
    let app_for_register = test::init_service(
        App::new()
            .app_data(web::Data::new(build_app_state(pool.clone())))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .service(web::scope("/api/v1").configure(routes::auth::configure)),
    )
    .await;
    let register_req = test::TestRequest::post()
        .uri("/api/v1/auth/register")
        .set_json(&register_payload)
        .to_request();
    let register_resp = test::call_service(&app_for_register, register_req).await;
    assert_eq!(register_resp.status(), actix_web::http::StatusCode::CREATED);
    let register_body = test::read_body(register_resp).await;
    let register_result: ApiResponse<AuthResponseData> =
        serde_json::from_slice(&register_body).expect("Failed to parse register response");
    let auth_data = register_result.data.expect("Expected auth data");
    let user_id = Uuid::parse_str(&auth_data.user.id).expect("Invalid user UUID");
    let tenant_id = Uuid::parse_str(&auth_data.tenant.id).expect("Invalid tenant UUID");

    let api_key_repo: Arc<dyn ApiKeyRepository> =
        Arc::new(SqliteApiKeyRepository::new(pool.clone()));
    let test_api_key = "evo_sk_csrf_git_test_key";
    create_test_api_key(&*api_key_repo, tenant_id, user_id, test_api_key).await;

    let mut app_state = build_app_state(pool);
    app_state.api_key_repo = api_key_repo;

    async fn git_receive_pack(_req: actix_web::HttpRequest) -> actix_web::HttpResponse {
        actix_web::HttpResponse::Ok().json(serde_json::json!({ "ok": true }))
    }

    let csrf = CsrfMiddleware::new();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(csrf)
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .route(
                "/repos/test-repo/git-receive-pack",
                web::post().to(git_receive_pack),
            ),
    )
    .await;

    let credentials = format!("gituser:{}", test_api_key);
    let encoded = base64::engine::general_purpose::STANDARD.encode(credentials);

    let req = test::TestRequest::post()
        .uri("/repos/test-repo/git-receive-pack")
        .insert_header(("Authorization", format!("Basic {}", encoded)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
}

#[actix_rt::test]
async fn graphql_viewer_returns_persisted_profile_and_rejects_wrong_tenant_claims() {
    let pool = setup_test_db().await;
    let app_state = build_app_state(pool);
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(from_fn(api::middleware::graphql_optional_auth_middleware))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .configure(routes::configure_routes),
    )
    .await;

    let register_req = test::TestRequest::post()
        .uri("/api/v1/auth/register")
        .set_json(serde_json::json!({
            "email": "graphql-viewer@example.com",
            "username": "graphqlviewer",
            "password": "TestPassword123!",
        }))
        .to_request();
    let register_resp = test::call_service(&app, register_req).await;
    assert_eq!(register_resp.status(), actix_web::http::StatusCode::CREATED);
    let registered: ApiResponse<AuthResponseData> = test::read_body_json(register_resp).await;
    let auth = registered.data.expect("registered user and JWT");

    let query = serde_json::json!({
        "query": "{ viewer { id email username role tenantId tenantRole emailVerified } }"
    });
    let request = test::TestRequest::post()
        .uri("/graphql")
        .insert_header(("Authorization", format!("Bearer {}", auth.token)))
        .set_json(&query)
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), actix_web::http::StatusCode::OK);
    let body: serde_json::Value = test::read_body_json(response).await;
    assert!(body.get("errors").is_none(), "{body}");
    assert_eq!(body["data"]["viewer"]["id"], auth.user.id);
    assert_eq!(
        body["data"]["viewer"]["email"],
        "graphql-viewer@example.com"
    );
    assert_eq!(body["data"]["viewer"]["username"], "graphqlviewer");
    assert_eq!(body["data"]["viewer"]["tenantId"], auth.user.tenant_id);
    assert_eq!(body["data"]["viewer"]["tenantRole"], "owner");
    assert_eq!(body["data"]["viewer"]["emailVerified"], false);

    let bad_tenant_token = JwtHandler::new(&create_test_config().jwt)
        .generate_token(
            Uuid::parse_str(&auth.user.id).expect("valid registered ID"),
            "user",
            Uuid::new_v4(),
            "owner",
        )
        .expect("sign a mismatched tenant claim")
        .0;
    let mismatch_request = test::TestRequest::post()
        .uri("/graphql")
        .insert_header(("Authorization", format!("Bearer {bad_tenant_token}")))
        .set_json(&query)
        .to_request();
    let mismatch_response = test::call_service(&app, mismatch_request).await;
    assert_eq!(mismatch_response.status(), actix_web::http::StatusCode::OK);
    let mismatch: serde_json::Value = test::read_body_json(mismatch_response).await;
    assert_eq!(mismatch["errors"][0]["extensions"]["code"], "UNAUTHORIZED");
    assert!(mismatch["data"]
        .get("viewer")
        .is_none_or(|value| value.is_null()));
}

#[actix_rt::test]
async fn graphql_viewer_requires_auth_and_fails_closed_on_invalid_bearer() {
    let pool = setup_test_db().await;
    let app_state = build_app_state(pool);
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(from_fn(api::middleware::graphql_optional_auth_middleware))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .configure(routes::configure_routes),
    )
    .await;

    let query = serde_json::json!({ "query": "{ viewer { id email } }" });
    let anonymous = test::TestRequest::post()
        .uri("/graphql")
        .set_json(&query)
        .to_request();
    let response = test::call_service(&app, anonymous).await;
    assert_eq!(response.status(), actix_web::http::StatusCode::OK);
    let body: serde_json::Value = test::read_body_json(response).await;
    assert_eq!(body["errors"][0]["extensions"]["code"], "UNAUTHORIZED");
    assert!(body["data"]
        .get("viewer")
        .is_none_or(|value| value.is_null()));

    let invalid = test::TestRequest::post()
        .uri("/graphql")
        .insert_header(("Authorization", "Bearer invalid-token"))
        .set_json(&query)
        .to_request();
    let error = test::try_call_service(&app, invalid)
        .await
        .expect_err("invalid supplied Bearer must be rejected by optional auth");
    assert_eq!(
        error.as_response_error().status_code(),
        actix_web::http::StatusCode::UNAUTHORIZED
    );
}

#[actix_rt::test]
async fn graphql_login_uses_shared_auth_service_and_issues_browser_cookies() {
    let pool = setup_test_db().await;
    let app_state = build_app_state(pool);
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(from_fn(api::middleware::graphql_optional_auth_middleware))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .wrap(api::middleware::CsrfMiddleware::new())
            .configure(routes::configure_routes),
    )
    .await;

    let register = test::TestRequest::post()
        .uri("/api/v1/auth/register")
        .set_json(serde_json::json!({
            "email": "graphql-login@example.com",
            "username": "graphqllogin",
            "password": "TestPassword123!",
        }))
        .to_request();
    assert_eq!(
        test::call_service(&app, register).await.status(),
        actix_web::http::StatusCode::CREATED
    );

    let operation = serde_json::json!({
        "query": "mutation Login($email: String!, $password: String!) { login(email: $email, password: $password) { token expiresAt user { id email tenantId tenantRole } tenant { id name slug plan } } }",
        "variables": { "email": "graphql-login@example.com", "password": "TestPassword123!" }
    });
    let request = test::TestRequest::post()
        .uri("/graphql")
        .set_json(&operation)
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), actix_web::http::StatusCode::OK);

    let cookies = response
        .headers()
        .get_all(actix_web::http::header::SET_COOKIE)
        .into_iter()
        .map(|value| value.to_str().expect("valid Set-Cookie").to_owned())
        .collect::<Vec<_>>();
    assert_eq!(cookies.len(), 2);
    let jwt_cookie = cookies
        .iter()
        .find(|value| value.starts_with("evolith_token="))
        .expect("session cookie");
    let csrf_cookie = cookies
        .iter()
        .find(|value| value.starts_with("csrf_token="))
        .expect("CSRF cookie");
    assert!(jwt_cookie.contains("HttpOnly"));
    assert!(!csrf_cookie.contains("HttpOnly"));
    for cookie in [&jwt_cookie, &csrf_cookie] {
        assert!(cookie.contains("Path=/"));
        assert!(cookie.contains("SameSite=Lax"));
        assert!(cookie.contains("Max-Age="));
    }

    let payload: serde_json::Value = test::read_body_json(response).await;
    assert!(payload.get("errors").is_none(), "{payload}");
    let token = payload["data"]["login"]["token"]
        .as_str()
        .expect("login JWT");
    let expires_at = payload["data"]["login"]["expiresAt"]
        .as_str()
        .expect("string epoch seconds")
        .parse::<i64>()
        .expect("numeric epoch");
    assert!(expires_at > chrono::Utc::now().timestamp());
    assert!(jwt_cookie.contains(token));
    assert_eq!(
        payload["data"]["login"]["user"]["email"],
        "graphql-login@example.com"
    );
    assert_eq!(payload["data"]["login"]["user"]["tenantRole"], "owner");

    // Reuse the real signed JWT at the existing GraphQL viewer auth boundary.
    let viewer = test::TestRequest::post()
        .uri("/graphql")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(serde_json::json!({ "query": "{ viewer { email tenantRole } }" }))
        .to_request();
    let profile: serde_json::Value =
        test::read_body_json(test::call_service(&app, viewer).await).await;
    assert!(profile.get("errors").is_none(), "{profile}");
    assert_eq!(profile["data"]["viewer"]["email"], "graphql-login@example.com");
    assert_eq!(profile["data"]["viewer"]["tenantRole"], "owner");
}

#[actix_rt::test]
async fn graphql_login_rejects_invalid_credentials_and_does_not_issue_cookies() {
    let pool = setup_test_db().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(build_app_state(pool)))
            .wrap(from_fn(api::middleware::graphql_optional_auth_middleware))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .wrap(api::middleware::CsrfMiddleware::new())
            .configure(routes::configure_routes),
    )
    .await;

    let wrong_password = test::TestRequest::post()
        .uri("/graphql")
        .set_json(serde_json::json!({
            "query": "mutation { login(email: \"absent@example.com\", password: \"WrongPassword123!\") { token } }"
        }))
        .to_request();
    let response = test::call_service(&app, wrong_password).await;
    assert_eq!(response.status(), actix_web::http::StatusCode::OK);
    assert!(
        response
            .headers()
            .get_all(actix_web::http::header::SET_COOKIE)
            .into_iter()
            .next()
            .is_none()
    );
    let body: serde_json::Value = test::read_body_json(response).await;
    assert_eq!(body["errors"][0]["extensions"]["code"], "INVALID_CREDENTIALS");
    assert_eq!(body["errors"][0]["extensions"]["retryable"], false);
    assert!(body["errors"][0]["extensions"]["requestId"].is_string());

    let invalid_email = test::TestRequest::post()
        .uri("/graphql")
        .set_json(serde_json::json!({
            "query": "mutation { login(email: \"not-an-email\", password: \"x\") { token } }"
        }))
        .to_request();
    let response = test::call_service(&app, invalid_email).await;
    let body: serde_json::Value = test::read_body_json(response).await;
    assert_eq!(body["errors"][0]["extensions"]["code"], "VALIDATION_ERROR");
}

#[actix_rt::test]
async fn graphql_login_preserves_cookie_csrf_and_invalid_bearer_fail_closed() {
    let pool = setup_test_db().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(build_app_state(pool)))
            .wrap(from_fn(api::middleware::graphql_optional_auth_middleware))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .wrap(api::middleware::CsrfMiddleware::new())
            .configure(routes::configure_routes),
    )
    .await;

    let operation = serde_json::json!({
        "query": "mutation { login(email: \"user@example.com\", password: \"TestPassword123!\") { token } }"
    });
    let without_csrf = test::TestRequest::post()
        .uri("/graphql")
        .cookie(actix_web::cookie::Cookie::new("evolith_token", "stale-token"))
        .set_json(&operation)
        .to_request();
    let error = test::try_call_service(&app, without_csrf)
        .await
        .expect_err("cookie session without double-submit CSRF must be rejected");
    assert_eq!(error.as_response_error().status_code(), actix_web::http::StatusCode::FORBIDDEN);

    // When CSRF is supplied, stale credentials are still rejected by optional-auth:
    // do not silently let an expired browser session become anonymous.
    let stale = test::TestRequest::post()
        .uri("/graphql")
        .cookie(actix_web::cookie::Cookie::new("evolith_token", "stale-token"))
        .cookie(actix_web::cookie::Cookie::new("csrf_token", "csrf-value"))
        .insert_header(("X-CSRF-Token", "csrf-value"))
        .set_json(&operation)
        .to_request();
    let error = test::try_call_service(&app, stale)
        .await
        .expect_err("stale cookie must not bypass credential validation");
    assert_eq!(error.as_response_error().status_code(), actix_web::http::StatusCode::UNAUTHORIZED);

    let invalid_bearer = test::TestRequest::post()
        .uri("/graphql")
        .insert_header(("Authorization", "Bearer invalid-token"))
        .set_json(&operation)
        .to_request();
    let error = test::try_call_service(&app, invalid_bearer)
        .await
        .expect_err("invalid supplied Bearer must be rejected");
    assert_eq!(error.as_response_error().status_code(), actix_web::http::StatusCode::UNAUTHORIZED);
}
