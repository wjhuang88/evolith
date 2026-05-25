//! MCP endpoint handlers
//! Handles Model Context Protocol requests for external AI agents
//!
//! Architecture:
//! - MCP Server endpoint: /mcp
//! - Authentication: API Key via Authorization header
//! - Each API Key is associated with a tenant, returns that tenant's tools
//! - Supports streamable HTTP mode

use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde_json::Value;
use uuid::Uuid;

use crate::dto::common::ApiResponse;
use crate::handlers::api_key_handlers::validate_api_key;
use crate::state::AppState;
use domain::tool::{HandlerType, Visibility};
use domain::ToolFilter;
use service_tool::executor::ExecuteRequest;
use service_tool::mcp::{
    self, build_initialize_result, McpRequest, McpResponse, ToolCallParams, ToolsListResult,
};

/// Main MCP endpoint handler
pub async fn handle_mcp_request(
    req: HttpRequest,
    body: web::Json<McpRequest>,
    state: web::Data<AppState>,
) -> impl Responder {
    let request = body.into_inner();
    let id = request.id;

    // Extract API Key from header for authentication
    let api_key = extract_api_key(&req);

    // If API Key is provided, validate and get tenant info
    let tenant_id = if let Some(key) = api_key {
        match validate_api_key(&key, &state).await {
            Ok(key_data) => {
                // Update last_used_at for the API key
                let _ = state.api_key_repo.update_last_used(key_data.id).await;
                Some(key_data.tenant_id)
            }
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
        _ => HttpResponse::Ok().json(McpResponse::method_not_found(id)),
    }
}

/// List available tools (public endpoint for discovery)
pub async fn list_available_tools(req: HttpRequest, state: web::Data<AppState>) -> impl Responder {
    // Extract API Key from header
    let api_key = extract_api_key(&req);

    let tenant_id = if let Some(key) = api_key {
        match validate_api_key(&key, &state).await {
            Ok(key_data) => Some(key_data.tenant_id),
            Err(_) => {
                return HttpResponse::Unauthorized().json(ApiResponse::<()>::error(
                    "UNAUTHORIZED",
                    "Invalid or expired API key",
                ));
            }
        }
    } else {
        None
    };

    // Build filter based on tenant
    let filter = ToolFilter {
        tenant_id,
        visibility: if tenant_id.is_none() {
            Some(Visibility::Public)
        } else {
            None
        },
        ..Default::default()
    };

    match state.tool_repo.find_all(filter).await {
        Ok(tools) => {
            let mcp_tools: Vec<service_tool::mcp::McpTool> = tools
                .into_iter()
                .map(|tool| service_tool::mcp::McpTool {
                    name: tool.name,
                    description: tool.description,
                    input_schema: tool.input_schema,
                })
                .collect();

            HttpResponse::Ok().json(ApiResponse::<ToolsListResult>::success(ToolsListResult {
                tools: mcp_tools,
            }))
        }
        Err(e) => {
            tracing::error!("Failed to list tools: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to list tools",
            ))
        }
    }
}

/// Extract API Key from request headers
fn extract_api_key(req: &HttpRequest) -> Option<String> {
    // Try Authorization header (Bearer token)
    if let Some(auth) = req.headers().get("authorization") {
        if let Ok(auth_str) = auth.to_str() {
            if let Some(stripped) = auth_str.strip_prefix("Bearer ") {
                return Some(stripped.to_string());
            }
            return Some(auth_str.to_string());
        }
    }

    // Try X-API-Key header
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
        serde_json::to_value(result).unwrap_or_default(),
    ))
}

