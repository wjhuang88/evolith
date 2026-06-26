//! API Key management handlers
//! Handles API key creation, listing, and revocation

use actix_web::{web, HttpMessage, HttpResponse, Responder};
use chrono::Utc;
use sha2::{Digest, Sha256};
use uuid::Uuid;
use validator::Validate;

use crate::dto::api_key_dto::*;
use crate::dto::common::ApiResponse;
use crate::middleware::api_key_scope::api_key_allows_api_key_management;
use crate::middleware::auth::AuthenticatedUser;
use crate::state::AppState;
use domain::api_key::{ApiKey, ApiKeyStatus, NewApiKey};

fn forbid_if_api_key(req: &actix_web::HttpRequest) -> Option<HttpResponse> {
    let extensions = req.extensions();
    let api_key = extensions.get::<ApiKey>()?;
    if api_key_allows_api_key_management(api_key) {
        None
    } else {
        Some(HttpResponse::Forbidden().json(ApiResponse::<()>::error(
            "FORBIDDEN",
            "API key management requires JWT authentication",
        )))
    }
}

/// Generate a new API key with prefix and hash
fn generate_api_key() -> (String, String, String) {
    // Generate random key using UUID
    let random_uuid = Uuid::new_v4();
    let full_key = format!("evo_sk_{}", random_uuid.simple());

    // Extract prefix (first 16 chars of key after prefix)
    let key_prefix = format!("evo_sk_{}...", &random_uuid.simple().to_string()[..8]);

    // Hash the key for storage using SHA-256 (deterministic)
    let key_hash = {
        let mut hasher = Sha256::new();
        hasher.update(full_key.as_bytes());
        hex::encode(hasher.finalize())
    };

    (full_key, key_prefix, key_hash)
}

/// List API keys for a tenant
pub async fn list_api_keys(
    req: actix_web::HttpRequest,
    tenant_id: web::Path<Uuid>,
    user: AuthenticatedUser,
    state: web::Data<AppState>,
) -> impl Responder {
    if let Some(resp) = forbid_if_api_key(&req) {
        return resp;
    }
    let tenant_id = tenant_id.into_inner();

    // Verify user has access to this tenant
    if user.tenant_id != tenant_id {
        return HttpResponse::Forbidden().json(ApiResponse::<()>::error(
            "FORBIDDEN",
            "You do not have access to this tenant",
        ));
    }

    // Fetch API keys from repository
    match state.api_key_repo.find_by_tenant(tenant_id).await {
        Ok(keys) => {
            let key_list: Vec<ApiKeyListItem> = keys
                .into_iter()
                .map(|k| ApiKeyListItem {
                    id: k.id.to_string(),
                    name: k.name,
                    key_prefix: k.key_prefix,
                    permissions: k.permissions,
                    expires_at: k.expires_at.map(|t| t.to_rfc3339()),
                    rate_limit: k.rate_limit as i32,
                    status: match k.status {
                        ApiKeyStatus::Active => "active".to_string(),
                        ApiKeyStatus::Revoked => "revoked".to_string(),
                        ApiKeyStatus::Expired => "expired".to_string(),
                    },
                    request_count: k.request_count as i64,
                    last_used_at: k.last_used_at.map(|t| t.to_rfc3339()),
                    created_at: k.created_at.to_rfc3339(),
                })
                .collect();

            let total = key_list.len();
            HttpResponse::Ok().json(ApiResponse::<ApiKeyListResponse>::success(
                ApiKeyListResponse {
                    keys: key_list,
                    total,
                },
            ))
        }
        Err(e) => {
            tracing::error!("Failed to list API keys: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to list API keys",
            ))
        }
    }
}

