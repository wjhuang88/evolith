//! Integration tests for SqliteApiKeyRepository

// Workspace `[lints.clippy] unwrap_used = "deny"` overrides `clippy.toml`
// `allow-unwrap-in-tests`; tests need unwrap for concise assertion failures.
#![allow(clippy::unwrap_used)]

mod test_helpers;

use chrono::{Duration, Utc};
use domain::api_key::{ApiKeyStatus, NewApiKey};
use domain::repository::{ApiKeyRepository, TenantRepository, UserRepository};
use domain::tenant::CreateTenantRequest;
use domain::user::{NewUser, TenantRole};
use infra::db::{SqliteApiKeyRepository, SqliteTenantRepository, SqliteUserRepository};
use test_helpers::setup_test_db;
use uuid::Uuid;

async fn create_test_tenant_and_user(pool: &sqlx::SqlitePool) -> (Uuid, Uuid) {
    let tenant_repo = SqliteTenantRepository::new(pool.clone());
    let user_repo = SqliteUserRepository::new(pool.clone());

    let owner_id = Uuid::new_v4();
    let tenant = tenant_repo
        .create(
            CreateTenantRequest {
                name: "ApiKey Test Tenant".to_string(),
                slug: "apikey-test".to_string(),
                owner_email: "owner@apikeytest.com".to_string(),
                owner_username: "owner".to_string(),
                owner_password: "password123".to_string(),
            },
            owner_id,
        )
        .await
        .unwrap();

    let user = user_repo
        .create(
            NewUser {
                username: "apikeyuser".to_string(),
                email: "apikeyuser@apikeytest.com".to_string(),
                password: "password123".to_string(),
            },
            tenant.id,
            TenantRole::Admin,
        )
        .await
        .unwrap();

    (tenant.id, user.id)
}

#[tokio::test]
async fn test_create_api_key() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteApiKeyRepository::new(pool.clone());

    let new_key = NewApiKey {
        tenant_id,
        user_id,
        name: "Test API Key".to_string(),
        key_hash: "hashed_key_123".to_string(),
        key_prefix: "evol_abc".to_string(),
        permissions: vec!["tools:read".to_string(), "tools:write".to_string()],
        rate_limit: Some(500),
        expires_at: None,
    };

    let api_key = repo.create(new_key).await.unwrap();

    assert!(!api_key.id.is_nil());
    assert_eq!(api_key.tenant_id, tenant_id);
    assert_eq!(api_key.user_id, user_id);
    assert_eq!(api_key.name, "Test API Key");
    assert_eq!(api_key.key_hash, "hashed_key_123");
    assert_eq!(api_key.key_prefix, "evol_abc");
    assert_eq!(api_key.permissions, vec!["tools:read", "tools:write"]);
    assert_eq!(api_key.status, ApiKeyStatus::Active);
    assert_eq!(api_key.rate_limit, 500);
    assert_eq!(api_key.request_count, 0);
    assert!(api_key.last_used_at.is_none());
    assert!(api_key.expires_at.is_none());
}

#[tokio::test]
async fn test_create_api_key_with_expiration() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteApiKeyRepository::new(pool.clone());

    let expires_at = Utc::now() + Duration::days(30);
    let new_key = NewApiKey {
        tenant_id,
        user_id,
        name: "Expiring Key".to_string(),
        key_hash: "hashed_expiry".to_string(),
        key_prefix: "evol_exp".to_string(),
        permissions: vec!["skills:read".to_string()],
        rate_limit: None,
        expires_at: Some(expires_at),
    };

    let api_key = repo.create(new_key).await.unwrap();

    assert!(api_key.expires_at.is_some());
    assert_eq!(api_key.rate_limit, 1000);
}

#[tokio::test]
async fn test_find_by_id() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteApiKeyRepository::new(pool.clone());

    let new_key = NewApiKey {
        tenant_id,
        user_id,
        name: "Find By ID Key".to_string(),
        key_hash: "hash_findbyid".to_string(),
        key_prefix: "evol_fnd".to_string(),
        permissions: vec!["snippets:read".to_string()],
        rate_limit: Some(100),
        expires_at: None,
    };

    let created = repo.create(new_key).await.unwrap();
    let found = repo.find_by_id(created.id).await.unwrap();

    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(found.id, created.id);
    assert_eq!(found.name, "Find By ID Key");
}

#[tokio::test]
async fn test_find_by_id_not_found() {
    let pool = setup_test_db().await;
    let repo = SqliteApiKeyRepository::new(pool.clone());

    let result = repo.find_by_id(Uuid::new_v4()).await.unwrap();
    assert!(result.is_none());
}

