#![allow(clippy::unwrap_used)]
#![allow(dead_code)]

use actix_web::{middleware::from_fn, test, web, App};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
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
const MIGRATION_011: &str = include_str!("../../../migrations/sqlite/011_outbox_events.sql");
const MIGRATION_012: &str = include_str!("../../../migrations/sqlite/012_outbox_claim_leases.sql");

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

#[derive(Debug, Deserialize)]
struct FileTreeResponseData {
    entries: Vec<FileTreeEntryData>,
}

#[derive(Debug, Deserialize)]
struct FileTreeEntryData {
    name: String,
    kind: String,
    oid: String,
    is_tree: bool,
}

#[derive(Debug, Deserialize)]
struct BlobResponseData {
    content: String,
    size: usize,
    encoding: String,
}

#[derive(Debug, Deserialize)]
struct CommitListResponseData {
    commits: Vec<CommitData>,
}

#[derive(Debug, Deserialize)]
struct CommitData {
    sha: String,
    author_name: String,
    author_email: String,
    message: String,
    timestamp: i64,
}

#[derive(Debug, Deserialize)]
struct CommitDetailData {
    sha: String,
    author_name: String,
    author_email: String,
    authored_at: i64,
    committer_name: String,
    committer_email: String,
    committed_at: i64,
    message: String,
    parents: Vec<String>,
    ref_name: String,
    changes: Vec<DiffEntryData>,
}

#[derive(Debug, Deserialize)]
struct DiffResponseData {
    entries: Vec<DiffEntryData>,
}

