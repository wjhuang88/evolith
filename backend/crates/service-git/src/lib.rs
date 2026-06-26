use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::Duration;

use gix::bstr::BStr;
use gix::objs::Kind;
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
    #[error("Resource exceeded: {0}")]
    ResourceExceeded(String),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Ref or object not found: {0}")]
    NotFound(String),
    #[error("Git operation timed out: {0}")]
    Timeout(String),
}

/// Maximum bytes returned by `read_blob` (EVO-116 resource bound).
///
/// Set deliberately so a single API call cannot blow up the response body for
/// large binary blobs. Callers that need a larger window should switch to a
/// future streaming endpoint.
pub const BLOB_MAX_BYTES: usize = 1_048_576;

/// Maximum entries returned by `read_file_tree` (EVO-116 resource bound).
///
/// Prevents pathological repos with hundreds of thousands of files from making
/// a single Context API response unbounded.
pub const FILE_TREE_MAX_ENTRIES: usize = 5_000;

/// Maximum entries returned by `read_diff` (EVO-116 resource bound).
pub const DIFF_MAX_ENTRIES: usize = 5_000;

/// Upper bound on a single blocking gix read.
///
/// Wrapping `web::block` does not bound wall-clock time — the closure still
/// runs to completion on the blocking pool. We surface a `Timeout` error
/// instead of letting a slow disk / huge tree pin a worker indefinitely.
pub const CONTEXT_BLOCKING_TIMEOUT: Duration = Duration::from_secs(5);

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

struct BoundedTreeVisitor {
    path_deque: VecDeque<Vec<u8>>,
    path: Vec<u8>,
    entries: Vec<FileTreeEntry>,
    max_entries: usize,
    exceeded: bool,
}

impl BoundedTreeVisitor {
    fn new(max_entries: usize) -> Self {
        Self {
            path_deque: VecDeque::new(),
            path: Vec::new(),
            entries: Vec::new(),
            max_entries,
            exceeded: false,
        }
    }

    fn push_element(&mut self, name: &BStr) {
        if name.is_empty() {
            return;
        }
        if !self.path.is_empty() {
            self.path.push(b'/');
        }
        self.path.extend_from_slice(name);
    }

    fn pop_element(&mut self) {
        if let Some(pos) = self.path.iter().rposition(|b| *b == b'/') {
            self.path.truncate(pos);
        } else {
            self.path.clear();
        }
    }

    fn record(&mut self, entry: &gix::objs::tree::EntryRef<'_>) -> std::ops::ControlFlow<(), bool> {
        if self.entries.len() >= self.max_entries {
            self.exceeded = true;
            return std::ops::ControlFlow::Break(());
        }

        let is_tree = entry.mode.is_tree();
        self.entries.push(FileTreeEntry {
            name: String::from_utf8_lossy(&self.path).to_string(),
            kind: if is_tree {
                "tree".to_string()
            } else {
                "blob".to_string()
            },
            oid: entry.oid.to_string(),
            is_tree,
        });
        std::ops::ControlFlow::Continue(true)
    }
}

impl gix::traverse::tree::Visit for BoundedTreeVisitor {
    fn pop_back_tracked_path_and_set_current(&mut self) {
        self.path = self.path_deque.pop_back().unwrap_or_default();
    }

    fn pop_front_tracked_path_and_set_current(&mut self) {
        self.path = self.path_deque.pop_front().unwrap_or_default();
    }

    fn push_back_tracked_path_component(&mut self, component: &BStr) {
        self.push_element(component);
        self.path_deque.push_back(self.path.clone());
    }

    fn push_path_component(&mut self, component: &BStr) {
        self.push_element(component);
    }

    fn pop_path_component(&mut self) {
        self.pop_element();
    }

    fn visit_tree(
        &mut self,
        entry: &gix::objs::tree::EntryRef<'_>,
    ) -> gix::traverse::tree::visit::Action {
        self.record(entry)
    }

    fn visit_nontree(
        &mut self,
        entry: &gix::objs::tree::EntryRef<'_>,
    ) -> gix::traverse::tree::visit::Action {
        self.record(entry)
    }
}

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
        .map_err(|e| GitStorageError::NotFound(format!("ref '{}' not found: {}", git_ref, e)))?;

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

    let mut visitor = BoundedTreeVisitor::new(FILE_TREE_MAX_ENTRIES);
    match tree.traverse().breadthfirst(&mut visitor) {
        Ok(()) => Ok(visitor.entries),
        Err(gix::traverse::tree::breadthfirst::Error::Cancelled) if visitor.exceeded => {
            Err(GitStorageError::ResourceExceeded(format!(
                "file-tree exceeds max {} entries",
                FILE_TREE_MAX_ENTRIES
            )))
        }
        Err(e) => Err(GitStorageError::ReadError(format!(
            "failed to traverse tree: {}",
            e
        ))),
    }
}

