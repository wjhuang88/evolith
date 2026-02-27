//! Health check handlers

use actix_web::{HttpResponse, Responder};
use serde_json::json;

/// Health check response
#[derive(serde::Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

/// Liveness probe - just checks if service is running
pub async fn liveness() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

/// Readiness probe - checks if service can handle requests
pub async fn readiness() -> impl Responder {
    // TODO: Add checks for database, cache, etc.
    HttpResponse::Ok().json(HealthResponse {
        status: "ready".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}
