#![allow(clippy::unwrap_used, dead_code)]

//! EVO-116 / EVO-103-C performance benchmarks.
//!
//! Captures the two P95 numbers required by the iteration plan and prints them
//! in a markdown-friendly table at the end of the run. Designed to be
//! readable from `cargo test -p api --test repo_perf_benchmarks -- --nocapture`
//! so the iteration Review can copy the table directly.
//!
//! Sample sizes match the iteration plan:
//! - `file-tree (100 files)` × 50 iterations (5 warmup discarded) → P95
//! - `repo list (1000 repos)` × 50 iterations (5 warmup discarded) → P95

use actix_web::{middleware::from_fn, test, web, App};
use serde::Deserialize;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use std::time::Instant;
use tempfile::TempDir;

use api::routes;
use api::state::AppState;
use common::execution::{CompositeProvider, ExecutionProvider};
use domain::git_repo::{NewGitRepo, RepoVisibility};
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
            secret: "test_secret_key_for_perf_benchmarks_32chars".to_string(),
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

fn push_n_files_to_bare_repo(bare_path: &Path, count: usize) {
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
    assert!(run(&["init", "-b", "main"]).status.success());
    assert!(run(&["config", "user.email", "perf@example.com"])
        .status
        .success());
    assert!(run(&["config", "user.name", "Perf"]).status.success());
    assert!(
        run(&["remote", "add", "origin", bare_path.to_str().unwrap()])
            .status
            .success()
    );
    for i in 0..count {
        let name = format!("file_{:04}.txt", i);
        std::fs::write(work_path.join(&name), format!("content {}\n", i)).unwrap();
        assert!(run(&["add", &name]).status.success());
    }
    assert!(run(&["commit", "-m", "perf seed"]).status.success());
    assert!(run(&["push", "origin", "HEAD:main"]).status.success());
}

fn p95(samples: &[f64]) -> f64 {
    let mut sorted: Vec<f64> = samples.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let idx = ((sorted.len() as f64) * 0.95).ceil() as usize - 1;
    sorted[idx.min(sorted.len() - 1)]
}

fn fmt_table(rows: &[(&str, &str, &str, &str)]) -> String {
    let mut out = String::new();
    out.push_str("| Scenario | Iterations | Sample size | P95 (ms) |\n");
    out.push_str("|----------|------------|-------------|----------|\n");
    for (scenario, iters, sample, p95) in rows {
        out.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            scenario, iters, sample, p95
        ));
    }
    out
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
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
}

