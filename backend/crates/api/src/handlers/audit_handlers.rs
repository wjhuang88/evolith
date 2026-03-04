use actix_web::{
    web::{self, Path, Query},
    HttpResponse, Result,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

/// In-memory audit log storage
pub struct AuditStore {
    pub logs: Mutex<Vec<AuditLogEntry>>,
}

impl Default for AuditStore {
    fn default() -> Self {
        Self {
            logs: Mutex::new(Vec::new()),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct AuditLogEntry {
    pub id: String,
    pub tenant_id: String,
    pub user_id: Option<String>,
    pub action: String,
    pub resource_type: Option<String>,
    pub resource_id: Option<String>,
    pub details: serde_json::Value,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: String,
}

#[derive(Deserialize)]
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

/// Audit state for storing logs in memory
#[derive(Clone)]
pub struct AuditState {
    pub store: Arc<AuditStore>,
}

impl AuditState {
    pub fn new() -> Self {
        Self {
            store: Arc::new(AuditStore::default()),
        }
    }

    pub fn log(&self, entry: AuditLogEntry) {
        if let Ok(mut logs) = self.store.logs.lock() {
            logs.push(entry);
        }
    }

    pub fn get_tenant_logs(
        &self,
        tenant_id: &str,
        limit: usize,
        offset: usize,
    ) -> Vec<AuditLogEntry> {
        if let Ok(logs) = self.store.logs.lock() {
            logs.iter()
                .filter(|l| l.tenant_id == tenant_id)
                .skip(offset)
                .take(limit)
                .cloned()
                .collect()
        } else {
            vec![]
        }
    }
}

impl Default for AuditState {
    fn default() -> Self {
        Self::new()
    }
}

/// Handler to get audit logs for a tenant
pub async fn get_tenant_audit_logs(
    path: Path<String>,
    pagination: Query<PaginationQuery>,
    state: web::Data<AuditState>,
) -> Result<HttpResponse> {
    let tenant_id = path.as_ref().clone();
    let logs = state.get_tenant_logs(&tenant_id, pagination.limit, pagination.offset);
    Ok(HttpResponse::Ok().json(logs))
}

/// Handler to get audit logs by user
pub async fn get_user_audit_logs(
    _path: Path<String>,
    _pagination: Query<PaginationQuery>,
) -> Result<HttpResponse> {
    let logs: Vec<AuditLogEntry> = vec![];
    Ok(HttpResponse::Ok().json(logs))
}

/// Handler to get audit logs by action
pub async fn get_action_audit_logs(
    _query: Query<ActionQuery>,
    _pagination: Query<PaginationQuery>,
) -> Result<HttpResponse> {
    let logs: Vec<AuditLogEntry> = vec![];
    Ok(HttpResponse::Ok().json(logs))
}

#[derive(Deserialize)]
pub struct ActionQuery {
    pub action: String,
}
