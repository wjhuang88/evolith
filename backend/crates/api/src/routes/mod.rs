//! API routes configuration

use actix_web::web;

pub mod api_keys;
pub mod auth;
pub mod billing;
pub mod health;
pub mod members;
pub mod mcp;
pub mod skills;
pub mod snippets;
pub mod tools;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Health check
        .service(web::scope("/health").route("", web::get().to(health::health_check)))
        // MCP endpoint (Model Context Protocol) - 统一入口
        .service(web::scope("/mcp").configure(mcp::configure))
        .service(
            web::scope("/api/v1")
                .configure(auth::configure)
                .configure(tools::configure)
                .configure(skills::configure)
                .configure(snippets::configure)
                // Tenant-scoped routes
                .service(web::scope("/tenant/{tenant_id}")
                    .configure(members::configure)
                    .configure(api_keys::configure)
                    .configure(billing::configure)
                ),
        );
}