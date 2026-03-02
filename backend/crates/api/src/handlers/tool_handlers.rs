//! Tool handlers

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::dto::common::{ApiResponse, PaginationMeta};

/// Tool categories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ToolCategory {
    Filesystem,
    Network,
    Database,
    Data,
    AI,
    Custom,
}

impl Default for ToolCategory {
    fn default() -> Self {
        ToolCategory::Custom
    }
}

impl std::fmt::Display for ToolCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToolCategory::Filesystem => write!(f, "filesystem"),
            ToolCategory::Network => write!(f, "network"),
            ToolCategory::Database => write!(f, "database"),
            ToolCategory::Data => write!(f, "data"),
            ToolCategory::AI => write!(f, "ai"),
            ToolCategory::Custom => write!(f, "custom"),
        }
    }
}

/// Tool visibility
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ToolVisibility {
    Public,
    Private,
}

impl Default for ToolVisibility {
    fn default() -> Self {
        ToolVisibility::Private
    }
}

/// Tool response
#[derive(Debug, Serialize, Clone)]
pub struct ToolResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub schema: serde_json::Value,
    pub handler: serde_json::Value,
    pub is_public: bool,
    pub owner_id: String,
    pub tenant_id: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Create tool request
#[derive(Debug, Deserialize, Validate)]
pub struct CreateToolRequest {
    #[validate(length(min = 1, max = 128))]
    pub name: String,

    #[validate(length(min = 1))]
    pub description: String,

    #[validate(length(min = 1))]
    pub category: String,

    pub schema: serde_json::Value,
    pub handler: Option<serde_json::Value>,
    pub is_public: Option<bool>,
}

/// Update tool request
#[derive(Debug, Deserialize)]
pub struct UpdateToolRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub schema: Option<serde_json::Value>,
    pub handler: Option<serde_json::Value>,
    pub is_public: Option<bool>,
}

/// Tool store
pub struct ToolStore {
    tools: Mutex<HashMap<String, StoredTool>>,
}

impl ToolStore {
    /// Get all tools
    pub fn get_all(&self) -> std::sync::MutexGuard<'_, HashMap<String, StoredTool>> {
        self.tools.lock().unwrap()
    }
}

#[derive(Clone)]
pub struct StoredTool {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub schema: serde_json::Value,
    pub handler: serde_json::Value,
    pub is_public: bool,
    pub owner_id: String,
    pub tenant_id: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Default for ToolStore {
    fn default() -> Self {
        let mut store = Self {
            tools: Mutex::new(HashMap::new()),
        };

        // Seed data
        let seed_tools = vec![
            (
                "550e8400-e29b-41d4-a716-446655440001",
                "filesystem_read",
                "Read files from the filesystem with safety checks",
                "filesystem",
                true,
            ),
            (
                "550e8400-e29b-41d4-a716-446655440002",
                "http_request",
                "Make HTTP requests to external APIs",
                "network",
                true,
            ),
            (
                "550e8400-e29b-41d4-a716-446655440003",
                "database_query",
                "Execute SQL queries against connected databases",
                "database",
                true,
            ),
            (
                "550e8400-e29b-41d4-a716-446655440004",
                "web_scraper",
                "Extract structured data from web pages",
                "network",
                false,
            ),
            (
                "550e8400-e29b-41d4-a716-446655440005",
                "json_transform",
                "Transform JSON data using JSONPath expressions",
                "data",
                true,
            ),
        ];

        let now = Utc::now().timestamp();

        for (id, name, desc, cat, public) in seed_tools {
            let schema = match name {
                "filesystem_read" => serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "The file path to read" },
                        "encoding": { "type": "string", "enum": ["utf-8", "binary"], "default": "utf-8" }
                    },
                    "required": ["path"]
                }),
                "http_request" => serde_json::json!({
                    "type": "object",
                    "properties": {
                        "url": { "type": "string", "description": "The URL to request" },
                        "method": { "type": "string", "enum": ["GET", "POST", "PUT", "DELETE"], "default": "GET" },
                        "headers": { "type": "object", "additionalProperties": { "type": "string" } },
                        "body": { "type": "string", "description": "Request body for POST/PUT" }
                    },
                    "required": ["url"]
                }),
                "database_query" => serde_json::json!({
                    "type": "object",
                    "properties": {
                        "query": { "type": "string", "description": "SQL query to execute" },
                        "params": { "type": "array", "items": { "type": "string" }, "description": "Query parameters" }
                    },
                    "required": ["query"]
                }),
                "web_scraper" => serde_json::json!({
                    "type": "object",
                    "properties": {
                        "url": { "type": "string", "description": "URL to scrape" },
                        "selector": { "type": "string", "description": "CSS selector for data extraction" },
                        "extract_links": { "type": "boolean", "default": false }
                    },
                    "required": ["url"]
                }),
                "json_transform" => serde_json::json!({
                    "type": "object",
                    "properties": {
                        "data": { "type": "object", "description": "Input JSON data" },
                        "expression": { "type": "string", "description": "JSONPath expression" }
                    },
                    "required": ["data", "expression"]
                }),
                _ => serde_json::json!({}),
            };

            let handler = serde_json::json!({
                "type": "http",
                "url": format!("https://api.example.com/tools/{}", name),
                "method": "POST"
            });

            let tool = StoredTool {
                id: id.to_string(),
                name: name.to_string(),
                description: desc.to_string(),
                category: cat.to_string(),
                schema,
                handler,
                is_public: public,
                owner_id: "00000000-0000-0000-0000-000000000001".to_string(),
                tenant_id: "00000000-0000-0000-0000-000000000001".to_string(),
                created_at: now,
                updated_at: now,
            };

            store.tools.lock().unwrap().insert(id.to_string(), tool);
        }

        store
    }
}

