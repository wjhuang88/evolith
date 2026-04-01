//! PostgreSQL Invitation Repository implementation

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use common::error::{AppError, Result};
use domain::repository::{InvitationRepository, NewInvitation};
use domain::Invitation;
use sqlx::PgPool;
use uuid::Uuid;

pub struct PgInvitationRepository {
    pool: PgPool,
}

impl PgInvitationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl InvitationRepository for PgInvitationRepository {
    async fn create(&self, invitation: NewInvitation) -> Result<Invitation> {
        let new_id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO tenant_invitations (id, tenant_id, email, role, token, expires_at, created_by, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(new_id)
        .bind(invitation.tenant_id)
        .bind(&invitation.email)
        .bind(&invitation.role)
        .bind(&invitation.token)
        .bind(invitation.expires_at)
        .bind(invitation.created_by)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Return the created invitation
        self.find_by_id(new_id).await?.ok_or_else(|| {
            AppError::DatabaseError("Failed to fetch created invitation".to_string())
        })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Invitation>> {
        let row = sqlx::query_as::<_, PgInvitationRow>(
            r#"SELECT * FROM tenant_invitations WHERE id = $1"#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| r.into()))
    }

    async fn find_by_token(&self, token: &str) -> Result<Option<Invitation>> {
        let row = sqlx::query_as::<_, PgInvitationRow>(
            r#"SELECT * FROM tenant_invitations WHERE token = $1"#,
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| r.into()))
    }

    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<Invitation>> {
        let rows = sqlx::query_as::<_, PgInvitationRow>(
            r#"SELECT * FROM tenant_invitations WHERE tenant_id = $1 AND accepted_at IS NULL ORDER BY created_at DESC"#,
        )
        .bind(tenant_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn find_by_email(&self, tenant_id: Uuid, email: &str) -> Result<Option<Invitation>> {
        let row = sqlx::query_as::<_, PgInvitationRow>(
            r#"SELECT * FROM tenant_invitations WHERE tenant_id = $1 AND email = $2"#,
        )
        .bind(tenant_id)
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| r.into()))
    }

    async fn accept(&self, id: Uuid) -> Result<()> {
        let accepted_at = Utc::now();
        sqlx::query(r#"UPDATE tenant_invitations SET accepted_at = $1 WHERE id = $2"#)
            .bind(accepted_at)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query(r#"DELETE FROM tenant_invitations WHERE id = $1"#)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct PgInvitationRow {
    id: Uuid,
    tenant_id: Uuid,
    email: String,
    role: String,
    token: String,
    expires_at: DateTime<Utc>,
    accepted_at: Option<DateTime<Utc>>,
    created_by: Uuid,
    created_at: DateTime<Utc>,
}

impl From<PgInvitationRow> for Invitation {
    fn from(row: PgInvitationRow) -> Self {
        Invitation {
            id: row.id,
            tenant_id: row.tenant_id,
            email: row.email,
            role: row.role,
            token: row.token,
            expires_at: row.expires_at,
            accepted_at: row.accepted_at,
            created_by: row.created_by,
            created_at: row.created_at,
        }
    }
}
