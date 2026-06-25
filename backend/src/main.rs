//! Evolith Backend Server

use std::sync::Arc;

use actix_cors::Cors;
use actix_governor::Governor;
use actix_web::{middleware, web, App, HttpServer};
use tracing::{error, info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use api::configure_routes;

mod frontend;

use api::middleware::csrf::CsrfMiddleware;
use api::middleware::rate_limit::create_unauthenticated_limiter;
use api::middleware::rbac::RbacMiddleware;
use api::middleware::request_id::RequestIdMiddleware;
use api::middleware::security_headers::SecurityHeadersMiddleware;
use api::state::AppState;
use common::error::Result;
use common::execution::{CompositeProvider, ExecutionProvider};
use infra::cache::create_cache;
use infra::config::AppConfig;
use infra::db::pool::{create_pool, DatabasePool};
use infra::db::{
    PgApiKeyRepository, PgAuditRepository, PgGitRepoRepository, PgInvitationRepository,
    PgSkillRepository, PgSnippetRepository, PgTenantRepository, PgToolRepository, PgUserRepository,
    SqliteApiKeyRepository, SqliteAuditRepository, SqliteGitRepoRepository,
    SqliteInvitationRepository, SqliteSkillRepository, SqliteSnippetRepository,
    SqliteTenantRepository, SqliteToolRepository, SqliteUserRepository,
};
use infra::mailer::create_mailer;
use service_auth::{Argon2Hasher, JwtHandler};
use service_skill::executor::{DefaultSkillExecutor, SkillExecutor, SkillExecutorAdapter};
use service_skill::{DockerSandboxProvider, PoolConfig, SandboxConfig};
use service_tool::executor::{ToolExecutor, ToolExecutorAdapter};
use service_tool::HttpProxyProvider;

#[actix_web::main]
async fn main() -> Result<()> {
    let config = AppConfig::from_env()?;

    init_tracing(&config);

    info!(
        "Starting Evolith server on {}:{}",
        config.server.host, config.server.port
    );
    info!("Database type: {}", config.database.database_type);
    info!(
        "Environment: {}",
        if config.is_production() {
            "production"
        } else {
            "development"
        }
    );

    let db_pool = create_pool(&config.database).await?;

    // Initialize cache (Redis if available, falls back to in-memory)
    let cache: Arc<dyn infra::cache::Cache> = Arc::from(create_cache(&config.redis).await);

    // Initialize mailer (SMTP if enabled, falls back to console)
    let mailer: Arc<dyn infra::mailer::Mailer> = Arc::from(create_mailer(&config.smtp));

    let docker_sandbox: Option<Arc<dyn ExecutionProvider>> = if config.sandbox.enabled {
        let sandbox_config = SandboxConfig::from_infra(&config.sandbox);
        let pool_config = PoolConfig::from_env();
        match DockerSandboxProvider::new(sandbox_config, pool_config) {
            Ok(provider) => {
                info!("Sandbox enabled, initializing Docker sandbox pool");
                let arc = Arc::new(provider);
                if let Err(e) = arc.prewarm().await {
                    error!("Failed to prewarm Docker sandbox pool: {}", e);
                    return Err(common::error::AppError::ConfigError(format!(
                        "Sandbox is enabled but Docker pool could not be prewarmed: {}. \
Set SANDBOX__ENABLED=false to disable skill execution explicitly.",
                        e
                    )));
                }
                Some(arc)
            }
            Err(e) => {
                error!(
                    "Failed to initialize Docker executor while sandbox is enabled: {}",
                    e
                );
                return Err(common::error::AppError::ConfigError(format!(
                    "Sandbox is enabled but Docker executor could not be initialized: {}. \
Set SANDBOX__ENABLED=false to disable skill execution explicitly.",
                    e
                )));
            }
        }
    } else {
        info!("Sandbox disabled, Docker sandbox provider unavailable");
        None
    };

    let http_proxy: Arc<dyn ExecutionProvider> = Arc::new(HttpProxyProvider::new());
    info!("HTTP proxy provider initialized");

    let execution_provider: Arc<dyn ExecutionProvider> = Arc::new(CompositeProvider::new(
        docker_sandbox.clone(),
        Some(http_proxy.clone()),
    ));

    let skill_executor: Arc<dyn SkillExecutor> = if docker_sandbox.is_some() {
        Arc::new(SkillExecutorAdapter::new(execution_provider.clone()))
    } else {
        info!("Sandbox disabled, using default skill executor");
        Arc::new(DefaultSkillExecutor::new())
    };

    let tool_executor: Arc<dyn ToolExecutor> =
        Arc::new(ToolExecutorAdapter::new(execution_provider.clone()));
    info!("Tool executor adapter initialized");

    let app_state = match db_pool {
        DatabasePool::Sqlite(pool) => {
            info!("Running SQLite migrations...");
            sqlx::migrate!("./migrations/sqlite")
                .run(&pool)
                .await
                .map_err(|e| {
                    common::error::AppError::DatabaseError(format!(
                        "Failed to run migrations: {}",
                        e
                    ))
                })?;
            info!("Migrations completed successfully");

            AppState {
                config: config.clone(),
                jwt: JwtHandler::new(&config.jwt),
                hasher: Argon2Hasher::new(),
                user_repo: Arc::new(SqliteUserRepository::new(pool.clone())),
                tenant_repo: Arc::new(SqliteTenantRepository::new(pool.clone())),
                tool_repo: Arc::new(SqliteToolRepository::new(pool.clone())),
                skill_repo: Arc::new(SqliteSkillRepository::new(pool.clone())),
                snippet_repo: Arc::new(SqliteSnippetRepository::new(pool.clone())),
                audit_repo: Arc::new(SqliteAuditRepository::new(pool.clone())),
                invitation_repo: Arc::new(SqliteInvitationRepository::new(pool.clone())),
                api_key_repo: Arc::new(SqliteApiKeyRepository::new(pool.clone())),
                git_repo_repo: Arc::new(SqliteGitRepoRepository::new(pool)),
                cache: cache.clone(),
                mailer: mailer.clone(),
                execution_provider: execution_provider.clone(),
                skill_executor: skill_executor.clone(),
                tool_executor: tool_executor.clone(),
                git_storage_base_path: config.git_storage.base_path.clone(),
            }
        }
        DatabasePool::Postgres(pool) => {
            info!("Running PostgreSQL migrations...");
            sqlx::migrate!("./migrations/postgres")
                .run(&pool)
                .await
                .map_err(|e| {
                    common::error::AppError::DatabaseError(format!(
                        "Failed to run migrations: {}",
                        e
                    ))
                })?;
            info!("Migrations completed successfully");

            AppState {
                config: config.clone(),
                jwt: JwtHandler::new(&config.jwt),
                hasher: Argon2Hasher::new(),
                user_repo: Arc::new(PgUserRepository::new(pool.clone())),
                tenant_repo: Arc::new(PgTenantRepository::new(pool.clone())),
                tool_repo: Arc::new(PgToolRepository::new(pool.clone())),
                skill_repo: Arc::new(PgSkillRepository::new(pool.clone())),
                snippet_repo: Arc::new(PgSnippetRepository::new(pool.clone())),
                audit_repo: Arc::new(PgAuditRepository::new(pool.clone())),
                invitation_repo: Arc::new(PgInvitationRepository::new(pool.clone())),
                api_key_repo: Arc::new(PgApiKeyRepository::new(pool.clone())),
                git_repo_repo: Arc::new(PgGitRepoRepository::new(pool)),
                cache: cache.clone(),
                mailer: mailer.clone(),
                execution_provider: execution_provider.clone(),
                skill_executor: skill_executor.clone(),
                tool_executor: tool_executor.clone(),
                git_storage_base_path: config.git_storage.base_path.clone(),
            }
        }
    };

    let app_state = web::Data::new(app_state);

    let host = config.server.host.clone();
    let port = config.server.port;
    let is_dev = config.is_development();
    let jwt_secret = config.jwt.secret.clone();
    let rate_limit_config = config.rate_limit.clone();

    HttpServer::new(move || {
        let governor_config = create_unauthenticated_limiter(&rate_limit_config);

        App::new()
            .app_data(app_state.clone())
            .wrap(CsrfMiddleware::new())
            .wrap(RbacMiddleware::new(jwt_secret.clone()))
            .wrap(Governor::new(&governor_config))
            .wrap(RequestIdMiddleware::new())
            .wrap(middleware::Logger::default())
            .wrap(cors_configuration(is_dev))
            .wrap(SecurityHeadersMiddleware::new(is_dev))
            .configure(configure_routes)
            .route("/config.js", web::get().to(frontend::config_js))
            .route("/favicon.ico", web::get().to(frontend::favicon_ico))
            .default_service(web::route().to(frontend::spa_fallback))
    })
    .bind((host.as_str(), port))?
    .shutdown_timeout(30)
    .run()
    .await?;

    info!("Server shutdown complete");
    Ok(())
}

fn init_tracing(config: &AppConfig) {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&config.log.level));

    if config.is_production() {
        tracing_subscriber::registry()
            .with(filter)
            .with(tracing_subscriber::fmt::layer().json())
            .init();
    } else {
        tracing_subscriber::registry()
            .with(filter)
            .with(tracing_subscriber::fmt::layer())
            .init();
    }
}

fn cors_configuration(is_development: bool) -> Cors {
    if is_development {
        warn!("CORS is set to permissive mode with credentials (development)");
        // Note: Cannot use allow_any_origin() with supports_credentials() per CORS spec.
        // Instead, we send the request's Origin back as the allowed origin.
        Cors::default()
            .allowed_origin_fn(|_origin, _req_head| true)
            .allow_any_method()
            .allow_any_header()
            .supports_credentials()
    } else {
        Cors::default()
            .allowed_origin("https://evolith.io")
            .allowed_origin("https://app.evolith.io")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::HeaderName::from_static("x-request-id"),
                actix_web::http::header::HeaderName::from_static("x-csrf-token"),
            ])
            .supports_credentials()
            .max_age(3600)
    }
}
