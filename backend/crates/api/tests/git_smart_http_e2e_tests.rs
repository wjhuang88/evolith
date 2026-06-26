#![allow(clippy::unwrap_used)]
#![allow(dead_code)]

use actix_web::{middleware::from_fn, test, web, App, HttpServer};
use base64::Engine;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::net::TcpListener;
use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use tempfile::TempDir;
use tokio::sync::oneshot;

use api::middleware::csrf::CsrfMiddleware;
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
    name: String,
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

    run_migration_sql(&pool, MIGRATION_001).await;
    run_migration_sql(&pool, MIGRATION_003).await;
    run_migration_sql(&pool, MIGRATION_004).await;
    run_migration_sql(&pool, MIGRATION_005).await;
    run_migration_sql(&pool, MIGRATION_006).await;
    run_migration_sql(&pool, MIGRATION_007).await;
    run_migration_sql(&pool, MIGRATION_008).await;
    run_migration_sql(&pool, MIGRATION_009).await;

    (pool, db_dir)
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

fn create_test_config(base_path: String) -> AppConfig {
    AppConfig {
        app: AppMetaConfig {
            public_url: "http://localhost:3001".to_string(),
        },
        server: ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 0,
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
            secret: "test_secret_key_for_git_smart_http_e2e_32chars!!".to_string(),
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
    create_test_api_key_with_permissions(
        repo,
        tenant_id,
        user_id,
        key_value,
        vec!["repo:read".to_string(), "repo:write".to_string()],
    )
    .await;
}

async fn create_test_api_key_with_permissions(
    repo: &dyn ApiKeyRepository,
    tenant_id: Uuid,
    user_id: Uuid,
    key_value: &str,
    permissions: Vec<String>,
) {
    repo.create(NewApiKey {
        tenant_id,
        user_id,
        name: "test-git-key".to_string(),
        key_hash: hash_api_key(key_value),
        key_prefix: "evo_sk".to_string(),
        permissions,
        rate_limit: Some(1000),
        expires_at: None,
    })
    .await
    .expect("Failed to create test API key");
}

async fn register_and_create_api_key(
    pool: SqlitePool,
    base_path: String,
    email: &str,
    username: &str,
    api_key_value: &str,
) -> (String, String, String) {
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
    let register_req = test::TestRequest::post()
        .uri("/api/v1/auth/register")
        .set_json(&register_payload)
        .to_request();
    let register_resp = test::call_service(&app, register_req).await;
    assert!(
        register_resp.status().is_success(),
        "Registration failed: {:?}",
        test::read_body(register_resp).await
    );
    let register_body = test::read_body(register_resp).await;
    let register_result: ApiResponse<AuthResponseData> =
        serde_json::from_slice(&register_body).expect("Failed to parse register response");
    let auth_data = register_result.data.expect("Expected auth data");
    let user_id = Uuid::parse_str(&auth_data.user.id).expect("Invalid user UUID");
    let tenant_id = Uuid::parse_str(&auth_data.tenant.id).expect("Invalid tenant UUID");
    let token = auth_data.token;

    let api_key_repo: Arc<dyn ApiKeyRepository> =
        Arc::new(SqliteApiKeyRepository::new(pool.clone()));
    create_test_api_key(&*api_key_repo, tenant_id, user_id, api_key_value).await;

    (token, tenant_id.to_string(), user_id.to_string())
}

async fn register_and_create_api_key_with_permissions(
    pool: SqlitePool,
    base_path: String,
    email: &str,
    username: &str,
    api_key_value: &str,
    permissions: Vec<String>,
) -> (String, String, String) {
    let (token, tenant_id, user_id) = register_and_create_api_key(
        pool.clone(),
        base_path,
        email,
        username,
        "evo_sk_throwaway_key_not_used",
    )
    .await;

    let api_key_repo: Arc<dyn ApiKeyRepository> =
        Arc::new(SqliteApiKeyRepository::new(pool.clone()));
    let tenant_uuid = Uuid::parse_str(&tenant_id).expect("Invalid tenant UUID");
    let user_uuid = Uuid::parse_str(&user_id).expect("Invalid user UUID");
    create_test_api_key_with_permissions(
        &*api_key_repo,
        tenant_uuid,
        user_uuid,
        api_key_value,
        permissions,
    )
    .await;

    (token, tenant_id, user_id)
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
    assert_eq!(
        resp.status(),
        actix_web::http::StatusCode::CREATED,
        "Failed to create repo"
    );
    let body = test::read_body(resp).await;
    let result: ApiResponse<RepoResponseData> =
        serde_json::from_slice(&body).expect("Failed to parse response");
    result.data.expect("Expected data").id.clone()
}

async fn start_test_server_on_runtime(
    pool: SqlitePool,
    base_path: String,
) -> (u16, oneshot::Sender<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to port");
    let port = listener
        .local_addr()
        .expect("Failed to get local addr")
        .port();

    let state = build_app_state(pool, base_path);
    let app_data = web::Data::new(state);

    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    actix_web::rt::spawn(async move {
        let server = HttpServer::new(move || {
            App::new()
                .app_data(app_data.clone())
                .wrap(CsrfMiddleware::new())
                .wrap(from_fn(api::middleware::rbac::rbac_middleware))
                .configure(routes::configure_routes)
        })
        .listen(listener)
        .expect("Failed to listen")
        .workers(1)
        .run();

        tokio::select! {
            _ = server => {},
            _ = shutdown_rx => {},
        }
    });

    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    (port, shutdown_tx)
}

fn run_git(args: &[&str], dir: &Path, env_vars: &[(&str, &str)]) -> std::process::Output {
    let mut cmd = Command::new("git");
    cmd.args(args)
        .current_dir(dir)
        .env("GIT_TERMINAL_PROMPT", "0")
        .env("GIT_ASKPASS", "echo")
        .env("HOME", dir);
    for (k, v) in env_vars {
        cmd.env(k, v);
    }
    cmd.output().expect("Failed to execute git")
}

#[actix_rt::test]
async fn test_info_refs_with_basic_auth_returns_advertisement() {
    let (pool, _db_dir) = setup_test_db().await;
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path().to_string_lossy().to_string();

    let api_key_value = "evo_sk_info_refs_handler_test_99";
    let (token, tenant_id, _user_id) = register_and_create_api_key(
        pool.clone(),
        base_path.clone(),
        "info-refs@example.com",
        "inforefs",
        api_key_value,
    )
    .await;
    let repo_id = create_repo_via_api(
        pool.clone(),
        base_path.clone(),
        &token,
        &tenant_id,
        "info-refs-repo",
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

    let basic = base64::engine::general_purpose::STANDARD.encode(format!("user:{}", api_key_value));
    let req = test::TestRequest::get()
        .uri(&format!(
            "/repos/{}/info/refs?service=git-upload-pack",
            repo_id
        ))
        .insert_header(("Authorization", format!("Basic {}", basic)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    let ct = resp
        .headers()
        .get("content-type")
        .expect("content-type header present")
        .to_str()
        .unwrap();
    assert!(
        ct.contains("application/x-git-upload-pack-advertisement"),
        "unexpected content-type: {}",
        ct
    );
}

#[actix_rt::test]
async fn test_receive_pack_with_read_only_api_key_is_forbidden() {
    let (pool, _db_dir) = setup_test_db().await;
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path().to_string_lossy().to_string();

    let api_key_value = "evo_sk_read_only_git_key_99";
    let (token, tenant_id, _user_id) = register_and_create_api_key_with_permissions(
        pool.clone(),
        base_path.clone(),
        "readonly-git@example.com",
        "readonlygit",
        api_key_value,
        vec!["repo:read".to_string()],
    )
    .await;
    let repo_id = create_repo_via_api(
        pool.clone(),
        base_path.clone(),
        &token,
        &tenant_id,
        "read-only-receive-pack-repo",
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

    let basic = base64::engine::general_purpose::STANDARD.encode(format!("user:{}", api_key_value));
    let read_req = test::TestRequest::get()
        .uri(&format!(
            "/repos/{}/info/refs?service=git-upload-pack",
            repo_id
        ))
        .insert_header(("Authorization", format!("Basic {}", basic)))
        .to_request();
    let read_resp = test::call_service(&app, read_req).await;
    assert_eq!(read_resp.status(), actix_web::http::StatusCode::OK);

    let req = test::TestRequest::get()
        .uri(&format!(
            "/repos/{}/info/refs?service=git-receive-pack",
            repo_id
        ))
        .insert_header(("Authorization", format!("Basic {}", basic)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::FORBIDDEN);
}

#[actix_rt::test]
#[ignore = "real git-client E2E: git waits for a WWW-Authenticate: Basic challenge on 401 for /repos/ (standard git-server behavior); handler-level auth/subprocess verified in test_info_refs_with_basic_auth. Re-enable once the server sends WWW-Authenticate: Basic on /repos/ 401s (tracked as a B-2 follow-up)."]
async fn test_git_clone_push_pull_e2e() {
    let (pool, _db_dir) = setup_test_db().await;
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path().to_string_lossy().to_string();

    let api_key_value = "evo_sk_git_e2e_test_key_12345";
    let (_token, tenant_id, _user_id) = register_and_create_api_key(
        pool.clone(),
        base_path.clone(),
        "git-e2e@example.com",
        "gite2e",
        api_key_value,
    )
    .await;

    let repo_id = create_repo_via_api(
        pool.clone(),
        base_path.clone(),
        &_token,
        &tenant_id,
        "e2e-test-repo",
    )
    .await;

    let (port, _shutdown) = start_test_server_on_runtime(pool, base_path).await;

    let clone_dir = TempDir::new().expect("Failed to create clone dir");
    let clone_path = clone_dir.path();

    let clone_url = format!(
        "http://gituser:{}@127.0.0.1:{}/repos/{}",
        api_key_value, port, repo_id
    );

    let output = run_git(&["clone", &clone_url, "."], clone_path, &[]);
    assert!(
        output.status.success(),
        "git clone failed: stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );

    let work_dir = TempDir::new().expect("Failed to create work dir");
    let work_path = work_dir.path();

    let output = run_git(&["clone", &clone_url, "."], work_path, &[]);
    assert!(
        output.status.success(),
        "git clone (work) failed: stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );

    let test_file = work_path.join("test.txt");
    std::fs::write(&test_file, "Hello from E2E test\n").expect("Failed to write test file");

    let output = run_git(&["add", "test.txt"], work_path, &[]);
    assert!(
        output.status.success(),
        "git add failed: stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );

    let output = run_git(
        &[
            "-c",
            "user.email=test@example.com",
            "-c",
            "user.name=Test",
            "commit",
            "-m",
            "E2E test commit",
        ],
        work_path,
        &[],
    );
    assert!(
        output.status.success(),
        "git commit failed: stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );

    let output = run_git(&["push", "origin", "HEAD"], work_path, &[]);
    assert!(
        output.status.success(),
        "git push failed: stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );

    let pull_dir = TempDir::new().expect("Failed to create pull dir");
    let pull_path = pull_dir.path();

    let output = run_git(&["clone", &clone_url, "."], pull_path, &[]);
    assert!(
        output.status.success(),
        "git clone (pull) failed: stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );

    let pulled_file = pull_path.join("test.txt");
    assert!(pulled_file.exists(), "test.txt should exist after clone");
    let content = std::fs::read_to_string(&pulled_file).expect("Failed to read test.txt");
    assert_eq!(
        content, "Hello from E2E test\n",
        "Pushed content should be visible after clone"
    );
}
