mod test_helpers;

use chrono::{Duration, Utc};
use common::error::AppError;
use domain::repository::OutboxRepository;
use domain::{NewOutboxEvent, OutboxHandler, OutboxStatus, OutboxWorker};
use infra::db::SqliteOutboxRepository;
use serde_json::json;
use sqlx::sqlite::SqlitePoolOptions;
use test_helpers::setup_test_db;

fn event(key: &str, max_attempts: u32) -> NewOutboxEvent {
    NewOutboxEvent {
        event_type: "repo.updated".to_string(),
        aggregate_type: "repo".to_string(),
        aggregate_id: "repo-1".to_string(),
        payload: json!({"sha":"abc123"}),
        idempotency_key: key.to_string(),
        max_attempts,
    }
}

struct TestHandler {
    fail: bool,
}
#[async_trait::async_trait]
impl OutboxHandler for TestHandler {
    async fn deliver(&self, _event: &domain::OutboxEvent) -> common::error::Result<()> {
        if self.fail {
            Err(AppError::ExternalServiceError {
                service: "test".to_string(),
                message: "failed".to_string(),
            })
        } else {
            Ok(())
        }
    }
}

#[tokio::test]
async fn enqueue_stays_pending_and_claim_transitions_to_processing() {
    let pool = setup_test_db().await;
    let repo = SqliteOutboxRepository::new(pool);
    let created = repo
        .enqueue(event("repo-1-update-1", 3))
        .await
        .expect("enqueue");
    assert_eq!(created.status, OutboxStatus::Pending);
    assert_eq!(created.attempts, 0);

    let claimed = repo
        .claim_due(10, Utc::now(), Duration::seconds(30))
        .await
        .expect("claim");
    assert_eq!(claimed.len(), 1);
    assert_eq!(claimed[0].status, OutboxStatus::Processing);
    assert_eq!(claimed[0].attempts, 1);
    assert!(claimed[0].claim_token.is_some());
    assert!(claimed[0].lease_expires_at.is_some());

    let duplicate_claim = repo
        .claim_due(10, Utc::now(), Duration::seconds(30))
        .await
        .expect("claim within lease");
    assert!(duplicate_claim.is_empty());
}

#[tokio::test]
async fn duplicate_idempotency_key_is_rejected() {
    let pool = setup_test_db().await;
    let repo = SqliteOutboxRepository::new(pool);
    repo.enqueue(event("duplicate-key", 3))
        .await
        .expect("enqueue");
    assert!(repo.enqueue(event("duplicate-key", 3)).await.is_err());
}

#[tokio::test]
async fn invalid_attempt_and_lease_bounds_are_rejected() {
    let pool = setup_test_db().await;
    let repo = SqliteOutboxRepository::new(pool);
    assert!(repo.enqueue(event("zero-attempts", 0)).await.is_err());
    repo.enqueue(event("invalid-lease", 3))
        .await
        .expect("enqueue valid event");
    assert!(repo
        .claim_due(1, Utc::now(), Duration::zero())
        .await
        .is_err());
    assert!(repo
        .claim_due(1, Utc::now(), Duration::seconds(-1))
        .await
        .is_err());
}

#[tokio::test]
async fn sqlite_011_to_012_upgrade_preserves_existing_events() {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("create SQLite upgrade database");
    sqlx::raw_sql(include_str!(
        "../../../migrations/sqlite/011_outbox_events.sql"
    ))
    .execute(&pool)
    .await
    .expect("apply SQLite migration 011");
    let event_id = uuid::Uuid::new_v4();
    let now = Utc::now().to_rfc3339();
    sqlx::query(
        "INSERT INTO outbox_events \
         (id,event_type,aggregate_type,aggregate_id,payload,idempotency_key,status,attempts, \
          max_attempts,next_attempt_at,created_at,updated_at) \
         VALUES (?,?,?,?,?,?,'delivered',1,3,?,?,?)",
    )
    .bind(event_id.to_string())
    .bind("repo.updated")
    .bind("repo")
    .bind("repo-1")
    .bind("{}")
    .bind("sqlite-upgrade-existing")
    .bind(&now)
    .bind(&now)
    .bind(&now)
    .execute(&pool)
    .await
    .expect("insert pre-012 SQLite event");
    sqlx::raw_sql(include_str!(
        "../../../migrations/sqlite/012_outbox_claim_leases.sql"
    ))
    .execute(&pool)
    .await
    .expect("upgrade SQLite through 012");
    let row: (String, Option<String>, Option<String>) = sqlx::query_as(
        "SELECT status, claim_token, lease_expires_at FROM outbox_events WHERE id=?",
    )
    .bind(event_id.to_string())
    .fetch_one(&pool)
    .await
    .expect("read upgraded SQLite event");
    assert_eq!(row, ("delivered".to_string(), None, None));
}

