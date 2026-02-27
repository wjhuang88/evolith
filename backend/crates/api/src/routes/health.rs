//! Health check endpoints

use actix_web::{web, HttpResponse, Responder};

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

pub async fn liveness() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok"
    }))
}

pub async fn readiness() -> impl Responder {
    // TODO: Check database, cache connections
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ready"
    }))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/health")
            .route("", web::get().to(health_check))
            .route("/live", web::get().to(liveness))
            .route("/ready", web::get().to(readiness)),
    );
}
