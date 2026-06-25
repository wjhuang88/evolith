use crate::handlers::repo_handlers;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/repos")
            .route("", web::get().to(repo_handlers::list_repos))
            .route("", web::post().to(repo_handlers::create_repo))
            .route("/{repo_id}", web::get().to(repo_handlers::get_repo))
            .route("/{repo_id}", web::patch().to(repo_handlers::update_repo))
            .route("/{repo_id}", web::delete().to(repo_handlers::delete_repo)),
    );
}
