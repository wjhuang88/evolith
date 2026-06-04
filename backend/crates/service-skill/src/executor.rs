//! Skill executor

use async_trait::async_trait;
use common::error::{AppError, Result};
use serde::{Deserialize, Serialize};

/// Skill execution request
#[derive(Debug, Serialize, Deserialize)]
pub struct ExecuteRequest {
    pub skill_id: String,
    pub code: String,
    pub language: String,
    pub parameters: serde_json::Value,
}

/// Skill execution response
#[derive(Debug, Serialize, Deserialize)]
pub struct ExecuteResponse {
    pub result: serde_json::Value,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i64,
    pub execution_time_ms: u64,
    pub timed_out: bool,
}

/// Skill executor trait
#[async_trait]
pub trait SkillExecutor: Send + Sync {
    async fn execute(&self, request: ExecuteRequest) -> Result<ExecuteResponse>;
}

/// Default skill executor implementation
pub struct DefaultSkillExecutor;

impl Default for DefaultSkillExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultSkillExecutor {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl SkillExecutor for DefaultSkillExecutor {
    async fn execute(&self, _request: ExecuteRequest) -> Result<ExecuteResponse> {
        Err(AppError::ConfigError(
            "Skill execution is unavailable because the sandbox executor is disabled or not initialized"
                .to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn default_executor_returns_explicit_error() {
        let executor = DefaultSkillExecutor::new();
        let request = ExecuteRequest {
            skill_id: "skill-1".to_string(),
            code: "print('hello')".to_string(),
            language: "python311".to_string(),
            parameters: serde_json::json!({}),
        };

        let err = executor
            .execute(request)
            .await
            .expect_err("default executor must not return a success response");

        assert!(matches!(err, AppError::ConfigError(_)));
        assert!(err.to_string().contains("Skill execution is unavailable"));
    }
}
