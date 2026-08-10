use std::path::Path;

use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use domain::api_key::ApiKey;
use futures_util::StreamExt;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;

use crate::middleware::api_key_scope::{api_key_allows_repo_read, api_key_allows_repo_write};
use crate::middleware::auth::AuthenticatedUser;
use crate::services::repo_push_event::{
    enqueue_repo_push_completed, reconcile_repo_push_completed,
};
use crate::state::AppState;

use service_git::{advertise_refs, spawn_rpc, GitService, GIT_SUBPROCESS_TIMEOUT};

const RECEIVE_PACK_RESPONSE_LIMIT: usize = 8 * 1024 * 1024;

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

    if service == GitService::ReceivePack {
        match reconcile_repo_push_completed(
            state.outbox_repo.as_ref(),
            repo.tenant_id,
            repo.id,
            &repo.default_branch,
            &abs_path,
        )
        .await
        {
            Ok(Some(event)) => tracing::debug!(
                repo_id = %repo.id,
                event_id = %event.id,
                "receive-pack advertisement reconciled durable push event"
            ),
            Ok(None) => {}
            Err(error) => {
                tracing::error!(
                    repo_id = %repo.id,
                    error_code = %error.error_code_info().code,
                    "receive-pack reconciliation failed"
                );
                return HttpResponse::ServiceUnavailable()
                    .insert_header(("Retry-After", "1"))
                    .body("Git push reconciliation unavailable");
            }
        }
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

    if service == GitService::ReceivePack {
        return handle_receive_pack(
            state,
            payload,
            repo.id,
            repo.tenant_id,
            &repo.default_branch,
            &abs_path,
        )
        .await;
    }

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

        stderr_drain.abort();
        stdin_task.abort();
    });

    let output_stream = tokio_stream::wrappers::ReceiverStream::new(rx);

    HttpResponse::Ok()
        .insert_header(("Content-Type", service.content_type_result()))
        .streaming(output_stream)
}

async fn handle_receive_pack(
    state: web::Data<AppState>,
    payload: web::Payload,
    repo_id: uuid::Uuid,
    tenant_id: uuid::Uuid,
    default_branch: &str,
    abs_path: &Path,
) -> HttpResponse {
    let mut child = match spawn_rpc(abs_path, GitService::ReceivePack).await {
        Ok(child) => child,
        Err(error) => {
            tracing::error!(repo_id = %repo_id, error = %error, "failed to start receive-pack");
            return HttpResponse::InternalServerError().body("Git receive-pack failed");
        }
    };
    let mut stdin = child.stdin.take().expect("stdin should be piped");
    let mut stdout = child.stdout.take().expect("stdout should be piped");
    let mut stderr = child.stderr.take().expect("stderr should be piped");

    let mut stdin_task = actix_web::rt::spawn(async move {
        let mut payload_stream = payload.into_inner();
        while let Some(chunk) = payload_stream.next().await {
            let bytes = chunk.map_err(|_| ())?;
            stdin.write_all(&bytes).await.map_err(|_| ())?;
        }
        stdin.shutdown().await.map_err(|_| ())
    });
    let stderr_task = actix_web::rt::spawn(async move {
        let mut stderr_buf = vec![0u8; 8192];
        while let Ok(read) = stderr.read(&mut stderr_buf).await {
            if read == 0 {
                break;
            }
            tracing::debug!("git receive-pack emitted stderr");
        }
    });

    let mut output = Vec::new();
    let rpc_result = tokio::time::timeout(GIT_SUBPROCESS_TIMEOUT, async {
        let mut buffer = vec![0u8; 8192];
        loop {
            let read = stdout.read(&mut buffer).await.map_err(|_| ())?;
            if read == 0 {
                break;
            }
            if output.len().saturating_add(read) > RECEIVE_PACK_RESPONSE_LIMIT {
                return Err(());
            }
            output.extend_from_slice(&buffer[..read]);
        }
        let status = child.wait().await.map_err(|_| ())?;
        (&mut stdin_task).await.map_err(|_| ())??;
        Ok::<_, ()>(status)
    })
    .await;

    stderr_task.abort();
    let status = match rpc_result {
        Ok(Ok(status)) => status,
        Ok(Err(())) => {
            stdin_task.abort();
            let _ = child.kill().await;
            tracing::error!(repo_id = %repo_id, "receive-pack I/O failed or response exceeded limit");
            return HttpResponse::BadGateway().body("Git receive-pack failed");
        }
        Err(_) => {
            stdin_task.abort();
            let _ = child.kill().await;
            tracing::warn!(repo_id = %repo_id, "receive-pack timed out");
            return HttpResponse::GatewayTimeout().body("Git receive-pack timed out");
        }
    };
    if !status.success() {
        tracing::warn!(repo_id = %repo_id, %status, "receive-pack rejected push");
        return HttpResponse::Ok()
            .insert_header((
                "Content-Type",
                GitService::ReceivePack.content_type_result(),
            ))
            .body(output);
    }

    let event = enqueue_repo_push_completed(
        state.outbox_repo.as_ref(),
        tenant_id,
        repo_id,
        default_branch,
        abs_path,
    )
    .await;
    match event {
        Ok(event) => {
            tracing::info!(
                repo_id = %repo_id,
                event_id = %event.id,
                "durable push event persisted"
            );
        }
        Err(error) => {
            tracing::error!(
                repo_id = %repo_id,
                error_code = %error.error_code_info().code,
                "Git ref changed but durable push event persistence failed; retry or reconcile required"
            );
            return HttpResponse::ServiceUnavailable()
                .insert_header(("Retry-After", "1"))
                .body("Push requires durable-event reconciliation; retry the push");
        }
    }

    HttpResponse::Ok()
        .insert_header((
            "Content-Type",
            GitService::ReceivePack.content_type_result(),
        ))
        .body(output)
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
