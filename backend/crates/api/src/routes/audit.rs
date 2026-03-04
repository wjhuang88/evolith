use actix_web::web;

use crate::handlers::audit_handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/audit-logs")
            // Get tenant audit logs - no path param needed (tenant_id from JWT)
            .route("", web::get().to(audit_handlers::get_tenant_audit_logs))
            // Get user audit logs
            .route(
                "/user/{user_id}",
                web::get().to(audit_handlers::get_user_audit_logs),
            )
            // Get action audit logs
            .route(
                "/action",
                web::get().to(audit_handlers::get_action_audit_logs),
            ),
    );
}
