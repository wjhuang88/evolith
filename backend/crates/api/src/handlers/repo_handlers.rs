use std::path::Path;

use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use validator::Validate;

use crate::dto::common::ApiResponse;
use crate::dto::repo_dto::*;
use crate::middleware::auth::AuthenticatedUser;
use crate::state::AppState;
use domain::git_repo::{NewGitRepo, RepoVisibility, UpdateGitRepo};

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
        last_commit_sha: repo.last_commit_sha.clone(),
        last_committed_at: repo.last_committed_at.map(|t| t.to_rfc3339()),
        created_at: repo.created_at.to_rfc3339(),
        updated_at: repo.updated_at.to_rfc3339(),
    }
}

pub async fn list_repos(
    tenant_id: web::Path<Uuid>,
    user: AuthenticatedUser,
    state: web::Data<AppState>,
) -> impl Responder {
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
    tenant_id: web::Path<Uuid>,
    user: AuthenticatedUser,
    body: web::Json<CreateRepoRequest>,
    state: web::Data<AppState>,
) -> impl Responder {
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

    let repo = match state.git_repo_repo.create(new_repo, tenant_id).await {
        Ok(repo) => repo,
        Err(e) => {
            tracing::error!("Failed to create repo: {}", e);
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to create repo",
            ));
        }
    };

    let base_path = Path::new(&state.git_storage_base_path);
    match service_git::init_bare_repo(base_path, tenant_id, repo.id) {
        Ok(storage_path) => {
            if body.seed_template.unwrap_or(false) {
                let branch = repo.default_branch.clone();
                if let Err(e) = service_git::write_seed_files(&storage_path, &repo.name, &branch) {
                    tracing::warn!("Failed to write seed files for repo {}: {}", repo.id, e);
                }
            }
        }
        Err(e) => {
            tracing::warn!(
                "Failed to init bare repo on disk for {} (DB record exists): {}",
                repo.id,
                e
            );
        }
    }

    HttpResponse::Created().json(ApiResponse::<RepoResponse>::success(repo_to_response(
        &repo,
    )))
}

pub async fn get_repo(
    path: web::Path<(Uuid, Uuid)>,
    user: AuthenticatedUser,
    state: web::Data<AppState>,
) -> impl Responder {
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
    path: web::Path<(Uuid, Uuid)>,
    user: AuthenticatedUser,
    body: web::Json<UpdateRepoRequest>,
    state: web::Data<AppState>,
) -> impl Responder {
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
    path: web::Path<(Uuid, Uuid)>,
    user: AuthenticatedUser,
    state: web::Data<AppState>,
) -> impl Responder {
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

    match state.git_repo_repo.delete(repo_id).await {
        Ok(()) => {
            let base_path = Path::new(&state.git_storage_base_path);
            let disk_path = service_git::repo_path(base_path, tenant_id, repo_id);
            if let Err(e) = service_git::remove_repo(&disk_path) {
                tracing::warn!(
                    "Failed to remove disk repo for {} (DB deleted): {}",
                    repo_id,
                    e
                );
            }
            HttpResponse::NoContent().finish()
        }
        Err(e) => {
            tracing::error!("Failed to delete repo: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to delete repo",
            ))
        }
    }
}
