//! Skill handlers using database-backed repositories

use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use validator::Validate;

use crate::dto::common::{ApiResponse, PaginationMeta};
use crate::dto::skill_dto::{
    parse_runtime, runtime_to_string, CreateSkillRequest, ExecuteSkillRequest,
    SkillExecutionResponse, SkillLoadResponse, SkillResponse, UpdateSkillRequest,
};
use crate::middleware::auth::AuthenticatedUser;
use crate::state::AppState;
use domain::skill::{Dependency, NewSkill, SkillFilter, UpdateSkill};
use domain::tool::Visibility;
use service_skill::executor::ExecuteRequest;

/// List skills handler
pub async fn list_skills(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    query: web::Query<SkillListQuery>,
) -> impl Responder {
    let filter = SkillFilter {
        tenant_id: Some(user.tenant_id),
        search: query.search.clone(),
        runtime: None, // Could parse from query if needed
        visibility: query.is_public.map(|p| {
            if p {
                Visibility::Public
            } else {
                Visibility::Private
            }
        }),
        owner_id: query
            .owner_id
            .as_ref()
            .and_then(|s| Uuid::parse_str(s).ok()),
        page: Some(query.page.unwrap_or(1)),
        per_page: Some(query.per_page.unwrap_or(20)),
    };

    match state.skill_repo.find_all(filter).await {
        Ok(skills) => {
            let total = skills.len() as u32;
            let responses: Vec<SkillResponse> =
                skills.into_iter().map(SkillResponse::from).collect();
            HttpResponse::Ok().json(ApiResponse::success_with_meta(
                responses,
                PaginationMeta {
                    page: query.page.unwrap_or(1),
                    per_page: query.per_page.unwrap_or(20),
                    total,
                },
            ))
        }
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            "INTERNAL_ERROR",
            &format!("Failed to list skills: {}", e),
        )),
    }
}

/// Query parameters for skill listing
#[derive(Debug, serde::Deserialize)]
pub struct SkillListQuery {
    pub search: Option<String>,
    pub is_public: Option<bool>,
    pub owner_id: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

/// Get skill by ID handler
pub async fn get_skill(
    path: web::Path<String>,
    state: web::Data<AppState>,
    user: AuthenticatedUser,
) -> impl Responder {
    let id = match Uuid::parse_str(&path) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                "INVALID_ID",
                "Invalid skill ID format",
            ));
        }
    };

    match state.skill_repo.find_by_id(id).await {
        Ok(Some(skill)) => {
            // Check tenant access
            if skill.tenant_id != user.tenant_id && skill.visibility != Visibility::Public {
                return HttpResponse::NotFound()
                    .json(ApiResponse::<()>::error("NOT_FOUND", "Skill not found"));
            }
            HttpResponse::Ok().json(ApiResponse::success(SkillResponse::from(skill)))
        }
        Ok(None) => {
            HttpResponse::NotFound().json(ApiResponse::<()>::error("NOT_FOUND", "Skill not found"))
        }
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            "INTERNAL_ERROR",
            &format!("Failed to get skill: {}", e),
        )),
    }
}

/// Create skill handler
pub async fn create_skill(
    body: web::Json<CreateSkillRequest>,
    state: web::Data<AppState>,
    user: AuthenticatedUser,
) -> impl Responder {
    // Validate request
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "VALIDATION_ERROR",
            &errors.to_string(),
        ));
    }

    // Parse runtime
    let runtime = match parse_runtime(&body.runtime) {
        Ok(r) => r,
        Err(e) => {
            return HttpResponse::BadRequest()
                .json(ApiResponse::<()>::error("INVALID_RUNTIME", &e));
        }
    };

    // Convert dependencies
    let dependencies: Vec<Dependency> = body.dependencies.iter().map(|d| d.to_domain()).collect();

    // Determine visibility
    let visibility = if body.is_public {
        Visibility::Public
    } else {
        Visibility::Private
    };

    let new_skill = NewSkill {
        name: body.name.clone(),
        version: body.version.clone(),
        description: body.description.clone(),
        skill_md: body.content.clone(),
        code_package_path: None,
        runtime,
        dependencies,
        visibility: Some(visibility),
    };

    match state
        .skill_repo
        .create(new_skill, user.user_id, user.tenant_id)
        .await
    {
        Ok(skill) => HttpResponse::Created().json(ApiResponse::success(SkillResponse::from(skill))),
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("UNIQUE") || msg.contains("duplicate") {
                HttpResponse::Conflict().json(ApiResponse::<()>::error(
                    "DUPLICATE",
                    "Skill with this name and version already exists",
                ))
            } else {
                HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                    "INTERNAL_ERROR",
                    &format!("Failed to create skill: {}", e),
                ))
            }
        }
    }
}

