//! Integration tests for SqliteSnippetRepository

// Workspace `[lints.clippy] unwrap_used = "deny"` overrides `clippy.toml`
// `allow-unwrap-in-tests`; tests need unwrap for concise assertion failures.
#![allow(clippy::unwrap_used)]

mod test_helpers;

use domain::repository::{SnippetRepository, TenantRepository, UserRepository};
use domain::snippet::{Dependency, NewSnippet, SnippetFilter, UpdateSnippet, Visibility};
use domain::tenant::CreateTenantRequest;
use domain::user::{NewUser, TenantRole};
use infra::db::{SqliteSnippetRepository, SqliteTenantRepository, SqliteUserRepository};
use test_helpers::setup_test_db;
use uuid::Uuid;

async fn create_test_tenant_and_user(pool: &sqlx::SqlitePool) -> (Uuid, Uuid) {
    let tenant_repo = SqliteTenantRepository::new(pool.clone());
    let user_repo = SqliteUserRepository::new(pool.clone());
    let owner_id = Uuid::new_v4();
    let tenant = tenant_repo
        .create(
            CreateTenantRequest {
                name: "Snippet Test Tenant".to_string(),
                slug: "snippet-test".to_string(),
                owner_email: "owner@snippettest.com".to_string(),
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
                username: "snippetowner".to_string(),
                email: "snippetowner@test.com".to_string(),
                password: "password123".to_string(),
            },
            tenant.id,
            TenantRole::Admin,
        )
        .await
        .unwrap();

    (tenant.id, user.id)
}

fn create_test_snippet(name: &str, language: &str, visibility: Option<Visibility>) -> NewSnippet {
    NewSnippet {
        name: name.to_string(),
        language: language.to_string(),
        framework: None,
        tags: vec!["test".to_string()],
        content: format!("Documentation for {}", name),
        code: format!("fn {}() {{ }}", name.replace('-', "_")),
        dependencies: vec![],
        visibility,
        version: "1.0.0".to_string(),
        summary: None,
        command: None,
        subcommands: vec![],
        inputs: vec![],
        output: None,
        examples: vec![],
        error_model: None,
    }
}

// ── Create ──────────────────────────────────────────────────────

#[tokio::test]
async fn test_create_snippet() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteSnippetRepository::new(pool.clone());

    let new_snippet = NewSnippet {
        name: "test-snippet".to_string(),
        language: "rust".to_string(),
        framework: Some("actix-web".to_string()),
        tags: vec!["web".to_string(), "api".to_string()],
        content: "Documentation content".to_string(),
        code: "fn hello() { println!(\"hello\"); }".to_string(),
        dependencies: vec![Dependency {
            name: "serde".to_string(),
            version: "1.0".to_string(),
            required: true,
        }],
        visibility: Some(Visibility::Public),
        version: "1.0.0".to_string(),
        summary: None,
        command: None,
        subcommands: vec![],
        inputs: vec![],
        output: None,
        examples: vec![],
        error_model: None,
    };

    let snippet = repo.create(new_snippet, user_id, tenant_id).await.unwrap();

    assert!(!snippet.id.is_nil());
    assert_eq!(snippet.name, "test-snippet");
    assert_eq!(snippet.language, "rust");
    assert_eq!(snippet.framework, Some("actix-web".to_string()));
    assert_eq!(snippet.tags, vec!["web", "api"]);
    assert_eq!(snippet.owner_id, user_id);
    assert_eq!(snippet.tenant_id, tenant_id);
    assert_eq!(snippet.visibility, Visibility::Public);
    assert_eq!(snippet.dependencies.len(), 1);
    assert!(snippet.estimated_tokens > 0);
}

#[tokio::test]
async fn test_create_snippet_default_visibility() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteSnippetRepository::new(pool.clone());

    let new_snippet = create_test_snippet("default-vis", "python", None);
    let snippet = repo.create(new_snippet, user_id, tenant_id).await.unwrap();

    assert_eq!(snippet.visibility, Visibility::Private);
}

#[tokio::test]
async fn test_create_snippet_with_no_framework() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteSnippetRepository::new(pool.clone());

    let new_snippet = create_test_snippet("no-framework", "javascript", Some(Visibility::Public));
    let snippet = repo.create(new_snippet, user_id, tenant_id).await.unwrap();

    assert!(snippet.framework.is_none());
}

// ── Find by ID ──────────────────────────────────────────────────

#[tokio::test]
async fn test_find_by_id() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteSnippetRepository::new(pool.clone());

    let new_snippet = create_test_snippet("find-me", "rust", Some(Visibility::Public));
    let created = repo.create(new_snippet, user_id, tenant_id).await.unwrap();

    let found = repo.find_by_id(created.id).await.unwrap();
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(found.id, created.id);
    assert_eq!(found.name, "find-me");
    assert_eq!(found.language, "rust");
}

