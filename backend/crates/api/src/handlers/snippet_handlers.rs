//! Snippet handlers

use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use validator::Validate;

use crate::dto::common::{ApiResponse, PaginationMeta};
use crate::dto::snippet_dto::{
    CreateSnippetRequest, SnippetReferenceResponse, SnippetResponse, UpdateSnippetRequest,
};
use crate::middleware::auth::AuthenticatedUser;
use crate::state::AppState;
use domain::snippet::{NewSnippet, SnippetFilter, Visibility};

/// List snippets handler
pub async fn list_snippets(
    query: web::Query<std::collections::HashMap<String, String>>,
    state: web::Data<AppState>,
    user: AuthenticatedUser,
) -> impl Responder {
    let language = query.get("language").cloned();
    let framework = query.get("framework").cloned();
    let search = query.get("q").cloned();

    let filter = SnippetFilter {
        tenant_id: Some(user.tenant_id),
        language,
        framework,
        search,
        ..Default::default()
    };

    match state.snippet_repo.find_all(filter).await {
        Ok(snippets) => {
            let total = snippets.len() as u32;
            let responses: Vec<SnippetResponse> =
                snippets.into_iter().map(SnippetResponse::from).collect();

            HttpResponse::Ok().json(ApiResponse::<Vec<SnippetResponse>>::success_with_meta(
                responses,
                PaginationMeta {
                    page: 1,
                    per_page: 20,
                    total,
                },
            ))
        }
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            "INTERNAL_ERROR",
            &format!("Failed to list snippets: {}", e),
        )),
    }
}

/// Get snippet handler
pub async fn get_snippet(
    id: web::Path<String>,
    state: web::Data<AppState>,
    user: AuthenticatedUser,
) -> impl Responder {
    let snippet_id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                "INVALID_ID",
                "Invalid snippet ID format",
            ));
        }
    };

    match state.snippet_repo.find_by_id(snippet_id).await {
        Ok(Some(snippet)) => {
            // Check tenant access
            if snippet.tenant_id != user.tenant_id {
                return HttpResponse::NotFound()
                    .json(ApiResponse::<()>::error("NOT_FOUND", "Snippet not found"));
            }

            HttpResponse::Ok().json(ApiResponse::<SnippetResponse>::success(
                SnippetResponse::from(snippet),
            ))
        }
        Ok(None) => HttpResponse::NotFound()
            .json(ApiResponse::<()>::error("NOT_FOUND", "Snippet not found")),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            "INTERNAL_ERROR",
            &format!("Failed to get snippet: {}", e),
        )),
    }
}

/// Create snippet handler
pub async fn create_snippet(
    body: web::Json<CreateSnippetRequest>,
    state: web::Data<AppState>,
    user: AuthenticatedUser,
) -> impl Responder {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "VALIDATION_ERROR",
            &errors.to_string(),
        ));
    }

    let new_snippet = NewSnippet {
        name: body.name.clone(),
        language: body.language.clone(),
        framework: body.framework.clone(),
        tags: body.tags.clone(),
        content: body.content.clone(),
        code: body.code.clone(),
        dependencies: body.dependencies.iter().map(|d| d.to_domain()).collect(),
        visibility: if body.is_public {
            Some(Visibility::Public)
        } else {
            Some(Visibility::Private)
        },
    };

    match state
        .snippet_repo
        .create(new_snippet, user.user_id, user.tenant_id)
        .await
    {
        Ok(snippet) => HttpResponse::Created().json(ApiResponse::<SnippetResponse>::success(
            SnippetResponse::from(snippet),
        )),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            "INTERNAL_ERROR",
            &format!("Failed to create snippet: {}", e),
        )),
    }
}

