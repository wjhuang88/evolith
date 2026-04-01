//! Tool handlers - database-backed repository implementation

use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use validator::Validate;

use crate::dto::tool_dto::{CreateToolRequest, ToolListResponse, ToolResponse, UpdateToolRequest};
use crate::middleware::auth::AuthenticatedUser;
use crate::state::AppState;
use domain::tool::{HandlerConfig, HandlerType, NewTool, ToolFilter, UpdateTool, Visibility};

/// List tools for current tenant
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

    // Fetch tools and count
    let tools = match state.tool_repo.find_all(filter.clone()).await {
        Ok(t) => t,
        Err(e) => {
            return HttpResponse::InternalServerError().json(
                crate::dto::common::ApiResponse::<()>::error("DATABASE_ERROR", &e.to_string()),
            );
        }
    };

    let total = match state.tool_repo.count(&filter).await {
        Ok(c) => c,
        Err(e) => {
            return HttpResponse::InternalServerError().json(
                crate::dto::common::ApiResponse::<()>::error("DATABASE_ERROR", &e.to_string()),
            );
        }
    };

    let responses: Vec<ToolResponse> = tools.into_iter().map(ToolResponse::from).collect();
    let list_response = ToolListResponse::new(responses, page, per_page, total);

    HttpResponse::Ok().json(crate::dto::common::ApiResponse::success(list_response))
}

/// Query parameters for listing tools
#[derive(Debug, serde::Deserialize)]
pub struct ListToolsQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub search: Option<String>,
}

/// Get a single tool by ID
pub async fn get_tool(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> impl Responder {
    let tool_id = path.into_inner();

    let tool = match state.tool_repo.find_by_id(tool_id).await {
        Ok(Some(t)) => t,
        Ok(None) => {
            return HttpResponse::NotFound().json(crate::dto::common::ApiResponse::<()>::error(
                "NOT_FOUND",
                "Tool not found",
            ));
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(
                crate::dto::common::ApiResponse::<()>::error("DATABASE_ERROR", &e.to_string()),
            );
        }
    };

    // Verify tenant access
    if tool.tenant_id != user.tenant_id {
        return HttpResponse::NotFound().json(crate::dto::common::ApiResponse::<()>::error(
            "NOT_FOUND",
            "Tool not found",
        ));
    }

    HttpResponse::Ok().json(crate::dto::common::ApiResponse::success(
        ToolResponse::from(tool),
    ))
}

/// Create a new tool
pub async fn create_tool(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    body: web::Json<CreateToolRequest>,
) -> impl Responder {
    // Validate request
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(crate::dto::common::ApiResponse::<()>::error(
            "VALIDATION_ERROR",
            &errors.to_string(),
        ));
    }

    // Parse handler type
    let handler_type = match body.handler_type.to_lowercase().as_str() {
        "http" => HandlerType::Http,
        _ => HandlerType::Function,
    };

    // Build handler config
    let handler = HandlerConfig {
        handler_type,
        url: body.handler_url.clone(),
        method: body.handler_method.clone(),
        timeout: body.handler_timeout,
    };

    // Build new tool
    let new_tool = NewTool {
        name: body.name.clone(),
        description: body.description.clone(),
        input_schema: body.input_schema.clone(),
        output_schema: None,
        handler,
        visibility: if body.is_public.unwrap_or(false) {
            Some(Visibility::Public)
        } else {
            Some(Visibility::Private)
        },
    };

    // Create tool via repository
    let tool = match state
        .tool_repo
        .create(new_tool, user.user_id, user.tenant_id)
        .await
    {
        Ok(t) => t,
        Err(e) => {
            let (status, code, msg) = match e {
                common::error::AppError::ValidationError(msg) => (
                    actix_web::http::StatusCode::BAD_REQUEST,
                    "VALIDATION_ERROR",
                    msg,
                ),
                common::error::AppError::DatabaseError(msg) => (
                    actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "DATABASE_ERROR",
                    msg,
                ),
                other => (
                    actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_ERROR",
                    other.to_string(),
                ),
            };
            return HttpResponse::build(status)
                .json(crate::dto::common::ApiResponse::<()>::error(code, &msg));
        }
    };

    HttpResponse::Created().json(crate::dto::common::ApiResponse::success(
        ToolResponse::from(tool),
    ))
}