/// Update skill handler
pub async fn update_skill(
    path: web::Path<String>,
    body: web::Json<UpdateSkillRequest>,
    state: web::Data<AppState>,
    user: AuthenticatedUser,
) -> impl Responder {
    let id = match Uuid::parse_str(&path) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                "INVALID_ID",
                "Invalid skill ID format",
            ));
        }
    };

    // Check skill exists and belongs to tenant
    let existing = match state.skill_repo.find_by_id(id).await {
        Ok(Some(skill)) => skill,
        Ok(None) => {
            return HttpResponse::NotFound()
                .json(ApiResponse::<()>::error("NOT_FOUND", "Skill not found"));
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                &format!("Failed to find skill: {}", e),
            ));
        }
    };

    if existing.tenant_id != user.tenant_id {
        return HttpResponse::NotFound()
            .json(ApiResponse::<()>::error("NOT_FOUND", "Skill not found"));
    }

    // Check ownership or admin
    if existing.owner_id != user.user_id && !user.is_admin() {
        return HttpResponse::Forbidden().json(ApiResponse::<()>::error(
            "FORBIDDEN",
            "You can only update your own skills",
        ));
    }

    // Parse runtime if provided
    let runtime = if let Some(ref rt) = body.runtime {
        match parse_runtime(rt) {
            Ok(r) => Some(r),
            Err(e) => {
                return HttpResponse::BadRequest()
                    .json(ApiResponse::<()>::error("INVALID_RUNTIME", &e));
            }
        }
    } else {
        None
    };

    // Convert dependencies if provided
    let dependencies = body.dependencies.as_ref().map(|deps| {
        deps.iter().map(|d| d.to_domain()).collect()
    });

    // Determine visibility if provided
    let visibility = body.is_public.map(|pub_flag| {
        if pub_flag {
            Visibility::Public
        } else {
            Visibility::Private
        }
    });

    let update = UpdateSkill {
        name: body.name.clone(),
        version: body.version.clone(),
        description: body.description.clone(),
        skill_md: body.content.clone(),
        runtime,
        dependencies,
        visibility,
    };

    match state.skill_repo.update(id, update).await {
        Ok(skill) => HttpResponse::Ok().json(ApiResponse::success(SkillResponse::from(skill))),
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
            HttpResponse::build(status)
                .json(ApiResponse::<()>::error(code, &msg))
        }
    }
}

/// Delete skill handler
pub async fn delete_skill(
    path: web::Path<String>,
    state: web::Data<AppState>,
    user: AuthenticatedUser,
) -> impl Responder {
    let id = match Uuid::parse_str(&path) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                "INVALID_ID",
                "Invalid skill ID format",
            ));
        }
    };

    // First verify the skill exists and user has access
    match state.skill_repo.find_by_id(id).await {
        Ok(Some(skill)) => {
            // Check ownership or admin role
            if skill.owner_id != user.user_id && !user.is_admin() {
                return HttpResponse::Forbidden().json(ApiResponse::<()>::error(
                    "FORBIDDEN",
                    "You can only delete your own skills",
                ));
            }
            // Check tenant
            if skill.tenant_id != user.tenant_id {
                return HttpResponse::Forbidden().json(ApiResponse::<()>::error(
                    "FORBIDDEN",
                    "Cannot delete skill from another tenant",
                ));
            }

            // Delete the skill
            match state.skill_repo.delete(id).await {
                Ok(()) => HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({
                    "message": "Skill deleted successfully"
                }))),
                Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                    "INTERNAL_ERROR",
                    &format!("Failed to delete skill: {}", e),
                )),
            }
        }
        Ok(None) => {
            HttpResponse::NotFound().json(ApiResponse::<()>::error("NOT_FOUND", "Skill not found"))
        }
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            "INTERNAL_ERROR",
            &format!("Failed to find skill: {}", e),
        )),
    }
}

