use std::collections::{HashSet, VecDeque};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::process::{Command, Output};
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

/// Complete, immutable evidence for one commit within a verified ref context.
#[derive(Debug, Clone)]
pub struct CommitDetail {
    pub sha: String,
    pub author_name: String,
    pub author_email: String,
    pub authored_at: i64,
    pub committer_name: String,
    pub committer_email: String,
    pub committed_at: i64,
    pub message: String,
    pub parents: Vec<String>,
    pub git_ref: String,
    pub changes: Vec<DiffEntry>,
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

fn ensure_real_directory(path: &Path, context: &str) -> Result<()> {
    match std::fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
            Err(GitStorageError::InitError(format!(
                "{context} must be a real directory: {}",
                path.display()
            )))
        }
        Ok(_) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => std::fs::create_dir(path)
            .map_err(|error| {
                GitStorageError::InitError(format!(
                    "failed to create {context} {}: {}",
                    path.display(),
                    error
                ))
            }),
        Err(error) => Err(GitStorageError::InitError(format!(
            "failed to inspect {context} {}: {}",
            path.display(),
            error
        ))),
    }
}

pub fn init_bare_repo(base_path: &Path, tenant_id: Uuid, repo_id: Uuid) -> Result<PathBuf> {
    let path = repo_path(base_path, tenant_id, repo_id);
    std::fs::create_dir_all(base_path).map_err(|error| {
        GitStorageError::InitError(format!(
            "failed to create storage root {}: {}",
            base_path.display(),
            error
        ))
    })?;
    ensure_real_directory(
        &base_path.join(tenant_id.to_string()),
        "tenant storage directory",
    )?;

    gix::init_bare(&path).map_err(|e| {
        GitStorageError::InitError(format!(
            "gix::init_bare failed for {}: {}",
            path.display(),
            e
        ))
    })?;

    Ok(path)
}

fn run_git(args: &[&str], working_dir: Option<&Path>) -> Result<Output> {
    let mut command = Command::new("git");
    command.args(args);
    if let Some(path) = working_dir {
        command.current_dir(path);
    }
    let output = command
        .output()
        .map_err(|error| GitStorageError::SubprocessError(error.to_string()))?;
    if output.status.success() {
        Ok(output)
    } else {
        Err(GitStorageError::SubprocessError(format!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        )))
    }
}

pub fn seed_initial_commit(
    storage_path: &Path,
    repo_name: &str,
    default_branch: &str,
) -> Result<String> {
    run_git(&["check-ref-format", "--branch", default_branch], None)?;
    let worktree = storage_path
        .parent()
        .ok_or(GitStorageError::NoBasePath)?
        .join(format!(".seed-{}", Uuid::new_v4()));
    std::fs::create_dir(&worktree).map_err(|source| GitStorageError::SeedWriteError {
        path: worktree.display().to_string(),
        source,
    })?;

    let result = (|| {
        run_git(
            &["init", "--initial-branch", default_branch],
            Some(&worktree),
        )?;
        run_git(&["config", "user.name", "Evolith"], Some(&worktree))?;
        run_git(
            &["config", "user.email", "noreply@evolith.local"],
            Some(&worktree),
        )?;

        let readme_path = worktree.join("README.md");
        let readme_content = format!("# {}\n\nDefault branch: `{}`\n", repo_name, default_branch);

        std::fs::write(&readme_path, &readme_content).map_err(|e| {
            GitStorageError::SeedWriteError {
                path: readme_path.display().to_string(),
                source: e,
            }
        })?;

        let evolith_dir = worktree.join(".evolith");
        std::fs::create_dir_all(&evolith_dir).map_err(|e| GitStorageError::SeedWriteError {
            path: evolith_dir.display().to_string(),
            source: e,
        })?;

        let policy_path = evolith_dir.join("policy.yaml");
        let policy_content = "default_action: require_review\n";

        std::fs::write(&policy_path, policy_content).map_err(|e| {
            GitStorageError::SeedWriteError {
                path: policy_path.display().to_string(),
                source: e,
            }
        })?;

        let agents_path = evolith_dir.join("agents.yaml");
        let agents_content = "agents: []\n";

        std::fs::write(&agents_path, agents_content).map_err(|e| {
            GitStorageError::SeedWriteError {
                path: agents_path.display().to_string(),
                source: e,
            }
        })?;

        run_git(&["add", "."], Some(&worktree))?;
        run_git(&["commit", "-m", "Initial commit"], Some(&worktree))?;
        let storage = storage_path
            .to_str()
            .ok_or_else(|| GitStorageError::InvalidInput("storage path is not UTF-8".into()))?;
        let destination = format!("HEAD:refs/heads/{default_branch}");
        run_git(&["push", storage, &destination], Some(&worktree))?;
        run_git(
            &[
                "--git-dir",
                storage,
                "symbolic-ref",
                "HEAD",
                &format!("refs/heads/{default_branch}"),
            ],
            None,
        )?;
        let output = run_git(&["rev-parse", "HEAD"], Some(&worktree))?;
        String::from_utf8(output.stdout)
            .map(|sha| sha.trim().to_string())
            .map_err(|error| GitStorageError::ReadError(error.to_string()))
    })();

    if let Err(error) = std::fs::remove_dir_all(&worktree) {
        tracing::warn!(
            "Failed to remove seed worktree {}: {}",
            worktree.display(),
            error
        );
    }
    result
}

