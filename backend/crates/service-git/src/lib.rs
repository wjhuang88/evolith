use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::Duration;

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
    #[error("Git subprocess error: {0}")]
    SubprocessError(String),
    #[error("Invalid git service: {0}")]
    InvalidService(String),
}

pub type Result<T> = std::result::Result<T, GitStorageError>;

/// Upper bound for a single git subprocess operation.
///
/// This is intentionally fixed at the git-service boundary so Smart HTTP cannot leave
/// unbounded `git` children behind if a client stalls or the repository is pathological.
#[cfg(not(test))]
pub const GIT_SUBPROCESS_TIMEOUT: Duration = Duration::from_secs(30);

#[cfg(test)]
pub const GIT_SUBPROCESS_TIMEOUT: Duration = Duration::from_millis(50);

/// Smart HTTP git service types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GitService {
    UploadPack,
    ReceivePack,
}

impl GitService {
    pub fn parse_service(s: &str) -> Option<Self> {
        match s {
            "git-upload-pack" => Some(GitService::UploadPack),
            "git-receive-pack" => Some(GitService::ReceivePack),
            _ => None,
        }
    }

    pub fn command_name(&self) -> &'static str {
        match self {
            GitService::UploadPack => "upload-pack",
            GitService::ReceivePack => "receive-pack",
        }
    }

    pub fn service_name(&self) -> &'static str {
        match self {
            GitService::UploadPack => "git-upload-pack",
            GitService::ReceivePack => "git-receive-pack",
        }
    }

    pub fn content_type_advertisement(&self) -> &'static str {
        match self {
            GitService::UploadPack => "application/x-git-upload-pack-advertisement",
            GitService::ReceivePack => "application/x-git-receive-pack-advertisement",
        }
    }

    pub fn content_type_result(&self) -> &'static str {
        match self {
            GitService::UploadPack => "application/x-git-upload-pack-result",
            GitService::ReceivePack => "application/x-git-receive-pack-result",
        }
    }
}

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

/// Run `git <service> --advertise-refs --stateless-rpc <repo_path>` and return
/// the output with Smart HTTP pkt-line prefix prepended.
pub async fn advertise_refs(repo_path: &Path, service: GitService) -> Result<Vec<u8>> {
    let child = tokio::process::Command::new("git")
        .args([
            service.command_name(),
            "--advertise-refs",
            "--stateless-rpc",
            repo_path.to_str().ok_or_else(|| {
                GitStorageError::SubprocessError("repo_path contains invalid UTF-8".into())
            })?,
        ])
        .kill_on_drop(true)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| GitStorageError::SubprocessError(format!("git subprocess failed: {e}")))?;

    let output = match tokio::time::timeout(GIT_SUBPROCESS_TIMEOUT, child.wait_with_output()).await
    {
        Ok(result) => result
            .map_err(|e| GitStorageError::SubprocessError(format!("git subprocess failed: {e}")))?,
        Err(_) => {
            return Err(GitStorageError::SubprocessError(format!(
                "git {} timed out after {}s",
                service.command_name(),
                GIT_SUBPROCESS_TIMEOUT.as_secs()
            )));
        }
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(GitStorageError::SubprocessError(format!(
            "git {} exited with status {}: {}",
            service.command_name(),
            output.status,
            stderr.trim()
        )));
    }

    let mut result = Vec::new();

    let svc_line = format!("# service={}\n", service.service_name());
    let pkt_len = 4 + svc_line.len();
    result.extend(format!("{:04x}{}", pkt_len, svc_line).as_bytes());
    result.extend(b"0000");

    result.extend(output.stdout);

    Ok(result)
}

/// Spawn `git <service> --stateless-rpc <repo_path>` and return the `Child` handle.
/// The caller pipes request body into `child.stdin` and streams `child.stdout` to the response.
pub async fn spawn_rpc(repo_path: &Path, service: GitService) -> Result<tokio::process::Child> {
    let child = tokio::process::Command::new("git")
        .args([
            service.command_name(),
            "--stateless-rpc",
            repo_path.to_str().ok_or_else(|| {
                GitStorageError::SubprocessError("repo_path contains invalid UTF-8".into())
            })?,
        ])
        .kill_on_drop(true)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| GitStorageError::SubprocessError(format!("git subprocess failed: {e}")))?;

    Ok(child)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use std::ffi::OsString;
    use tokio::sync::Mutex;

    static PATH_LOCK: Mutex<()> = Mutex::const_new(());

    #[tokio::test]
    #[cfg(unix)]
    async fn advertise_refs_times_out_slow_git_process() {
        use std::os::unix::fs::PermissionsExt;

        let _guard = PATH_LOCK.lock().await;
        let test_dir =
            std::env::temp_dir().join(format!("evolith-fake-git-{}", uuid::Uuid::new_v4()));
        let bin_dir = test_dir.join("bin");
        std::fs::create_dir_all(&bin_dir).unwrap();

        let fake_git = bin_dir.join("git");
        std::fs::write(&fake_git, "#!/bin/sh\nsleep 1\n").unwrap();
        let mut permissions = std::fs::metadata(&fake_git).unwrap().permissions();
        permissions.set_mode(0o755);
        std::fs::set_permissions(&fake_git, permissions).unwrap();

        let original_path = std::env::var_os("PATH");
        let mut new_path = OsString::from(&bin_dir);
        new_path.push(":");
        new_path.push(original_path.clone().unwrap_or_default());
        std::env::set_var("PATH", new_path);

        let result =
            advertise_refs(Path::new("/tmp/nonexistent.git"), GitService::UploadPack).await;

        if let Some(path) = original_path {
            std::env::set_var("PATH", path);
        } else {
            std::env::remove_var("PATH");
        }
        let _ = std::fs::remove_dir_all(test_dir);

        let err = result.expect_err("slow fake git should time out");
        assert!(
            err.to_string().contains("timed out"),
            "unexpected error: {}",
            err
        );
    }
}
