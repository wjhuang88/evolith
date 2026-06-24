use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Git repository entity — metadata index for a hosted git repo.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitRepo {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub description: String,
    pub default_branch: String,
    /// Filesystem path to the bare git repository.
    pub storage_path: String,
    pub visibility: RepoVisibility,
    pub auto_merge: bool,
    pub require_review: bool,
    pub last_commit_sha: Option<String>,
    pub last_committed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RepoVisibility {
    Public,
    #[default]
    Private,
}

/// Input for creating a new git repo.
#[derive(Debug, Clone, Deserialize)]
pub struct NewGitRepo {
    pub name: String,
    pub description: Option<String>,
    pub default_branch: Option<String>,
    pub visibility: Option<RepoVisibility>,
    pub auto_merge: Option<bool>,
    pub require_review: Option<bool>,
}

/// Input for updating an existing git repo.
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateGitRepo {
    pub name: Option<String>,
    pub description: Option<String>,
    pub default_branch: Option<String>,
    pub visibility: Option<RepoVisibility>,
    pub auto_merge: Option<bool>,
    pub require_review: Option<bool>,
}
