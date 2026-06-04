//! Tool executor

use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use common::error::{AppError, Result};
use common::execution::{
    ExecutionCaller, ExecutionConstraints, ExecutionContext, ExecutionPayload, ExecutionProvider,
    ExecutionRequest, ExecutionResponse as UnifiedResponse,
};
use futures_util::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Maximum response body size (1MB)
const MAX_RESPONSE_BYTES: usize = 1_048_576;

/// Tool execution request
#[derive(Debug, Serialize, Deserialize)]
pub struct ExecuteRequest {
    pub tool_id: String,
    pub parameters: serde_json::Value,
    pub url: String,
    pub method: String,
    pub timeout_ms: u32,
}

/// Tool execution response
#[derive(Debug, Serialize, Deserialize)]
pub struct ExecuteResponse {
    pub result: serde_json::Value,
    pub execution_time_ms: u64,
    pub status: u16,
    pub error: Option<String>,
}

/// Tool executor trait
#[async_trait]
pub trait ToolExecutor: Send + Sync {
    async fn execute(&self, request: ExecuteRequest) -> Result<ExecuteResponse>;
}

/// HTTP tool executor that makes real HTTP requests
pub struct HttpToolExecutor {
    client: Client,
    default_timeout: Duration,
}

impl Default for HttpToolExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpToolExecutor {
    /// Create a new HttpToolExecutor with default settings
    pub fn new() -> Self {
        let client = Client::builder()
            // System proxy discovery can panic in restricted macOS runtimes. Proxy support
            // needs explicit configuration before it is safe for tool execution.
            .no_proxy()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("static HTTP client configuration must be valid");

        Self {
            client,
            default_timeout: Duration::from_secs(30),
        }
    }

    /// Create a new HttpToolExecutor with a custom default timeout
    pub fn with_timeout(timeout: Duration) -> Self {
        let client = Client::builder()
            .no_proxy()
            .timeout(timeout)
            .build()
            .expect("static HTTP client configuration must be valid");

        Self {
            client,
            default_timeout: timeout,
        }
    }
}

#[async_trait]
impl ToolExecutor for HttpToolExecutor {
    async fn execute(&self, request: ExecuteRequest) -> Result<ExecuteResponse> {
        let start = std::time::Instant::now();

        let timeout = if request.timeout_ms > 0 {
            Duration::from_millis(request.timeout_ms as u64)
        } else {
            self.default_timeout
        };

        let method = parse_method(&request.method)?;

        let response = self
            .client
            .request(method, &request.url)
            .timeout(timeout)
            .json(&request.parameters)
            .send()
            .await
            .map_err(|e| map_reqwest_error(e, &request.url))?;

        let status = response.status().as_u16();
        let is_success = response.status().is_success();
        let mut body_bytes = Vec::new();
        let mut body_stream = response.bytes_stream();
        while let Some(chunk) = body_stream.next().await {
            let chunk = chunk.map_err(|e| AppError::ExternalServiceError {
                service: "http-tool".to_string(),
                message: format!("Failed to read response body: {}", e),
            })?;

            if body_bytes.len() + chunk.len() > MAX_RESPONSE_BYTES {
                return Err(AppError::ExternalServiceError {
                    service: "http-tool".to_string(),
                    message: format!(
                        "Response body exceeded {} byte limit for {}",
                        MAX_RESPONSE_BYTES, request.url
                    ),
                });
            }

            body_bytes.extend_from_slice(&chunk);
        }

        let body_text = String::from_utf8_lossy(&body_bytes).into_owned();
        let execution_time_ms = start.elapsed().as_millis() as u64;

        // Try to parse as JSON, fall back to text
        let result = if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body_text) {
            json
        } else {
            serde_json::Value::String(body_text.clone())
        };

        let error = if !is_success {
            Some(format!(
                "HTTP {} from {}: {}",
                status,
                request.url,
                body_text.chars().take(500).collect::<String>()
            ))
        } else {
            None
        };

        Ok(ExecuteResponse {
            result,
            execution_time_ms,
            status,
            error,
        })
    }
}

/// Facade adapter: implements `ToolExecutor` by delegating to `ExecutionProvider`.
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
        let tool_id = Uuid::parse_str(&request.tool_id).unwrap_or_else(|_| Uuid::new_v4());

        let timeout_secs = if request.timeout_ms > 0 {
            (request.timeout_ms / 1000).max(1)
        } else {
            30
        };

        let constraints = ExecutionConstraints {
            timeout_seconds: timeout_secs,
            network_enabled: true,
            ..Default::default()
        };

        let unified_request = ExecutionRequest {
            caller: ExecutionCaller::McpTool { tool_id },
            payload: ExecutionPayload::HttpProxy {
                url: request.url,
                method: request.method,
            },
            constraints,
            input: request.parameters,
            context: ExecutionContext::default(),
        };

        let unified_response = self.provider.execute(unified_request).await?;
        Ok(to_tool_response(unified_response))
    }
}

fn to_tool_response(resp: UnifiedResponse) -> ExecuteResponse {
    let error = if resp.exit_code != 0 || !resp.stderr.is_empty() {
        Some(resp.stderr.clone())
    } else {
        None
    };

    ExecuteResponse {
        result: resp.output,
        execution_time_ms: resp.execution_time_ms,
        status: resp.http_status.unwrap_or(0),
        error,
    }
}