#[tokio::test]
async fn test_find_by_id_not_found() {
    let pool = setup_test_db().await;
    let repo = SqliteSnippetRepository::new(pool.clone());

    let result = repo.find_by_id(Uuid::new_v4()).await.unwrap();
    assert!(result.is_none());
}

// ── Find all (filters) ─────────────────────────────────────────

#[tokio::test]
async fn test_find_all_no_filter() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteSnippetRepository::new(pool.clone());

    for i in 0..3 {
        let s = create_test_snippet(&format!("snippet-{}", i), "rust", Some(Visibility::Public));
        repo.create(s, user_id, tenant_id).await.unwrap();
    }

    let filter = SnippetFilter::default();
    let snippets = repo.find_all(filter).await.unwrap();
    assert_eq!(snippets.len(), 3);
}

#[tokio::test]
async fn test_find_all_with_tenant_filter() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteSnippetRepository::new(pool.clone());

    for i in 0..2 {
        let s = create_test_snippet(
            &format!("tenant-snippet-{}", i),
            "python",
            Some(Visibility::Public),
        );
        repo.create(s, user_id, tenant_id).await.unwrap();
    }

    let filter = SnippetFilter {
        tenant_id: Some(tenant_id),
        ..SnippetFilter::default()
    };
    let snippets = repo.find_all(filter).await.unwrap();
    assert_eq!(snippets.len(), 2);
}

#[tokio::test]
async fn test_find_all_with_visibility_filter() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteSnippetRepository::new(pool.clone());

    let public = create_test_snippet("public-snip", "rust", Some(Visibility::Public));
    repo.create(public, user_id, tenant_id).await.unwrap();

    let private = create_test_snippet("private-snip", "rust", Some(Visibility::Private));
    repo.create(private, user_id, tenant_id).await.unwrap();

    let filter = SnippetFilter {
        visibility: Some(Visibility::Public),
        ..SnippetFilter::default()
    };
    let snippets = repo.find_all(filter).await.unwrap();
    assert_eq!(snippets.len(), 1);
    assert_eq!(snippets[0].visibility, Visibility::Public);
}

#[tokio::test]
async fn test_find_all_with_language_filter() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteSnippetRepository::new(pool.clone());

    let rust_snip = create_test_snippet("rust-snippet", "rust", Some(Visibility::Public));
    repo.create(rust_snip, user_id, tenant_id).await.unwrap();

    let py_snip = create_test_snippet("python-snippet", "python", Some(Visibility::Public));
    repo.create(py_snip, user_id, tenant_id).await.unwrap();

    let filter = SnippetFilter {
        language: Some("rust".to_string()),
        ..SnippetFilter::default()
    };
    let snippets = repo.find_all(filter).await.unwrap();
    assert_eq!(snippets.len(), 1);
    assert_eq!(snippets[0].language, "rust");
}

#[tokio::test]
async fn test_find_all_with_search_filter() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteSnippetRepository::new(pool.clone());

    let s1 = NewSnippet {
        name: "auth-middleware".to_string(),
        language: "rust".to_string(),
        framework: Some("actix-web".to_string()),
        tags: vec!["auth".to_string()],
        content: "JWT authentication middleware".to_string(),
        code: "fn auth_middleware() {}".to_string(),
        dependencies: vec![],
        visibility: Some(Visibility::Public),
        version: "1.0.0".to_string(),
        summary: None,
        command: None,
        subcommands: vec![],
        inputs: vec![],
        output: None,
        examples: vec![],
        error_model: None,
    };
    repo.create(s1, user_id, tenant_id).await.unwrap();

    let s2 = create_test_snippet("database-helper", "rust", Some(Visibility::Public));
    repo.create(s2, user_id, tenant_id).await.unwrap();

    let filter = SnippetFilter {
        search: Some("auth".to_string()),
        ..SnippetFilter::default()
    };
    let snippets = repo.find_all(filter).await.unwrap();
    assert_eq!(snippets.len(), 1);
    assert_eq!(snippets[0].name, "auth-middleware");
}

#[tokio::test]
async fn test_find_all_pagination() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteSnippetRepository::new(pool.clone());

    for i in 0..5 {
        let s = create_test_snippet(
            &format!("page-snip-{}", i),
            "rust",
            Some(Visibility::Public),
        );
        repo.create(s, user_id, tenant_id).await.unwrap();
    }

    let filter = SnippetFilter {
        page: Some(1),
        per_page: Some(2),
        ..SnippetFilter::default()
    };
    let page1 = repo.find_all(filter).await.unwrap();
    assert_eq!(page1.len(), 2);

    let filter = SnippetFilter {
        page: Some(3),
        per_page: Some(2),
        ..SnippetFilter::default()
    };
    let page3 = repo.find_all(filter).await.unwrap();
    assert_eq!(page3.len(), 1);
}

