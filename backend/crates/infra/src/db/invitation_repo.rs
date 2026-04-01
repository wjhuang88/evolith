//! SQLite Invitation Repository implementation

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use common::error::{AppError, Result};
use domain::repository::{InvitationRepository, NewInvitation};
use domain::Invitation;
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct SqliteInvitationRepository {
    pool: SqlitePool,
}

impl SqliteInvitationRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl InvitationRepository for SqliteInvitationRepository {
    async fn create(&self, invitation: NewInvitation) -> Result<Invitation> {
        let new_id = Uuid::new_v4();

        sqlx::query(
            r#"
            INSERT INTO tenant_invitations (id, tenant_id, email, role, token, expires_at, created_by, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(new_id.to_string())
        .bind(invitation.tenant_id.to_string())
        .bind(&invitation.email)
        .bind(&invitation.role)
        .bind(&invitation.token)
        .bind(invitation.expires_at.to_rfc3339())
        .bind(invitation.created_by.to_string())
        .bind(Utc::now().to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Return the created invitation
        self.find_by_id(new_id).await?.ok_or_else(|| {
            AppError::DatabaseError("Failed to fetch created invitation".to_string())
        })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Invitation>> {
        let row =
            sqlx::query_as::<_, InvitationRow>(r#"SELECT * FROM tenant_invitations WHERE id = ?"#)
                .bind(id.to_string())
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| r.into()))
    }

    async fn find_by_token(&self, token: &str) -> Result<Option<Invitation>> {
        let row = sqlx::query_as::<_, InvitationRow>(
            r#"SELECT * FROM tenant_invitations WHERE token = ?"#,
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| r.into()))
    }

    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<Invitation>> {
        let rows = sqlx::query_as::<_, InvitationRow>(
            r#"SELECT * FROM tenant_invitations WHERE tenant_id = ? AND accepted_at IS NULL ORDER BY created_at DESC"#,
        )
        .bind(tenant_id.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn find_by_email(&self, tenant_id: Uuid, email: &str) -> Result<Option<Invitation>> {
        let row = sqlx::query_as::<_, InvitationRow>(
            r#"SELECT * FROM tenant_invitations WHERE tenant_id = ? AND email = ?"#,
        )
        .bind(tenant_id.to_string())
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| r.into()))
    }

    async fn accept(&self, id: Uuid) -> Result<()> {
        let accepted_at = Utc::now().to_rfc3339();
        sqlx::query(r#"UPDATE tenant_invitations SET accepted_at = ? WHERE id = ?"#)
            .bind(accepted_at)
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query(r#"DELETE FROM tenant_invitations WHERE id = ?"#)
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct InvitationRow {
    id: String,
    tenant_id: String,
    email: String,
    role: String,
    token: String,
    expires_at: String,
    accepted_at: Option<String>,
    created_by: String,
    created_at: String,
}

impl From<InvitationRow> for Invitation {
    fn from(row: InvitationRow) -> Self {
        Invitation {
            id: row.id.parse().unwrap_or_default(),
            tenant_id: row.tenant_id.parse().unwrap_or_default(),
            email: row.email,
            role: row.role,
            token: row.token,
            expires_at: DateTime::parse_from_rfc3339(&row.expires_at)
                .unwrap_or_else(|_| Utc::now().into())
                .with_timezone(&Utc),
            accepted_at: row.accepted_at.and_then(|s| {
                DateTime::parse_from_rfc3339(&s)
                    .ok()
                    .map(|dt| dt.with_timezone(&Utc))
            }),
            created_by: row.created_by.parse().unwrap_or_default(),
            created_at: DateTime::parse_from_rfc3339(&row.created_at)
                .unwrap_or_else(|_| Utc::now().into())
                .with_timezone(&Utc),
        }
    }
}
