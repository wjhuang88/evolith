use async_trait::async_trait;
use chrono::{DateTime, Utc};
use common::error::{AppError, Result};
use domain::api_key::{ApiKey, ApiKeyStatus, NewApiKey};
use domain::repository::ApiKeyRepository;
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct SqliteApiKeyRepository {
    pool: SqlitePool,
}

impl SqliteApiKeyRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ApiKeyRepository for SqliteApiKeyRepository {
    async fn create(&self, api_key: NewApiKey) -> Result<ApiKey> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let permissions_str = serde_json::to_string(&api_key.permissions).map_err(|e| {
            AppError::ValidationError(format!("Failed to serialize permissions: {}", e))
        })?;
        let rate_limit = api_key.rate_limit.unwrap_or(1000);

        sqlx::query(
            r#"
            INSERT INTO api_keys (
                id, tenant_id, user_id, name, key_hash, key_prefix,
                permissions, rate_limit, status, request_count,
                expires_at, last_used_at, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(id.to_string())
        .bind(api_key.tenant_id.to_string())
        .bind(api_key.user_id.to_string())
        .bind(&api_key.name)
        .bind(&api_key.key_hash)
        .bind(&api_key.key_prefix)
        .bind(&permissions_str)
        .bind(rate_limit as i32)
        .bind("active")
        .bind(0i32)
        .bind(api_key.expires_at.map(|e| e.to_rfc3339()))
        .bind(None::<String>)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
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
        let row = sqlx::query_as::<_, ApiKeyRow>(r#"SELECT * FROM api_keys WHERE id = ?"#)
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| r.into()))
    }

    async fn find_by_key(&self, key_hash: &str) -> Result<Option<ApiKey>> {
        let row = sqlx::query_as::<_, ApiKeyRow>(r#"SELECT * FROM api_keys WHERE key_hash = ?"#)
            .bind(key_hash)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| r.into()))
    }

    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<ApiKey>> {
        let rows = sqlx::query_as::<_, ApiKeyRow>(
            r#"SELECT * FROM api_keys WHERE tenant_id = ? ORDER BY created_at DESC"#,
        )
        .bind(tenant_id.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn revoke(&self, id: Uuid) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(r#"UPDATE api_keys SET status = ?, updated_at = ? WHERE id = ?"#)
            .bind("revoked")
            .bind(&now)
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query(r#"DELETE FROM api_keys WHERE id = ?"#)
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn update_last_used(&self, id: Uuid) -> Result<()> {
        let now = Utc::now();
        sqlx::query(
            r#"UPDATE api_keys SET last_used_at = ?, request_count = request_count + 1, updated_at = ? WHERE id = ?"#
        )
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct ApiKeyRow {
    id: String,
    tenant_id: String,
    user_id: String,
    name: String,
    key_hash: String,
    key_prefix: String,
    permissions: Option<String>,
    rate_limit: Option<i32>,
    status: Option<String>,
    request_count: Option<i32>,
    last_used_at: Option<String>,
    expires_at: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<ApiKeyRow> for ApiKey {
    fn from(row: ApiKeyRow) -> Self {
        ApiKey {
            id: row.id.parse().unwrap_or_default(),
            tenant_id: row.tenant_id.parse().unwrap_or_default(),
            user_id: row.user_id.parse().unwrap_or_default(),
            name: row.name,
            key_hash: row.key_hash,
            key_prefix: row.key_prefix,
            permissions: row
                .permissions
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default(),
            status: match row.status.as_deref() {
                Some("active") => ApiKeyStatus::Active,
                Some("revoked") => ApiKeyStatus::Revoked,
                Some("expired") => ApiKeyStatus::Expired,
                _ => ApiKeyStatus::Active,
            },
            rate_limit: row.rate_limit.unwrap_or(1000) as u32,
            request_count: row.request_count.unwrap_or(0) as u32,
            last_used_at: row.last_used_at.and_then(|s| {
                DateTime::parse_from_rfc3339(&s)
                    .ok()
                    .map(|dt| dt.with_timezone(&Utc))
            }),
            expires_at: row.expires_at.and_then(|s| {
                DateTime::parse_from_rfc3339(&s)
                    .ok()
                    .map(|dt| dt.with_timezone(&Utc))
            }),
            created_at: DateTime::parse_from_rfc3339(&row.created_at)
                .unwrap_or_else(|_| Utc::now().into())
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&row.updated_at)
                .unwrap_or_else(|_| Utc::now().into())
                .with_timezone(&Utc),
        }
    }
}
