//! HTTP proxy provider for MCP tool execution.
//!
//! Implements `ExecutionProvider` for `HttpProxy` payloads by forwarding
//! requests via reqwest.

use std::time::{Duration, Instant};

use async_trait::async_trait;
use common::error::{AppError, Result};
use common::execution::{
    ExecutionPayload, ExecutionProvider, ExecutionRequest, ExecutionResponse,
};
use futures_util::StreamExt;
use reqwest::Client;
use tracing::debug;

const MAX_RESPONSE_BYTES: usize = 1_048_576;

pub struct HttpProxyProvider {
    client: Client,
}

impl Default for HttpProxyProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpProxyProvider {
    pub fn new() -> Self {
        let client = Client::builder()
            .no_proxy()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("static HTTP client configuration must be valid");

        Self { client }
    }

    pub fn with_timeout(timeout: Duration) -> Self {
        let client = Client::builder()
            .no_proxy()
            .timeout(timeout)
            .build()
            .expect("static HTTP client configuration must be valid");

        Self { client }
    }
}

#[async_trait]
impl ExecutionProvider for HttpProxyProvider {
    async fn execute(&self, request: ExecutionRequest) -> Result<ExecutionResponse> {
        let start = Instant::now();

        let (url, method) = match &request.payload {
            ExecutionPayload::HttpProxy { url, method } => (url.clone(), method.clone()),
            _ => {
                return Err(AppError::ValidationError(
                    "HttpProxyProvider only handles HttpProxy payloads".to_string(),
                ))
            }
        };

        let timeout = Duration::from_secs(request.constraints.timeout_seconds as u64);
        let method = parse_method(&method)?;

        let response = self
            .client
            .request(method, &url)
            .timeout(timeout)
            .json(&request.input)
            .send()
            .await
            .map_err(|e| map_reqwest_error(e, &url))?;

        let status = response.status().as_u16();
        let is_success = response.status().is_success();
        let mut body_bytes = Vec::new();
        let mut body_stream = response.bytes_stream();
        while let Some(chunk) = body_stream.next().await {
            let chunk = chunk.map_err(|e| AppError::ExternalServiceError {
                service: "http-proxy".to_string(),
                message: format!("Failed to read response body: {}", e),
            })?;

            if body_bytes.len() + chunk.len() > MAX_RESPONSE_BYTES {
                return Err(AppError::ExternalServiceError {
                    service: "http-proxy".to_string(),
                    message: format!(
                        "Response body exceeded {} byte limit for {}",
                        MAX_RESPONSE_BYTES, url
                    ),
                });
            }

            body_bytes.extend_from_slice(&chunk);
        }

        let body_text = String::from_utf8_lossy(&body_bytes).into_owned();
        let elapsed = start.elapsed().as_millis() as u64;

        let output = if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body_text) {
            json
        } else {
            serde_json::Value::String(body_text.clone())
        };

        let stderr = if !is_success {
            format!(
                "HTTP {} from {}: {}",
                status,
                url,
                body_text.chars().take(500).collect::<String>()
            )
        } else {
            String::new()
        };

        debug!("HTTP proxy executed: {} -> {}", url, status);

        Ok(ExecutionResponse {
            output,
            stdout: body_text,
            stderr,
            exit_code: if is_success { 0 } else { -1 },
            execution_time_ms: elapsed,
            timed_out: false,
            http_status: Some(status),
        })
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
            service: "http-proxy".to_string(),
            message: format!("Request to {} timed out", url),
        }
    } else if err.is_connect() {
        AppError::ExternalServiceError {
            service: "http-proxy".to_string(),
            message: format!("Failed to connect to {}: {}", url, err),
        }
    } else if err.is_request() {
        AppError::ValidationError(format!("Invalid request to {}: {}", url, err))
    } else {
        AppError::ExternalServiceError {
            service: "http-proxy".to_string(),
            message: format!("Request to {} failed: {}", url, err),
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use common::execution::{ExecutionCaller, ExecutionConstraints, ExecutionContext};

    #[test]
    fn parse_method_valid() {
        assert_eq!(parse_method("GET").unwrap(), reqwest::Method::GET);
        assert_eq!(parse_method("post").unwrap(), reqwest::Method::POST);
        assert_eq!(parse_method("Put").unwrap(), reqwest::Method::PUT);
        assert_eq!(parse_method("delete").unwrap(), reqwest::Method::DELETE);
        assert_eq!(parse_method("PATCH").unwrap(), reqwest::Method::PATCH);
    }

    #[test]
    fn parse_method_invalid() {
        let result = parse_method("INVALID");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Unsupported HTTP method"));
    }

    #[test]
    fn http_proxy_provider_default() {
        let _provider = HttpProxyProvider::new();
    }

    #[test]
    fn http_proxy_provider_with_timeout() {
        let _provider = HttpProxyProvider::with_timeout(Duration::from_secs(10));
    }

    #[tokio::test]
    async fn http_proxy_rejects_non_http_payload() {
        let provider = HttpProxyProvider::new();
        let request = ExecutionRequest {
            caller: ExecutionCaller::Skill {
                skill_id: uuid::Uuid::new_v4(),
                runtime: "python311".to_string(),
            },
            payload: ExecutionPayload::Code {
                source: "print('hi')".to_string(),
                language: "python311".to_string(),
            },
            constraints: ExecutionConstraints::default(),
            input: serde_json::json!({}),
            context: ExecutionContext::default(),
        };
        let err = provider.execute(request).await.unwrap_err();
        assert!(err.to_string().contains("HttpProxyProvider only handles HttpProxy payloads"));
    }

    #[tokio::test]
    async fn http_proxy_connection_error_returns_error() {
        let provider = HttpProxyProvider::new();
        let request = ExecutionRequest {
            caller: ExecutionCaller::McpTool {
                tool_id: uuid::Uuid::new_v4(),
            },
            payload: ExecutionPayload::HttpProxy {
                url: "http://localhost:19999/nonexistent".to_string(),
                method: "GET".to_string(),
            },
            constraints: ExecutionConstraints::default(),
            input: serde_json::json!({}),
            context: ExecutionContext::default(),
        };
        let err = provider.execute(request).await.unwrap_err();
        assert!(
            err.to_string().contains("connect")
                || err.to_string().contains("Failed")
                || err.to_string().contains("timed out")
        );
    }
}
