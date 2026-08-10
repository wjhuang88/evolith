#![allow(clippy::unwrap_used)]

use std::collections::HashSet;

use chrono::{Duration, Utc};
use domain::repository::OutboxRepository;
use domain::NewOutboxEvent;
use infra::db::PgOutboxRepository;
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../migrations/postgres");

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

#[tokio::test]
async fn postgres_claims_are_disjoint_and_expired_claims_are_fenced() {
    let Ok(database_url) = std::env::var("TEST_POSTGRES_URL") else {
        eprintln!("TEST_POSTGRES_URL is not set; skipping PostgreSQL integration test");
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
        .run_to(11, &pool)
        .await
        .expect("apply PostgreSQL migrations through 011");
    let existing_event_id = Uuid::new_v4();
    let existing_at = Utc::now();
    sqlx::query(
        "INSERT INTO outbox_events \
         (id,event_type,aggregate_type,aggregate_id,payload,idempotency_key,status,attempts, \
          max_attempts,next_attempt_at,created_at,updated_at) \
         VALUES ($1,$2,$3,$4,$5,$6,'delivered',1,3,$7,$7,$7)",
    )
    .bind(existing_event_id)
    .bind("repo.updated")
    .bind("repo")
    .bind("repo-1")
    .bind(json!({}))
    .bind("postgres-upgrade-existing")
    .bind(existing_at)
    .execute(&pool)
    .await
    .expect("insert pre-012 PostgreSQL event");
    MIGRATOR
        .run(&pool)
        .await
        .expect("upgrade PostgreSQL through 012");
    let first = PgOutboxRepository::new(pool.clone())
        .enqueue_idempotent(event("postgres-idempotent", 3))
        .await
        .expect("first PostgreSQL idempotent enqueue");
    let duplicate = PgOutboxRepository::new(pool.clone())
        .enqueue_idempotent(event("postgres-idempotent", 3))
        .await
        .expect("duplicate PostgreSQL idempotent enqueue");
    assert_eq!(duplicate.id, first.id);
    let mut collision = event("postgres-idempotent", 3);
    collision.payload = serde_json::json!({"sha":"different"});
    assert!(PgOutboxRepository::new(pool.clone())
        .enqueue_idempotent(collision)
        .await
        .is_err());
    sqlx::query("DELETE FROM outbox_events WHERE id=$1")
        .bind(first.id)
        .execute(&pool)
        .await
        .expect("remove idempotency fixture before claim assertions");
    let upgraded: (String, Option<Uuid>, Option<chrono::DateTime<Utc>>) = sqlx::query_as(
        "SELECT status, claim_token, lease_expires_at FROM outbox_events WHERE id=$1",
    )
    .bind(existing_event_id)
    .fetch_one(&pool)
    .await
    .expect("read upgraded PostgreSQL event");
    assert_eq!(upgraded, ("delivered".to_string(), None, None));

    let writer = PgOutboxRepository::new(pool.clone());
    for index in 0..10 {
        writer
            .enqueue(event(&format!("concurrent-claim-{index}"), 3))
            .await
            .expect("enqueue concurrent claim event");
    }
    let first_repository = PgOutboxRepository::new(pool.clone());
    let second_repository = PgOutboxRepository::new(pool.clone());
    let claimed_at = Utc::now();
    let (first, second) = tokio::join!(
        first_repository.claim_due(5, claimed_at, Duration::seconds(30)),
        second_repository.claim_due(5, claimed_at, Duration::seconds(30)),
    );
    let first = first.expect("first concurrent claim");
    let second = second.expect("second concurrent claim");
    assert_eq!(first.len(), 5);
    assert_eq!(second.len(), 5);
    let first_ids: HashSet<_> = first.iter().map(|event| event.id).collect();
    let second_ids: HashSet<_> = second.iter().map(|event| event.id).collect();
    assert!(first_ids.is_disjoint(&second_ids));
    assert_eq!(first_ids.union(&second_ids).count(), 10);

    let crash_event = writer
        .enqueue(event("postgres-crash-recovery", 3))
        .await
        .expect("enqueue crash event");
    let crash_claimed_at = Utc::now();
    let stale = writer
        .claim_due(1, crash_claimed_at, Duration::seconds(1))
        .await
        .expect("claim crash event");
    let stale_token = stale[0].claim_token.expect("stale claim token");
    let reclaimed = writer
        .claim_due(
            1,
            crash_claimed_at + Duration::seconds(2),
            Duration::seconds(30),
        )
        .await
        .expect("reclaim crash event");
    assert_eq!(reclaimed.len(), 1);
    assert_eq!(reclaimed[0].id, crash_event.id);
    let current_token = reclaimed[0].claim_token.expect("current claim token");
    assert_ne!(stale_token, current_token);
    assert!(writer
        .mark_delivered(crash_event.id, stale_token)
        .await
        .is_err());
    assert!(writer
        .mark_failed(crash_event.id, stale_token, "late failure", Utc::now())
        .await
        .is_err());
    writer
        .mark_delivered(crash_event.id, current_token)
        .await
        .expect("complete current claim");

    let dead_letter = writer
        .enqueue(event("postgres-dead-letter-replay", 1))
        .await
        .expect("enqueue replay event");
    let replay_claim = writer
        .claim_due(1, Utc::now(), Duration::seconds(30))
        .await
        .expect("claim replay event");
    writer
        .mark_failed(
            dead_letter.id,
            replay_claim[0].claim_token.expect("replay claim token"),
            "EXTERNAL_SERVICE_ERROR",
            Utc::now(),
        )
        .await
        .expect("dead-letter replay event");
    let replayed = writer
        .replay_dead_letter(dead_letter.id, Utc::now())
        .await
        .expect("replay PostgreSQL dead letter");
    assert_eq!(replayed.status.as_str(), "pending");
    assert_eq!(replayed.attempts, 0);
    assert!(replayed.last_error.is_none());
    assert!(writer
        .replay_dead_letter(crash_event.id, Utc::now())
        .await
        .is_err());

    sqlx::raw_sql("DROP SCHEMA public CASCADE; CREATE SCHEMA public")
        .execute(&pool)
        .await
        .expect("clean dedicated PostgreSQL test schema");
}
