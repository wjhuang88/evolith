#![allow(clippy::unwrap_used, dead_code)]

use actix_web::{middleware::from_fn, test, web, App};
use base64::Engine;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::sync::Arc;
use tempfile::TempDir;

use api::routes;
use api::state::AppState;
use common::execution::{CompositeProvider, ExecutionProvider};
use domain::api_key::NewApiKey;
use domain::repository::{
    ApiKeyRepository, AuditRepository, GitRepoRepository, InvitationRepository, SkillRepository,
    SnippetRepository, TenantRepository, ToolRepository, UserRepository,
};
use infra::config::{
    AppConfig, AppMetaConfig, DatabaseConfig, GitStorageConfig, JwtConfig, LogConfig,
    RateLimitConfig, RedisConfig, SandboxConfig, ServerConfig, SmtpConfig, StorageConfig,
    StripeConfig,
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
    tenant_id: String,
}

#[derive(Debug, Deserialize)]
struct TenantInfo {
    id: String,
}

#[derive(Debug, Deserialize)]
struct RepoResponseData {
    id: String,
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
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("Failed to create in-memory SQLite pool");

    for sql in [
        MIGRATION_001,
        MIGRATION_003,
        MIGRATION_004,
        MIGRATION_005,
        MIGRATION_006,
        MIGRATION_007,
        MIGRATION_008,
        MIGRATION_009,
    ] {
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
                .execute(&pool)
                .await
                .unwrap_or_else(|e| {
                    panic!(
                        "Failed to execute migration statement: {}\nSQL: {}",
                        e, without_comments
                    )
                });
        }
    }

    pool
}

fn create_test_config(base_path: String) -> AppConfig {
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
            secret: "test_secret_key_for_api_key_scope_32chars!!".to_string(),
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
        git_storage: GitStorageConfig { base_path },
    }
}

fn build_app_state(pool: SqlitePool, base_path: String) -> AppState {
    let config = create_test_config(base_path);
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
        git_storage_base_path: config.git_storage.base_path.clone(),
    }
}

fn hash_key(key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    hex::encode(hasher.finalize())
}

async fn seed_api_key(
    pool: &SqlitePool,
    tenant_id: Uuid,
    user_id: Uuid,
    key_value: &str,
    permissions: Vec<String>,
) {
    let repo: Arc<dyn ApiKeyRepository> = Arc::new(SqliteApiKeyRepository::new(pool.clone()));
    repo.create(NewApiKey {
        tenant_id,
        user_id,
        name: "scope-test-key".to_string(),
        key_hash: hash_key(key_value),
        key_prefix: "evo_sk".to_string(),
        permissions,
        rate_limit: Some(1000),
        expires_at: None,
    })
    .await
    .expect("Failed to create test API key");
}

async fn register_user(
    pool: SqlitePool,
    base_path: String,
    email: &str,
    username: &str,
) -> (String, String, String) {
    let app_state = build_app_state(pool, base_path);
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .configure(routes::configure_routes),
    )
    .await;

    let register_payload = serde_json::json!({
        "email": email,
        "username": username,
        "password": "TestPassword123!",
    });
    let req = test::TestRequest::post()
        .uri("/api/v1/auth/register")
        .set_json(&register_payload)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(
        resp.status().is_success(),
        "register failed: {:?}",
        test::read_body(resp).await
    );
    let body = test::read_body(resp).await;
    let result: ApiResponse<AuthResponseData> =
        serde_json::from_slice(&body).expect("Failed to parse register response");
    let auth_data = result.data.expect("Expected auth data");
    (
        auth_data.token,
        auth_data.tenant.id,
        auth_data.user.id,
    )
}

fn basic_auth_header(api_key: &str) -> String {
    let encoded =
        base64::engine::general_purpose::STANDARD.encode(format!("user:{}", api_key));
    format!("Basic {}", encoded)
}

async fn seed_repo(
    pool: SqlitePool,
    base_path: String,
    token: &str,
    tenant_id: &str,
    name: &str,
) -> String {
    let app_state = build_app_state(pool, base_path);
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .configure(routes::configure_routes),
    )
    .await;

    let req = test::TestRequest::post()
        .uri(&format!("/api/v1/tenant/{}/repos", tenant_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({"name": name}))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::CREATED);
    let body = test::read_body(resp).await;
    let result: ApiResponse<RepoResponseData> =
        serde_json::from_slice(&body).expect("Failed to parse create response");
    result.data.expect("Expected data").id
}

