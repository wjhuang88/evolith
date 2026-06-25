#![allow(clippy::unwrap_used)]

mod test_helpers;

use domain::git_repo::{NewGitRepo, RepoVisibility, UpdateGitRepo};
use domain::repository::{GitRepoRepository, TenantRepository};
use domain::tenant::CreateTenantRequest;
use infra::db::{SqliteGitRepoRepository, SqliteTenantRepository};
use test_helpers::setup_test_db;
use uuid::Uuid;

async fn setup_tenant(pool: &sqlx::SqlitePool) -> domain::tenant::Tenant {
    let tenant_repo = SqliteTenantRepository::new(pool.clone());
    let owner_id = Uuid::new_v4();
    let request = CreateTenantRequest {
        name: "Test Company".to_string(),
        slug: format!("test-{}", Uuid::new_v4().simple()),
        owner_email: "owner@test.com".to_string(),
        owner_username: "owner".to_string(),
        owner_password: "password123".to_string(),
    };
    tenant_repo.create(request, owner_id).await.unwrap()
}

fn make_new_repo(name: &str) -> NewGitRepo {
    NewGitRepo {
        name: name.to_string(),
        description: Some("Test repository".to_string()),
        default_branch: None,
        visibility: None,
        auto_merge: None,
        require_review: None,
    }
}

#[tokio::test]
async fn test_create_git_repo() {
    let pool = setup_test_db().await;
    let tenant = setup_tenant(&pool).await;
    let repo = SqliteGitRepoRepository::new(pool);

    let created = repo
        .create(make_new_repo("test-repo"), tenant.id)
        .await
        .unwrap();

    assert_eq!(created.name, "test-repo");
    assert_eq!(created.description, "Test repository");
    assert_eq!(created.default_branch, "main");
    assert_eq!(created.visibility, RepoVisibility::Private);
    assert!(!created.auto_merge);
    assert!(created.require_review);
    assert!(created.storage_path.contains(&tenant.id.to_string()));
    assert!(created.storage_path.contains("test-repo"));
}

#[tokio::test]
async fn test_find_by_id() {
    let pool = setup_test_db().await;
    let tenant = setup_tenant(&pool).await;
    let repo = SqliteGitRepoRepository::new(pool);

    let created = repo
        .create(make_new_repo("test-repo"), tenant.id)
        .await
        .unwrap();
    let found = repo.find_by_id(created.id).await.unwrap().unwrap();

    assert_eq!(found.id, created.id);
    assert_eq!(found.name, "test-repo");
}

#[tokio::test]
async fn test_find_by_id_not_found() {
    let pool = setup_test_db().await;
    let repo = SqliteGitRepoRepository::new(pool);
    let result = repo.find_by_id(Uuid::new_v4()).await.unwrap();
    assert!(result.is_none());
}

#[tokio::test]
async fn test_find_by_name() {
    let pool = setup_test_db().await;
    let tenant = setup_tenant(&pool).await;
    let repo = SqliteGitRepoRepository::new(pool);

    let created = repo
        .create(make_new_repo("test-repo"), tenant.id)
        .await
        .unwrap();
    let found = repo
        .find_by_name(tenant.id, "test-repo")
        .await
        .unwrap()
        .unwrap();

    assert_eq!(found.id, created.id);
}

#[tokio::test]
async fn test_find_by_name_not_found() {
    let pool = setup_test_db().await;
    let tenant = setup_tenant(&pool).await;
    let repo = SqliteGitRepoRepository::new(pool);

    let result = repo.find_by_name(tenant.id, "nonexistent").await.unwrap();
    assert!(result.is_none());
}

#[tokio::test]
async fn test_find_by_tenant() {
    let pool = setup_test_db().await;
    let tenant = setup_tenant(&pool).await;
    let repo = SqliteGitRepoRepository::new(pool);

    repo.create(make_new_repo("repo-a"), tenant.id)
        .await
        .unwrap();
    repo.create(make_new_repo("repo-b"), tenant.id)
        .await
        .unwrap();

    let repos = repo.find_by_tenant(tenant.id).await.unwrap();
    assert_eq!(repos.len(), 2);
}

#[tokio::test]
async fn test_find_by_tenant_empty() {
    let pool = setup_test_db().await;
    let repo = SqliteGitRepoRepository::new(pool);
    let repos = repo.find_by_tenant(Uuid::new_v4()).await.unwrap();
    assert!(repos.is_empty());
}

