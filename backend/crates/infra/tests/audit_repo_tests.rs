//! Integration tests for SqliteAuditRepository

// Workspace `[lints.clippy] unwrap_used = "deny"` overrides `clippy.toml`
// `allow-unwrap-in-tests`; tests need unwrap for concise assertion failures.
#![allow(clippy::unwrap_used)]

mod test_helpers;

use chrono::Utc;
use domain::audit::AuditLog;
use domain::repository::{AuditRepository, TenantRepository, UserRepository};
use domain::tenant::CreateTenantRequest;
use domain::user::{NewUser, TenantRole};
use infra::db::{SqliteAuditRepository, SqliteTenantRepository, SqliteUserRepository};
use test_helpers::setup_test_db;
use uuid::Uuid;

async fn create_test_tenant_and_user(pool: &sqlx::SqlitePool) -> (Uuid, Uuid) {
    let tenant_repo = SqliteTenantRepository::new(pool.clone());
    let user_repo = SqliteUserRepository::new(pool.clone());

    let owner_id = Uuid::new_v4();
    let tenant = tenant_repo
        .create(
            CreateTenantRequest {
                name: "Audit Test Tenant".to_string(),
                slug: "audit-test".to_string(),
                owner_email: "owner@audittest.com".to_string(),
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
                username: "audituser".to_string(),
                email: "audituser@audittest.com".to_string(),
                password: "password123".to_string(),
            },
            tenant.id,
            TenantRole::Admin,
        )
        .await
        .unwrap();

    (tenant.id, user.id)
}

fn create_test_audit_log(tenant_id: Option<Uuid>, user_id: Option<Uuid>, action: &str) -> AuditLog {
    AuditLog {
        id: Uuid::new_v4(),
        tenant_id,
        user_id,
        action: action.to_string(),
        resource_type: Some("test_resource".to_string()),
        resource_id: Some(Uuid::new_v4().to_string()),
        details: serde_json::json!({"key": "value"}),
        ip_address: Some("127.0.0.1".to_string()),
        user_agent: Some("TestAgent/1.0".to_string()),
        created_at: Utc::now(),
    }
}

#[tokio::test]
async fn test_create_audit_log() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteAuditRepository::new(pool.clone());

    let log = create_test_audit_log(Some(tenant_id), Some(user_id), "user.login");

    repo.create(log.clone()).await.unwrap();

    let found = repo.find_by_tenant(tenant_id, 10, 0).await.unwrap();
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].action, "user.login");
    assert_eq!(found[0].tenant_id, Some(tenant_id));
    assert_eq!(found[0].user_id, Some(user_id));
}

#[tokio::test]
async fn test_create_audit_log_without_tenant() {
    let pool = setup_test_db().await;
    let repo = SqliteAuditRepository::new(pool.clone());

    let log = create_test_audit_log(None, None, "system.startup");

    repo.create(log.clone()).await.unwrap();

    let found = repo.find_by_action("system.startup", 10, 0).await.unwrap();
    assert_eq!(found.len(), 1);
    assert!(found[0].tenant_id.is_none());
    assert!(found[0].user_id.is_none());
}

#[tokio::test]
async fn test_create_audit_log_with_complex_details() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteAuditRepository::new(pool.clone());

    let log = AuditLog {
        id: Uuid::new_v4(),
        tenant_id: Some(tenant_id),
        user_id: Some(user_id),
        action: "skill.execute".to_string(),
        resource_type: Some("skill".to_string()),
        resource_id: Some("skill-uuid-123".to_string()),
        details: serde_json::json!({
            "skill_name": "file_reader",
            "arguments": {
                "path": "/tmp/test.txt",
                "max_size": 1024
            },
            "result": {
                "success": true,
                "lines_read": 42
            }
        }),
        ip_address: Some("192.168.1.100".to_string()),
        user_agent: Some("Evolith-CLI/1.0".to_string()),
        created_at: Utc::now(),
    };

    repo.create(log.clone()).await.unwrap();

    let found = repo.find_by_tenant(tenant_id, 10, 0).await.unwrap();
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].action, "skill.execute");
    let details = found[0].details.as_object().unwrap();
    assert_eq!(details["skill_name"], "file_reader");
}

