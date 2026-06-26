use actix_web::web;

use crate::handlers::git_smart_http_handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route(
                "/{repo_id}/info/refs",
                web::get().to(git_smart_http_handlers::info_refs),
            )
            .route(
                "/{repo_id}/git-upload-pack",
                web::post().to(git_smart_http_handlers::upload_pack),
            )
            .route(
                "/{repo_id}/git-receive-pack",
                web::post().to(git_smart_http_handlers::receive_pack),
            ),
    );
}
