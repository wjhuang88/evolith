//! Tool handlers - database-backed repository implementation.

use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use chrono::Utc;
use uuid::Uuid;
use validator::Validate;

use crate::dto::common::ApiResponse;
use crate::dto::tool_dto::{CreateToolRequest, ToolListResponse, ToolResponse, UpdateToolRequest};
use crate::middleware::auth::AuthenticatedUser;
use crate::state::AppState;
use domain::api_key::ApiKey;
use domain::audit::AuditLog;
use domain::tool::{HandlerConfig, HandlerType, NewTool, ToolFilter, UpdateTool, Visibility};
use service_tool::{EgressPolicy, SafeHttpClient};

#[derive(Debug, Clone, Copy)]
enum ToolManagementAction {
    Create,
    Update,
}

impl ToolManagementAction {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Create => "create",
            Self::Update => "update",
        }
    }
}

async fn authorize_tool_management(
    req: &HttpRequest,
    user: &AuthenticatedUser,
    action: ToolManagementAction,
    target_tool_id: Option<Uuid>,
    state: &AppState,
) -> Result<(), HttpResponse> {
    let authenticated_with_api_key = req.extensions().get::<ApiKey>().is_some();
    if !authenticated_with_api_key && user.is_admin() {
        return Ok(());
    }

    let reason = if authenticated_with_api_key {
        "api_key_authentication"
    } else {
        "insufficient_tenant_role"
    };
    persist_tool_audit(
        req,
        state,
        user.tenant_id,
        user.user_id,
        "tool.management.denied",
        target_tool_id,
        serde_json::json!({
            "operation": action.as_str(),
            "reason": reason,
            "tenant_role": &user.tenant_role,
        }),
    )
    .await;

    Err(HttpResponse::Forbidden().json(ApiResponse::<()>::error(
        "FORBIDDEN",
        "Tool management requires tenant owner or admin JWT authentication",
    )))
}

async fn persist_tool_audit(
    req: &HttpRequest,
    state: &AppState,
    tenant_id: Uuid,
    user_id: Uuid,
    action: &str,
    tool_id: Option<Uuid>,
    details: serde_json::Value,
) {
    let audit_log = AuditLog {
        id: Uuid::new_v4(),
        tenant_id: Some(tenant_id),
        user_id: Some(user_id),
        action: action.to_string(),
        resource_type: Some("tool".to_string()),
        resource_id: tool_id.map(|id| id.to_string()),
        details,
        ip_address: req
            .connection_info()
            .realip_remote_addr()
            .map(str::to_string),
        user_agent: req
            .headers()
            .get(actix_web::http::header::USER_AGENT)
            .and_then(|value| value.to_str().ok())
            .map(str::to_string),
        created_at: Utc::now(),
    };

    if let Err(error) = state.audit_repo.create(audit_log).await {
        tracing::error!(
            audit_action = action,
            tenant_id = %tenant_id,
            user_id = %user_id,
            "Failed to persist Tool audit event: {}",
            error
        );
    }
}

async fn audit_invalid_configuration(
    req: &HttpRequest,
    state: &AppState,
    user: &AuthenticatedUser,
    action: ToolManagementAction,
    tool_id: Option<Uuid>,
) {
    persist_tool_audit(
        req,
        state,
        user.tenant_id,
        user.user_id,
        "tool.configuration.denied",
        tool_id,
        serde_json::json!({
            "operation": action.as_str(),
            "reason": "invalid_or_disallowed_handler",
        }),
    )
    .await;
}

fn validation_error(message: &str) -> HttpResponse {
    HttpResponse::BadRequest().json(ApiResponse::<()>::error("VALIDATION_ERROR", message))
}

fn parse_handler_type(raw: &str) -> Result<HandlerType, HttpResponse> {
    match raw.to_ascii_lowercase().as_str() {
        "http" => Ok(HandlerType::Http),
        "function" => Ok(HandlerType::Function),
        _ => Err(validation_error("Unsupported Tool handler type")),
    }
}