// ── Count ───────────────────────────────────────────────────────

#[tokio::test]
async fn test_count() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteSnippetRepository::new(pool.clone());

    for i in 0..4 {
        let s = create_test_snippet(
            &format!("count-snip-{}", i),
            "rust",
            Some(Visibility::Public),
        );
        repo.create(s, user_id, tenant_id).await.unwrap();
    }

    let filter = SnippetFilter::default();
    let count = repo.count(&filter).await.unwrap();
    assert_eq!(count, 4);
}

#[tokio::test]
async fn test_count_with_language_filter() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteSnippetRepository::new(pool.clone());

    let rust_snip = create_test_snippet("rust-count", "rust", Some(Visibility::Public));
    repo.create(rust_snip, user_id, tenant_id).await.unwrap();

    let py_snip = create_test_snippet("python-count", "python", Some(Visibility::Public));
    repo.create(py_snip, user_id, tenant_id).await.unwrap();

    let filter = SnippetFilter {
        language: Some("python".to_string()),
        ..SnippetFilter::default()
    };
    let count = repo.count(&filter).await.unwrap();
    assert_eq!(count, 1);
}

// ── Delete ──────────────────────────────────────────────────────

#[tokio::test]
async fn test_delete_snippet() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteSnippetRepository::new(pool.clone());

    let s = create_test_snippet("to-delete", "rust", Some(Visibility::Public));
    let created = repo.create(s, user_id, tenant_id).await.unwrap();

    repo.delete(created.id).await.unwrap();

    let found = repo.find_by_id(created.id).await.unwrap();
    assert!(found.is_none());
}

#[tokio::test]
async fn test_delete_nonexistent_snippet() {
    let pool = setup_test_db().await;
    let repo = SqliteSnippetRepository::new(pool.clone());

    let result = repo.delete(Uuid::new_v4()).await;
    assert!(result.is_ok());
}

// ── Estimated tokens ────────────────────────────────────────────

#[tokio::test]
async fn test_estimated_tokens_calculation() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteSnippetRepository::new(pool.clone());

    let s = NewSnippet {
        name: "token-test".to_string(),
        language: "rust".to_string(),
        framework: None,
        tags: vec![],
        content: "a".repeat(100),
        code: "b".repeat(300),
        dependencies: vec![],
        visibility: Some(Visibility::Public),
        version: "1.0.0".to_string(),
        summary: None,
        command: None,
        subcommands: vec![],
        inputs: vec![],
        output: None,
        examples: vec![],
        error_model: None,
    };

    let created = repo.create(s, user_id, tenant_id).await.unwrap();
    // (100 + 300) / 4 = 100
    assert_eq!(created.estimated_tokens, 100);
}

// ── Dependencies ────────────────────────────────────────────────

#[tokio::test]
async fn test_snippet_with_dependencies() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteSnippetRepository::new(pool.clone());

    let deps = vec![
        Dependency {
            name: "tokio".to_string(),
            version: "1.0".to_string(),
            required: true,
        },
        Dependency {
            name: "serde".to_string(),
            version: "1.0".to_string(),
            required: false,
        },
    ];

    let s = NewSnippet {
        name: "deps-snippet".to_string(),
        language: "rust".to_string(),
        framework: None,
        tags: vec![],
        content: "Snippet with deps".to_string(),
        code: "use tokio;".to_string(),
        dependencies: deps,
        visibility: Some(Visibility::Public),
        version: "1.0.0".to_string(),
        summary: None,
        command: None,
        subcommands: vec![],
        inputs: vec![],
        output: None,
        examples: vec![],
        error_model: None,
    };

    let created = repo.create(s, user_id, tenant_id).await.unwrap();
    assert_eq!(created.dependencies.len(), 2);

    // Verify round-trip through DB
    let found = repo.find_by_id(created.id).await.unwrap().unwrap();
    assert_eq!(found.dependencies.len(), 2);
    assert_eq!(found.dependencies[0].name, "tokio");
    assert_eq!(found.dependencies[1].name, "serde");
}

// ── Tags ────────────────────────────────────────────────────────

