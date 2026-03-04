//! SQLite Audit Repository implementation

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use common::error::{AppError, Result};
use domain::audit::AuditLog;
use domain::repository::AuditRepository;
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Clone)]
pub struct SqliteAuditRepository {
    pool: SqlitePool,
}

impl SqliteAuditRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuditRepository for SqliteAuditRepository {
    async fn create(&self, log: AuditLog) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO audit_logs (
                id, tenant_id, user_id, action, resource_type, resource_id, 
                details, ip_address, user_agent, created_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(log.id)
        .bind(log.tenant_id)
        .bind(log.user_id)
        .bind(log.action)
        .bind(log.resource_type)
        .bind(log.resource_id)
        .bind(serde_json::to_string(&log.details)?)
        .bind(log.ip_address)
        .bind(log.user_agent)
        .bind(log.created_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_by_tenant(
        &self,
        tenant_id: Uuid,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<AuditLog>> {
        let rows = sqlx::query_as::<_, AuditLogRow>(
            r#"
            SELECT id, tenant_id, user_id, action, resource_type, resource_id, 
                   details, ip_address, user_agent, created_at
            FROM audit_logs 
            WHERE tenant_id = ?
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
        )
        .bind(tenant_id)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn find_by_user(
        &self,
        user_id: Uuid,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<AuditLog>> {
        let rows = sqlx::query_as::<_, AuditLogRow>(
            r#"
            SELECT id, tenant_id, user_id, action, resource_type, resource_id, 
                   details, ip_address, user_agent, created_at
            FROM audit_logs 
            WHERE user_id = ?
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
        )
        .bind(user_id)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn find_by_action(
        &self,
        action: &str,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<AuditLog>> {
        let rows = sqlx::query_as::<_, AuditLogRow>(
            r#"
            SELECT id, tenant_id, user_id, action, resource_type, resource_id, 
                   details, ip_address, user_agent, created_at
            FROM audit_logs 
            WHERE action = ?
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
        )
        .bind(action)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
}

#[derive(sqlx::FromRow)]
struct AuditLogRow {
    id: Uuid,
    tenant_id: Option<Uuid>,
    user_id: Option<Uuid>,
    action: String,
    resource_type: Option<String>,
    resource_id: Option<String>,
    details: String,
    ip_address: Option<String>,
    user_agent: Option<String>,
    created_at: DateTime<Utc>,
}

impl From<AuditLogRow> for AuditLog {
    fn from(row: AuditLogRow) -> Self {
        AuditLog {
            id: row.id,
            tenant_id: row.tenant_id,
            user_id: row.user_id,
            action: row.action,
            resource_type: row.resource_type,
            resource_id: row.resource_id,
            details: serde_json::from_str(&row.details).unwrap_or(serde_json::Value::Null),
            ip_address: row.ip_address,
            user_agent: row.user_agent,
            created_at: row.created_at,
        }
    }
}