#[tokio::test]
async fn test_unique_tenant_name() {
    let pool = setup_test_db().await;
    let tenant = setup_tenant(&pool).await;
    let repo = SqliteGitRepoRepository::new(pool);

    repo.create(make_new_repo("test-repo"), tenant.id)
        .await
        .unwrap();

    let result = repo.create(make_new_repo("test-repo"), tenant.id).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_same_name_different_tenant() {
    let pool = setup_test_db().await;
    let tenant1 = setup_tenant(&pool).await;
    let tenant2 = setup_tenant(&pool).await;
    let repo = SqliteGitRepoRepository::new(pool);

    let r1 = repo
        .create(make_new_repo("test-repo"), tenant1.id)
        .await
        .unwrap();
    let r2 = repo
        .create(make_new_repo("test-repo"), tenant2.id)
        .await
        .unwrap();

    assert_eq!(r1.name, r2.name);
    assert_ne!(r1.tenant_id, r2.tenant_id);
}

#[tokio::test]
async fn test_update_git_repo() {
    let pool = setup_test_db().await;
    let tenant = setup_tenant(&pool).await;
    let repo = SqliteGitRepoRepository::new(pool);

    let created = repo
        .create(make_new_repo("test-repo"), tenant.id)
        .await
        .unwrap();

    let update = UpdateGitRepo {
        name: Some("renamed-repo".to_string()),
        description: Some("Updated description".to_string()),
        default_branch: Some("develop".to_string()),
        visibility: Some(RepoVisibility::Public),
        auto_merge: Some(false),
        require_review: Some(true),
    };
    let updated = repo.update(created.id, update).await.unwrap();

    assert_eq!(updated.name, "renamed-repo");
    assert_eq!(updated.description, "Updated description");
    assert_eq!(updated.default_branch, "develop");
    assert_eq!(updated.visibility, RepoVisibility::Public);
    assert!(!updated.auto_merge);
    assert!(updated.require_review);
}

#[tokio::test]
async fn test_update_partial() {
    let pool = setup_test_db().await;
    let tenant = setup_tenant(&pool).await;
    let repo = SqliteGitRepoRepository::new(pool);

    let created = repo
        .create(make_new_repo("test-repo"), tenant.id)
        .await
        .unwrap();

    let update = UpdateGitRepo {
        name: None,
        description: Some("Only description changed".to_string()),
        default_branch: None,
        visibility: None,
        auto_merge: None,
        require_review: None,
    };
    let updated = repo.update(created.id, update).await.unwrap();

    assert_eq!(updated.name, "test-repo");
    assert_eq!(updated.description, "Only description changed");
    assert_eq!(updated.default_branch, "main");
}

#[tokio::test]
async fn test_delete_git_repo() {
    let pool = setup_test_db().await;
    let tenant = setup_tenant(&pool).await;
    let repo = SqliteGitRepoRepository::new(pool);

    let created = repo
        .create(make_new_repo("test-repo"), tenant.id)
        .await
        .unwrap();
    repo.delete(created.id).await.unwrap();

    let found = repo.find_by_id(created.id).await.unwrap();
    assert!(found.is_none());
}

#[tokio::test]
async fn test_update_last_commit() {
    let pool = setup_test_db().await;
    let tenant = setup_tenant(&pool).await;
    let repo = SqliteGitRepoRepository::new(pool);

    let created = repo
        .create(make_new_repo("test-repo"), tenant.id)
        .await
        .unwrap();
    let now = chrono::Utc::now();

    repo.update_last_commit(created.id, "abc123def456", now)
        .await
        .unwrap();

    let found = repo.find_by_id(created.id).await.unwrap().unwrap();
    assert_eq!(found.last_commit_sha.unwrap(), "abc123def456");
    assert!(found.last_committed_at.is_some());
}

#[tokio::test]
async fn test_create_public_repo() {
    let pool = setup_test_db().await;
    let tenant = setup_tenant(&pool).await;
    let repo = SqliteGitRepoRepository::new(pool);

    let new_repo = NewGitRepo {
        name: "public-repo".to_string(),
        description: None,
        default_branch: None,
        visibility: Some(RepoVisibility::Public),
        auto_merge: None,
        require_review: None,
    };
    let created = repo.create(new_repo, tenant.id).await.unwrap();
    assert_eq!(created.visibility, RepoVisibility::Public);
}

#[tokio::test]
async fn test_create_with_policy() {
    let pool = setup_test_db().await;
    let tenant = setup_tenant(&pool).await;
    let repo = SqliteGitRepoRepository::new(pool);

    let new_repo = NewGitRepo {
        name: "policy-repo".to_string(),
        description: None,
        default_branch: Some("release".to_string()),
        visibility: None,
        auto_merge: Some(false),
        require_review: Some(true),
    };
    let created = repo.create(new_repo, tenant.id).await.unwrap();
    assert!(!created.auto_merge);
    assert!(created.require_review);
    assert_eq!(created.default_branch, "release");
}
