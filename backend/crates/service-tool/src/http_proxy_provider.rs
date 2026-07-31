//! HTTP proxy provider for MCP tool execution.
//!
//! All tenant-controlled requests are delegated to the shared egress boundary.

use std::time::{Duration, Instant};

use async_trait::async_trait;
use common::error::{AppError, Result};
use common::execution::{ExecutionPayload, ExecutionProvider, ExecutionRequest, ExecutionResponse};
use tracing::debug;

use crate::egress::{EgressError, EgressPolicy, SafeHttpClient};

pub struct HttpProxyProvider {
    client: SafeHttpClient,
    default_timeout: Duration,
}

impl Default for HttpProxyProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpProxyProvider {
    pub fn new() -> Self {
        Self::with_policy_and_timeout(EgressPolicy::default(), Duration::from_secs(30))
    }

    pub fn with_timeout(timeout: Duration) -> Self {
        Self::with_policy_and_timeout(EgressPolicy::default(), timeout)
    }

    pub fn with_policy(policy: EgressPolicy) -> Self {
        Self::with_policy_and_timeout(policy, Duration::from_secs(30))
    }

    pub fn with_policy_and_timeout(policy: EgressPolicy, timeout: Duration) -> Self {
        Self {
            client: SafeHttpClient::new(policy),
            default_timeout: timeout,
        }
    }

    pub fn with_client(client: SafeHttpClient, timeout: Duration) -> Self {
        Self {
            client,
            default_timeout: timeout,
        }
    }
}

#[async_trait]
impl ExecutionProvider for HttpProxyProvider {
    async fn execute(&self, request: ExecutionRequest) -> Result<ExecutionResponse> {
        let start = Instant::now();

        let (url, method) = match &request.payload {
            ExecutionPayload::HttpProxy { url, method } => (url, method),
            _ => {
                return Err(AppError::ValidationError(
                    "HttpProxyProvider only handles HttpProxy payloads".to_string(),
                ))
            }
        };

        let timeout = if request.constraints.timeout_seconds > 0 {
            Duration::from_secs(request.constraints.timeout_seconds as u64)
        } else {
            self.default_timeout
        };
        let method = parse_method(method)?;
        let response = self
            .client
            .execute(method, url, &request.input, timeout)
            .await
            .map_err(to_app_error)?;

        let status = response.status.as_u16();
        let is_success = response.status.is_success();
        let body_text = String::from_utf8_lossy(&response.body).into_owned();
        let output = serde_json::from_str::<serde_json::Value>(&body_text)
            .unwrap_or_else(|_| serde_json::Value::String(body_text.clone()));

        debug!(
            tool_id = ?request.caller,
            http_status = status,
            "HTTP tool egress request completed"
        );

        Ok(ExecutionResponse {
            output,
            stdout: body_text,
            stderr: if is_success {
                String::new()
            } else {
                format!("HTTP {status} error")
            },
            exit_code: if is_success { 0 } else { -1 },
            execution_time_ms: start.elapsed().as_millis() as u64,
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
        _ => Err(AppError::ValidationError(
            "Unsupported HTTP method".to_string(),
        )),
    }
}

fn to_app_error(error: EgressError) -> AppError {
    match error {
        EgressError::InvalidTarget
        | EgressError::SchemeNotAllowed
        | EgressError::CredentialsNotAllowed
        | EgressError::TargetNotAllowed
        | EgressError::DnsResolutionFailed
        | EgressError::RedirectRejected
        | EgressError::TooManyRedirects => AppError::ValidationError(error.to_string()),
        EgressError::RequestTimedOut => AppError::ExternalServiceError {
            service: "http-proxy".to_string(),
            message: error.to_string(),
        },
        EgressError::ConcurrencyLimit
        | EgressError::RequestFailed
        | EgressError::ResponseHeadersTooLarge
        | EgressError::ResponseBodyTooLarge => AppError::ExternalServiceError {
            service: "http-proxy".to_string(),
            message: error.to_string(),
        },
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    #![allow(deprecated)]

    use super::*;
    use common::execution::{ExecutionCaller, ExecutionConstraints, ExecutionContext};

    #[test]
    fn parse_method_valid() {
        assert_eq!(parse_method("GET").unwrap(), reqwest::Method::GET);
        assert_eq!(parse_method("post").unwrap(), reqwest::Method::POST);
        assert_eq!(parse_method("Put").unwrap(), reqwest::Method::PUT);
    }

    #[test]
    fn parse_method_invalid_is_redacted() {
        let error = parse_method("SECRET-METHOD").unwrap_err();
        assert_eq!(
            error.to_string(),
            "Validation error: Unsupported HTTP method"
        );
    }

    #[tokio::test]
    async fn rejects_non_http_payload() {
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
        let error = provider.execute(request).await.unwrap_err();
        assert!(error
            .to_string()
            .contains("HttpProxyProvider only handles HttpProxy payloads"));
    }

    #[tokio::test]
    async fn rejects_loopback_before_connecting() {
        let provider = HttpProxyProvider::new();
        let request = ExecutionRequest {
            caller: ExecutionCaller::McpTool {
                tool_id: uuid::Uuid::new_v4(),
            },
            payload: ExecutionPayload::HttpProxy {
                url: "http://127.0.0.1:19999/nonexistent".to_string(),
                method: "GET".to_string(),
            },
            constraints: ExecutionConstraints::default(),
            input: serde_json::json!({}),
            context: ExecutionContext::default(),
        };
        let error = provider.execute(request).await.unwrap_err();
        assert!(error.to_string().contains("target is not allowed"));
    }
}