async fn validate_handler(handler: &HandlerConfig) -> Result<(), &'static str> {
    if let Some(timeout) = handler.timeout {
        if timeout == 0 || timeout > 30_000 {
            return Err("Tool handler timeout must be between 1 and 30000 milliseconds");
        }
    }

    if let Some(method) = handler.method.as_deref() {
        if !matches!(
            method.to_ascii_uppercase().as_str(),
            "GET" | "POST" | "PUT" | "DELETE" | "PATCH" | "HEAD" | "OPTIONS"
        ) {
            return Err("Unsupported HTTP method");
        }
    }

    if matches!(handler.handler_type, HandlerType::Http) {
        let url = handler
            .url
            .as_deref()
            .ok_or("HTTP Tool handler URL is required")?;
        let client = SafeHttpClient::new(EgressPolicy::default());
        client
            .validate_target(url)
            .await
            .map_err(|_| "HTTP Tool handler URL is not allowed")?;
    }

    Ok(())
}

pub async fn list_tools(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    query: web::Query<ListToolsQuery>,
) -> impl Responder {
    let filter = ToolFilter {
        tenant_id: Some(user.tenant_id),
        search: query.search.clone(),
        visibility: None,
        owner_id: None,
        page: Some(query.page.unwrap_or(1)),
        per_page: Some(query.per_page.unwrap_or(20)),
    };

    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);
    let tools = match state.tool_repo.find_all(filter.clone()).await {
        Ok(tools) => tools,
        Err(error) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "DATABASE_ERROR",
                &error.to_string(),
            ));
        }
    };
    let total = match state.tool_repo.count(&filter).await {
        Ok(total) => total,
        Err(error) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "DATABASE_ERROR",
                &error.to_string(),
            ));
        }
    };

    let responses = tools.into_iter().map(ToolResponse::from).collect();
    HttpResponse::Ok().json(ApiResponse::success(ToolListResponse::new(
        responses, page, per_page, total,
    )))
}

#[derive(Debug, serde::Deserialize)]
pub struct ListToolsQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub search: Option<String>,
}

pub async fn get_tool(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> impl Responder {
    let tool = match state.tool_repo.find_by_id(path.into_inner()).await {
        Ok(Some(tool)) if tool.tenant_id == user.tenant_id => tool,
        Ok(Some(_)) | Ok(None) => {
            return HttpResponse::NotFound()
                .json(ApiResponse::<()>::error("NOT_FOUND", "Tool not found"));
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "DATABASE_ERROR",
                &error.to_string(),
            ));
        }
    };

    HttpResponse::Ok().json(ApiResponse::success(ToolResponse::from(tool)))
}

/// Authorization intentionally precedes DTO deserialization.
pub async fn create_tool(
    req: HttpRequest,
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    body: web::Bytes,
) -> impl Responder {
    if let Err(response) =
        authorize_tool_management(&req, &user, ToolManagementAction::Create, None, &state).await
    {
        return response;
    }

    let body = match serde_json::from_slice::<CreateToolRequest>(&body) {
        Ok(body) => body,
        Err(_) => return validation_error("Invalid Tool request"),
    };
    if let Err(errors) = body.validate() {
        return validation_error(&errors.to_string());
    }

    let handler = HandlerConfig {
        handler_type: match parse_handler_type(&body.handler_type) {
            Ok(handler_type) => handler_type,
            Err(response) => return response,
        },
        url: body.handler_url,
        method: body.handler_method,
        timeout: body.handler_timeout,
    };
    if let Err(message) = validate_handler(&handler).await {
        audit_invalid_configuration(&req, &state, &user, ToolManagementAction::Create, None).await;
        return validation_error(message);
    }

    let new_tool = NewTool {
        name: body.name,
        description: body.description,
        input_schema: body.input_schema,
        output_schema: None,
        handler,
        visibility: Some(if body.is_public.unwrap_or(false) {
            Visibility::Public
        } else {
            Visibility::Private
        }),
    };

    let tool = match state
        .tool_repo
        .create(new_tool, user.user_id, user.tenant_id)
        .await
    {
        Ok(tool) => tool,
        Err(common::error::AppError::ValidationError(message)) => {
            return validation_error(&message);
        }
        Err(common::error::AppError::DatabaseError(message)) => {
            return HttpResponse::InternalServerError()
                .json(ApiResponse::<()>::error("DATABASE_ERROR", &message));
        }
        Err(error) => {
            tracing::error!("Failed to create Tool: {}", error);
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to create Tool",
            ));
        }
    };

    persist_tool_audit(
        &req,
        &state,
        user.tenant_id,
        user.user_id,
        "tool.created",
        Some(tool.id),
        serde_json::json!({"handler_type": &tool.handler.handler_type}),
    )
    .await;

    HttpResponse::Created().json(ApiResponse::success(ToolResponse::from(tool)))
}

