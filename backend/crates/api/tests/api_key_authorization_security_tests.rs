#![allow(clippy::unwrap_used, dead_code)]

use actix_web::{dev::ServerHandle, middleware::from_fn, test, web, App, HttpResponse, HttpServer};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::net::TcpListener;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tempfile::TempDir;
use uuid::Uuid;

use api::routes;
use api::state::AppState;
use common::execution::{CompositeProvider, ExecutionProvider};
use domain::api_key::NewApiKey;
use domain::repository::{
    ApiKeyRepository, AuditRepository, GitRepoRepository, InvitationRepository, SkillRepository,
    SnippetRepository, TenantRepository, ToolRepository, UserRepository,
};
use domain::{HandlerConfig, HandlerType, NewTool, Visibility};
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
struct CreatedApiKey {
    id: String,
    permissions: Vec<String>,
}

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
            max_connections: 1,
            seed_database: false,
        },
        redis: RedisConfig {
            url: "redis://localhost:6379".to_string(),
        },
        jwt: JwtConfig {
            secret: "test_secret_key_for_authz_security_32chars!!".to_string(),
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
            api_key_rpm: 1000,
        },
        git_storage: GitStorageConfig { base_path },
    }
}

fn build_app_state(pool: SqlitePool, base_path: String) -> AppState {
    let config = create_test_config(base_path);
    let http_proxy: Arc<dyn ExecutionProvider> = Arc::new(HttpProxyProvider::new());
    let execution_provider: Arc<dyn ExecutionProvider> =
        Arc::new(CompositeProvider::new(None, Some(http_proxy)));

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

async fn seed_tenant_users(
    pool: &SqlitePool,
    tenant_id: Uuid,
    owner_id: Uuid,
    member_id: Uuid,
) {
    sqlx::query("INSERT INTO tenants (id, name, slug, owner_id) VALUES (?, ?, ?, ?)")
        .bind(tenant_id.to_string())
        .bind("Authorization Test Tenant")
        .bind(format!("authz-{tenant_id}"))
        .bind(owner_id.to_string())
        .execute(pool)
        .await
        .expect("seed tenant");

    for (user_id, username, email, tenant_role) in [
        (owner_id, "authz-owner", "owner@authz.test", "owner"),
        (member_id, "authz-member", "member@authz.test", "member"),
    ] {
        sqlx::query(
            r#"
            INSERT INTO users (
                id, username, email, password_hash, role, tenant_id, tenant_role
            ) VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(user_id.to_string())
        .bind(username)
        .bind(email)
        .bind("not-used-in-tests")
        .bind("user")
        .bind(tenant_id.to_string())
        .bind(tenant_role)
        .execute(pool)
        .await
        .expect("seed user");
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
    key: &str,
    permissions: Vec<&str>,
) {
    SqliteApiKeyRepository::new(pool.clone())
        .create(NewApiKey {
            tenant_id,
            user_id,
            name: format!("test-{key}"),
            key_hash: hash_key(key),
            key_prefix: "evo_sk_test...".to_string(),
            permissions: permissions.into_iter().map(str::to_string).collect(),
            rate_limit: None,
            expires_at: None,
        })
        .await
        .expect("seed API key");
}

async fn seed_http_tool(pool: &SqlitePool, tenant_id: Uuid, owner_id: Uuid, url: String) {
    SqliteToolRepository::new(pool.clone())
        .create(
            NewTool {
                name: "authorization-echo".to_string(),
                description: "authorization test tool".to_string(),
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
        .expect("seed HTTP tool");
}

fn start_counting_server() -> (String, Arc<AtomicUsize>, ServerHandle) {
    let hits = Arc::new(AtomicUsize::new(0));
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind local test server");
    let address = listener.local_addr().expect("read local address");
    let app_hits = hits.clone();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_hits.clone()))
            .route(
                "/echo",
                web::post().to(
                    |hits: web::Data<Arc<AtomicUsize>>,
                     body: web::Json<serde_json::Value>| async move {
                        hits.fetch_add(1, Ordering::SeqCst);
                        HttpResponse::Ok().json(body.into_inner())
                    },
                ),
            )
    })
    .listen(listener)
    .expect("listen on local test server")
    .run();
    let handle = server.handle();
    actix_rt::spawn(server);

    (format!("http://{address}"), hits, handle)
}

#[actix_rt::test]
async fn member_cannot_manage_api_keys_and_denials_are_audited() {
    let pool = setup_test_db().await;
    let temp_dir = TempDir::new().expect("temp dir");
    let tenant_id = Uuid::new_v4();
    let owner_id = Uuid::new_v4();
    let member_id = Uuid::new_v4();
    seed_tenant_users(&pool, tenant_id, owner_id, member_id).await;

    let state = build_app_state(pool.clone(), temp_dir.path().display().to_string());
    let audit_repo = state.audit_repo.clone();
    let (member_token, _) = state
        .jwt
        .generate_token(member_id, "user", tenant_id, "member")
        .expect("member token");
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .configure(routes::configure_routes),
    )
    .await;

    let list_request = test::TestRequest::get()
        .uri(&format!("/api/v1/tenant/{tenant_id}/api-keys"))
        .insert_header(("Authorization", format!("Bearer {member_token}")))
        .to_request();
    let list_response = test::call_service(&app, list_request).await;
    assert_eq!(list_response.status(), actix_web::http::StatusCode::FORBIDDEN);

    let create_request = test::TestRequest::post()
        .uri(&format!("/api/v1/tenant/{tenant_id}/api-keys"))
        .insert_header(("Authorization", format!("Bearer {member_token}")))
        .set_json(serde_json::json!({
            "name": "member-key",
            "permissions": ["read"]
        }))
        .to_request();
    let create_response = test::call_service(&app, create_request).await;
    assert_eq!(create_response.status(), actix_web::http::StatusCode::FORBIDDEN);

    let revoke_request = test::TestRequest::delete()
        .uri(&format!(
            "/api/v1/tenant/{tenant_id}/api-keys/{}",
            Uuid::new_v4()
        ))
        .insert_header(("Authorization", format!("Bearer {member_token}")))
        .set_json(serde_json::json!({"reason": "not allowed"}))
        .to_request();
    let revoke_response = test::call_service(&app, revoke_request).await;
    assert_eq!(revoke_response.status(), actix_web::http::StatusCode::FORBIDDEN);

    let audit_logs = audit_repo
        .find_by_action("api_key.management.denied", 10, 0)
        .await
        .expect("read denial audits");
    assert_eq!(audit_logs.len(), 3);
    assert!(audit_logs
        .iter()
        .all(|log| log.user_id == Some(member_id) && log.tenant_id == Some(tenant_id)));
}

