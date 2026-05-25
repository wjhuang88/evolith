//! MCP tool execution integration tests

use actix_web::{dev::ServerHandle, test, web, App, HttpResponse, HttpServer};
use domain::api_key::NewApiKey;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::{net::TcpListener, sync::Arc, time::Duration};

use api::middleware::rbac::RbacMiddleware;
use api::routes;
use api::state::AppState;
use domain::repository::{
    ApiKeyRepository, AuditRepository, InvitationRepository, SkillRepository, SnippetRepository,
    TenantRepository, ToolRepository, UserRepository,
};
use domain::{HandlerConfig, HandlerType, NewTool, Visibility};
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
use sha2::{Digest, Sha256};
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

const TEST_API_KEY: &str = "evo_sk_11111111111111111111111111111111";
const TEST_TENANT_ID: &str = "11111111-1111-1111-1111-111111111111";
const TEST_USER_ID: &str = "22222222-2222-2222-2222-222222222222";

fn test_tenant_id() -> Uuid {
    Uuid::parse_str(TEST_TENANT_ID).expect("valid tenant test uuid")
}

fn test_user_id() -> Uuid {
    Uuid::parse_str(TEST_USER_ID).expect("valid user test uuid")
}

async fn seed_tenant_identity(pool: &SqlitePool) {
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO tenants (id, name, slug, owner_id)
        VALUES (?, ?, ?, ?)
        "#,
    )
    .bind(TEST_TENANT_ID)
    .bind("MCP Test Tenant")
    .bind("mcp-test-tenant")
    .bind(TEST_USER_ID)
    .execute(pool)
    .await
    .expect("seed test tenant");

    sqlx::query(
        r#"
        INSERT OR IGNORE INTO users (
            id, username, email, password_hash, role, tenant_id, tenant_role
        ) VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(TEST_USER_ID)
    .bind("mcp-test-user")
    .bind("mcp-test@example.com")
    .bind("not-used-in-mcp-tests")
    .bind("user")
    .bind(TEST_TENANT_ID)
    .bind("owner")
    .execute(pool)
    .await
    .expect("seed test user");
}

async fn seed_api_key(pool: &SqlitePool) {
    seed_tenant_identity(pool).await;
    let key_hash = {
        let mut hasher = Sha256::new();
        hasher.update(TEST_API_KEY.as_bytes());
        hex::encode(hasher.finalize())
    };

    SqliteApiKeyRepository::new(pool.clone())
        .create(NewApiKey {
            tenant_id: test_tenant_id(),
            user_id: test_user_id(),
            name: "mcp-test-key".to_string(),
            key_hash,
            key_prefix: "evo_sk_11111111...".to_string(),
            permissions: vec!["execute".to_string()],
            rate_limit: None,
            expires_at: None,
        })
        .await
        .expect("seed API key");
}

async fn seed_http_tool(pool: &SqlitePool, name: &str, url: String, visibility: Visibility) {
    seed_tenant_identity(pool).await;
    SqliteToolRepository::new(pool.clone())
        .create(
            NewTool {
                name: name.to_string(),
                description: "test HTTP tool".to_string(),
                input_schema: serde_json::json!({"type": "object"}),
                output_schema: None,
                handler: HandlerConfig {
                    handler_type: HandlerType::Http,
                    url: Some(url),
                    method: Some("POST".to_string()),
                    timeout: Some(1_000),
                },
                visibility: Some(visibility),
            },
            test_user_id(),
            test_tenant_id(),
        )
        .await
        .expect("seed HTTP tool");
}

fn start_test_server() -> (String, ServerHandle) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind mock HTTP server");
    let address = listener.local_addr().expect("read mock server address");
    let server = HttpServer::new(|| {
        App::new()
            .route(
                "/echo",
                web::post().to(|body: web::Json<serde_json::Value>| async move {
                    HttpResponse::Ok().json(body.into_inner())
                }),
            )
            .route(
                "/bad-gateway",
                web::post().to(|| async { HttpResponse::BadGateway().body("upstream failed") }),
            )
            .route(
                "/slow",
                web::post().to(|| async {
                    tokio::time::sleep(Duration::from_millis(200)).await;
                    HttpResponse::Ok().body("late")
                }),
            )
            .route(
                "/large",
                web::post().to(|| async { HttpResponse::Ok().body("x".repeat(1_048_577)) }),
            )
    })
    .listen(listener)
    .expect("listen on mock server")
    .run();
    let handle = server.handle();
    actix_rt::spawn(server);
    (format!("http://{}", address), handle)
}