/// Read a blob by its SHA (hex string).
pub fn read_blob(repo_path: &Path, sha: &str) -> Result<Vec<u8>> {
    if sha.trim().is_empty() {
        return Err(GitStorageError::InvalidInput("blob sha is empty".into()));
    }

    let repo = gix::open(repo_path).map_err(|e| {
        GitStorageError::ReadError(format!(
            "failed to open repo at {}: {}",
            repo_path.display(),
            e
        ))
    })?;

    let oid = gix::ObjectId::from_hex(sha.as_bytes())
        .map_err(|e| GitStorageError::InvalidInput(format!("invalid sha '{}': {}", sha, e)))?;

    let header = repo
        .find_header(oid)
        .map_err(|e| GitStorageError::NotFound(format!("blob '{}' not found: {}", sha, e)))?;
    if header.kind() != Kind::Blob {
        return Err(GitStorageError::NotFound(format!(
            "object '{}' is not a blob",
            sha
        )));
    }
    if header.size() as usize > BLOB_MAX_BYTES {
        return Err(GitStorageError::ResourceExceeded(format!(
            "blob size {} exceeds max {} bytes",
            header.size(),
            BLOB_MAX_BYTES
        )));
    }

    let blob = repo
        .find_blob(oid)
        .map_err(|e| GitStorageError::NotFound(format!("blob '{}' not found: {}", sha, e)))?;

    Ok(blob.data.clone())
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
        .map_err(|e| GitStorageError::NotFound(format!("ref '{}' not found: {}", git_ref, e)))?;

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
        GitStorageError::NotFound(format!("base ref '{}' not found: {}", base_ref, e))
    })?;

    let head_id = repo.rev_parse_single(BStr::new(head_ref)).map_err(|e| {
        GitStorageError::NotFound(format!("head ref '{}' not found: {}", head_ref, e))
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

    let mut entries = Vec::new();
    let mut exceeded = false;
    let mut changes = base_tree
        .changes()
        .map_err(|e| GitStorageError::ReadError(format!("failed to configure diff: {}", e)))?;
    changes
        .for_each_to_obtain_tree(&head_tree, |change| {
            use gix::object::tree::diff::Change;

            if entries.len() >= DIFF_MAX_ENTRIES {
                exceeded = true;
                return Ok::<_, std::convert::Infallible>(std::ops::ControlFlow::Break(()));
            }

            let (change_type, old_oid, new_oid) = match change {
                Change::Addition { id, .. } => ("added".to_string(), String::new(), id.to_string()),
                Change::Deletion { id, .. } => {
                    ("deleted".to_string(), id.to_string(), String::new())
                }
                Change::Modification {
                    previous_id, id, ..
                } => (
                    "modified".to_string(),
                    previous_id.to_string(),
                    id.to_string(),
                ),
                Change::Rewrite { source_id, id, .. } => (
                    "modified".to_string(),
                    source_id.to_string(),
                    id.to_string(),
                ),
            };

            let path = change.location().to_string();
            entries.push(DiffEntry {
                path,
                change_type,
                old_oid,
                new_oid,
            });
            Ok(std::ops::ControlFlow::Continue(()))
        })
        .map_err(|e| GitStorageError::ReadError(format!("failed to compute diff: {}", e)))?;

    if exceeded {
        return Err(GitStorageError::ResourceExceeded(format!(
            "diff exceeds max {} entries",
            DIFF_MAX_ENTRIES
        )));
    }

    Ok(entries)
}

/// Resolve `ref_name` to a commit (oid + timestamp).
///
/// Used by Smart HTTP to update `git_repos.last_commit_sha` / `last_committed_at`
/// after a successful push. Accepts both branch shorthand (e.g. `main`) and
/// fully-qualified ref names (e.g. `refs/heads/main`).
/// Returns `NotFound` if the ref does not exist (so the caller can log and
/// continue without breaking the push response).
pub fn resolve_ref(repo_path: &Path, ref_name: &str) -> Result<(String, i64)> {
    let repo = gix::open(repo_path).map_err(|e| {
        GitStorageError::ReadError(format!(
            "failed to open repo at {}: {}",
            repo_path.display(),
            e
        ))
    })?;

    let commit_object = if ref_name.starts_with("refs/") {
        let reference = repo.find_reference(ref_name).map_err(|e| {
            GitStorageError::NotFound(format!("ref '{}' not found: {}", ref_name, e))
        })?;
        let peeled_id = reference
            .into_fully_peeled_id()
            .map_err(|e| GitStorageError::ReadError(format!("failed to peel ref: {}", e)))?;
        repo.find_commit(peeled_id).map_err(|e| {
            GitStorageError::NotFound(format!("commit for ref '{}' not found: {}", ref_name, e))
        })?
    } else {
        let id = repo.rev_parse_single(BStr::new(ref_name)).map_err(|e| {
            GitStorageError::NotFound(format!("ref '{}' not found: {}", ref_name, e))
        })?;
        id.object()
            .map_err(|e| GitStorageError::ReadError(format!("failed to resolve object: {}", e)))?
            .into_commit()
    };

    let decoded = commit_object
        .decode()
        .map_err(|e| GitStorageError::ReadError(format!("failed to decode commit: {}", e)))?;
    let author = decoded
        .author()
        .map_err(|e| GitStorageError::ReadError(format!("failed to read author: {}", e)))?;
    let timestamp = author.time().map(|t| t.seconds).unwrap_or(0);
    let oid_str = commit_object.id.to_string();
    Ok((oid_str, timestamp))
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
