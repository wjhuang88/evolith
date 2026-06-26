//! API key scope helpers
//!
//! Single source of truth for translating `ApiKey.permissions` strings into
//! resource-action capabilities. Handlers MUST use these helpers instead of
//! rolling their own string checks so that read-only keys cannot accidentally
//! escalate to repo writes or key management.
//!
//! Permission taxonomy (current implementations, all case-sensitive):
//!
//! | Permission token           | Read | Write | Manage keys | Notes |
//! |----------------------------|:----:|:-----:|:-----------:|-------|
//! | `read` / `repo:read`       |  ✓   |       |             | clone/fetch + Context API |
//! | `write` / `repo:write`     |  ✓   |   ✓   |             | push + Repo CRUD |
//! | `admin`                    |  ✓   |   ✓   |             | full repo (key management still JWT-only) |
//! | `commit`                   |  ✓   |   ✓   |             | Smart HTTP push |
//! | `commit:*` (e.g. `commit:main`) |  ✓ |  ✓ |         | scoped commit permission |
//! | `execute`                  |      |       |             | MCP tool execution only — NO repo access |
//! | other / empty              |      |       |             | no implicit access |
//!
//! Notes:
//! - API keys are NEVER allowed to manage other API keys regardless of
//!   permissions. Management is JWT-only and is enforced in the handler.
//! - The `admin` token grants repo write/read but still does NOT bypass the
//!   key-management restriction above.

use domain::api_key::ApiKey;

/// A subset of the Smart HTTP / Repo operations that need a scope check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepoScope {
    /// clone/fetch via `git-upload-pack`, GET Repo Context endpoints, list/get repo.
    Read,
    /// push via `git-receive-pack`, POST/PATCH/DELETE Repo CRUD.
    Write,
}

/// Return `true` if `api_key` is allowed to perform `scope` on repos owned by
/// the same tenant as the key. Returns `false` when no matching permission is
/// present (including when the key has only `execute` or unrelated tokens).
///
/// Use this helper from every handler that touches Repo CRUD, Context API or
/// Smart HTTP. JWT-authenticated requests should bypass this helper entirely
/// (their existing `CurrentUser.tenant_role` checks are authoritative).
pub fn api_key_allows_repo(api_key: &ApiKey, scope: RepoScope) -> bool {
    api_key.permissions.iter().any(|permission| {
        let p = permission.as_str();
        match scope {
            RepoScope::Read => is_repo_read_permission(p),
            RepoScope::Write => is_repo_write_permission(p),
        }
    })
}

/// Convenience wrapper for read-only checks.
pub fn api_key_allows_repo_read(api_key: &ApiKey) -> bool {
    api_key_allows_repo(api_key, RepoScope::Read)
}

/// Convenience wrapper for write checks.
pub fn api_key_allows_repo_write(api_key: &ApiKey) -> bool {
    api_key_allows_repo(api_key, RepoScope::Write)
}

/// API keys are NEVER allowed to manage other API keys. This helper exists so
/// handlers have a single, auditable answer for the question "may this caller
/// rotate keys?" without re-implementing it inline.
pub fn api_key_allows_api_key_management(api_key: &ApiKey) -> bool {
    let _ = api_key;
    false
}

fn is_repo_read_permission(p: &str) -> bool {
    matches!(
        p,
        "read" | "repo:read" | "write" | "repo:write" | "admin" | "commit"
    ) || p.starts_with("commit:")
}

fn is_repo_write_permission(p: &str) -> bool {
    matches!(p, "write" | "repo:write" | "admin" | "commit") || p.starts_with("commit:")
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
    fn read_only_key_allows_repo_read_only() {
        let k = api_key_with(vec!["repo:read"]);
        assert!(api_key_allows_repo_read(&k));
        assert!(!api_key_allows_repo_write(&k));
    }

    #[test]
    fn read_token_alias_works() {
        let k = api_key_with(vec!["read"]);
        assert!(api_key_allows_repo_read(&k));
        assert!(!api_key_allows_repo_write(&k));
    }

    #[test]
    fn write_key_allows_both() {
        let k = api_key_with(vec!["repo:write"]);
        assert!(api_key_allows_repo_read(&k));
        assert!(api_key_allows_repo_write(&k));
    }

    #[test]
    fn write_alias_allows_both() {
        let k = api_key_with(vec!["write"]);
        assert!(api_key_allows_repo_read(&k));
        assert!(api_key_allows_repo_write(&k));
    }

    #[test]
    fn admin_allows_both() {
        let k = api_key_with(vec!["admin"]);
        assert!(api_key_allows_repo_read(&k));
        assert!(api_key_allows_repo_write(&k));
    }

    #[test]
    fn commit_token_allows_write() {
        let k = api_key_with(vec!["commit"]);
        assert!(api_key_allows_repo_read(&k));
        assert!(api_key_allows_repo_write(&k));
    }

    #[test]
    fn commit_scoped_token_allows_write() {
        let k = api_key_with(vec!["commit:main"]);
        assert!(api_key_allows_repo_read(&k));
        assert!(api_key_allows_repo_write(&k));
    }

    #[test]
    fn execute_only_key_cannot_access_repo() {
        let k = api_key_with(vec!["execute"]);
        assert!(!api_key_allows_repo_read(&k));
        assert!(!api_key_allows_repo_write(&k));
    }

    #[test]
    fn empty_permissions_cannot_access_repo() {
        let k = api_key_with(vec![]);
        assert!(!api_key_allows_repo_read(&k));
        assert!(!api_key_allows_repo_write(&k));
    }

    #[test]
    fn unknown_permission_cannot_access_repo() {
        let k = api_key_with(vec!["mcp:tools:read"]);
        assert!(!api_key_allows_repo_read(&k));
        assert!(!api_key_allows_repo_write(&k));
    }

    #[test]
    fn admin_key_still_cannot_manage_api_keys() {
        let k = api_key_with(vec!["admin"]);
        assert!(!api_key_allows_api_key_management(&k));
    }

    #[test]
    fn mixed_permissions_grant_union() {
        let k = api_key_with(vec!["execute", "repo:write"]);
        assert!(api_key_allows_repo_read(&k));
        assert!(api_key_allows_repo_write(&k));
    }
}
