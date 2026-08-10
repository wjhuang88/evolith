use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use common::error::{AppError, Result};
use domain::repository::OutboxRepository;
use domain::{NewOutboxEvent, OutboxEvent, OutboxStatus};
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

pub struct SqliteOutboxRepository {
    pool: SqlitePool,
}
impl SqliteOutboxRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

fn parse_status(value: &str) -> Result<OutboxStatus> {
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
fn row_to_event(row: &sqlx::sqlite::SqliteRow) -> Result<OutboxEvent> {
    let status = parse_status(&row.get::<String, _>("status"))?;
    Ok(OutboxEvent {
        id: row
            .get::<String, _>("id")
            .parse()
            .map_err(|e| AppError::DatabaseError(format!("invalid outbox id: {e}")))?,
        event_type: row.get("event_type"),
        aggregate_type: row.get("aggregate_type"),
        aggregate_id: row.get("aggregate_id"),
        payload: serde_json::from_str(&row.get::<String, _>("payload"))
            .map_err(|e| AppError::DatabaseError(e.to_string()))?,
        idempotency_key: row.get("idempotency_key"),
        status,
        attempts: row.get::<i64, _>("attempts") as u32,
        max_attempts: row.get::<i64, _>("max_attempts") as u32,
        next_attempt_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("next_attempt_at"))
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .with_timezone(&Utc),
        last_error: row.get("last_error"),
        claim_token: row
            .get::<Option<String>, _>("claim_token")
            .map(|value| {
                value.parse().map_err(|error| {
                    AppError::DatabaseError(format!("invalid outbox claim token: {error}"))
                })
            })
            .transpose()?,
        lease_expires_at: row
            .get::<Option<String>, _>("lease_expires_at")
            .map(|value| {
                DateTime::parse_from_rfc3339(&value)
                    .map(|parsed| parsed.with_timezone(&Utc))
                    .map_err(|error| AppError::DatabaseError(error.to_string()))
            })
            .transpose()?,
        created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .with_timezone(&Utc),
        updated_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("updated_at"))
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .with_timezone(&Utc),
    })
}

#[async_trait]
impl OutboxRepository for SqliteOutboxRepository {
    async fn enqueue(&self, event: NewOutboxEvent) -> Result<OutboxEvent> {
        if event.max_attempts == 0 {
            return Err(AppError::ValidationError(
                "outbox max_attempts must be at least 1".to_string(),
            ));
        }
        let id = Uuid::new_v4();
        let now = Utc::now();
        let payload = serde_json::to_string(&event.payload)
            .map_err(|e| AppError::ValidationError(e.to_string()))?;
        sqlx::query("INSERT INTO outbox_events (id,event_type,aggregate_type,aggregate_id,payload,idempotency_key,max_attempts,next_attempt_at,created_at,updated_at) VALUES (?,?,?,?,?,?,?,?,?,?)").bind(id.to_string()).bind(event.event_type).bind(event.aggregate_type).bind(event.aggregate_id).bind(payload).bind(event.idempotency_key).bind(event.max_attempts as i64).bind(now.to_rfc3339()).bind(now.to_rfc3339()).bind(now.to_rfc3339()).execute(&self.pool).await.map_err(|e| AppError::DatabaseError(e.to_string()))?;
        let row = sqlx::query("SELECT * FROM outbox_events WHERE id=?")
            .bind(id.to_string())
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        row_to_event(&row)
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
        let now = now.to_rfc3339();
        let lease_expires_at = (DateTime::parse_from_rfc3339(&now)
            .map_err(|error| AppError::DatabaseError(error.to_string()))?
            + lease_duration)
            .to_rfc3339();
        let claim_token = Uuid::new_v4().to_string();
        let mut transaction = self
            .pool
            .begin()
            .await
            .map_err(|error| AppError::DatabaseError(error.to_string()))?;

        sqlx::query(
            "UPDATE outbox_events \
             SET status='dead_letter', claim_token=NULL, lease_expires_at=NULL, \
                 last_error='CLAIM_LEASE_EXPIRED', \
                 updated_at=? \
             WHERE status='processing' AND lease_expires_at <= ? AND attempts >= max_attempts",
        )
        .bind(&now)
        .bind(&now)
        .execute(&mut *transaction)
        .await
        .map_err(|error| AppError::DatabaseError(error.to_string()))?;

        let rows = sqlx::query(
            "UPDATE outbox_events \
             SET status='processing', attempts=attempts+1, claim_token=?, \
                 lease_expires_at=?, updated_at=? \
             WHERE id IN ( \
                 SELECT id FROM outbox_events \
                 WHERE attempts < max_attempts AND ( \
                     (status IN ('pending','failed') AND next_attempt_at <= ?) OR \
                     (status='processing' AND lease_expires_at <= ?) \
                 ) \
                 ORDER BY created_at LIMIT ? \
             ) AND attempts < max_attempts AND ( \
                 (status IN ('pending','failed') AND next_attempt_at <= ?) OR \
                 (status='processing' AND lease_expires_at <= ?) \
             ) \
             RETURNING *",
        )
        .bind(claim_token)
        .bind(lease_expires_at)
        .bind(&now)
        .bind(&now)
        .bind(&now)
        .bind(i64::from(limit))
        .bind(&now)
        .bind(&now)
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
             SET status='delivered', claim_token=NULL, lease_expires_at=NULL, updated_at=? \
             WHERE id=? AND status='processing' AND claim_token=?",
        )
        .bind(Utc::now().to_rfc3339())
        .bind(id.to_string())
        .bind(claim_token.to_string())
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
                 last_error=?, next_attempt_at=?, claim_token=NULL, lease_expires_at=NULL, updated_at=? \
             WHERE id=? AND status='processing' AND claim_token=?",
        )
        .bind(error)
        .bind(next.to_rfc3339())
        .bind(Utc::now().to_rfc3339())
        .bind(id.to_string())
        .bind(claim_token.to_string())
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
             SET status='pending', attempts=0, next_attempt_at=?, last_error=NULL, \
                 claim_token=NULL, lease_expires_at=NULL, updated_at=? \
             WHERE id=? AND status='dead_letter' RETURNING *",
        )
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|error| AppError::DatabaseError(error.to_string()))?
        .ok_or_else(|| {
            AppError::ConflictError("outbox event is not in dead-letter state".to_string())
        })?;
        row_to_event(&row)
    }
}
