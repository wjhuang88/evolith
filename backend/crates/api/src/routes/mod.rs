//! API routes configuration

use crate::handlers::members::accept_invitation;
use actix_web::web;

pub mod api_keys;
pub mod audit;
pub mod auth;
pub mod billing;
pub mod git_smart_http;
pub mod health;
pub mod mcp;
pub mod members;
pub mod repo_context;
pub mod repos;
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
        // Smart HTTP git endpoints at /repos/ (auth-required via global rbac_middleware)
        .service(web::scope("/repos").configure(git_smart_http::configure))
        .service(
            web::scope("/api/v1")
                // Auth routes - public (login, register)
                .configure(auth::configure)
                // Public invitation acceptance route. Tenant-scoped /members/join is kept for compatibility.
                .route("/invitations/accept", web::post().to(accept_invitation))
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
                        .configure(audit::configure)
                        .configure(repos::configure)
                        .configure(repo_context::configure),
                ),
        ); // Terminate the chain
}
