#![allow(clippy::unwrap_used)]

use std::process::Stdio;
use std::time::Duration as StdDuration;

use chrono::{Duration, Utc};
use domain::repository::OutboxRepository;
use domain::NewOutboxEvent;
use infra::db::{PgOutboxRepository, SqliteOutboxRepository};
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use sqlx::sqlite::SqlitePoolOptions;
use tempfile::TempDir;
use tokio::process::Command;

const SECRET: &str = "must-not-appear-in-worker-output";

fn event(key: String, event_type: &str, max_attempts: u32) -> NewOutboxEvent {
    NewOutboxEvent {
        event_type: event_type.to_string(),
        aggregate_type: "worker-test".to_string(),
        aggregate_id: key.clone(),
        payload: json!({"secret": SECRET}),
        idempotency_key: key,
        max_attempts,
    }
}

fn worker_command(database_type: &str, database_url: &str) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_outbox-worker"));
    command
        .env_remove("ENVIRONMENT")
        .env("DATABASE__DATABASE_TYPE", database_type)
        .env("DATABASE__URL", database_url)
        .env("DATABASE__MAX_CONNECTIONS", "5")
        .env("OUTBOX__BATCH_SIZE", "200")
        .env("OUTBOX__DELIVERY_TIMEOUT_SECONDS", "1")
        .env("OUTBOX__LEASE_SECONDS", "300")
        .env("OUTBOX__POLL_INTERVAL_MS", "10")
        .env("OUTBOX__BASE_BACKOFF_SECONDS", "1")
        .env("RUST_LOG", "info");
    command
}

#[tokio::test]
async fn sqlite_process_recovers_backlog_sanitizes_errors_and_controls_replay() {
    let directory = TempDir::new().expect("temporary SQLite directory");
    let database_path = directory.path().join("worker.db");
    let database_url = format!("sqlite://{}?mode=rwc", database_path.display());
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("connect SQLite process database");
    sqlx::migrate!("./migrations/sqlite")
        .run(&pool)
        .await
        .expect("migrate SQLite process database");
    let repository = SqliteOutboxRepository::new(pool.clone());
    for index in 0..101 {
        repository
            .enqueue(event(
                format!("sqlite-probe-{index}"),
                "system.outbox.probe",
                3,
            ))
            .await
            .expect("enqueue SQLite probe event");
    }
    repository
        .claim_due(1, Utc::now(), Duration::seconds(1))
        .await
        .expect("create an abandoned SQLite claim");
    tokio::time::sleep(StdDuration::from_millis(1100)).await;
    let unknown = repository
        .enqueue(event(
            "sqlite-unknown".to_string(),
            "unknown.secret-bearing-event",
            1,
        ))
        .await
        .expect("enqueue unknown event");

    let output = worker_command("sqlite", &database_url)
        .arg("run")
        .arg("--once")
        .output()
        .await
        .expect("run SQLite worker process");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!combined.contains(SECRET));
    let delivered: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM outbox_events WHERE event_type='system.outbox.probe' AND status='delivered'",
    )
    .fetch_one(&pool)
    .await
    .expect("count delivered probes");
    assert_eq!(delivered, 101);
    let failure: (String, Option<String>) =
        sqlx::query_as("SELECT status,last_error FROM outbox_events WHERE id=?")
            .bind(unknown.id.to_string())
            .fetch_one(&pool)
            .await
            .expect("read sanitized failure");
    assert_eq!(
        failure,
        (
            "dead_letter".to_string(),
            Some("SERVICE_UNAVAILABLE".to_string())
        )
    );

    let refused = worker_command("sqlite", &database_url)
        .arg("replay")
        .arg(unknown.id.to_string())
        .output()
        .await
        .expect("run unconfirmed replay");
    assert!(!refused.status.success());
    let status: String = sqlx::query_scalar("SELECT status FROM outbox_events WHERE id=?")
        .bind(unknown.id.to_string())
        .fetch_one(&pool)
        .await
        .expect("confirm refused replay did not mutate state");
    assert_eq!(status, "dead_letter");

    let replayed = worker_command("sqlite", &database_url)
        .arg("replay")
        .arg(unknown.id.to_string())
        .arg("--confirm")
        .output()
        .await
        .expect("run confirmed replay");
    assert!(replayed.status.success());
    let reset: (String, i64, Option<String>) =
        sqlx::query_as("SELECT status,attempts,last_error FROM outbox_events WHERE id=?")
            .bind(unknown.id.to_string())
            .fetch_one(&pool)
            .await
            .expect("read replayed event");
    assert_eq!(reset, ("pending".to_string(), 0, None));
}

