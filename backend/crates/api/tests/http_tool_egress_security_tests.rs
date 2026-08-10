#![allow(clippy::unwrap_used)]

use actix_web::{middleware::from_fn, test, web, App, HttpResponse, HttpServer};
use sha2::{Digest, Sha256};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::net::TcpListener;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use uuid::Uuid;

use api::routes;
use api::state::AppState;
use common::execution::{CompositeProvider, ExecutionProvider};
use domain::api_key::NewApiKey;
use domain::repository::{
    ApiKeyRepository, AuditRepository, GitRepoRepository, InvitationRepository, SkillRepository,
    SnippetRepository, TenantRepository, ToolRepository, UserRepository,
};
use domain::{HandlerConfig, HandlerType, NewTool, ToolFilter, Visibility};
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
use service_tool::executor::{ToolExecutor, ToolExecutorAdapter};
use service_tool::{EgressPolicy, HttpProxyProvider};

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
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("--") {
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
        .expect("create in-memory SQLite pool");

    for migration in [
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
        for statement in migration.split(';') {
            let statement = strip_leading_comments(statement.trim());
            if statement.is_empty() {
                continue;
            }
            sqlx::query(sqlx::AssertSqlSafe(statement))
                .execute(&pool)
                .await
                .unwrap_or_else(|error| panic!("migration failed: {error}\nSQL: {statement}"));
        }
    }

    pool
}

fn test_config() -> AppConfig {
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
            max_connections: 1,
            seed_database: false,
        },
        redis: RedisConfig {
            url: "redis://localhost:6379".to_string(),
        },
        jwt: JwtConfig {
            secret: "test_secret_key_for_http_egress_32chars!!".to_string(),
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
            max_output_bytes: 10_485_760,
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
            api_key_rpm: 1_000,
        },
        git_storage: GitStorageConfig {
            base_path: "/tmp/evolith-http-egress-tests".to_string(),
        },
    }
}

fn build_app_state(pool: SqlitePool) -> AppState {
    let config = test_config();
    let http_proxy: Arc<dyn ExecutionProvider> =
        Arc::new(HttpProxyProvider::with_policy(EgressPolicy::default()));
    let execution_provider: Arc<dyn ExecutionProvider> =
        Arc::new(CompositeProvider::new(None, Some(http_proxy)));
    let tool_executor: Arc<dyn ToolExecutor> =
        Arc::new(ToolExecutorAdapter::new(execution_provider.clone()));

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
        execution_provider,
        skill_executor: Arc::new(DefaultSkillExecutor::new()) as Arc<dyn SkillExecutor>,
        tool_executor,
        git_storage_base_path: config.git_storage.base_path.clone(),
    }
}

