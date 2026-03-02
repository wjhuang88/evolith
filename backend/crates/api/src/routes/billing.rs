//! Billing routes
//! Handles plans, subscriptions, invoices, and usage endpoints

use crate::handlers::billing_handlers::*;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/billing")
            // Plans (public)
            .route("/plans", web::get().to(get_plans))
            // Subscription
            .route("/subscription", web::get().to(get_subscription))
            .route("/subscription", web::post().to(create_subscription))
            .route("/subscription", web::patch().to(update_subscription))
            .route("/subscription", web::delete().to(cancel_subscription))
            // Invoices
            .route("/invoices", web::get().to(get_invoices))
            // Usage
            .route("/usage", web::get().to(get_usage)),
    );
}
