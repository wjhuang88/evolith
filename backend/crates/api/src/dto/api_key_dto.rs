//! API Key management DTOs
//! Defines request/response structures for API key management

use serde::{Deserialize, Serialize};
use validator::Validate;

/// Create API key request
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateApiKeyRequest {
    #[validate(length(min = 1, max = 128))]
    pub name: String,

    /// Permissions: read, write, admin
    pub permissions: Option<Vec<String>>,

    /// Days until expiration (optional)
    pub expires_in_days: Option<i32>,

    /// Rate limit per hour (optional)
    pub rate_limit: Option<i32>,
}

/// API key response (includes the secret key only once on creation)
#[derive(Debug, Serialize)]
pub struct ApiKeyResponse {
    pub id: String,
    pub name: String,
    pub key: Option<String>, // Only included on creation
    pub key_prefix: String,
    pub permissions: Vec<String>,
    pub expires_at: Option<String>,
    pub rate_limit: i32,
    pub status: String,
    pub created_at: String,
}

/// API key list item (without secret)
#[derive(Debug, Serialize)]
pub struct ApiKeyListItem {
    pub id: String,
    pub name: String,
    pub key_prefix: String,
    pub permissions: Vec<String>,
    pub expires_at: Option<String>,
    pub rate_limit: i32,
    pub status: String,
    pub request_count: i64,
    pub last_used_at: Option<String>,
    pub created_at: String,
}

/// API key list response
#[derive(Debug, Serialize)]
pub struct ApiKeyListResponse {
    pub keys: Vec<ApiKeyListItem>,
    pub total: usize,
}

/// Revoke API key request
#[derive(Debug, Deserialize, Serialize)]
pub struct RevokeApiKeyRequest {
    pub reason: Option<String>,
}
