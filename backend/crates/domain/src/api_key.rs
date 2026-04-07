//! API Key domain model

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub key_hash: String,
    pub key_prefix: String,
    pub permissions: Vec<String>,
    pub status: ApiKeyStatus,
    pub rate_limit: u32,
    pub request_count: u32,
    pub last_used_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ApiKeyStatus {
    #[default]
    Active,
    Revoked,
    Expired,
}

#[derive(Debug, Clone)]
pub struct NewApiKey {
    pub tenant_id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub key_hash: String,
    pub key_prefix: String,
    pub permissions: Vec<String>,
    pub rate_limit: Option<u32>,
    pub expires_at: Option<DateTime<Utc>>,
}