/// Create a new API key
pub async fn create_api_key(
    req: actix_web::HttpRequest,
    tenant_id: web::Path<Uuid>,
    user: AuthenticatedUser,
    body: web::Json<CreateApiKeyRequest>,
    state: web::Data<AppState>,
) -> impl Responder {
    if let Some(resp) = forbid_if_api_key(&req) {
        return resp;
    }
    let tenant_id = tenant_id.into_inner();

    // Verify user has access to this tenant
    if user.tenant_id != tenant_id {
        return HttpResponse::Forbidden().json(ApiResponse::<()>::error(
            "FORBIDDEN",
            "You do not have access to this tenant",
        ));
    }

    // Validate request
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "VALIDATION_ERROR",
            &errors.to_string(),
        ));
    }

    // Generate API key
    let (full_key, key_prefix, key_hash) = generate_api_key();

    // Calculate expiration
    let expires_at = body
        .expires_in_days
        .map(|days| Utc::now() + chrono::Duration::days(days as i64));

    // Create new API key
    let new_key = NewApiKey {
        tenant_id,
        user_id: user.user_id,
        name: body.name.clone(),
        key_hash,
        key_prefix,
        permissions: body
            .permissions
            .clone()
            .unwrap_or_else(|| vec!["read".to_string()]),
        rate_limit: body.rate_limit.map(|r| r as u32),
        expires_at,
    };

    match state.api_key_repo.create(new_key).await {
        Ok(api_key) => {
            // Return response with the full key (only shown once!)
            let response = ApiKeyResponse {
                id: api_key.id.to_string(),
                name: api_key.name,
                key: Some(full_key), // ⚠️ Only shown once!
                key_prefix: api_key.key_prefix,
                permissions: api_key.permissions,
                expires_at: api_key.expires_at.map(|t| t.to_rfc3339()),
                rate_limit: api_key.rate_limit as i32,
                status: match api_key.status {
                    ApiKeyStatus::Active => "active".to_string(),
                    ApiKeyStatus::Revoked => "revoked".to_string(),
                    ApiKeyStatus::Expired => "expired".to_string(),
                },
                created_at: api_key.created_at.to_rfc3339(),
            };

            HttpResponse::Ok().json(ApiResponse::<ApiKeyResponse>::success(response))
        }
        Err(e) => {
            tracing::error!("Failed to create API key: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to create API key",
            ))
        }
    }
}

/// Revoke an API key
pub async fn revoke_api_key(
    req: actix_web::HttpRequest,
    path: web::Path<(Uuid, Uuid)>,
    user: AuthenticatedUser,
    _body: web::Json<RevokeApiKeyRequest>,
    state: web::Data<AppState>,
) -> impl Responder {
    if let Some(resp) = forbid_if_api_key(&req) {
        return resp;
    }
    let (tenant_id, key_id) = path.into_inner();

    // Verify user has access to this tenant
    if user.tenant_id != tenant_id {
        return HttpResponse::Forbidden().json(ApiResponse::<()>::error(
            "FORBIDDEN",
            "You do not have access to this tenant",
        ));
    }

    // Verify the key belongs to this tenant before revoking
    match state.api_key_repo.find_by_id(key_id).await {
        Ok(Some(key)) if key.tenant_id == tenant_id => {
            match state.api_key_repo.revoke(key_id).await {
                Ok(()) => HttpResponse::Ok().json(ApiResponse::<serde_json::Value>::success(
                    serde_json::json!({
                        "message": "API key revoked successfully"
                    }),
                )),
                Err(e) => {
                    tracing::error!("Failed to revoke API key: {}", e);
                    HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                        "INTERNAL_ERROR",
                        "Failed to revoke API key",
                    ))
                }
            }
        }
        Ok(Some(_)) => HttpResponse::Forbidden().json(ApiResponse::<()>::error(
            "FORBIDDEN",
            "API key does not belong to this tenant",
        )),
        Ok(None) => HttpResponse::NotFound()
            .json(ApiResponse::<()>::error("NOT_FOUND", "API key not found")),
        Err(e) => {
            tracing::error!("Failed to find API key: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to find API key",
            ))
        }
    }
}

/// Validate an API key (for MCP handlers and middleware)
/// Takes AppState and key string, returns the ApiKey if valid
pub async fn validate_api_key(key: &str, state: &AppState) -> Result<ApiKey, String> {
    // Extract prefix from key format: evo_sk_{uuid}
    let parts: Vec<&str> = key.split('_').collect();
    if parts.len() < 3 || parts[0] != "evo" || parts[1] != "sk" {
        return Err("Invalid API key format".to_string());
    }

    // Hash the provided key to compare with stored hash
    let key_hash = {
        let mut hasher = Sha256::new();
        hasher.update(key.as_bytes());
        hex::encode(hasher.finalize())
    };

    // Find key by hash
    match state.api_key_repo.find_by_key(&key_hash).await {
        Ok(Some(api_key)) => {
            // Check status
            if api_key.status != ApiKeyStatus::Active {
                return Err("API key is not active".to_string());
            }

            // Check expiration
            if let Some(expires_at) = api_key.expires_at {
                if expires_at < Utc::now() {
                    return Err("API key has expired".to_string());
                }
            }

            Ok(api_key)
        }
        Ok(None) => Err("API key not found".to_string()),
        Err(e) => {
            tracing::error!("Failed to validate API key: {}", e);
            Err("Failed to validate API key".to_string())
        }
    }
}
