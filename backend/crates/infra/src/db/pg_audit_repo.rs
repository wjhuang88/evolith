//! PostgreSQL Audit Repository implementation

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use common::error::{AppError, Result};
use domain::audit::AuditLog;
use domain::repository::AuditRepository;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct PgAuditRepository {
    pool: PgPool,
}

impl PgAuditRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuditRepository for PgAuditRepository {
    async fn create(&self, log: AuditLog) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO audit_logs (
                id, tenant_id, user_id, action, resource_type, resource_id, 
                details, ip_address, user_agent, created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
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
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn find_by_tenant(
        &self,
        tenant_id: Uuid,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<AuditLog>> {
        let rows = sqlx::query_as::<_, PgAuditLogRow>(
            r#"
            SELECT id, tenant_id, user_id, action, resource_type, resource_id, 
                   details, ip_address, user_agent, created_at
            FROM audit_logs 
            WHERE tenant_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(tenant_id)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn find_by_user(
        &self,
        user_id: Uuid,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<AuditLog>> {
        let rows = sqlx::query_as::<_, PgAuditLogRow>(
            r#"
            SELECT id, tenant_id, user_id, action, resource_type, resource_id, 
                   details, ip_address, user_agent, created_at
            FROM audit_logs 
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(user_id)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn find_by_action(
        &self,
        action: &str,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<AuditLog>> {
        let rows = sqlx::query_as::<_, PgAuditLogRow>(
            r#"
            SELECT id, tenant_id, user_id, action, resource_type, resource_id, 
                   details, ip_address, user_agent, created_at
            FROM audit_logs 
            WHERE action = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(action)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
}

#[derive(sqlx::FromRow)]
struct PgAuditLogRow {
    id: Uuid,
    tenant_id: Option<Uuid>,
    user_id: Option<Uuid>,
    action: String,
    resource_type: Option<String>,
    resource_id: Option<String>,
    details: serde_json::Value,
    ip_address: Option<String>,
    user_agent: Option<String>,
    created_at: DateTime<Utc>,
}

impl From<PgAuditLogRow> for AuditLog {
    fn from(row: PgAuditLogRow) -> Self {
        AuditLog {
            id: row.id,
            tenant_id: row.tenant_id,
            user_id: row.user_id,
            action: row.action,
            resource_type: row.resource_type,
            resource_id: row.resource_id,
            details: row.details,
            ip_address: row.ip_address,
            user_agent: row.user_agent,
            created_at: row.created_at,
        }
    }
}
