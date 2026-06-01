//! Integration tests for SqliteUserRepository

// Workspace `[lints.clippy] unwrap_used = "deny"` overrides `clippy.toml`
// `allow-unwrap-in-tests`; tests need unwrap for concise assertion failures.
#![allow(clippy::unwrap_used)]

mod test_helpers;

use chrono::{Duration, Utc};
use domain::repository::{TenantRepository, UserRepository};
use domain::tenant::CreateTenantRequest;
use domain::user::{NewUser, TenantRole, UpdateUser};
use infra::db::{SqliteTenantRepository, SqliteUserRepository};
use test_helpers::setup_test_db;
use uuid::Uuid;

async fn create_test_tenant(pool: &sqlx::SqlitePool) -> (Uuid, Uuid) {
    let tenant_repo = SqliteTenantRepository::new(pool.clone());
    let owner_id = Uuid::new_v4();
    let tenant = tenant_repo
        .create(
            CreateTenantRequest {
                name: "Test Tenant".to_string(),
                slug: "test-tenant".to_string(),
                owner_email: "owner@test.com".to_string(),
                owner_username: "owner".to_string(),
                owner_password: "password123".to_string(),
            },
            owner_id,
        )
        .await
        .unwrap();
    (tenant.id, owner_id)
}

#[tokio::test]
async fn test_create_user() {
    let pool = setup_test_db().await;
    let (tenant_id, _) = create_test_tenant(&pool).await;
    let repo = SqliteUserRepository::new(pool.clone());

    let new_user = NewUser {
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };

    let user = repo
        .create(new_user, tenant_id, TenantRole::Member)
        .await
        .unwrap();

    assert!(!user.id.is_nil());
    assert_eq!(user.username, "testuser");
    assert_eq!(user.email, "test@example.com");
    assert_eq!(user.tenant_id, tenant_id);
    assert_eq!(user.tenant_role, TenantRole::Member);
    assert!(!user.email_verified);
}

#[tokio::test]
async fn test_find_by_id() {
    let pool = setup_test_db().await;
    let (tenant_id, _) = create_test_tenant(&pool).await;
    let repo = SqliteUserRepository::new(pool.clone());

    let new_user = NewUser {
        username: "findbyid".to_string(),
        email: "findbyid@example.com".to_string(),
        password: "password123".to_string(),
    };

    let created = repo
        .create(new_user, tenant_id, TenantRole::Member)
        .await
        .unwrap();
    let found = repo.find_by_id(created.id).await.unwrap();

    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(found.id, created.id);
    assert_eq!(found.username, "findbyid");
}

#[tokio::test]
async fn test_find_by_id_not_found() {
    let pool = setup_test_db().await;
    let repo = SqliteUserRepository::new(pool.clone());

    let result = repo.find_by_id(Uuid::new_v4()).await.unwrap();
    assert!(result.is_none());
}

#[tokio::test]
async fn test_find_by_email() {
    let pool = setup_test_db().await;
    let (tenant_id, _) = create_test_tenant(&pool).await;
    let repo = SqliteUserRepository::new(pool.clone());

    let new_user = NewUser {
        username: "emailuser".to_string(),
        email: "emailuser@example.com".to_string(),
        password: "password123".to_string(),
    };

    let created = repo
        .create(new_user, tenant_id, TenantRole::Member)
        .await
        .unwrap();
    let found = repo.find_by_email("emailuser@example.com").await.unwrap();

    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(found.id, created.id);
    assert_eq!(found.email, "emailuser@example.com");
}

#[tokio::test]
async fn test_find_by_username() {
    let pool = setup_test_db().await;
    let (tenant_id, _) = create_test_tenant(&pool).await;
    let repo = SqliteUserRepository::new(pool.clone());

    let new_user = NewUser {
        username: "usernameuser".to_string(),
        email: "usernameuser@example.com".to_string(),
        password: "password123".to_string(),
    };

    let created = repo
        .create(new_user, tenant_id, TenantRole::Member)
        .await
        .unwrap();
    let found = repo
        .find_by_username("usernameuser", tenant_id)
        .await
        .unwrap();

    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(found.id, created.id);
    assert_eq!(found.username, "usernameuser");
}

#[tokio::test]
async fn test_update_username() {
    let pool = setup_test_db().await;
    let (tenant_id, _) = create_test_tenant(&pool).await;
    let repo = SqliteUserRepository::new(pool.clone());

    let new_user = NewUser {
        username: "originalname".to_string(),
        email: "original@example.com".to_string(),
        password: "password123".to_string(),
    };

    let created = repo
        .create(new_user, tenant_id, TenantRole::Member)
        .await
        .unwrap();

    let update = UpdateUser {
        username: Some("newname".to_string()),
        email: None,
    };

    let updated = repo.update(created.id, update).await.unwrap();
    assert_eq!(updated.username, "newname");
    assert_eq!(updated.email, "original@example.com");
}

#[tokio::test]
async fn test_update_email() {
    let pool = setup_test_db().await;
    let (tenant_id, _) = create_test_tenant(&pool).await;
    let repo = SqliteUserRepository::new(pool.clone());

    let new_user = NewUser {
        username: "emailupdater".to_string(),
        email: "oldemail@example.com".to_string(),
        password: "password123".to_string(),
    };

    let created = repo
        .create(new_user, tenant_id, TenantRole::Member)
        .await
        .unwrap();

    let update = UpdateUser {
        username: None,
        email: Some("newemail@example.com".to_string()),
    };

    let updated = repo.update(created.id, update).await.unwrap();
    assert_eq!(updated.email, "newemail@example.com");
    assert_eq!(updated.username, "emailupdater");
}

