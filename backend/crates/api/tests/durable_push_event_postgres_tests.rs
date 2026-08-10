#![allow(clippy::unwrap_used)]

use std::sync::Arc;

use api::services::repo_push_event::{reconcile_repo_push_completed, RepoPushEventHandler};
use domain::OutboxWorker;
use infra::db::{PgGitRepoRepository, PgOutboxRepository};
use sqlx::postgres::PgPoolOptions;
use tempfile::TempDir;
use uuid::Uuid;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../migrations/postgres");

#[tokio::test]
async fn postgres_push_event_reconcile_and_subscriber_are_idempotent() {
    let Ok(database_url) = std::env::var("TEST_POSTGRES_URL") else {
        eprintln!("TEST_POSTGRES_URL is not set; skipping PostgreSQL push event test");
        return;
    };
    assert!(
        database_url.contains("_test"),
        "TEST_POSTGRES_URL must point to a dedicated *_test database"
    );
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("connect dedicated PostgreSQL test database");
    sqlx::raw_sql("DROP SCHEMA public CASCADE; CREATE SCHEMA public")
        .execute(&pool)
        .await
        .expect("reset dedicated PostgreSQL test schema");
    MIGRATOR
        .run(&pool)
        .await
        .expect("run PostgreSQL migrations");

    let tenant_id = Uuid::new_v4();
    let repo_id = Uuid::new_v4();
    let owner_id = Uuid::new_v4();
    sqlx::query("INSERT INTO tenants (id,name,slug,owner_id) VALUES ($1,$2,$3,$4)")
        .bind(tenant_id)
        .bind("Push Event Test")
        .bind(format!("push-event-{tenant_id}"))
        .bind(owner_id)
        .execute(&pool)
        .await
        .expect("insert tenant fixture");

    let storage = TempDir::new().expect("Git storage temp dir");
    let repo_path = service_git::init_bare_repo(storage.path(), tenant_id, repo_id)
        .expect("initialize bare repo");
    let sha = service_git::seed_initial_commit(&repo_path, "push-event", "main")
        .expect("seed initial commit");
    sqlx::query(
        "INSERT INTO git_repos \
         (id,tenant_id,name,description,default_branch,storage_path,visibility,auto_merge, \
          require_review,lifecycle_status) \
         VALUES ($1,$2,$3,'','main',$4,'private',TRUE,FALSE,'ACTIVE')",
    )
    .bind(repo_id)
    .bind(tenant_id)
    .bind("push-event")
    .bind(repo_path.to_string_lossy().to_string())
    .execute(&pool)
    .await
    .expect("insert repo fixture");

    let outbox = PgOutboxRepository::new(pool.clone());
    let first = reconcile_repo_push_completed(&outbox, tenant_id, repo_id, "main", &repo_path)
        .await
        .expect("enqueue PostgreSQL push event")
        .expect("default branch event");
    let duplicate = reconcile_repo_push_completed(&outbox, tenant_id, repo_id, "main", &repo_path)
        .await
        .expect("repeat PostgreSQL reconciliation")
        .expect("default branch event");
    assert_eq!(first.id, duplicate.id);

    let worker = OutboxWorker::new(
        PgOutboxRepository::new(pool.clone()),
        RepoPushEventHandler::new(
            Arc::new(PgGitRepoRepository::new(pool.clone())),
            storage.path(),
        ),
        1,
        1,
        30,
        5,
    )
    .expect("build PostgreSQL push event worker");
    let batch = worker.process_once().await.expect("deliver push event");
    assert_eq!(batch.delivered, 1);
    let metadata_sha: Option<String> =
        sqlx::query_scalar("SELECT last_commit_sha FROM git_repos WHERE id=$1")
            .bind(repo_id)
            .fetch_one(&pool)
            .await
            .expect("read updated PostgreSQL metadata");
    assert_eq!(metadata_sha.as_deref(), Some(sha.as_str()));
    let event_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM outbox_events WHERE idempotency_key=$1")
            .bind(&first.idempotency_key)
            .fetch_one(&pool)
            .await
            .expect("count PostgreSQL event rows");
    assert_eq!(event_count, 1);
}