async fn seed_tenant(
    pool: &SqlitePool,
    tenant_id: Uuid,
    owner_id: Uuid,
    users: &[(Uuid, &str)],
    label: &str,
) {
    sqlx::query("INSERT INTO tenants (id, name, slug, owner_id) VALUES (?, ?, ?, ?)")
        .bind(tenant_id.to_string())
        .bind(format!("Egress Test Tenant {label}"))
        .bind(format!("egress-{label}-{tenant_id}"))
        .bind(owner_id.to_string())
        .execute(pool)
        .await
        .expect("seed tenant");

    for (user_id, role) in users {
        sqlx::query(
            r#"
            INSERT INTO users (
                id, username, email, password_hash, role, tenant_id, tenant_role
            ) VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(user_id.to_string())
        .bind(format!("{label}-{role}-{user_id}"))
        .bind(format!("{role}-{user_id}@egress.test"))
        .bind("not-used-in-tests")
        .bind("user")
        .bind(tenant_id.to_string())
        .bind(*role)
        .execute(pool)
        .await
        .expect("seed user");
    }
}

fn hash_api_key(key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    hex::encode(hasher.finalize())
}

async fn seed_api_key(
    pool: &SqlitePool,
    tenant_id: Uuid,
    user_id: Uuid,
    raw_key: &str,
    permissions: Vec<&str>,
) {
    SqliteApiKeyRepository::new(pool.clone())
        .create(NewApiKey {
            tenant_id,
            user_id,
            name: "egress-test-key".to_string(),
            key_hash: hash_api_key(raw_key),
            key_prefix: "evo_sk_egress...".to_string(),
            permissions: permissions.into_iter().map(str::to_string).collect(),
            rate_limit: None,
            expires_at: None,
        })
        .await
        .expect("seed API key");
}

async fn seed_http_tool(
    pool: &SqlitePool,
    tenant_id: Uuid,
    owner_id: Uuid,
    name: &str,
    url: String,
) -> Uuid {
    SqliteToolRepository::new(pool.clone())
        .create(
            NewTool {
                name: name.to_string(),
                description: "security test HTTP tool".to_string(),
                input_schema: serde_json::json!({"type": "object"}),
                output_schema: None,
                handler: HandlerConfig {
                    handler_type: HandlerType::Http,
                    url: Some(url),
                    method: Some("POST".to_string()),
                    timeout: Some(1_000),
                },
                visibility: Some(Visibility::Private),
            },
            owner_id,
            tenant_id,
        )
        .await
        .expect("seed HTTP Tool")
        .id
}

fn create_tool_payload(url: &str) -> serde_json::Value {
    serde_json::json!({
        "name": "http-security-tool",
        "description": "HTTP egress security test",
        "input_schema": {"type": "object"},
        "type": "http",
        "handler_url": url,
        "handler_method": "POST",
        "handler_timeout": 1000,
        "is_public": false
    })
}

fn start_counting_server() -> (String, Arc<AtomicUsize>, actix_web::dev::ServerHandle) {
    let hits = Arc::new(AtomicUsize::new(0));
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind local target");
    let address = listener.local_addr().expect("read local target address");
    let app_hits = hits.clone();
    let server = HttpServer::new(move || {
        App::new().app_data(web::Data::new(app_hits.clone())).route(
            "/hit",
            web::post().to(|hits: web::Data<Arc<AtomicUsize>>| async move {
                hits.fetch_add(1, Ordering::SeqCst);
                HttpResponse::Ok().json(serde_json::json!({"ok": true}))
            }),
        )
    })
    .listen(listener)
    .expect("listen on local target")
    .run();
    let handle = server.handle();
    actix_rt::spawn(server);
    (format!("http://{address}/hit"), hits, handle)
}

#[actix_rt::test]
async fn member_invalid_body_is_denied_before_deserialization_and_write() {
    let pool = setup_test_db().await;
    let tenant_id = Uuid::new_v4();
    let owner_id = Uuid::new_v4();
    let member_id = Uuid::new_v4();
    seed_tenant(
        &pool,
        tenant_id,
        owner_id,
        &[(owner_id, "owner"), (member_id, "member")],
        "member",
    )
    .await;

    let state = build_app_state(pool);
    let tool_repo = state.tool_repo.clone();
    let audit_repo = state.audit_repo.clone();
    let (member_token, _) = state
        .jwt
        .generate_token(member_id, "user", tenant_id, "member")
        .expect("member JWT");
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .configure(routes::configure_routes),
    )
    .await;

    let request = test::TestRequest::post()
        .uri("/api/v1/tools")
        .insert_header(("Authorization", format!("Bearer {member_token}")))
        .insert_header(("Content-Type", "application/json"))
        .set_payload("{not-json")
        .to_request();
    let response = test::call_service(&app, request).await;

    assert_eq!(response.status(), actix_web::http::StatusCode::FORBIDDEN);
    assert_eq!(
        tool_repo
            .count(&ToolFilter {
                tenant_id: Some(tenant_id),
                ..Default::default()
            })
            .await
            .expect("count tools"),
        0
    );
    let audits = audit_repo
        .find_by_tenant(tenant_id, 20, 0)
        .await
        .expect("read audits");
    assert!(audits.iter().any(|audit| {
        audit.action == "tool.management.denied"
            && audit.details["reason"] == "insufficient_tenant_role"
    }));
}

#[actix_rt::test]
async fn api_key_cannot_create_tool_or_trigger_deserialization() {
    const RAW_KEY: &str = "evo_sk_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

    let pool = setup_test_db().await;
    let tenant_id = Uuid::new_v4();
    let owner_id = Uuid::new_v4();
    seed_tenant(
        &pool,
        tenant_id,
        owner_id,
        &[(owner_id, "owner")],
        "api-key",
    )
    .await;
    seed_api_key(&pool, tenant_id, owner_id, RAW_KEY, vec!["execute"]).await;

    let state = build_app_state(pool);
    let tool_repo = state.tool_repo.clone();
    let audit_repo = state.audit_repo.clone();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .configure(routes::configure_routes),
    )
    .await;

    let request = test::TestRequest::post()
        .uri("/api/v1/tools")
        .insert_header(("X-API-Key", RAW_KEY))
        .insert_header(("Content-Type", "application/json"))
        .set_payload("{not-json")
        .to_request();
    let response = test::call_service(&app, request).await;

    assert_eq!(response.status(), actix_web::http::StatusCode::FORBIDDEN);
    assert_eq!(
        tool_repo
            .count(&ToolFilter {
                tenant_id: Some(tenant_id),
                ..Default::default()
            })
            .await
            .expect("count tools"),
        0
    );
    let audits = audit_repo
        .find_by_tenant(tenant_id, 20, 0)
        .await
        .expect("read audits");
    assert!(audits.iter().any(|audit| {
        audit.action == "tool.management.denied"
            && audit.details["reason"] == "api_key_authentication"
    }));
}

#[actix_rt::test]
async fn owner_cannot_persist_loopback_http_tool() {
    let pool = setup_test_db().await;
    let tenant_id = Uuid::new_v4();
    let owner_id = Uuid::new_v4();
    seed_tenant(
        &pool,
        tenant_id,
        owner_id,
        &[(owner_id, "owner")],
        "owner-loopback",
    )
    .await;

    let state = build_app_state(pool);
    let tool_repo = state.tool_repo.clone();
    let (owner_token, _) = state
        .jwt
        .generate_token(owner_id, "user", tenant_id, "owner")
        .expect("owner JWT");
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .configure(routes::configure_routes),
    )
    .await;

    let request = test::TestRequest::post()
        .uri("/api/v1/tools")
        .insert_header(("Authorization", format!("Bearer {owner_token}")))
        .set_json(create_tool_payload("http://127.0.0.1:8080/internal"))
        .to_request();
    let response = test::call_service(&app, request).await;

    assert_eq!(response.status(), actix_web::http::StatusCode::BAD_REQUEST);
    assert_eq!(
        tool_repo
            .count(&ToolFilter {
                tenant_id: Some(tenant_id),
                ..Default::default()
            })
            .await
            .expect("count tools"),
        0
    );
}

