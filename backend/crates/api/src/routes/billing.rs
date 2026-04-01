//! Billing routes

use crate::handlers::billing_handlers;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/billing")
            .route("/plans", web::get().to(billing_handlers::get_plans))
            .route(
                "/subscription",
                web::get().to(billing_handlers::get_current_plan),
            )
            .route(
                "/subscription/checkout",
                web::post().to(billing_handlers::create_checkout_session),
            )
            .route(
                "/subscription/manage",
                web::post().to(billing_handlers::manage_subscription),
            )
            .route("/usage", web::get().to(billing_handlers::get_usage)),
    );
    // TODO: re-enable /webhooks/stripe route after service-payment is fixed (Phase 2)
}
