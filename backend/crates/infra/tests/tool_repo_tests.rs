//! Integration tests for SqliteToolRepository

mod test_helpers;

use domain::repository::{TenantRepository, ToolRepository, UserRepository};
use domain::tenant::CreateTenantRequest;
use domain::tool::{HandlerConfig, HandlerType, NewTool, ToolFilter, UpdateTool, Visibility};
use domain::user::{NewUser, TenantRole};
use infra::db::{SqliteTenantRepository, SqliteToolRepository, SqliteUserRepository};
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
                username: "toolowner".to_string(),
                email: "toolowner@test.com".to_string(),
                password: "password123".to_string(),
            },
            tenant.id,
            TenantRole::Admin,
        )
        .await
        .unwrap();

    (tenant.id, user.id)
}

fn create_test_handler() -> HandlerConfig {
    HandlerConfig {
        handler_type: HandlerType::Http,
        url: Some("https://api.example.com/tools/test".to_string()),
        method: Some("POST".to_string()),
        timeout: Some(30000),
    }
}

fn create_test_input_schema() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "properties": {
            "query": { "type": "string" }
        },
        "required": ["query"]
    })
}

#[tokio::test]
async fn test_create_tool() {
    let pool = setup_test_db().await;
    let (tenant_id, owner_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteToolRepository::new(pool.clone());

    let new_tool = NewTool {
        name: "test-tool".to_string(),
        description: "A test tool".to_string(),
        input_schema: create_test_input_schema(),
        output_schema: None,
        handler: create_test_handler(),
        visibility: Some(Visibility::Public),
    };

    let tool = repo.create(new_tool, owner_id, tenant_id).await.unwrap();

    assert!(!tool.id.is_nil());
    assert_eq!(tool.name, "test-tool");
    assert_eq!(tool.description, "A test tool");
    assert_eq!(tool.owner_id, owner_id);
    assert_eq!(tool.tenant_id, tenant_id);
    assert_eq!(tool.visibility, Visibility::Public);
}

#[tokio::test]
async fn test_create_tool_with_private_visibility() {
    let pool = setup_test_db().await;
    let (tenant_id, owner_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteToolRepository::new(pool.clone());

    let new_tool = NewTool {
        name: "private-tool".to_string(),
        description: "A private tool".to_string(),
        input_schema: create_test_input_schema(),
        output_schema: None,
        handler: create_test_handler(),
        visibility: Some(Visibility::Private),
    };

    let tool = repo.create(new_tool, owner_id, tenant_id).await.unwrap();
    assert_eq!(tool.visibility, Visibility::Private);
}

#[tokio::test]
async fn test_create_tool_default_visibility() {
    let pool = setup_test_db().await;
    let (tenant_id, owner_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteToolRepository::new(pool.clone());

    let new_tool = NewTool {
        name: "default-visibility-tool".to_string(),
        description: "Tool with default visibility".to_string(),
        input_schema: create_test_input_schema(),
        output_schema: None,
        handler: create_test_handler(),
        visibility: None,
    };

    let tool = repo.create(new_tool, owner_id, tenant_id).await.unwrap();
    assert_eq!(tool.visibility, Visibility::Private);
}

#[tokio::test]
async fn test_find_by_id() {
    let pool = setup_test_db().await;
    let (tenant_id, owner_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteToolRepository::new(pool.clone());

    let new_tool = NewTool {
        name: "find-by-id-tool".to_string(),
        description: "Tool to find by ID".to_string(),
        input_schema: create_test_input_schema(),
        output_schema: None,
        handler: create_test_handler(),
        visibility: Some(Visibility::Public),
    };

    let created = repo.create(new_tool, owner_id, tenant_id).await.unwrap();
    let found = repo.find_by_id(created.id).await.unwrap();

    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(found.id, created.id);
    assert_eq!(found.name, "find-by-id-tool");
}

#[tokio::test]
async fn test_find_by_id_not_found() {
    let pool = setup_test_db().await;
    let repo = SqliteToolRepository::new(pool.clone());

    let result = repo.find_by_id(Uuid::new_v4()).await.unwrap();
    assert!(result.is_none());
}

#[tokio::test]
async fn test_find_by_name() {
    let pool = setup_test_db().await;
    let (tenant_id, owner_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteToolRepository::new(pool.clone());

    let new_tool = NewTool {
        name: "unique-tool-name".to_string(),
        description: "Tool with unique name".to_string(),
        input_schema: create_test_input_schema(),
        output_schema: None,
        handler: create_test_handler(),
        visibility: Some(Visibility::Public),
    };

    let created = repo.create(new_tool, owner_id, tenant_id).await.unwrap();
    let found = repo.find_by_name("unique-tool-name").await.unwrap();

    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(found.id, created.id);
    assert_eq!(found.name, "unique-tool-name");
}

#[tokio::test]
async fn test_find_by_name_not_found() {
    let pool = setup_test_db().await;
    let repo = SqliteToolRepository::new(pool.clone());

    let result = repo.find_by_name("nonexistent-tool").await.unwrap();
    assert!(result.is_none());
}

#[tokio::test]
async fn test_find_all_no_filter() {
    let pool = setup_test_db().await;
    let (tenant_id, owner_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteToolRepository::new(pool.clone());

    for i in 0..3 {
        let new_tool = NewTool {
            name: format!("tool-{}", i),
            description: format!("Tool {}", i),
            input_schema: create_test_input_schema(),
            output_schema: None,
            handler: create_test_handler(),
            visibility: Some(Visibility::Public),
        };
        repo.create(new_tool, owner_id, tenant_id).await.unwrap();
    }

    let filter = ToolFilter::default();
    let tools = repo.find_all(filter).await.unwrap();

    assert_eq!(tools.len(), 3);
}

#[tokio::test]
async fn test_find_all_with_tenant_filter() {
    let pool = setup_test_db().await;
    let (tenant_id, owner_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteToolRepository::new(pool.clone());

    for i in 0..2 {
        let new_tool = NewTool {
            name: format!("tenant-tool-{}", i),
            description: format!("Tenant Tool {}", i),
            input_schema: create_test_input_schema(),
            output_schema: None,
            handler: create_test_handler(),
            visibility: Some(Visibility::Public),
        };
        repo.create(new_tool, owner_id, tenant_id).await.unwrap();
    }

    let filter = ToolFilter {
        tenant_id: Some(tenant_id),
        ..ToolFilter::default()
    };
    let tools = repo.find_all(filter).await.unwrap();

    assert_eq!(tools.len(), 2);
}

#[tokio::test]
async fn test_find_all_with_visibility_filter() {
    let pool = setup_test_db().await;
    let (tenant_id, owner_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteToolRepository::new(pool.clone());

    for i in 0..2 {
        let new_tool = NewTool {
            name: format!("public-tool-{}", i),
            description: format!("Public Tool {}", i),
            input_schema: create_test_input_schema(),
            output_schema: None,
            handler: create_test_handler(),
            visibility: Some(Visibility::Public),
        };
        repo.create(new_tool, owner_id, tenant_id).await.unwrap();
    }

    let private_tool = NewTool {
        name: "private-tool-filter".to_string(),
        description: "Private tool".to_string(),
        input_schema: create_test_input_schema(),
        output_schema: None,
        handler: create_test_handler(),
        visibility: Some(Visibility::Private),
    };
    repo.create(private_tool, owner_id, tenant_id)
        .await
        .unwrap();

    let filter = ToolFilter {
        visibility: Some(Visibility::Public),
        ..ToolFilter::default()
    };
    let tools = repo.find_all(filter).await.unwrap();

    assert_eq!(tools.len(), 2);
    for tool in &tools {
        assert_eq!(tool.visibility, Visibility::Public);
    }
}

#[tokio::test]
async fn test_find_all_with_search_filter() {
    let pool = setup_test_db().await;
    let (tenant_id, owner_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteToolRepository::new(pool.clone());

    let tool1 = NewTool {
        name: "search-weather-tool".to_string(),
        description: "Get weather data".to_string(),
        input_schema: create_test_input_schema(),
        output_schema: None,
        handler: create_test_handler(),
        visibility: Some(Visibility::Public),
    };
    repo.create(tool1, owner_id, tenant_id).await.unwrap();

    let tool2 = NewTool {
        name: "another-tool".to_string(),
        description: "Different search functionality".to_string(),
        input_schema: create_test_input_schema(),
        output_schema: None,
        handler: create_test_handler(),
        visibility: Some(Visibility::Public),
    };
    repo.create(tool2, owner_id, tenant_id).await.unwrap();

    let filter = ToolFilter {
        search: Some("weather".to_string()),
        ..ToolFilter::default()
    };
    let tools = repo.find_all(filter).await.unwrap();

    assert_eq!(tools.len(), 1);
    assert_eq!(tools[0].name, "search-weather-tool");
}

#[tokio::test]
async fn test_find_all_pagination() {
    let pool = setup_test_db().await;
    let (tenant_id, owner_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteToolRepository::new(pool.clone());

    for i in 0..5 {
        let new_tool = NewTool {
            name: format!("page-tool-{}", i),
            description: format!("Page Tool {}", i),
            input_schema: create_test_input_schema(),
            output_schema: None,
            handler: create_test_handler(),
            visibility: Some(Visibility::Public),
        };
        repo.create(new_tool, owner_id, tenant_id).await.unwrap();
    }

    let filter = ToolFilter {
        page: Some(1),
        per_page: Some(2),
        ..ToolFilter::default()
    };
    let tools = repo.find_all(filter).await.unwrap();
    assert_eq!(tools.len(), 2);

    let filter = ToolFilter {
        page: Some(2),
        per_page: Some(2),
        ..ToolFilter::default()
    };
    let tools = repo.find_all(filter).await.unwrap();
    assert_eq!(tools.len(), 2);

    let filter = ToolFilter {
        page: Some(3),
        per_page: Some(2),
        ..ToolFilter::default()
    };
    let tools = repo.find_all(filter).await.unwrap();
    assert_eq!(tools.len(), 1);
}

#[tokio::test]
async fn test_count() {
    let pool = setup_test_db().await;
    let (tenant_id, owner_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteToolRepository::new(pool.clone());

    for i in 0..4 {
        let new_tool = NewTool {
            name: format!("count-tool-{}", i),
            description: format!("Count Tool {}", i),
            input_schema: create_test_input_schema(),
            output_schema: None,
            handler: create_test_handler(),
            visibility: Some(Visibility::Public),
        };
        repo.create(new_tool, owner_id, tenant_id).await.unwrap();
    }

    let filter = ToolFilter::default();
    let count = repo.count(&filter).await.unwrap();
    assert_eq!(count, 4);
}

#[tokio::test]
async fn test_count_with_filter() {
    let pool = setup_test_db().await;
    let (tenant_id, owner_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteToolRepository::new(pool.clone());

    for i in 0..2 {
        let new_tool = NewTool {
            name: format!("public-count-{}", i),
            description: format!("Public Count {}", i),
            input_schema: create_test_input_schema(),
            output_schema: None,
            handler: create_test_handler(),
            visibility: Some(Visibility::Public),
        };
        repo.create(new_tool, owner_id, tenant_id).await.unwrap();
    }

    let private_tool = NewTool {
        name: "private-count".to_string(),
        description: "Private count tool".to_string(),
        input_schema: create_test_input_schema(),
        output_schema: None,
        handler: create_test_handler(),
        visibility: Some(Visibility::Private),
    };
    repo.create(private_tool, owner_id, tenant_id)
        .await
        .unwrap();

    let filter = ToolFilter {
        visibility: Some(Visibility::Public),
        ..ToolFilter::default()
    };
    let count = repo.count(&filter).await.unwrap();
    assert_eq!(count, 2);
}

#[tokio::test]
async fn test_update_name() {
    let pool = setup_test_db().await;
    let (tenant_id, owner_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteToolRepository::new(pool.clone());

    let new_tool = NewTool {
        name: "original-name".to_string(),
        description: "Original description".to_string(),
        input_schema: create_test_input_schema(),
        output_schema: None,
        handler: create_test_handler(),
        visibility: Some(Visibility::Public),
    };

    let created = repo.create(new_tool, owner_id, tenant_id).await.unwrap();

    let update = UpdateTool {
        name: Some("updated-name".to_string()),
        description: None,
        input_schema: None,
        output_schema: None,
        handler: None,
        visibility: None,
    };

    let updated = repo.update(created.id, update).await.unwrap();
    assert_eq!(updated.name, "updated-name");
    assert_eq!(updated.description, "Original description");
}

#[tokio::test]
async fn test_update_description() {
    let pool = setup_test_db().await;
    let (tenant_id, owner_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteToolRepository::new(pool.clone());

    let new_tool = NewTool {
        name: "update-desc-tool".to_string(),
        description: "Original description".to_string(),
        input_schema: create_test_input_schema(),
        output_schema: None,
        handler: create_test_handler(),
        visibility: Some(Visibility::Public),
    };

    let created = repo.create(new_tool, owner_id, tenant_id).await.unwrap();

    let update = UpdateTool {
        name: None,
        description: Some("Updated description".to_string()),
        input_schema: None,
        output_schema: None,
        handler: None,
        visibility: None,
    };

    let updated = repo.update(created.id, update).await.unwrap();
    assert_eq!(updated.description, "Updated description");
    assert_eq!(updated.name, "update-desc-tool");
}

#[tokio::test]
async fn test_update_visibility() {
    let pool = setup_test_db().await;
    let (tenant_id, owner_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteToolRepository::new(pool.clone());

    let new_tool = NewTool {
        name: "visibility-update-tool".to_string(),
        description: "Tool for visibility update".to_string(),
        input_schema: create_test_input_schema(),
        output_schema: None,
        handler: create_test_handler(),
        visibility: Some(Visibility::Private),
    };

    let created = repo.create(new_tool, owner_id, tenant_id).await.unwrap();
    assert_eq!(created.visibility, Visibility::Private);

    let update = UpdateTool {
        name: None,
        description: None,
        input_schema: None,
        output_schema: None,
        handler: None,
        visibility: Some(Visibility::Public),
    };

    let updated = repo.update(created.id, update).await.unwrap();
    assert_eq!(updated.visibility, Visibility::Public);
}

#[tokio::test]
async fn test_update_handler() {
    let pool = setup_test_db().await;
    let (tenant_id, owner_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteToolRepository::new(pool.clone());

    let new_tool = NewTool {
        name: "handler-update-tool".to_string(),
        description: "Tool for handler update".to_string(),
        input_schema: create_test_input_schema(),
        output_schema: None,
        handler: create_test_handler(),
        visibility: Some(Visibility::Public),
    };

    let created = repo.create(new_tool, owner_id, tenant_id).await.unwrap();

    let new_handler = HandlerConfig {
        handler_type: HandlerType::Function,
        url: Some("https://api.newexample.com/function".to_string()),
        method: Some("GET".to_string()),
        timeout: Some(60000),
    };

    let update = UpdateTool {
        name: None,
        description: None,
        input_schema: None,
        output_schema: None,
        handler: Some(new_handler.clone()),
        visibility: None,
    };

    let updated = repo.update(created.id, update).await.unwrap();
    assert_eq!(updated.handler.handler_type, HandlerType::Function);
    assert_eq!(
        updated.handler.url,
        Some("https://api.newexample.com/function".to_string())
    );
    assert_eq!(updated.handler.timeout, Some(60000));
}

#[tokio::test]
async fn test_update_input_schema() {
    let pool = setup_test_db().await;
    let (tenant_id, owner_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteToolRepository::new(pool.clone());

    let new_tool = NewTool {
        name: "schema-update-tool".to_string(),
        description: "Tool for schema update".to_string(),
        input_schema: create_test_input_schema(),
        output_schema: None,
        handler: create_test_handler(),
        visibility: Some(Visibility::Public),
    };

    let created = repo.create(new_tool, owner_id, tenant_id).await.unwrap();

    let new_schema = serde_json::json!({
        "type": "object",
        "properties": {
            "input": { "type": "string" },
            "count": { "type": "number" }
        },
        "required": ["input"]
    });

    let update = UpdateTool {
        name: None,
        description: None,
        input_schema: Some(new_schema.clone()),
        output_schema: None,
        handler: None,
        visibility: None,
    };

    let updated = repo.update(created.id, update).await.unwrap();
    assert!(updated
        .input_schema
        .get("properties")
        .unwrap()
        .get("count")
        .is_some());
}

#[tokio::test]
async fn test_update_output_schema() {
    let pool = setup_test_db().await;
    let (tenant_id, owner_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteToolRepository::new(pool.clone());

    let new_tool = NewTool {
        name: "output-schema-tool".to_string(),
        description: "Tool for output schema".to_string(),
        input_schema: create_test_input_schema(),
        output_schema: None,
        handler: create_test_handler(),
        visibility: Some(Visibility::Public),
    };

    let created = repo.create(new_tool, owner_id, tenant_id).await.unwrap();
    assert!(created.output_schema.is_none());

    let output_schema = serde_json::json!({
        "type": "object",
        "properties": {
            "result": { "type": "string" }
        }
    });

    let update = UpdateTool {
        name: None,
        description: None,
        input_schema: None,
        output_schema: Some(output_schema.clone()),
        handler: None,
        visibility: None,
    };

    let updated = repo.update(created.id, update).await.unwrap();
    assert!(updated.output_schema.is_some());
    let output = updated.output_schema.unwrap();
    assert!(output.get("properties").unwrap().get("result").is_some());
}

#[tokio::test]
async fn test_delete_tool() {
    let pool = setup_test_db().await;
    let (tenant_id, owner_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteToolRepository::new(pool.clone());

    let new_tool = NewTool {
        name: "delete-tool".to_string(),
        description: "Tool to delete".to_string(),
        input_schema: create_test_input_schema(),
        output_schema: None,
        handler: create_test_handler(),
        visibility: Some(Visibility::Public),
    };

    let created = repo.create(new_tool, owner_id, tenant_id).await.unwrap();
    repo.delete(created.id).await.unwrap();

    let found = repo.find_by_id(created.id).await.unwrap();
    assert!(found.is_none());
}

#[tokio::test]
async fn test_delete_nonexistent_tool() {
    let pool = setup_test_db().await;
    let repo = SqliteToolRepository::new(pool.clone());

    let result = repo.delete(Uuid::new_v4()).await;
    assert!(result.is_ok());
}
