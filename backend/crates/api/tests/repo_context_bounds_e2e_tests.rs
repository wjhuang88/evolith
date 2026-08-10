#![allow(clippy::unwrap_used, dead_code)]

use actix_web::{middleware::from_fn, test, web, App};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::path::Path;
use std::process::Command;
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
    message: Option<String>,
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

#[derive(Debug, Deserialize)]
struct RepoDetailData {
    last_commit_sha: Option<String>,
    last_committed_at: Option<String>,
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

async fn setup_test_db() -> (SqlitePool, TempDir) {
    let db_dir = TempDir::new().expect("Failed to create db temp dir");
    let db_path = db_dir.path().join("test.db");
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&format!("sqlite://{}?mode=rwc", db_path.display()))
        .await
        .expect("Failed to create SQLite pool");

    for sql in [
        MIGRATION_001,
        MIGRATION_003,
        MIGRATION_004,
        MIGRATION_005,
        MIGRATION_006,
        MIGRATION_007,
        MIGRATION_008,
        MIGRATION_009,
        MIGRATION_010,
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

    (pool, db_dir)
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
            secret: "test_secret_key_for_resource_bounds_32chars!!".to_string(),
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

fn hash_apikey(key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    hex::encode(hasher.finalize())
}

async fn register_user_with_write_api_key(
    pool: SqlitePool,
    base_path: String,
    email: &str,
    username: &str,
    api_key_value: &str,
) -> (String, String) {
    let app_state = build_app_state(pool.clone(), base_path.clone());
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
    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    let result: ApiResponse<AuthResponseData> =
        serde_json::from_slice(&body).expect("Failed to parse register response");
    let auth_data = result.data.expect("Expected auth data");
    let user_id = Uuid::parse_str(&auth_data.user.id).unwrap();
    let tenant_id = Uuid::parse_str(&auth_data.tenant.id).unwrap();
    let token = auth_data.token;

    let api_key_repo: Arc<dyn ApiKeyRepository> =
        Arc::new(SqliteApiKeyRepository::new(pool.clone()));
    api_key_repo
        .create(NewApiKey {
            tenant_id,
            user_id,
            name: "test-key".to_string(),
            key_hash: hash_apikey(api_key_value),
            key_prefix: "evo_sk".to_string(),
            permissions: vec!["repo:read".to_string(), "repo:write".to_string()],
            rate_limit: Some(1000),
            expires_at: None,
        })
        .await
        .expect("Failed to create test API key");

    (token, tenant_id.to_string())
}

async fn create_repo_via_api(
    pool: SqlitePool,
    base_path: String,
    token: &str,
    tenant_id: &str,
    repo_name: &str,
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
        .set_json(serde_json::json!({
            "name": repo_name,
            "seed_template": false,
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::CREATED);
    let body = test::read_body(resp).await;
    let result: ApiResponse<RepoResponseData> =
        serde_json::from_slice(&body).expect("Failed to parse response");
    result.data.expect("Expected data").id
}

fn push_git_file(bare_path: &Path, file_name: &str, content: &str) {
    let work_dir = TempDir::new().expect("Failed to create work dir");
    let work_path = work_dir.path();
    let run = |args: &[&str]| {
        Command::new("git")
            .args(args)
            .current_dir(work_path)
            .env("GIT_TERMINAL_PROMPT", "0")
            .env("HOME", work_path)
            .output()
            .expect("Failed to execute git")
    };
    assert!(run(&["init"]).status.success());
    assert!(run(&["config", "user.email", "test@example.com"])
        .status
        .success());
    assert!(run(&["config", "user.name", "Test"]).status.success());
    assert!(
        run(&["remote", "add", "origin", bare_path.to_str().unwrap()])
            .status
            .success()
    );
    std::fs::write(work_path.join(file_name), content).expect("Failed to write file");
    assert!(run(&["add", file_name]).status.success());
    assert!(run(&["commit", "-m", "initial"]).status.success());
    assert!(run(&["push", "origin", "HEAD:main"]).status.success());
}

fn create_many_files_repo(bare_path: &Path, count: usize) {
    let work_dir = TempDir::new().expect("Failed to create work dir");
    let work_path = work_dir.path();
    let run = |args: &[&str]| {
        Command::new("git")
            .args(args)
            .current_dir(work_path)
            .env("GIT_TERMINAL_PROMPT", "0")
            .env("HOME", work_path)
            .output()
            .expect("Failed to execute git")
    };
    assert!(run(&["init"]).status.success());
    assert!(run(&["config", "user.email", "test@example.com"])
        .status
        .success());
    assert!(run(&["config", "user.name", "Test"]).status.success());
    assert!(
        run(&["remote", "add", "origin", bare_path.to_str().unwrap()])
            .status
            .success()
    );

    for i in 0..count {
        let file_name = format!("file_{:04}.txt", i);
        std::fs::write(work_path.join(file_name), format!("content {}\n", i))
            .expect("Failed to write test file");
    }

    assert!(run(&["add", "."]).status.success());
    assert!(run(&["commit", "-m", "many files"]).status.success());
    assert!(run(&["push", "origin", "HEAD:main"]).status.success());
}

fn create_large_diff_repo(bare_path: &Path, count: usize) -> (String, String) {
    let work_dir = TempDir::new().expect("Failed to create work dir");
    let work_path = work_dir.path();
    let run = |args: &[&str]| {
        Command::new("git")
            .args(args)
            .current_dir(work_path)
            .env("GIT_TERMINAL_PROMPT", "0")
            .env("HOME", work_path)
            .output()
            .expect("Failed to execute git")
    };
    assert!(run(&["init"]).status.success());
    assert!(run(&["config", "user.email", "test@example.com"])
        .status
        .success());
    assert!(run(&["config", "user.name", "Test"]).status.success());
    assert!(
        run(&["remote", "add", "origin", bare_path.to_str().unwrap()])
            .status
            .success()
    );

    std::fs::write(work_path.join("base.txt"), "base\n").expect("Failed to write base file");
    assert!(run(&["add", "base.txt"]).status.success());
    assert!(run(&["commit", "-m", "base"]).status.success());
    let base_out = run(&["rev-parse", "HEAD"]);
    assert!(base_out.status.success());
    let base_sha = String::from_utf8_lossy(&base_out.stdout).trim().to_string();

    for i in 0..count {
        let file_name = format!("diff_{:04}.txt", i);
        std::fs::write(work_path.join(file_name), format!("content {}\n", i))
            .expect("Failed to write diff file");
    }
    assert!(run(&["add", "."]).status.success());
    assert!(run(&["commit", "-m", "large diff"]).status.success());
    let head_out = run(&["rev-parse", "HEAD"]);
    assert!(head_out.status.success());
    let head_sha = String::from_utf8_lossy(&head_out.stdout).trim().to_string();

    assert!(run(&["push", "origin", "HEAD:main"]).status.success());
    (base_sha, head_sha)
}

#[actix_rt::test]
async fn test_invalid_ref_returns_404_not_500() {
    let (pool, _db_dir) = setup_test_db().await;
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path().to_string_lossy().to_string();
    let api_key_value = "evo_sk_invalid_ref_test_99";

    let (token, tenant_id) = register_user_with_write_api_key(
        pool.clone(),
        base_path.clone(),
        "invalid-ref@example.com",
        "invalidref",
        api_key_value,
    )
    .await;

    let repo_id = create_repo_via_api(
        pool.clone(),
        base_path.clone(),
        &token,
        &tenant_id,
        "invalid-ref-repo",
    )
    .await;

    let repo_uuid = Uuid::parse_str(&repo_id).unwrap();
    let tenant_uuid = Uuid::parse_str(&tenant_id).unwrap();
    let bare_path = temp_dir
        .path()
        .join(tenant_uuid.to_string())
        .join(format!("{}.git", repo_uuid));
    push_git_file(&bare_path, "README.md", "# hi\n");

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
            "/api/v1/tenant/{}/repos/{}/file-tree?ref=nonexistent-branch",
            tenant_id, repo_id
        ))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(
        resp.status(),
        actix_web::http::StatusCode::NOT_FOUND,
        "invalid ref should map to 404"
    );
    let body = test::read_body(resp).await;
    let result: ApiResponse<serde_json::Value> =
        serde_json::from_slice(&body).expect("Failed to parse");
    assert_eq!(
        result.error.expect("Expected error").code,
        "NOT_FOUND",
        "invalid ref must NOT map to 500"
    );
}

#[actix_rt::test]
async fn test_invalid_sha_returns_404_not_500() {
    let (pool, _db_dir) = setup_test_db().await;
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path().to_string_lossy().to_string();
    let api_key_value = "evo_sk_invalid_sha_test_99";

    let (token, tenant_id) = register_user_with_write_api_key(
        pool.clone(),
        base_path.clone(),
        "invalid-sha@example.com",
        "invalidsha",
        api_key_value,
    )
    .await;

    let repo_id = create_repo_via_api(
        pool.clone(),
        base_path.clone(),
        &token,
        &tenant_id,
        "invalid-sha-repo",
    )
    .await;

    let repo_uuid = Uuid::parse_str(&repo_id).unwrap();
    let tenant_uuid = Uuid::parse_str(&tenant_id).unwrap();
    let bare_path = temp_dir
        .path()
        .join(tenant_uuid.to_string())
        .join(format!("{}.git", repo_uuid));
    push_git_file(&bare_path, "a.txt", "A\n");

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
            "/api/v1/tenant/{}/repos/{}/blobs/0000000000000000000000000000000000000000",
            tenant_id, repo_id
        ))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(
        resp.status(),
        actix_web::http::StatusCode::NOT_FOUND,
        "non-existent blob should map to 404"
    );

