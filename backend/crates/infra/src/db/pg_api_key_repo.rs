//! PostgreSQL API Key Repository implementation

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use common::error::{AppError, Result};
use domain::api_key::{ApiKey, ApiKeyStatus, NewApiKey};
use domain::repository::ApiKeyRepository;
use sqlx::PgPool;
use uuid::Uuid;

pub struct PgApiKeyRepository {
    pool: PgPool,
}

impl PgApiKeyRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ApiKeyRepository for PgApiKeyRepository {
    async fn create(&self, api_key: NewApiKey) -> Result<ApiKey> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let rate_limit = api_key.rate_limit.unwrap_or(1000);

        sqlx::query(
            r#"
            INSERT INTO api_keys (
                id, tenant_id, user_id, name, key_hash, key_prefix,
                permissions, rate_limit, status, request_count,
                expires_at, last_used_at, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            "#,
        )
        .bind(id)
        .bind(api_key.tenant_id)
        .bind(api_key.user_id)
        .bind(&api_key.name)
        .bind(&api_key.key_hash)
        .bind(&api_key.key_prefix)
        .bind(serde_json::to_value(&api_key.permissions).map_err(|e| {
            AppError::ValidationError(format!("Failed to serialize permissions: {}", e))
        })?)
        .bind(rate_limit as i32)
        .bind("active")
        .bind(0i32)
        .bind(api_key.expires_at)
        .bind(None::<DateTime<Utc>>)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(ApiKey {
            id,
            tenant_id: api_key.tenant_id,
            user_id: api_key.user_id,
            name: api_key.name,
            key_hash: api_key.key_hash,
            key_prefix: api_key.key_prefix,
            permissions: api_key.permissions,
            status: ApiKeyStatus::Active,
            rate_limit,
            request_count: 0,
            last_used_at: None,
            expires_at: api_key.expires_at,
            created_at: now,
            updated_at: now,
        })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<ApiKey>> {
        let row = sqlx::query_as::<_, PgApiKeyRow>(r#"SELECT * FROM api_keys WHERE id = $1"#)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| r.into()))
    }

    async fn find_by_key(&self, key_hash: &str) -> Result<Option<ApiKey>> {
        let row = sqlx::query_as::<_, PgApiKeyRow>(r#"SELECT * FROM api_keys WHERE key_hash = $1"#)
            .bind(key_hash)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| r.into()))
    }

    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<ApiKey>> {
        let rows = sqlx::query_as::<_, PgApiKeyRow>(
            r#"SELECT * FROM api_keys WHERE tenant_id = $1 ORDER BY created_at DESC"#,
        )
        .bind(tenant_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn revoke(&self, id: Uuid) -> Result<()> {
        let now = Utc::now();
        sqlx::query(r#"UPDATE api_keys SET status = $1, updated_at = $2 WHERE id = $3"#)
            .bind("revoked")
            .bind(now)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query(r#"DELETE FROM api_keys WHERE id = $1"#)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn update_last_used(&self, id: Uuid) -> Result<()> {
        let now = Utc::now();
        sqlx::query(
            r#"UPDATE api_keys SET last_used_at = $1, request_count = request_count + 1, updated_at = $2 WHERE id = $3"#
        )
        .bind(now)
        .bind(now)
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct PgApiKeyRow {
    id: Uuid,
    tenant_id: Uuid,
    user_id: Uuid,
    name: String,
    key_hash: String,
    key_prefix: String,
    permissions: Option<serde_json::Value>,
    rate_limit: Option<i32>,
    status: Option<String>,
    request_count: Option<i32>,
    last_used_at: Option<DateTime<Utc>>,
    expires_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<PgApiKeyRow> for ApiKey {
    fn from(row: PgApiKeyRow) -> Self {
        ApiKey {
            id: row.id,
            tenant_id: row.tenant_id,
            user_id: row.user_id,
            name: row.name,
            key_hash: row.key_hash,
            key_prefix: row.key_prefix,
            permissions: row
                .permissions
                .map(|v| serde_json::from_value(v).unwrap_or_default())
                .unwrap_or_default(),
            status: match row.status.as_deref() {
                Some("active") => ApiKeyStatus::Active,
                Some("revoked") => ApiKeyStatus::Revoked,
                Some("expired") => ApiKeyStatus::Expired,
                _ => ApiKeyStatus::Active,
            },
            rate_limit: row.rate_limit.unwrap_or(1000) as u32,
            request_count: row.request_count.unwrap_or(0) as u32,
            last_used_at: row.last_used_at,
            expires_at: row.expires_at,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}
