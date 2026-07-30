//! API Key management handlers
//! Handles API key creation, listing, and revocation

use actix_web::{web, HttpMessage, HttpResponse, Responder};
use chrono::Utc;
use sha2::{Digest, Sha256};
use uuid::Uuid;
use validator::Validate;

use crate::dto::api_key_dto::*;
use crate::dto::common::ApiResponse;
use crate::middleware::api_key_scope::legacy_api_key_permissions;
use crate::middleware::auth::AuthenticatedUser;
use crate::state::AppState;
use domain::api_key::{ApiKey, ApiKeyStatus, NewApiKey};
use domain::audit::AuditLog;

#[derive(Debug, Clone, Copy)]
enum ApiKeyManagementAction {
    List,
    Create,
    Revoke,
}

impl ApiKeyManagementAction {
    const fn as_str(self) -> &'static str {
        match self {
            Self::List => "list",
            Self::Create => "create",
            Self::Revoke => "revoke",
        }
    }
}

async fn authorize_api_key_management(
    req: &actix_web::HttpRequest,
    user: &AuthenticatedUser,
    target_tenant_id: Uuid,
    action: ApiKeyManagementAction,
    state: &AppState,
) -> Result<(), HttpResponse> {
    let authenticated_with_api_key = req.extensions().get::<ApiKey>().is_some();
    let allowed = !authenticated_with_api_key
        && user.tenant_id == target_tenant_id
        && user.is_admin();

    if allowed {
        return Ok(());
    }

    let denial_reason = if authenticated_with_api_key {
        "api_key_authentication"
    } else if user.tenant_id != target_tenant_id {
        "tenant_mismatch"
    } else {
        "insufficient_tenant_role"
    };

    // Denial events belong to the caller's tenant. Writing the caller identity into the target
    // tenant's audit stream would create a cross-tenant side effect and expose foreign identity
    // metadata. The attempted target remains available as non-owning event detail.
    let audit_log = AuditLog {
        id: Uuid::new_v4(),
        tenant_id: Some(user.tenant_id),
        user_id: Some(user.user_id),
        action: "api_key.management.denied".to_string(),
        resource_type: Some("api_key".to_string()),
        resource_id: None,
        details: serde_json::json!({
            "operation": action.as_str(),
            "reason": denial_reason,
            "caller_tenant_id": user.tenant_id,
            "target_tenant_id": target_tenant_id,
            "tenant_role": &user.tenant_role,
        }),
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

    if let Err(error) = state.audit_repo.create(audit_log).await {
        tracing::error!(
            action = action.as_str(),
            user_id = %user.user_id,
            caller_tenant_id = %user.tenant_id,
            target_tenant_id = %target_tenant_id,
            "Failed to persist API key authorization denial audit: {}",
            error
        );
    }

    Err(HttpResponse::Forbidden().json(ApiResponse::<()>::error(
        "FORBIDDEN",
        "API key management requires tenant owner or admin JWT authentication",
    )))
}

/// Generate a new API key with prefix and hash
fn generate_api_key() -> (String, String, String) {
    let random_uuid = Uuid::new_v4();
    let full_key = format!("evo_sk_{}", random_uuid.simple());
    let key_prefix = format!("evo_sk_{}...", &random_uuid.simple().to_string()[..8]);
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
    let tenant_id = tenant_id.into_inner();
    if let Err(response) = authorize_api_key_management(
        &req,
        &user,
        tenant_id,
        ApiKeyManagementAction::List,
        &state,
    )
    .await
    {
        return response;
    }

    match state.api_key_repo.find_by_tenant(tenant_id).await {
        Ok(keys) => {
            for key in &keys {
                let legacy_permissions = legacy_api_key_permissions(key);
                if !legacy_permissions.is_empty() {
                    tracing::warn!(
                        api_key_id = %key.id,
                        tenant_id = %tenant_id,
                        permissions = ?legacy_permissions,
                        "API key uses legacy permissions and should be rotated"
                    );
                }
            }

            let key_list: Vec<ApiKeyListItem> = keys
                .into_iter()
                .map(|key| ApiKeyListItem {
                    id: key.id.to_string(),
                    name: key.name,
                    key_prefix: key.key_prefix,
                    permissions: key.permissions,
                    expires_at: key.expires_at.map(|time| time.to_rfc3339()),
                    rate_limit: key.rate_limit as i32,
                    status: match key.status {
                        ApiKeyStatus::Active => "active".to_string(),
                        ApiKeyStatus::Revoked => "revoked".to_string(),
                        ApiKeyStatus::Expired => "expired".to_string(),
                    },
                    request_count: key.request_count as i64,
                    last_used_at: key.last_used_at.map(|time| time.to_rfc3339()),
                    created_at: key.created_at.to_rfc3339(),
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
        Err(error) => {
            tracing::error!("Failed to list API keys: {}", error);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to list API keys",
            ))
        }
    }
}

/// Create a new API key.
///
/// The raw body is intentionally accepted as `Bytes`: authorization must happen before typed
/// capability deserialization so an unauthorized caller cannot turn an invalid capability into a
/// DTO-level 400 that bypasses the required 403 and denial audit.
pub async fn create_api_key(
    req: actix_web::HttpRequest,
    tenant_id: web::Path<Uuid>,
    user: AuthenticatedUser,
    body: web::Bytes,
    state: web::Data<AppState>,
) -> impl Responder {
    let tenant_id = tenant_id.into_inner();
    if let Err(response) = authorize_api_key_management(
        &req,
        &user,
        tenant_id,
        ApiKeyManagementAction::Create,
        &state,
    )
    .await
    {
        return response;
    }

    let body = match serde_json::from_slice::<CreateApiKeyRequest>(&body) {
        Ok(body) => body,
        Err(error) => {
            return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                "VALIDATION_ERROR",
                &format!("Invalid API key request: {error}"),
            ));
        }
    };

    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "VALIDATION_ERROR",
            &errors.to_string(),
        ));
    }

    let permissions = match body.permission_tokens() {
        Ok(permissions) => permissions,
        Err(message) => {
            return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                "VALIDATION_ERROR",
                message,
            ));
        }
    };

    let (full_key, key_prefix, key_hash) = generate_api_key();
    let expires_at = body
        .expires_in_days
        .map(|days| Utc::now() + chrono::Duration::days(days as i64));

    let new_key = NewApiKey {
        tenant_id,
        user_id: user.user_id,
        name: body.name,
        key_hash,
        key_prefix,
        permissions,
        rate_limit: body.rate_limit.map(|rate_limit| rate_limit as u32),
        expires_at,
    };

    match state.api_key_repo.create(new_key).await {
        Ok(api_key) => {
            let response = ApiKeyResponse {
                id: api_key.id.to_string(),
                name: api_key.name,
                key: Some(full_key),
                key_prefix: api_key.key_prefix,
                permissions: api_key.permissions,
                expires_at: api_key.expires_at.map(|time| time.to_rfc3339()),
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
        Err(error) => {
            tracing::error!("Failed to create API key: {}", error);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to create API key",
            ))
        }
    }
}

