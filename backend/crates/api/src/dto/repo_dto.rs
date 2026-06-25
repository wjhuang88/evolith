use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateRepoRequest {
    #[validate(length(min = 1, max = 128))]
    pub name: String,
    pub description: Option<String>,
    pub default_branch: Option<String>,
    pub visibility: Option<String>,
    pub auto_merge: Option<bool>,
    pub require_review: Option<bool>,
    pub seed_template: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateRepoRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub default_branch: Option<String>,
    pub visibility: Option<String>,
    pub auto_merge: Option<bool>,
    pub require_review: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct RepoResponse {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub description: String,
    pub default_branch: String,
    pub storage_path: String,
    pub visibility: String,
    pub auto_merge: bool,
    pub require_review: bool,
    pub last_commit_sha: Option<String>,
    pub last_committed_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct RepoListResponse {
    pub repos: Vec<RepoResponse>,
    pub total: usize,
}
