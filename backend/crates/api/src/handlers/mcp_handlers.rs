//! MCP endpoint handlers
//! Handles Model Context Protocol requests for external AI agents
//!
//! 架构说明:
//! - MCP Server 端点统一: /mcp
//! - 认证方式: 通过 Authorization header 传递 API Key
//! - 每个 API Key 关联一个租户,返回该租户的工具列表
//! - 支持 streamable HTTP 模式

use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde_json::Value;

use crate::handlers::api_key_handlers::{validate_api_key, ApiKeyState};
use crate::handlers::tool_handlers::ToolStore;
use service_tool::mcp::{
    self, build_initialize_result, McpRequest, McpResponse, ToolCallParams, ToolsListResult,
};

/// MCP endpoint state
#[derive(Clone)]
pub struct McpState {
    pub tool_store: std::sync::Arc<ToolStore>,
    pub api_key_store: std::sync::Arc<ApiKeyState>,
}

impl McpState {
    pub fn new(
        tool_store: std::sync::Arc<ToolStore>,
        api_key_store: std::sync::Arc<ApiKeyState>,
    ) -> Self {
        Self {
            tool_store,
            api_key_store,
        }
    }
}

/// Main MCP endpoint handler
pub async fn mcp_endpoint(
    req: HttpRequest,
    body: web::Json<McpRequest>,
    state: web::Data<McpState>,
) -> impl Responder {
    let request = body.into_inner();
    let id = request.id;

    // 从 header 获取 API Key 进行认证
    let api_key = extract_api_key(&req);

    // 如果提供了 API Key,验证并获取租户信息
    let tenant_id = if let Some(key) = api_key {
        match validate_api_key(&key, &state.api_key_store).await {
            Ok(key_data) => Some(key_data.tenant_id),
            Err(_) => {
                return HttpResponse::Ok().json(McpResponse::error(
                    id,
                    -32001,
                    "Invalid or expired API key".to_string(),
                    None,
                ));
            }
        }
    } else {
        None
    };

    match request.method.as_str() {
        "initialize" => handle_initialize(id).await,
        "tools/list" => handle_tools_list(id, &state, tenant_id).await,
        "tools/call" => handle_tools_call(id, &state, tenant_id, request.params).await,
        "resources/list" => handle_resources_list(id).await,
        "resources/read" => handle_resources_read(id).await,
        "prompts/list" => handle_prompts_list(id).await,
        "ping" => handle_ping(id).await,
        "endpoint" => handle_sse(id).await,
        _ => HttpResponse::Ok().json(McpResponse::method_not_found(id)),
    }
}

/// 从请求头提取 API Key
fn extract_api_key(req: &HttpRequest) -> Option<String> {
    if let Some(auth) = req.headers().get("authorization") {
        if let Ok(auth_str) = auth.to_str() {
            if auth_str.starts_with("Bearer ") {
                return Some(auth_str[7..].to_string());
            }
            return Some(auth_str.to_string());
        }
    }

    if let Some(key) = req.headers().get("x-api-key") {
        if let Ok(key_str) = key.to_str() {
            return Some(key_str.to_string());
        }
    }

    None
}

async fn handle_initialize(id: Value) -> HttpResponse {
    let result = build_initialize_result();
    HttpResponse::Ok().json(McpResponse::success(
        id,
        serde_json::to_value(result).unwrap(),
    ))
}

async fn handle_tools_list(id: Value, state: &McpState, tenant_id: Option<String>) -> HttpResponse {
    let all_tools = state.tool_store.get_all();

    let mcp_tools: Vec<service_tool::mcp::McpTool> = all_tools
        .values()
        .filter(|tool| {
            if let Some(ref tid) = tenant_id {
                tool.tenant_id == *tid
            } else {
                tool.is_public
            }
        })
        .map(|tool| service_tool::mcp::McpTool {
            name: tool.name.clone(),
            description: tool.description.clone(),
            input_schema: tool.schema.clone(),
        })
        .collect();

    let result = ToolsListResult { tools: mcp_tools };
    HttpResponse::Ok().json(McpResponse::success(
        id,
        serde_json::to_value(result).unwrap(),
    ))
}

async fn handle_tools_call(
    id: Value,
    state: &McpState,
    tenant_id: Option<String>,
    params: Option<Value>,
) -> HttpResponse {
    let params = match params {
        Some(p) => p,
        None => {
            return HttpResponse::Ok().json(McpResponse::invalid_params(id, "Missing params"));
        }
    };

    let call_params: ToolCallParams = match serde_json::from_value(params) {
        Ok(p) => p,
        Err(e) => {
            return HttpResponse::Ok().json(McpResponse::invalid_params(
                id,
                &format!("Invalid params: {}", e),
            ));
        }
    };

    let tool_name = call_params.name;
    let arguments = call_params.arguments;

    let all_tools = state.tool_store.get_all();

    // Find tool by name (MCP uses tool names, not UUIDs)
    let tool = match all_tools.values().find(|t| t.name == tool_name) {
        Some(t) => {
            if let Some(ref tid) = tenant_id {
                if t.tenant_id != *tid {
                    return HttpResponse::Ok().json(McpResponse::error(
                        id,
                        -32001,
                        format!("Tool '{}' not found or access denied", tool_name),
                        None,
                    ));
                }
            } else if !t.is_public {
                return HttpResponse::Ok().json(McpResponse::error(
                    id,
                    -32001,
                    "Authentication required to access this tool".to_string(),
                    None,
                ));
            }
            t
        }
        None => {
            return HttpResponse::Ok().json(McpResponse::error(
                id,
                -32601,
                format!("Tool '{}' not found", tool_name),
                None,
            ));
        }
    };

    let schema: Value = tool.schema.clone();
    if let Err(e) = mcp::validate_arguments(&schema, &arguments) {
        return HttpResponse::Ok().json(McpResponse::error(id, -32602, e, None));
    }

    let result_text = format!("Tool '{}' executed with args: {:?}", tool_name, arguments);

    let result = service_tool::mcp::ToolCallResult {
        content: vec![service_tool::mcp::ToolContent {
            content_type: "text".to_string(),
            text: result_text,
        }],
    };

    HttpResponse::Ok().json(McpResponse::success(
        id,
        serde_json::to_value(result).unwrap(),
    ))
}

async fn handle_resources_list(id: Value) -> HttpResponse {
    let result = serde_json::json!({ "resources": [] });
    HttpResponse::Ok().json(McpResponse::success(id, result))
}

async fn handle_resources_read(id: Value) -> HttpResponse {
    HttpResponse::Ok().json(McpResponse::method_not_found(id))
}

async fn handle_prompts_list(id: Value) -> HttpResponse {
    let result = serde_json::json!({ "prompts": [] });
    HttpResponse::Ok().json(McpResponse::success(id, result))
}

async fn handle_ping(id: Value) -> HttpResponse {
    HttpResponse::Ok().json(McpResponse::success(id, serde_json::json!({})))
}

async fn handle_sse(id: Value) -> HttpResponse {
    HttpResponse::Ok().json(McpResponse::success(
        id,
        serde_json::json!({ "message": "Use POST /mcp for tool calls" }),
    ))
}