#[tokio::test]
async fn test_snippet_tags_round_trip() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteSnippetRepository::new(pool.clone());

    let s = NewSnippet {
        name: "tagged-snippet".to_string(),
        language: "typescript".to_string(),
        framework: Some("next.js".to_string()),
        tags: vec![
            "react".to_string(),
            "ssr".to_string(),
            "frontend".to_string(),
        ],
        content: "Server-side rendering".to_string(),
        code: "export default function Page() {}".to_string(),
        dependencies: vec![],
        visibility: Some(Visibility::Public),
        version: "1.0.0".to_string(),
        summary: None,
        command: None,
        subcommands: vec![],
        inputs: vec![],
        output: None,
        examples: vec![],
        error_model: None,
    };

    let created = repo.create(s, user_id, tenant_id).await.unwrap();
    let found = repo.find_by_id(created.id).await.unwrap().unwrap();
    assert_eq!(found.tags, vec!["react", "ssr", "frontend"]);
}

#[tokio::test]
async fn test_create_snippet_with_cli_fields() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteSnippetRepository::new(pool.clone());

    let new_snippet = NewSnippet {
        name: "cli-snippet".to_string(),
        language: "rust".to_string(),
        framework: None,
        tags: vec!["cli".to_string()],
        content: "CLI tool documentation".to_string(),
        code: "fn main() {}".to_string(),
        dependencies: vec![],
        visibility: Some(Visibility::Public),
        version: "2.0.0".to_string(),
        summary: Some("A CLI tool snippet".to_string()),
        command: Some("mytool".to_string()),
        subcommands: vec![
            serde_json::json!({"name": "init", "description": "Initialize project"}),
            serde_json::json!({"name": "build", "description": "Build project"}),
        ],
        inputs: vec![serde_json::json!({"name": "project_name", "type": "string"})],
        output: Some(serde_json::json!({"type": "object", "properties": {"status": "string"}})),
        examples: vec![serde_json::json!({"command": "mytool init", "output": "Project initialized"})],
        error_model: Some(serde_json::json!({"codes": [1, 2, 3]})),
    };

    let snippet = repo.create(new_snippet, user_id, tenant_id).await.unwrap();

    assert_eq!(snippet.version, "2.0.0");
    assert_eq!(snippet.summary, Some("A CLI tool snippet".to_string()));
    assert_eq!(snippet.command, Some("mytool".to_string()));
    assert_eq!(snippet.subcommands.len(), 2);
    assert_eq!(snippet.inputs.len(), 1);
    assert!(snippet.output.is_some());
    assert_eq!(snippet.examples.len(), 1);
    assert!(snippet.error_model.is_some());

    let found = repo.find_by_id(snippet.id).await.unwrap().unwrap();
    assert_eq!(found.version, "2.0.0");
    assert_eq!(found.summary, Some("A CLI tool snippet".to_string()));
    assert_eq!(found.command, Some("mytool".to_string()));
    assert_eq!(found.subcommands.len(), 2);
    assert_eq!(found.inputs.len(), 1);
    assert!(found.output.is_some());
    assert_eq!(found.examples.len(), 1);
    assert!(found.error_model.is_some());
}

#[tokio::test]
async fn test_update_snippet() {
    let pool = setup_test_db().await;
    let (tenant_id, user_id) = create_test_tenant_and_user(&pool).await;
    let repo = SqliteSnippetRepository::new(pool.clone());

    let new_snippet = create_test_snippet("update-test", "rust", Some(Visibility::Private));
    let created = repo.create(new_snippet, user_id, tenant_id).await.unwrap();

    let update = UpdateSnippet {
        name: None,
        language: None,
        framework: Some("tokio".to_string()),
        tags: None,
        content: None,
        code: None,
        dependencies: None,
        estimated_tokens: None,
        visibility: None,
        version: Some("3.0.0".to_string()),
        summary: Some("Updated summary".to_string()),
        command: Some("updated-cmd".to_string()),
        subcommands: Some(vec![serde_json::json!({"name": "run"})]),
        inputs: Some(vec![]),
        output: None,
        examples: Some(vec![serde_json::json!({"cmd": "updated-cmd run"})]),
        error_model: None,
    };

    let updated = repo.update(created.id, update).await.unwrap();

    assert_eq!(updated.version, "3.0.0");
    assert_eq!(updated.summary, Some("Updated summary".to_string()));
    assert_eq!(updated.command, Some("updated-cmd".to_string()));
    assert_eq!(updated.subcommands.len(), 1);
    assert_eq!(updated.framework, Some("tokio".to_string()));
    assert_eq!(updated.examples.len(), 1);

    let found = repo.find_by_id(created.id).await.unwrap().unwrap();
    assert_eq!(found.version, "3.0.0");
    assert_eq!(found.summary, Some("Updated summary".to_string()));
    assert_eq!(found.command, Some("updated-cmd".to_string()));
}