pub fn remove_repo(storage_path: &Path) -> Result<()> {
    std::fs::remove_dir_all(storage_path).map_err(|e| GitStorageError::RemoveError {
        path: storage_path.display().to_string(),
        source: e,
    })
}

/// Return tenant repo ids represented by direct `<uuid>.git` directories.
/// Other entries, including symlinks and quarantine data, are ignored.
pub fn inventory_tenant_repos(base_path: &Path, tenant_id: Uuid) -> Result<HashSet<Uuid>> {
    let tenant_path = base_path.join(tenant_id.to_string());
    match std::fs::symlink_metadata(&tenant_path) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
            return Err(GitStorageError::ReadError(format!(
                "tenant storage must be a real directory: {}",
                tenant_path.display()
            )));
        }
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(HashSet::new());
        }
        Err(error) => {
            return Err(GitStorageError::ReadError(format!(
                "failed to inspect tenant storage {}: {}",
                tenant_path.display(),
                error
            )));
        }
    }

    let entries = std::fs::read_dir(&tenant_path).map_err(|error| {
        GitStorageError::ReadError(format!(
            "failed to inventory tenant storage {}: {}",
            tenant_path.display(),
            error
        ))
    })?;
    let mut repo_ids = HashSet::new();
    for entry in entries {
        let entry = entry.map_err(|error| {
            GitStorageError::ReadError(format!(
                "failed to read tenant storage entry in {}: {}",
                tenant_path.display(),
                error
            ))
        })?;
        let file_type = entry.file_type().map_err(|error| {
            GitStorageError::ReadError(format!(
                "failed to inspect tenant storage entry {}: {}",
                entry.path().display(),
                error
            ))
        })?;
        if !file_type.is_dir() {
            continue;
        }
        let Some(name) = entry.file_name().to_str().map(str::to_owned) else {
            continue;
        };
        let Some(id) = name.strip_suffix(".git") else {
            continue;
        };
        if let Ok(id) = Uuid::parse_str(id) {
            repo_ids.insert(id);
        }
    }
    Ok(repo_ids)
}