async fn handle_tools_list(id: Value, state: &AppState, tenant_id: Option<Uuid>) -> HttpResponse {
    // Build filter based on tenant
    let filter = ToolFilter {
        tenant_id,
        visibility: if tenant_id.is_none() {
            Some(Visibility::Public)
        } else {
            None
        },
        ..Default::default()
    };

    match state.tool_repo.find_all(filter).await {
        Ok(tools) => {
            let mcp_tools: Vec<service_tool::mcp::McpTool> = tools
                .into_iter()
                .map(|tool| service_tool::mcp::McpTool {
                    name: tool.name,
                    description: tool.description,
                    input_schema: tool.input_schema,
                })
                .collect();

            let result = ToolsListResult { tools: mcp_tools };
            HttpResponse::Ok().json(McpResponse::success(
                id,
                serde_json::to_value(result).unwrap_or_default(),
            ))
        }
        Err(e) => {
            tracing::error!("Failed to list tools: {}", e);
            HttpResponse::Ok().json(McpResponse::error(
                id,
                -32603,
                "Internal server error".to_string(),
                None,
            ))
        }
    }
}

async fn handle_tools_call(
    id: Value,
    state: &AppState,
    tenant_id: Option<Uuid>,
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

    // Find tool by name
    match state.tool_repo.find_by_name(&tool_name).await {
        Ok(Some(tool)) => {
            // Check access permissions
            if let Some(tid) = tenant_id {
                if tool.tenant_id != tid {
                    return HttpResponse::Ok().json(McpResponse::error(
                        id,
                        -32001,
                        format!("Tool '{}' not found or access denied", tool_name),
                        None,
                    ));
                }
            } else if tool.visibility != Visibility::Public {
                return HttpResponse::Ok().json(McpResponse::error(
                    id,
                    -32001,
                    "Authentication required to access this tool".to_string(),
                    None,
                ));
            }

            // Validate arguments against schema
            let schema: Value = tool.input_schema.clone();
            if let Err(e) = mcp::validate_arguments(&schema, &arguments) {
                return HttpResponse::Ok().json(McpResponse::error(id, -32602, e, None));
            }

            // Dispatch to appropriate executor based on handler type
            match tool.handler.handler_type {
                HandlerType::Http => {
                    let url = match &tool.handler.url {
                        Some(u) => u.clone(),
                        None => {
                            return HttpResponse::Ok().json(McpResponse::error(
                                id,
                                -32603,
                                "HTTP tool missing URL configuration".to_string(),
                                None,
                            ));
                        }
                    };

                    let method = tool.handler.method.clone().unwrap_or_else(|| "POST".to_string());
                    let timeout_ms = tool.handler.timeout.unwrap_or(30000);

                    let exec_request = ExecuteRequest {
                        tool_id: tool.id.to_string(),
                        parameters: arguments,
                        url,
                        method,
                        timeout_ms,
                    };

                    match state.tool_executor.execute(exec_request).await {
                        Ok(exec_response) => {
                            let result_text = if let Some(ref error) = exec_response.error {
                                format!(
                                    "Status: {}\nError: {}\nResponse: {}",
                                    exec_response.status,
                                    error,
                                    serde_json::to_string_pretty(&exec_response.result)
                                        .unwrap_or_default()
                                )
                            } else {
                                serde_json::to_string_pretty(&exec_response.result)
                                    .unwrap_or_default()
                            };

                            let result = service_tool::mcp::ToolCallResult {
                                content: vec![service_tool::mcp::ToolContent {
                                    content_type: "text".to_string(),
                                    text: result_text,
                                }],
                            };

                            HttpResponse::Ok().json(McpResponse::success(
                                id,
                                serde_json::to_value(result).unwrap_or_default(),
                            ))
                        }
                        Err(e) => {
                            tracing::error!("Tool execution failed: {}", e);
                            HttpResponse::Ok().json(McpResponse::error(
                                id,
                                -32603,
                                format!("Tool execution failed: {}", e),
                                None,
                            ))
                        }
                    }
                }
                HandlerType::Function => {
                    HttpResponse::Ok().json(McpResponse::error(
                        id,
                        -32601,
                        format!("Function-type tool execution not yet supported: {}", tool_name),
                        None,
                    ))
                }
            }
        }
        Ok(None) => HttpResponse::Ok().json(McpResponse::error(
            id,
            -32601,
            format!("Tool '{}' not found", tool_name),
            None,
        )),
        Err(e) => {
            tracing::error!("Failed to find tool: {}", e);
            HttpResponse::Ok().json(McpResponse::error(
                id,
                -32603,
                "Internal server error".to_string(),
                None,
            ))
        }
    }
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
