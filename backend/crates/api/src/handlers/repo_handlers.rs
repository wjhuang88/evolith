use std::sync::Arc;

use actix_web::{web, HttpMessage, HttpResponse, Responder};
use chrono::Utc;
use uuid::Uuid;
use validator::Validate;

use crate::dto::common::ApiResponse;
use crate::dto::repo_dto::*;
use crate::middleware::api_key_scope::{api_key_allows_repo_read, api_key_allows_repo_write};
use crate::middleware::auth::AuthenticatedUser;
use crate::services::repo_lifecycle::{
    FileSystemRepoStorage, RepoLifecycleError, RepoLifecycleService,
};
use crate::state::AppState;
use domain::api_key::ApiKey;
use domain::audit::AuditLog;
use domain::git_repo::{NewGitRepo, RepoLifecycleStatus, RepoVisibility, UpdateGitRepo};

fn lifecycle_service(state: &AppState) -> RepoLifecycleService {
    RepoLifecycleService::new(
        state.git_repo_repo.clone(),
        Arc::new(FileSystemRepoStorage::new(
            state.git_storage_base_path.clone(),
        )),
    )
}

fn forbid_if_api_key_lacks(req: &actix_web::HttpRequest, write: bool) -> Option<HttpResponse> {
    let extensions = req.extensions();
    let api_key = extensions.get::<ApiKey>()?;
    let allowed = if write {
        api_key_allows_repo_write(api_key)
    } else {
        api_key_allows_repo_read(api_key)
    };
    if allowed {
        None
    } else {
        Some(HttpResponse::Forbidden().json(ApiResponse::<()>::error(
            "FORBIDDEN",
            "API key lacks required repository permission",
        )))
    }
}

fn visibility_from_str(s: &str) -> RepoVisibility {
    match s {
        "public" => RepoVisibility::Public,
        _ => RepoVisibility::Private,
    }
}

fn repo_to_response(repo: &domain::git_repo::GitRepo) -> RepoResponse {
    RepoResponse {
        id: repo.id.to_string(),
        tenant_id: repo.tenant_id.to_string(),
        name: repo.name.clone(),
        description: repo.description.clone(),
        default_branch: repo.default_branch.clone(),
        storage_path: repo.storage_path.clone(),
        visibility: match repo.visibility {
            RepoVisibility::Public => "public".to_string(),
            RepoVisibility::Private => "private".to_string(),
        },
        auto_merge: repo.auto_merge,
        require_review: repo.require_review,
        lifecycle_status: match repo.lifecycle_status {
            RepoLifecycleStatus::Creating => "CREATING".to_string(),
            RepoLifecycleStatus::Active => "ACTIVE".to_string(),
            RepoLifecycleStatus::Error => "ERROR".to_string(),
            RepoLifecycleStatus::Deleting => "DELETING".to_string(),
        },
        last_commit_sha: repo.last_commit_sha.clone(),
        last_committed_at: repo.last_committed_at.map(|t| t.to_rfc3339()),
        created_at: repo.created_at.to_rfc3339(),
        updated_at: repo.updated_at.to_rfc3339(),
    }
}

pub async fn list_repos(
    req: actix_web::HttpRequest,
    tenant_id: web::Path<Uuid>,
    user: AuthenticatedUser,
    state: web::Data<AppState>,
) -> impl Responder {
    if let Some(resp) = forbid_if_api_key_lacks(&req, false) {
        return resp;
    }
    let tenant_id = tenant_id.into_inner();

    if user.tenant_id != tenant_id {
        return HttpResponse::Forbidden().json(ApiResponse::<()>::error(
            "FORBIDDEN",
            "You do not have access to this tenant",
        ));
    }

    match state.git_repo_repo.find_by_tenant(tenant_id).await {
        Ok(repos) => {
            let repo_list: Vec<RepoResponse> = repos.iter().map(repo_to_response).collect();
            let total = repo_list.len();
            HttpResponse::Ok().json(ApiResponse::<RepoListResponse>::success(RepoListResponse {
                repos: repo_list,
                total,
            }))
        }
        Err(e) => {
            tracing::error!("Failed to list repos: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to list repos",
            ))
        }
    }
}