/// Update snippet handler - Returns 501 Not Implemented
pub async fn update_snippet(
    _id: web::Path<String>,
    _body: web::Json<UpdateSnippetRequest>,
    _state: web::Data<AppState>,
    _user: AuthenticatedUser,
) -> impl Responder {
    HttpResponse::NotImplemented().json(ApiResponse::<()>::error(
        "NOT_IMPLEMENTED",
        "Snippet update is not yet implemented",
    ))
}

/// Delete snippet handler
pub async fn delete_snippet(
    id: web::Path<String>,
    state: web::Data<AppState>,
    user: AuthenticatedUser,
) -> impl Responder {
    let snippet_id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                "INVALID_ID",
                "Invalid snippet ID format",
            ));
        }
    };

    // First check if snippet exists and belongs to tenant
    match state.snippet_repo.find_by_id(snippet_id).await {
        Ok(Some(snippet)) => {
            if snippet.tenant_id != user.tenant_id {
                return HttpResponse::NotFound()
                    .json(ApiResponse::<()>::error("NOT_FOUND", "Snippet not found"));
            }

            // Check ownership or admin role
            if snippet.owner_id != user.user_id && !user.is_admin() {
                return HttpResponse::Forbidden().json(ApiResponse::<()>::error(
                    "FORBIDDEN",
                    "Not authorized to delete this snippet",
                ));
            }

            match state.snippet_repo.delete(snippet_id).await {
                Ok(()) => HttpResponse::Ok().json(ApiResponse::<serde_json::Value>::success(
                    serde_json::json!({ "message": "Snippet deleted successfully" }),
                )),
                Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                    "INTERNAL_ERROR",
                    &format!("Failed to delete snippet: {}", e),
                )),
            }
        }
        Ok(None) => HttpResponse::NotFound()
            .json(ApiResponse::<()>::error("NOT_FOUND", "Snippet not found")),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            "INTERNAL_ERROR",
            &format!("Failed to get snippet: {}", e),
        )),
    }
}

/// Search snippets handler
pub async fn search_snippets(
    query: web::Query<std::collections::HashMap<String, String>>,
    state: web::Data<AppState>,
    user: AuthenticatedUser,
) -> impl Responder {
    let search_term = query.get("q").cloned();
    let language = query.get("language").cloned();
    let framework = query.get("framework").cloned();

    let filter = SnippetFilter {
        tenant_id: Some(user.tenant_id),
        search: search_term,
        language,
        framework,
        ..Default::default()
    };

    match state.snippet_repo.find_all(filter).await {
        Ok(snippets) => {
            let total = snippets.len() as u32;
            let responses: Vec<SnippetResponse> =
                snippets.into_iter().map(SnippetResponse::from).collect();

            HttpResponse::Ok().json(ApiResponse::<Vec<SnippetResponse>>::success_with_meta(
                responses,
                PaginationMeta {
                    page: 1,
                    per_page: 20,
                    total,
                },
            ))
        }
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            "INTERNAL_ERROR",
            &format!("Failed to search snippets: {}", e),
        )),
    }
}

/// Get reference handler (for LLM consumption)
pub async fn get_reference(
    id: web::Path<String>,
    state: web::Data<AppState>,
    user: AuthenticatedUser,
) -> impl Responder {
    let snippet_id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                "INVALID_ID",
                "Invalid snippet ID format",
            ));
        }
    };

    match state.snippet_repo.find_by_id(snippet_id).await {
        Ok(Some(snippet)) => {
            // Check tenant access
            if snippet.tenant_id != user.tenant_id {
                return HttpResponse::NotFound()
                    .json(ApiResponse::<()>::error("NOT_FOUND", "Snippet not found"));
            }

            HttpResponse::Ok().json(ApiResponse::<SnippetReferenceResponse>::success(
                SnippetReferenceResponse::from(snippet),
            ))
        }
        Ok(None) => HttpResponse::NotFound()
            .json(ApiResponse::<()>::error("NOT_FOUND", "Snippet not found")),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            "INTERNAL_ERROR",
            &format!("Failed to get snippet reference: {}", e),
        )),
    }
}
