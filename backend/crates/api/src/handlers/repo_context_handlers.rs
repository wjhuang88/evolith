use std::path::Path;

use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

use crate::dto::common::ApiResponse;
use crate::dto::repo_context_dto::*;
use crate::middleware::auth::AuthenticatedUser;
use crate::state::AppState;

pub async fn get_file_tree(
    path: web::Path<(Uuid, Uuid)>,
    query: web::Query<RefQuery>,
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

    let repo = match state.git_repo_repo.find_by_id(repo_id).await {
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

    let base_path = Path::new(&state.git_storage_base_path);
    let abs = service_git::repo_path(base_path, repo.tenant_id, repo.id);
    let git_ref = query.git_ref.clone().unwrap_or(repo.default_branch.clone());

    let result = web::block(move || service_git::read_file_tree(&abs, &git_ref)).await;
    match result {
        Ok(Ok(entries)) => {
            let dtos: Vec<FileTreeEntryDto> = entries
                .into_iter()
                .map(|e| FileTreeEntryDto {
                    name: e.name,
                    kind: e.kind,
                    oid: e.oid,
                    is_tree: e.is_tree,
                })
                .collect();
            HttpResponse::Ok().json(ApiResponse::success(FileTreeResponse { entries: dtos }))
        }
        Ok(Err(e)) => {
            tracing::error!("Failed to read file tree: {}", e);
            HttpResponse::InternalServerError()
                .json(ApiResponse::<()>::error("INTERNAL_ERROR", &e.to_string()))
        }
        Err(e) => {
            tracing::error!("Blocking task panicked: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to read file tree",
            ))
        }
    }
}

pub async fn get_blob(
    path: web::Path<(Uuid, Uuid, String)>,
    user: AuthenticatedUser,
    state: web::Data<AppState>,
) -> impl Responder {
    let (tenant_id, repo_id, sha) = path.into_inner();

    if user.tenant_id != tenant_id {
        return HttpResponse::Forbidden().json(ApiResponse::<()>::error(
            "FORBIDDEN",
            "You do not have access to this tenant",
        ));
    }

    let repo = match state.git_repo_repo.find_by_id(repo_id).await {
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

    let base_path = Path::new(&state.git_storage_base_path);
    let abs = service_git::repo_path(base_path, repo.tenant_id, repo.id);
    let sha_clone = sha.clone();

    let result = web::block(move || service_git::read_blob(&abs, &sha_clone)).await;
    match result {
        Ok(Ok(data)) => {
            let is_binary = data.contains(&0u8);
            let (content, encoding) = if is_binary {
                use base64::Engine;
                (
                    base64::engine::general_purpose::STANDARD.encode(&data),
                    "base64".to_string(),
                )
            } else {
                (
                    String::from_utf8_lossy(&data).to_string(),
                    "utf-8".to_string(),
                )
            };
            let size = data.len();
            HttpResponse::Ok().json(ApiResponse::success(BlobResponse {
                content,
                size,
                encoding,
            }))
        }
        Ok(Err(e)) => {
            tracing::error!("Failed to read blob: {}", e);
            HttpResponse::InternalServerError()
                .json(ApiResponse::<()>::error("INTERNAL_ERROR", &e.to_string()))
        }
        Err(e) => {
            tracing::error!("Blocking task panicked: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to read blob",
            ))
        }
    }
}

pub async fn get_commits(
    path: web::Path<(Uuid, Uuid)>,
    query: web::Query<CommitQuery>,
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

    let repo = match state.git_repo_repo.find_by_id(repo_id).await {
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

    let base_path = Path::new(&state.git_storage_base_path);
    let abs = service_git::repo_path(base_path, repo.tenant_id, repo.id);
    let git_ref = query.git_ref.clone().unwrap_or(repo.default_branch.clone());
    let limit = query.limit.unwrap_or(50);

    let result = web::block(move || service_git::read_commits(&abs, &git_ref, limit)).await;
    match result {
        Ok(Ok(commits)) => {
            let dtos: Vec<CommitDto> = commits
                .into_iter()
                .map(|c| CommitDto {
                    sha: c.sha,
                    author_name: c.author_name,
                    author_email: c.author_email,
                    message: c.message,
                    timestamp: c.timestamp,
                })
                .collect();
            HttpResponse::Ok().json(ApiResponse::success(CommitListResponse { commits: dtos }))
        }
        Ok(Err(e)) => {
            tracing::error!("Failed to read commits: {}", e);
            HttpResponse::InternalServerError()
                .json(ApiResponse::<()>::error("INTERNAL_ERROR", &e.to_string()))
        }
        Err(e) => {
            tracing::error!("Blocking task panicked: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to read commits",
            ))
        }
    }
}

pub async fn get_diff(
    path: web::Path<(Uuid, Uuid)>,
    query: web::Query<DiffQuery>,
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

    let repo = match state.git_repo_repo.find_by_id(repo_id).await {
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

    let base_path = Path::new(&state.git_storage_base_path);
    let abs = service_git::repo_path(base_path, repo.tenant_id, repo.id);
    let base_ref = query.base.clone();
    let head_ref = query.head.clone();

    let result = web::block(move || service_git::read_diff(&abs, &base_ref, &head_ref)).await;
    match result {
        Ok(Ok(entries)) => {
            let dtos: Vec<DiffEntryDto> = entries
                .into_iter()
                .map(|e| DiffEntryDto {
                    path: e.path,
                    change_type: e.change_type,
                    old_oid: e.old_oid,
                    new_oid: e.new_oid,
                })
                .collect();
            HttpResponse::Ok().json(ApiResponse::success(DiffResponse { entries: dtos }))
        }
        Ok(Err(e)) => {
            tracing::error!("Failed to read diff: {}", e);
            HttpResponse::InternalServerError()
                .json(ApiResponse::<()>::error("INTERNAL_ERROR", &e.to_string()))
        }
        Err(e) => {
            tracing::error!("Blocking task panicked: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to read diff",
            ))
        }
    }
}