/// Revoke an API key. The endpoint intentionally accepts no request body so browser and CLI
/// clients can issue a plain DELETE while authorization remains enforced server-side.
pub async fn revoke_api_key(
    req: actix_web::HttpRequest,
    path: web::Path<(Uuid, Uuid)>,
    user: AuthenticatedUser,
    state: web::Data<AppState>,
) -> impl Responder {
    let (tenant_id, key_id) = path.into_inner();
    if let Err(response) = authorize_api_key_management(
        &req,
        &user,
        tenant_id,
        ApiKeyManagementAction::Revoke,
        &state,
    )
    .await
    {
        return response;
    }

    match state.api_key_repo.find_by_id(key_id).await {
        Ok(Some(key)) if key.tenant_id == tenant_id => match state.api_key_repo.revoke(key_id).await {
            Ok(()) => HttpResponse::Ok().json(ApiResponse::<serde_json::Value>::success(
                serde_json::json!({
                    "message": "API key revoked successfully"
                }),
            )),
            Err(error) => {
                tracing::error!("Failed to revoke API key: {}", error);
                HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                    "INTERNAL_ERROR",
                    "Failed to revoke API key",
                ))
            }
        },
        Ok(Some(_)) | Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()>::error(
            "NOT_FOUND",
            "API key not found or access denied",
        )),
        Err(error) => {
            tracing::error!("Failed to find API key: {}", error);
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
    let parts: Vec<&str> = key.split('_').collect();
    if parts.len() < 3 || parts[0] != "evo" || parts[1] != "sk" {
        return Err("Invalid API key format".to_string());
    }

    let key_hash = {
        let mut hasher = Sha256::new();
        hasher.update(key.as_bytes());
        hex::encode(hasher.finalize())
    };

    match state.api_key_repo.find_by_key(&key_hash).await {
        Ok(Some(api_key)) => {
            if api_key.status != ApiKeyStatus::Active {
                return Err("API key is not active".to_string());
            }

            if let Some(expires_at) = api_key.expires_at {
                if expires_at < Utc::now() {
                    return Err("API key has expired".to_string());
                }
            }

            Ok(api_key)
        }
        Ok(None) => Err("API key not found".to_string()),
        Err(error) => {
            tracing::error!("Failed to validate API key: {}", error);
            Err("Failed to validate API key".to_string())
        }
    }
}