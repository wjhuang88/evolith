use crate::repository::OutboxRepository;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use common::error::{AppError, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::future::Future;
use std::time::Duration;
use tracing::{info, warn};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum OutboxStatus {
    Pending,
    Processing,
    Delivered,
    Failed,
    DeadLetter,
}

impl OutboxStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Processing => "processing",
            Self::Delivered => "delivered",
            Self::Failed => "failed",
            Self::DeadLetter => "dead_letter",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OutboxEvent {
    pub id: Uuid,
    pub event_type: String,
    pub aggregate_type: String,
    pub aggregate_id: String,
    pub payload: Value,
    pub idempotency_key: String,
    pub status: OutboxStatus,
    pub attempts: u32,
    pub max_attempts: u32,
    pub next_attempt_at: DateTime<Utc>,
    pub last_error: Option<String>,
    pub claim_token: Option<Uuid>,
    pub lease_expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct NewOutboxEvent {
    pub event_type: String,
    pub aggregate_type: String,
    pub aggregate_id: String,
    pub payload: Value,
    pub idempotency_key: String,
    pub max_attempts: u32,
}

#[async_trait]
pub trait OutboxHandler: Send + Sync {
    async fn deliver(&self, event: &OutboxEvent) -> Result<()>;
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct OutboxBatchResult {
    pub claimed: usize,
    pub delivered: usize,
    pub failed: usize,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct OutboxRunStats {
    pub batches: usize,
    pub claimed: usize,
    pub delivered: usize,
    pub failed: usize,
}

pub struct OutboxWorker<R, H> {
    repository: R,
    handler: H,
    batch_size: u32,
    base_backoff_seconds: i64,
    lease_duration_seconds: i64,
    delivery_timeout: Duration,
}

impl<R, H> OutboxWorker<R, H>
where
    R: OutboxRepository,
    H: OutboxHandler,
{
    pub fn new(
        repository: R,
        handler: H,
        batch_size: u32,
        base_backoff_seconds: u64,
        lease_duration_seconds: u64,
        delivery_timeout_seconds: u64,
    ) -> Result<Self> {
        let batch_timeout = u64::from(batch_size)
            .checked_mul(delivery_timeout_seconds)
            .ok_or_else(|| AppError::ConfigError("outbox batch timeout overflow".to_string()))?;
        if batch_size == 0
            || base_backoff_seconds == 0
            || base_backoff_seconds > 3_600
            || lease_duration_seconds == 0
            || lease_duration_seconds > 86_400
            || delivery_timeout_seconds == 0
            || delivery_timeout_seconds > 3_600
            || batch_timeout >= lease_duration_seconds
        {
            return Err(AppError::ConfigError(
                "invalid outbox worker bounds or lease budget".to_string(),
            ));
        }
        Ok(Self {
            repository,
            handler,
            batch_size,
            base_backoff_seconds: base_backoff_seconds as i64,
            lease_duration_seconds: lease_duration_seconds as i64,
            delivery_timeout: Duration::from_secs(delivery_timeout_seconds),
        })
    }

    pub async fn process_once(&self) -> Result<OutboxBatchResult> {
        let claimed_at = Utc::now();
        let events = self
            .repository
            .claim_due(
                self.batch_size,
                claimed_at,
                chrono::Duration::seconds(self.lease_duration_seconds),
            )
            .await?;
        let mut result = OutboxBatchResult {
            claimed: events.len(),
            ..OutboxBatchResult::default()
        };
        for event in events {
            let claim_token = event.claim_token.ok_or_else(|| {
                AppError::DatabaseError("claimed outbox event is missing claim token".to_string())
            })?;
            let delivery =
                tokio::time::timeout(self.delivery_timeout, self.handler.deliver(&event)).await;
            match delivery {
                Ok(Ok(())) => {
                    self.repository
                        .mark_delivered(event.id, claim_token)
                        .await?;
                    result.delivered += 1;
                    info!(event_id = %event.id, event_type = %event.event_type, "outbox event delivered");
                }
                outcome => {
                    let error_code = match outcome {
                        Ok(Err(error)) => error.error_code_info().code,
                        Err(_) => "DELIVERY_TIMEOUT".to_string(),
                        Ok(Ok(())) => unreachable!(),
                    };
                    let exponent = event.attempts.saturating_sub(1).min(10);
                    let delay = self
                        .base_backoff_seconds
                        .saturating_mul(2_i64.saturating_pow(exponent));
                    self.repository
                        .mark_failed(
                            event.id,
                            claim_token,
                            &error_code,
                            Utc::now() + chrono::Duration::seconds(delay),
                        )
                        .await?;
                    result.failed += 1;
                    warn!(
                        event_id = %event.id,
                        event_type = %event.event_type,
                        attempts = event.attempts,
                        error_code = %error_code,
                        "outbox delivery failed"
                    );
                }
            }
        }
        Ok(result)
    }
}

pub struct OutboxRuntime<R, H> {
    worker: OutboxWorker<R, H>,
    poll_interval: Duration,
}

impl<R, H> OutboxRuntime<R, H>
where
    R: OutboxRepository,
    H: OutboxHandler,
{
    pub fn new(worker: OutboxWorker<R, H>, poll_interval_ms: u64) -> Result<Self> {
        if poll_interval_ms == 0 {
            return Err(AppError::ConfigError(
                "outbox poll interval must be positive".to_string(),
            ));
        }
        Ok(Self {
            worker,
            poll_interval: Duration::from_millis(poll_interval_ms),
        })
    }

    pub async fn run_until<F>(&self, shutdown: F) -> Result<OutboxRunStats>
    where
        F: Future<Output = ()>,
    {
        tokio::pin!(shutdown);
        let mut stats = OutboxRunStats::default();
        loop {
            tokio::select! {
                biased;
                _ = &mut shutdown => return Ok(stats),
                _ = tokio::task::yield_now() => {}
            }
            // Once a batch starts, let it finish. The constructor guarantees a bounded
            // delivery budget inside the lease, so shutdown never abandons claimed work.
            let batch = self.worker.process_once().await?;
            stats.batches += 1;
            stats.claimed += batch.claimed;
            stats.delivered += batch.delivered;
            stats.failed += batch.failed;
            if batch.claimed == 0 {
                tokio::select! {
                    _ = &mut shutdown => return Ok(stats),
                    _ = tokio::time::sleep(self.poll_interval) => {}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration as ChronoDuration;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, Mutex};

    #[derive(Clone)]
    struct RecordingRepository {
        event: OutboxEvent,
        claimed: Arc<AtomicBool>,
        failures: Arc<Mutex<Vec<String>>>,
    }

    #[async_trait]
    impl OutboxRepository for RecordingRepository {
        async fn enqueue(&self, _event: NewOutboxEvent) -> Result<OutboxEvent> {
            Err(AppError::InternalError("unused test operation".to_string()))
        }

        async fn enqueue_idempotent(&self, _event: NewOutboxEvent) -> Result<OutboxEvent> {
            Err(AppError::InternalError("unused test operation".to_string()))
        }

        async fn claim_due(
            &self,
            _limit: u32,
            _now: DateTime<Utc>,
            _lease_duration: ChronoDuration,
        ) -> Result<Vec<OutboxEvent>> {
            if self.claimed.swap(true, Ordering::SeqCst) {
                Ok(Vec::new())
            } else {
                Ok(vec![self.event.clone()])
            }
        }

        async fn mark_delivered(&self, _id: Uuid, _claim_token: Uuid) -> Result<()> {
            Ok(())
        }

        async fn mark_failed(
            &self,
            _id: Uuid,
            _claim_token: Uuid,
            error: &str,
            _next_attempt_at: DateTime<Utc>,
        ) -> Result<()> {
            self.failures
                .lock()
                .expect("failure recorder lock")
                .push(error.to_string());
            Ok(())
        }

        async fn replay_dead_letter(&self, _id: Uuid, _now: DateTime<Utc>) -> Result<OutboxEvent> {
            Err(AppError::InternalError("unused test operation".to_string()))
        }
    }

    struct ErrorHandler;

    #[async_trait]
    impl OutboxHandler for ErrorHandler {
        async fn deliver(&self, _event: &OutboxEvent) -> Result<()> {
            Err(AppError::ExternalServiceError {
                service: "secret-service-name".to_string(),
                message: "secret raw handler failure".to_string(),
            })
        }
    }

    struct SlowHandler;

    #[async_trait]
    impl OutboxHandler for SlowHandler {
        async fn deliver(&self, _event: &OutboxEvent) -> Result<()> {
            tokio::time::sleep(Duration::from_secs(2)).await;
            Ok(())
        }
    }

    fn repository() -> (RecordingRepository, Arc<Mutex<Vec<String>>>) {
        let now = Utc::now();
        let failures = Arc::new(Mutex::new(Vec::new()));
        (
            RecordingRepository {
                event: OutboxEvent {
                    id: Uuid::new_v4(),
                    event_type: "test.event".to_string(),
                    aggregate_type: "test".to_string(),
                    aggregate_id: "aggregate".to_string(),
                    payload: serde_json::json!({"secret":"payload secret"}),
                    idempotency_key: "test-event-1".to_string(),
                    status: OutboxStatus::Processing,
                    attempts: 1,
                    max_attempts: 3,
                    next_attempt_at: now,
                    last_error: None,
                    claim_token: Some(Uuid::new_v4()),
                    lease_expires_at: Some(now + ChronoDuration::seconds(10)),
                    created_at: now,
                    updated_at: now,
                },
                claimed: Arc::new(AtomicBool::new(false)),
                failures: failures.clone(),
            },
            failures,
        )
    }

    #[tokio::test]
    async fn handler_error_is_reduced_to_stable_code() {
        let (repository, failures) = repository();
        let worker =
            OutboxWorker::new(repository, ErrorHandler, 1, 1, 10, 1).expect("valid worker");
        let result = worker
            .process_once()
            .await
            .expect("process failed delivery");
        assert_eq!(result.failed, 1);
        assert_eq!(
            failures.lock().expect("failure recorder lock").as_slice(),
            ["EXTERNAL_SERVICE_ERROR"]
        );
    }

    #[tokio::test]
    async fn handler_exceeding_delivery_timeout_is_failed_safely() {
        let (repository, failures) = repository();
        let worker = OutboxWorker::new(repository, SlowHandler, 1, 1, 10, 1).expect("valid worker");
        let result = worker.process_once().await.expect("process timeout");
        assert_eq!(result.failed, 1);
        assert_eq!(
            failures.lock().expect("failure recorder lock").as_slice(),
            ["DELIVERY_TIMEOUT"]
        );
    }

    #[test]
    fn worker_rejects_batch_budget_at_or_beyond_lease() {
        let (repository, _) = repository();
        assert!(OutboxWorker::new(repository, ErrorHandler, 10, 1, 10, 1).is_err());
    }
}
