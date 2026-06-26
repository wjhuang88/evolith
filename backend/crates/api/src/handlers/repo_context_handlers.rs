use std::path::Path;

use actix_web::{web, HttpMessage, HttpResponse, Responder};
use uuid::Uuid;

use crate::dto::common::ApiResponse;
use crate::dto::repo_context_dto::*;
use crate::middleware::api_key_scope::api_key_allows_repo_read;
use crate::middleware::auth::AuthenticatedUser;
use crate::state::AppState;
use domain::api_key::ApiKey;

fn forbid_if_api_key_lacks(req: &actix_web::HttpRequest) -> Option<HttpResponse> {
    let extensions = req.extensions();
    let api_key = extensions.get::<ApiKey>()?;
    if api_key_allows_repo_read(api_key) {
        None
    } else {
        Some(HttpResponse::Forbidden().json(ApiResponse::<()>::error(
            "FORBIDDEN",
            "API key lacks required repository permission",
        )))
    }
}

fn map_context_error(e: service_git::GitStorageError) -> HttpResponse {
    use service_git::GitStorageError;
    match e {
        GitStorageError::InvalidInput(msg) => {
            HttpResponse::BadRequest().json(ApiResponse::<()>::error("INVALID_INPUT", &msg))
        }
        GitStorageError::NotFound(msg) => {
            HttpResponse::NotFound().json(ApiResponse::<()>::error("NOT_FOUND", &msg))
        }
        GitStorageError::ResourceExceeded(msg) => HttpResponse::PayloadTooLarge()
            .json(ApiResponse::<()>::error("RESOURCE_EXCEEDED", &msg)),
        GitStorageError::Timeout(msg) => {
            HttpResponse::GatewayTimeout().json(ApiResponse::<()>::error("TIMEOUT", &msg))
        }
        GitStorageError::ReadError(msg) => {
            tracing::error!("context read error: {}", msg);
            HttpResponse::InternalServerError()
                .json(ApiResponse::<()>::error("INTERNAL_ERROR", &msg))
        }
        other => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            "INTERNAL_ERROR",
            &other.to_string(),
        )),
    }
}

async fn run_with_timeout<F, T>(
    fut: F,
) -> Result<Result<T, service_git::GitStorageError>, actix_web::Error>
where
    F: std::future::Future<
        Output = Result<Result<T, service_git::GitStorageError>, actix_web::error::BlockingError>,
    >,
{
    match tokio::time::timeout(service_git::CONTEXT_BLOCKING_TIMEOUT, fut).await {
        Ok(Ok(result)) => Ok(result),
        Ok(Err(blocking_err)) => Err(actix_web::error::ErrorInternalServerError(format!(
            "blocking task panicked: {}",
            blocking_err
        ))),
        Err(_) => Ok(Err(service_git::GitStorageError::Timeout(format!(
            "blocking gix read exceeded {}s",
            service_git::CONTEXT_BLOCKING_TIMEOUT.as_secs()
        )))),
    }
}

pub async fn get_file_tree(
    req: actix_web::HttpRequest,
    path: web::Path<(Uuid, Uuid)>,
    query: web::Query<RefQuery>,
    user: AuthenticatedUser,
    state: web::Data<AppState>,
) -> impl Responder {
    if let Some(resp) = forbid_if_api_key_lacks(&req) {
        return resp;
    }
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
    let git_ref_for_blocking = git_ref.clone();

    let blocking = web::block(move || service_git::read_file_tree(&abs, &git_ref_for_blocking));
    let timed = run_with_timeout(blocking).await;
    let result = match timed {
        Ok(inner) => inner,
        Err(e) => {
            tracing::error!("Blocking task panicked: {}", e);
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to read file tree",
            ));
        }
    };

    match result {
        Ok(entries) => {
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
        Err(e) => {
            tracing::error!("Failed to read file tree for ref '{}': {}", git_ref, e);
            map_context_error(e)
        }
    }
}

pub async fn get_blob(
    req: actix_web::HttpRequest,
    path: web::Path<(Uuid, Uuid, String)>,
    user: AuthenticatedUser,
    state: web::Data<AppState>,
) -> impl Responder {
    if let Some(resp) = forbid_if_api_key_lacks(&req) {
        return resp;
    }
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

    let blocking = web::block(move || service_git::read_blob(&abs, &sha_clone));
    let timed = run_with_timeout(blocking).await;
    let result = match timed {
        Ok(inner) => inner,
        Err(e) => {
            tracing::error!("Blocking task panicked: {}", e);
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to read blob",
            ));
        }
    };

    match result {
        Ok(data) => {
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
        Err(e) => {
            tracing::error!("Failed to read blob {}: {}", sha, e);
            map_context_error(e)
        }
    }
}

pub async fn get_commits(
    req: actix_web::HttpRequest,
    path: web::Path<(Uuid, Uuid)>,
    query: web::Query<CommitQuery>,
    user: AuthenticatedUser,
    state: web::Data<AppState>,
) -> impl Responder {
    if let Some(resp) = forbid_if_api_key_lacks(&req) {
        return resp;
    }
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
    let git_ref_for_blocking = git_ref.clone();

    let blocking =
        web::block(move || service_git::read_commits(&abs, &git_ref_for_blocking, limit));
    let timed = run_with_timeout(blocking).await;
    let result = match timed {
        Ok(inner) => inner,
        Err(e) => {
            tracing::error!("Blocking task panicked: {}", e);
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to read commits",
            ));
        }
    };

    match result {
        Ok(commits) => {
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
        Err(e) => {
            tracing::error!("Failed to read commits for ref '{}': {}", git_ref, e);
            map_context_error(e)
        }
    }
}

pub async fn get_diff(
    req: actix_web::HttpRequest,
    path: web::Path<(Uuid, Uuid)>,
    query: web::Query<DiffQuery>,
    user: AuthenticatedUser,
    state: web::Data<AppState>,
) -> impl Responder {
    if let Some(resp) = forbid_if_api_key_lacks(&req) {
        return resp;
    }
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

    let blocking = web::block(move || service_git::read_diff(&abs, &base_ref, &head_ref));
    let timed = run_with_timeout(blocking).await;
    let result = match timed {
        Ok(inner) => inner,
        Err(e) => {
            tracing::error!("Blocking task panicked: {}", e);
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to read diff",
            ));
        }
    };

    match result {
        Ok(entries) => {
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
        Err(e) => {
            tracing::error!(
                "Failed to read diff base={} head={}: {}",
                query.base,
                query.head,
                e
            );
            map_context_error(e)
        }
    }
}
