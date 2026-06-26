use std::path::Path;

use actix_web::{web, HttpResponse};
use futures_util::StreamExt;
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc;

use crate::middleware::auth::AuthenticatedUser;
use crate::state::AppState;

use service_git::{advertise_refs, spawn_rpc, GitService};

pub async fn info_refs(
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
    path: web::Path<String>,
    user: AuthenticatedUser,
    state: web::Data<AppState>,
    payload: web::Payload,
) -> HttpResponse {
    handle_rpc(path, user, state, payload, GitService::UploadPack).await
}

pub async fn receive_pack(
    path: web::Path<String>,
    user: AuthenticatedUser,
    state: web::Data<AppState>,
    payload: web::Payload,
) -> HttpResponse {
    handle_rpc(path, user, state, payload, GitService::ReceivePack).await
}

async fn handle_rpc(
    path: web::Path<String>,
    user: AuthenticatedUser,
    state: web::Data<AppState>,
    payload: web::Payload,
    service: GitService,
) -> HttpResponse {
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

    let mut child = match spawn_rpc(&abs_path, service).await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("spawn_rpc failed for repo {}: {}", repo_id, e);
            return HttpResponse::InternalServerError().body(format!("Git error: {}", e));
        }
    };

    let mut stdin = child.stdin.take().expect("stdin should be piped");
    let mut stdout = child.stdout.take().expect("stdout should be piped");

    let (tx, rx) = mpsc::channel::<Result<actix_web::web::Bytes, actix_web::Error>>(32);
    let tx_reader = tx.clone();

    actix_web::rt::spawn(async move {
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
        loop {
            match tokio::io::AsyncReadExt::read(&mut stdout, &mut buf).await {
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

        match child.wait().await {
            Ok(status) if !status.success() => {
                tracing::warn!("git subprocess exited with status: {}", status);
            }
            Err(e) => {
                tracing::error!("Failed to wait for git subprocess: {}", e);
            }
            _ => {}
        }
    });

    let output_stream = tokio_stream::wrappers::ReceiverStream::new(rx);

    HttpResponse::Ok()
        .insert_header(("Content-Type", service.content_type_result()))
        .streaming(output_stream)
}
