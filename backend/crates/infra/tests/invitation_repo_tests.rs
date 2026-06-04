//! Integration tests for SqliteInvitationRepository

// Workspace `[lints.clippy] unwrap_used = "deny"` overrides `clippy.toml`
// `allow-unwrap-in-tests`; tests need unwrap for concise assertion failures.
#![allow(clippy::unwrap_used)]

mod test_helpers;

use chrono::{Duration, Utc};
use domain::repository::NewInvitation;
use domain::repository::{InvitationRepository, TenantRepository, UserRepository};
use domain::tenant::CreateTenantRequest;
use domain::user::{NewUser, TenantRole};
use infra::db::{SqliteInvitationRepository, SqliteTenantRepository, SqliteUserRepository};
use test_helpers::setup_test_db;
use uuid::Uuid;

async fn setup_tenant_with_user(pool: &sqlx::SqlitePool) -> (Uuid, Uuid, Uuid) {
    let tenant_repo = SqliteTenantRepository::new(pool.clone());
    let user_repo = SqliteUserRepository::new(pool.clone());

    let owner_id = Uuid::new_v4();
    let tenant = tenant_repo
        .create(
            CreateTenantRequest {
                name: "Invitation Test Tenant".to_string(),
                slug: "invitation-test".to_string(),
                owner_email: "owner@invitationtest.com".to_string(),
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
                    username: "inviter".to_string(),
                    email: "inviter@invitationtest.com".to_string(),
                    password_hash: "password123".to_string(),
                },
            tenant.id,
            TenantRole::Admin,
        )
        .await
        .unwrap();

    (tenant.id, user.id, owner_id)
}

#[tokio::test]
async fn test_create_invitation() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id, _) = setup_tenant_with_user(&pool).await;
    let repo = SqliteInvitationRepository::new(pool.clone());

    let invitation = NewInvitation::new(
        tenant_id,
        "newuser@example.com".to_string(),
        "member".to_string(),
        "invite-token-123".to_string(),
        Utc::now() + Duration::days(7),
        user_id,
    );

    let created = repo.create(invitation).await.unwrap();

    assert!(!created.id.is_nil());
    assert_eq!(created.tenant_id, tenant_id);
    assert_eq!(created.email, "newuser@example.com");
    assert_eq!(created.role, "member");
    assert_eq!(created.token, "invite-token-123");
    assert!(created.accepted_at.is_none());
}

#[tokio::test]
async fn test_find_by_id() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id, _) = setup_tenant_with_user(&pool).await;
    let repo = SqliteInvitationRepository::new(pool.clone());

    let invitation = NewInvitation::new(
        tenant_id,
        "findbyid@example.com".to_string(),
        "admin".to_string(),
        "token-findbyid".to_string(),
        Utc::now() + Duration::days(7),
        user_id,
    );

    let created = repo.create(invitation).await.unwrap();
    let found = repo.find_by_id(created.id).await.unwrap();

    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(found.id, created.id);
    assert_eq!(found.email, "findbyid@example.com");
}

#[tokio::test]
async fn test_find_by_token() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id, _) = setup_tenant_with_user(&pool).await;
    let repo = SqliteInvitationRepository::new(pool.clone());

    let unique_token = "unique-token-abc123";
    let invitation = NewInvitation::new(
        tenant_id,
        "bytoken@example.com".to_string(),
        "member".to_string(),
        unique_token.to_string(),
        Utc::now() + Duration::days(7),
        user_id,
    );

    let created = repo.create(invitation).await.unwrap();
    let found = repo.find_by_token(unique_token).await.unwrap();

    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(found.id, created.id);
    assert_eq!(found.token, unique_token);
}

#[tokio::test]
async fn test_find_by_tenant_returns_pending_only() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id, _) = setup_tenant_with_user(&pool).await;
    let repo = SqliteInvitationRepository::new(pool.clone());

    let inv1 = NewInvitation::new(
        tenant_id,
        "pending1@example.com".to_string(),
        "member".to_string(),
        "token-pending-1".to_string(),
        Utc::now() + Duration::days(7),
        user_id,
    );
    let inv2 = NewInvitation::new(
        tenant_id,
        "pending2@example.com".to_string(),
        "admin".to_string(),
        "token-pending-2".to_string(),
        Utc::now() + Duration::days(7),
        user_id,
    );

    let created1 = repo.create(inv1).await.unwrap();
    repo.create(inv2).await.unwrap();

    let pending = repo.find_by_tenant(tenant_id).await.unwrap();
    assert_eq!(pending.len(), 2, "Should have 2 pending invitations");

    repo.accept(created1.id).await.unwrap();

    let after_accept = repo.find_by_tenant(tenant_id).await.unwrap();
    assert_eq!(
        after_accept.len(),
        1,
        "Accepted invitation should not appear"
    );
    assert_eq!(after_accept[0].email, "pending2@example.com");
}

#[tokio::test]
async fn test_find_by_email() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id, _) = setup_tenant_with_user(&pool).await;
    let repo = SqliteInvitationRepository::new(pool.clone());

    let invitation = NewInvitation::new(
        tenant_id,
        "byemail@example.com".to_string(),
        "member".to_string(),
        "token-byemail".to_string(),
        Utc::now() + Duration::days(7),
        user_id,
    );

    let created = repo.create(invitation).await.unwrap();
    let found = repo
        .find_by_email(tenant_id, "byemail@example.com")
        .await
        .unwrap();

    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(found.id, created.id);
}

#[tokio::test]
async fn test_accept_invitation() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id, _) = setup_tenant_with_user(&pool).await;
    let repo = SqliteInvitationRepository::new(pool.clone());

    let invitation = NewInvitation::new(
        tenant_id,
        "acceptme@example.com".to_string(),
        "member".to_string(),
        "token-accept".to_string(),
        Utc::now() + Duration::days(7),
        user_id,
    );

    let created = repo.create(invitation).await.unwrap();
    assert!(created.accepted_at.is_none());

    repo.accept(created.id).await.unwrap();

    let accepted = repo.find_by_id(created.id).await.unwrap().unwrap();
    assert!(
        accepted.accepted_at.is_some(),
        "accepted_at should be set after accept"
    );
}

#[tokio::test]
async fn test_delete_invitation() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id, _) = setup_tenant_with_user(&pool).await;
    let repo = SqliteInvitationRepository::new(pool.clone());

    let invitation = NewInvitation::new(
        tenant_id,
        "deleteme@example.com".to_string(),
        "member".to_string(),
        "token-delete".to_string(),
        Utc::now() + Duration::days(7),
        user_id,
    );

    let created = repo.create(invitation).await.unwrap();

    repo.delete(created.id).await.unwrap();

    let found = repo.find_by_id(created.id).await.unwrap();
    assert!(found.is_none(), "Invitation should be deleted");
}
