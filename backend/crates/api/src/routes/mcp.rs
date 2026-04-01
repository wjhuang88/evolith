//! MCP routes
//! Model Context Protocol endpoint

use crate::handlers::mcp_handlers;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::post().to(mcp_handlers::handle_mcp_request))
        .route("/tools", web::get().to(mcp_handlers::list_available_tools));
}
