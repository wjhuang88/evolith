//! Evolith Backend Server
//!
//! Main entry point for the Evolith agent development platform.

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use api::configure_routes;
use api::handlers::api_key_handlers::ApiKeyState;
use api::handlers::auth_handlers::AuthState;
use api::handlers::mcp_handlers::McpState;
use api::handlers::member_handlers::MemberState;
use api::handlers::tool_handlers::ToolStore;
use api::handlers::billing_handlers::BillingState;
use api::handlers::audit_handlers::AuditState;
use common::error::Result;
use infra::config::AppConfig;

#[actix_web::main]
async fn main() -> Result<()> {
    // Load configuration
    let config = AppConfig::from_env()?;

    // Initialize logging
    init_tracing(&config);

    info!(
        "Starting Evolith server on {}:{}",
        config.server.host, config.server.port
    );

    // Create auth state
    let auth_state = AuthState::new(&config.jwt);

    // Create tool store for MCP
    let tool_store = std::sync::Arc::new(ToolStore::default());
    // API Key state (used both for MCP and direct API key management)
    let api_key_store = ApiKeyState::new();
    // API Key state (used both for MCP and direct API key management)
    let api_key_store = ApiKeyState::new();
    let mcp_state = McpState::new(tool_store, std::sync::Arc::new(api_key_store.clone()));

    // Create member state for invitations
    let member_state = MemberState::new();

    // Audit state for logging
    let audit_state = AuditState::new();

    // Billing state for plans and subscriptions
    let billing_state = BillingState::new();

    // Create HTTP server
    let host = config.server.host.clone();
    let port = config.server.port;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(auth_state.clone()))
            .app_data(web::Data::new(mcp_state.clone()))
            .app_data(web::Data::new(member_state.clone()))
            .app_data(web::Data::new(api_key_store.clone()))
            .app_data(web::Data::new(audit_state.clone()))
            .app_data(web::Data::new(billing_state.clone()))
            .wrap(middleware::Logger::default())
            .wrap(cors_configuration())
            .configure(configure_routes)
    })
    .bind((host.as_str(), port))?
    .run()
    .await?;

    Ok(())
}

fn init_tracing(config: &AppConfig) {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&config.log.level));

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
}

fn cors_configuration() -> Cors {
    Cors::permissive()
}
