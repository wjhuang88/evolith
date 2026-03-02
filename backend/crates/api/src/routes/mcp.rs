//! MCP routes
//! Model Context Protocol endpoint

use crate::handlers::mcp_handlers::mcp_endpoint;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::post().to(mcp_endpoint));
}