/// Move an unowned repo into a tenant-scoped quarantine directory without deleting it.
pub fn quarantine_repo(base_path: &Path, tenant_id: Uuid, repo_id: Uuid) -> Result<PathBuf> {
    let source = repo_path(base_path, tenant_id, repo_id);
    let source_metadata =
        std::fs::symlink_metadata(&source).map_err(|error| GitStorageError::RemoveError {
            path: source.display().to_string(),
            source: error,
        })?;
    if source_metadata.file_type().is_symlink() || !source_metadata.is_dir() {
        return Err(GitStorageError::ReadError(format!(
            "repo storage must be a real directory: {}",
            source.display()
        )));
    }
    let quarantine_root = base_path.join(".reconcile-trash");
    ensure_real_directory(&quarantine_root, "reconcile quarantine directory")?;
    let quarantine_dir = quarantine_root.join(tenant_id.to_string());
    ensure_real_directory(&quarantine_dir, "tenant reconcile quarantine directory")?;
    let destination = quarantine_dir.join(format!("{}.git.{}", repo_id, Uuid::new_v4()));
    std::fs::rename(&source, &destination).map_err(|error| GitStorageError::RemoveError {
        path: source.display().to_string(),
        source: error,
    })?;
    Ok(destination)
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

/// Read one exact commit and prove that it is reachable from `git_ref`.
///
/// The returned diff is a bounded, structured list of changed files. Root commits are compared
/// with an empty tree by treating every blob in their tree as an addition.
pub fn read_commit_detail(repo_path: &Path, sha: &str, git_ref: &str) -> Result<CommitDetail> {
    if sha.len() != 40 || !sha.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(GitStorageError::InvalidInput(
            "commit sha must be exactly 40 hexadecimal characters".to_string(),
        ));
    }
    if git_ref.trim().is_empty() {
        return Err(GitStorageError::InvalidInput(
            "ref must not be empty".to_string(),
        ));
    }

    let repo = gix::open(repo_path).map_err(|e| {
        GitStorageError::ReadError(format!(
            "failed to open repo at {}: {}",
            repo_path.display(),
            e
        ))
    })?;

    let target_id = repo
        .rev_parse_single(BStr::new(sha))
        .map_err(|e| GitStorageError::NotFound(format!("commit '{}' not found: {}", sha, e)))?;
    let target = target_id
        .object()
        .map_err(|e| GitStorageError::ReadError(format!("failed to resolve commit: {}", e)))?
        .into_commit();
    if !target.id.to_string().eq_ignore_ascii_case(sha) {
        return Err(GitStorageError::NotFound(format!(
            "commit '{}' not found",
            sha
        )));
    }

    let ref_id = repo
        .rev_parse_single(BStr::new(git_ref))
        .map_err(|e| GitStorageError::NotFound(format!("ref '{}' not found: {}", git_ref, e)))?;
    let ref_commit = ref_id
        .object()
        .map_err(|e| GitStorageError::ReadError(format!("failed to resolve ref: {}", e)))?
        .into_commit();
    let reachable = if ref_commit.id == target.id {
        true
    } else {
        ref_id
            .ancestors()
            .all()
            .map_err(|e| GitStorageError::ReadError(format!("failed to walk ref: {}", e)))?
            .any(|info| info.is_ok_and(|entry| entry.id == target.id))
    };
    if !reachable {
        return Err(GitStorageError::NotFound(format!(
            "commit '{}' is not reachable from ref '{}'",
            sha, git_ref
        )));
    }

    let decoded = target
        .decode()
        .map_err(|e| GitStorageError::ReadError(format!("failed to decode commit: {}", e)))?;
    let author = decoded
        .author()
        .map_err(|e| GitStorageError::ReadError(format!("failed to read author: {}", e)))?;
    let committer = decoded
        .committer()
        .map_err(|e| GitStorageError::ReadError(format!("failed to read committer: {}", e)))?;
    let parents: Vec<String> = decoded.parents().map(|id| id.to_string()).collect();

    let changes = if let Some(parent) = parents.first() {
        read_diff(repo_path, parent, sha)?
    } else {
        let entries = read_file_tree(repo_path, sha)?;
        let blobs: Vec<_> = entries.into_iter().filter(|entry| !entry.is_tree).collect();
        if blobs.len() > DIFF_MAX_ENTRIES {
            return Err(GitStorageError::ResourceExceeded(format!(
                "diff exceeds max {} entries",
                DIFF_MAX_ENTRIES
            )));
        }
        blobs
            .into_iter()
            .map(|entry| DiffEntry {
                path: entry.name,
                change_type: "added".to_string(),
                old_oid: String::new(),
                new_oid: entry.oid,
            })
            .collect()
    };

    Ok(CommitDetail {
        sha: target.id.to_string(),
        author_name: author.name.to_string(),
        author_email: author.email.to_string(),
        authored_at: author.time().map(|time| time.seconds).unwrap_or(0),
        committer_name: committer.name.to_string(),
        committer_email: committer.email.to_string(),
        committed_at: committer.time().map(|time| time.seconds).unwrap_or(0),
        message: decoded.message.to_string(),
        parents,
        git_ref: git_ref.to_string(),
        changes,
    })
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
    async fn seed_initial_commit_creates_cloneable_branch() {
        let _guard = PATH_LOCK.lock().await;
        let root = std::env::temp_dir().join(format!("evolith-seed-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&root).unwrap();
        let tenant_id = Uuid::new_v4();
        let repo_id = Uuid::new_v4();
        let repo_path = init_bare_repo(&root, tenant_id, repo_id).unwrap();

        let sha = seed_initial_commit(&repo_path, "seeded-repo", "main").unwrap();
        let (resolved, _) = resolve_ref(&repo_path, "refs/heads/main").unwrap();
        assert_eq!(sha, resolved);

        let tree = read_file_tree(&repo_path, &sha).unwrap();
        let paths: Vec<_> = tree.into_iter().map(|entry| entry.name).collect();
        assert!(paths.contains(&"README.md".to_string()));
        assert!(paths.contains(&".evolith/policy.yaml".to_string()));
        assert!(paths.contains(&".evolith/agents.yaml".to_string()));

        let detail = read_commit_detail(&repo_path, &sha, "main").unwrap();
        assert_eq!(detail.sha, sha);
        assert!(detail.parents.is_empty());
        assert_eq!(detail.git_ref, "main");
        assert!(detail
            .changes
            .iter()
            .any(|entry| entry.path == "README.md" && entry.change_type == "added"));
        assert!(matches!(
            read_commit_detail(&repo_path, "not-a-sha", "main"),
            Err(GitStorageError::InvalidInput(_))
        ));

        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn inventory_and_quarantine_are_tenant_scoped() {
        let root = std::env::temp_dir().join(format!("evolith-inventory-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&root).unwrap();
        let tenant_id = Uuid::new_v4();
        let other_tenant_id = Uuid::new_v4();
        let repo_id = Uuid::new_v4();
        let other_repo_id = Uuid::new_v4();
        init_bare_repo(&root, tenant_id, repo_id).unwrap();
        init_bare_repo(&root, other_tenant_id, other_repo_id).unwrap();
        std::fs::write(
            root.join(tenant_id.to_string()).join("not-a-repo"),
            b"ignored",
        )
        .unwrap();

        let inventory = inventory_tenant_repos(&root, tenant_id).unwrap();
        assert_eq!(inventory, HashSet::from([repo_id]));

        let quarantined = quarantine_repo(&root, tenant_id, repo_id).unwrap();
        assert!(quarantined.exists());
        assert!(!repo_path(&root, tenant_id, repo_id).exists());
        assert!(repo_path(&root, other_tenant_id, other_repo_id).exists());

        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    #[cfg(unix)]
    fn tenant_storage_symlink_is_rejected() {
        use std::os::unix::fs::symlink;

        let root = std::env::temp_dir().join(format!("evolith-symlink-{}", Uuid::new_v4()));
        let outside = std::env::temp_dir().join(format!("evolith-outside-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&root).unwrap();
        std::fs::create_dir_all(&outside).unwrap();
        let tenant_id = Uuid::new_v4();
        symlink(&outside, root.join(tenant_id.to_string())).unwrap();

        assert!(init_bare_repo(&root, tenant_id, Uuid::new_v4()).is_err());
        assert!(inventory_tenant_repos(&root, tenant_id).is_err());
        assert!(std::fs::read_dir(&outside).unwrap().next().is_none());

        std::fs::remove_dir_all(root).unwrap();
        std::fs::remove_dir_all(outside).unwrap();
    }

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
