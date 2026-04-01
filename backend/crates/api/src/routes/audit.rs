//! Audit log routes

use actix_web::web;

use crate::handlers::audit_handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/audit-logs")
            // List audit logs for tenant (tenant_id from path, verified in handler)
            .route("", web::get().to(audit_handlers::list_audit_logs))
            // Get single audit log by ID
            .route("/{log_id}", web::get().to(audit_handlers::get_audit_log)),
    );
}