/// Tool state
#[derive(Clone)]
pub struct ToolState {
    pub store: Arc<ToolStore>,
}

impl ToolState {
    pub fn new() -> Self {
        Self {
            store: Arc::new(ToolStore::default()),
        }
    }
}

impl Default for ToolState {
    fn default() -> Self {
        Self::new()
    }
}

/// List tools handler
pub async fn list_tools(state: web::Data<ToolState>) -> impl Responder {
    let tools = state.store.tools.lock().unwrap();
    let tools: Vec<ToolResponse> = tools
        .values()
        .map(|t| ToolResponse {
            id: t.id.clone(),
            name: t.name.clone(),
            description: t.description.clone(),
            category: t.category.clone(),
            schema: t.schema.clone(),
            handler: t.handler.clone(),
            is_public: t.is_public,
            owner_id: t.owner_id.clone(),
            tenant_id: t.tenant_id.clone(),
            created_at: chrono::DateTime::from_timestamp(t.created_at, 0)
                .unwrap_or_default()
                .to_rfc3339(),
            updated_at: chrono::DateTime::from_timestamp(t.updated_at, 0)
                .unwrap_or_default()
                .to_rfc3339(),
        })
        .collect();
    let total = tools.len() as u32;
    
    HttpResponse::Ok().json(ApiResponse::<Vec<ToolResponse>>::success_with_meta(
        tools,
        PaginationMeta {
            page: 1,
            per_page: 20,
            total,
        },
    ))
}
/// Get tool handler
pub async fn get_tool(id: web::Path<String>, state: web::Data<ToolState>) -> impl Responder {
    let tools = state.store.tools.lock().unwrap();

    if let Some(tool) = tools.get(&id.to_string()) {
        let response = ToolResponse {
            id: tool.id.clone(),
            name: tool.name.clone(),
            description: tool.description.clone(),
            category: tool.category.clone(),
            schema: tool.schema.clone(),
            handler: tool.handler.clone(),
            is_public: tool.is_public,
            owner_id: tool.owner_id.clone(),
            tenant_id: tool.tenant_id.clone(),
            created_at: chrono::DateTime::from_timestamp(tool.created_at, 0)
                .unwrap_or_default()
                .to_rfc3339(),
            updated_at: chrono::DateTime::from_timestamp(tool.updated_at, 0)
                .unwrap_or_default()
                .to_rfc3339(),
        };

        HttpResponse::Ok().json(ApiResponse::<ToolResponse>::success(response))
    } else {
        HttpResponse::NotFound().json(ApiResponse::<()>::error("NOT_FOUND", "Tool not found"))
    }
}

