//! Health check endpoints

use actix_web::web;

use crate::handlers::health;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/health")
            .route("", web::get().to(health::health_check))
            .route("/live", web::get().to(health::liveness))
            .route("/ready", web::get().to(health::readiness)),
    );
}
