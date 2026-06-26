use std::path::Path;

use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use chrono::{TimeZone, Utc};
use domain::api_key::ApiKey;
use futures_util::StreamExt;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;

use crate::middleware::api_key_scope::{api_key_allows_repo_read, api_key_allows_repo_write};
use crate::middleware::auth::AuthenticatedUser;
use crate::state::AppState;

use service_git::{advertise_refs, spawn_rpc, GitService, GIT_SUBPROCESS_TIMEOUT};

pub async fn info_refs(
    req: HttpRequest,
    path: web::Path<String>,
    query: web::Query<std::collections::HashMap<String, String>>,
    user: AuthenticatedUser,
    state: web::Data<AppState>,
) -> HttpResponse {
    let repo_id = path.into_inner();
    let repo_id = match uuid::Uuid::parse_str(&repo_id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().body("Invalid repo ID");
        }
    };

    let service_str = match query.get("service") {
        Some(s) => s.clone(),
        None => {
            return HttpResponse::BadRequest().body("Missing 'service' query parameter");
        }
    };

    let service = match GitService::parse_service(&service_str) {
        Some(s) => s,
        None => {
            return HttpResponse::BadRequest().body(format!(
                "Invalid service: {}. Must be git-upload-pack or git-receive-pack",
                service_str
            ));
        }
    };

    if let Err(resp) = authorize_git_service(&req, service) {
        return resp;
    }

    let repo = match state.git_repo_repo.find_by_id(repo_id).await {
        Ok(Some(repo)) => repo,
        Ok(None) => {
            return HttpResponse::NotFound().body("Repo not found");
        }
        Err(e) => {
            tracing::error!("Failed to find repo {}: {}", repo_id, e);
            return HttpResponse::InternalServerError().body("Internal error");
        }
    };

    if repo.tenant_id != user.tenant_id {
        return HttpResponse::NotFound().body("Repo not found");
    }

    let base_path = Path::new(&state.git_storage_base_path);
    let abs_path = service_git::repo_path(base_path, repo.tenant_id, repo.id);

    if !abs_path.exists() {
        return HttpResponse::NotFound().body("Repo not found on disk");
    }

    match advertise_refs(&abs_path, service).await {
        Ok(output) => HttpResponse::Ok()
            .insert_header(("Content-Type", service.content_type_advertisement()))
            .insert_header(("Cache-Control", "no-cache"))
            .body(output),
        Err(e) => {
            tracing::error!("advertise_refs failed for repo {}: {}", repo_id, e);
            HttpResponse::InternalServerError().body(format!("Git error: {}", e))
        }
    }
}

pub async fn upload_pack(
    req: HttpRequest,
    path: web::Path<String>,
    user: AuthenticatedUser,
    state: web::Data<AppState>,
    payload: web::Payload,
) -> HttpResponse {
    handle_rpc(req, path, user, state, payload, GitService::UploadPack).await
}

pub async fn receive_pack(
    req: HttpRequest,
    path: web::Path<String>,
    user: AuthenticatedUser,
    state: web::Data<AppState>,
    payload: web::Payload,
) -> HttpResponse {
    handle_rpc(req, path, user, state, payload, GitService::ReceivePack).await
}