#[actix_rt::test]
async fn owner_and_admin_issue_only_canonical_capabilities() {
    let pool = setup_test_db().await;
    let temp_dir = TempDir::new().expect("temp dir");
    let tenant_id = Uuid::new_v4();
    let owner_id = Uuid::new_v4();
    let admin_id = Uuid::new_v4();
    seed_tenant_users(&pool, tenant_id, owner_id, admin_id).await;

    let state = build_app_state(pool.clone(), temp_dir.path().display().to_string());
    let api_key_repo = state.api_key_repo.clone();
    let (owner_token, _) = state
        .jwt
        .generate_token(owner_id, "user", tenant_id, "owner")
        .expect("owner token");
    let (admin_token, _) = state
        .jwt
        .generate_token(admin_id, "user", tenant_id, "admin")
        .expect("admin token");
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .configure(routes::configure_routes),
    )
    .await;

    for (name, token, permissions) in [
        (
            "owner-canonical",
            owner_token.as_str(),
            vec!["repo:read", "execute"],
        ),
        ("admin-canonical", admin_token.as_str(), vec!["repo:write"]),
    ] {
        let request = test::TestRequest::post()
            .uri(&format!("/api/v1/tenant/{tenant_id}/api-keys"))
            .insert_header(("Authorization", format!("Bearer {token}")))
            .set_json(serde_json::json!({"name": name, "permissions": permissions}))
            .to_request();
        let response = test::call_service(&app, request).await;
        assert_eq!(response.status(), actix_web::http::StatusCode::OK);
        let body = test::read_body(response).await;
        let result: ApiResponse<CreatedApiKey> =
            serde_json::from_slice(&body).expect("parse create response");
        assert!(result.success);
        assert!(result.error.is_none());
        assert!(!result.data.expect("created key").permissions.is_empty());
    }

    for permission in ["admin", "manage_keys", "write", "commit:main", "unknown"] {
        let request = test::TestRequest::post()
            .uri(&format!("/api/v1/tenant/{tenant_id}/api-keys"))
            .insert_header(("Authorization", format!("Bearer {owner_token}")))
            .set_json(serde_json::json!({
                "name": format!("rejected-{permission}"),
                "permissions": [permission]
            }))
            .to_request();
        let response = test::call_service(&app, request).await;
        assert_eq!(response.status(), actix_web::http::StatusCode::BAD_REQUEST);
    }

    let empty_request = test::TestRequest::post()
        .uri(&format!("/api/v1/tenant/{tenant_id}/api-keys"))
        .insert_header(("Authorization", format!("Bearer {owner_token}")))
        .set_json(serde_json::json!({"name": "empty", "permissions": []}))
        .to_request();
    let empty_response = test::call_service(&app, empty_request).await;
    assert_eq!(empty_response.status(), actix_web::http::StatusCode::BAD_REQUEST);

    let stored_keys = api_key_repo
        .find_by_tenant(tenant_id)
        .await
        .expect("list stored keys");
    assert_eq!(stored_keys.len(), 2, "rejected payloads must not create keys");
}

