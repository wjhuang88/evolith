//! API routes configuration

use actix_web::web;

pub mod api_keys;
pub mod audit;
pub mod auth;
pub mod billing;
pub mod health;
pub mod mcp;
pub mod members;
pub mod skills;
pub mod snippets;
pub mod tools;

/// Data structure to hold JWT secret for RBAC middleware
pub struct JwtSecret(pub String);

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Health check - public (no auth required)
        .configure(health::configure)
        // MCP endpoint - requires API key auth, handled separately
        .service(web::scope("/mcp").configure(mcp::configure))
        .service(
            web::scope("/api/v1")
                // Auth routes - public (login, register)
                .configure(auth::configure)
                // Protected routes - require JWT auth
                .configure(tools::configure)
                .configure(skills::configure)
                .configure(snippets::configure)
                // Tenant-scoped routes - require JWT auth
                .service(
                    web::scope("/tenant/{tenant_id}")
                        .configure(members::configure)
                        .configure(api_keys::configure)
                        .configure(billing::configure)
                        .configure(audit::configure),
                ),
        ); // Terminate the chain
}
