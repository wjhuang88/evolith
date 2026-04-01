//! Snippet routes

use actix_web::web;

use crate::handlers::snippet_handlers::{
    create_snippet, delete_snippet, get_reference, get_snippet, list_snippets, search_snippets,
    update_snippet,
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/snippets", web::get().to(list_snippets))
        .route("/snippets", web::post().to(create_snippet))
        .route("/snippets/search", web::get().to(search_snippets))
        .route("/snippets/{id}", web::get().to(get_snippet))
        .route("/snippets/{id}", web::put().to(update_snippet))
        .route("/snippets/{id}", web::delete().to(delete_snippet))
        .route("/snippets/{id}/reference", web::get().to(get_reference));
}