#[tokio::test]
async fn failure_retries_then_moves_to_dead_letter() {
    let pool = setup_test_db().await;
    let repo = SqliteOutboxRepository::new(pool);
    let created = repo.enqueue(event("retry-key", 2)).await.expect("enqueue");
    let claimed = repo
        .claim_due(1, Utc::now(), Duration::seconds(30))
        .await
        .expect("claim");
    repo.mark_failed(
        created.id,
        claimed[0].claim_token.expect("claim token"),
        "delivery failed",
        Utc::now() - Duration::seconds(1),
    )
    .await
    .expect("fail");
    let next = repo
        .claim_due(1, Utc::now(), Duration::seconds(30))
        .await
        .expect("claim retry");
    assert_eq!(next[0].status, OutboxStatus::Processing);
    assert_eq!(next[0].attempts, 2);
    repo.mark_failed(
        next[0].id,
        next[0].claim_token.expect("retry claim token"),
        "delivery failed again",
        Utc::now(),
    )
    .await
    .expect("dead letter");
    let row = repo
        .claim_due(1, Utc::now(), Duration::seconds(30))
        .await
        .expect("no dead letter claim");
    assert!(row.is_empty());
    let _ = claimed;
}

#[tokio::test]
async fn worker_delivers_and_marks_event() {
    let pool = setup_test_db().await;
    let repo = SqliteOutboxRepository::new(pool);
    repo.enqueue(event("worker-success", 3))
        .await
        .expect("enqueue");
    let worker =
        OutboxWorker::new(repo, TestHandler { fail: false }, 10, 1, 30, 1).expect("valid worker");
    let result = worker.process_once().await.expect("process");
    assert_eq!(result.claimed, 1);
    assert_eq!(result.delivered, 1);
}

#[tokio::test]
async fn expired_claim_is_recovered_and_stale_completion_is_fenced() {
    let pool = setup_test_db().await;
    let repo = SqliteOutboxRepository::new(pool);
    let created = repo
        .enqueue(event("recover-after-crash", 3))
        .await
        .expect("enqueue");
    let first_claim_at = Utc::now();
    let first = repo
        .claim_due(1, first_claim_at, Duration::seconds(1))
        .await
        .expect("first claim");
    let first_token = first[0].claim_token.expect("first claim token");

    let second = repo
        .claim_due(
            1,
            first_claim_at + Duration::seconds(2),
            Duration::seconds(30),
        )
        .await
        .expect("reclaim expired lease");
    assert_eq!(second.len(), 1);
    assert_eq!(second[0].id, created.id);
    assert_eq!(second[0].attempts, 2);
    let second_token = second[0].claim_token.expect("second claim token");
    assert_ne!(first_token, second_token);

    assert!(repo.mark_delivered(created.id, first_token).await.is_err());
    assert!(repo
        .mark_failed(created.id, first_token, "late failure", Utc::now())
        .await
        .is_err());
    repo.mark_delivered(created.id, second_token)
        .await
        .expect("current claim completes");
}

#[tokio::test]
async fn expired_final_attempt_moves_to_dead_letter() {
    let pool = setup_test_db().await;
    let repo = SqliteOutboxRepository::new(pool.clone());
    let created = repo
        .enqueue(event("crash-on-final-attempt", 1))
        .await
        .expect("enqueue");
    let claimed_at = Utc::now();
    repo.claim_due(1, claimed_at, Duration::seconds(1))
        .await
        .expect("final claim");

    let recovered = repo
        .claim_due(1, claimed_at + Duration::seconds(2), Duration::seconds(30))
        .await
        .expect("recover final attempt");
    assert!(recovered.is_empty());
    let status: String = sqlx::query_scalar("SELECT status FROM outbox_events WHERE id = ?")
        .bind(created.id.to_string())
        .fetch_one(&pool)
        .await
        .expect("read dead letter status");
    assert_eq!(status, "dead_letter");
    let error: String = sqlx::query_scalar("SELECT last_error FROM outbox_events WHERE id = ?")
        .bind(created.id.to_string())
        .fetch_one(&pool)
        .await
        .expect("read dead letter error");
    assert_eq!(error, "CLAIM_LEASE_EXPIRED");
}

#[tokio::test]
async fn dead_letter_replay_resets_delivery_state() {
    let pool = setup_test_db().await;
    let repo = SqliteOutboxRepository::new(pool);
    let created = repo
        .enqueue(event("replay-dead-letter", 1))
        .await
        .expect("enqueue");
    let claimed = repo
        .claim_due(1, Utc::now(), Duration::seconds(30))
        .await
        .expect("claim");
    repo.mark_failed(
        created.id,
        claimed[0].claim_token.expect("claim token"),
        "EXTERNAL_SERVICE_ERROR",
        Utc::now(),
    )
    .await
    .expect("dead letter");

    let replayed = repo
        .replay_dead_letter(created.id, Utc::now())
        .await
        .expect("replay");
    assert_eq!(replayed.status, OutboxStatus::Pending);
    assert_eq!(replayed.attempts, 0);
    assert!(replayed.last_error.is_none());
    assert!(replayed.claim_token.is_none());
    assert!(replayed.lease_expires_at.is_none());
}

#[tokio::test]
async fn replay_rejects_non_dead_letter_without_mutation() {
    let pool = setup_test_db().await;
    let repo = SqliteOutboxRepository::new(pool);
    let created = repo
        .enqueue(event("replay-pending", 3))
        .await
        .expect("enqueue");
    assert!(repo
        .replay_dead_letter(created.id, Utc::now())
        .await
        .is_err());
    let claimed = repo
        .claim_due(1, Utc::now(), Duration::seconds(30))
        .await
        .expect("pending event remains claimable");
    assert_eq!(claimed[0].id, created.id);
    assert_eq!(claimed[0].attempts, 1);
}
