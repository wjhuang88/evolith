//! Tool executor

use async_trait::async_trait;
use common::error::{AppError, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

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
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap_or_default();

        Self {
            client,
            default_timeout: Duration::from_secs(30),
        }
    }

    /// Create a new HttpToolExecutor with a custom default timeout
    pub fn with_timeout(timeout: Duration) -> Self {
        let client = Client::builder()
            .timeout(timeout)
            .build()
            .unwrap_or_default();

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
        let body_bytes = response
            .bytes()
            .await
            .map_err(|e| AppError::ExternalServiceError {
                service: "http-tool".to_string(),
                message: format!("Failed to read response body: {}", e),
            })?;

        // Truncate response body if larger than 1MB
        let truncated = if body_bytes.len() > MAX_RESPONSE_BYTES {
            &body_bytes[..MAX_RESPONSE_BYTES]
        } else {
            &body_bytes[..]
        };

        let body_text = String::from_utf8_lossy(truncated).into_owned();
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
}