#[actix_rt::test]
async fn test_read_only_api_key_can_list_repos_but_cannot_create() {
    let pool = setup_test_db().await;
    let temp_dir = TempDir::new().expect("temp dir");
    let base_path = temp_dir.path().to_string_lossy().to_string();

    let (_token, tenant_id, user_id) =
        register_user(pool.clone(), base_path.clone(), "read-only@example.com", "readonly").await;

    let tenant_uuid = Uuid::parse_str(&tenant_id).unwrap();
    let user_uuid = Uuid::parse_str(&user_id).unwrap();
    let api_key_value = "evo_sk_read_only_scope_99";
    seed_api_key(
        &pool,
        tenant_uuid,
        user_uuid,
        api_key_value,
        vec!["repo:read".to_string()],
    )
    .await;

    let app_state = build_app_state(pool.clone(), base_path.clone());
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .configure(routes::configure_routes),
    )
    .await;

    let list_req = test::TestRequest::get()
        .uri(&format!("/api/v1/tenant/{}/repos", tenant_id))
        .insert_header(("Authorization", basic_auth_header(api_key_value)))
        .to_request();
    let list_resp = test::call_service(&app, list_req).await;
    assert_eq!(
        list_resp.status(),
        actix_web::http::StatusCode::OK,
        "read-only key should be able to list"
    );

    let create_req = test::TestRequest::post()
        .uri(&format!("/api/v1/tenant/{}/repos", tenant_id))
        .insert_header(("Authorization", basic_auth_header(api_key_value)))
        .set_json(serde_json::json!({"name": "should-not-create"}))
        .to_request();
    let create_resp = test::call_service(&app, create_req).await;
    assert_eq!(
        create_resp.status(),
        actix_web::http::StatusCode::FORBIDDEN,
        "read-only key must NOT be able to create repo"
    );
}

