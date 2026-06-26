use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RefQuery {
    #[serde(rename = "ref")]
    pub git_ref: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DiffQuery {
    pub base: String,
    pub head: String,
}

#[derive(Debug, Deserialize)]
pub struct CommitQuery {
    #[serde(rename = "ref")]
    pub git_ref: Option<String>,
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct FileTreeEntryDto {
    pub name: String,
    pub kind: String,
    pub oid: String,
    pub is_tree: bool,
}

#[derive(Debug, Serialize)]
pub struct FileTreeResponse {
    pub entries: Vec<FileTreeEntryDto>,
}

#[derive(Debug, Serialize)]
pub struct BlobResponse {
    pub content: String,
    pub size: usize,
    pub encoding: String,
}

#[derive(Debug, Serialize)]
pub struct CommitDto {
    pub sha: String,
    pub author_name: String,
    pub author_email: String,
    pub message: String,
    pub timestamp: i64,
}

#[derive(Debug, Serialize)]
pub struct CommitListResponse {
    pub commits: Vec<CommitDto>,
}

#[derive(Debug, Serialize)]
pub struct DiffEntryDto {
    pub path: String,
    pub change_type: String,
    pub old_oid: String,
    pub new_oid: String,
}

#[derive(Debug, Serialize)]
pub struct DiffResponse {
    pub entries: Vec<DiffEntryDto>,
}
