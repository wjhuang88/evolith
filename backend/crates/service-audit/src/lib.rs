use chrono::{DateTime, Utc};
use common::error::Result;
use domain::audit::AuditLog;
use domain::repository::AuditRepository;
use serde::{Deserialize, Serialize};
use uuid::Uuid as RealUuid;

#[derive(Clone)]
pub struct AuditService<R: AuditRepository> {
    audit_repo: R,
}

impl<R: AuditRepository> AuditService<R> {
    pub fn new(audit_repo: R) -> Self {
        Self { audit_repo }
    }

    pub async fn log_action(
        &self,
        tenant_id: Option<RealUuid>,
        user_id: Option<RealUuid>,
        action: &str,
        resource_type: Option<&str>,
        resource_id: Option<&str>,
        details: serde_json::Value,
    ) -> Result<()> {
        let log = AuditLog {
            id: RealUuid::new_v4(),
            tenant_id,
            user_id,
            action: action.to_string(),
            resource_type: resource_type.map(|s| s.to_string()),
            resource_id: resource_id.map(|s| s.to_string()),
            details,
            ip_address: None, // Will be passed separately if needed
            user_agent: None, // Will be passed separately if needed
            created_at: Utc::now(),
        };

        self.audit_repo.create(log).await
    }

    pub async fn get_tenant_logs(
        &self,
        tenant_id: RealUuid,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<AuditLog>> {
        self.audit_repo
            .find_by_tenant(tenant_id, limit, offset)
            .await
    }

    pub async fn get_user_logs(
        &self,
        user_id: RealUuid,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<AuditLog>> {
        self.audit_repo.find_by_user(user_id, limit, offset).await
    }

    pub async fn get_action_logs(
        &self,
        action: &str,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<AuditLog>> {
        self.audit_repo.find_by_action(action, limit, offset).await
    }
}

#[derive(Debug, Clone)]
pub struct AuditContext {
    pub tenant_id: Option<RealUuid>,
    pub user_id: Option<RealUuid>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

impl AuditContext {
    pub fn new(tenant_id: Option<RealUuid>, user_id: Option<RealUuid>) -> Self {
        Self {
            tenant_id,
            user_id,
            ip_address: None,
            user_agent: None,
        }
    }

    pub fn with_request_info(
        mut self,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Self {
        self.ip_address = ip_address;
        self.user_agent = user_agent;
        self
    }
}
