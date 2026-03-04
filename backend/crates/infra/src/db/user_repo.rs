//! SQLite User Repository implementation

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use common::error::{AppError, Result};
use domain::repository::UserRepository;
use domain::user::{NewUser, TenantRole, UpdateUser, User, UserRole};
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct SqliteUserRepository {
    pool: SqlitePool,
}

impl SqliteUserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for SqliteUserRepository {
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
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(id.to_string())
        .bind(tenant_id.to_string())
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.password) // TODO: Hash password
        .bind(role_str)
        .bind(tenant_role_str)
        .bind(false)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(User {
            id,
            username: user.username,
            email: user.email,
            password_hash: user.password,
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
        let row = sqlx::query_as::<_, UserRow>(r#"SELECT * FROM users WHERE id = ?"#)
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| r.into()))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let row = sqlx::query_as::<_, UserRow>(r#"SELECT * FROM users WHERE email = ?"#)
            .bind(email)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| r.into()))
    }

    async fn find_by_username(&self, username: &str, tenant_id: Uuid) -> Result<Option<User>> {
        let row = sqlx::query_as::<_, UserRow>(
            r#"SELECT * FROM users WHERE username = ? AND tenant_id = ?"#,
        )
        .bind(username)
        .bind(tenant_id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| r.into()))
    }

    async fn update(&self, id: Uuid, user: UpdateUser) -> Result<User> {
        let now = Utc::now().to_rfc3339();

        if let Some(username) = &user.username {
            sqlx::query(r#"UPDATE users SET username = ?, updated_at = ? WHERE id = ?"#)
                .bind(username)
                .bind(&now)
                .bind(id.to_string())
                .execute(&self.pool)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        }

        if let Some(email) = &user.email {
            sqlx::query(r#"UPDATE users SET email = ?, updated_at = ? WHERE id = ?"#)
                .bind(email)
                .bind(&now)
                .bind(id.to_string())
                .execute(&self.pool)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        }

        self.find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFoundError(format!("User {} not found", id)))
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query(r#"DELETE FROM users WHERE id = ?"#)
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn verify_email(&self, id: Uuid) -> Result<()> {
        let now = Utc::now();
        sqlx::query(
            r#"UPDATE users SET email_verified = ?, verified_at = ?, verify_token = ?, updated_at = ? WHERE id = ?"#
        )
        .bind(true)
        .bind(Some(now.to_rfc3339()))
        .bind(None::<String>)
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn set_verify_token(&self, id: Uuid, token: &str) -> Result<()> {
        sqlx::query(r#"UPDATE users SET verify_token = ? WHERE id = ?"#)
            .bind(token)
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn find_by_verify_token(&self, token: &str) -> Result<Option<User>> {
        let row = sqlx::query_as::<_, UserRow>(r#"SELECT * FROM users WHERE verify_token = ?"#)
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
        sqlx::query(r#"UPDATE users SET reset_token = ?, reset_expires_at = ? WHERE id = ?"#)
            .bind(token)
            .bind(expires_at.to_rfc3339())
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn find_by_reset_token(&self, token: &str) -> Result<Option<User>> {
        let row = sqlx::query_as::<_, UserRow>(r#"SELECT * FROM users WHERE reset_token = ?"#)
            .bind(token)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| r.into()))
    }

    async fn clear_reset_token(&self, id: Uuid) -> Result<()> {
        sqlx::query(r#"UPDATE users SET reset_token = ?, reset_expires_at = ? WHERE id = ?"#)
            .bind(None::<String>)
            .bind(None::<String>)
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn update_password(&self, id: Uuid, password_hash: &str) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(r#"UPDATE users SET password_hash = ?, updated_at = ? WHERE id = ?"#)
            .bind(password_hash)
            .bind(now)
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct UserRow {
    id: String,
    tenant_id: String,
    username: String,
    email: String,
    password_hash: String,
    role: String,
    tenant_role: String,
    email_verified: bool,
    verify_token: Option<String>,
    verified_at: Option<String>,
    reset_token: Option<String>,
    reset_expires_at: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        User {
            id: row.id.parse().unwrap_or_default(),
            tenant_id: row.tenant_id.parse().unwrap_or_default(),
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
            verified_at: row.verified_at.and_then(|s| {
                DateTime::parse_from_rfc3339(&s)
                    .ok()
                    .map(|dt| dt.with_timezone(&Utc))
            }),
            reset_token: row.reset_token,
            reset_expires_at: row.reset_expires_at.and_then(|s| {
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