    let req = test::TestRequest::get()
        .uri(&format!(
            "/api/v1/tenant/{}/repos/{}/blobs/not-a-valid-sha",
            tenant_id, repo_id
        ))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(
        resp.status(),
        actix_web::http::StatusCode::BAD_REQUEST,
        "malformed sha should map to 400"
    );
}

#[actix_rt::test]
async fn test_oversized_blob_returns_413_not_500() {
    let (pool, _db_dir) = setup_test_db().await;
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path().to_string_lossy().to_string();
    let api_key_value = "evo_sk_oversize_blob_test_99";

    let (token, tenant_id) = register_user_with_write_api_key(
        pool.clone(),
        base_path.clone(),
        "oversize-blob@example.com",
        "oversizeblob",
        api_key_value,
    )
    .await;

    let repo_id = create_repo_via_api(
        pool.clone(),
        base_path.clone(),
        &token,
        &tenant_id,
        "oversize-repo",
    )
    .await;

    let repo_uuid = Uuid::parse_str(&repo_id).unwrap();
    let tenant_uuid = Uuid::parse_str(&tenant_id).unwrap();
    let bare_path = temp_dir
        .path()
        .join(tenant_uuid.to_string())
        .join(format!("{}.git", repo_uuid));

    let work_dir = TempDir::new().expect("work dir");
    let work_path = work_dir.path();
    let run = |args: &[&str]| {
        Command::new("git")
            .args(args)
            .current_dir(work_path)
            .env("GIT_TERMINAL_PROMPT", "0")
            .env("HOME", work_path)
            .output()
            .expect("git failed")
    };
    assert!(run(&["init"]).status.success());
    assert!(run(&["config", "user.email", "test@example.com"])
        .status
        .success());
    assert!(run(&["config", "user.name", "Test"]).status.success());
    assert!(
        run(&["remote", "add", "origin", bare_path.to_str().unwrap()])
            .status
            .success()
    );

    let oversize_bytes = vec![0u8; service_git::BLOB_MAX_BYTES + 1];
    std::fs::write(work_path.join("big.bin"), &oversize_bytes).expect("Failed to write big file");
    assert!(run(&["add", "big.bin"]).status.success());
    assert!(run(&["commit", "-m", "oversize"]).status.success());
    assert!(run(&["push", "origin", "HEAD:main"]).status.success());

    let file_tree_req = test::TestRequest::get()
        .uri(&format!(
            "/api/v1/tenant/{}/repos/{}/file-tree?ref=main",
            tenant_id, repo_id
        ))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let app_state = build_app_state(pool.clone(), base_path.clone());
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .configure(routes::configure_routes),
    )
    .await;
    let tree_resp = test::call_service(&app, file_tree_req).await;
    let tree_body = test::read_body(tree_resp).await;
    let tree_result: ApiResponse<serde_json::Value> = serde_json::from_slice(&tree_body).unwrap();
    let big_oid = tree_result
        .data
        .unwrap()
        .get("entries")
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .find(|e| e.get("name").unwrap() == "big.bin")
        .unwrap()
        .get("oid")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();

    let req = test::TestRequest::get()
        .uri(&format!(
            "/api/v1/tenant/{}/repos/{}/blobs/{}",
            tenant_id, repo_id, big_oid
        ))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(
        resp.status(),
        actix_web::http::StatusCode::PAYLOAD_TOO_LARGE,
        "oversized blob must map to 413"
    );
    let body = test::read_body(resp).await;
    let result: ApiResponse<serde_json::Value> = serde_json::from_slice(&body).unwrap();
    assert_eq!(
        result.error.expect("Expected error").code,
        "RESOURCE_EXCEEDED"
    );
}