/// Update an existing tool
pub async fn update_tool(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    body: web::Json<UpdateToolRequest>,
) -> impl Responder {
    let tool_id = path.into_inner();

    // Validate request
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(crate::dto::common::ApiResponse::<()>::error(
            "VALIDATION_ERROR",
            &errors.to_string(),
        ));
    }

    // Check tool exists and belongs to tenant
    let existing = match state.tool_repo.find_by_id(tool_id).await {
        Ok(Some(t)) => t,
        Ok(None) => {
            return HttpResponse::NotFound().json(crate::dto::common::ApiResponse::<()>::error(
                "NOT_FOUND",
                "Tool not found",
            ));
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(
                crate::dto::common::ApiResponse::<()>::error("DATABASE_ERROR", &e.to_string()),
            );
        }
    };

    if existing.tenant_id != user.tenant_id {
        return HttpResponse::NotFound().json(crate::dto::common::ApiResponse::<()>::error(
            "NOT_FOUND",
            "Tool not found",
        ));
    }

    // Build update struct
    let update = UpdateTool {
        name: body.name.clone(),
        description: body.description.clone(),
        input_schema: body.input_schema.clone(),
        output_schema: None,
        handler: if body.handler_type.is_some()
            || body.handler_url.is_some()
            || body.handler_method.is_some()
            || body.handler_timeout.is_some()
        {
            Some(HandlerConfig {
                handler_type: body
                    .handler_type
                    .as_ref()
                    .map(|t| match t.to_lowercase().as_str() {
                        "http" => HandlerType::Http,
                        _ => HandlerType::Function,
                    })
                    .unwrap_or(existing.handler.handler_type),
                url: body.handler_url.clone().or(existing.handler.url),
                method: body.handler_method.clone().or(existing.handler.method),
                timeout: body.handler_timeout.or(existing.handler.timeout),
            })
        } else {
            None
        },
        visibility: body.is_public.map(|pub_flag| {
            if pub_flag {
                Visibility::Public
            } else {
                Visibility::Private
            }
        }),
    };

    // Update via repository
    let tool = match state.tool_repo.update(tool_id, update).await {
        Ok(t) => t,
        Err(e) => {
            let (status, code, msg) = match e {
                common::error::AppError::NotFoundError(msg) => {
                    (actix_web::http::StatusCode::NOT_FOUND, "NOT_FOUND", msg)
                }
                common::error::AppError::ValidationError(msg) => (
                    actix_web::http::StatusCode::BAD_REQUEST,
                    "VALIDATION_ERROR",
                    msg,
                ),
                common::error::AppError::DatabaseError(msg) => (
                    actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "DATABASE_ERROR",
                    msg,
                ),
                other => (
                    actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_ERROR",
                    other.to_string(),
                ),
            };
            return HttpResponse::build(status)
                .json(crate::dto::common::ApiResponse::<()>::error(code, &msg));
        }
    };

    HttpResponse::Ok().json(crate::dto::common::ApiResponse::success(
        ToolResponse::from(tool),
    ))
}

/// Delete a tool
pub async fn delete_tool(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> impl Responder {
    let tool_id = path.into_inner();

    // Check tool exists and belongs to tenant
    let existing = match state.tool_repo.find_by_id(tool_id).await {
        Ok(Some(t)) => t,
        Ok(None) => {
            return HttpResponse::NotFound().json(crate::dto::common::ApiResponse::<()>::error(
                "NOT_FOUND",
                "Tool not found",
            ));
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(
                crate::dto::common::ApiResponse::<()>::error("DATABASE_ERROR", &e.to_string()),
            );
        }
    };

    if existing.tenant_id != user.tenant_id {
        return HttpResponse::NotFound().json(crate::dto::common::ApiResponse::<()>::error(
            "NOT_FOUND",
            "Tool not found",
        ));
    }

    // Delete via repository
    if let Err(e) = state.tool_repo.delete(tool_id).await {
        return HttpResponse::InternalServerError().json(
            crate::dto::common::ApiResponse::<()>::error("DATABASE_ERROR", &e.to_string()),
        );
    }

    HttpResponse::Ok().json(crate::dto::common::ApiResponse::success(
        serde_json::json!({
            "message": "Tool deleted successfully"
        }),
    ))
}
