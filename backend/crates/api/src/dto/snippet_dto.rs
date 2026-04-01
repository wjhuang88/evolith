//! Snippet DTOs

use serde::{Deserialize, Serialize};
use validator::Validate;

use domain::snippet::{Dependency, Snippet, Visibility};

/// Request to create a new snippet
#[derive(Debug, Deserialize, Validate)]
pub struct CreateSnippetRequest {
    /// Snippet name/title (1-128 chars)
    #[validate(length(min = 1, max = 128))]
    pub name: String,

    /// Programming language (1-32 chars)
    #[validate(length(min = 1, max = 32))]
    pub language: String,

    /// Optional framework name
    pub framework: Option<String>,

    /// Tags for categorization
    #[serde(default)]
    pub tags: Vec<String>,

    /// Markdown documentation content
    #[validate(length(min = 1))]
    pub content: String,

    /// Code snippet
    #[validate(length(min = 1))]
    pub code: String,

    /// Package dependencies
    #[serde(default)]
    pub dependencies: Vec<DependencyInput>,

    /// Estimated token count (optional, calculated if not provided)
    pub estimated_tokens: Option<u32>,

    /// Public visibility (default: false = private)
    #[serde(default)]
    pub is_public: bool,
}

/// Request to update an existing snippet
#[derive(Debug, Deserialize)]
pub struct UpdateSnippetRequest {
    pub name: Option<String>,
    pub language: Option<String>,
    pub framework: Option<String>,
    pub tags: Option<Vec<String>>,
    pub content: Option<String>,
    pub code: Option<String>,
    pub dependencies: Option<Vec<DependencyInput>>,
    pub estimated_tokens: Option<u32>,
    pub is_public: Option<bool>,
}

/// Dependency input for snippet creation
#[derive(Debug, Clone, Deserialize)]
pub struct DependencyInput {
    /// Package name
    pub name: String,
    /// Version constraint
    pub version: String,
    /// Whether this dependency is required
    #[serde(default)]
    pub required: bool,
}

impl DependencyInput {
    /// Convert to domain Dependency
    pub fn to_domain(&self) -> Dependency {
        Dependency {
            name: self.name.clone(),
            version: self.version.clone(),
            required: self.required,
        }
    }
}

/// Snippet response DTO
#[derive(Debug, Serialize)]
pub struct SnippetResponse {
    pub id: String,
    /// Title (mapped from domain 'name')
    pub title: String,
    /// Description (not in domain, always empty)
    #[serde(default)]
    pub description: String,
    pub language: String,
    pub framework: Option<String>,
    pub tags: Vec<String>,
    /// Markdown documentation content
    pub content: String,
    /// Code snippet
    pub code: String,
    pub dependencies: Vec<DependencyResponse>,
    pub estimated_tokens: u32,
    /// Public visibility
    pub is_public: bool,
    pub owner_id: String,
    pub tenant_id: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Dependency in response
#[derive(Debug, Serialize)]
pub struct DependencyResponse {
    pub name: String,
    pub version: String,
    pub required: bool,
}

impl From<Dependency> for DependencyResponse {
    fn from(dep: Dependency) -> Self {
        Self {
            name: dep.name,
            version: dep.version,
            required: dep.required,
        }
    }
}

impl From<Snippet> for SnippetResponse {
    fn from(snippet: Snippet) -> Self {
        Self {
            id: snippet.id.to_string(),
            title: snippet.name,
            description: String::new(),
            language: snippet.language,
            framework: snippet.framework,
            tags: snippet.tags,
            content: snippet.content,
            code: snippet.code,
            dependencies: snippet
                .dependencies
                .into_iter()
                .map(DependencyResponse::from)
                .collect(),
            estimated_tokens: snippet.estimated_tokens,
            is_public: matches!(snippet.visibility, Visibility::Public),
            owner_id: snippet.owner_id.to_string(),
            tenant_id: snippet.tenant_id.to_string(),
            created_at: snippet.created_at.to_rfc3339(),
            updated_at: snippet.updated_at.to_rfc3339(),
        }
    }
}

/// Response for snippet reference (LLM-optimized format)
#[derive(Debug, Serialize)]
pub struct SnippetReferenceResponse {
    /// Snippet name/title
    pub name: String,
    /// Programming language
    pub language: String,
    /// Optional framework
    pub framework: Option<String>,
    /// Documentation content
    pub content: String,
    /// Code snippet
    pub code: String,
    /// Estimated tokens
    pub estimated_tokens: u32,
}

impl From<Snippet> for SnippetReferenceResponse {
    fn from(snippet: Snippet) -> Self {
        Self {
            name: snippet.name,
            language: snippet.language,
            framework: snippet.framework,
            content: snippet.content,
            code: snippet.code,
            estimated_tokens: snippet.estimated_tokens,
        }
    }
}
