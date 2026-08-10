use chrono::{DateTime, Utc};
use common::error::{AppError, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{NewOutboxEvent, OutboxEvent};

pub const REPO_PUSH_COMPLETED_EVENT_TYPE: &str = "repo.push.completed.v1";
pub const REPO_PUSH_AGGREGATE_TYPE: &str = "git_repo";
pub const REPO_PUSH_MAX_ATTEMPTS: u32 = 8;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RepoPushCompletedPayload {
    pub tenant_id: Uuid,
    pub repo_id: Uuid,
    pub default_branch: String,
    pub commit_sha: String,
    pub committed_at: DateTime<Utc>,
}

impl RepoPushCompletedPayload {
    pub fn new(
        tenant_id: Uuid,
        repo_id: Uuid,
        default_branch: String,
        commit_sha: String,
        committed_at: DateTime<Utc>,
    ) -> Result<Self> {
        if default_branch.is_empty()
            || default_branch.len() > 255
            || default_branch.chars().any(char::is_control)
            || commit_sha.len() != 40
            || !commit_sha.bytes().all(|byte| byte.is_ascii_hexdigit())
        {
            return Err(AppError::ValidationError(
                "invalid repo push event facts".to_string(),
            ));
        }
        Ok(Self {
            tenant_id,
            repo_id,
            default_branch,
            commit_sha: commit_sha.to_ascii_lowercase(),
            committed_at,
        })
    }

    pub fn idempotency_key(&self) -> String {
        let name = format!("{}\0{}", self.default_branch, self.commit_sha);
        format!("repo-push:{}", Uuid::new_v5(&self.repo_id, name.as_bytes()))
    }

    pub fn into_outbox_event(self) -> Result<NewOutboxEvent> {
        let idempotency_key = self.idempotency_key();
        let aggregate_id = self.repo_id.to_string();
        let payload = serde_json::to_value(self)
            .map_err(|error| AppError::ValidationError(error.to_string()))?;
        Ok(NewOutboxEvent {
            event_type: REPO_PUSH_COMPLETED_EVENT_TYPE.to_string(),
            aggregate_type: REPO_PUSH_AGGREGATE_TYPE.to_string(),
            aggregate_id,
            payload,
            idempotency_key,
            max_attempts: REPO_PUSH_MAX_ATTEMPTS,
        })
    }

    pub fn from_outbox_event(event: &OutboxEvent) -> Result<Self> {
        if event.event_type != REPO_PUSH_COMPLETED_EVENT_TYPE
            || event.aggregate_type != REPO_PUSH_AGGREGATE_TYPE
            || event.max_attempts != REPO_PUSH_MAX_ATTEMPTS
        {
            return Err(AppError::ValidationError(
                "unsupported repo push event contract".to_string(),
            ));
        }
        let payload: Self = serde_json::from_value(event.payload.clone())
            .map_err(|_| AppError::ValidationError("invalid repo push payload".to_string()))?;
        let validated = Self::new(
            payload.tenant_id,
            payload.repo_id,
            payload.default_branch,
            payload.commit_sha,
            payload.committed_at,
        )?;
        if event.aggregate_id != validated.repo_id.to_string()
            || event.idempotency_key != validated.idempotency_key()
        {
            return Err(AppError::ValidationError(
                "repo push event identity mismatch".to_string(),
            ));
        }
        Ok(validated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::OutboxStatus;

    fn payload() -> RepoPushCompletedPayload {
        RepoPushCompletedPayload::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            "main".to_string(),
            "a".repeat(40),
            Utc::now(),
        )
        .expect("valid payload")
    }

    #[test]
    fn idempotency_key_is_stable_and_payload_contains_no_credentials() {
        let payload = payload();
        assert_eq!(payload.idempotency_key(), payload.idempotency_key());
        let json = serde_json::to_string(&payload).expect("serialize payload");
        assert!(!json.contains("authorization"));
        assert!(!json.contains("credential"));
        assert!(!json.contains("pack"));
    }

    #[test]
    fn rejects_tampered_event_identity_and_unknown_payload_fields() {
        let payload = payload();
        let new_event = payload.clone().into_outbox_event().expect("new event");
        let mut event = OutboxEvent {
            id: Uuid::new_v4(),
            event_type: new_event.event_type,
            aggregate_type: new_event.aggregate_type,
            aggregate_id: new_event.aggregate_id,
            payload: new_event.payload,
            idempotency_key: new_event.idempotency_key,
            status: OutboxStatus::Pending,
            attempts: 0,
            max_attempts: REPO_PUSH_MAX_ATTEMPTS,
            next_attempt_at: Utc::now(),
            last_error: None,
            claim_token: None,
            lease_expires_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        assert_eq!(
            RepoPushCompletedPayload::from_outbox_event(&event).expect("valid event"),
            payload
        );
        event.payload["authorization"] = serde_json::json!("secret");
        assert!(RepoPushCompletedPayload::from_outbox_event(&event).is_err());
        event
            .payload
            .as_object_mut()
            .expect("object")
            .remove("authorization");
        event.aggregate_id = Uuid::new_v4().to_string();
        assert!(RepoPushCompletedPayload::from_outbox_event(&event).is_err());
        event.aggregate_id = payload.repo_id.to_string();
        event.max_attempts = 1;
        assert!(RepoPushCompletedPayload::from_outbox_event(&event).is_err());
    }
}