#[tokio::test]
async fn continuous_process_exits_cleanly_on_sigint() {
    let directory = TempDir::new().expect("temporary SQLite directory");
    let database_path = directory.path().join("graceful.db");
    let database_url = format!("sqlite://{}?mode=rwc", database_path.display());
    let pool = SqlitePoolOptions::new()
        .max_connections(2)
        .connect(&database_url)
        .await
        .expect("connect graceful shutdown database");
    sqlx::migrate!("./migrations/sqlite")
        .run(&pool)
        .await
        .expect("migrate graceful shutdown database");
    SqliteOutboxRepository::new(pool.clone())
        .enqueue(event(
            "graceful-ready".to_string(),
            "system.outbox.probe",
            3,
        ))
        .await
        .expect("enqueue readiness probe");
    let child = worker_command("sqlite", &database_url)
        .arg("run")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn continuous worker");
    let ready_deadline = tokio::time::Instant::now() + StdDuration::from_secs(5);
    loop {
        let delivered: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM outbox_events WHERE idempotency_key='graceful-ready' AND status='delivered'",
        )
        .fetch_one(&pool)
        .await
        .expect("poll graceful worker readiness");
        if delivered == 1 {
            break;
        }
        assert!(
            tokio::time::Instant::now() < ready_deadline,
            "worker did not become ready"
        );
        tokio::time::sleep(StdDuration::from_millis(25)).await;
    }
    let pid = child.id().expect("worker process id");
    let signal = Command::new("kill")
        .arg("-INT")
        .arg(pid.to_string())
        .status()
        .await
        .expect("send SIGINT");
    assert!(signal.success());
    let output = child
        .wait_with_output()
        .await
        .expect("wait for worker shutdown");
    assert!(
        output.status.success(),
        "status={:?}\nstdout={}\nstderr={}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("shutdown complete"));
}

#[tokio::test]
async fn postgres_process_recovers_large_backlog() {
    let Ok(database_url) = std::env::var("TEST_POSTGRES_URL") else {
        eprintln!("TEST_POSTGRES_URL is not set; skipping PostgreSQL process test");
        return;
    };
    assert!(database_url.contains("_test"));
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("connect dedicated PostgreSQL test database");
    sqlx::raw_sql("DROP SCHEMA public CASCADE; CREATE SCHEMA public")
        .execute(&pool)
        .await
        .expect("reset PostgreSQL process schema");
    sqlx::migrate!("./migrations/postgres")
        .run(&pool)
        .await
        .expect("migrate PostgreSQL process database");
    let repository = PgOutboxRepository::new(pool.clone());
    for index in 0..101 {
        repository
            .enqueue(event(
                format!("postgres-probe-{index}"),
                "system.outbox.probe",
                3,
            ))
            .await
            .expect("enqueue PostgreSQL probe event");
    }
    repository
        .claim_due(1, Utc::now(), Duration::seconds(1))
        .await
        .expect("create an abandoned PostgreSQL claim");
    tokio::time::sleep(StdDuration::from_millis(1100)).await;
    let output = worker_command("postgres", &database_url)
        .arg("run")
        .arg("--once")
        .output()
        .await
        .expect("run PostgreSQL worker process");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let delivered: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM outbox_events WHERE event_type='system.outbox.probe' AND status='delivered'",
    )
    .fetch_one(&pool)
    .await
    .expect("count PostgreSQL delivered probes");
    assert_eq!(delivered, 101);
    sqlx::raw_sql("DROP SCHEMA public CASCADE; CREATE SCHEMA public")
        .execute(&pool)
        .await
        .expect("clean PostgreSQL process schema");
}