#[actix_rt::test]
async fn mcp_tools_call_requires_execute_and_does_not_invoke_executor_without_it() {
    let pool = setup_test_db().await;
    let temp_dir = TempDir::new().expect("temp dir");
    let tenant_id = Uuid::new_v4();
    let owner_id = Uuid::new_v4();
    let member_id = Uuid::new_v4();
    seed_tenant_users(&pool, tenant_id, owner_id, member_id).await;

    let read_key = "evo_sk_read_only_security_test";
    let execute_key = "evo_sk_execute_security_test";
    seed_api_key(&pool, tenant_id, owner_id, read_key, vec!["repo:read"]).await;
    seed_api_key(&pool, tenant_id, owner_id, execute_key, vec!["execute"]).await;

    let (base_url, hits, server_handle) = start_counting_server();
    seed_http_tool(&pool, tenant_id, owner_id, format!("{base_url}/echo")).await;

    let state = build_app_state(pool, temp_dir.path().display().to_string());
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .wrap(from_fn(api::middleware::rbac::rbac_middleware))
            .configure(routes::configure_routes),
    )
    .await;

    let request_body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": "authorization-echo",
            "arguments": {"value": "ok"}
        }
    });

    let read_request = test::TestRequest::post()
        .uri("/mcp")
        .insert_header(("X-API-Key", read_key))
        .set_json(&request_body)
        .to_request();
    let read_response: serde_json::Value = test::call_and_read_body_json(&app, read_request).await;
    assert_eq!(read_response["error"]["code"], -32003);
    assert!(read_response["error"]["message"]
        .as_str()
        .expect("authorization message")
        .contains("execute"));
    assert_eq!(hits.load(Ordering::SeqCst), 0, "executor must not be invoked");

    let execute_request = test::TestRequest::post()
        .uri("/mcp")
        .insert_header(("X-API-Key", execute_key))
        .set_json(&request_body)
        .to_request();
    let execute_response: serde_json::Value =
        test::call_and_read_body_json(&app, execute_request).await;
    assert!(execute_response["error"].is_null());
    assert_eq!(hits.load(Ordering::SeqCst), 1);

    server_handle.stop(true).await;
}