/// Authorization intentionally precedes DTO deserialization and resource lookup.
pub async fn update_tool(
    req: HttpRequest,
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    body: web::Bytes,
) -> impl Responder {
    let tool_id = path.into_inner();
    if let Err(response) = authorize_tool_management(
        &req,
        &user,
        ToolManagementAction::Update,
        Some(tool_id),
        &state,
    )
    .await
    {
        return response;
    }

    let existing = match state.tool_repo.find_by_id(tool_id).await {
        Ok(Some(tool)) if tool.tenant_id == user.tenant_id => tool,
        Ok(Some(_)) | Ok(None) => {
            persist_tool_audit(
                &req,
                &state,
                user.tenant_id,
                user.user_id,
                "tool.management.denied",
                Some(tool_id),
                serde_json::json!({
                    "operation": ToolManagementAction::Update.as_str(),
                    "reason": "not_found_or_tenant_mismatch",
                }),
            )
            .await;
            return HttpResponse::NotFound().json(ApiResponse::<()>::error(
                "NOT_FOUND",
                "Tool not found or access denied",
            ));
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "DATABASE_ERROR",
                &error.to_string(),
            ));
        }
    };

    let body = match serde_json::from_slice::<UpdateToolRequest>(&body) {
        Ok(body) => body,
        Err(_) => return validation_error("Invalid Tool request"),
    };
    if let Err(errors) = body.validate() {
        return validation_error(&errors.to_string());
    }

    let handler = if body.handler_type.is_some()
        || body.handler_url.is_some()
        || body.handler_method.is_some()
        || body.handler_timeout.is_some()
    {
        let handler = HandlerConfig {
            handler_type: match body.handler_type.as_deref() {
                Some(raw) => match parse_handler_type(raw) {
                    Ok(handler_type) => handler_type,
                    Err(response) => return response,
                },
                None => existing.handler.handler_type.clone(),
            },
            url: body.handler_url.or_else(|| existing.handler.url.clone()),
            method: body
                .handler_method
                .or_else(|| existing.handler.method.clone()),
            timeout: body.handler_timeout.or(existing.handler.timeout),
        };
        if let Err(message) = validate_handler(&handler).await {
            audit_invalid_configuration(
                &req,
                &state,
                &user,
                ToolManagementAction::Update,
                Some(tool_id),
            )
            .await;
            return validation_error(message);
        }
        Some(handler)
    } else {
        None
    };

    let update = UpdateTool {
        name: body.name,
        description: body.description,
        input_schema: body.input_schema,
        output_schema: None,
        handler,
        visibility: body.is_public.map(|is_public| {
            if is_public {
                Visibility::Public
            } else {
                Visibility::Private
            }
        }),
    };

    let tool = match state.tool_repo.update(tool_id, update).await {
        Ok(tool) => tool,
        Err(common::error::AppError::NotFoundError(_)) => {
            return HttpResponse::NotFound().json(ApiResponse::<()>::error(
                "NOT_FOUND",
                "Tool not found or access denied",
            ));
        }
        Err(common::error::AppError::ValidationError(message)) => {
            return validation_error(&message);
        }
        Err(common::error::AppError::DatabaseError(message)) => {
            return HttpResponse::InternalServerError()
                .json(ApiResponse::<()>::error("DATABASE_ERROR", &message));
        }
        Err(error) => {
            tracing::error!(tool_id = %tool_id, "Failed to update Tool: {}", error);
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to update Tool",
            ));
        }
    };

    persist_tool_audit(
        &req,
        &state,
        user.tenant_id,
        user.user_id,
        "tool.updated",
        Some(tool.id),
        serde_json::json!({"handler_type": &tool.handler.handler_type}),
    )
    .await;

    HttpResponse::Ok().json(ApiResponse::success(ToolResponse::from(tool)))
}

pub async fn delete_tool(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> impl Responder {
    let tool_id = path.into_inner();
    match state.tool_repo.find_by_id(tool_id).await {
        Ok(Some(tool)) if tool.tenant_id == user.tenant_id => {}
        Ok(Some(_)) | Ok(None) => {
            return HttpResponse::NotFound()
                .json(ApiResponse::<()>::error("NOT_FOUND", "Tool not found"));
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "DATABASE_ERROR",
                &error.to_string(),
            ));
        }
    }

    if let Err(error) = state.tool_repo.delete(tool_id).await {
        return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            "DATABASE_ERROR",
            &error.to_string(),
        ));
    }

    HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({
        "message": "Tool deleted successfully"
    })))
}
