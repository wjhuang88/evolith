//! MCP tool execution integration tests

use actix_web::{test, web, App};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::sync::Arc;

use api::middleware::rbac::RbacMiddleware;
use api::routes;
use api::state::AppState;
use domain::repository::{
    ApiKeyRepository, AuditRepository, InvitationRepository, SkillRepository, SnippetRepository,
    TenantRepository, ToolRepository, UserRepository,
};
use infra::config::{
    AppConfig, AppMetaConfig, DatabaseConfig, JwtConfig, LogConfig, RateLimitConfig, RedisConfig,
    SandboxConfig, ServerConfig, SmtpConfig, StorageConfig, StripeConfig,
};
use infra::db::{
    SqliteApiKeyRepository, SqliteAuditRepository, SqliteInvitationRepository,
    SqliteSkillRepository, SqliteSnippetRepository, SqliteTenantRepository, SqliteToolRepository,
    SqliteUserRepository,
};
use service_auth::{Argon2Hasher, JwtHandler};
use service_skill::executor::{DefaultSkillExecutor, SkillExecutor};
use service_tool::executor::{HttpToolExecutor, ToolExecutor};
use uuid::Uuid;

const MIGRATION_001: &str = include_str!("../../../migrations/sqlite/001_initial_schema.sql");
const MIGRATION_003: &str = include_str!("../../../migrations/sqlite/003_multi_tenant.sql");
const MIGRATION_004: &str = include_str!("../../../migrations/sqlite/004_user_permissions.sql");
const MIGRATION_005: &str = include_str!("../../../migrations/sqlite/005_payment_integration.sql");

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
        sqlx::query(without_comments)
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
            secret: "test_secret_key_for_mcp_e2e_testing_32chars!!".to_string(),
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
        api_key_repo: Arc::new(SqliteApiKeyRepository::new(pool)) as Arc<dyn ApiKeyRepository>,
        cache: Arc::new(infra::cache::InMemoryCache::new()),
        mailer: Arc::new(infra::mailer::ConsoleMailer),
        skill_executor: Arc::new(DefaultSkillExecutor::new()) as Arc<dyn SkillExecutor>,
        tool_executor: Arc::new(HttpToolExecutor::new()) as Arc<dyn ToolExecutor>,
    }
}

#[actix_rt::test]
async fn test_mcp_tools_call_nonexistent_tool() {
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

    let mcp_request = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": "nonexistent-tool",
            "arguments": {}
        }
    });

    let req = test::TestRequest::post()
        .uri("/mcp")
        .set_json(&mcp_request)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

    let body = test::read_body(resp).await;
    let response: serde_json::Value = serde_json::from_slice(&body).expect("Invalid JSON");

    assert!(response["error"].is_object(), "Expected error for nonexistent tool");
    assert_eq!(response["error"]["code"], -32601);
}

#[actix_rt::test]
async fn test_mcp_tools_call_missing_params() {
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

    let mcp_request = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call"
    });

    let req = test::TestRequest::post()
        .uri("/mcp")
        .set_json(&mcp_request)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

    let body = test::read_body(resp).await;
    let response: serde_json::Value = serde_json::from_slice(&body).expect("Invalid JSON");

    assert!(response["error"].is_object(), "Expected error for missing params");
    assert_eq!(response["error"]["code"], -32602);
}

#[actix_rt::test]
async fn test_http_executor_connection_failure() {
    let executor = HttpToolExecutor::new();

    let request = service_tool::executor::ExecuteRequest {
        tool_id: "test-tool".to_string(),
        parameters: serde_json::json!({}),
        url: "http://localhost:1".to_string(),
        method: "POST".to_string(),
        timeout_ms: 1000,
    };

    let result = executor.execute(request).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    let err_msg = err.to_string();
    assert!(
        err_msg.contains("Failed to connect")
            || err_msg.contains("connection")
            || err_msg.contains("timed out"),
        "Expected connection error, got: {}",
        err_msg
    );
}

#[actix_rt::test]
async fn test_http_executor_timeout() {
    let executor = HttpToolExecutor::new();

    let request = service_tool::executor::ExecuteRequest {
        tool_id: "test-tool".to_string(),
        parameters: serde_json::json!({}),
        url: "http://httpbin.org/delay/10".to_string(),
        method: "GET".to_string(),
        timeout_ms: 100,
    };

    let result = executor.execute(request).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("timed out"));
}

#[actix_rt::test]
async fn test_http_executor_invalid_method() {
    let executor = HttpToolExecutor::new();

    let request = service_tool::executor::ExecuteRequest {
        tool_id: "test-tool".to_string(),
        parameters: serde_json::json!({}),
        url: "http://localhost:8080".to_string(),
        method: "INVALID".to_string(),
        timeout_ms: 5000,
    };

    let result = executor.execute(request).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Unsupported HTTP method"));
}