#[derive(Debug, Deserialize)]
struct DiffEntryData {
    path: String,
    change_type: String,
    old_oid: String,
    new_oid: String,
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
    run_migration_sql(&pool, MIGRATION_010).await;
    run_migration_sql(&pool, MIGRATION_011).await;
    run_migration_sql(&pool, MIGRATION_012).await;

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
            secret: "test_secret_key_for_repo_context_e2e_32chars!!".to_string(),
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
        git_repo_repo: Arc::new(SqliteGitRepoRepository::new(pool.clone()))
            as Arc<dyn GitRepoRepository>,
        outbox_repo: Arc::new(SqliteOutboxRepository::new(pool)),
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

async fn create_test_api_key(
    repo: &dyn ApiKeyRepository,
    tenant_id: Uuid,
    user_id: Uuid,
    key_value: &str,
) {
    repo.create(NewApiKey {
        tenant_id,
        user_id,
        name: "test-repo-context-key".to_string(),
        key_hash: hash_apikey(key_value),
        key_prefix: "evo_sk".to_string(),
        permissions: vec!["repo:read".to_string(), "repo:write".to_string()],
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

fn run_git(
    args: &[&str],
    dir: &std::path::Path,
    env_vars: &[(&str, &str)],
) -> std::process::Output {
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

fn push_to_bare_repo(
    bare_path: &std::path::Path,
    work_dir: &TempDir,
    file_name: &str,
    content: &str,
    commit_msg: &str,
) {
    let work_path = work_dir.path();

    let output = run_git(&["init"], work_path, &[]);
    assert!(
        output.status.success(),
        "git init failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let output = run_git(
        &["config", "user.email", "test@example.com"],
        work_path,
        &[],
    );
    assert!(output.status.success());

    let output = run_git(&["config", "user.name", "Test"], work_path, &[]);
    assert!(output.status.success());

    let output = run_git(
        &["remote", "add", "origin", bare_path.to_str().unwrap()],
        work_path,
        &[],
    );
    assert!(output.status.success());

    let file_path = work_path.join(file_name);
    std::fs::write(&file_path, content).expect("Failed to write test file");

    let output = run_git(&["add", file_name], work_path, &[]);
    assert!(
        output.status.success(),
        "git add failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let output = run_git(&["commit", "-m", commit_msg], work_path, &[]);
    assert!(
        output.status.success(),
        "git commit failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let output = run_git(&["push", "origin", "HEAD:main"], work_path, &[]);
    assert!(
        output.status.success(),
        "git push failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn push_second_commit(work_dir: &TempDir, file_name: &str, content: &str, commit_msg: &str) {
    let work_path = work_dir.path();

    let file_path = work_path.join(file_name);
    std::fs::write(&file_path, content).expect("Failed to write test file");

    let output = run_git(&["add", file_name], work_path, &[]);
    assert!(
        output.status.success(),
        "git add failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let output = run_git(&["commit", "-m", commit_msg], work_path, &[]);
    assert!(
        output.status.success(),
        "git commit failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let output = run_git(&["push", "origin", "HEAD:main"], work_path, &[]);
    assert!(
        output.status.success(),
        "git push failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[actix_rt::test]
async fn test_file_tree_returns_entries_for_ref_main() {
    let (pool, _db_dir) = setup_test_db().await;
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path().to_string_lossy().to_string();

    let api_key_value = "evo_sk_repo_context_test_01";
    let (token, tenant_id, _user_id) = register_and_create_api_key(
        pool.clone(),
        base_path.clone(),
        "filetree@example.com",
        "filetree",
        api_key_value,
    )
    .await;

    let repo_id = create_repo_via_api(
        pool.clone(),
        base_path.clone(),
        &token,
        &tenant_id,
        "filetree-repo",
    )
    .await;

    let repo_uuid = Uuid::parse_str(&repo_id).expect("Invalid repo UUID");
    let tenant_uuid = Uuid::parse_str(&tenant_id).expect("Invalid tenant UUID");
    let bare_path = temp_dir
        .path()
        .join(tenant_uuid.to_string())
        .join(format!("{}.git", repo_uuid));

    let work_dir = TempDir::new().expect("Failed to create work dir");
    push_to_bare_repo(
        &bare_path,
        &work_dir,
        "README.md",
        "# Hello\n",
        "Initial commit",
    );

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
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

    let body = test::read_body(resp).await;
    let result: ApiResponse<FileTreeResponseData> =
        serde_json::from_slice(&body).expect("Failed to parse response");
    assert!(result.success);
    let data = result.data.expect("Expected data");
    assert!(!data.entries.is_empty(), "file-tree should have entries");
    let has_readme = data.entries.iter().any(|e| e.name == "README.md");
    assert!(has_readme, "file-tree should contain README.md");
}

#[actix_rt::test]
async fn test_blob_by_sha_returns_content() {
    let (pool, _db_dir) = setup_test_db().await;
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path().to_string_lossy().to_string();

    let api_key_value = "evo_sk_repo_context_test_02";
    let (token, tenant_id, _user_id) = register_and_create_api_key(
        pool.clone(),
        base_path.clone(),
        "blob@example.com",
        "blobtest",
        api_key_value,
    )
    .await;

    let repo_id = create_repo_via_api(
        pool.clone(),
        base_path.clone(),
        &token,
        &tenant_id,
        "blob-repo",
    )
    .await;

    let repo_uuid = Uuid::parse_str(&repo_id).expect("Invalid repo UUID");
    let tenant_uuid = Uuid::parse_str(&tenant_id).expect("Invalid tenant UUID");
    let bare_path = temp_dir
        .path()
        .join(tenant_uuid.to_string())
        .join(format!("{}.git", repo_uuid));

    let work_dir = TempDir::new().expect("Failed to create work dir");
    push_to_bare_repo(
        &bare_path,
        &work_dir,
        "hello.txt",
        "Hello World\n",
        "Add hello",
    );

    let app_state = build_app_state(pool.clone(), base_path.clone());
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
    let body = test::read_body(resp).await;
    let tree_result: ApiResponse<FileTreeResponseData> =
        serde_json::from_slice(&body).expect("Failed to parse tree response");
    let tree_data = tree_result.data.expect("Expected tree data");
    let blob_oid = tree_data
        .entries
        .iter()
        .find(|e| e.name == "hello.txt")
        .expect("hello.txt should exist")
        .oid
        .clone();

    let req = test::TestRequest::get()
        .uri(&format!(
            "/api/v1/tenant/{}/repos/{}/blobs/{}",
            tenant_id, repo_id, blob_oid
        ))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

    let body = test::read_body(resp).await;
    let result: ApiResponse<BlobResponseData> =
        serde_json::from_slice(&body).expect("Failed to parse blob response");
    assert!(result.success);
    let data = result.data.expect("Expected blob data");
    assert_eq!(data.content, "Hello World\n");
    assert_eq!(data.encoding, "utf-8");
    assert_eq!(data.size, 12);
}

#[actix_rt::test]
async fn test_commits_returns_list_sorted_by_time_desc() {
    let (pool, _db_dir) = setup_test_db().await;
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path().to_string_lossy().to_string();

    let api_key_value = "evo_sk_repo_context_test_03";
    let (token, tenant_id, _user_id) = register_and_create_api_key(
        pool.clone(),
        base_path.clone(),
        "commits@example.com",
        "committest",
        api_key_value,
    )
    .await;

    let repo_id = create_repo_via_api(
        pool.clone(),
        base_path.clone(),
        &token,
        &tenant_id,
        "commits-repo",
    )
    .await;

    let repo_uuid = Uuid::parse_str(&repo_id).expect("Invalid repo UUID");
    let tenant_uuid = Uuid::parse_str(&tenant_id).expect("Invalid tenant UUID");
    let bare_path = temp_dir
        .path()
        .join(tenant_uuid.to_string())
        .join(format!("{}.git", repo_uuid));

    let work_dir = TempDir::new().expect("Failed to create work dir");
    push_to_bare_repo(&bare_path, &work_dir, "a.txt", "A\n", "First commit");
    std::thread::sleep(std::time::Duration::from_secs(1));
    push_second_commit(&work_dir, "b.txt", "B\n", "Second commit");

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
            "/api/v1/tenant/{}/repos/{}/commits?ref=main",
            tenant_id, repo_id
        ))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

    let body = test::read_body(resp).await;
    let result: ApiResponse<CommitListResponseData> =
        serde_json::from_slice(&body).expect("Failed to parse commits response");
    assert!(result.success);
    let data = result.data.expect("Expected commits data");
    assert_eq!(data.commits.len(), 2);
    assert_eq!(data.commits[0].message, "Second commit\n");
    assert_eq!(data.commits[1].message, "First commit\n");
}

#[actix_rt::test]
async fn test_commit_detail_returns_verified_metadata_parents_and_changes() {
    let (pool, _db_dir) = setup_test_db().await;
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path().to_string_lossy().to_string();
    let (token, tenant_id, _user_id) = register_and_create_api_key(
        pool.clone(),
        base_path.clone(),
        "commit-detail@example.com",
        "commitdetail",
        "evo_sk_repo_context_commit_detail",
    )
    .await;
    let repo_id = create_repo_via_api(
        pool.clone(),
        base_path.clone(),
        &token,
        &tenant_id,
        "commit-detail-repo",
    )
    .await;
    let bare_path = temp_dir
        .path()
        .join(&tenant_id)
        .join(format!("{}.git", repo_id));
    let work_dir = TempDir::new().expect("Failed to create work dir");
    push_to_bare_repo(&bare_path, &work_dir, "a.txt", "A\n", "First commit");
    push_second_commit(&work_dir, "b.txt", "B\n", "Second commit");

    let app_state = build_app_state(pool, base_path);
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .configure(routes::configure_routes),
    )
    .await;

    let list_req = test::TestRequest::get()
        .uri(&format!(
            "/api/v1/tenant/{}/repos/{}/commits?ref=main",
            tenant_id, repo_id
        ))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let list_body = test::read_body(test::call_service(&app, list_req).await).await;
    let list: ApiResponse<CommitListResponseData> =
        serde_json::from_slice(&list_body).expect("Failed to parse commits response");
    let commits = list.data.expect("Expected commits").commits;
    let head = &commits[0].sha;
    let parent = &commits[1].sha;

    let req = test::TestRequest::get()
        .uri(&format!(
            "/api/v1/tenant/{}/repos/{}/commits/{}?ref=main",
            tenant_id, repo_id, head
        ))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
    let body = test::read_body(resp).await;
    let result: ApiResponse<CommitDetailData> =
        serde_json::from_slice(&body).expect("Failed to parse commit detail");
    let detail = result.data.expect("Expected commit detail");
    assert_eq!(detail.sha, *head);
    assert_eq!(detail.parents, vec![parent.clone()]);
    assert_eq!(detail.ref_name, "main");
    assert_eq!(detail.message, "Second commit\n");
    assert_eq!(detail.author_name, "Test");
    assert_eq!(detail.author_email, "test@example.com");
    assert_eq!(detail.committer_name, "Test");
    assert_eq!(detail.committer_email, "test@example.com");
    assert!(detail.authored_at > 0);
    assert!(detail.committed_at > 0);
    assert!(detail
        .changes
        .iter()
        .any(|change| change.path == "b.txt" && change.change_type == "added"));

    let root_req = test::TestRequest::get()
        .uri(&format!(
            "/api/v1/tenant/{}/repos/{}/commits/{}?ref=main",
            tenant_id, repo_id, parent
        ))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let root_body = test::read_body(test::call_service(&app, root_req).await).await;
    let root_result: ApiResponse<CommitDetailData> =
        serde_json::from_slice(&root_body).expect("Failed to parse root commit detail");
    let root = root_result.data.expect("Expected root commit detail");
    assert!(root.parents.is_empty());
    assert!(root
        .changes
        .iter()
        .any(|change| change.path == "a.txt" && change.change_type == "added"));

    let unreachable_req = test::TestRequest::get()
        .uri(&format!(
            "/api/v1/tenant/{}/repos/{}/commits/{}?ref={}",
            tenant_id, repo_id, head, parent
        ))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let unreachable_resp = test::call_service(&app, unreachable_req).await;
    assert_eq!(
        unreachable_resp.status(),
        actix_web::http::StatusCode::NOT_FOUND
    );

    let invalid_req = test::TestRequest::get()
        .uri(&format!(
            "/api/v1/tenant/{}/repos/{}/commits/not-a-sha?ref=main",
            tenant_id, repo_id
        ))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let invalid_resp = test::call_service(&app, invalid_req).await;
    assert_eq!(
        invalid_resp.status(),
        actix_web::http::StatusCode::BAD_REQUEST
    );
}

#[actix_rt::test]
async fn test_diff_between_two_refs_returns_changes() {
    let (pool, _db_dir) = setup_test_db().await;
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path().to_string_lossy().to_string();

    let api_key_value = "evo_sk_repo_context_test_04";
    let (token, tenant_id, _user_id) = register_and_create_api_key(
        pool.clone(),
        base_path.clone(),
        "diff@example.com",
        "difftest",
        api_key_value,
    )
    .await;

    let repo_id = create_repo_via_api(
        pool.clone(),
        base_path.clone(),
        &token,
        &tenant_id,
        "diff-repo",
    )
    .await;

    let repo_uuid = Uuid::parse_str(&repo_id).expect("Invalid repo UUID");
    let tenant_uuid = Uuid::parse_str(&tenant_id).expect("Invalid tenant UUID");
    let bare_path = temp_dir
        .path()
        .join(tenant_uuid.to_string())
        .join(format!("{}.git", repo_uuid));

    let work_dir = TempDir::new().expect("Failed to create work dir");
    push_to_bare_repo(&bare_path, &work_dir, "a.txt", "A\n", "First commit");

    let app_state = build_app_state(pool.clone(), base_path.clone());
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .configure(routes::configure_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri(&format!(
            "/api/v1/tenant/{}/repos/{}/commits?ref=main&limit=2",
            tenant_id, repo_id
        ))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    let body = test::read_body(resp).await;
    let commits_result: ApiResponse<CommitListResponseData> =
        serde_json::from_slice(&body).expect("Failed to parse commits response");
    let commits_data = commits_result.data.expect("Expected commits data");
    let head_sha = commits_data.commits[0].sha.clone();

    push_second_commit(&work_dir, "b.txt", "B\n", "Second commit");

    let req = test::TestRequest::get()
        .uri(&format!(
            "/api/v1/tenant/{}/repos/{}/diff?base={}&head=main",
            tenant_id, repo_id, head_sha
        ))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

    let body = test::read_body(resp).await;
    let result: ApiResponse<DiffResponseData> =
        serde_json::from_slice(&body).expect("Failed to parse diff response");
    assert!(result.success);
    let data = result.data.expect("Expected diff data");
    assert!(!data.entries.is_empty(), "diff should have entries");
    let has_b_txt = data
        .entries
        .iter()
        .any(|e| e.path == "b.txt" && e.change_type == "added");
    assert!(has_b_txt, "diff should show b.txt as added");
}

#[actix_rt::test]
async fn test_ref_defaults_to_repo_default_branch() {
    let (pool, _db_dir) = setup_test_db().await;
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path().to_string_lossy().to_string();

    let api_key_value = "evo_sk_repo_context_test_05";
    let (token, tenant_id, _user_id) = register_and_create_api_key(
        pool.clone(),
        base_path.clone(),
        "defaultref@example.com",
        "defaultreftest",
        api_key_value,
    )
    .await;

    let repo_id = create_repo_via_api(
        pool.clone(),
        base_path.clone(),
        &token,
        &tenant_id,
        "default-ref-repo",
    )
    .await;

    let repo_uuid = Uuid::parse_str(&repo_id).expect("Invalid repo UUID");
    let tenant_uuid = Uuid::parse_str(&tenant_id).expect("Invalid tenant UUID");
    let bare_path = temp_dir
        .path()
        .join(tenant_uuid.to_string())
        .join(format!("{}.git", repo_uuid));

    let work_dir = TempDir::new().expect("Failed to create work dir");
    push_to_bare_repo(&bare_path, &work_dir, "README.md", "# Default\n", "Init");

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
            "/api/v1/tenant/{}/repos/{}/file-tree",
            tenant_id, repo_id
        ))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

    let body = test::read_body(resp).await;
    let result: ApiResponse<FileTreeResponseData> =
        serde_json::from_slice(&body).expect("Failed to parse response");
    assert!(result.success);
    let data = result.data.expect("Expected data");
    assert!(
        !data.entries.is_empty(),
        "file-tree should have entries without ?ref"
    );
}

#[actix_rt::test]
async fn test_cross_tenant_private_repo_returns_403() {
    let (pool, _db_dir) = setup_test_db().await;
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path().to_string_lossy().to_string();

    let api_key_value_a = "evo_sk_repo_context_test_06a";
    let (token_a, tenant_a, _user_id_a) = register_and_create_api_key(
        pool.clone(),
        base_path.clone(),
        "tenant-a-ctx@example.com",
        "tenantactx",
        api_key_value_a,
    )
    .await;

    let repo_id = create_repo_via_api(
        pool.clone(),
        base_path.clone(),
        &token_a,
        &tenant_a,
        "private-repo",
    )
    .await;

    let api_key_value_b = "evo_sk_repo_context_test_06b";
    let (token_b, tenant_b, _user_id_b) = register_and_create_api_key(
        pool.clone(),
        base_path.clone(),
        "tenant-b-ctx@example.com",
        "tenantbctx",
        api_key_value_b,
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
            tenant_b, repo_id
        ))
        .insert_header(("Authorization", format!("Bearer {}", token_b)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::FORBIDDEN);

    let detail_req = test::TestRequest::get()
        .uri(&format!(
            "/api/v1/tenant/{}/repos/{}/commits/{}?ref=main",
            tenant_b,
            repo_id,
            "0".repeat(40)
        ))
        .insert_header(("Authorization", format!("Bearer {}", token_b)))
        .to_request();
    let detail_resp = test::call_service(&app, detail_req).await;
    assert_eq!(detail_resp.status(), actix_web::http::StatusCode::FORBIDDEN);
}
