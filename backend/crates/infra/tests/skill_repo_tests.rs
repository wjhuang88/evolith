//! Integration tests for SqliteSkillRepository

// Workspace `[lints.clippy] unwrap_used = "deny"` overrides `clippy.toml`
// `allow-unwrap-in-tests`; tests need unwrap for concise assertion failures.
#![allow(clippy::unwrap_used)]

mod test_helpers;

use domain::repository::{SkillRepository, TenantRepository, UserRepository};
use domain::skill::{Dependency, NewSkill, Runtime, SkillFilter, UpdateSkill, Visibility};
use domain::tenant::CreateTenantRequest;
use domain::user::{NewUser, TenantRole};
use infra::db::{SqliteSkillRepository, SqliteTenantRepository, SqliteUserRepository};
use test_helpers::setup_test_db;
use uuid::Uuid;

async fn create_test_tenant_and_user(pool: &sqlx::SqlitePool) -> (Uuid, Uuid) {
    let tenant_repo = SqliteTenantRepository::new(pool.clone());
    let user_repo = SqliteUserRepository::new(pool.clone());
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

    let user = user_repo
        .create(
            NewUser {
                username: "skillowner".to_string(),
                email: "skillowner@test.com".to_string(),
                password: "password123".to_string(),
            },
            tenant.id,
            TenantRole::Admin,
        )
        .await
        .unwrap();

    (tenant.id, user.id)
}

fn create_test_skill_md() -> String {
    r#"---
name: test-skill
description: A test skill
---

# Test Skill

This is a test skill markdown content.
"#
    .to_string()
}

fn create_test_dependencies() -> Vec<Dependency> {
    vec![
        Dependency {
            name: "requests".to_string(),
            version: "2.28.0".to_string(),
        },
        Dependency {
            name: "numpy".to_string(),
            version: "1.24.0".to_string(),
        },
    ]
}

