use async_trait::async_trait;
use chrono::{DateTime, Utc};
use common::error::{AppError, Result};
use domain::git_repo::{GitRepo, NewGitRepo, RepoLifecycleStatus, RepoVisibility, UpdateGitRepo};
use domain::repository::GitRepoRepository;
use sqlx::PgPool;
use sqlx::Row;
use uuid::Uuid;

pub struct PgGitRepoRepository {
    pool: PgPool,
}

impl PgGitRepoRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

fn row_to_git_repo(row: &sqlx::postgres::PgRow) -> std::result::Result<GitRepo, sqlx::Error> {
    let visibility_str: String = row.get("visibility");
    let visibility = match visibility_str.as_str() {
        "public" => RepoVisibility::Public,
        "private" => RepoVisibility::Private,
        _ => {
            return Err(sqlx::Error::Decode(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Unknown visibility: {}", visibility_str),
            ))));
        }
    };
    let lifecycle_status = match row.get::<String, _>("lifecycle_status").as_str() {
        "CREATING" => RepoLifecycleStatus::Creating,
        "ACTIVE" => RepoLifecycleStatus::Active,
        "ERROR" => RepoLifecycleStatus::Error,
        "DELETING" => RepoLifecycleStatus::Deleting,
        value => {
            return Err(sqlx::Error::Decode(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Unknown lifecycle status: {value}"),
            ))))
        }
    };

    Ok(GitRepo {
        id: row.get("id"),
        tenant_id: row.get("tenant_id"),
        name: row.get("name"),
        description: row.get("description"),
        default_branch: row.get("default_branch"),
        storage_path: row.get("storage_path"),
        visibility,
        auto_merge: row.get("auto_merge"),
        require_review: row.get("require_review"),
        lifecycle_status,
        last_commit_sha: row.get("last_commit_sha"),
        last_committed_at: row.get("last_committed_at"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

#[async_trait]
impl GitRepoRepository for PgGitRepoRepository {
    async fn create(&self, repo: NewGitRepo, tenant_id: Uuid) -> Result<GitRepo> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let visibility = repo.visibility.unwrap_or_default();
        let visibility_str = match &visibility {
            RepoVisibility::Public => "public",
            RepoVisibility::Private => "private",
        };
        let storage_path = format!("{}/{}.git", tenant_id, id);

        sqlx::query(
            r#"INSERT INTO git_repos (
                id, tenant_id, name, description, default_branch, storage_path,
                visibility, auto_merge, require_review, created_at, updated_at,
                lifecycle_status
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)"#,
        )
        .bind(id)
        .bind(tenant_id)
        .bind(&repo.name)
        .bind(repo.description.as_deref().unwrap_or(""))
        .bind(repo.default_branch.as_deref().unwrap_or("main"))
        .bind(&storage_path)
        .bind(visibility_str)
        .bind(repo.auto_merge.unwrap_or(false))
        .bind(repo.require_review.unwrap_or(true))
        .bind(now)
        .bind(now)
        .bind("CREATING")
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to create git repo: {}", e)))?;

        self.find_by_id(id)
            .await?
            .ok_or_else(|| AppError::DatabaseError("Created git repo not found".to_string()))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<GitRepo>> {
        sqlx::query("SELECT * FROM git_repos WHERE id = $1")
            .bind(id)
            .try_map(|row: sqlx::postgres::PgRow| row_to_git_repo(&row))
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to find git repo: {}", e)))
    }

    async fn find_by_name(&self, tenant_id: Uuid, name: &str) -> Result<Option<GitRepo>> {
        sqlx::query("SELECT * FROM git_repos WHERE tenant_id = $1 AND name = $2")
            .bind(tenant_id)
            .bind(name)
            .try_map(|row: sqlx::postgres::PgRow| row_to_git_repo(&row))
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to find git repo by name: {}", e)))
    }

    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<GitRepo>> {
        sqlx::query("SELECT * FROM git_repos WHERE tenant_id = $1 ORDER BY name")
            .bind(tenant_id)
            .try_map(|row: sqlx::postgres::PgRow| row_to_git_repo(&row))
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                AppError::DatabaseError(format!("Failed to find git repos by tenant: {}", e))
            })
    }

    async fn update(&self, id: Uuid, repo: UpdateGitRepo) -> Result<GitRepo> {
        let existing = self
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFoundError("Git repo not found".to_string()))?;

        let name = repo.name.unwrap_or(existing.name);
        let description = repo.description.unwrap_or(existing.description);
        let default_branch = repo.default_branch.unwrap_or(existing.default_branch);
        let visibility = repo.visibility.unwrap_or(existing.visibility);
        let visibility_str = match &visibility {
            RepoVisibility::Public => "public",
            RepoVisibility::Private => "private",
        };
        let now = Utc::now();

        sqlx::query(
            r#"UPDATE git_repos SET
                name = $1, description = $2, default_branch = $3,
                visibility = $4, auto_merge = $5, require_review = $6,
                updated_at = $7
            WHERE id = $8"#,
        )
        .bind(&name)
        .bind(&description)
        .bind(&default_branch)
        .bind(visibility_str)
        .bind(repo.auto_merge.unwrap_or(existing.auto_merge))
        .bind(repo.require_review.unwrap_or(existing.require_review))
        .bind(now)
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to update git repo: {}", e)))?;

        self.find_by_id(id)
            .await?
            .ok_or_else(|| AppError::DatabaseError("Updated git repo not found".to_string()))
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM git_repos WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to delete git repo: {}", e)))?;
        Ok(())
    }

    async fn update_lifecycle_status(
        &self,
        id: Uuid,
        status: RepoLifecycleStatus,
    ) -> Result<GitRepo> {
        let value = match status {
            RepoLifecycleStatus::Creating => "CREATING",
            RepoLifecycleStatus::Active => "ACTIVE",
            RepoLifecycleStatus::Error => "ERROR",
            RepoLifecycleStatus::Deleting => "DELETING",
        };
        sqlx::query("UPDATE git_repos SET lifecycle_status = $1, updated_at = $2 WHERE id = $3")
            .bind(value)
            .bind(Utc::now())
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                AppError::DatabaseError(format!("Failed to update repo lifecycle: {e}"))
            })?;
        self.find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFoundError("Git repo not found".to_string()))
    }

    async fn update_last_commit(
        &self,
        id: Uuid,
        sha: &str,
        committed_at: DateTime<Utc>,
    ) -> Result<()> {
        sqlx::query(
            r#"UPDATE git_repos SET last_commit_sha = $1, last_committed_at = $2, updated_at = $3 WHERE id = $4"#,
        )
        .bind(sha)
        .bind(committed_at)
        .bind(Utc::now())
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            AppError::DatabaseError(format!("Failed to update last commit: {}", e))
        })?;
        Ok(())
    }
}
