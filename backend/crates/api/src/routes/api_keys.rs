//! API Key management routes
//! Handles tenant API key endpoints

use crate::handlers::api_key_handlers::*;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api-keys")
            // List API keys
            .route("", web::get().to(list_api_keys))
            // Create API key
            .route("", web::post().to(create_api_key))
            // Get API key details
            .route("/{key_id}", web::get().to(get_api_key))
            // Revoke API key
            .route("/{key_id}", web::delete().to(revoke_api_key))
            // Rotate API key
            .route("/{key_id}/rotate", web::post().to(rotate_api_key)),
    );
}
