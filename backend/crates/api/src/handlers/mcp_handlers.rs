//! MCP endpoint handlers
//! Handles Model Context Protocol requests for external AI agents.

use actix_web::{web, HttpRequest, HttpResponse, Responder};
use chrono::Utc;
use serde_json::Value;
use uuid::Uuid;

use crate::dto::common::ApiResponse;
use crate::handlers::api_key_handlers::validate_api_key;
use crate::middleware::api_key_scope::api_key_allows_tool_execute;
use crate::state::AppState;
use domain::api_key::ApiKey;
use domain::audit::AuditLog;
use domain::tool::{HandlerType, Visibility};
use domain::ToolFilter;
use service_tool::executor::ExecuteRequest;
use service_tool::mcp::{
    self, build_initialize_result, McpRequest, McpResponse, ToolCallParams, ToolsListResult,
};

pub async fn handle_mcp_request(
    req: HttpRequest,
    body: web::Json<McpRequest>,
    state: web::Data<AppState>,
) -> impl Responder {
    let request = body.into_inner();
    let id = request.id;

    let validated_api_key = if let Some(key) = extract_api_key(&req) {
        match validate_api_key(&key, &state).await {
            Ok(key_data) => {
                let _ = state.api_key_repo.update_last_used(key_data.id).await;
                Some(key_data)
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

    let tenant_id = validated_api_key.as_ref().map(|key| key.tenant_id);

    match request.method.as_str() {
        "initialize" => handle_initialize(id).await,
        "tools/list" => handle_tools_list(id, &state, tenant_id).await,
        "tools/call" => {
            handle_tools_call(id, &state, validated_api_key.as_ref(), request.params).await
        }
        "resources/list" => handle_resources_list(id).await,
        "resources/read" => handle_resources_read(id).await,
        "prompts/list" => handle_prompts_list(id).await,
        "ping" => handle_ping(id).await,
        _ => HttpResponse::Ok().json(McpResponse::method_not_found(id)),
    }
}

pub async fn list_available_tools(req: HttpRequest, state: web::Data<AppState>) -> impl Responder {
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
            let mcp_tools = tools
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
        Err(error) => {
            tracing::error!("Failed to list tools: {}", error);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to list tools",
            ))
        }
    }
}

fn extract_api_key(req: &HttpRequest) -> Option<String> {
    if let Some(auth) = req.headers().get("authorization") {
        if let Ok(auth_str) = auth.to_str() {
            if let Some(stripped) = auth_str.strip_prefix("Bearer ") {
                return Some(stripped.to_string());
            }
            return Some(auth_str.to_string());
        }
    }

    req.headers()
        .get("x-api-key")
        .and_then(|key| key.to_str().ok())
        .map(str::to_string)
}

async fn persist_tool_execution_audit(
    state: &AppState,
    api_key: &ApiKey,
    tool_id: Option<Uuid>,
    action: &str,
    details: Value,
) {
    let audit_log = AuditLog {
        id: Uuid::new_v4(),
        tenant_id: Some(api_key.tenant_id),
        user_id: Some(api_key.user_id),
        action: action.to_string(),
        resource_type: Some("tool".to_string()),
        resource_id: tool_id.map(|id| id.to_string()),
        details,
        ip_address: None,
        user_agent: None,
        created_at: Utc::now(),
    };

    if let Err(error) = state.audit_repo.create(audit_log).await {
        tracing::error!(
            audit_action = action,
            tenant_id = %api_key.tenant_id,
            api_key_id = %api_key.id,
            "Failed to persist HTTP Tool audit event: {}",
            error
        );
    }
}

async fn handle_initialize(id: Value) -> HttpResponse {
    let result = build_initialize_result();
    HttpResponse::Ok().json(McpResponse::success(
        id,
        serde_json::to_value(result).unwrap_or_default(),
    ))
}

async fn handle_tools_list(id: Value, state: &AppState, tenant_id: Option<Uuid>) -> HttpResponse {
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
            let tools = tools
                .into_iter()
                .map(|tool| service_tool::mcp::McpTool {
                    name: tool.name,
                    description: tool.description,
                    input_schema: tool.input_schema,
                })
                .collect();
            HttpResponse::Ok().json(McpResponse::success(
                id,
                serde_json::to_value(ToolsListResult { tools }).unwrap_or_default(),
            ))
        }
        Err(error) => {
            tracing::error!("Failed to list tools: {}", error);
            HttpResponse::Ok().json(McpResponse::error(
                id,
                -32603,
                "Internal server error".to_string(),
                None,
            ))
        }
    }
}

fn hidden_tool_response(id: Value) -> HttpResponse {
    HttpResponse::Ok().json(McpResponse::error(
        id,
        -32001,
        "Tool not found or access denied".to_string(),
        None,
    ))
}

