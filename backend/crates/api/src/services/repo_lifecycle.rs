use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use chrono::Utc;
use domain::git_repo::{GitRepo, NewGitRepo, RepoLifecycleStatus};
use domain::repository::GitRepoRepository;
use serde::Serialize;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum RepoLifecycleError {
    #[error("repo metadata operation failed: {0}")]
    Metadata(String),
    #[error("repo storage initialization failed: {0}")]
    StorageInit(String),
    #[error("repo seed failed: {0}")]
    Seed(String),
    #[error("repo activation failed: {0}")]
    Activation(String),
    #[error("repo storage deletion failed: {0}")]
    StorageDelete(String),
    #[error("repo metadata deletion failed: {0}")]
    MetadataDelete(String),
    #[error("repo inventory failed: {0}")]
    Inventory(String),
    #[error("repo reconciliation failed: {0}")]
    Reconcile(String),
}

pub trait RepoStorage: Send + Sync {
    fn init(&self, tenant_id: Uuid, repo_id: Uuid) -> Result<PathBuf, String>;
    fn seed(&self, path: &Path, repo_name: &str, branch: &str) -> Result<String, String>;
    fn remove(&self, tenant_id: Uuid, repo_id: Uuid) -> Result<(), String>;
    fn inventory(&self, tenant_id: Uuid) -> Result<HashSet<Uuid>, String>;
    fn quarantine(&self, tenant_id: Uuid, repo_id: Uuid) -> Result<String, String>;
}

pub struct FileSystemRepoStorage {
    base_path: PathBuf,
}

impl FileSystemRepoStorage {
    pub fn new(base_path: impl Into<PathBuf>) -> Self {
        Self {
            base_path: base_path.into(),
        }
    }
}

impl RepoStorage for FileSystemRepoStorage {
    fn init(&self, tenant_id: Uuid, repo_id: Uuid) -> Result<PathBuf, String> {
        service_git::init_bare_repo(&self.base_path, tenant_id, repo_id)
            .map_err(|error| error.to_string())
    }

    fn seed(&self, path: &Path, repo_name: &str, branch: &str) -> Result<String, String> {
        service_git::seed_initial_commit(path, repo_name, branch).map_err(|error| error.to_string())
    }

    fn remove(&self, tenant_id: Uuid, repo_id: Uuid) -> Result<(), String> {
        service_git::remove_repo(&service_git::repo_path(&self.base_path, tenant_id, repo_id))
            .map_err(|error| error.to_string())
    }

    fn inventory(&self, tenant_id: Uuid) -> Result<HashSet<Uuid>, String> {
        service_git::inventory_tenant_repos(&self.base_path, tenant_id)
            .map_err(|error| error.to_string())
    }