#[actix_rt::test]
async fn test_large_file_tree_returns_413() {
    let (pool, _db_dir) = setup_test_db().await;
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path().to_string_lossy().to_string();
    let api_key_value = "evo_sk_large_tree_test_99";

    let (token, tenant_id) = register_user_with_write_api_key(
        pool.clone(),
        base_path.clone(),
        "large-tree@example.com",
        "largetree",
        api_key_value,
    )
    .await;

    let repo_id = create_repo_via_api(
        pool.clone(),
        base_path.clone(),
        &token,
        &tenant_id,
        "large-tree-repo",
    )
    .await;

    let repo_uuid = Uuid::parse_str(&repo_id).unwrap();
    let tenant_uuid = Uuid::parse_str(&tenant_id).unwrap();
    let bare_path = temp_dir
        .path()
        .join(tenant_uuid.to_string())
        .join(format!("{}.git", repo_uuid));
    create_many_files_repo(&bare_path, service_git::FILE_TREE_MAX_ENTRIES + 1);

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
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(
        resp.status(),
        actix_web::http::StatusCode::PAYLOAD_TOO_LARGE
    );
    let body = test::read_body(resp).await;
    let result: ApiResponse<serde_json::Value> = serde_json::from_slice(&body).unwrap();
    assert_eq!(
        result.error.expect("Expected error").code,
        "RESOURCE_EXCEEDED"
    );
}

