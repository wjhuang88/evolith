//! Audit log handlers
//! Handles retrieval of audit logs for tenant administrators

use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use uuid::Uuid;

use crate::dto::common::ApiResponse;
use crate::middleware::auth::AuthenticatedUser;
use crate::state::AppState;
use domain::audit::AuditLog;

/// Pagination query parameters
#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "default_limit")]
    pub limit: usize,
    #[serde(default = "default_offset")]
    pub offset: usize,
}

fn default_limit() -> usize {
    50
}

fn default_offset() -> usize {
    0
}

/// Audit log response DTO
#[derive(Debug, serde::Serialize)]
pub struct AuditLogResponse {
    pub id: String,
    pub tenant_id: Option<String>,
    pub user_id: Option<String>,
    pub action: String,
    pub resource_type: Option<String>,
    pub resource_id: Option<String>,
    pub details: serde_json::Value,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: String,
}

impl From<AuditLog> for AuditLogResponse {
    fn from(log: AuditLog) -> Self {
        Self {
            id: log.id.to_string(),
            tenant_id: log.tenant_id.map(|id| id.to_string()),
            user_id: log.user_id.map(|id| id.to_string()),
            action: log.action,
            resource_type: log.resource_type,
            resource_id: log.resource_id,
            details: log.details,
            ip_address: log.ip_address,
            user_agent: log.user_agent,
            created_at: log.created_at.to_rfc3339(),
        }
    }
}

/// Audit log list response
#[derive(Debug, serde::Serialize)]
pub struct AuditLogListResponse {
    pub logs: Vec<AuditLogResponse>,
    pub total: usize,
}

/// List audit logs for a tenant
pub async fn list_audit_logs(
    tenant_id: web::Path<Uuid>,
    user: AuthenticatedUser,
    pagination: web::Query<PaginationQuery>,
    state: web::Data<AppState>,
) -> impl Responder {
    let tenant_id = tenant_id.into_inner();

    // Verify user has access to this tenant
    if user.tenant_id != tenant_id {
        return HttpResponse::Forbidden().json(ApiResponse::<()>::error(
            "FORBIDDEN",
            "You do not have access to this tenant",
        ));
    }

    // Only admins can view audit logs
    if !user.is_admin() {
        return HttpResponse::Forbidden().json(ApiResponse::<()>::error(
            "FORBIDDEN",
            "Only admins can view audit logs",
        ));
    }

    // Fetch audit logs from repository
    match state
        .audit_repo
        .find_by_tenant(tenant_id, pagination.limit, pagination.offset)
        .await
    {
        Ok(logs) => {
            let log_responses: Vec<AuditLogResponse> =
                logs.into_iter().map(AuditLogResponse::from).collect();

            let total = log_responses.len();
            HttpResponse::Ok().json(ApiResponse::<AuditLogListResponse>::success(
                AuditLogListResponse {
                    logs: log_responses,
                    total,
                },
            ))
        }
        Err(e) => {
            tracing::error!("Failed to list audit logs: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to list audit logs",
            ))
        }
    }
}

/// Get a single audit log by ID
pub async fn get_audit_log(
    path: web::Path<(Uuid, Uuid)>,
    user: AuthenticatedUser,
    _state: web::Data<AppState>,
) -> impl Responder {
    let (tenant_id, _log_id) = path.into_inner();

    // Verify user has access to this tenant
    if user.tenant_id != tenant_id {
        return HttpResponse::Forbidden().json(ApiResponse::<()>::error(
            "FORBIDDEN",
            "You do not have access to this tenant",
        ));
    }

    // Only admins can view audit logs
    if !user.is_admin() {
        return HttpResponse::Forbidden().json(ApiResponse::<()>::error(
            "FORBIDDEN",
            "Only admins can view audit logs",
        ));
    }

    // Note: AuditRepository doesn't have find_by_id method
    // Return 501 Not Implemented for now
    HttpResponse::NotImplemented().json(ApiResponse::<()>::error(
        "NOT_IMPLEMENTED",
        "Get audit log by ID is not yet implemented",
    ))
}
