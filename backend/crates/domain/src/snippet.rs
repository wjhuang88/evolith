//! Snippet domain model

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snippet {
    pub id: Uuid,
    pub name: String,
    pub language: String,
    pub framework: Option<String>,
    pub tags: Vec<String>,
    pub content: String,
    pub code: String,
    pub dependencies: Vec<Dependency>,
    pub estimated_tokens: u32,
    pub owner_id: Uuid,
    pub visibility: Visibility,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub version: String,
    pub required: bool,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct NewSnippet {
    #[validate(length(min = 1, max = 128))]
    pub name: String,
    #[validate(length(min = 1, max = 32))]
    pub language: String,
    pub framework: Option<String>,
    pub tags: Vec<String>,
    #[validate(length(min = 1))]
    pub content: String,
    #[validate(length(min = 1))]
    pub code: String,
    pub dependencies: Vec<Dependency>,
    pub visibility: Option<Visibility>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SnippetFilter {
    pub search: Option<String>,
    pub language: Option<String>,
    pub framework: Option<String>,
    pub tags: Vec<String>,
    pub visibility: Option<Visibility>,
    pub owner_id: Option<Uuid>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

impl Default for SnippetFilter {
    fn default() -> Self {
        Self {
            search: None,
            language: None,
            framework: None,
            tags: Vec::new(),
            visibility: None,
            owner_id: None,
            page: Some(1),
            per_page: Some(20),
        }
    }
}

// Re-export Visibility from tool module
pub use crate::tool::Visibility;
