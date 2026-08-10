use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use common::error::{AppError, Result};
use domain::repository::OutboxRepository;
use domain::{NewOutboxEvent, OutboxEvent, OutboxStatus};
use sqlx::{PgPool, Row};
use uuid::Uuid;

pub struct PgOutboxRepository {
    pool: PgPool,
}
impl PgOutboxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

fn status(value: &str) -> Result<OutboxStatus> {
    match value {
        "pending" => Ok(OutboxStatus::Pending),
        "processing" => Ok(OutboxStatus::Processing),
        "delivered" => Ok(OutboxStatus::Delivered),
        "failed" => Ok(OutboxStatus::Failed),
        "dead_letter" => Ok(OutboxStatus::DeadLetter),
        _ => Err(AppError::DatabaseError(format!(
            "unknown outbox status: {value}"
        ))),
    }
}
fn row_to_event(row: &sqlx::postgres::PgRow) -> Result<OutboxEvent> {
    Ok(OutboxEvent {
        id: row.get("id"),
        event_type: row.get("event_type"),
        aggregate_type: row.get("aggregate_type"),
        aggregate_id: row.get("aggregate_id"),
        payload: row.get("payload"),
        idempotency_key: row.get("idempotency_key"),
        status: status(row.get::<String, _>("status").as_str())?,
        attempts: row.get::<i32, _>("attempts") as u32,
        max_attempts: row.get::<i32, _>("max_attempts") as u32,
        next_attempt_at: row.get("next_attempt_at"),
        last_error: row.get("last_error"),
        claim_token: row.get("claim_token"),
        lease_expires_at: row.get("lease_expires_at"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

#[async_trait]
impl OutboxRepository for PgOutboxRepository {
    async fn enqueue(&self, event: NewOutboxEvent) -> Result<OutboxEvent> {
        if event.max_attempts == 0 {
            return Err(AppError::ValidationError(
                "outbox max_attempts must be at least 1".to_string(),
            ));
        }
        let id = Uuid::new_v4();
        let now = Utc::now();
        sqlx::query("INSERT INTO outbox_events (id,event_type,aggregate_type,aggregate_id,payload,idempotency_key,max_attempts,next_attempt_at,created_at,updated_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$8,$8)")
            .bind(id).bind(event.event_type).bind(event.aggregate_type).bind(event.aggregate_id).bind(event.payload).bind(event.idempotency_key).bind(event.max_attempts as i32).bind(now).execute(&self.pool).await.map_err(|e| AppError::DatabaseError(e.to_string()))?;
        let row = sqlx::query("SELECT * FROM outbox_events WHERE id=$1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        row_to_event(&row)
    }
    async fn enqueue_idempotent(&self, event: NewOutboxEvent) -> Result<OutboxEvent> {
        if event.max_attempts == 0 {
            return Err(AppError::ValidationError(
                "outbox max_attempts must be at least 1".to_string(),
            ));
        }
        let id = Uuid::new_v4();
        let now = Utc::now();
        let mut transaction = self
            .pool
            .begin()
            .await
            .map_err(|error| AppError::DatabaseError(error.to_string()))?;
        sqlx::query(
            "INSERT INTO outbox_events \
             (id,event_type,aggregate_type,aggregate_id,payload,idempotency_key,max_attempts, \
              next_attempt_at,created_at,updated_at) \
             VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$8,$8) \
             ON CONFLICT (idempotency_key) DO NOTHING",
        )
        .bind(id)
        .bind(&event.event_type)
        .bind(&event.aggregate_type)
        .bind(&event.aggregate_id)
        .bind(&event.payload)
        .bind(&event.idempotency_key)
        .bind(event.max_attempts as i32)
        .bind(now)
        .execute(&mut *transaction)
        .await
        .map_err(|error| AppError::DatabaseError(error.to_string()))?;
        let row = sqlx::query("SELECT * FROM outbox_events WHERE idempotency_key=$1")
            .bind(&event.idempotency_key)
            .fetch_one(&mut *transaction)
            .await
            .map_err(|error| AppError::DatabaseError(error.to_string()))?;
        let persisted = row_to_event(&row)?;
        if persisted.event_type != event.event_type
            || persisted.aggregate_type != event.aggregate_type
            || persisted.aggregate_id != event.aggregate_id
            || persisted.payload != event.payload
            || persisted.max_attempts != event.max_attempts
        {
            return Err(AppError::ConflictError(
                "outbox idempotency key conflicts with another event".to_string(),
            ));
        }
        transaction
            .commit()
            .await
            .map_err(|error| AppError::DatabaseError(error.to_string()))?;
        Ok(persisted)
    }
    async fn claim_due(
        &self,
        limit: u32,
        now: DateTime<Utc>,
        lease_duration: Duration,
    ) -> Result<Vec<OutboxEvent>> {
        if lease_duration <= Duration::zero() {
            return Err(AppError::ValidationError(
                "outbox claim lease must be positive".to_string(),
            ));
        }
        let claim_token = Uuid::new_v4();
        let lease_expires_at = now + lease_duration;
        let mut transaction = self
            .pool
            .begin()
            .await
            .map_err(|error| AppError::DatabaseError(error.to_string()))?;
        sqlx::query(
            "UPDATE outbox_events \
             SET status='dead_letter', claim_token=NULL, lease_expires_at=NULL, \
                 last_error='CLAIM_LEASE_EXPIRED', \
                 updated_at=$1 \
             WHERE status='processing' AND lease_expires_at <= $1 AND attempts >= max_attempts",
        )
        .bind(now)
        .execute(&mut *transaction)
        .await
        .map_err(|error| AppError::DatabaseError(error.to_string()))?;
        let rows = sqlx::query(
            "WITH due AS ( \
                 SELECT id FROM outbox_events \
                 WHERE attempts < max_attempts AND ( \
                     (status IN ('pending','failed') AND next_attempt_at <= $1) OR \
                     (status='processing' AND lease_expires_at <= $1) \
                 ) \
                 ORDER BY created_at FOR UPDATE SKIP LOCKED LIMIT $2 \
             ) \
             UPDATE outbox_events AS events \
             SET status='processing', attempts=events.attempts+1, claim_token=$3, \
                 lease_expires_at=$4, updated_at=$1 \
             FROM due WHERE events.id=due.id RETURNING events.*",
        )
        .bind(now)
        .bind(i64::from(limit))
        .bind(claim_token)
        .bind(lease_expires_at)
        .fetch_all(&mut *transaction)
        .await
        .map_err(|error| AppError::DatabaseError(error.to_string()))?;
        transaction
            .commit()
            .await
            .map_err(|error| AppError::DatabaseError(error.to_string()))?;
        rows.iter().map(row_to_event).collect()
    }
    async fn mark_delivered(&self, id: Uuid, claim_token: Uuid) -> Result<()> {
        let result = sqlx::query(
            "UPDATE outbox_events \
             SET status='delivered', claim_token=NULL, lease_expires_at=NULL, updated_at=$1 \
             WHERE id=$2 AND status='processing' AND claim_token=$3",
        )
        .bind(Utc::now())
        .bind(id)
        .bind(claim_token)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        if result.rows_affected() != 1 {
            return Err(AppError::ConflictError(
                "outbox claim is no longer current".to_string(),
            ));
        }
        Ok(())
    }
    async fn mark_failed(
        &self,
        id: Uuid,
        claim_token: Uuid,
        error: &str,
        next: DateTime<Utc>,
    ) -> Result<()> {
        let result = sqlx::query(
            "UPDATE outbox_events \
             SET status=CASE WHEN attempts >= max_attempts THEN 'dead_letter' ELSE 'failed' END, \
                 last_error=$1, next_attempt_at=$2, claim_token=NULL, lease_expires_at=NULL, updated_at=$3 \
             WHERE id=$4 AND status='processing' AND claim_token=$5",
        )
        .bind(error)
        .bind(next)
        .bind(Utc::now())
        .bind(id)
        .bind(claim_token)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        if result.rows_affected() != 1 {
            return Err(AppError::ConflictError(
                "outbox claim is no longer current".to_string(),
            ));
        }
        Ok(())
    }

    async fn replay_dead_letter(&self, id: Uuid, now: DateTime<Utc>) -> Result<OutboxEvent> {
        let row = sqlx::query(
            "UPDATE outbox_events \
             SET status='pending', attempts=0, next_attempt_at=$1, last_error=NULL, \
                 claim_token=NULL, lease_expires_at=NULL, updated_at=$1 \
             WHERE id=$2 AND status='dead_letter' RETURNING *",
        )
        .bind(now)
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|error| AppError::DatabaseError(error.to_string()))?
        .ok_or_else(|| {
            AppError::ConflictError("outbox event is not in dead-letter state".to_string())
        })?;
        row_to_event(&row)
    }
}
