#![allow(clippy::unwrap_used)]

use domain::git_repo::{NewGitRepo, RepoLifecycleStatus};
use domain::repository::GitRepoRepository;
use infra::db::PgGitRepoRepository;
use sqlx::postgres::PgPoolOptions;
use sqlx::Row;
use uuid::Uuid;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../migrations/postgres");

#[tokio::test]
async fn postgres_repo_lifecycle_upgrade_and_repository_round_trip() {
    let Ok(database_url) = std::env::var("TEST_POSTGRES_URL") else {
        eprintln!("TEST_POSTGRES_URL is not set; skipping PostgreSQL integration test");
        return;
    };
    assert!(
        database_url.contains("_test"),
        "TEST_POSTGRES_URL must point to a dedicated *_test database"
    );

    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&database_url)
        .await
        .expect("connect dedicated PostgreSQL test database");
    sqlx::raw_sql("DROP SCHEMA public CASCADE; CREATE SCHEMA public")
        .execute(&pool)
        .await
        .expect("reset dedicated PostgreSQL test schema");

    MIGRATOR
        .run_to(9, &pool)
        .await
        .expect("apply PostgreSQL migrations through 009");

    let tenant_id = Uuid::new_v4();
    let owner_id = Uuid::new_v4();
    let legacy_repo_id = Uuid::new_v4();
    sqlx::query("INSERT INTO tenants (id, name, slug, owner_id) VALUES ($1, $2, $3, $4)")
        .bind(tenant_id)
        .bind("Lifecycle Test")
        .bind(format!("lifecycle-{tenant_id}"))
        .bind(owner_id)
        .execute(&pool)
        .await
        .expect("insert tenant before lifecycle upgrade");
    sqlx::query(
        r#"INSERT INTO git_repos
           (id, tenant_id, name, description, default_branch, storage_path,
            visibility, auto_merge, require_review)
           VALUES ($1, $2, 'legacy', '', 'main', $3, 'private', false, true)"#,
    )
    .bind(legacy_repo_id)
    .bind(tenant_id)
    .bind(format!("{tenant_id}/{legacy_repo_id}.git"))
    .execute(&pool)
    .await
    .expect("insert pre-010 repo row");

    MIGRATOR
        .run(&pool)
        .await
        .expect("upgrade PostgreSQL schema through 010");
    let legacy_status: String = sqlx::query("SELECT lifecycle_status FROM git_repos WHERE id = $1")
        .bind(legacy_repo_id)
        .fetch_one(&pool)
        .await
        .expect("read upgraded legacy repo")
        .get("lifecycle_status");
    assert_eq!(legacy_status, "ACTIVE");

    let repository = PgGitRepoRepository::new(pool.clone());
    let created = repository
        .create(
            NewGitRepo {
                name: "new-repo".to_string(),
                description: None,
                default_branch: None,
                visibility: None,
                auto_merge: None,
                require_review: None,
            },
            tenant_id,
        )
        .await
        .expect("create PostgreSQL repo metadata");
    assert_eq!(created.lifecycle_status, RepoLifecycleStatus::Creating);

    let active = repository
        .update_lifecycle_status(created.id, RepoLifecycleStatus::Active)
        .await
        .expect("activate PostgreSQL repo metadata");
    assert_eq!(active.lifecycle_status, RepoLifecycleStatus::Active);

    sqlx::raw_sql("DROP SCHEMA public CASCADE; CREATE SCHEMA public")
        .execute(&pool)
        .await
        .expect("clean dedicated PostgreSQL test schema");
}