#[actix_rt::test]
async fn test_large_diff_returns_413() {
    let (pool, _db_dir) = setup_test_db().await;
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path().to_string_lossy().to_string();
    let api_key_value = "evo_sk_large_diff_test_99";

    let (token, tenant_id) = register_user_with_write_api_key(
        pool.clone(),
        base_path.clone(),
        "large-diff@example.com",
        "largediff",
        api_key_value,
    )
    .await;

    let repo_id = create_repo_via_api(
        pool.clone(),
        base_path.clone(),
        &token,
        &tenant_id,
        "large-diff-repo",
    )
    .await;

    let repo_uuid = Uuid::parse_str(&repo_id).unwrap();
    let tenant_uuid = Uuid::parse_str(&tenant_id).unwrap();
    let bare_path = temp_dir
        .path()
        .join(tenant_uuid.to_string())
        .join(format!("{}.git", repo_uuid));
    let (base_sha, head_sha) =
        create_large_diff_repo(&bare_path, service_git::DIFF_MAX_ENTRIES + 1);

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
            "/api/v1/tenant/{}/repos/{}/diff?base={}&head={}",
            tenant_id, repo_id, base_sha, head_sha
        ))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(
        resp.status(),
        actix_web::http::StatusCode::PAYLOAD_TOO_LARGE
    );
    let body = test::read_body(resp).await;
    let result: ApiResponse<serde_json::Value> = serde_json::from_slice(&body).unwrap();
    assert_eq!(
        result.error.expect("Expected error").code,
        "RESOURCE_EXCEEDED"
    );
}