    fn quarantine(&self, tenant_id: Uuid, repo_id: Uuid) -> Result<String, String> {
        let destination = service_git::quarantine_repo(&self.base_path, tenant_id, repo_id)
            .map_err(|error| error.to_string())?;
        destination
            .strip_prefix(&self.base_path)
            .map(|path| path.to_string_lossy().to_string())
            .map_err(|_| "quarantine destination escaped configured storage root".to_string())
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReconcileClassification {
    Consistent,
    DbOnly,
    DiskOnly,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReconcileAction {
    None,
    MarkedError,
    Quarantined,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReconcileEntry {
    pub repo_id: Uuid,
    pub classification: ReconcileClassification,
    pub action: ReconcileAction,
    pub quarantine_path: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReconcileReport {
    pub apply: bool,
    pub consistent: usize,
    pub db_only: usize,
    pub disk_only: usize,
    pub entries: Vec<ReconcileEntry>,
}

pub struct RepoLifecycleService {
    repository: Arc<dyn GitRepoRepository>,
    storage: Arc<dyn RepoStorage>,
}

impl RepoLifecycleService {
    pub fn new(repository: Arc<dyn GitRepoRepository>, storage: Arc<dyn RepoStorage>) -> Self {
        Self {
            repository,
            storage,
        }
    }

    pub async fn create(
        &self,
        tenant_id: Uuid,
        new_repo: NewGitRepo,
        seed_template: bool,
    ) -> Result<GitRepo, RepoLifecycleError> {
        let repo = self
            .repository
            .create(new_repo, tenant_id)
            .await
            .map_err(|error| RepoLifecycleError::Metadata(error.to_string()))?;

        let storage_path = match self.storage.init(tenant_id, repo.id) {
            Ok(path) => path,
            Err(error) => {
                self.mark_error(repo.id).await;
                return Err(RepoLifecycleError::StorageInit(error));
            }
        };

        if seed_template {
            let sha = match self
                .storage
                .seed(&storage_path, &repo.name, &repo.default_branch)
            {
                Ok(sha) => sha,
                Err(error) => {
                    self.mark_error(repo.id).await;
                    return Err(RepoLifecycleError::Seed(error));
                }
            };
            if let Err(error) = self
                .repository
                .update_last_commit(repo.id, &sha, Utc::now())
                .await
            {
                self.mark_error(repo.id).await;
                return Err(RepoLifecycleError::Metadata(error.to_string()));
            }
        }

        match self
            .repository
            .update_lifecycle_status(repo.id, RepoLifecycleStatus::Active)
            .await
        {
            Ok(repo) => Ok(repo),
            Err(error) => {
                self.mark_error(repo.id).await;
                Err(RepoLifecycleError::Activation(error.to_string()))
            }
        }
    }

    pub async fn delete(&self, repo: &GitRepo) -> Result<(), RepoLifecycleError> {
        self.repository
            .update_lifecycle_status(repo.id, RepoLifecycleStatus::Deleting)
            .await
            .map_err(|error| RepoLifecycleError::Metadata(error.to_string()))?;

        if let Err(error) = self.storage.remove(repo.tenant_id, repo.id) {
            self.mark_error(repo.id).await;
            return Err(RepoLifecycleError::StorageDelete(error));
        }

        if let Err(error) = self.repository.delete(repo.id).await {
            self.mark_error(repo.id).await;
            return Err(RepoLifecycleError::MetadataDelete(error.to_string()));
        }
        Ok(())
    }

    pub async fn reconcile(
        &self,
        tenant_id: Uuid,
        apply: bool,
    ) -> Result<ReconcileReport, RepoLifecycleError> {
        let db_repos = self
            .repository
            .find_by_tenant(tenant_id)
            .await
            .map_err(|error| RepoLifecycleError::Inventory(error.to_string()))?;
        let disk_ids = self
            .storage
            .inventory(tenant_id)
            .map_err(RepoLifecycleError::Inventory)?;
        let db_by_id: HashMap<Uuid, GitRepo> =
            db_repos.into_iter().map(|repo| (repo.id, repo)).collect();

        let mut all_ids: HashSet<Uuid> = db_by_id.keys().copied().collect();
        all_ids.extend(disk_ids.iter().copied());
        let mut all_ids: Vec<Uuid> = all_ids.into_iter().collect();
        all_ids.sort_unstable();

        let mut entries = Vec::with_capacity(all_ids.len());
        let mut consistent = 0;
        let mut db_only = 0;
        let mut disk_only = 0;

        for repo_id in all_ids {
            match (db_by_id.contains_key(&repo_id), disk_ids.contains(&repo_id)) {
                (true, true) => {
                    consistent += 1;
                    entries.push(ReconcileEntry {
                        repo_id,
                        classification: ReconcileClassification::Consistent,
                        action: ReconcileAction::None,
                        quarantine_path: None,
                    });
                }
                (true, false) => {
                    db_only += 1;
                    let action = if apply {
                        self.repository
                            .update_lifecycle_status(repo_id, RepoLifecycleStatus::Error)
                            .await
                            .map_err(|error| RepoLifecycleError::Reconcile(error.to_string()))?;
                        ReconcileAction::MarkedError
                    } else {
                        ReconcileAction::None
                    };
                    entries.push(ReconcileEntry {
                        repo_id,
                        classification: ReconcileClassification::DbOnly,
                        action,
                        quarantine_path: None,
                    });
                }
                (false, true) => {
                    disk_only += 1;
                    let quarantine_path = if apply {
                        Some(
                            self.storage
                                .quarantine(tenant_id, repo_id)
                                .map_err(RepoLifecycleError::Reconcile)?,
                        )
                    } else {
                        None
                    };
                    let action = if apply {
                        ReconcileAction::Quarantined
                    } else {
                        ReconcileAction::None
                    };
                    entries.push(ReconcileEntry {
                        repo_id,
                        classification: ReconcileClassification::DiskOnly,
                        action,
                        quarantine_path,
                    });
                }
                (false, false) => unreachable!("repo id came from neither inventory"),
            }
        }

        Ok(ReconcileReport {
            apply,
            consistent,
            db_only,
            disk_only,
            entries,
        })
    }

    async fn mark_error(&self, repo_id: Uuid) {
        if let Err(error) = self
            .repository
            .update_lifecycle_status(repo_id, RepoLifecycleStatus::Error)
            .await
        {
            tracing::error!(repo_id = %repo_id, "Failed to mark repo ERROR: {}", error);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use common::error::{AppError, Result as AppResult};
    use domain::git_repo::{RepoVisibility, UpdateGitRepo};
    use std::sync::Mutex;

    type StorageResult<T> = std::result::Result<T, String>;

    #[derive(Default)]
    struct FakeStorage {
        init_error: bool,
        seed_error: bool,
        remove_error: bool,
    }

    impl RepoStorage for FakeStorage {
        fn init(&self, _tenant_id: Uuid, _repo_id: Uuid) -> StorageResult<PathBuf> {
            if self.init_error {
                Err("injected init failure".to_string())
            } else {
                Ok(PathBuf::from("/tmp/fake.git"))
            }
        }

        fn seed(&self, _path: &Path, _repo_name: &str, _branch: &str) -> StorageResult<String> {
            if self.seed_error {
                Err("injected seed failure".to_string())
            } else {
                Ok("0123456789012345678901234567890123456789".to_string())
            }
        }

        fn remove(&self, _tenant_id: Uuid, _repo_id: Uuid) -> StorageResult<()> {
            if self.remove_error {
                Err("injected remove failure".to_string())
            } else {
                Ok(())
            }
        }

        fn inventory(&self, _tenant_id: Uuid) -> StorageResult<HashSet<Uuid>> {
            Ok(HashSet::new())
        }

        fn quarantine(&self, _tenant_id: Uuid, _repo_id: Uuid) -> StorageResult<String> {
            Ok(".reconcile-trash/fake.git".to_string())
        }
    }

    struct FakeRepository {
        repos: Mutex<Vec<GitRepo>>,
    }

    impl FakeRepository {
        fn new() -> Self {
            Self {
                repos: Mutex::new(Vec::new()),
            }
        }

        fn status(&self, id: Uuid) -> RepoLifecycleStatus {
            self.repos
                .lock()
                .unwrap()
                .iter()
                .find(|repo| repo.id == id)
                .unwrap()
                .lifecycle_status
                .clone()
        }
    }

    #[async_trait]
    impl GitRepoRepository for FakeRepository {
        async fn create(&self, repo: NewGitRepo, tenant_id: Uuid) -> AppResult<GitRepo> {
            let now = Utc::now();
            let value = GitRepo {
                id: Uuid::new_v4(),
                tenant_id,
                name: repo.name,
                description: repo.description.unwrap_or_default(),
                default_branch: repo.default_branch.unwrap_or_else(|| "main".to_string()),
                storage_path: format!("{tenant_id}/fake.git"),
                visibility: repo.visibility.unwrap_or(RepoVisibility::Private),
                auto_merge: repo.auto_merge.unwrap_or(false),
                require_review: repo.require_review.unwrap_or(true),
                lifecycle_status: RepoLifecycleStatus::Creating,
                last_commit_sha: None,
                last_committed_at: None,
                created_at: now,
                updated_at: now,
            };
            self.repos.lock().unwrap().push(value.clone());
            Ok(value)
        }

        async fn find_by_id(&self, id: Uuid) -> AppResult<Option<GitRepo>> {
            Ok(self
                .repos
                .lock()
                .unwrap()
                .iter()
                .find(|repo| repo.id == id)
                .cloned())
        }

        async fn find_by_name(&self, _tenant_id: Uuid, _name: &str) -> AppResult<Option<GitRepo>> {
            Ok(None)
        }

        async fn find_by_tenant(&self, tenant_id: Uuid) -> AppResult<Vec<GitRepo>> {
            Ok(self
                .repos
                .lock()
                .unwrap()
                .iter()
                .filter(|repo| repo.tenant_id == tenant_id)
                .cloned()
                .collect())
        }

        async fn update(&self, _id: Uuid, _repo: UpdateGitRepo) -> AppResult<GitRepo> {
            Err(AppError::NotFoundError("unused".to_string()))
        }

        async fn delete(&self, id: Uuid) -> AppResult<()> {
            self.repos.lock().unwrap().retain(|repo| repo.id != id);
            Ok(())
        }

        async fn update_lifecycle_status(
            &self,
            id: Uuid,
            status: RepoLifecycleStatus,
        ) -> AppResult<GitRepo> {
            let mut repos = self.repos.lock().unwrap();
            let repo = repos
                .iter_mut()
                .find(|repo| repo.id == id)
                .ok_or_else(|| AppError::NotFoundError("unused".to_string()))?;
            repo.lifecycle_status = status;
            Ok(repo.clone())
        }

        async fn update_last_commit(
            &self,
            id: Uuid,
            sha: &str,
            committed_at: chrono::DateTime<Utc>,
        ) -> AppResult<()> {
            let mut repos = self.repos.lock().unwrap();
            let repo = repos
                .iter_mut()
                .find(|repo| repo.id == id)
                .ok_or_else(|| AppError::NotFoundError("unused".to_string()))?;
            repo.last_commit_sha = Some(sha.to_string());
            repo.last_committed_at = Some(committed_at);
            Ok(())
        }
    }

    #[tokio::test]
    async fn create_storage_failure_marks_repo_error() {
        let repository = Arc::new(FakeRepository::new());
        let service = RepoLifecycleService::new(
            repository.clone(),
            Arc::new(FakeStorage {
                init_error: true,
                ..Default::default()
            }),
        );
        let error = service
            .create(
                Uuid::new_v4(),
                NewGitRepo {
                    name: "failure".to_string(),
                    description: None,
                    default_branch: None,
                    visibility: None,
                    auto_merge: None,
                    require_review: None,
                },
                false,
            )
            .await
            .expect_err("injected init failure should fail create");
        assert!(matches!(error, RepoLifecycleError::StorageInit(_)));
        let repo_id = repository.repos.lock().unwrap()[0].id;
        assert_eq!(repository.status(repo_id), RepoLifecycleStatus::Error);
    }

    #[tokio::test]
    async fn seed_failure_marks_repo_error() {
        let repository = Arc::new(FakeRepository::new());
        let service = RepoLifecycleService::new(
            repository.clone(),
            Arc::new(FakeStorage {
                seed_error: true,
                ..Default::default()
            }),
        );
        let error = service
            .create(
                Uuid::new_v4(),
                NewGitRepo {
                    name: "seed-failure".to_string(),
                    description: None,
                    default_branch: None,
                    visibility: None,
                    auto_merge: None,
                    require_review: None,
                },
                true,
            )
            .await
            .expect_err("injected seed failure should fail create");
        assert!(matches!(error, RepoLifecycleError::Seed(_)));
        let repo_id = repository.repos.lock().unwrap()[0].id;
        assert_eq!(repository.status(repo_id), RepoLifecycleStatus::Error);
    }

    #[tokio::test]
    async fn delete_storage_failure_retains_error_record() {
        let repository = Arc::new(FakeRepository::new());
        let service = RepoLifecycleService::new(
            repository.clone(),
            Arc::new(FakeStorage {
                remove_error: true,
                ..Default::default()
            }),
        );
        let repo = service
            .create(
                Uuid::new_v4(),
                NewGitRepo {
                    name: "delete-failure".to_string(),
                    description: None,
                    default_branch: None,
                    visibility: None,
                    auto_merge: None,
                    require_review: None,
                },
                false,
            )
            .await
            .unwrap();
        let error = service
            .delete(&repo)
            .await
            .expect_err("injected remove failure should fail delete");
        assert!(matches!(error, RepoLifecycleError::StorageDelete(_)));
        assert_eq!(repository.status(repo.id), RepoLifecycleStatus::Error);
    }
}