#[tokio::test]
async fn test_find_by_tenant() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteAuditRepository::new(pool.clone());

    let log1 = create_test_audit_log(Some(tenant_id), Some(user_id), "action.one");
    let log2 = create_test_audit_log(Some(tenant_id), Some(user_id), "action.two");
    let log3 = create_test_audit_log(None, None, "action.three");

    repo.create(log1).await.unwrap();
    repo.create(log2).await.unwrap();
    repo.create(log3).await.unwrap();

    let found = repo.find_by_tenant(tenant_id, 10, 0).await.unwrap();
    assert_eq!(found.len(), 2);
}

#[tokio::test]
async fn test_find_by_tenant_with_pagination() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteAuditRepository::new(pool.clone());

    for i in 0..5 {
        let log = create_test_audit_log(Some(tenant_id), Some(user_id), &format!("action.{}", i));
        repo.create(log).await.unwrap();
    }

    let page1 = repo.find_by_tenant(tenant_id, 2, 0).await.unwrap();
    assert_eq!(page1.len(), 2);

    let page2 = repo.find_by_tenant(tenant_id, 2, 2).await.unwrap();
    assert_eq!(page2.len(), 2);

    let page3 = repo.find_by_tenant(tenant_id, 2, 4).await.unwrap();
    assert_eq!(page3.len(), 1);
}

#[tokio::test]
async fn test_find_by_tenant_empty() {
    let pool = setup_test_db().await;
    let repo = SqliteAuditRepository::new(pool.clone());

    let found = repo.find_by_tenant(Uuid::new_v4(), 10, 0).await.unwrap();
    assert!(found.is_empty());
}

#[tokio::test]
async fn test_find_by_user() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteAuditRepository::new(pool.clone());

    let log1 = create_test_audit_log(Some(tenant_id), Some(user_id), "user.action.1");
    let log2 = create_test_audit_log(Some(tenant_id), Some(user_id), "user.action.2");
    let log3 = create_test_audit_log(Some(tenant_id), None, "user.action.3");

    repo.create(log1).await.unwrap();
    repo.create(log2).await.unwrap();
    repo.create(log3).await.unwrap();

    let found = repo.find_by_user(user_id, 10, 0).await.unwrap();
    assert_eq!(found.len(), 2);
}

#[tokio::test]
async fn test_find_by_user_with_pagination() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteAuditRepository::new(pool.clone());

    for i in 0..4 {
        let log = create_test_audit_log(
            Some(tenant_id),
            Some(user_id),
            &format!("user.paginated.{}", i),
        );
        repo.create(log).await.unwrap();
    }

    let page1 = repo.find_by_user(user_id, 2, 0).await.unwrap();
    assert_eq!(page1.len(), 2);

    let page2 = repo.find_by_user(user_id, 2, 2).await.unwrap();
    assert_eq!(page2.len(), 2);
}

#[tokio::test]
async fn test_find_by_user_empty() {
    let pool = setup_test_db().await;
    let repo = SqliteAuditRepository::new(pool.clone());

    let found = repo.find_by_user(Uuid::new_v4(), 10, 0).await.unwrap();
    assert!(found.is_empty());
}

#[tokio::test]
async fn test_find_by_action() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteAuditRepository::new(pool.clone());

    let log1 = create_test_audit_log(Some(tenant_id), Some(user_id), "tool.create");
    let log2 = create_test_audit_log(Some(tenant_id), Some(user_id), "tool.create");
    let log3 = create_test_audit_log(Some(tenant_id), Some(user_id), "tool.delete");

    repo.create(log1).await.unwrap();
    repo.create(log2).await.unwrap();
    repo.create(log3).await.unwrap();

    let found = repo.find_by_action("tool.create", 10, 0).await.unwrap();
    assert_eq!(found.len(), 2);

    let found_delete = repo.find_by_action("tool.delete", 10, 0).await.unwrap();
    assert_eq!(found_delete.len(), 1);
}

#[tokio::test]
async fn test_find_by_action_with_pagination() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteAuditRepository::new(pool.clone());

    for _ in 0..3 {
        let log = create_test_audit_log(Some(tenant_id), Some(user_id), "paginated.action");
        repo.create(log).await.unwrap();
    }

    let page1 = repo.find_by_action("paginated.action", 2, 0).await.unwrap();
    assert_eq!(page1.len(), 2);

    let page2 = repo.find_by_action("paginated.action", 2, 2).await.unwrap();
    assert_eq!(page2.len(), 1);
}

#[tokio::test]
async fn test_find_by_action_empty() {
    let pool = setup_test_db().await;
    let repo = SqliteAuditRepository::new(pool.clone());

    let found = repo
        .find_by_action("nonexistent.action", 10, 0)
        .await
        .unwrap();
    assert!(found.is_empty());
}