#[actix_rt::test]
async fn foreign_and_missing_tool_updates_have_identical_responses() {
    let pool = setup_test_db().await;
    let caller_tenant_id = Uuid::new_v4();
    let caller_owner_id = Uuid::new_v4();
    let foreign_tenant_id = Uuid::new_v4();
    let foreign_owner_id = Uuid::new_v4();
    seed_tenant(
        &pool,
        caller_tenant_id,
        caller_owner_id,
        &[(caller_owner_id, "owner")],
        "caller",
    )
    .await;
    seed_tenant(
        &pool,
        foreign_tenant_id,
        foreign_owner_id,
        &[(foreign_owner_id, "owner")],
        "foreign",
    )
    .await;
    let foreign_tool_id = seed_http_tool(
        &pool,
        foreign_tenant_id,
        foreign_owner_id,
        "foreign-tool",
        "https://example.com/tool".to_string(),
    )
    .await;

    let state = build_app_state(pool);
    let tool_repo = state.tool_repo.clone();
    let (caller_token, _) = state
        .jwt
        .generate_token(caller_owner_id, "user", caller_tenant_id, "owner")
        .expect("caller JWT");
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .configure(routes::configure_routes),
    )
    .await;

    let make_request = |tool_id: Uuid| {
        test::TestRequest::put()
            .uri(&format!("/api/v1/tools/{tool_id}"))
            .insert_header(("Authorization", format!("Bearer {caller_token}")))
            .set_json(serde_json::json!({"name": "changed"}))
            .to_request()
    };

    let foreign_response = test::call_service(&app, make_request(foreign_tool_id)).await;
    let foreign_status = foreign_response.status();
    let foreign_body = test::read_body(foreign_response).await;
    let missing_response = test::call_service(&app, make_request(Uuid::new_v4())).await;
    let missing_status = missing_response.status();
    let missing_body = test::read_body(missing_response).await;

    assert_eq!(foreign_status, actix_web::http::StatusCode::NOT_FOUND);
    assert_eq!(foreign_status, missing_status);
    assert_eq!(foreign_body, missing_body);
    let foreign_tool = tool_repo
        .find_by_id(foreign_tool_id)
        .await
        .expect("read foreign tool")
        .expect("foreign tool remains");
    assert_eq!(foreign_tool.name, "foreign-tool");
}

#[actix_rt::test]
async fn mcp_loopback_rejection_has_zero_network_hits_and_redacted_audit() {
    const RAW_KEY: &str = "evo_sk_bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

    let (target_url, hits, target_handle) = start_counting_server();
    let pool = setup_test_db().await;
    let tenant_id = Uuid::new_v4();
    let owner_id = Uuid::new_v4();
    seed_tenant(
        &pool,
        tenant_id,
        owner_id,
        &[(owner_id, "owner")],
        "mcp-loopback",
    )
    .await;
    seed_api_key(&pool, tenant_id, owner_id, RAW_KEY, vec!["execute"]).await;
    seed_http_tool(
        &pool,
        tenant_id,
        owner_id,
        "loopback-probe",
        target_url.clone(),
    )
    .await;

    let state = build_app_state(pool);
    let audit_repo = state.audit_repo.clone();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .configure(routes::configure_routes),
    )
    .await;

    let request = test::TestRequest::post()
        .uri("/mcp")
        .insert_header(("X-API-Key", RAW_KEY))
        .set_json(serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": {"name": "loopback-probe", "arguments": {}}
        }))
        .to_request();
    let response: serde_json::Value = test::call_and_read_body_json(&app, request).await;
    let serialized = serde_json::to_string(&response).expect("serialize response");

    assert_eq!(
        response["error"]["message"],
        "HTTP tool request was rejected"
    );
    assert!(!serialized.contains(&target_url));
    assert!(!serialized.contains("127.0.0.1"));
    assert_eq!(hits.load(Ordering::SeqCst), 0);

    let audits = audit_repo
        .find_by_tenant(tenant_id, 20, 0)
        .await
        .expect("read audits");
    let denial = audits
        .iter()
        .find(|audit| audit.action == "tool.egress.denied_or_failed")
        .expect("egress denial audit");
    let audit_json = serde_json::to_string(denial).expect("serialize audit");
    assert_eq!(denial.tenant_id, Some(tenant_id));
    assert_eq!(denial.details["reason"], "policy_or_network_failure");
    assert!(!audit_json.contains(&target_url));
    assert!(!audit_json.contains("127.0.0.1"));

    target_handle.stop(true).await;
}
