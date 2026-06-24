use async_trait::async_trait;
use chrono::{DateTime, Utc};
use common::error::{AppError, Result};
use domain::git_repo::{GitRepo, NewGitRepo, RepoVisibility, UpdateGitRepo};
use domain::repository::GitRepoRepository;
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct SqliteGitRepoRepository {
    pool: SqlitePool,
}

impl SqliteGitRepoRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

fn row_to_git_repo(row: &sqlx::sqlite::SqliteRow) -> std::result::Result<GitRepo, sqlx::Error> {
    use sqlx::Row;
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
    let auto_merge_int: i64 = row.get("auto_merge");
    let require_review_int: i64 = row.get("require_review");

    let id_str: String = row.get("id");
    let tenant_id_str: String = row.get("tenant_id");
    let created_at_str: String = row.get("created_at");
    let updated_at_str: String = row.get("updated_at");

    let last_committed_at: Option<String> = row.get("last_committed_at");
    let last_committed_at = match last_committed_at {
        Some(s) => Some(
            DateTime::parse_from_rfc3339(&s)
                .map_err(|e| {
                    sqlx::Error::Decode(Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Failed to parse last_committed_at: {}", e),
                    )))
                })?
                .with_timezone(&Utc),
        ),
        None => None,
    };

    Ok(GitRepo {
        id: id_str.parse().unwrap_or_default(),
        tenant_id: tenant_id_str.parse().unwrap_or_default(),
        name: row.get("name"),
        description: row.get("description"),
        default_branch: row.get("default_branch"),
        storage_path: row.get("storage_path"),
        visibility,
        auto_merge: auto_merge_int != 0,
        require_review: require_review_int != 0,
        last_commit_sha: row.get("last_commit_sha"),
        last_committed_at,
        created_at: DateTime::parse_from_rfc3339(&created_at_str)
            .map_err(|e| {
                sqlx::Error::Decode(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Failed to parse created_at: {}", e),
                )))
            })?
            .with_timezone(&Utc),
        updated_at: DateTime::parse_from_rfc3339(&updated_at_str)
            .map_err(|e| {
                sqlx::Error::Decode(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Failed to parse updated_at: {}", e),
                )))
            })?
            .with_timezone(&Utc),
    })
}

#[async_trait]
impl GitRepoRepository for SqliteGitRepoRepository {
    async fn create(&self, repo: NewGitRepo, tenant_id: Uuid) -> Result<GitRepo> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let visibility = repo.visibility.unwrap_or_default();
        let visibility_str = match &visibility {
            RepoVisibility::Public => "public",
            RepoVisibility::Private => "private",
        };
        let storage_path = format!("repos/{}/{}", tenant_id, &repo.name);
        let auto_merge = if repo.auto_merge.unwrap_or(true) { 1 } else { 0 };
        let require_review = if repo.require_review.unwrap_or(false) { 1 } else { 0 };

        sqlx::query(
            r#"INSERT INTO git_repos (
                id, tenant_id, name, description, default_branch, storage_path,
                visibility, auto_merge, require_review, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(id.to_string())
        .bind(tenant_id.to_string())
        .bind(&repo.name)
        .bind(repo.description.as_deref().unwrap_or(""))
        .bind(repo.default_branch.as_deref().unwrap_or("main"))
        .bind(&storage_path)
        .bind(visibility_str)
        .bind(auto_merge)
        .bind(require_review)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to create git repo: {}", e)))?;

        self.find_by_id(id)
            .await?
            .ok_or_else(|| AppError::DatabaseError("Created git repo not found".to_string()))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<GitRepo>> {
        sqlx::query("SELECT * FROM git_repos WHERE id = ?")
            .bind(id.to_string())
            .try_map(|row: sqlx::sqlite::SqliteRow| row_to_git_repo(&row))
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to find git repo: {}", e)))
    }

    async fn find_by_name(&self, tenant_id: Uuid, name: &str) -> Result<Option<GitRepo>> {
        sqlx::query("SELECT * FROM git_repos WHERE tenant_id = ? AND name = ?")
            .bind(tenant_id.to_string())
            .bind(name)
            .try_map(|row: sqlx::sqlite::SqliteRow| row_to_git_repo(&row))
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to find git repo by name: {}", e)))
    }

    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<GitRepo>> {
        sqlx::query("SELECT * FROM git_repos WHERE tenant_id = ? ORDER BY name")
            .bind(tenant_id.to_string())
            .try_map(|row: sqlx::sqlite::SqliteRow| row_to_git_repo(&row))
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
        let auto_merge = if repo.auto_merge.unwrap_or(existing.auto_merge) {
            1
        } else {
            0
        };
        let require_review = if repo.require_review.unwrap_or(existing.require_review) {
            1
        } else {
            0
        };
        let now = Utc::now();

        sqlx::query(
            r#"UPDATE git_repos SET
                name = ?, description = ?, default_branch = ?,
                visibility = ?, auto_merge = ?, require_review = ?,
                updated_at = ?
            WHERE id = ?"#,
        )
        .bind(&name)
        .bind(&description)
        .bind(&default_branch)
        .bind(visibility_str)
        .bind(auto_merge)
        .bind(require_review)
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to update git repo: {}", e)))?;

        self.find_by_id(id)
            .await?
            .ok_or_else(|| AppError::DatabaseError("Updated git repo not found".to_string()))
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM git_repos WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to delete git repo: {}", e)))?;
        Ok(())
    }

    async fn update_last_commit(
        &self,
        id: Uuid,
        sha: &str,
        committed_at: DateTime<Utc>,
    ) -> Result<()> {
        sqlx::query(
            r#"UPDATE git_repos SET last_commit_sha = ?, last_committed_at = ?, updated_at = ? WHERE id = ?"#,
        )
        .bind(sha)
        .bind(committed_at.to_rfc3339())
        .bind(Utc::now().to_rfc3339())
        .bind(id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| {
            AppError::DatabaseError(format!("Failed to update last commit: {}", e))
        })?;
        Ok(())
    }
}
