//! Tool routes

use actix_web::web;

use crate::handlers::tool_handlers::{create_tool, delete_tool, get_tool, list_tools, update_tool};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/tools", web::get().to(list_tools))
        .route("/tools", web::post().to(create_tool))
        .route("/tools/{id}", web::get().to(get_tool))
        .route("/tools/{id}", web::put().to(update_tool))
        .route("/tools/{id}", web::delete().to(delete_tool));
}