#[actix_rt::test]
async fn test_mcp_tools_call_nonexistent_tool() {
    let pool = setup_test_db().await;
    seed_api_key(&pool).await;
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
        .insert_header(("X-API-Key", TEST_API_KEY))
        .set_json(&mcp_request)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

    let body = test::read_body(resp).await;
    let response: serde_json::Value = serde_json::from_slice(&body).expect("Invalid JSON");

    assert!(
        response["error"].is_object(),
        "Expected error for nonexistent tool"
    );
    assert_eq!(response["error"]["code"], -32601);
}

#[actix_rt::test]
async fn test_mcp_tools_call_missing_params() {
    let pool = setup_test_db().await;
    seed_api_key(&pool).await;
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
        .insert_header(("X-API-Key", TEST_API_KEY))
        .set_json(&mcp_request)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

    let body = test::read_body(resp).await;
    let response: serde_json::Value = serde_json::from_slice(&body).expect("Invalid JSON");

    assert!(
        response["error"].is_object(),
        "Expected error for missing params"
    );
    assert_eq!(response["error"]["code"], -32602);
}

#[actix_rt::test]
async fn test_mcp_tools_call_requires_api_key_even_for_public_tool() {
    let pool = setup_test_db().await;
    seed_http_tool(
        &pool,
        "public-tool",
        "http://127.0.0.1:1/no-call".to_string(),
        Visibility::Public,
    )
    .await;
    let app_state = build_app_state(pool);
    let jwt_secret = app_state.config.jwt.secret.clone();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(RbacMiddleware::new(jwt_secret))
            .configure(routes::configure_routes),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/mcp")
        .set_json(serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": {"name": "public-tool", "arguments": {}}
        }))
        .to_request();
    let response: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    assert!(response["error"]["message"]
        .as_str()
        .expect("error message")
        .contains("valid API key"));
}

#[actix_rt::test]
async fn test_mcp_tools_call_executes_authenticated_http_tool() {
    let (base_url, handle) = start_test_server();
    let pool = setup_test_db().await;
    seed_api_key(&pool).await;
    seed_http_tool(
        &pool,
        "echo-tool",
        format!("{}/echo", base_url),
        Visibility::Private,
    )
    .await;
    let app_state = build_app_state(pool);
    let jwt_secret = app_state.config.jwt.secret.clone();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(RbacMiddleware::new(jwt_secret))
            .configure(routes::configure_routes),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/mcp")
        .insert_header(("X-API-Key", TEST_API_KEY))
        .set_json(serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": {"name": "echo-tool", "arguments": {"value": "ok"}}
        }))
        .to_request();
    let response: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let text = response["result"]["content"][0]["text"]
        .as_str()
        .expect("tool result text");

    assert!(response["error"].is_null());
    assert!(text.contains("\"value\": \"ok\""));
    handle.stop(true).await;
}

#[actix_rt::test]
async fn test_mcp_tools_call_maps_http_failure_to_mcp_error() {
    let (base_url, handle) = start_test_server();
    let pool = setup_test_db().await;
    seed_api_key(&pool).await;
    seed_http_tool(
        &pool,
        "failure-tool",
        format!("{}/bad-gateway", base_url),
        Visibility::Private,
    )
    .await;
    let app_state = build_app_state(pool);
    let jwt_secret = app_state.config.jwt.secret.clone();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(RbacMiddleware::new(jwt_secret))
            .configure(routes::configure_routes),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/mcp")
        .insert_header(("X-API-Key", TEST_API_KEY))
        .set_json(serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": {"name": "failure-tool", "arguments": {}}
        }))
        .to_request();
    let response: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(response["error"]["code"], -32002);
    assert!(response["error"]["message"]
        .as_str()
        .expect("error message")
        .contains("HTTP 502"));
    handle.stop(true).await;
}

#[actix_rt::test]
async fn test_http_executor_times_out_against_local_server() {
    let (base_url, handle) = start_test_server();
    let executor = HttpToolExecutor::new();
    let result = executor
        .execute(service_tool::executor::ExecuteRequest {
            tool_id: "slow-tool".to_string(),
            parameters: serde_json::json!({}),
            url: format!("{}/slow", base_url),
            method: "POST".to_string(),
            timeout_ms: 10,
        })
        .await;

    assert!(result
        .expect_err("slow request should time out")
        .to_string()
        .contains("timed out"));
    handle.stop(true).await;
}

#[actix_rt::test]
async fn test_http_executor_rejects_oversized_response() {
    let (base_url, handle) = start_test_server();
    let executor = HttpToolExecutor::new();
    let result = executor
        .execute(service_tool::executor::ExecuteRequest {
            tool_id: "large-tool".to_string(),
            parameters: serde_json::json!({}),
            url: format!("{}/large", base_url),
            method: "POST".to_string(),
            timeout_ms: 1_000,
        })
        .await;

    assert!(result
        .expect_err("large response should be rejected")
        .to_string()
        .contains("exceeded"));
    handle.stop(true).await;
}