/// Load skill handler - returns skill content for execution
pub async fn load_skill(
    path: web::Path<String>,
    state: web::Data<AppState>,
    user: AuthenticatedUser,
) -> impl Responder {
    let id = match Uuid::parse_str(&path) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                "INVALID_ID",
                "Invalid skill ID format",
            ));
        }
    };

    match state.skill_repo.find_by_id(id).await {
        Ok(Some(skill)) => {
            // Check tenant access (public skills can be loaded by anyone)
            if skill.tenant_id != user.tenant_id && skill.visibility != Visibility::Public {
                return HttpResponse::NotFound()
                    .json(ApiResponse::<()>::error("NOT_FOUND", "Skill not found"));
            }
            HttpResponse::Ok().json(ApiResponse::success(SkillLoadResponse::from(skill)))
        }
        Ok(None) => {
            HttpResponse::NotFound().json(ApiResponse::<()>::error("NOT_FOUND", "Skill not found"))
        }
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            "INTERNAL_ERROR",
            &format!("Failed to load skill: {}", e),
        )),
    }
}

/// Execute skill handler
pub async fn execute_skill(
    path: web::Path<String>,
    body: web::Json<ExecuteSkillRequest>,
    state: web::Data<AppState>,
    user: AuthenticatedUser,
) -> impl Responder {
    // 1. Parse skill ID
    let id = match Uuid::parse_str(&path) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                "INVALID_ID",
                "Invalid skill ID format",
            ));
        }
    };

    // 2. Check sandbox enabled
    if !state.config.sandbox.enabled {
        return HttpResponse::ServiceUnavailable().json(ApiResponse::<()>::error(
            "SANDBOX_DISABLED",
            "Skill execution is not enabled",
        ));
    }

    // 3. Fetch skill from repo
    let skill = match state.skill_repo.find_by_id(id).await {
        Ok(Some(skill)) => skill,
        Ok(None) => {
            return HttpResponse::NotFound()
                .json(ApiResponse::<()>::error("NOT_FOUND", "Skill not found"));
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                &format!("Failed to fetch skill: {}", e),
            ));
        }
    };

    // 4. Check tenant access
    if skill.tenant_id != user.tenant_id && skill.visibility != Visibility::Public {
        return HttpResponse::NotFound()
            .json(ApiResponse::<()>::error("NOT_FOUND", "Skill not found"));
    }

    // 5. Determine code to execute
    let code = body.code.clone().unwrap_or_else(|| skill.skill_md.clone());

    // 6. Build ExecuteRequest
    let request = ExecuteRequest {
        skill_id: skill.id.to_string(),
        code,
        language: runtime_to_string(&skill.runtime),
        parameters: body.parameters.clone(),
    };

    // 7. Execute
    match state.skill_executor.execute(request).await {
        Ok(response) => {
            let status = if response.timed_out {
                "timeout"
            } else if response.exit_code == 0 {
                "success"
            } else {
                "error"
            };

            HttpResponse::Ok().json(ApiResponse::success(SkillExecutionResponse {
                status: status.to_string(),
                skill_id: skill.id.to_string(),
                skill_name: skill.name,
                runtime: runtime_to_string(&skill.runtime),
                output: response.stdout,
                errors: response.stderr,
                exit_code: response.exit_code,
                execution_time_ms: response.execution_time_ms,
                timed_out: response.timed_out,
            }))
        }
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            "EXECUTION_ERROR",
            &format!("Skill execution failed: {}", e),
        )),
    }
}
