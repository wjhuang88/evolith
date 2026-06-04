//! PostgreSQL User Repository implementation

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use common::error::{AppError, Result};
use domain::repository::UserRepository;
use domain::user::{NewUser, TenantRole, UpdateUser, User, UserRole};
use sqlx::PgPool;
use uuid::Uuid;

pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn create(
        &self,
        user: NewUser,
        tenant_id: Uuid,
        tenant_role: TenantRole,
    ) -> Result<User> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let role_str = match UserRole::default() {
            UserRole::Admin => "admin",
            UserRole::User => "user",
        };
        let tenant_role_str = match tenant_role {
            TenantRole::Owner => "owner",
            TenantRole::Admin => "admin",
            TenantRole::Member => "member",
        };

        sqlx::query(
            r#"
            INSERT INTO users (
                id, tenant_id, username, email, password_hash, role, tenant_role,
                email_verified, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
        )
        .bind(id)
        .bind(tenant_id)
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(role_str)
        .bind(tenant_role_str)
        .bind(false)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(User {
            id,
            username: user.username,
            email: user.email,
            password_hash: user.password_hash,
            role: UserRole::default(),
            tenant_id,
            tenant_role,
            email_verified: false,
            verify_token: None,
            verified_at: None,
            reset_token: None,
            reset_expires_at: None,
            created_at: now,
            updated_at: now,
        })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let row = sqlx::query_as::<_, PgUserRow>(r#"SELECT * FROM users WHERE id = $1"#)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| r.into()))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let row = sqlx::query_as::<_, PgUserRow>(r#"SELECT * FROM users WHERE email = $1"#)
            .bind(email)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| r.into()))
    }

    async fn find_by_username(&self, username: &str, tenant_id: Uuid) -> Result<Option<User>> {
        let row = sqlx::query_as::<_, PgUserRow>(
            r#"SELECT * FROM users WHERE username = $1 AND tenant_id = $2"#,
        )
        .bind(username)
        .bind(tenant_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| r.into()))
    }

    async fn update(&self, id: Uuid, user: UpdateUser) -> Result<User> {
        let now = Utc::now();

        if let Some(username) = &user.username {
            sqlx::query(r#"UPDATE users SET username = $1, updated_at = $2 WHERE id = $3"#)
                .bind(username)
                .bind(now)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        }

        if let Some(email) = &user.email {
            sqlx::query(r#"UPDATE users SET email = $1, updated_at = $2 WHERE id = $3"#)
                .bind(email)
                .bind(now)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        }

        self.find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFoundError(format!("User {} not found", id)))
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query(r#"DELETE FROM users WHERE id = $1"#)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn verify_email(&self, id: Uuid) -> Result<()> {
        let now = Utc::now();
        sqlx::query(
            r#"UPDATE users SET email_verified = $1, verified_at = $2, verify_token = $3, updated_at = $4 WHERE id = $5"#
        )
        .bind(true)
        .bind(Some(now))
        .bind(None::<String>)
        .bind(now)
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn set_verify_token(&self, id: Uuid, token: &str) -> Result<()> {
        sqlx::query(r#"UPDATE users SET verify_token = $1 WHERE id = $2"#)
            .bind(token)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn find_by_verify_token(&self, token: &str) -> Result<Option<User>> {
        let row = sqlx::query_as::<_, PgUserRow>(r#"SELECT * FROM users WHERE verify_token = $1"#)
            .bind(token)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| r.into()))
    }

    async fn set_reset_token(
        &self,
        id: Uuid,
        token: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<()> {
        sqlx::query(r#"UPDATE users SET reset_token = $1, reset_expires_at = $2 WHERE id = $3"#)
            .bind(token)
            .bind(expires_at)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn find_by_reset_token(&self, token: &str) -> Result<Option<User>> {
        let row = sqlx::query_as::<_, PgUserRow>(r#"SELECT * FROM users WHERE reset_token = $1"#)
            .bind(token)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| r.into()))
    }

    async fn clear_reset_token(&self, id: Uuid) -> Result<()> {
        sqlx::query(r#"UPDATE users SET reset_token = $1, reset_expires_at = $2 WHERE id = $3"#)
            .bind(None::<String>)
            .bind(None::<DateTime<Utc>>)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn update_password(&self, id: Uuid, password_hash: &str) -> Result<()> {
        let now = Utc::now();
        sqlx::query(r#"UPDATE users SET password_hash = $1, updated_at = $2 WHERE id = $3"#)
            .bind(password_hash)
            .bind(now)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<User>> {
        let rows = sqlx::query_as::<_, PgUserRow>(
            r#"SELECT * FROM users WHERE tenant_id = $1 ORDER BY created_at ASC"#,
        )
        .bind(tenant_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn remove_from_tenant(&self, user_id: Uuid) -> Result<()> {
        let now = Utc::now();
        sqlx::query(
            r#"UPDATE users SET tenant_id = NULL, tenant_role = NULL, updated_at = $1 WHERE id = $2"#,
        )
        .bind(now)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct PgUserRow {
    id: Uuid,
    tenant_id: Uuid,
    username: String,
    email: String,
    password_hash: String,
    role: String,
    tenant_role: String,
    email_verified: bool,
    verify_token: Option<String>,
    verified_at: Option<DateTime<Utc>>,
    reset_token: Option<String>,
    reset_expires_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<PgUserRow> for User {
    fn from(row: PgUserRow) -> Self {
        User {
            id: row.id,
            tenant_id: row.tenant_id,
            username: row.username,
            email: row.email,
            password_hash: row.password_hash,
            role: match row.role.as_str() {
                "admin" => UserRole::Admin,
                _ => UserRole::User,
            },
            tenant_role: match row.tenant_role.as_str() {
                "owner" => TenantRole::Owner,
                "admin" => TenantRole::Admin,
                _ => TenantRole::Member,
            },
            email_verified: row.email_verified,
            verify_token: row.verify_token,
            verified_at: row.verified_at,
            reset_token: row.reset_token,
            reset_expires_at: row.reset_expires_at,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}