#[actix_rt::test]
async fn test_execute_only_api_key_cannot_read_repo_context() {
    let pool = setup_test_db().await;
    let temp_dir = TempDir::new().expect("temp dir");
    let base_path = temp_dir.path().to_string_lossy().to_string();

    let (token, tenant_id, user_id) =
        register_user(pool.clone(), base_path.clone(), "exec-only@example.com", "exconly").await;

    let tenant_uuid = Uuid::parse_str(&tenant_id).unwrap();
    let user_uuid = Uuid::parse_str(&user_id).unwrap();
    let repo_id = seed_repo(
        pool.clone(),
        base_path.clone(),
        &token,
        &tenant_id,
        "exec-only-target",
    )
    .await;

    let api_key_value = "evo_sk_exec_only_99";
    seed_api_key(
        &pool,
        tenant_uuid,
        user_uuid,
        api_key_value,
        vec!["execute".to_string()],
    )
    .await;

    let app_state = build_app_state(pool, base_path);
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .configure(routes::configure_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri(&format!(
            "/api/v1/tenant/{}/repos/{}/file-tree?ref=main",
            tenant_id, repo_id
        ))
        .insert_header(("Authorization", basic_auth_header(api_key_value)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(
        resp.status(),
        actix_web::http::StatusCode::FORBIDDEN,
        "execute-only key must NOT access Context API"
    );
}

#[actix_rt::test]
async fn test_api_key_cannot_manage_other_api_keys() {
    let pool = setup_test_db().await;
    let temp_dir = TempDir::new().expect("temp dir");
    let base_path = temp_dir.path().to_string_lossy().to_string();

    let (_token, tenant_id, user_id) =
        register_user(pool.clone(), base_path.clone(), "admin-key@example.com", "adminkey").await;

    let tenant_uuid = Uuid::parse_str(&tenant_id).unwrap();
    let user_uuid = Uuid::parse_str(&user_id).unwrap();
    let api_key_value = "evo_sk_admin_key_99";
    seed_api_key(
        &pool,
        tenant_uuid,
        user_uuid,
        api_key_value,
        vec!["admin".to_string(), "write".to_string(), "read".to_string()],
    )
    .await;

    let app_state = build_app_state(pool, base_path);
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .configure(routes::configure_routes),
    )
    .await;

    let list_req = test::TestRequest::get()
        .uri(&format!("/api/v1/tenant/{}/api-keys", tenant_id))
        .insert_header(("Authorization", basic_auth_header(api_key_value)))
        .to_request();
    let list_resp = test::call_service(&app, list_req).await;
    assert_eq!(
        list_resp.status(),
        actix_web::http::StatusCode::FORBIDDEN,
        "API key (even admin) must NOT be able to list keys"
    );

    let create_req = test::TestRequest::post()
        .uri(&format!("/api/v1/tenant/{}/api-keys", tenant_id))
        .insert_header(("Authorization", basic_auth_header(api_key_value)))
        .set_json(serde_json::json!({"name": "should-not-create"}))
        .to_request();
    let create_resp = test::call_service(&app, create_req).await;
    assert_eq!(
        create_resp.status(),
        actix_web::http::StatusCode::FORBIDDEN,
        "API key (even admin) must NOT be able to create keys"
    );
}

#[actix_rt::test]
async fn test_jwt_user_original_behavior_preserved() {
    let pool = setup_test_db().await;
    let temp_dir = TempDir::new().expect("temp dir");
    let base_path = temp_dir.path().to_string_lossy().to_string();

    let (token, tenant_id, _user_id) =
        register_user(pool.clone(), base_path.clone(), "jwt-user@example.com", "jwtuser").await;

    let app_state = build_app_state(pool.clone(), base_path.clone());
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .configure(routes::configure_routes),
    )
    .await;

    let list_req = test::TestRequest::get()
        .uri(&format!("/api/v1/tenant/{}/repos", tenant_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let list_resp = test::call_service(&app, list_req).await;
    assert_eq!(list_resp.status(), actix_web::http::StatusCode::OK);

    let create_req = test::TestRequest::post()
        .uri(&format!("/api/v1/tenant/{}/repos", tenant_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({"name": "jwt-create"}))
        .to_request();
    let create_resp = test::call_service(&app, create_req).await;
    assert_eq!(create_resp.status(), actix_web::http::StatusCode::CREATED);
}

#[actix_rt::test]
async fn test_write_api_key_can_create_and_update_repo() {
    let pool = setup_test_db().await;
    let temp_dir = TempDir::new().expect("temp dir");
    let base_path = temp_dir.path().to_string_lossy().to_string();

    let (_token, tenant_id, user_id) =
        register_user(pool.clone(), base_path.clone(), "write-key@example.com", "writekey").await;
    let tenant_uuid = Uuid::parse_str(&tenant_id).unwrap();
    let user_uuid = Uuid::parse_str(&user_id).unwrap();
    let api_key_value = "evo_sk_write_scope_99";
    seed_api_key(
        &pool,
        tenant_uuid,
        user_uuid,
        api_key_value,
        vec!["repo:write".to_string()],
    )
    .await;

    let app_state = build_app_state(pool.clone(), base_path.clone());
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .configure(routes::configure_routes),
    )
    .await;

    let create_req = test::TestRequest::post()
        .uri(&format!("/api/v1/tenant/{}/repos", tenant_id))
        .insert_header(("Authorization", basic_auth_header(api_key_value)))
        .set_json(serde_json::json!({"name": "write-create"}))
        .to_request();
    let create_resp = test::call_service(&app, create_req).await;
    assert_eq!(create_resp.status(), actix_web::http::StatusCode::CREATED);
    let body = test::read_body(create_resp).await;
    let result: ApiResponse<RepoResponseData> = serde_json::from_slice(&body).unwrap();
    let repo_id = result.data.unwrap().id;

    let patch_req = test::TestRequest::patch()
        .uri(&format!("/api/v1/tenant/{}/repos/{}", tenant_id, repo_id))
        .insert_header(("Authorization", basic_auth_header(api_key_value)))
        .set_json(serde_json::json!({"description": "updated"}))
        .to_request();
    let patch_resp = test::call_service(&app, patch_req).await;
    assert_eq!(patch_resp.status(), actix_web::http::StatusCode::OK);

    let del_req = test::TestRequest::delete()
        .uri(&format!("/api/v1/tenant/{}/repos/{}", tenant_id, repo_id))
        .insert_header(("Authorization", basic_auth_header(api_key_value)))
        .to_request();
    let del_resp = test::call_service(&app, del_req).await;
    assert_eq!(del_resp.status(), actix_web::http::StatusCode::NO_CONTENT);
}