#[tokio::test]
async fn test_find_by_key() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteApiKeyRepository::new(pool.clone());

    let key_hash = "unique_hash_for_lookup";
    let new_key = NewApiKey {
        tenant_id,
        user_id,
        name: "Key Lookup Test".to_string(),
        key_hash: key_hash.to_string(),
        key_prefix: "evol_lkp".to_string(),
        permissions: vec![],
        rate_limit: None,
        expires_at: None,
    };

    let created = repo.create(new_key).await.unwrap();
    let found = repo.find_by_key(key_hash).await.unwrap();

    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(found.id, created.id);
    assert_eq!(found.key_hash, key_hash);
}

#[tokio::test]
async fn test_find_by_key_not_found() {
    let pool = setup_test_db().await;
    let repo = SqliteApiKeyRepository::new(pool.clone());

    let result = repo.find_by_key("nonexistent_hash").await.unwrap();
    assert!(result.is_none());
}

#[tokio::test]
async fn test_find_by_tenant() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteApiKeyRepository::new(pool.clone());

    let key1 = NewApiKey {
        tenant_id,
        user_id,
        name: "Tenant Key 1".to_string(),
        key_hash: "hash_tenant_1".to_string(),
        key_prefix: "evol_t1".to_string(),
        permissions: vec![],
        rate_limit: None,
        expires_at: None,
    };
    let key2 = NewApiKey {
        tenant_id,
        user_id,
        name: "Tenant Key 2".to_string(),
        key_hash: "hash_tenant_2".to_string(),
        key_prefix: "evol_t2".to_string(),
        permissions: vec![],
        rate_limit: None,
        expires_at: None,
    };

    repo.create(key1).await.unwrap();
    repo.create(key2).await.unwrap();

    let keys = repo.find_by_tenant(tenant_id).await.unwrap();
    assert_eq!(keys.len(), 2);
}

#[tokio::test]
async fn test_find_by_tenant_empty() {
    let pool = setup_test_db().await;
    let repo = SqliteApiKeyRepository::new(pool.clone());

    let keys = repo.find_by_tenant(Uuid::new_v4()).await.unwrap();
    assert!(keys.is_empty());
}

#[tokio::test]
async fn test_revoke_api_key() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteApiKeyRepository::new(pool.clone());

    let new_key = NewApiKey {
        tenant_id,
        user_id,
        name: "To Revoke Key".to_string(),
        key_hash: "hash_revoke".to_string(),
        key_prefix: "evol_rvk".to_string(),
        permissions: vec![],
        rate_limit: None,
        expires_at: None,
    };

    let created = repo.create(new_key).await.unwrap();
    assert_eq!(created.status, ApiKeyStatus::Active);

    repo.revoke(created.id).await.unwrap();

    let revoked = repo.find_by_id(created.id).await.unwrap().unwrap();
    assert_eq!(revoked.status, ApiKeyStatus::Revoked);
}

#[tokio::test]
async fn test_delete_api_key() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteApiKeyRepository::new(pool.clone());

    let new_key = NewApiKey {
        tenant_id,
        user_id,
        name: "To Delete Key".to_string(),
        key_hash: "hash_delete".to_string(),
        key_prefix: "evol_del".to_string(),
        permissions: vec![],
        rate_limit: None,
        expires_at: None,
    };

    let created = repo.create(new_key).await.unwrap();

    repo.delete(created.id).await.unwrap();

    let found = repo.find_by_id(created.id).await.unwrap();
    assert!(found.is_none());
}

#[tokio::test]
async fn test_update_last_used() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteApiKeyRepository::new(pool.clone());

    let new_key = NewApiKey {
        tenant_id,
        user_id,
        name: "Usage Tracking Key".to_string(),
        key_hash: "hash_usage".to_string(),
        key_prefix: "evol_usg".to_string(),
        permissions: vec![],
        rate_limit: None,
        expires_at: None,
    };

    let created = repo.create(new_key).await.unwrap();
    assert!(created.last_used_at.is_none());
    assert_eq!(created.request_count, 0);

    repo.update_last_used(created.id).await.unwrap();

    let updated = repo.find_by_id(created.id).await.unwrap().unwrap();
    assert!(updated.last_used_at.is_some());
    assert_eq!(updated.request_count, 1);

    repo.update_last_used(created.id).await.unwrap();

    let updated2 = repo.find_by_id(created.id).await.unwrap().unwrap();
    assert_eq!(updated2.request_count, 2);
}
