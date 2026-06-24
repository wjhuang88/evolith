//! Skill executor

// SkillExecutorAdapter constructs deprecated ExecutionCaller::Skill and
// ExecutionPayload::Code variants. This entire module is a legacy facade that
// will be removed in EVO-111.
#![allow(deprecated)]

use std::sync::Arc;

use async_trait::async_trait;
use common::error::{AppError, Result};
use common::execution::{
    ExecutionCaller, ExecutionConstraints, ExecutionContext, ExecutionPayload, ExecutionProvider,
    ExecutionRequest, ExecutionResponse as UnifiedResponse,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

/// Facade adapter: implements `SkillExecutor` by delegating to `ExecutionProvider`.
pub struct SkillExecutorAdapter {
    provider: Arc<dyn ExecutionProvider>,
}

impl SkillExecutorAdapter {
    pub fn new(provider: Arc<dyn ExecutionProvider>) -> Self {
        Self { provider }
    }
}

#[async_trait]
impl SkillExecutor for SkillExecutorAdapter {
    async fn execute(&self, request: ExecuteRequest) -> Result<ExecuteResponse> {
        let skill_id = Uuid::parse_str(&request.skill_id).unwrap_or_else(|_| Uuid::new_v4());

        let unified_request = ExecutionRequest {
            caller: ExecutionCaller::Skill {
                skill_id,
                runtime: request.language.clone(),
            },
            payload: ExecutionPayload::Code {
                source: request.code,
                language: request.language,
            },
            constraints: ExecutionConstraints::default(),
            input: request.parameters,
            context: ExecutionContext::default(),
        };

        let unified_response = self.provider.execute(unified_request).await?;
        Ok(to_skill_response(unified_response))
    }
}

fn to_skill_response(resp: UnifiedResponse) -> ExecuteResponse {
    ExecuteResponse {
        result: resp.output,
        stdout: resp.stdout,
        stderr: resp.stderr,
        exit_code: resp.exit_code,
        execution_time_ms: resp.execution_time_ms,
        timed_out: resp.timed_out,
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

    #[tokio::test]
    async fn skill_adapter_converts_request_and_response() {
        use std::sync::Arc;

        struct MockProvider;
        #[async_trait]
        impl ExecutionProvider for MockProvider {
            async fn execute(&self, request: ExecutionRequest) -> Result<UnifiedResponse> {
                assert!(matches!(request.caller, ExecutionCaller::Skill { .. }));
                assert!(matches!(request.payload, ExecutionPayload::Code { .. }));
                Ok(UnifiedResponse {
                    output: serde_json::json!({"ok": true}),
                    stdout: "hello".to_string(),
                    stderr: String::new(),
                    exit_code: 0,
                    execution_time_ms: 10,
                    timed_out: false,
                    http_status: None,
                })
            }
        }

        let adapter = SkillExecutorAdapter::new(Arc::new(MockProvider));
        let request = ExecuteRequest {
            skill_id: Uuid::new_v4().to_string(),
            code: "print('hi')".to_string(),
            language: "python311".to_string(),
            parameters: serde_json::json!({"key": "value"}),
        };

        let resp = adapter.execute(request).await.unwrap();
        assert_eq!(resp.stdout, "hello");
        assert_eq!(resp.exit_code, 0);
        assert!(!resp.timed_out);
        assert_eq!(resp.result["ok"], true);
    }

    #[tokio::test]
    async fn skill_adapter_propagates_errors() {
        use std::sync::Arc;

        struct ErrProvider;
        #[async_trait]
        impl ExecutionProvider for ErrProvider {
            async fn execute(&self, _request: ExecutionRequest) -> Result<UnifiedResponse> {
                Err(AppError::ConfigError("provider failed".to_string()))
            }
        }

        let adapter = SkillExecutorAdapter::new(Arc::new(ErrProvider));
        let request = ExecuteRequest {
            skill_id: Uuid::new_v4().to_string(),
            code: "x".to_string(),
            language: "python311".to_string(),
            parameters: serde_json::json!({}),
        };

        let err = adapter.execute(request).await.unwrap_err();
        assert!(err.to_string().contains("provider failed"));
    }
}
