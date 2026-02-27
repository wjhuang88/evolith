//! Evolith Backend Server
//!
//! Main entry point for the Evolith agent development platform.

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use api::configure_routes;
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

    // Create HTTP server
    let host = config.server.host.clone();
    let port = config.server.port;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
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
