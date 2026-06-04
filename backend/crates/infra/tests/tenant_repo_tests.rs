//! Integration tests for SqliteTenantRepository

mod test_helpers;

use domain::repository::{TenantRepository, UserRepository};
use domain::tenant::{CreateTenantRequest, TenantPlan, TenantStatus};
use domain::user::{NewUser, TenantRole};
use infra::db::{SqliteTenantRepository, SqliteUserRepository};
use test_helpers::setup_test_db;
use uuid::Uuid;

#[tokio::test]
async fn test_create_tenant() {
    let pool = setup_test_db().await;
    let repo = SqliteTenantRepository::new(pool.clone());
    let owner_id = Uuid::new_v4();

    let request = CreateTenantRequest {
        name: "New Company".to_string(),
        slug: "new-company".to_string(),
        owner_email: "owner@newcompany.com".to_string(),
        owner_username: "owner".to_string(),
        owner_password: "password123".to_string(),
    };

    let tenant = repo.create(request, owner_id).await.unwrap();

    assert!(!tenant.id.is_nil());
    assert_eq!(tenant.name, "New Company");
    assert_eq!(tenant.slug, "new-company");
    assert_eq!(tenant.plan, TenantPlan::Free);
    assert_eq!(tenant.status, TenantStatus::Active);
    assert_eq!(tenant.owner_id, owner_id);
    assert!(tenant.domain.is_none());
}

#[tokio::test]
async fn test_find_by_id() {
    let pool = setup_test_db().await;
    let repo = SqliteTenantRepository::new(pool.clone());
    let owner_id = Uuid::new_v4();

    let request = CreateTenantRequest {
        name: "Find By ID Corp".to_string(),
        slug: "find-by-id".to_string(),
        owner_email: "owner@findbyid.com".to_string(),
        owner_username: "owner".to_string(),
        owner_password: "password123".to_string(),
    };

    let created = repo.create(request, owner_id).await.unwrap();
    let found = repo.find_by_id(created.id).await.unwrap();

    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(found.id, created.id);
    assert_eq!(found.name, "Find By ID Corp");
}

#[tokio::test]
async fn test_update_usage_counts_users() {
    let pool = setup_test_db().await;
    let tenant_repo = SqliteTenantRepository::new(pool.clone());
    let user_repo = SqliteUserRepository::new(pool.clone());
    let owner_id = Uuid::new_v4();

    let request = CreateTenantRequest {
        name: "Usage Test Corp".to_string(),
        slug: "usage-test".to_string(),
        owner_email: "owner@usagetest.com".to_string(),
        owner_username: "owner".to_string(),
        owner_password: "password123".to_string(),
    };

    let tenant = tenant_repo.create(request, owner_id).await.unwrap();
    assert_eq!(
        tenant.usage.current_users, 1,
        "Tenant initially has current_users=1 from create"
    );

    user_repo
        .create(
            NewUser {
                    username: "actualuser".to_string(),
                    email: "actualuser@usagetest.com".to_string(),
                    password_hash: "password123".to_string(),
                },
            tenant.id,
            TenantRole::Member,
        )
        .await
        .unwrap();

    tenant_repo.update_usage(tenant.id).await.unwrap();

    let updated = tenant_repo.find_by_id(tenant.id).await.unwrap().unwrap();
    assert_eq!(
        updated.usage.current_users, 1,
        "update_usage counts actual users in DB"
    );
}

#[tokio::test]
async fn test_find_by_slug() {
    let pool = setup_test_db().await;
    let repo = SqliteTenantRepository::new(pool.clone());
    let owner_id = Uuid::new_v4();

    let request = CreateTenantRequest {
        name: "Slug Search Inc".to_string(),
        slug: "unique-slug-123".to_string(),
        owner_email: "owner@slugsearch.com".to_string(),
        owner_username: "owner".to_string(),
        owner_password: "password123".to_string(),
    };

    let created = repo.create(request, owner_id).await.unwrap();
    let found = repo.find_by_slug("unique-slug-123").await.unwrap();

    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(found.id, created.id);
    assert_eq!(found.slug, "unique-slug-123");
}

