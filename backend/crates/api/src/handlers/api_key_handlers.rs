//! API Key management handlers
//! Handles API key creation, listing, and revocation

use std::sync::{Arc, Mutex};

use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use uuid::Uuid;
use validator::Validate;

use crate::dto::api_key_dto::*;
use crate::dto::common::ApiResponse;

/// In-memory API key store for development
pub struct ApiKeyStore {
    keys: Mutex<Vec<StoredApiKey>>,
}

#[derive(Clone)]
pub struct StoredApiKey {
    pub id: String,
    pub tenant_id: String,
    pub user_id: String,
    pub name: String,
    pub key_hash: String,
    pub key: String, // The actual key (for demo)
    pub key_prefix: String,
    pub permissions: Vec<String>,
    pub expires_at: Option<i64>,
    pub last_used_at: Option<i64>,
    pub rate_limit: i32,
    pub status: String,
    pub request_count: i64,
    pub created_at: i64,
}

impl Default for ApiKeyStore {
    fn default() -> Self {
        Self {
            keys: Mutex::new(Vec::new()),
        }
    }
}

/// API Key state
#[derive(Clone)]
pub struct ApiKeyState {
    pub key_store: Arc<ApiKeyStore>,
}

impl ApiKeyState {
    pub fn new() -> Self {
        Self {
            key_store: Arc::new(ApiKeyStore::default()),
        }
    }
}

impl Default for ApiKeyState {
    fn default() -> Self {
        Self::new()
    }
}

/// Generate a new API key
fn generate_api_key() -> (String, String, String) {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let random: u64 = {
        use std::collections::hash_map::RandomState;
        use std::hash::{BuildHasher, Hasher};
        RandomState::new().build_hasher().finish()
    };
    let full_key = format!("evo_sk_{:x}{:016x}", timestamp, random);
    let key_prefix = format!("evo_sk_{:x}", random % 0xFFFFFFFF);
    let key_hash = format!("{:x}", {
        use std::collections::hash_map::RandomState;
        use std::hash::{BuildHasher, Hasher};
        let mut hasher = RandomState::new().build_hasher();
        hasher.write(full_key.as_bytes());
        hasher.finish()
    });
    (full_key, key_prefix, key_hash)
}

/// Simple SHA256 hash (for demo purposes)
fn simple_hash(data: &str) -> String {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};
    format!("{:016x}", {
        let mut hasher = RandomState::new().build_hasher();
        hasher.write(data.as_bytes());
        hasher.finish()
    })
}

/// List API keys for a tenant
pub async fn list_api_keys(
    _tenant_id: web::Path<String>,
    _state: web::Data<ApiKeyState>,
) -> impl Responder {
    // In production, fetch from database
    HttpResponse::Ok().json(ApiResponse::<ApiKeyListResponse>::success(
        ApiKeyListResponse {
            keys: vec![],
            total: 0,
        },
    ))
}

/// Create a new API key
pub async fn create_api_key(
    _tenant_id: web::Path<String>,
    body: web::Json<CreateApiKeyRequest>,
    state: web::Data<ApiKeyState>,
) -> impl Responder {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "VALIDATION_ERROR",
            &errors.to_string(),
        ));
    }

    let name = body.name.clone();
    let permissions = body
        .permissions
        .clone()
        .unwrap_or_else(|| vec!["read".to_string()]);
    let rate_limit = body.rate_limit.unwrap_or(1000);
    let expires_in_days = body.expires_in_days;

    // Generate API key
    let (full_key, key_prefix, key_hash) = generate_api_key();
    let now = Utc::now().timestamp();
    let expires_at = expires_in_days.map(|days| now + (days as i64 * 24 * 60 * 60));

    let api_key = StoredApiKey {
        id: Uuid::new_v4().to_string(),
        tenant_id: "00000000-0000-0000-0000-000000000001".to_string(),
        user_id: "00000000-0000-0000-0000-000000000001".to_string(),
        name: name.clone(),
        key_hash,
        key: full_key.clone(),
        key_prefix: key_prefix.clone(),
        permissions: permissions.clone(),
        expires_at,
        last_used_at: None,
        rate_limit,
        status: "active".to_string(),
        request_count: 0,
        created_at: now,
    };

    // Store the key
    {
        let mut keys = state.key_store.keys.lock().unwrap();
        keys.push(api_key.clone());
    }

    // Return response with the full key (only shown once!)
    let expires_at_str = expires_at.map(|ts| {
        chrono::DateTime::from_timestamp(ts, 0)
            .unwrap_or_default()
            .to_rfc3339()
    });

    let response = ApiKeyResponse {
        id: api_key.id,
        name: api_key.name,
        key: Some(full_key), // ⚠️ Only shown once!
        key_prefix,
        permissions,
        expires_at: expires_at_str,
        rate_limit,
        status: api_key.status,
        created_at: chrono::DateTime::from_timestamp(api_key.created_at, 0)
            .unwrap_or_default()
            .to_rfc3339(),
    };

    HttpResponse::Ok().json(ApiResponse::<ApiKeyResponse>::success(response))
}

/// Get API key details
pub async fn get_api_key(
    _tenant_id: web::Path<String>,
    _key_id: web::Path<String>,
    _state: web::Data<ApiKeyState>,
) -> impl Responder {
    // In production, fetch from database
    HttpResponse::Ok().json(ApiResponse::<()>::error(
        "NOT_IMPLEMENTED",
        "Get API key details not implemented yet",
    ))
}

/// Revoke an API key
pub async fn revoke_api_key(
    _tenant_id: web::Path<String>,
    _key_id: web::Path<String>,
    _body: web::Json<RevokeApiKeyRequest>,
    _state: web::Data<ApiKeyState>,
) -> impl Responder {
    // In production:
    // 1. Check permissions
    // 2. Mark key as revoked in database

    HttpResponse::Ok().json(ApiResponse::<serde_json::Value>::success(
        serde_json::json!({
            "message": "API key revoked successfully"
        }),
    ))
}

/// Rotate an API key (regenerate)
pub async fn rotate_api_key(
    _tenant_id: web::Path<String>,
    _key_id: web::Path<String>,
    _state: web::Data<ApiKeyState>,
) -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::<()>::error(
        "NOT_IMPLEMENTED",
        "API key rotation not implemented yet",
    ))
}

/// Validate an API key (for middleware)
pub async fn validate_api_key(key: &str, state: &ApiKeyState) -> Result<StoredApiKey, String> {
    // Extract prefix from key
    let parts: Vec<&str> = key.split('_').collect();
    if parts.len() < 3 || parts[0] != "evo" || parts[1] != "sk" {
        return Err("Invalid API key format".to_string());
    }

    let prefix = format!("evo_sk_{}", parts[2]);

    // Find key by prefix
    let keys = state.key_store.keys.lock().unwrap();
    let key_data = keys
        .iter()
        .find(|k| k.key_prefix == prefix && k.status == "active")
        .cloned()
        .ok_or("API key not found")?;

    // Verify key
    let key_hash = simple_hash(key);
    if key_hash != key_data.key_hash {
        return Err("Invalid API key".to_string());
    }

    // Check expiration
    if let Some(expires_at) = key_data.expires_at {
        if expires_at < Utc::now().timestamp() {
            return Err("API key has expired".to_string());
        }
    }

    Ok(key_data)
}
