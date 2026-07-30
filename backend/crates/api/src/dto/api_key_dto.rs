//! API Key management DTOs
//! Defines request/response structures for API key management

use domain::api_key::ApiKeyCapability;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Create API key request
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateApiKeyRequest {
    #[validate(length(min = 1, max = 128))]
    pub name: String,

    /// Canonical capabilities accepted for newly issued keys.
    /// Legacy aliases remain runtime-compatible for existing keys but are rejected here.
    pub permissions: Option<Vec<ApiKeyCapability>>,

    /// Days until expiration (optional)
    pub expires_in_days: Option<i32>,

    /// Rate limit per hour (optional)
    pub rate_limit: Option<i32>,
}

impl CreateApiKeyRequest {
    pub fn permission_tokens(&self) -> Result<Vec<String>, &'static str> {
        let capabilities = self
            .permissions
            .clone()
            .unwrap_or_else(|| vec![ApiKeyCapability::Read]);

        if capabilities.is_empty() {
            return Err("At least one API key capability is required");
        }

        let mut permissions = Vec::with_capacity(capabilities.len());
        for capability in capabilities {
            let token = capability.as_str().to_string();
            if !permissions.contains(&token) {
                permissions.push(token);
            }
        }
        Ok(permissions)
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_to_read_capability() {
        let request = CreateApiKeyRequest {
            name: "default".to_string(),
            permissions: None,
            expires_in_days: None,
            rate_limit: None,
        };
        assert_eq!(request.permission_tokens().expect("permissions"), vec!["read"]);
    }

    #[test]
    fn deduplicates_canonical_capabilities() {
        let request = CreateApiKeyRequest {
            name: "dedupe".to_string(),
            permissions: Some(vec![
                ApiKeyCapability::RepoRead,
                ApiKeyCapability::RepoRead,
                ApiKeyCapability::Execute,
            ]),
            expires_in_days: None,
            rate_limit: None,
        };
        assert_eq!(
            request.permission_tokens().expect("permissions"),
            vec!["repo:read", "execute"]
        );
    }

    #[test]
    fn rejects_empty_capability_list() {
        let request = CreateApiKeyRequest {
            name: "empty".to_string(),
            permissions: Some(vec![]),
            expires_in_days: None,
            rate_limit: None,
        };
        assert!(request.permission_tokens().is_err());
    }

    #[test]
    fn rejects_legacy_capability_during_deserialization() {
        let payload = serde_json::json!({
            "name": "legacy",
            "permissions": ["admin"]
        });
        assert!(serde_json::from_value::<CreateApiKeyRequest>(payload).is_err());
    }
}