pub async fn create_repo(
    req: actix_web::HttpRequest,
    tenant_id: web::Path<Uuid>,
    user: AuthenticatedUser,
    body: web::Json<CreateRepoRequest>,
    state: web::Data<AppState>,
) -> impl Responder {
    if let Some(resp) = forbid_if_api_key_lacks(&req, true) {
        return resp;
    }
    let tenant_id = tenant_id.into_inner();

    if user.tenant_id != tenant_id {
        return HttpResponse::Forbidden().json(ApiResponse::<()>::error(
            "FORBIDDEN",
            "You do not have access to this tenant",
        ));
    }

    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "VALIDATION_ERROR",
            &errors.to_string(),
        ));
    }

    match state
        .git_repo_repo
        .find_by_name(tenant_id, &body.name)
        .await
    {
        Ok(Some(_)) => {
            return HttpResponse::Conflict().json(ApiResponse::<()>::error(
                "REPO_EXISTS",
                "A repo with this name already exists in the tenant",
            ));
        }
        Ok(None) => {}
        Err(e) => {
            tracing::error!("Failed to check repo name: {}", e);
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to check repo name",
            ));
        }
    }

    let new_repo = NewGitRepo {
        name: body.name.clone(),
        description: body.description.clone(),
        default_branch: body.default_branch.clone(),
        visibility: body.visibility.as_deref().map(visibility_from_str),
        auto_merge: body.auto_merge,
        require_review: body.require_review,
    };

    let repo = match lifecycle_service(&state)
        .create(tenant_id, new_repo, body.seed_template.unwrap_or(false))
        .await
    {
        Ok(repo) => repo,
        Err(error) => {
            tracing::error!(tenant_id = %tenant_id, "Failed to create repo: {}", error);
            let (code, message) = match error {
                RepoLifecycleError::StorageInit(_) => (
                    "REPO_STORAGE_INIT_FAILED",
                    "Repo storage could not be initialized; retry or clean up the errored repo",
                ),
                RepoLifecycleError::Seed(_) => (
                    "REPO_SEED_FAILED",
                    "Repo seed could not be committed; retry or clean up the errored repo",
                ),
                RepoLifecycleError::Activation(_) => (
                    "REPO_ACTIVATION_FAILED",
                    "Repo was initialized but could not be activated; retry status reconciliation",
                ),
                _ => (
                    "REPO_METADATA_UPDATE_FAILED",
                    "Repo initialization could not be recorded; retry or clean up",
                ),
            };
            return HttpResponse::InternalServerError()
                .json(ApiResponse::<()>::error(code, message));
        }
    };

    HttpResponse::Created().json(ApiResponse::<RepoResponse>::success(repo_to_response(
        &repo,
    )))
}

pub async fn get_repo(
    req: actix_web::HttpRequest,
    path: web::Path<(Uuid, Uuid)>,
    user: AuthenticatedUser,
    state: web::Data<AppState>,
) -> impl Responder {
    if let Some(resp) = forbid_if_api_key_lacks(&req, false) {
        return resp;
    }
    let (tenant_id, repo_id) = path.into_inner();

    if user.tenant_id != tenant_id {
        return HttpResponse::Forbidden().json(ApiResponse::<()>::error(
            "FORBIDDEN",
            "You do not have access to this tenant",
        ));
    }

    match state.git_repo_repo.find_by_id(repo_id).await {
        Ok(Some(repo)) if repo.tenant_id == tenant_id => {
            HttpResponse::Ok().json(ApiResponse::<RepoResponse>::success(repo_to_response(
                &repo,
            )))
        }
        Ok(Some(_)) => HttpResponse::Forbidden().json(ApiResponse::<()>::error(
            "FORBIDDEN",
            "Repo does not belong to this tenant",
        )),
        Ok(None) => {
            HttpResponse::NotFound().json(ApiResponse::<()>::error("NOT_FOUND", "Repo not found"))
        }
        Err(e) => {
            tracing::error!("Failed to find repo: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to find repo",
            ))
        }
    }
}

pub async fn update_repo(
    req: actix_web::HttpRequest,
    path: web::Path<(Uuid, Uuid)>,
    user: AuthenticatedUser,
    body: web::Json<UpdateRepoRequest>,
    state: web::Data<AppState>,
) -> impl Responder {
    if let Some(resp) = forbid_if_api_key_lacks(&req, true) {
        return resp;
    }
    let (tenant_id, repo_id) = path.into_inner();

    if user.tenant_id != tenant_id {
        return HttpResponse::Forbidden().json(ApiResponse::<()>::error(
            "FORBIDDEN",
            "You do not have access to this tenant",
        ));
    }

    match state.git_repo_repo.find_by_id(repo_id).await {
        Ok(Some(repo)) if repo.tenant_id == tenant_id => repo,
        Ok(Some(_)) => {
            return HttpResponse::Forbidden().json(ApiResponse::<()>::error(
                "FORBIDDEN",
                "Repo does not belong to this tenant",
            ));
        }
        Ok(None) => {
            return HttpResponse::NotFound()
                .json(ApiResponse::<()>::error("NOT_FOUND", "Repo not found"));
        }
        Err(e) => {
            tracing::error!("Failed to find repo: {}", e);
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to find repo",
            ));
        }
    };

    let update = UpdateGitRepo {
        name: body.name.clone(),
        description: body.description.clone(),
        default_branch: body.default_branch.clone(),
        visibility: body.visibility.as_deref().map(visibility_from_str),
        auto_merge: body.auto_merge,
        require_review: body.require_review,
    };

    match state.git_repo_repo.update(repo_id, update).await {
        Ok(repo) => HttpResponse::Ok().json(ApiResponse::<RepoResponse>::success(
            repo_to_response(&repo),
        )),
        Err(e) => {
            tracing::error!("Failed to update repo: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to update repo",
            ))
        }
    }
}

