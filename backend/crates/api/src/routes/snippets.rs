//! Snippet routes

use crate::handlers::snippet_handlers::*;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.app_data(web::Data::new(SnippetState::new()))
        .route("/snippets", web::get().to(list_snippets))
        .route("/snippets", web::post().to(create_snippet))
        .route("/snippets/search", web::get().to(search_snippets))
        .route("/snippets/{id}", web::get().to(get_snippet))
        .route("/snippets/{id}", web::put().to(update_snippet))
        .route("/snippets/{id}", web::delete().to(delete_snippet))
        .route("/snippets/{id}/reference", web::get().to(get_reference));
}
