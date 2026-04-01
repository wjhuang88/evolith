//! Health check handlers

use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;

use crate::state::AppState;

#[derive(Serialize)]
struct HealthCheckResponse {
    status: &'static str,
    version: &'static str,
}

#[derive(Serialize)]
struct LivenessResponse {
    status: &'static str,
    version: &'static str,
}

#[derive(Serialize)]
struct ReadinessResponse {
    status: &'static str,
    version: &'static str,
    checks: HealthChecks,
}

#[derive(Serialize)]
struct HealthChecks {
    database: bool,
}

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(HealthCheckResponse {
        status: "healthy",
        version: env!("CARGO_PKG_VERSION"),
    })
}

pub async fn liveness() -> impl Responder {
    HttpResponse::Ok().json(LivenessResponse {
        status: "ok",
        version: env!("CARGO_PKG_VERSION"),
    })
}

pub async fn readiness(state: web::Data<AppState>) -> impl Responder {
    let db_healthy = check_database(&state).await;

    let status = if db_healthy { "healthy" } else { "degraded" };

    HttpResponse::Ok().json(ReadinessResponse {
        status,
        version: env!("CARGO_PKG_VERSION"),
        checks: HealthChecks {
            database: db_healthy,
        },
    })
}

async fn check_database(state: &AppState) -> bool {
    state
        .user_repo
        .find_by_id(uuid::Uuid::nil())
        .await
        .map(|_| true)
        .unwrap_or(false)
}