#[actix_rt::test]
async fn bench_file_tree_100_files_p95() {
    let pool = setup_test_db().await;
    let temp_dir = TempDir::new().expect("temp dir");
    let base_path = temp_dir.path().to_string_lossy().to_string();

    let app_state = build_app_state(pool.clone(), base_path.clone());
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .configure(routes::configure_routes),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/v1/auth/register")
        .set_json(serde_json::json!({
            "email": "perf-ft@example.com",
            "username": "perfft",
            "password": "TestPassword123!",
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    let auth: ApiResponse<AuthResponseData> =
        serde_json::from_slice(&body).expect("register parse");
    let auth_data = auth.data.unwrap();
    let token = auth_data.token;
    let tenant_id = auth_data.tenant.id;

    let create_req = test::TestRequest::post()
        .uri(&format!("/api/v1/tenant/{}/repos", tenant_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({"name": "perf-ft"}))
        .to_request();
    let resp = test::call_service(&app, create_req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::CREATED);
    let body = test::read_body(resp).await;
    let create_data: ApiResponse<serde_json::Value> = serde_json::from_slice(&body).unwrap();
    let repo_id = create_data
        .data
        .unwrap()
        .get("id")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();

    let tenant_uuid = uuid::Uuid::parse_str(&tenant_id).unwrap();
    let repo_uuid = uuid::Uuid::parse_str(&repo_id).unwrap();
    let bare_path = temp_dir
        .path()
        .join(tenant_uuid.to_string())
        .join(format!("{}.git", repo_uuid));
    push_n_files_to_bare_repo(&bare_path, 100);

    let app_state2 = build_app_state(pool, base_path);
    let app2 = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state2))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .configure(routes::configure_routes),
    )
    .await;

    let mut samples: Vec<f64> = Vec::with_capacity(50);
    for _ in 0..55 {
        let req = test::TestRequest::get()
            .uri(&format!(
                "/api/v1/tenant/{}/repos/{}/file-tree?ref=main",
                tenant_id, repo_id
            ))
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .to_request();
        let t0 = Instant::now();
        let resp = test::call_service(&app2, req).await;
        let elapsed = t0.elapsed().as_secs_f64() * 1000.0;
        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
        samples.push(elapsed);
    }
    samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let warmup_skip = 5;
    let measured: Vec<f64> = samples.iter().skip(warmup_skip).copied().collect();
    let p95_ms = p95(&measured);
    let min_ms = measured.first().copied().unwrap();
    let max_ms = measured.last().copied().unwrap();
    let median_ms = measured[measured.len() / 2];
    println!(
        "\nEVO116_PERF file-tree: samples={} warmup={} min={:.2}ms median={:.2}ms max={:.2}ms p95={:.2}ms",
        measured.len(),
        warmup_skip,
        min_ms,
        median_ms,
        max_ms,
        p95_ms
    );
    let table = fmt_table(&[(
        "file-tree (100 files, ?ref=main)",
        "50",
        "100 files",
        &format!("{:.2}", p95_ms),
    )]);
    println!("EVO116_PERF_TABLE\n{}", table);
    assert!(
        p95_ms < 200.0,
        "file-tree p95 must be < 200ms (EVO-116 BDD target was < 100ms in ideal case)"
    );
}

#[actix_rt::test]
async fn bench_repo_list_1000_repos_p95() {
    let pool = setup_test_db().await;
    let temp_dir = TempDir::new().expect("temp dir");
    let base_path = temp_dir.path().to_string_lossy().to_string();

    let app_state = build_app_state(pool.clone(), base_path.clone());
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .configure(routes::configure_routes),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/v1/auth/register")
        .set_json(serde_json::json!({
            "email": "perf-list@example.com",
            "username": "perflist",
            "password": "TestPassword123!",
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    let auth: ApiResponse<AuthResponseData> =
        serde_json::from_slice(&body).expect("register parse");
    let auth_data = auth.data.unwrap();
    let token = auth_data.token;
    let tenant_id = auth_data.tenant.id;

    let tenant_uuid = uuid::Uuid::parse_str(&tenant_id).unwrap();
    let repo_repo: Arc<dyn GitRepoRepository> =
        Arc::new(SqliteGitRepoRepository::new(pool.clone()));
    for i in 0..1000 {
        let name = format!("repo-{:04}", i);
        repo_repo
            .create(
                NewGitRepo {
                    name,
                    description: Some(format!("perf {}", i)),
                    default_branch: Some("main".to_string()),
                    visibility: Some(RepoVisibility::Private),
                    auto_merge: Some(false),
                    require_review: Some(true),
                },
                tenant_uuid,
            )
            .await
            .expect("seed repo create");
    }

    let app_state2 = build_app_state(pool, base_path);
    let app2 = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state2))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .configure(routes::configure_routes),
    )
    .await;

    let mut samples: Vec<f64> = Vec::with_capacity(50);
    for _ in 0..55 {
        let req = test::TestRequest::get()
            .uri(&format!("/api/v1/tenant/{}/repos", tenant_id))
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .to_request();
        let t0 = Instant::now();
        let resp = test::call_service(&app2, req).await;
        let elapsed = t0.elapsed().as_secs_f64() * 1000.0;
        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
        samples.push(elapsed);
    }
    samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let warmup_skip = 5;
    let measured: Vec<f64> = samples.iter().skip(warmup_skip).copied().collect();
    let p95_ms = p95(&measured);
    let min_ms = measured.first().copied().unwrap();
    let max_ms = measured.last().copied().unwrap();
    let median_ms = measured[measured.len() / 2];
    println!(
        "\nEVO116_PERF repo-list: samples={} warmup={} min={:.2}ms median={:.2}ms max={:.2}ms p95={:.2}ms",
        measured.len(),
        warmup_skip,
        min_ms,
        median_ms,
        max_ms,
        p95_ms
    );
    let table = fmt_table(&[(
        "repo list (1000 repos, single tenant)",
        "50",
        "1000 repos",
        &format!("{:.2}", p95_ms),
    )]);
    println!("EVO116_PERF_TABLE\n{}", table);
    assert!(
        p95_ms < 500.0,
        "repo list p95 must be < 500ms (EVO-116 BDD target was < 200ms in ideal case)"
    );
}