#[actix_rt::test]
async fn test_push_updates_repo_default_branch_metadata() {
    use actix_web::{App, HttpServer};
    use api::middleware::csrf::CsrfMiddleware;
    use std::net::TcpListener;

    let (pool, _db_dir) = setup_test_db().await;
    let temp_dir = TempDir::new().expect("temp dir");
    let base_path = temp_dir.path().to_string_lossy().to_string();
    let api_key_value = "evo_sk_push_meta_test_99";

    let (token, tenant_id) = register_user_with_write_api_key(
        pool.clone(),
        base_path.clone(),
        "push-meta@example.com",
        "pushmeta",
        api_key_value,
    )
    .await;
    let repo_id = create_repo_via_api(
        pool.clone(),
        base_path.clone(),
        &token,
        &tenant_id,
        "push-meta-repo",
    )
    .await;
    let repo_uuid = Uuid::parse_str(&repo_id).unwrap();

    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let port = listener.local_addr().unwrap().port();
    let app_data = web::Data::new(build_app_state(pool.clone(), base_path.clone()));
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
    let _server_task = actix_web::rt::spawn(async move {
        let server = HttpServer::new(move || {
            App::new()
                .app_data(app_data.clone())
                .wrap(CsrfMiddleware::new())
                .wrap(from_fn(api::middleware::rbac::rbac_middleware))
                .configure(routes::configure_routes)
        })
        .listen(listener)
        .expect("listen")
        .workers(1)
        .run();
        tokio::select! {
            _ = server => {},
            _ = shutdown_rx => {},
        }
    });
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    let work_dir = TempDir::new().expect("work dir");
    let work_path = work_dir.path();
    let run = |args: &[&str], envs: &[(&str, &str)]| {
        let mut cmd = Command::new("git");
        cmd.args(args)
            .current_dir(work_path)
            .env("GIT_TERMINAL_PROMPT", "0")
            .env("GIT_ASKPASS", "echo")
            .env("HOME", work_path);
        for (k, v) in envs {
            cmd.env(k, v);
        }
        cmd.output().expect("git failed")
    };
    let clone_url = format!(
        "http://gituser:{}@127.0.0.1:{}/repos/{}",
        api_key_value, port, repo_id
    );
    let out = run(&["clone", &clone_url, "."], &[]);
    assert!(
        out.status.success(),
        "clone failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(run(&["config", "user.email", "test@example.com"], &[])
        .status
        .success());
    assert!(run(&["config", "user.name", "Test"], &[]).status.success());
    assert!(run(&["config", "init.defaultBranch", "main"], &[])
        .status
        .success());
    let _ = run(&["symbolic-ref", "HEAD", "refs/heads/main"], &[]);
    std::fs::write(work_path.join("pushed.txt"), "pushed\n").unwrap();
    assert!(run(&["add", "pushed.txt"], &[]).status.success());
    assert!(run(&["commit", "-m", "push-meta"], &[]).status.success());
    let push_out = run(&["push", "origin", "HEAD:refs/heads/main"], &[]);
    assert!(
        push_out.status.success(),
        "git push failed: {}",
        String::from_utf8_lossy(&push_out.stderr)
    );

    let mut updated = None;
    for _ in 0..40 {
        tokio::time::sleep(std::time::Duration::from_millis(250)).await;
        let candidate = pool_repo_find(&pool, repo_uuid).await;
        if candidate.last_commit_sha.is_some() {
            updated = Some(candidate);
            break;
        }
    }
    drop(shutdown_tx);
    let _ = _server_task.await;

    let updated = updated.expect(
        "last_commit_sha should be set after push (background task did not complete within 10s)",
    );
    assert!(
        updated.last_committed_at.is_some(),
        "last_committed_at should be set after push"
    );
    let sha = updated.last_commit_sha.as_ref().unwrap();
    assert_eq!(sha.len(), 40, "expected 40-char hex sha, got {}", sha);
}

async fn pool_repo_find(pool: &SqlitePool, repo_id: Uuid) -> domain::git_repo::GitRepo {
    let repo_repo: Arc<dyn GitRepoRepository> =
        Arc::new(SqliteGitRepoRepository::new(pool.clone()));
    repo_repo
        .find_by_id(repo_id)
        .await
        .unwrap()
        .expect("repo should exist")
}