async fn handle_tools_call(
    id: Value,
    state: &AppState,
    api_key: Option<&ApiKey>,
    params: Option<Value>,
) -> HttpResponse {
    let api_key = match api_key {
        Some(api_key) => api_key,
        None => {
            return HttpResponse::Ok().json(McpResponse::error(
                id,
                -32001,
                "A valid API key is required to execute tools".to_string(),
                None,
            ));
        }
    };

    if !api_key_allows_tool_execute(api_key) {
        persist_tool_execution_audit(
            state,
            api_key,
            None,
            "tool.execution.denied",
            serde_json::json!({"reason": "missing_execute_capability"}),
        )
        .await;
        return HttpResponse::Ok().json(McpResponse::error(
            id,
            -32003,
            "API key lacks the execute capability".to_string(),
            None,
        ));
    }

    let params = match params {
        Some(params) => params,
        None => {
            return HttpResponse::Ok().json(McpResponse::invalid_params(id, "Missing params"));
        }
    };
    let call_params: ToolCallParams = match serde_json::from_value(params) {
        Ok(params) => params,
        Err(_) => {
            return HttpResponse::Ok().json(McpResponse::invalid_params(id, "Invalid params"));
        }
    };

    let tool_name = call_params.name;
    let arguments = call_params.arguments;
    let tool = match state.tool_repo.find_by_name(&tool_name).await {
        Ok(Some(tool)) if tool.tenant_id == api_key.tenant_id => tool,
        Ok(Some(_)) => {
            persist_tool_execution_audit(
                state,
                api_key,
                None,
                "tool.execution.denied",
                serde_json::json!({"reason": "not_found_or_access_denied"}),
            )
            .await;
            return hidden_tool_response(id);
        }
        Ok(None) => {
            persist_tool_execution_audit(
                state,
                api_key,
                None,
                "tool.execution.denied",
                serde_json::json!({"reason": "not_found_or_access_denied"}),
            )
            .await;
            return hidden_tool_response(id);
        }
        Err(error) => {
            tracing::error!("Failed to find tool: {}", error);
            return HttpResponse::Ok().json(McpResponse::error(
                id,
                -32603,
                "Internal server error".to_string(),
                None,
            ));
        }
    };

    if let Err(error) = mcp::validate_arguments(&tool.input_schema, &arguments) {
        return HttpResponse::Ok().json(McpResponse::error(id, -32602, error, None));
    }

    match tool.handler.handler_type {
        HandlerType::Http => {
            let url = match &tool.handler.url {
                Some(url) => url.clone(),
                None => {
                    persist_tool_execution_audit(
                        state,
                        api_key,
                        Some(tool.id),
                        "tool.egress.denied",
                        serde_json::json!({"reason": "missing_configuration"}),
                    )
                    .await;
                    return HttpResponse::Ok().json(McpResponse::error(
                        id,
                        -32603,
                        "HTTP tool configuration is invalid".to_string(),
                        None,
                    ));
                }
            };
            let method = tool
                .handler
                .method
                .clone()
                .unwrap_or_else(|| "POST".to_string());
            let timeout_ms = tool.handler.timeout.unwrap_or(30_000);
            let exec_request = ExecuteRequest {
                tool_id: tool.id.to_string(),
                parameters: arguments,
                url,
                method,
                timeout_ms,
            };

            match state.tool_executor.execute(exec_request).await {
                Ok(exec_response) if exec_response.error.is_none() => {
                    persist_tool_execution_audit(
                        state,
                        api_key,
                        Some(tool.id),
                        "tool.egress.succeeded",
                        serde_json::json!({"status": exec_response.status}),
                    )
                    .await;
                    let result_text = match exec_response.result {
                        Value::String(text) => text,
                        value => serde_json::to_string_pretty(&value).unwrap_or_default(),
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
                Ok(exec_response) => {
                    persist_tool_execution_audit(
                        state,
                        api_key,
                        Some(tool.id),
                        "tool.egress.failed",
                        serde_json::json!({
                            "reason": "upstream_status",
                            "status": exec_response.status,
                        }),
                    )
                    .await;
                    HttpResponse::Ok().json(McpResponse::error(
                        id,
                        -32002,
                        format!("HTTP {} error", exec_response.status),
                        Some(serde_json::json!({"status": exec_response.status})),
                    ))
                }
                Err(_) => {
                    persist_tool_execution_audit(
                        state,
                        api_key,
                        Some(tool.id),
                        "tool.egress.denied_or_failed",
                        serde_json::json!({"reason": "policy_or_network_failure"}),
                    )
                    .await;
                    tracing::warn!(
                        tenant_id = %api_key.tenant_id,
                        tool_id = %tool.id,
                        "HTTP Tool execution was rejected or failed"
                    );
                    HttpResponse::Ok().json(McpResponse::error(
                        id,
                        -32603,
                        "HTTP tool request was rejected".to_string(),
                        None,
                    ))
                }
            }
        }
        HandlerType::Function => HttpResponse::Ok().json(McpResponse::error(
            id,
            -32601,
            "Function-type tool execution is not supported".to_string(),
            None,
        )),
    }
}

async fn handle_resources_list(id: Value) -> HttpResponse {
    HttpResponse::Ok().json(McpResponse::success(
        id,
        serde_json::json!({ "resources": [] }),
    ))
}

async fn handle_resources_read(id: Value) -> HttpResponse {
    HttpResponse::Ok().json(McpResponse::method_not_found(id))
}

async fn handle_prompts_list(id: Value) -> HttpResponse {
    HttpResponse::Ok().json(McpResponse::success(
        id,
        serde_json::json!({ "prompts": [] }),
    ))
}

async fn handle_ping(id: Value) -> HttpResponse {
    HttpResponse::Ok().json(McpResponse::success(id, serde_json::json!({})))
}