#[tokio::test]
async fn test_create_skill() {
    let pool = setup_test_db().await;
    let (tenant_id, owner_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteSkillRepository::new(pool.clone());

    let new_skill = NewSkill {
        name: "test-skill".to_string(),
        version: "1.0.0".to_string(),
        description: "A test skill".to_string(),
        skill_md: create_test_skill_md(),
        code_package_path: Some("/skills/test-skill.tar.gz".to_string()),
        runtime: Runtime::Python311,
        dependencies: create_test_dependencies(),
        visibility: Some(Visibility::Public),
        author: None,
        tags: vec![],
        skill_type: "instruction".to_string(),
        execution: "client".to_string(),
        entrypoint: None,
        timeout: 30,
        memory_mb: 256,
        permissions: None,
        license: None,
        compatibility: None,
        disable_model_invocation: false,
        user_invocable: true,
        argument_hint: None,
    };

    let skill = repo.create(new_skill, owner_id, tenant_id).await.unwrap();

    assert!(!skill.id.is_nil());
    assert_eq!(skill.name, "test-skill");
    assert_eq!(skill.version, "1.0.0");
    assert_eq!(skill.description, "A test skill");
    assert_eq!(skill.owner_id, owner_id);
    assert_eq!(skill.tenant_id, tenant_id);
    assert_eq!(skill.visibility, Visibility::Public);
    assert_eq!(skill.runtime, Runtime::Python311);
    assert_eq!(skill.dependencies.len(), 2);
}

#[tokio::test]
async fn test_create_skill_with_node_runtime() {
    let pool = setup_test_db().await;
    let (tenant_id, owner_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteSkillRepository::new(pool.clone());

    let new_skill = NewSkill {
        name: "node-skill".to_string(),
        version: "2.0.0".to_string(),
        description: "A Node.js skill".to_string(),
        skill_md: create_test_skill_md(),
        code_package_path: None,
        runtime: Runtime::Node20,
        dependencies: vec![],
        visibility: Some(Visibility::Public),
        author: None,
        tags: vec![],
        skill_type: "instruction".to_string(),
        execution: "client".to_string(),
        entrypoint: None,
        timeout: 30,
        memory_mb: 256,
        permissions: None,
        license: None,
        compatibility: None,
        disable_model_invocation: false,
        user_invocable: true,
        argument_hint: None,
    };

    let skill = repo.create(new_skill, owner_id, tenant_id).await.unwrap();
    assert_eq!(skill.runtime, Runtime::Node20);
}

#[tokio::test]
async fn test_create_skill_with_wasm_runtime() {
    let pool = setup_test_db().await;
    let (tenant_id, owner_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteSkillRepository::new(pool.clone());

    let new_skill = NewSkill {
        name: "wasm-skill".to_string(),
        version: "1.0.0".to_string(),
        description: "A WASM skill".to_string(),
        skill_md: create_test_skill_md(),
        code_package_path: Some("/skills/wasm-skill.wasm".to_string()),
        runtime: Runtime::Wasm,
        dependencies: vec![],
        visibility: Some(Visibility::Public),
        author: None,
        tags: vec![],
        skill_type: "instruction".to_string(),
        execution: "client".to_string(),
        entrypoint: None,
        timeout: 30,
        memory_mb: 256,
        permissions: None,
        license: None,
        compatibility: None,
        disable_model_invocation: false,
        user_invocable: true,
        argument_hint: None,
    };

    let skill = repo.create(new_skill, owner_id, tenant_id).await.unwrap();
    assert_eq!(skill.runtime, Runtime::Wasm);
}

#[tokio::test]
async fn test_create_skill_default_visibility() {
    let pool = setup_test_db().await;
    let (tenant_id, owner_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteSkillRepository::new(pool.clone());

    let new_skill = NewSkill {
        name: "default-visibility-skill".to_string(),
        version: "1.0.0".to_string(),
        description: "Skill with default visibility".to_string(),
        skill_md: create_test_skill_md(),
        code_package_path: None,
        runtime: Runtime::Python311,
        dependencies: vec![],
        visibility: None,
        author: None,
        tags: vec![],
        skill_type: "instruction".to_string(),
        execution: "client".to_string(),
        entrypoint: None,
        timeout: 30,
        memory_mb: 256,
        permissions: None,
        license: None,
        compatibility: None,
        disable_model_invocation: false,
        user_invocable: true,
        argument_hint: None,
    };

    let skill = repo.create(new_skill, owner_id, tenant_id).await.unwrap();
    assert_eq!(skill.visibility, Visibility::Private);
}

#[tokio::test]
async fn test_find_by_id() {
    let pool = setup_test_db().await;
    let (tenant_id, owner_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteSkillRepository::new(pool.clone());

    let new_skill = NewSkill {
        name: "find-by-id-skill".to_string(),
        version: "1.0.0".to_string(),
        description: "Skill to find by ID".to_string(),
        skill_md: create_test_skill_md(),
        code_package_path: None,
        runtime: Runtime::Python311,
        dependencies: vec![],
        visibility: Some(Visibility::Public),
        author: None,
        tags: vec![],
        skill_type: "instruction".to_string(),
        execution: "client".to_string(),
        entrypoint: None,
        timeout: 30,
        memory_mb: 256,
        permissions: None,
        license: None,
        compatibility: None,
        disable_model_invocation: false,
        user_invocable: true,
        argument_hint: None,
    };

    let created = repo.create(new_skill, owner_id, tenant_id).await.unwrap();
    let found = repo.find_by_id(created.id).await.unwrap();

    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(found.id, created.id);
    assert_eq!(found.name, "find-by-id-skill");
}

#[tokio::test]
async fn test_find_by_id_not_found() {
    let pool = setup_test_db().await;
    let repo = SqliteSkillRepository::new(pool.clone());

    let result = repo.find_by_id(Uuid::new_v4()).await.unwrap();
    assert!(result.is_none());
}

#[tokio::test]
async fn test_find_by_name_and_version() {
    let pool = setup_test_db().await;
    let (tenant_id, owner_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteSkillRepository::new(pool.clone());

    let new_skill = NewSkill {
        name: "versioned-skill".to_string(),
        version: "2.1.0".to_string(),
        description: "A versioned skill".to_string(),
        skill_md: create_test_skill_md(),
        code_package_path: None,
        runtime: Runtime::Python311,
        dependencies: vec![],
        visibility: Some(Visibility::Public),
        author: None,
        tags: vec![],
        skill_type: "instruction".to_string(),
        execution: "client".to_string(),
        entrypoint: None,
        timeout: 30,
        memory_mb: 256,
        permissions: None,
        license: None,
        compatibility: None,
        disable_model_invocation: false,
        user_invocable: true,
        argument_hint: None,
    };

    let created = repo.create(new_skill, owner_id, tenant_id).await.unwrap();
    let found = repo
        .find_by_name_and_version("versioned-skill", "2.1.0")
        .await
        .unwrap();

    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(found.id, created.id);
    assert_eq!(found.name, "versioned-skill");
    assert_eq!(found.version, "2.1.0");
}

#[tokio::test]
async fn test_find_by_name_and_version_different_version() {
    let pool = setup_test_db().await;
    let (tenant_id, owner_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteSkillRepository::new(pool.clone());

    let new_skill = NewSkill {
        name: "multi-version-skill".to_string(),
        version: "1.0.0".to_string(),
        description: "Multi version skill".to_string(),
        skill_md: create_test_skill_md(),
        code_package_path: None,
        runtime: Runtime::Python311,
        dependencies: vec![],
        visibility: Some(Visibility::Public),
        author: None,
        tags: vec![],
        skill_type: "instruction".to_string(),
        execution: "client".to_string(),
        entrypoint: None,
        timeout: 30,
        memory_mb: 256,
        permissions: None,
        license: None,
        compatibility: None,
        disable_model_invocation: false,
        user_invocable: true,
        argument_hint: None,
    };

    repo.create(new_skill, owner_id, tenant_id).await.unwrap();

    let found = repo
        .find_by_name_and_version("multi-version-skill", "2.0.0")
        .await
        .unwrap();
    assert!(found.is_none());
}

#[tokio::test]
async fn test_find_all_no_filter() {
    let pool = setup_test_db().await;
    let (tenant_id, owner_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteSkillRepository::new(pool.clone());

    for i in 0..3 {
        let new_skill = NewSkill {
            name: format!("skill-{}", i),
            version: "1.0.0".to_string(),
            description: format!("Skill {}", i),
            skill_md: create_test_skill_md(),
            code_package_path: None,
            runtime: Runtime::Python311,
            dependencies: vec![],
            visibility: Some(Visibility::Public),
        author: None,
        tags: vec![],
        skill_type: "instruction".to_string(),
        execution: "client".to_string(),
        entrypoint: None,
        timeout: 30,
        memory_mb: 256,
        permissions: None,
        license: None,
        compatibility: None,
        disable_model_invocation: false,
        user_invocable: true,
        argument_hint: None,
        };
        repo.create(new_skill, owner_id, tenant_id).await.unwrap();
    }

    let filter = SkillFilter::default();
    let skills = repo.find_all(filter).await.unwrap();

    assert_eq!(skills.len(), 3);
}

#[tokio::test]
async fn test_find_all_with_tenant_filter() {
    let pool = setup_test_db().await;
    let (tenant_id, owner_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteSkillRepository::new(pool.clone());

    for i in 0..2 {
        let new_skill = NewSkill {
            name: format!("tenant-skill-{}", i),
            version: "1.0.0".to_string(),
            description: format!("Tenant Skill {}", i),
            skill_md: create_test_skill_md(),
            code_package_path: None,
            runtime: Runtime::Python311,
            dependencies: vec![],
            visibility: Some(Visibility::Public),
        author: None,
        tags: vec![],
        skill_type: "instruction".to_string(),
        execution: "client".to_string(),
        entrypoint: None,
        timeout: 30,
        memory_mb: 256,
        permissions: None,
        license: None,
        compatibility: None,
        disable_model_invocation: false,
        user_invocable: true,
        argument_hint: None,
        };
        repo.create(new_skill, owner_id, tenant_id).await.unwrap();
    }

    let filter = SkillFilter {
        tenant_id: Some(tenant_id),
        ..SkillFilter::default()
    };
    let skills = repo.find_all(filter).await.unwrap();

    assert_eq!(skills.len(), 2);
}

#[tokio::test]
async fn test_find_all_with_visibility_filter() {
    let pool = setup_test_db().await;
    let (tenant_id, owner_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteSkillRepository::new(pool.clone());

    for i in 0..2 {
        let new_skill = NewSkill {
            name: format!("public-skill-{}", i),
            version: "1.0.0".to_string(),
            description: format!("Public Skill {}", i),
            skill_md: create_test_skill_md(),
            code_package_path: None,
            runtime: Runtime::Python311,
            dependencies: vec![],
            visibility: Some(Visibility::Public),
        author: None,
        tags: vec![],
        skill_type: "instruction".to_string(),
        execution: "client".to_string(),
        entrypoint: None,
        timeout: 30,
        memory_mb: 256,
        permissions: None,
        license: None,
        compatibility: None,
        disable_model_invocation: false,
        user_invocable: true,
        argument_hint: None,
        };
        repo.create(new_skill, owner_id, tenant_id).await.unwrap();
    }

    let private_skill = NewSkill {
        name: "private-skill-filter".to_string(),
        version: "1.0.0".to_string(),
        description: "Private skill".to_string(),
        skill_md: create_test_skill_md(),
        code_package_path: None,
        runtime: Runtime::Python311,
        dependencies: vec![],
        visibility: Some(Visibility::Private),
        author: None,
        tags: vec![],
        skill_type: "instruction".to_string(),
        execution: "client".to_string(),
        entrypoint: None,
        timeout: 30,
        memory_mb: 256,
        permissions: None,
        license: None,
        compatibility: None,
        disable_model_invocation: false,
        user_invocable: true,
        argument_hint: None,
    };
    repo.create(private_skill, owner_id, tenant_id)
        .await
        .unwrap();

    let filter = SkillFilter {
        visibility: Some(Visibility::Public),
        ..SkillFilter::default()
    };
    let skills = repo.find_all(filter).await.unwrap();

    assert_eq!(skills.len(), 2);
    for skill in &skills {
        assert_eq!(skill.visibility, Visibility::Public);
    }
}

#[tokio::test]
async fn test_find_all_with_runtime_filter() {
    let pool = setup_test_db().await;
    let (tenant_id, owner_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteSkillRepository::new(pool.clone());

    let python_skill = NewSkill {
        name: "python-runtime-skill".to_string(),
        version: "1.0.0".to_string(),
        description: "Python skill".to_string(),
        skill_md: create_test_skill_md(),
        code_package_path: None,
        runtime: Runtime::Python311,
        dependencies: vec![],
        visibility: Some(Visibility::Public),
        author: None,
        tags: vec![],
        skill_type: "instruction".to_string(),
        execution: "client".to_string(),
        entrypoint: None,
        timeout: 30,
        memory_mb: 256,
        permissions: None,
        license: None,
        compatibility: None,
        disable_model_invocation: false,
        user_invocable: true,
        argument_hint: None,
    };
    repo.create(python_skill, owner_id, tenant_id)
        .await
        .unwrap();

    let node_skill = NewSkill {
        name: "node-runtime-skill".to_string(),
        version: "1.0.0".to_string(),
        description: "Node skill".to_string(),
        skill_md: create_test_skill_md(),
        code_package_path: None,
        runtime: Runtime::Node20,
        dependencies: vec![],
        visibility: Some(Visibility::Public),
        author: None,
        tags: vec![],
        skill_type: "instruction".to_string(),
        execution: "client".to_string(),
        entrypoint: None,
        timeout: 30,
        memory_mb: 256,
        permissions: None,
        license: None,
        compatibility: None,
        disable_model_invocation: false,
        user_invocable: true,
        argument_hint: None,
    };
    repo.create(node_skill, owner_id, tenant_id).await.unwrap();

    let filter = SkillFilter {
        runtime: Some(Runtime::Python311),
        ..SkillFilter::default()
    };
    let skills = repo.find_all(filter).await.unwrap();

    assert_eq!(skills.len(), 1);
    assert_eq!(skills[0].runtime, Runtime::Python311);
}

#[tokio::test]
async fn test_find_all_with_search_filter() {
    let pool = setup_test_db().await;
    let (tenant_id, owner_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteSkillRepository::new(pool.clone());

    let skill1 = NewSkill {
        name: "weather-forecast-skill".to_string(),
        version: "1.0.0".to_string(),
        description: "Get weather forecasts".to_string(),
        skill_md: create_test_skill_md(),
        code_package_path: None,
        runtime: Runtime::Python311,
        dependencies: vec![],
        visibility: Some(Visibility::Public),
        author: None,
        tags: vec![],
        skill_type: "instruction".to_string(),
        execution: "client".to_string(),
        entrypoint: None,
        timeout: 30,
        memory_mb: 256,
        permissions: None,
        license: None,
        compatibility: None,
        disable_model_invocation: false,
        user_invocable: true,
        argument_hint: None,
    };
    repo.create(skill1, owner_id, tenant_id).await.unwrap();

    let skill2 = NewSkill {
        name: "translation-skill".to_string(),
        version: "1.0.0".to_string(),
        description: "Translate text between languages".to_string(),
        skill_md: create_test_skill_md(),
        code_package_path: None,
        runtime: Runtime::Python311,
        dependencies: vec![],
        visibility: Some(Visibility::Public),
        author: None,
        tags: vec![],
        skill_type: "instruction".to_string(),
        execution: "client".to_string(),
        entrypoint: None,
        timeout: 30,
        memory_mb: 256,
        permissions: None,
        license: None,
        compatibility: None,
        disable_model_invocation: false,
        user_invocable: true,
        argument_hint: None,
    };
    repo.create(skill2, owner_id, tenant_id).await.unwrap();

    let filter = SkillFilter {
        search: Some("weather".to_string()),
        ..SkillFilter::default()
    };
    let skills = repo.find_all(filter).await.unwrap();

    assert_eq!(skills.len(), 1);
    assert_eq!(skills[0].name, "weather-forecast-skill");
}

#[tokio::test]
async fn test_find_all_pagination() {
    let pool = setup_test_db().await;
    let (tenant_id, owner_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteSkillRepository::new(pool.clone());

    for i in 0..5 {
        let new_skill = NewSkill {
            name: format!("page-skill-{}", i),
            version: "1.0.0".to_string(),
            description: format!("Page Skill {}", i),
            skill_md: create_test_skill_md(),
            code_package_path: None,
            runtime: Runtime::Python311,
            dependencies: vec![],
            visibility: Some(Visibility::Public),
        author: None,
        tags: vec![],
        skill_type: "instruction".to_string(),
        execution: "client".to_string(),
        entrypoint: None,
        timeout: 30,
        memory_mb: 256,
        permissions: None,
        license: None,
        compatibility: None,
        disable_model_invocation: false,
        user_invocable: true,
        argument_hint: None,
        };
        repo.create(new_skill, owner_id, tenant_id).await.unwrap();
    }

    let filter = SkillFilter {
        page: Some(1),
        per_page: Some(2),
        ..SkillFilter::default()
    };
    let skills = repo.find_all(filter).await.unwrap();
    assert_eq!(skills.len(), 2);

    let filter = SkillFilter {
        page: Some(2),
        per_page: Some(2),
        ..SkillFilter::default()
    };
    let skills = repo.find_all(filter).await.unwrap();
    assert_eq!(skills.len(), 2);

    let filter = SkillFilter {
        page: Some(3),
        per_page: Some(2),
        ..SkillFilter::default()
    };
    let skills = repo.find_all(filter).await.unwrap();
    assert_eq!(skills.len(), 1);
}

#[tokio::test]
async fn test_count() {
    let pool = setup_test_db().await;
    let (tenant_id, owner_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteSkillRepository::new(pool.clone());

    for i in 0..4 {
        let new_skill = NewSkill {
            name: format!("count-skill-{}", i),
            version: "1.0.0".to_string(),
            description: format!("Count Skill {}", i),
            skill_md: create_test_skill_md(),
            code_package_path: None,
            runtime: Runtime::Python311,
            dependencies: vec![],
            visibility: Some(Visibility::Public),
        author: None,
        tags: vec![],
        skill_type: "instruction".to_string(),
        execution: "client".to_string(),
        entrypoint: None,
        timeout: 30,
        memory_mb: 256,
        permissions: None,
        license: None,
        compatibility: None,
        disable_model_invocation: false,
        user_invocable: true,
        argument_hint: None,
        };
        repo.create(new_skill, owner_id, tenant_id).await.unwrap();
    }

    let filter = SkillFilter::default();
    let count = repo.count(&filter).await.unwrap();
    assert_eq!(count, 4);
}

#[tokio::test]
async fn test_count_with_filter() {
    let pool = setup_test_db().await;
    let (tenant_id, owner_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteSkillRepository::new(pool.clone());

    for i in 0..2 {
        let new_skill = NewSkill {
            name: format!("public-count-{}", i),
            version: "1.0.0".to_string(),
            description: format!("Public Count {}", i),
            skill_md: create_test_skill_md(),
            code_package_path: None,
            runtime: Runtime::Python311,
            dependencies: vec![],
            visibility: Some(Visibility::Public),
        author: None,
        tags: vec![],
        skill_type: "instruction".to_string(),
        execution: "client".to_string(),
        entrypoint: None,
        timeout: 30,
        memory_mb: 256,
        permissions: None,
        license: None,
        compatibility: None,
        disable_model_invocation: false,
        user_invocable: true,
        argument_hint: None,
        };
        repo.create(new_skill, owner_id, tenant_id).await.unwrap();
    }

    let private_skill = NewSkill {
        name: "private-count".to_string(),
        version: "1.0.0".to_string(),
        description: "Private count skill".to_string(),
        skill_md: create_test_skill_md(),
        code_package_path: None,
        runtime: Runtime::Python311,
        dependencies: vec![],
        visibility: Some(Visibility::Private),
        author: None,
        tags: vec![],
        skill_type: "instruction".to_string(),
        execution: "client".to_string(),
        entrypoint: None,
        timeout: 30,
        memory_mb: 256,
        permissions: None,
        license: None,
        compatibility: None,
        disable_model_invocation: false,
        user_invocable: true,
        argument_hint: None,
    };
    repo.create(private_skill, owner_id, tenant_id)
        .await
        .unwrap();

    let filter = SkillFilter {
        visibility: Some(Visibility::Public),
        ..SkillFilter::default()
    };
    let count = repo.count(&filter).await.unwrap();
    assert_eq!(count, 2);
}

#[tokio::test]
async fn test_count_with_runtime_filter() {
    let pool = setup_test_db().await;
    let (tenant_id, owner_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteSkillRepository::new(pool.clone());

    let python_skill = NewSkill {
        name: "python-count".to_string(),
        version: "1.0.0".to_string(),
        description: "Python count skill".to_string(),
        skill_md: create_test_skill_md(),
        code_package_path: None,
        runtime: Runtime::Python311,
        dependencies: vec![],
        visibility: Some(Visibility::Public),
        author: None,
        tags: vec![],
        skill_type: "instruction".to_string(),
        execution: "client".to_string(),
        entrypoint: None,
        timeout: 30,
        memory_mb: 256,
        permissions: None,
        license: None,
        compatibility: None,
        disable_model_invocation: false,
        user_invocable: true,
        argument_hint: None,
    };
    repo.create(python_skill, owner_id, tenant_id)
        .await
        .unwrap();

    let node_skill = NewSkill {
        name: "node-count".to_string(),
        version: "1.0.0".to_string(),
        description: "Node count skill".to_string(),
        skill_md: create_test_skill_md(),
        code_package_path: None,
        runtime: Runtime::Node20,
        dependencies: vec![],
        visibility: Some(Visibility::Public),
        author: None,
        tags: vec![],
        skill_type: "instruction".to_string(),
        execution: "client".to_string(),
        entrypoint: None,
        timeout: 30,
        memory_mb: 256,
        permissions: None,
        license: None,
        compatibility: None,
        disable_model_invocation: false,
        user_invocable: true,
        argument_hint: None,
    };
    repo.create(node_skill, owner_id, tenant_id).await.unwrap();

    let filter = SkillFilter {
        runtime: Some(Runtime::Python311),
        ..SkillFilter::default()
    };
    let count = repo.count(&filter).await.unwrap();
    assert_eq!(count, 1);
}

#[tokio::test]
async fn test_delete_skill() {
    let pool = setup_test_db().await;
    let (tenant_id, owner_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteSkillRepository::new(pool.clone());

    let new_skill = NewSkill {
        name: "delete-skill".to_string(),
        version: "1.0.0".to_string(),
        description: "Skill to delete".to_string(),
        skill_md: create_test_skill_md(),
        code_package_path: None,
        runtime: Runtime::Python311,
        dependencies: vec![],
        visibility: Some(Visibility::Public),
        author: None,
        tags: vec![],
        skill_type: "instruction".to_string(),
        execution: "client".to_string(),
        entrypoint: None,
        timeout: 30,
        memory_mb: 256,
        permissions: None,
        license: None,
        compatibility: None,
        disable_model_invocation: false,
        user_invocable: true,
        argument_hint: None,
    };

    let created = repo.create(new_skill, owner_id, tenant_id).await.unwrap();
    repo.delete(created.id).await.unwrap();

    let found = repo.find_by_id(created.id).await.unwrap();
    assert!(found.is_none());
}

#[tokio::test]
async fn test_delete_nonexistent_skill() {
    let pool = setup_test_db().await;
    let repo = SqliteSkillRepository::new(pool.clone());

    let result = repo.delete(Uuid::new_v4()).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_skill_with_dependencies() {
    let pool = setup_test_db().await;
    let (tenant_id, owner_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteSkillRepository::new(pool.clone());

    let dependencies = vec![
        Dependency {
            name: "requests".to_string(),
            version: "2.28.0".to_string(),
        },
        Dependency {
            name: "beautifulsoup4".to_string(),
            version: "4.12.0".to_string(),
        },
        Dependency {
            name: "lxml".to_string(),
            version: "4.9.0".to_string(),
        },
    ];

    let new_skill = NewSkill {
        name: "web-scraper-skill".to_string(),
        version: "1.0.0".to_string(),
        description: "Web scraping skill".to_string(),
        skill_md: create_test_skill_md(),
        code_package_path: Some("/skills/web-scraper.tar.gz".to_string()),
        runtime: Runtime::Python311,
        dependencies: dependencies.clone(),
        visibility: Some(Visibility::Public),
        author: None,
        tags: vec![],
        skill_type: "instruction".to_string(),
        execution: "client".to_string(),
        entrypoint: None,
        timeout: 30,
        memory_mb: 256,
        permissions: None,
        license: None,
        compatibility: None,
        disable_model_invocation: false,
        user_invocable: true,
        argument_hint: None,
    };

    let created = repo.create(new_skill, owner_id, tenant_id).await.unwrap();
    assert_eq!(created.dependencies.len(), 3);
    assert_eq!(created.dependencies[0].name, "requests");
    assert_eq!(created.dependencies[1].name, "beautifulsoup4");
    assert_eq!(created.dependencies[2].name, "lxml");
}

#[tokio::test]
async fn test_create_skill_with_agent_skills_fields() {
    let pool = setup_test_db().await;
    let (tenant_id, owner_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteSkillRepository::new(pool.clone());

    let new_skill = NewSkill {
        name: "agent-skill".to_string(),
        version: "1.0.0".to_string(),
        description: "A skill with agent fields".to_string(),
        skill_md: create_test_skill_md(),
        code_package_path: None,
        runtime: Runtime::Python311,
        dependencies: vec![],
        visibility: Some(Visibility::Public),
        author: Some("test-author".to_string()),
        tags: vec!["ai".to_string(), "agent".to_string()],
        skill_type: "agent".to_string(),
        execution: "server".to_string(),
        entrypoint: Some("main.py".to_string()),
        timeout: 60,
        memory_mb: 512,
        permissions: Some(serde_json::json!({"read": true, "write": false})),
        license: Some("MIT".to_string()),
        compatibility: "python>=3.9".to_string().into(),
        disable_model_invocation: true,
        user_invocable: false,
        argument_hint: Some("--config path/to/config".to_string()),
    };

    let skill = repo.create(new_skill, owner_id, tenant_id).await.unwrap();

    assert_eq!(skill.author, Some("test-author".to_string()));
    assert_eq!(skill.tags, vec!["ai", "agent"]);
    assert_eq!(skill.skill_type, "agent");
    assert_eq!(skill.execution, "server");
    assert_eq!(skill.entrypoint, Some("main.py".to_string()));
    assert_eq!(skill.timeout, 60);
    assert_eq!(skill.memory_mb, 512);
    assert_eq!(skill.license, Some("MIT".to_string()));
    assert_eq!(skill.compatibility, Some("python>=3.9".to_string()));
    assert!(skill.disable_model_invocation);
    assert!(!skill.user_invocable);
    assert_eq!(skill.argument_hint, Some("--config path/to/config".to_string()));

    let found = repo.find_by_id(skill.id).await.unwrap().unwrap();
    assert_eq!(found.author, Some("test-author".to_string()));
    assert_eq!(found.tags, vec!["ai", "agent"]);
    assert_eq!(found.skill_type, "agent");
    assert_eq!(found.execution, "server");
    assert_eq!(found.timeout, 60);
    assert_eq!(found.memory_mb, 512);
    assert!(found.disable_model_invocation);
    assert!(!found.user_invocable);
}

#[tokio::test]
async fn test_update_skill_agent_skills_fields() {
    let pool = setup_test_db().await;
    let (tenant_id, owner_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteSkillRepository::new(pool.clone());

    let new_skill = NewSkill {
        name: "update-test-skill".to_string(),
        version: "1.0.0".to_string(),
        description: "Skill to update".to_string(),
        skill_md: create_test_skill_md(),
        code_package_path: None,
        runtime: Runtime::Python311,
        dependencies: vec![],
        visibility: Some(Visibility::Private),
        author: None,
        tags: vec![],
        skill_type: "instruction".to_string(),
        execution: "client".to_string(),
        entrypoint: None,
        timeout: 30,
        memory_mb: 256,
        permissions: None,
        license: None,
        compatibility: None,
        disable_model_invocation: false,
        user_invocable: true,
        argument_hint: None,
    };

    let created = repo.create(new_skill, owner_id, tenant_id).await.unwrap();

    let update = UpdateSkill {
        name: None,
        version: None,
        description: None,
        skill_md: None,
        runtime: None,
        dependencies: None,
        visibility: None,
        author: Some("updated-author".to_string()),
        tags: Some(vec!["updated".to_string()]),
        skill_type: Some("agent".to_string()),
        execution: Some("server".to_string()),
        entrypoint: Some("run.py".to_string()),
        timeout: Some(120),
        memory_mb: Some(1024),
        permissions: Some(serde_json::json!({"admin": true})),
        license: Some("Apache-2.0".to_string()),
        compatibility: Some("python>=3.10".to_string()),
        disable_model_invocation: Some(true),
        user_invocable: Some(false),
        argument_hint: Some("--verbose".to_string()),
    };

    let updated = repo.update(created.id, update).await.unwrap();

    assert_eq!(updated.author, Some("updated-author".to_string()));
    assert_eq!(updated.tags, vec!["updated"]);
    assert_eq!(updated.skill_type, "agent");
    assert_eq!(updated.execution, "server");
    assert_eq!(updated.entrypoint, Some("run.py".to_string()));
    assert_eq!(updated.timeout, 120);
    assert_eq!(updated.memory_mb, 1024);
    assert_eq!(updated.license, Some("Apache-2.0".to_string()));
    assert!(updated.disable_model_invocation);
    assert!(!updated.user_invocable);
    assert_eq!(updated.argument_hint, Some("--verbose".to_string()));
}