pub async fn delete_repo(
    req: actix_web::HttpRequest,
    path: web::Path<(Uuid, Uuid)>,
    user: AuthenticatedUser,
    state: web::Data<AppState>,
) -> impl Responder {
    if let Some(resp) = forbid_if_api_key_lacks(&req, true) {
        return resp;
    }
    let (tenant_id, repo_id) = path.into_inner();

    if user.tenant_id != tenant_id {
        return HttpResponse::Forbidden().json(ApiResponse::<()>::error(
            "FORBIDDEN",
            "You do not have access to this tenant",
        ));
    }

    let _existing = match state.git_repo_repo.find_by_id(repo_id).await {
        Ok(Some(repo)) if repo.tenant_id == tenant_id => repo,
        Ok(Some(_)) => {
            return HttpResponse::Forbidden().json(ApiResponse::<()>::error(
                "FORBIDDEN",
                "Repo does not belong to this tenant",
            ));
        }
        Ok(None) => {
            return HttpResponse::NotFound()
                .json(ApiResponse::<()>::error("NOT_FOUND", "Repo not found"));
        }
        Err(e) => {
            tracing::error!("Failed to find repo: {}", e);
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to find repo",
            ));
        }
    };

    match lifecycle_service(&state).delete(&_existing).await {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(error) => {
            tracing::error!("Failed to delete repo {}: {}", repo_id, error);
            let (code, message) = match error {
                RepoLifecycleError::StorageDelete(_) => (
                    "REPO_STORAGE_DELETE_FAILED",
                    "Repo storage could not be deleted; it remains recorded for reconciliation",
                ),
                RepoLifecycleError::MetadataDelete(_) => (
                    "REPO_METADATA_DELETE_FAILED",
                    "Repo storage was removed but metadata cleanup failed; reconciliation required",
                ),
                _ => (
                    "REPO_DELETE_STATE_FAILED",
                    "Repo could not enter deleting state",
                ),
            };
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(code, message))
        }
    }
}

pub async fn reconcile_repos(
    req: actix_web::HttpRequest,
    tenant_id: web::Path<Uuid>,
    user: AuthenticatedUser,
    body: web::Json<ReconcileReposRequest>,
    state: web::Data<AppState>,
) -> impl Responder {
    let tenant_id = tenant_id.into_inner();
    let api_key_auth = req.extensions().get::<ApiKey>().is_some();
    if api_key_auth || user.tenant_id != tenant_id || !user.is_admin() {
        persist_reconcile_audit(
            &req,
            &state,
            user.tenant_id,
            user.user_id,
            "repo.reconcile.denied",
            serde_json::json!({
                "target_tenant_id": tenant_id,
                "apply": body.apply,
                "reason": if api_key_auth {
                    "api_key_authentication"
                } else if user.tenant_id != tenant_id {
                    "tenant_mismatch"
                } else {
                    "insufficient_tenant_role"
                },
            }),
        )
        .await;
        return HttpResponse::Forbidden().json(ApiResponse::<()>::error(
            "FORBIDDEN",
            "Repo reconciliation requires tenant owner or admin JWT authentication",
        ));
    }

    match lifecycle_service(&state)
        .reconcile(tenant_id, body.apply)
        .await
    {
        Ok(report) => {
            persist_reconcile_audit(
                &req,
                &state,
                tenant_id,
                user.user_id,
                "repo.reconcile.completed",
                serde_json::json!({
                    "apply": report.apply,
                    "consistent": report.consistent,
                    "db_only": report.db_only,
                    "disk_only": report.disk_only,
                }),
            )
            .await;
            HttpResponse::Ok().json(ApiResponse::success(report))
        }
        Err(error) => {
            tracing::error!(tenant_id = %tenant_id, "Repo reconciliation failed: {}", error);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "REPO_RECONCILE_FAILED",
                "Repo reconciliation failed; no unknown Git data was deleted",
            ))
        }
    }
}

async fn persist_reconcile_audit(
    req: &actix_web::HttpRequest,
    state: &AppState,
    tenant_id: Uuid,
    user_id: Uuid,
    action: &str,
    details: serde_json::Value,
) {
    let audit = AuditLog {
        id: Uuid::new_v4(),
        tenant_id: Some(tenant_id),
        user_id: Some(user_id),
        action: action.to_string(),
        resource_type: Some("git_repo".to_string()),
        resource_id: None,
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
    if let Err(error) = state.audit_repo.create(audit).await {
        tracing::error!(
            audit_action = action,
            "Failed to persist repo reconcile audit: {}",
            error
        );
    }
}
