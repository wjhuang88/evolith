//! Tool DTOs for request/response handling

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::dto::common::PaginationMeta;

/// Request to create a new tool
#[derive(Debug, Deserialize, Validate)]
pub struct CreateToolRequest {
    /// Tool name (1-128 characters)
    #[validate(length(min = 1, max = 128))]
    pub name: String,

    /// Tool description
    #[validate(length(min = 1))]
    pub description: String,

    /// JSON Schema for tool input
    pub input_schema: serde_json::Value,

    /// Handler type: "http" or "function"
    #[serde(rename = "type", default = "default_handler_type")]
    pub handler_type: String,

    /// Handler URL (for HTTP type)
    #[serde(default)]
    pub handler_url: Option<String>,

    /// HTTP method (for HTTP type)
    #[serde(default)]
    pub handler_method: Option<String>,

    /// Request timeout in milliseconds
    #[serde(default)]
    pub handler_timeout: Option<u32>,

    /// Whether tool is publicly visible
    #[serde(default)]
    pub is_public: Option<bool>,
}

fn default_handler_type() -> String {
    "function".to_string()
}

/// Request to update an existing tool
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateToolRequest {
    /// Tool name (1-128 characters)
    #[validate(length(min = 1, max = 128))]
    pub name: Option<String>,

    /// Tool description
    #[validate(length(min = 1))]
    pub description: Option<String>,

    /// JSON Schema for tool input
    pub input_schema: Option<serde_json::Value>,

    /// Handler type: "http" or "function"
    #[serde(rename = "type")]
    pub handler_type: Option<String>,

    /// Handler URL (for HTTP type)
    pub handler_url: Option<String>,

    /// HTTP method (for HTTP type)
    pub handler_method: Option<String>,

    /// Request timeout in milliseconds
    pub handler_timeout: Option<u32>,

    /// Whether tool is publicly visible
    pub is_public: Option<bool>,
}

/// Tool response for API
#[derive(Debug, Serialize)]
pub struct ToolResponse {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    /// Category defaults to "custom" (not in domain model)
    pub category: String,
    /// JSON Schema for input
    pub schema: serde_json::Value,
    /// Handler configuration as JSON object
    pub handler: serde_json::Value,
    /// Derived from visibility
    pub is_public: bool,
    pub owner_id: Uuid,
    pub tenant_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<domain::tool::Tool> for ToolResponse {
    fn from(tool: domain::tool::Tool) -> Self {
        use domain::tool::Visibility;

        let is_public = matches!(tool.visibility, Visibility::Public);
        let handler = serde_json::to_value(&tool.handler).unwrap_or_else(|_| {
            serde_json::json!({
                "type": "function"
            })
        });

        Self {
            id: tool.id,
            name: tool.name,
            description: tool.description,
            category: "custom".to_string(),
            schema: tool.input_schema,
            handler,
            is_public,
            owner_id: tool.owner_id,
            tenant_id: tool.tenant_id,
            created_at: tool.created_at,
            updated_at: tool.updated_at,
        }
    }
}

/// List tools response with pagination
#[derive(Debug, Serialize)]
pub struct ToolListResponse {
    pub tools: Vec<ToolResponse>,
    #[serde(flatten)]
    pub pagination: PaginationMeta,
}

impl ToolListResponse {
    pub fn new(tools: Vec<ToolResponse>, page: u32, per_page: u32, total: u32) -> Self {
        Self {
            tools,
            pagination: PaginationMeta {
                page,
                per_page,
                total,
            },
        }
    }
}
