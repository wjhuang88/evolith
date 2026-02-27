//! Tool routes

use crate::handlers::tool_handlers::*;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.app_data(web::Data::new(ToolState::new()))
        .route("/tools", web::get().to(list_tools))
        .route("/tools", web::post().to(create_tool))
        .route("/tools/{id}", web::get().to(get_tool))
        .route("/tools/{id}", web::put().to(update_tool))
        .route("/tools/{id}", web::delete().to(delete_tool));
}
