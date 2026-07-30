//! API Key domain model

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Capabilities that may be issued to new API keys.
///
/// Stored API key permissions remain strings for database compatibility. This enum is the
/// canonical issuance contract: legacy aliases such as `write`, `admin`, `commit` and
/// `commit:<branch>` may still be interpreted by authorization helpers for existing keys, but
/// they cannot be deserialized through the create-key API.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ApiKeyCapability {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "repo:read")]
    RepoRead,
    #[serde(rename = "repo:write")]
    RepoWrite,
    #[serde(rename = "execute")]
    Execute,
    #[serde(rename = "promote")]
    Promote,
}

impl ApiKeyCapability {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Read => "read",
            Self::RepoRead => "repo:read",
            Self::RepoWrite => "repo:write",
            Self::Execute => "execute",
            Self::Promote => "promote",
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_capabilities_round_trip() {
        for (capability, token) in [
            (ApiKeyCapability::Read, "read"),
            (ApiKeyCapability::RepoRead, "repo:read"),
            (ApiKeyCapability::RepoWrite, "repo:write"),
            (ApiKeyCapability::Execute, "execute"),
            (ApiKeyCapability::Promote, "promote"),
        ] {
            assert_eq!(capability.as_str(), token);
            assert_eq!(serde_json::to_string(&capability).expect("serialize"), format!("\"{}\"", token));
            assert_eq!(
                serde_json::from_str::<ApiKeyCapability>(&format!("\"{}\"", token))
                    .expect("deserialize"),
                capability
            );
        }
    }

    #[test]
    fn legacy_and_unknown_capabilities_cannot_be_newly_issued() {
        for token in ["write", "admin", "manage_keys", "commit", "commit:main", "unknown"] {
            assert!(serde_json::from_str::<ApiKeyCapability>(&format!("\"{}\"", token)).is_err());
        }
    }
}