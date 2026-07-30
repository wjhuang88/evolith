//! API key authorization helpers.
//!
//! New API keys may only be issued with canonical capabilities defined by
//! `domain::api_key::ApiKeyCapability`. Stored permissions remain strings for database and
//! rolling-upgrade compatibility, so existing `write`, `admin`, `commit` and `commit:<branch>`
//! tokens continue to authorize their historical repo operations. They are deliberately not
//! accepted by the create-key DTO.

use domain::api_key::ApiKey;

/// Resource actions that an API key may request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiKeyAction {
    RepoRead,
    RepoWrite,
    ToolExecute,
    Promote,
}

/// Backward-compatible subset used by existing repo handlers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepoScope {
    Read,
    Write,
}

/// Single source of truth for API-key action authorization.
pub fn api_key_allows(api_key: &ApiKey, action: ApiKeyAction) -> bool {
    api_key
        .permissions
        .iter()
        .any(|permission| permission_grants(permission, action))
}

pub fn api_key_allows_repo(api_key: &ApiKey, scope: RepoScope) -> bool {
    api_key_allows(
        api_key,
        match scope {
            RepoScope::Read => ApiKeyAction::RepoRead,
            RepoScope::Write => ApiKeyAction::RepoWrite,
        },
    )
}

pub fn api_key_allows_repo_read(api_key: &ApiKey) -> bool {
    api_key_allows(api_key, ApiKeyAction::RepoRead)
}

pub fn api_key_allows_repo_write(api_key: &ApiKey) -> bool {
    api_key_allows(api_key, ApiKeyAction::RepoWrite)
}

pub fn api_key_allows_tool_execute(api_key: &ApiKey) -> bool {
    api_key_allows(api_key, ApiKeyAction::ToolExecute)
}

pub fn api_key_allows_promote(api_key: &ApiKey) -> bool {
    api_key_allows(api_key, ApiKeyAction::Promote)
}

/// API keys can never manage or mint other API keys, regardless of stored permissions.
pub fn api_key_allows_api_key_management(api_key: &ApiKey) -> bool {
    let _ = api_key;
    false
}

/// Return legacy permission tokens that should be rotated to canonical capabilities.
pub fn legacy_api_key_permissions(api_key: &ApiKey) -> Vec<&str> {
    api_key
        .permissions
        .iter()
        .map(String::as_str)
        .filter(|permission| is_legacy_permission(permission))
        .collect()
}

fn permission_grants(permission: &str, action: ApiKeyAction) -> bool {
    match action {
        ApiKeyAction::RepoRead => {
            matches!(
                permission,
                "read" | "repo:read" | "write" | "repo:write" | "admin" | "commit"
            ) || permission.starts_with("commit:")
        }
        ApiKeyAction::RepoWrite => {
            matches!(permission, "write" | "repo:write" | "admin" | "commit")
                || permission.starts_with("commit:")
        }
        ApiKeyAction::ToolExecute => permission == "execute",
        ApiKeyAction::Promote => permission == "promote",
    }
}

fn is_legacy_permission(permission: &str) -> bool {
    matches!(permission, "write" | "admin" | "commit") || permission.starts_with("commit:")
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use chrono::Utc;
    use domain::api_key::{ApiKey, ApiKeyStatus};
    use uuid::Uuid;

    fn api_key_with(perms: Vec<&str>) -> ApiKey {
        ApiKey {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            name: "test".to_string(),
            key_hash: "deadbeef".to_string(),
            key_prefix: "evo_sk".to_string(),
            permissions: perms.into_iter().map(String::from).collect(),
            status: ApiKeyStatus::Active,
            rate_limit: 1000,
            request_count: 0,
            last_used_at: None,
            expires_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn canonical_repo_capabilities_are_action_scoped() {
        let read = api_key_with(vec!["repo:read"]);
        assert!(api_key_allows_repo_read(&read));
        assert!(!api_key_allows_repo_write(&read));
        assert!(!api_key_allows_tool_execute(&read));

        let write = api_key_with(vec!["repo:write"]);
        assert!(api_key_allows_repo_read(&write));
        assert!(api_key_allows_repo_write(&write));
        assert!(!api_key_allows_tool_execute(&write));
    }

    #[test]
    fn execute_and_promote_do_not_imply_repo_access() {
        let execute = api_key_with(vec!["execute"]);
        assert!(api_key_allows_tool_execute(&execute));
        assert!(!api_key_allows_repo_read(&execute));
        assert!(!api_key_allows_promote(&execute));

        let promote = api_key_with(vec!["promote"]);
        assert!(api_key_allows_promote(&promote));
        assert!(!api_key_allows_repo_read(&promote));
        assert!(!api_key_allows_tool_execute(&promote));
    }

    #[test]
    fn legacy_repo_permissions_remain_runtime_compatible() {
        for permission in ["write", "admin", "commit", "commit:main"] {
            let key = api_key_with(vec![permission]);
            assert!(api_key_allows_repo_read(&key));
            assert!(api_key_allows_repo_write(&key));
            assert!(!api_key_allows_tool_execute(&key));
            assert_eq!(legacy_api_key_permissions(&key), vec![permission]);
        }
    }

    #[test]
    fn api_keys_never_manage_other_keys() {
        let key = api_key_with(vec!["admin", "execute", "repo:write"]);
        assert!(!api_key_allows_api_key_management(&key));
    }

    #[test]
    fn unknown_or_empty_permissions_fail_closed() {
        for permissions in [vec![], vec!["mcp:tools:read"], vec!["unknown"]] {
            let key = api_key_with(permissions);
            assert!(!api_key_allows_repo_read(&key));
            assert!(!api_key_allows_repo_write(&key));
            assert!(!api_key_allows_tool_execute(&key));
            assert!(!api_key_allows_promote(&key));
        }
    }

    #[test]
    fn mixed_permissions_grant_only_their_union() {
        let key = api_key_with(vec!["execute", "repo:write"]);
        assert!(api_key_allows_repo_read(&key));
        assert!(api_key_allows_repo_write(&key));
        assert!(api_key_allows_tool_execute(&key));
        assert!(!api_key_allows_promote(&key));
    }
}
