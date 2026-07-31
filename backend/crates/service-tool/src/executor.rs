//! Tool executor facades.

use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use common::error::Result;
use common::execution::{
    ExecutionCaller, ExecutionConstraints, ExecutionContext, ExecutionPayload, ExecutionProvider,
    ExecutionRequest, ExecutionResponse as UnifiedResponse,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::egress::EgressPolicy;
use crate::http_proxy_provider::HttpProxyProvider;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecuteRequest {
    pub tool_id: String,
    pub parameters: serde_json::Value,
    pub url: String,
    pub method: String,
    pub timeout_ms: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecuteResponse {
    pub result: serde_json::Value,
    pub execution_time_ms: u64,
    pub status: u16,
    pub error: Option<String>,
}

#[async_trait]
pub trait ToolExecutor: Send + Sync {
    async fn execute(&self, request: ExecuteRequest) -> Result<ExecuteResponse>;
}

/// Legacy direct facade retained for compatibility. It delegates to the same
/// controlled HTTP provider used by the production execution path.
pub struct HttpToolExecutor {
    provider: HttpProxyProvider,
    default_timeout: Duration,
}

impl Default for HttpToolExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpToolExecutor {
    pub fn new() -> Self {
        Self::with_policy_and_timeout(default_constructor_policy(), Duration::from_secs(30))
    }

    pub fn with_timeout(timeout: Duration) -> Self {
        Self::with_policy_and_timeout(default_constructor_policy(), timeout)
    }

    pub fn with_policy(policy: EgressPolicy) -> Self {
        Self::with_policy_and_timeout(policy, Duration::from_secs(30))
    }

    pub fn with_policy_and_timeout(policy: EgressPolicy, timeout: Duration) -> Self {
        Self {
            provider: HttpProxyProvider::with_policy_and_timeout(policy, timeout),
            default_timeout: timeout,
        }
    }
}

fn default_constructor_policy() -> EgressPolicy {
    #[cfg(feature = "test-egress")]
    {
        EgressPolicy::for_test_allow_private_networks()
    }
    #[cfg(not(feature = "test-egress"))]
    {
        EgressPolicy::default()
    }
}

#[async_trait]
impl ToolExecutor for HttpToolExecutor {
    async fn execute(&self, request: ExecuteRequest) -> Result<ExecuteResponse> {
        let unified_request = to_unified_request(request, self.default_timeout);
        self.provider
            .execute(unified_request)
            .await
            .map(to_tool_response)
    }
}

/// Facade adapter that delegates to the configured unified execution provider.
pub struct ToolExecutorAdapter {
    provider: Arc<dyn ExecutionProvider>,
}

impl ToolExecutorAdapter {
    pub fn new(provider: Arc<dyn ExecutionProvider>) -> Self {
        Self { provider }
    }
}

#[async_trait]
impl ToolExecutor for ToolExecutorAdapter {
    async fn execute(&self, request: ExecuteRequest) -> Result<ExecuteResponse> {
        let unified_request = to_unified_request(request, Duration::from_secs(30));
        self.provider
            .execute(unified_request)
            .await
            .map(to_tool_response)
    }
}

fn to_unified_request(request: ExecuteRequest, default_timeout: Duration) -> ExecutionRequest {
    let tool_id = Uuid::parse_str(&request.tool_id).unwrap_or_else(|_| Uuid::new_v4());
    let timeout_ms = if request.timeout_ms > 0 {
        request.timeout_ms as u64
    } else {
        default_timeout.as_millis().min(u32::MAX as u128) as u64
    };
    let timeout_seconds = timeout_ms.div_ceil(1_000).max(1).min(u32::MAX as u64) as u32;

    ExecutionRequest {
        caller: ExecutionCaller::McpTool { tool_id },
        payload: ExecutionPayload::HttpProxy {
            url: request.url,
            method: request.method,
        },
        constraints: ExecutionConstraints {
            timeout_seconds,
            network_enabled: true,
            ..Default::default()
        },
        input: request.parameters,
        context: ExecutionContext::default(),
    }
}

fn to_tool_response(response: UnifiedResponse) -> ExecuteResponse {
    let error = if response.exit_code != 0 || !response.stderr.is_empty() {
        Some(response.stderr.clone())
    } else {
        None
    };

    ExecuteResponse {
        result: response.output,
        execution_time_ms: response.execution_time_ms,
        status: response.http_status.unwrap_or(0),
        error,
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use common::error::AppError;

    #[test]
    fn direct_executor_uses_positive_default_timeout() {
        let executor = HttpToolExecutor::new();
        assert!(executor.default_timeout.as_secs() > 0);
    }

    #[tokio::test]
    async fn direct_executor_rejects_loopback_through_shared_policy() {
        let executor = HttpToolExecutor::with_policy(EgressPolicy::default());
        let error = executor
            .execute(ExecuteRequest {
                tool_id: Uuid::new_v4().to_string(),
                parameters: serde_json::json!({}),
                url: "http://127.0.0.1:19999".to_string(),
                method: "GET".to_string(),
                timeout_ms: 1_000,
            })
            .await
            .unwrap_err();
        assert!(error.to_string().contains("target is not allowed"));
    }

    #[tokio::test]
    async fn adapter_converts_request_and_response() {
        struct MockProvider;
        #[async_trait]
        impl ExecutionProvider for MockProvider {
            async fn execute(&self, request: ExecutionRequest) -> Result<UnifiedResponse> {
                assert!(matches!(request.caller, ExecutionCaller::McpTool { .. }));
                assert_eq!(request.constraints.timeout_seconds, 5);
                Ok(UnifiedResponse {
                    output: serde_json::json!({"status": "ok"}),
                    stdout: "{\"status\":\"ok\"}".to_string(),
                    stderr: String::new(),
                    exit_code: 0,
                    execution_time_ms: 25,
                    timed_out: false,
                    http_status: Some(200),
                })
            }
        }

        let adapter = ToolExecutorAdapter::new(Arc::new(MockProvider));
        let response = adapter
            .execute(ExecuteRequest {
                tool_id: Uuid::new_v4().to_string(),
                parameters: serde_json::json!({"key": "value"}),
                url: "https://example.com/api".to_string(),
                method: "POST".to_string(),
                timeout_ms: 5_000,
            })
            .await
            .unwrap();
        assert_eq!(response.status, 200);
        assert!(response.error.is_none());
    }

    #[tokio::test]
    async fn adapter_propagates_errors() {
        struct ErrorProvider;
        #[async_trait]
        impl ExecutionProvider for ErrorProvider {
            async fn execute(&self, _request: ExecutionRequest) -> Result<UnifiedResponse> {
                Err(AppError::ExternalServiceError {
                    service: "test".to_string(),
                    message: "request failed".to_string(),
                })
            }
        }

        let adapter = ToolExecutorAdapter::new(Arc::new(ErrorProvider));
        let error = adapter
            .execute(ExecuteRequest {
                tool_id: Uuid::new_v4().to_string(),
                parameters: serde_json::json!({}),
                url: "https://example.com".to_string(),
                method: "GET".to_string(),
                timeout_ms: 1_000,
            })
            .await
            .unwrap_err();
        assert!(error.to_string().contains("request failed"));
    }
}
