//! API Key management routes
//! Handles tenant API key endpoints

use crate::handlers::api_key_handlers;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api-keys")
            // List API keys
            .route("", web::get().to(api_key_handlers::list_api_keys))
            // Create API key
            .route("", web::post().to(api_key_handlers::create_api_key))
            // Revoke API key
            .route(
                "/{key_id}",
                web::delete().to(api_key_handlers::revoke_api_key),
            ),
    );
}
