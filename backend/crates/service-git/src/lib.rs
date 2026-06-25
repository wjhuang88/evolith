use std::path::{Path, PathBuf};

use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum GitStorageError {
    #[error("Failed to initialize bare repo: {0}")]
    InitError(String),
    #[error("Failed to write seed file {path}: {source}")]
    SeedWriteError {
        path: String,
        source: std::io::Error,
    },
    #[error("Failed to remove repo {path}: {source}")]
    RemoveError {
        path: String,
        source: std::io::Error,
    },
    #[error("Base path not configured")]
    NoBasePath,
}

pub type Result<T> = std::result::Result<T, GitStorageError>;

pub fn repo_path(base_path: &Path, tenant_id: Uuid, repo_id: Uuid) -> PathBuf {
    base_path
        .join(tenant_id.to_string())
        .join(format!("{}.git", repo_id))
}

pub fn init_bare_repo(base_path: &Path, tenant_id: Uuid, repo_id: Uuid) -> Result<PathBuf> {
    let path = repo_path(base_path, tenant_id, repo_id);

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            GitStorageError::InitError(format!(
                "Failed to create parent directory {}: {}",
                parent.display(),
                e
            ))
        })?;
    }

    gix::init_bare(&path).map_err(|e| {
        GitStorageError::InitError(format!(
            "gix::init_bare failed for {}: {}",
            path.display(),
            e
        ))
    })?;

    Ok(path)
}

pub fn write_seed_files(storage_path: &Path, repo_name: &str, default_branch: &str) -> Result<()> {
    let readme_path = storage_path.join("README.md");
    let readme_content = format!("# {}\n\nDefault branch: `{}`\n", repo_name, default_branch);

    std::fs::write(&readme_path, &readme_content).map_err(|e| GitStorageError::SeedWriteError {
        path: readme_path.display().to_string(),
        source: e,
    })?;

    let evolith_dir = storage_path.join(".evolith");
    std::fs::create_dir_all(&evolith_dir).map_err(|e| GitStorageError::SeedWriteError {
        path: evolith_dir.display().to_string(),
        source: e,
    })?;

    let policy_path = evolith_dir.join("policy.yaml");
    let policy_content = "default_action: require_review\n";

    std::fs::write(&policy_path, policy_content).map_err(|e| GitStorageError::SeedWriteError {
        path: policy_path.display().to_string(),
        source: e,
    })?;

    let agents_path = evolith_dir.join("agents.yaml");
    let agents_content = "agents: []\n";

    std::fs::write(&agents_path, agents_content).map_err(|e| GitStorageError::SeedWriteError {
        path: agents_path.display().to_string(),
        source: e,
    })?;

    Ok(())
}

pub fn remove_repo(storage_path: &Path) -> Result<()> {
    std::fs::remove_dir_all(storage_path).map_err(|e| GitStorageError::RemoveError {
        path: storage_path.display().to_string(),
        source: e,
    })
}
