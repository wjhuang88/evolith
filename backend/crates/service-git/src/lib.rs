use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::Duration;

use gix::bstr::BStr;
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
    #[error("Read error: {0}")]
    ReadError(String),
}

/// Entry in a file tree listing.
#[derive(Debug, Clone)]
pub struct FileTreeEntry {
    pub name: String,
    pub kind: String, // "blob" or "tree"
    pub oid: String,
    pub is_tree: bool,
}

/// Information about a single commit.
#[derive(Debug, Clone)]
pub struct CommitInfo {
    pub sha: String,
    pub author_name: String,
    pub author_email: String,
    pub message: String,
    pub timestamp: i64,
}

/// A single change in a diff between two refs.
#[derive(Debug, Clone)]
pub struct DiffEntry {
    pub path: String,
    pub change_type: String, // "added", "deleted", "modified"
    pub old_oid: String,
    pub new_oid: String,
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

/// Read the file tree at a given git ref (recursive, breadth-first).
pub fn read_file_tree(repo_path: &Path, git_ref: &str) -> Result<Vec<FileTreeEntry>> {
    let repo = gix::open(repo_path).map_err(|e| {
        GitStorageError::ReadError(format!(
            "failed to open repo at {}: {}",
            repo_path.display(),
            e
        ))
    })?;

    let id = repo
        .rev_parse_single(BStr::new(git_ref))
        .map_err(|e| GitStorageError::ReadError(format!("ref '{}' not found: {}", git_ref, e)))?;

    let commit = id
        .object()
        .map_err(|e| GitStorageError::ReadError(format!("failed to resolve object: {}", e)))?
        .into_commit();

    let tree_id = commit
        .tree_id()
        .map_err(|e| GitStorageError::ReadError(format!("failed to get tree id: {}", e)))?;
    let tree = repo
        .find_tree(tree_id)
        .map_err(|e| GitStorageError::ReadError(format!("failed to find tree: {}", e)))?;

    let entries: Vec<FileTreeEntry> = tree
        .traverse()
        .breadthfirst
        .files()
        .map_err(|e| GitStorageError::ReadError(format!("failed to traverse tree: {}", e)))?
        .into_iter()
        .map(|entry| {
            let is_tree = entry.mode.is_tree();
            FileTreeEntry {
                name: String::from_utf8_lossy(&entry.filepath).to_string(),
                kind: if is_tree {
                    "tree".to_string()
                } else {
                    "blob".to_string()
                },
                oid: entry.oid.to_string(),
                is_tree,
            }
        })
        .collect();

    Ok(entries)
}

/// Read a blob by its SHA (hex string).
pub fn read_blob(repo_path: &Path, sha: &str) -> Result<Vec<u8>> {
    let repo = gix::open(repo_path).map_err(|e| {
        GitStorageError::ReadError(format!(
            "failed to open repo at {}: {}",
            repo_path.display(),
            e
        ))
    })?;

    let oid = gix::ObjectId::from_hex(sha.as_bytes())
        .map_err(|e| GitStorageError::ReadError(format!("invalid sha '{}': {}", sha, e)))?;

    let blob = repo
        .find_blob(oid)
        .map_err(|e| GitStorageError::ReadError(format!("blob '{}' not found: {}", sha, e)))?;

    Ok(blob.data.to_vec())
}

/// Read commit history starting from a ref, limited to `limit` entries (default 50, max 100).
pub fn read_commits(repo_path: &Path, git_ref: &str, limit: usize) -> Result<Vec<CommitInfo>> {
    let limit = limit.clamp(1, 100);

    let repo = gix::open(repo_path).map_err(|e| {
        GitStorageError::ReadError(format!(
            "failed to open repo at {}: {}",
            repo_path.display(),
            e
        ))
    })?;

    let id = repo
        .rev_parse_single(BStr::new(git_ref))
        .map_err(|e| GitStorageError::ReadError(format!("ref '{}' not found: {}", git_ref, e)))?;

    let commit_id = id
        .object()
        .map_err(|e| GitStorageError::ReadError(format!("failed to resolve object: {}", e)))?
        .into_commit();

    let mut commits = Vec::new();
    let ancestors = commit_id
        .ancestors()
        .sorting(gix::revision::walk::Sorting::ByCommitTime(
            gix::traverse::commit::simple::CommitTimeOrder::NewestFirst,
        ))
        .all()
        .map_err(|e| GitStorageError::ReadError(format!("failed to walk ancestors: {}", e)))?;

    for ancestor in ancestors {
        if commits.len() >= limit {
            break;
        }
        let info = ancestor.map_err(|e| {
            GitStorageError::ReadError(format!("failed to read commit info: {}", e))
        })?;
        let commit = info
            .id()
            .object()
            .map_err(|e| {
                GitStorageError::ReadError(format!("failed to resolve commit object: {}", e))
            })?
            .into_commit();
        let decoded = commit
            .decode()
            .map_err(|e| GitStorageError::ReadError(format!("failed to decode commit: {}", e)))?;

        let author = decoded
            .author()
            .map_err(|e| GitStorageError::ReadError(format!("failed to read author: {}", e)))?;
        let timestamp = author.time().map(|t| t.seconds).unwrap_or(0);

        commits.push(CommitInfo {
            sha: info.id.to_string(),
            author_name: author.name.to_string(),
            author_email: author.email.to_string(),
            message: decoded.message.to_string(),
            timestamp,
        });
    }

    Ok(commits)
}

/// Read diff between two refs.
pub fn read_diff(repo_path: &Path, base_ref: &str, head_ref: &str) -> Result<Vec<DiffEntry>> {
    let repo = gix::open(repo_path).map_err(|e| {
        GitStorageError::ReadError(format!(
            "failed to open repo at {}: {}",
            repo_path.display(),
            e
        ))
    })?;

    let base_id = repo.rev_parse_single(BStr::new(base_ref)).map_err(|e| {
        GitStorageError::ReadError(format!("base ref '{}' not found: {}", base_ref, e))
    })?;

    let head_id = repo.rev_parse_single(BStr::new(head_ref)).map_err(|e| {
        GitStorageError::ReadError(format!("head ref '{}' not found: {}", head_ref, e))
    })?;

    let base_tree_id = base_id
        .object()
        .map_err(|e| GitStorageError::ReadError(format!("failed to resolve base object: {}", e)))?
        .into_commit()
        .tree_id()
        .map_err(|e| GitStorageError::ReadError(format!("failed to get base tree id: {}", e)))?;

    let head_tree_id = head_id
        .object()
        .map_err(|e| GitStorageError::ReadError(format!("failed to resolve head object: {}", e)))?
        .into_commit()
        .tree_id()
        .map_err(|e| GitStorageError::ReadError(format!("failed to get head tree id: {}", e)))?;

    let base_tree = repo
        .find_tree(base_tree_id)
        .map_err(|e| GitStorageError::ReadError(format!("failed to find base tree: {}", e)))?;
    let head_tree = repo
        .find_tree(head_tree_id)
        .map_err(|e| GitStorageError::ReadError(format!("failed to find head tree: {}", e)))?;

    let changes = repo
        .diff_tree_to_tree(Some(&base_tree), Some(&head_tree), None)
        .map_err(|e| GitStorageError::ReadError(format!("failed to compute diff: {}", e)))?;

    let entries: Vec<DiffEntry> = changes
        .into_iter()
        .map(|change| {
            let (change_type, old_oid, new_oid) = match change {
                gix::object::tree::diff::ChangeDetached::Addition { id, .. } => {
                    ("added".to_string(), String::new(), id.to_string())
                }
                gix::object::tree::diff::ChangeDetached::Deletion { id, .. } => {
                    ("deleted".to_string(), id.to_string(), String::new())
                }
                gix::object::tree::diff::ChangeDetached::Modification {
                    previous_id, id, ..
                } => (
                    "modified".to_string(),
                    previous_id.to_string(),
                    id.to_string(),
                ),
                gix::object::tree::diff::ChangeDetached::Rewrite { source_id, id, .. } => (
                    "modified".to_string(),
                    source_id.to_string(),
                    id.to_string(),
                ),
            };

            let path = change.location().to_string();

            DiffEntry {
                path,
                change_type,
                old_oid,
                new_oid,
            }
        })
        .collect();

    Ok(entries)
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