#[tokio::test]
async fn test_delete_user() {
    let pool = setup_test_db().await;
    let (tenant_id, _) = create_test_tenant(&pool).await;
    let repo = SqliteUserRepository::new(pool.clone());

    let new_user = NewUser {
        username: "deleteuser".to_string(),
        email: "deleteuser@example.com".to_string(),
        password: "password123".to_string(),
    };

    let created = repo
        .create(new_user, tenant_id, TenantRole::Member)
        .await
        .unwrap();

    repo.delete(created.id).await.unwrap();

    let found = repo.find_by_id(created.id).await.unwrap();
    assert!(found.is_none());
}

#[tokio::test]
async fn test_verify_email() {
    let pool = setup_test_db().await;
    let (tenant_id, _) = create_test_tenant(&pool).await;
    let repo = SqliteUserRepository::new(pool.clone());

    let new_user = NewUser {
        username: "verifyuser".to_string(),
        email: "verifyuser@example.com".to_string(),
        password: "password123".to_string(),
    };

    let created = repo
        .create(new_user, tenant_id, TenantRole::Member)
        .await
        .unwrap();
    assert!(!created.email_verified);
    assert!(created.verified_at.is_none());

    repo.verify_email(created.id).await.unwrap();

    let verified = repo.find_by_id(created.id).await.unwrap().unwrap();
    assert!(verified.email_verified);
    assert!(verified.verified_at.is_some());
    assert!(verified.verify_token.is_none());
}

#[tokio::test]
async fn test_set_and_find_verify_token() {
    let pool = setup_test_db().await;
    let (tenant_id, _) = create_test_tenant(&pool).await;
    let repo = SqliteUserRepository::new(pool.clone());

    let new_user = NewUser {
        username: "tokenuser".to_string(),
        email: "tokenuser@example.com".to_string(),
        password: "password123".to_string(),
    };

    let created = repo
        .create(new_user, tenant_id, TenantRole::Member)
        .await
        .unwrap();
    let token = "verify-token-12345";

    repo.set_verify_token(created.id, token).await.unwrap();

    let found = repo.find_by_verify_token(token).await.unwrap();
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(found.id, created.id);
    assert_eq!(found.verify_token, Some(token.to_string()));
}

#[tokio::test]
async fn test_set_and_find_reset_token() {
    let pool = setup_test_db().await;
    let (tenant_id, _) = create_test_tenant(&pool).await;
    let repo = SqliteUserRepository::new(pool.clone());

    let new_user = NewUser {
        username: "resetuser".to_string(),
        email: "resetuser@example.com".to_string(),
        password: "password123".to_string(),
    };

    let created = repo
        .create(new_user, tenant_id, TenantRole::Member)
        .await
        .unwrap();
    let token = "reset-token-abcde";
    let expires_at = Utc::now() + Duration::hours(1);

    repo.set_reset_token(created.id, token, expires_at)
        .await
        .unwrap();

    let found = repo.find_by_reset_token(token).await.unwrap();
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(found.id, created.id);
    assert_eq!(found.reset_token, Some(token.to_string()));
    assert!(found.reset_expires_at.is_some());
}

#[tokio::test]
async fn test_clear_reset_token() {
    let pool = setup_test_db().await;
    let (tenant_id, _) = create_test_tenant(&pool).await;
    let repo = SqliteUserRepository::new(pool.clone());

    let new_user = NewUser {
        username: "clearreset".to_string(),
        email: "clearreset@example.com".to_string(),
        password: "password123".to_string(),
    };

    let created = repo
        .create(new_user, tenant_id, TenantRole::Member)
        .await
        .unwrap();
    let token = "temp-reset-token";
    let expires_at = Utc::now() + Duration::hours(1);

    repo.set_reset_token(created.id, token, expires_at)
        .await
        .unwrap();

    let before_clear = repo.find_by_id(created.id).await.unwrap().unwrap();
    assert!(before_clear.reset_token.is_some());

    repo.clear_reset_token(created.id).await.unwrap();

    let after_clear = repo.find_by_id(created.id).await.unwrap().unwrap();
    assert!(after_clear.reset_token.is_none());
    assert!(after_clear.reset_expires_at.is_none());
}

#[tokio::test]
async fn test_update_password() {
    let pool = setup_test_db().await;
    let (tenant_id, _) = create_test_tenant(&pool).await;
    let repo = SqliteUserRepository::new(pool.clone());

    let new_user = NewUser {
        username: "pwduser".to_string(),
        email: "pwduser@example.com".to_string(),
        password: "oldpassword".to_string(),
    };

    let created = repo
        .create(new_user, tenant_id, TenantRole::Member)
        .await
        .unwrap();
    let new_hash = "$argon2id$new_hash_here";

    repo.update_password(created.id, new_hash).await.unwrap();

    let updated = repo.find_by_id(created.id).await.unwrap().unwrap();
    assert_eq!(updated.password_hash, new_hash);
}
