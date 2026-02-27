//! API routes configuration

use actix_web::web;

pub mod health;
pub mod auth;
pub mod tools;
pub mod skills;
pub mod snippets;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Health check
        .service(
            web::scope("/health")
                .route("", web::get().to(health::health_check))
        )
        // API v1
        .service(
            web::scope("/api/v1")
                .configure(auth::configure)
                .configure(tools::configure)
                .configure(skills::configure)
                .configure(snippets::configure)
        );
}
