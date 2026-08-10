#![allow(clippy::unwrap_used, dead_code)]

use actix_web::{middleware::from_fn, test, web, App};
use serde::Deserialize;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::sync::Arc;
use tempfile::TempDir;

use api::routes;
use api::state::AppState;
use common::execution::{CompositeProvider, ExecutionProvider};
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
    lifecycle_status: String,
}

#[derive(Debug, Deserialize)]
struct RepoListData {
    repos: Vec<RepoResponseData>,
    total: usize,
}

#[derive(Debug, Deserialize)]
struct ReconcileEntryData {
    repo_id: String,
    classification: String,
    action: String,
    quarantine_path: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ReconcileData {
    apply: bool,
    consistent: usize,
    db_only: usize,
    disk_only: usize,
    entries: Vec<ReconcileEntryData>,
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
            secret: "test_secret_key_for_repo_e2e_testing_32chars!!".to_string(),
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

#[actix_rt::test]
async fn test_create_repo_returns_201_with_disk_repo() {
    let pool = setup_test_db().await;
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path().to_string_lossy().to_string();
    let app_state = build_app_state(pool, base_path);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .configure(routes::configure_routes),
    )
    .await;

    let register_payload = serde_json::json!({
        "email": "create-repo@example.com",
        "username": "createrepo",
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
    let auth_data = register_result.data.expect("Expected auth data");
    let token = auth_data.token;
    let tenant_id = auth_data.tenant.id;

    let req = test::TestRequest::post()
        .uri(&format!("/api/v1/tenant/{}/repos", tenant_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "name": "my-repo",
            "description": "Test repo",
            "seed_template": true,
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::CREATED);

    let body = test::read_body(resp).await;
    let result: ApiResponse<RepoResponseData> =
        serde_json::from_slice(&body).expect("Failed to parse response");
    assert!(result.success);
    let data = result.data.expect("Expected data");
    assert_eq!(data.name, "my-repo");
    assert_eq!(data.lifecycle_status, "ACTIVE");
    assert!(!data.id.is_empty());

    let repo_id = Uuid::parse_str(&data.id).expect("Invalid repo UUID");
    let tenant_uuid = Uuid::parse_str(&tenant_id).expect("Invalid tenant UUID");
    let disk_path = temp_dir
        .path()
        .join(tenant_uuid.to_string())
        .join(format!("{}.git", repo_id));
    assert!(
        disk_path.exists(),
        "Disk bare repo should exist at {:?}",
        disk_path
    );

    let (sha, _) = service_git::resolve_ref(&disk_path, "refs/heads/main")
        .expect("seed_template should create main ref");
    let paths: Vec<_> = service_git::read_file_tree(&disk_path, &sha)
        .expect("seed commit tree should be readable")
        .into_iter()
        .map(|entry| entry.name)
        .collect();
    assert!(paths.contains(&"README.md".to_string()));
    assert!(paths.contains(&".evolith/policy.yaml".to_string()));
    assert!(paths.contains(&".evolith/agents.yaml".to_string()));
}

#[actix_rt::test]
async fn test_create_repo_without_seed_template_has_no_seed_files() {
    let pool = setup_test_db().await;
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path().to_string_lossy().to_string();
    let app_state = build_app_state(pool, base_path);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .configure(routes::configure_routes),
    )
    .await;

    let register_payload = serde_json::json!({
        "email": "no-seed@example.com",
        "username": "noseed",
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
    let auth_data = register_result.data.expect("Expected auth data");
    let token = auth_data.token;
    let tenant_id = auth_data.tenant.id;

    let req = test::TestRequest::post()
        .uri(&format!("/api/v1/tenant/{}/repos", tenant_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "name": "no-seed-repo",
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::CREATED);

    let body = test::read_body(resp).await;
    let result: ApiResponse<RepoResponseData> =
        serde_json::from_slice(&body).expect("Failed to parse response");
    let data = result.data.expect("Expected data");
    assert_eq!(data.lifecycle_status, "ACTIVE");
    let repo_id = Uuid::parse_str(&data.id).expect("Invalid repo UUID");
    let tenant_uuid = Uuid::parse_str(&tenant_id).expect("Invalid tenant UUID");
    let disk_path = temp_dir
        .path()
        .join(tenant_uuid.to_string())
        .join(format!("{}.git", repo_id));
    assert!(disk_path.exists(), "Disk bare repo should exist");

    assert!(
        service_git::resolve_ref(&disk_path, "refs/heads/main").is_err(),
        "repo without seed_template should not contain an initial commit"
    );
}

#[actix_rt::test]
async fn test_reconcile_classifies_and_repairs_db_disk_split() {
    let pool = setup_test_db().await;
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path().to_string_lossy().to_string();
    let app_state = build_app_state(pool, base_path.clone());
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
            "email": "reconcile@example.com",
            "username": "reconcile",
            "password": "TestPassword123!",
        }))
        .to_request();
    let register_resp = test::call_service(&app, register_req).await;
    assert!(register_resp.status().is_success());
    let register_body = test::read_body(register_resp).await;
    let auth_data: AuthResponseData =
        serde_json::from_slice::<ApiResponse<AuthResponseData>>(&register_body)
            .unwrap()
            .data
            .unwrap();
    let tenant_id = Uuid::parse_str(&auth_data.tenant.id).unwrap();
    let token = auth_data.token;

    let create_req = test::TestRequest::post()
        .uri(&format!("/api/v1/tenant/{tenant_id}/repos"))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(serde_json::json!({"name": "db-only"}))
        .to_request();
    let create_resp = test::call_service(&app, create_req).await;
    assert_eq!(create_resp.status(), actix_web::http::StatusCode::CREATED);
    let created: RepoResponseData = serde_json::from_slice::<ApiResponse<RepoResponseData>>(
        &test::read_body(create_resp).await,
    )
    .unwrap()
    .data
    .unwrap();
    let db_only_id = Uuid::parse_str(&created.id).unwrap();
    let db_only_path = temp_dir
        .path()
        .join(tenant_id.to_string())
        .join(format!("{db_only_id}.git"));
    std::fs::remove_dir_all(&db_only_path).unwrap();

    let disk_only_id = Uuid::new_v4();
    service_git::init_bare_repo(temp_dir.path(), tenant_id, disk_only_id).unwrap();

    let dry_run_req = test::TestRequest::post()
        .uri(&format!("/api/v1/tenant/{tenant_id}/repos/reconcile"))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(serde_json::json!({"apply": false}))
        .to_request();
    let dry_run_resp = test::call_service(&app, dry_run_req).await;
    assert_eq!(dry_run_resp.status(), actix_web::http::StatusCode::OK);
    let dry_run: ReconcileData =
        serde_json::from_slice::<ApiResponse<ReconcileData>>(&test::read_body(dry_run_resp).await)
            .unwrap()
            .data
            .unwrap();
    assert!(!dry_run.apply);
    assert_eq!(dry_run.db_only, 1);
    assert_eq!(dry_run.disk_only, 1);
    assert!(dry_run.consistent == 0);
    assert!(dry_run.entries.iter().all(|entry| entry.action == "NONE"));

    let apply_req = test::TestRequest::post()
        .uri(&format!("/api/v1/tenant/{tenant_id}/repos/reconcile"))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(serde_json::json!({"apply": true}))
        .to_request();
    let apply_resp = test::call_service(&app, apply_req).await;
    assert_eq!(apply_resp.status(), actix_web::http::StatusCode::OK);
    let applied: ReconcileData =
        serde_json::from_slice::<ApiResponse<ReconcileData>>(&test::read_body(apply_resp).await)
            .unwrap()
            .data
            .unwrap();
    assert!(applied.apply);
    assert!(applied
        .entries
        .iter()
        .any(|entry| entry.repo_id == db_only_id.to_string() && entry.action == "MARKED_ERROR"));
    let quarantined = applied
        .entries
        .iter()
        .find(|entry| entry.repo_id == disk_only_id.to_string())
        .and_then(|entry| entry.quarantine_path.as_ref())
        .expect("disk-only repo should have quarantine path");
    assert_eq!(
        applied
            .entries
            .iter()
            .find(|entry| entry.repo_id == disk_only_id.to_string())
            .unwrap()
            .classification,
        "DISK_ONLY"
    );
    assert!(temp_dir.path().join(quarantined).exists());
    assert!(!db_only_path.exists());
}

#[actix_rt::test]
async fn test_reconcile_rejects_member_and_cross_tenant_jwt() {
    let pool = setup_test_db().await;
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path().to_string_lossy().to_string();
    let jwt = JwtHandler::new(&create_test_config(base_path.clone()).jwt);
    let app_state = build_app_state(pool, base_path);
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .configure(routes::configure_routes),
    )
    .await;

    let target_register_req = test::TestRequest::post()
        .uri("/api/v1/auth/register")
        .set_json(serde_json::json!({
            "email": "reconcile-target@example.com",
            "username": "reconciletarget",
            "password": "TestPassword123!",
        }))
        .to_request();
    let target_register_resp = test::call_service(&app, target_register_req).await;
    assert!(target_register_resp.status().is_success());
    let target_auth: AuthResponseData = serde_json::from_slice::<ApiResponse<AuthResponseData>>(
        &test::read_body(target_register_resp).await,
    )
    .unwrap()
    .data
    .unwrap();
    let target_tenant_id = Uuid::parse_str(&target_auth.tenant.id).unwrap();
    let target_user_id = Uuid::parse_str(&target_auth.user.id).unwrap();

    let (member_token, _) = jwt
        .generate_token(target_user_id, "user", target_tenant_id, "member")
        .unwrap();
    let member_req = test::TestRequest::post()
        .uri(&format!(
            "/api/v1/tenant/{target_tenant_id}/repos/reconcile"
        ))
        .insert_header(("Authorization", format!("Bearer {member_token}")))
        .set_json(serde_json::json!({"apply": true}))
        .to_request();
    let member_resp = test::call_service(&app, member_req).await;
    assert_eq!(member_resp.status(), actix_web::http::StatusCode::FORBIDDEN);

    let foreign_register_req = test::TestRequest::post()
        .uri("/api/v1/auth/register")
        .set_json(serde_json::json!({
            "email": "reconcile-foreign@example.com",
            "username": "reconcileforeign",
            "password": "TestPassword123!",
        }))
        .to_request();
    let foreign_register_resp = test::call_service(&app, foreign_register_req).await;
    assert!(foreign_register_resp.status().is_success());
    let foreign_auth: AuthResponseData = serde_json::from_slice::<ApiResponse<AuthResponseData>>(
        &test::read_body(foreign_register_resp).await,
    )
    .unwrap()
    .data
    .unwrap();

    let cross_tenant_req = test::TestRequest::post()
        .uri(&format!(
            "/api/v1/tenant/{target_tenant_id}/repos/reconcile"
        ))
        .insert_header(("Authorization", format!("Bearer {}", foreign_auth.token)))
        .set_json(serde_json::json!({"apply": true}))
        .to_request();
    let cross_tenant_resp = test::call_service(&app, cross_tenant_req).await;
    assert_eq!(
        cross_tenant_resp.status(),
        actix_web::http::StatusCode::FORBIDDEN
    );
}

#[actix_rt::test]
async fn test_list_repos_returns_only_caller_tenant() {
    let pool = setup_test_db().await;
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path().to_string_lossy().to_string();
    let app_state = build_app_state(pool, base_path);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .configure(routes::configure_routes),
    )
    .await;

    let register_payload = serde_json::json!({
        "email": "list-repos@example.com",
        "username": "listrepos",
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
    let auth_data = register_result.data.expect("Expected auth data");
    let token = auth_data.token;
    let tenant_id = auth_data.tenant.id;

    for name in ["repo-a", "repo-b"] {
        let req = test::TestRequest::post()
            .uri(&format!("/api/v1/tenant/{}/repos", tenant_id))
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .set_json(serde_json::json!({
                "name": name,
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    let req = test::TestRequest::get()
        .uri(&format!("/api/v1/tenant/{}/repos", tenant_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

    let body = test::read_body(resp).await;
    let result: ApiResponse<RepoListData> =
        serde_json::from_slice(&body).expect("Failed to parse response");
    assert!(result.success);
    let data = result.data.expect("Expected data");
    assert_eq!(data.total, 2);
}

#[actix_rt::test]
async fn test_cross_tenant_get_returns_403() {
    let pool = setup_test_db().await;
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path().to_string_lossy().to_string();
    let app_state = build_app_state(pool, base_path);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .configure(routes::configure_routes),
    )
    .await;

    let register_payload = serde_json::json!({
        "email": "tenant-a@example.com",
        "username": "tenanta",
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
    let auth_data_a = register_result.data.expect("Expected auth data");
    let token_a = auth_data_a.token;
    let tenant_a = auth_data_a.tenant.id;

    let req = test::TestRequest::post()
        .uri(&format!("/api/v1/tenant/{}/repos", tenant_a))
        .insert_header(("Authorization", format!("Bearer {}", token_a)))
        .set_json(serde_json::json!({
            "name": "tenant-a-repo",
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    let result: ApiResponse<RepoResponseData> =
        serde_json::from_slice(&body).expect("Failed to parse create response");
    let repo_id = result.data.expect("Expected data").id;

    let register_payload2 = serde_json::json!({
        "email": "tenant-b@example.com",
        "username": "tenantb",
        "password": "TestPassword123!",
    });
    let register_req2 = test::TestRequest::post()
        .uri("/api/v1/auth/register")
        .set_json(&register_payload2)
        .to_request();
    let register_resp2 = test::call_service(&app, register_req2).await;
    assert!(register_resp2.status().is_success());
    let register_body2 = test::read_body(register_resp2).await;
    let register_result2: ApiResponse<AuthResponseData> =
        serde_json::from_slice(&register_body2).expect("Failed to parse register response");
    let auth_data_b = register_result2.data.expect("Expected auth data");
    let token_b = auth_data_b.token;
    let tenant_b = auth_data_b.tenant.id;

    let req = test::TestRequest::get()
        .uri(&format!("/api/v1/tenant/{}/repos/{}", tenant_b, repo_id))
        .insert_header(("Authorization", format!("Bearer {}", token_b)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::FORBIDDEN);
}

#[actix_rt::test]
async fn test_duplicate_name_returns_409() {
    let pool = setup_test_db().await;
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path().to_string_lossy().to_string();
    let app_state = build_app_state(pool, base_path);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .configure(routes::configure_routes),
    )
    .await;

    let register_payload = serde_json::json!({
        "email": "dup-repo@example.com",
        "username": "duprepo",
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
    let auth_data = register_result.data.expect("Expected auth data");
    let token = auth_data.token;
    let tenant_id = auth_data.tenant.id;

    let req = test::TestRequest::post()
        .uri(&format!("/api/v1/tenant/{}/repos", tenant_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "name": "dup-repo",
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let req2 = test::TestRequest::post()
        .uri(&format!("/api/v1/tenant/{}/repos", tenant_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "name": "dup-repo",
        }))
        .to_request();
    let resp2 = test::call_service(&app, req2).await;
    assert_eq!(resp2.status(), actix_web::http::StatusCode::CONFLICT);

    let body = test::read_body(resp2).await;
    let result: ApiResponse<serde_json::Value> =
        serde_json::from_slice(&body).expect("Failed to parse response");
    assert_eq!(result.error.expect("Expected error").code, "REPO_EXISTS");
}

#[actix_rt::test]
async fn test_update_repo_returns_200() {
    let pool = setup_test_db().await;
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path().to_string_lossy().to_string();
    let app_state = build_app_state(pool, base_path);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .configure(routes::configure_routes),
    )
    .await;

    let register_payload = serde_json::json!({
        "email": "update-repo@example.com",
        "username": "updaterepo",
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
    let auth_data = register_result.data.expect("Expected auth data");
    let token = auth_data.token;
    let tenant_id = auth_data.tenant.id;

    let req = test::TestRequest::post()
        .uri(&format!("/api/v1/tenant/{}/repos", tenant_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "name": "update-me",
            "description": "Original",
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    let result: ApiResponse<RepoResponseData> =
        serde_json::from_slice(&body).expect("Failed to parse create response");
    let repo_id = result.data.expect("Expected data").id;

    let req = test::TestRequest::patch()
        .uri(&format!("/api/v1/tenant/{}/repos/{}", tenant_id, repo_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "description": "Updated description",
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

    let body = test::read_body(resp).await;
    let result: ApiResponse<serde_json::Value> =
        serde_json::from_slice(&body).expect("Failed to parse update response");
    assert!(result.success);
    let updated = result.data.expect("Expected data");
    assert_eq!(updated["description"], "Updated description");
}

#[actix_rt::test]
async fn test_delete_repo_returns_204_and_removes_disk() {
    let pool = setup_test_db().await;
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path().to_string_lossy().to_string();
    let app_state = build_app_state(pool, base_path);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .configure(routes::configure_routes),
    )
    .await;

    let register_payload = serde_json::json!({
        "email": "delete-repo@example.com",
        "username": "deleterepo",
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
    let auth_data = register_result.data.expect("Expected auth data");
    let token = auth_data.token;
    let tenant_id = auth_data.tenant.id;

    let req = test::TestRequest::post()
        .uri(&format!("/api/v1/tenant/{}/repos", tenant_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "name": "delete-me",
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    let result: ApiResponse<RepoResponseData> =
        serde_json::from_slice(&body).expect("Failed to parse create response");
    let repo_id = result.data.expect("Expected data").id;
    let repo_uuid = Uuid::parse_str(&repo_id).expect("Invalid repo UUID");
    let tenant_uuid = Uuid::parse_str(&tenant_id).expect("Invalid tenant UUID");

    let disk_path = temp_dir
        .path()
        .join(tenant_uuid.to_string())
        .join(format!("{}.git", repo_uuid));
    assert!(disk_path.exists(), "Disk repo should exist before delete");

    let req = test::TestRequest::delete()
        .uri(&format!("/api/v1/tenant/{}/repos/{}", tenant_id, repo_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::NO_CONTENT);

    assert!(
        !disk_path.exists(),
        "Disk bare repo should be removed after delete"
    );

    let req = test::TestRequest::get()
        .uri(&format!("/api/v1/tenant/{}/repos/{}", tenant_id, repo_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::NOT_FOUND);
}