fn parse_method(method: &str) -> Result<reqwest::Method> {
    match method.to_uppercase().as_str() {
        "GET" => Ok(reqwest::Method::GET),
        "POST" => Ok(reqwest::Method::POST),
        "PUT" => Ok(reqwest::Method::PUT),
        "DELETE" => Ok(reqwest::Method::DELETE),
        "PATCH" => Ok(reqwest::Method::PATCH),
        "HEAD" => Ok(reqwest::Method::HEAD),
        "OPTIONS" => Ok(reqwest::Method::OPTIONS),
        _ => Err(AppError::ValidationError(format!(
            "Unsupported HTTP method: {}",
            method
        ))),
    }
}

fn map_reqwest_error(err: reqwest::Error, url: &str) -> AppError {
    if err.is_timeout() {
        AppError::ExternalServiceError {
            service: "http-tool".to_string(),
            message: format!("Request to {} timed out", url),
        }
    } else if err.is_connect() {
        AppError::ExternalServiceError {
            service: "http-tool".to_string(),
            message: format!("Failed to connect to {}: {}", url, err),
        }
    } else if err.is_request() {
        AppError::ValidationError(format!("Invalid request to {}: {}", url, err))
    } else {
        AppError::ExternalServiceError {
            service: "http-tool".to_string(),
            message: format!("Request to {} failed: {}", url, err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_method_valid() {
        assert_eq!(parse_method("GET").unwrap(), reqwest::Method::GET);
        assert_eq!(parse_method("post").unwrap(), reqwest::Method::POST);
        assert_eq!(parse_method("Put").unwrap(), reqwest::Method::PUT);
        assert_eq!(parse_method("delete").unwrap(), reqwest::Method::DELETE);
        assert_eq!(parse_method("PATCH").unwrap(), reqwest::Method::PATCH);
    }

    #[test]
    fn test_parse_method_invalid() {
        let result = parse_method("INVALID");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Unsupported HTTP method"));
    }

    #[test]
    fn test_http_executor_default() {
        let executor = HttpToolExecutor::new();
        assert!(executor.default_timeout.as_secs() > 0);
    }

    #[test]
    fn test_http_executor_with_timeout() {
        let timeout = Duration::from_secs(10);
        let executor = HttpToolExecutor::with_timeout(timeout);
        assert_eq!(executor.default_timeout, timeout);
    }

    #[tokio::test]
    async fn tool_adapter_converts_request_and_response() {
        use std::sync::Arc;

        struct MockProvider;
        #[async_trait]
        impl ExecutionProvider for MockProvider {
            async fn execute(&self, request: ExecutionRequest) -> Result<UnifiedResponse> {
                assert!(matches!(request.caller, ExecutionCaller::McpTool { .. }));
                assert!(matches!(request.payload, ExecutionPayload::HttpProxy { .. }));
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
        let request = ExecuteRequest {
            tool_id: Uuid::new_v4().to_string(),
            parameters: serde_json::json!({"key": "value"}),
            url: "http://example.com/api".to_string(),
            method: "POST".to_string(),
            timeout_ms: 5000,
        };

        let resp = adapter.execute(request).await.unwrap();
        assert_eq!(resp.status, 200);
        assert_eq!(resp.execution_time_ms, 25);
        assert!(resp.error.is_none());
        assert_eq!(resp.result["status"], "ok");
    }

    #[tokio::test]
    async fn tool_adapter_propagates_errors() {
        use std::sync::Arc;

        struct ErrProvider;
        #[async_trait]
        impl ExecutionProvider for ErrProvider {
            async fn execute(&self, _request: ExecutionRequest) -> Result<UnifiedResponse> {
                Err(AppError::ExternalServiceError {
                    service: "test".to_string(),
                    message: "connection refused".to_string(),
                })
            }
        }

        let adapter = ToolExecutorAdapter::new(Arc::new(ErrProvider));
        let request = ExecuteRequest {
            tool_id: Uuid::new_v4().to_string(),
            parameters: serde_json::json!({}),
            url: "http://localhost:9999".to_string(),
            method: "GET".to_string(),
            timeout_ms: 1000,
        };

        let err = adapter.execute(request).await.unwrap_err();
        assert!(err.to_string().contains("connection refused"));
    }

    #[tokio::test]
    async fn tool_adapter_maps_error_status() {
        use std::sync::Arc;

        struct ErrorProvider;
        #[async_trait]
        impl ExecutionProvider for ErrorProvider {
            async fn execute(&self, _request: ExecutionRequest) -> Result<UnifiedResponse> {
                Ok(UnifiedResponse {
                    output: serde_json::json!({"error": "bad"}),
                    stdout: String::new(),
                    stderr: "HTTP 500 error".to_string(),
                    exit_code: -1,
                    execution_time_ms: 50,
                    timed_out: false,
                    http_status: Some(500),
                })
            }
        }

        let adapter = ToolExecutorAdapter::new(Arc::new(ErrorProvider));
        let request = ExecuteRequest {
            tool_id: Uuid::new_v4().to_string(),
            parameters: serde_json::json!({}),
            url: "http://example.com".to_string(),
            method: "GET".to_string(),
            timeout_ms: 0,
        };

        let resp = adapter.execute(request).await.unwrap();
        assert_eq!(resp.status, 500);
        assert!(resp.error.is_some());
        assert!(resp.error.as_ref().unwrap().contains("HTTP 500 error"));
    }
}