async fn handle_rpc(
    req: HttpRequest,
    path: web::Path<String>,
    user: AuthenticatedUser,
    state: web::Data<AppState>,
    payload: web::Payload,
    service: GitService,
) -> HttpResponse {
    if let Err(resp) = authorize_git_service(&req, service) {
        return resp;
    }

    let repo_id = path.into_inner();
    let repo_id = match uuid::Uuid::parse_str(&repo_id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().body("Invalid repo ID");
        }
    };

    let repo = match state.git_repo_repo.find_by_id(repo_id).await {
        Ok(Some(repo)) => repo,
        Ok(None) => {
            return HttpResponse::NotFound().body("Repo not found");
        }
        Err(e) => {
            tracing::error!("Failed to find repo {}: {}", repo_id, e);
            return HttpResponse::InternalServerError().body("Internal error");
        }
    };

    if repo.tenant_id != user.tenant_id {
        return HttpResponse::NotFound().body("Repo not found");
    }

    let base_path = Path::new(&state.git_storage_base_path);
    let abs_path = service_git::repo_path(base_path, repo.tenant_id, repo.id);

    if !abs_path.exists() {
        return HttpResponse::NotFound().body("Repo not found on disk");
    }

    let default_branch = repo.default_branch.clone();
    let repo_id_for_meta = repo.id;
    let state_for_meta = state.clone();
    let abs_path_for_meta = abs_path.clone();
    let service_for_meta = service;
    let user_tenant_for_meta = user.tenant_id;

    let mut child = match spawn_rpc(&abs_path, service).await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("spawn_rpc failed for repo {}: {}", repo_id, e);
            return HttpResponse::InternalServerError().body(format!("Git error: {}", e));
        }
    };

    let mut stdin = child.stdin.take().expect("stdin should be piped");
    let mut stdout = child.stdout.take().expect("stdout should be piped");
    let mut stderr = child.stderr.take().expect("stderr should be piped");

    let (tx, rx) = mpsc::channel::<Result<actix_web::web::Bytes, actix_web::Error>>(32);
    let tx_reader = tx.clone();

    let stdin_task = actix_web::rt::spawn(async move {
        let mut payload_stream = payload.into_inner();
        while let Some(chunk) = payload_stream.next().await {
            match chunk {
                Ok(bytes) => {
                    if let Err(e) = stdin.write_all(&bytes).await {
                        tracing::error!("Failed to write to git stdin: {}", e);
                        let _ = tx
                            .send(Err(actix_web::error::ErrorInternalServerError(
                                "Failed to write to git process",
                            )))
                            .await;
                        return;
                    }
                }
                Err(e) => {
                    let _ = tx
                        .send(Err(actix_web::error::ErrorInternalServerError(format!(
                            "Request stream error: {e}"
                        ))))
                        .await;
                    return;
                }
            }
        }

        if let Err(e) = stdin.shutdown().await {
            tracing::error!("Failed to close git stdin: {}", e);
        }
        drop(stdin);
    });

    let default_branch_for_meta = default_branch.clone();
    actix_web::rt::spawn(async move {
        let mut buf = vec![0u8; 8192];
        let stderr_drain = actix_web::rt::spawn(async move {
            let mut stderr_buf = vec![0u8; 8192];
            loop {
                match stderr.read(&mut stderr_buf).await {
                    Ok(0) => break,
                    Ok(n) => {
                        tracing::debug!(
                            "git stderr: {}",
                            String::from_utf8_lossy(&stderr_buf[..n]).trim()
                        );
                    }
                    Err(e) => {
                        tracing::debug!("Failed to drain git stderr: {}", e);
                        break;
                    }
                }
            }
        });

        let rpc_result = tokio::time::timeout(GIT_SUBPROCESS_TIMEOUT, async {
            loop {
                match stdout.read(&mut buf).await {
                    Ok(0) => break,
                    Ok(n) => {
                        let data = actix_web::web::Bytes::copy_from_slice(&buf[..n]);
                        if tx_reader.send(Ok(data)).await.is_err() {
                            break;
                        }
                    }
                    Err(e) => {
                        tracing::error!("Failed to read from git stdout: {}", e);
                        break;
                    }
                }
            }

            child.wait().await
        })
        .await;

        let push_succeeded = matches!(&rpc_result, Ok(Ok(status)) if status.success());

        match rpc_result {
            Ok(Ok(status)) if !status.success() => {
                tracing::warn!("git subprocess exited with status: {}", status);
            }
            Ok(Err(e)) => {
                tracing::error!("Failed to wait for git subprocess: {}", e);
            }
            Err(_) => {
                tracing::warn!(
                    "git subprocess timed out after {}s",
                    GIT_SUBPROCESS_TIMEOUT.as_secs()
                );
                let _ = child.kill().await;
                let _ = tx_reader
                    .send(Err(actix_web::error::ErrorGatewayTimeout(
                        "Git subprocess timed out",
                    )))
                    .await;
            }
            _ => {}
        }

        if push_succeeded && service_for_meta == GitService::ReceivePack {
            update_repo_metadata_after_push(
                &state_for_meta,
                repo_id_for_meta,
                user_tenant_for_meta,
                &default_branch_for_meta,
                &abs_path_for_meta,
            )
            .await;
        }

        stderr_drain.abort();
        stdin_task.abort();
    });

    let output_stream = tokio_stream::wrappers::ReceiverStream::new(rx);

    HttpResponse::Ok()
        .insert_header(("Content-Type", service.content_type_result()))
        .streaming(output_stream)
}

async fn update_repo_metadata_after_push(
    state: &web::Data<AppState>,
    repo_id: uuid::Uuid,
    user_tenant_id: uuid::Uuid,
    default_branch: &str,
    abs_path: &Path,
) {
    let ref_name = format!("refs/heads/{}", default_branch);
    let path_for_blocking = abs_path.to_path_buf();
    let ref_name_for_blocking = ref_name.clone();
    let resolve =
        web::block(move || service_git::resolve_ref(&path_for_blocking, &ref_name_for_blocking));
    let resolved = match tokio::time::timeout(service_git::CONTEXT_BLOCKING_TIMEOUT, resolve).await
    {
        Ok(Ok(resolved)) => resolved,
        Ok(Err(e)) => {
            tracing::warn!(
                "push metadata resolve_ref error for repo {}: {}",
                repo_id,
                e
            );
            return;
        }
        Err(_) => {
            tracing::warn!(
                "push metadata resolve_ref timed out for repo {} after {}s",
                repo_id,
                service_git::CONTEXT_BLOCKING_TIMEOUT.as_secs()
            );
            return;
        }
    };

    let (sha, timestamp) = match resolved {
        Ok(t) => t,
        Err(e) => {
            tracing::warn!(
                "push metadata: cannot resolve default branch ref '{}' for repo {}: {}",
                ref_name,
                repo_id,
                e
            );
            return;
        }
    };

    let committed_at = Utc
        .timestamp_opt(timestamp, 0)
        .single()
        .unwrap_or_else(Utc::now);
    if let Err(e) = state
        .git_repo_repo
        .update_last_commit(repo_id, &sha, committed_at)
        .await
    {
        tracing::warn!(
            "push metadata update failed for repo {} (push already succeeded): {}",
            repo_id,
            e
        );
        return;
    }

    tracing::info!(
        "push metadata updated for repo {} (tenant={}): {} @ {}",
        repo_id,
        user_tenant_id,
        &sha[..sha.len().min(12)],
        committed_at.to_rfc3339()
    );
}

fn authorize_git_service(req: &HttpRequest, service: GitService) -> Result<(), HttpResponse> {
    let extensions = req.extensions();
    let api_key = match extensions.get::<ApiKey>() {
        Some(k) => k,
        None => return Ok(()),
    };

    let allowed = match service {
        GitService::UploadPack => api_key_allows_repo_read(api_key),
        GitService::ReceivePack => api_key_allows_repo_write(api_key),
    };

    if allowed {
        Ok(())
    } else {
        Err(HttpResponse::Forbidden()
            .insert_header(("Content-Type", "text/plain; charset=utf-8"))
            .body("API key lacks required repository permission"))
    }
}
