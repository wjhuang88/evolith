//! Tool domain model

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
    pub output_schema: Option<serde_json::Value>,
    pub handler: HandlerConfig,
    pub owner_id: Uuid,
    pub tenant_id: Uuid,
    pub visibility: Visibility,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandlerConfig {
    #[serde(rename = "type")]
    pub handler_type: HandlerType,
    pub url: Option<String>,
    pub method: Option<String>,
    pub timeout: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HandlerType {
    Http,
    Function,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Public,
    Private,
}

impl Default for Visibility {
    fn default() -> Self {
        Visibility::Private
    }
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct NewTool {
    #[validate(length(min = 1, max = 128))]
    pub name: String,
    #[validate(length(min = 1))]
    pub description: String,
    pub input_schema: serde_json::Value,
    pub output_schema: Option<serde_json::Value>,
    pub handler: HandlerConfig,
    pub visibility: Option<Visibility>,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateTool {
    #[validate(length(min = 1, max = 128))]
    pub name: Option<String>,
    #[validate(length(min = 1))]
    pub description: Option<String>,
    pub input_schema: Option<serde_json::Value>,
    pub output_schema: Option<serde_json::Value>,
    pub handler: Option<HandlerConfig>,
    pub visibility: Option<Visibility>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ToolFilter {
    pub tenant_id: Option<Uuid>,
    pub search: Option<String>,
    pub visibility: Option<Visibility>,
    pub owner_id: Option<Uuid>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

impl Default for ToolFilter {
    fn default() -> Self {
        Self {
            tenant_id: None,
            search: None,
            visibility: None,
            owner_id: None,
            page: Some(1),
            per_page: Some(20),
        }
    }
}
