use std::path::{Path, PathBuf};
use std::sync::Arc;

use async_trait::async_trait;
use chrono::{TimeZone, Utc};
use common::error::{AppError, Result};
use domain::repository::{GitRepoRepository, OutboxRepository};
use domain::{
    OutboxEvent, OutboxHandler, RepoPushCompletedPayload, REPO_PUSH_COMPLETED_EVENT_TYPE,
};
use uuid::Uuid;

pub async fn enqueue_repo_push_completed(
    repository: &dyn OutboxRepository,
    tenant_id: Uuid,
    repo_id: Uuid,
    default_branch: &str,
    repo_path: &Path,
) -> Result<OutboxEvent> {
    let (commit_sha, committed_at) = resolve_default_branch(repo_path, default_branch)
        .await?
        .ok_or_else(|| AppError::StorageError("push default branch is unavailable".to_string()))?;
    let payload = RepoPushCompletedPayload::new(
        tenant_id,
        repo_id,
        default_branch.to_string(),
        commit_sha,
        committed_at,
    )?;
    repository
        .enqueue_idempotent(payload.into_outbox_event()?)
        .await
}

pub async fn reconcile_repo_push_completed(
    repository: &dyn OutboxRepository,
    tenant_id: Uuid,
    repo_id: Uuid,
    default_branch: &str,
    repo_path: &Path,
) -> Result<Option<OutboxEvent>> {
    let Some((commit_sha, committed_at)) =
        resolve_default_branch(repo_path, default_branch).await?
    else {
        return Ok(None);
    };
    let payload = RepoPushCompletedPayload::new(
        tenant_id,
        repo_id,
        default_branch.to_string(),
        commit_sha,
        committed_at,
    )?;
    repository
        .enqueue_idempotent(payload.into_outbox_event()?)
        .await
        .map(Some)
}

pub struct RepoPushEventHandler {
    git_repo_repo: Arc<dyn GitRepoRepository>,
    git_storage_base_path: PathBuf,
}

impl RepoPushEventHandler {
    pub fn new(
        git_repo_repo: Arc<dyn GitRepoRepository>,
        git_storage_base_path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            git_repo_repo,
            git_storage_base_path: git_storage_base_path.into(),
        }
    }

    pub async fn deliver_repo_push(&self, event: &OutboxEvent) -> Result<()> {
        let payload = RepoPushCompletedPayload::from_outbox_event(event)?;
        let repo = self
            .git_repo_repo
            .find_by_id(payload.repo_id)
            .await?
            .ok_or_else(|| AppError::NotFoundError("push event repo not found".to_string()))?;
        if repo.tenant_id != payload.tenant_id {
            return Err(AppError::AuthorizationError(
                "push event repo identity mismatch".to_string(),
            ));
        }
        if repo.default_branch != payload.default_branch {
            tracing::info!(
                event_id = %event.id,
                repo_id = %repo.id,
                "push event for former default branch skipped"
            );
            return Ok(());
        }

        let repo_path =
            service_git::repo_path(&self.git_storage_base_path, repo.tenant_id, repo.id);
        let (current_sha, committed_at) = resolve_default_branch(&repo_path, &repo.default_branch)
            .await?
            .ok_or_else(|| {
                AppError::StorageError("push default branch is unavailable".to_string())
            })?;
        if current_sha != payload.commit_sha {
            tracing::info!(
                event_id = %event.id,
                repo_id = %repo.id,
                "stale repo push event skipped"
            );
            return Ok(());
        }

        self.git_repo_repo
            .update_last_commit(repo.id, &current_sha, committed_at)
            .await?;
        tracing::info!(
            event_id = %event.id,
            repo_id = %repo.id,
            commit_sha = %short_sha(&current_sha),
            "repo metadata refreshed from durable push event"
        );
        Ok(())
    }
}

#[async_trait]
impl OutboxHandler for RepoPushEventHandler {
    async fn deliver(&self, event: &OutboxEvent) -> Result<()> {
        if event.event_type != REPO_PUSH_COMPLETED_EVENT_TYPE {
            return Err(AppError::ServiceUnavailableError(
                "no registered outbox handler".to_string(),
            ));
        }
        self.deliver_repo_push(event).await
    }
}

async fn resolve_default_branch(
    repo_path: &Path,
    default_branch: &str,
) -> Result<Option<(String, chrono::DateTime<Utc>)>> {
    let repo_path = repo_path.to_path_buf();
    let ref_name = format!("refs/heads/{default_branch}");
    let task = tokio::task::spawn_blocking(move || service_git::resolve_ref(&repo_path, &ref_name));
    let resolved = tokio::time::timeout(service_git::CONTEXT_BLOCKING_TIMEOUT, task)
        .await
        .map_err(|_| {
            AppError::ServiceUnavailableError("push ref resolution timed out".to_string())
        })?
        .map_err(|_| AppError::InternalError("push ref resolution task failed".to_string()))?;
    let resolved = match resolved {
        Ok(resolved) => resolved,
        Err(service_git::GitStorageError::NotFound(_)) => return Ok(None),
        Err(_) => {
            return Err(AppError::StorageError(
                "push default branch is unavailable".to_string(),
            ))
        }
    };
    let committed_at = Utc
        .timestamp_opt(resolved.1, 0)
        .single()
        .ok_or_else(|| AppError::ValidationError("invalid push commit timestamp".to_string()))?;
    Ok(Some((resolved.0, committed_at)))
}

fn short_sha(sha: &str) -> &str {
    &sha[..sha.len().min(12)]
}