/// Create tool handler
pub async fn create_tool(
    body: web::Json<CreateToolRequest>,
    state: web::Data<ToolState>,
) -> impl Responder {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "VALIDATION_ERROR",
            &errors.to_string(),
        ));
    }

    let now = Utc::now().timestamp();
    let id = Uuid::new_v4().to_string();

    let tool = StoredTool {
        id: id.clone(),
        name: body.name.clone(),
        description: body.description.clone(),
        category: body.category.clone(),
        schema: body.schema.clone(),
        handler: body
            .handler
            .clone()
            .unwrap_or(serde_json::json!({"type": "function"})),
        is_public: body.is_public.unwrap_or(false),
        owner_id: "00000000-0000-0000-0000-000000000001".to_string(),
        tenant_id: "00000000-0000-0000-0000-000000000001".to_string(),
        created_at: now,
        updated_at: now,
    };

    {
        let mut tools = state.store.tools.lock().unwrap();
        tools.insert(id.clone(), tool.clone());
    }

    let response = ToolResponse {
        id: tool.id,
        name: tool.name,
        description: tool.description,
        category: tool.category,
        schema: tool.schema,
        handler: tool.handler,
        is_public: tool.is_public,
        owner_id: tool.owner_id,
        tenant_id: tool.tenant_id,
        created_at: chrono::DateTime::from_timestamp(tool.created_at, 0)
            .unwrap_or_default()
            .to_rfc3339(),
        updated_at: chrono::DateTime::from_timestamp(tool.updated_at, 0)
            .unwrap_or_default()
            .to_rfc3339(),
    };

    HttpResponse::Ok().json(ApiResponse::<ToolResponse>::success(response))
}

/// Update tool handler
pub async fn update_tool(
    id: web::Path<String>,
    body: web::Json<UpdateToolRequest>,
    state: web::Data<ToolState>,
) -> impl Responder {
    let mut tools = state.store.tools.lock().unwrap();

    if let Some(tool) = tools.get_mut(&id.to_string()) {
        if let Some(name) = &body.name {
            tool.name = name.clone();
        }
        if let Some(description) = &body.description {
            tool.description = description.clone();
        }
        if let Some(category) = &body.category {
            tool.category = category.clone();
        }
        if let Some(schema) = &body.schema {
            tool.schema = schema.clone();
        }
        if let Some(handler) = &body.handler {
            tool.handler = handler.clone();
        }
        if let Some(is_public) = body.is_public {
            tool.is_public = is_public;
        }
        tool.updated_at = Utc::now().timestamp();

        let response = ToolResponse {
            id: tool.id.clone(),
            name: tool.name.clone(),
            description: tool.description.clone(),
            category: tool.category.clone(),
            schema: tool.schema.clone(),
            handler: tool.handler.clone(),
            is_public: tool.is_public,
            owner_id: tool.owner_id.clone(),
            tenant_id: tool.tenant_id.clone(),
            created_at: chrono::DateTime::from_timestamp(tool.created_at, 0)
                .unwrap_or_default()
                .to_rfc3339(),
            updated_at: chrono::DateTime::from_timestamp(tool.updated_at, 0)
                .unwrap_or_default()
                .to_rfc3339(),
        };

        HttpResponse::Ok().json(ApiResponse::<ToolResponse>::success(response))
    } else {
        HttpResponse::NotFound().json(ApiResponse::<()>::error("NOT_FOUND", "Tool not found"))
    }
}

/// Delete tool handler
pub async fn delete_tool(id: web::Path<String>, state: web::Data<ToolState>) -> impl Responder {
    let mut tools = state.store.tools.lock().unwrap();

    if tools.remove(&id.to_string()).is_some() {
        HttpResponse::Ok().json(ApiResponse::<serde_json::Value>::success(
            serde_json::json!({ "message": "Tool deleted successfully" }),
        ))
    } else {
        HttpResponse::NotFound().json(ApiResponse::<()>::error("NOT_FOUND", "Tool not found"))
    }
}