#[tokio::test]
async fn test_find_by_domain_returns_none_when_null() {
    let pool = setup_test_db().await;
    let repo = SqliteTenantRepository::new(pool.clone());
    let owner_id = Uuid::new_v4();

    let request = CreateTenantRequest {
        name: "No Domain LLC".to_string(),
        slug: "no-domain".to_string(),
        owner_email: "owner@nodomain.com".to_string(),
        owner_username: "owner".to_string(),
        owner_password: "password123".to_string(),
    };

    repo.create(request, owner_id).await.unwrap();

    let found = repo.find_by_domain("example.com").await.unwrap();
    assert!(
        found.is_none(),
        "Domain is null on create, should return None"
    );
}

#[tokio::test]
async fn test_update_tenant() {
    let pool = setup_test_db().await;
    let repo = SqliteTenantRepository::new(pool.clone());
    let owner_id = Uuid::new_v4();

    let request = CreateTenantRequest {
        name: "Original Name".to_string(),
        slug: "original-slug".to_string(),
        owner_email: "owner@original.com".to_string(),
        owner_username: "owner".to_string(),
        owner_password: "password123".to_string(),
    };

    let created = repo.create(request, owner_id).await.unwrap();

    let update = CreateTenantRequest {
        name: "Updated Name".to_string(),
        slug: "updated-slug".to_string(),
        owner_email: "owner@updated.com".to_string(),
        owner_username: "owner".to_string(),
        owner_password: "password123".to_string(),
    };

    let updated = repo.update(created.id, update).await.unwrap();
    assert_eq!(updated.name, "Updated Name");
    assert_eq!(updated.slug, "updated-slug");
}

#[tokio::test]
async fn test_delete_tenant_soft_delete() {
    let pool = setup_test_db().await;
    let repo = SqliteTenantRepository::new(pool.clone());
    let owner_id = Uuid::new_v4();

    let request = CreateTenantRequest {
        name: "To Be Deleted".to_string(),
        slug: "to-delete".to_string(),
        owner_email: "owner@tobedeleted.com".to_string(),
        owner_username: "owner".to_string(),
        owner_password: "password123".to_string(),
    };

    let created = repo.create(request, owner_id).await.unwrap();
    assert_eq!(created.status, TenantStatus::Active);
    assert!(created.deleted_at.is_none());

    repo.delete(created.id).await.unwrap();

    let deleted = repo.find_by_id(created.id).await.unwrap().unwrap();
    assert_eq!(deleted.status, TenantStatus::Deleted);
    assert!(deleted.deleted_at.is_some());
}

#[tokio::test]
async fn test_update_usage_counts_multiple_users() {
    let pool = setup_test_db().await;
    let tenant_repo = SqliteTenantRepository::new(pool.clone());
    let user_repo = SqliteUserRepository::new(pool.clone());
    let owner_id = Uuid::new_v4();

    let request = CreateTenantRequest {
        name: "Usage Test Corp".to_string(),
        slug: "usage-test".to_string(),
        owner_email: "owner@usagetest.com".to_string(),
        owner_username: "owner".to_string(),
        owner_password: "password123".to_string(),
    };

    let tenant = tenant_repo.create(request, owner_id).await.unwrap();

    user_repo
        .create(
            NewUser {
                    username: "user1".to_string(),
                    email: "user1@usagetest.com".to_string(),
                    password_hash: "password123".to_string(),
                },
            tenant.id,
            TenantRole::Member,
        )
        .await
        .unwrap();

    user_repo
        .create(
            NewUser {
                    username: "user2".to_string(),
                    email: "user2@usagetest.com".to_string(),
                    password_hash: "password123".to_string(),
                },
            tenant.id,
            TenantRole::Member,
        )
        .await
        .unwrap();

    tenant_repo.update_usage(tenant.id).await.unwrap();

    let updated = tenant_repo.find_by_id(tenant.id).await.unwrap().unwrap();
    // update_usage counts actual users in DB for this tenant
    assert!(
        updated.usage.current_users >= 2,
        "Should have at least 2 users created for this tenant"
    );
}
