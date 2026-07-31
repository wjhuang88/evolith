//! Unified execution abstraction for Evolith.
//!
//! This module defines a single `ExecutionProvider` trait that all execution
//! backends (Docker sandbox, HTTP proxy, future serverless) implement.
//! Existing `SkillExecutor` and `ToolExecutor` traits are preserved as facade
//! adapters during migration (Phase 1 of SERVERLESS-RUNTIME.md).

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{AppError, Result};

// ---------------------------------------------------------------------------
// Request types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionCaller {
    #[deprecated(
        since = "0.8.0",
        note = "will be removed in EVO-111; sandbox execution deprecated per ADR-0005"
    )]
    Skill { skill_id: Uuid, runtime: String },
    #[deprecated(
        since = "0.8.0",
        note = "will be removed in EVO-111; sandbox execution deprecated per ADR-0005"
    )]
    Cli { snippet_id: Uuid, command: String },
    McpTool { tool_id: Uuid },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionPayload {
    #[deprecated(
        since = "0.8.0",
        note = "will be removed in EVO-111; sandbox execution deprecated per ADR-0005"
    )]
    Code { source: String, language: String },
    #[deprecated(
        since = "0.8.0",
        note = "will be removed in EVO-111; sandbox execution deprecated per ADR-0005"
    )]
    Command { command: String, args: Vec<String> },
    /// Forward to an external HTTP endpoint (MCP HTTP tool).
    /// `timeout_ms` preserves the existing millisecond Tool contract; the
    /// generic second-based execution constraint remains the fallback.
    HttpProxy {
        url: String,
        method: String,
        timeout_ms: Option<u32>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionConstraints {
    pub timeout_seconds: u32,
    pub memory_mb: u32,
    pub cpu_shares: i64,
    pub pids_limit: i64,
    pub network_enabled: bool,
    pub max_output_bytes: usize,
}

impl Default for ExecutionConstraints {
    fn default() -> Self {
        Self {
            timeout_seconds: 30,
            memory_mb: 256,
            cpu_shares: 512,
            pids_limit: 256,
            network_enabled: false,
            max_output_bytes: 10 * 1024 * 1024,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionContext {
    pub tenant_id: Uuid,
    pub user_id: Uuid,
    pub request_id: String,
}

impl Default for ExecutionContext {
    fn default() -> Self {
        Self {
            tenant_id: Uuid::nil(),
            user_id: Uuid::nil(),
            request_id: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionRequest {
    pub caller: ExecutionCaller,
    pub payload: ExecutionPayload,
    pub constraints: ExecutionConstraints,
    pub input: serde_json::Value,
    pub context: ExecutionContext,
}

// ---------------------------------------------------------------------------
// Response type
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResponse {
    pub output: serde_json::Value,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i64,
    pub execution_time_ms: u64,
    pub timed_out: bool,
    pub http_status: Option<u16>,
}

// ---------------------------------------------------------------------------
// Trait and composite provider
// ---------------------------------------------------------------------------

#[async_trait]
pub trait ExecutionProvider: Send + Sync {
    async fn execute(&self, request: ExecutionRequest) -> Result<ExecutionResponse>;
}

pub struct CompositeProvider {
    docker_sandbox: Option<std::sync::Arc<dyn ExecutionProvider>>,
    http_proxy: Option<std::sync::Arc<dyn ExecutionProvider>>,
}

impl CompositeProvider {
    pub fn new(
        docker_sandbox: Option<std::sync::Arc<dyn ExecutionProvider>>,
        http_proxy: Option<std::sync::Arc<dyn ExecutionProvider>>,
    ) -> Self {
        Self {
            docker_sandbox,
            http_proxy,
        }
    }

    #[allow(deprecated)]
    fn route(&self, request: &ExecutionRequest) -> Result<&dyn ExecutionProvider> {
        match &request.payload {
            ExecutionPayload::Code { .. } | ExecutionPayload::Command { .. } => self
                .docker_sandbox
                .as_ref()
                .map(|provider| provider.as_ref() as &dyn ExecutionProvider)
                .ok_or_else(|| {
                    AppError::ConfigError("Docker sandbox provider is not available".to_string())
                }),
            ExecutionPayload::HttpProxy { .. } => self
                .http_proxy
                .as_ref()
                .map(|provider| provider.as_ref() as &dyn ExecutionProvider)
                .ok_or_else(|| {
                    AppError::ConfigError("HTTP proxy provider is not available".to_string())
                }),
        }
    }
}

#[async_trait]
impl ExecutionProvider for CompositeProvider {
    async fn execute(&self, request: ExecutionRequest) -> Result<ExecutionResponse> {
        let provider = self.route(&request)?;
        provider.execute(request).await
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    #![allow(deprecated)]

    use super::*;

    #[test]
    fn execution_constraints_default() {
        let constraints = ExecutionConstraints::default();
        assert_eq!(constraints.timeout_seconds, 30);
        assert_eq!(constraints.memory_mb, 256);
        assert!(!constraints.network_enabled);
    }

    #[test]
    fn execution_context_default() {
        let context = ExecutionContext::default();
        assert_eq!(context.tenant_id, Uuid::nil());
        assert_eq!(context.user_id, Uuid::nil());
        assert!(context.request_id.is_empty());
    }

    #[test]
    fn execution_request_can_be_built() {
        let request = ExecutionRequest {
            caller: ExecutionCaller::Skill {
                skill_id: Uuid::new_v4(),
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
        assert!(matches!(request.caller, ExecutionCaller::Skill { .. }));
        assert!(matches!(request.payload, ExecutionPayload::Code { .. }));
    }

    #[test]
    fn execution_response_serializes() {
        let response = ExecutionResponse {
            output: serde_json::json!({"ok": true}),
            stdout: "hello".to_string(),
            stderr: String::new(),
            exit_code: 0,
            execution_time_ms: 42,
            timed_out: false,
            http_status: None,
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("hello"));
        assert!(json.contains("42"));
    }

    #[tokio::test]
    async fn composite_provider_routes_code_and_http_payloads() {
        use std::sync::Arc;

        struct MockDocker;
        #[async_trait]
        impl ExecutionProvider for MockDocker {
            async fn execute(&self, _request: ExecutionRequest) -> Result<ExecutionResponse> {
                Ok(ExecutionResponse {
                    output: serde_json::json!({"from": "docker"}),
                    stdout: String::new(),
                    stderr: String::new(),
                    exit_code: 0,
                    execution_time_ms: 0,
                    timed_out: false,
                    http_status: None,
                })
            }
        }

        struct MockHttp;
        #[async_trait]
        impl ExecutionProvider for MockHttp {
            async fn execute(&self, _request: ExecutionRequest) -> Result<ExecutionResponse> {
                Ok(ExecutionResponse {
                    output: serde_json::json!({"from": "http"}),
                    stdout: String::new(),
                    stderr: String::new(),
                    exit_code: 0,
                    execution_time_ms: 0,
                    timed_out: false,
                    http_status: Some(200),
                })
            }
        }

        let composite =
            CompositeProvider::new(Some(Arc::new(MockDocker)), Some(Arc::new(MockHttp)));
        let code_request = ExecutionRequest {
            caller: ExecutionCaller::Skill {
                skill_id: Uuid::new_v4(),
                runtime: "python311".to_string(),
            },
            payload: ExecutionPayload::Code {
                source: "x=1".to_string(),
                language: "python311".to_string(),
            },
            constraints: ExecutionConstraints::default(),
            input: serde_json::json!({}),
            context: ExecutionContext::default(),
        };
        assert_eq!(
            composite.execute(code_request).await.unwrap().output["from"],
            "docker"
        );

        let http_request = ExecutionRequest {
            caller: ExecutionCaller::McpTool {
                tool_id: Uuid::new_v4(),
            },
            payload: ExecutionPayload::HttpProxy {
                url: "http://example.com".to_string(),
                method: "GET".to_string(),
                timeout_ms: Some(250),
            },
            constraints: ExecutionConstraints::default(),
            input: serde_json::json!({}),
            context: ExecutionContext::default(),
        };
        assert_eq!(
            composite.execute(http_request).await.unwrap().output["from"],
            "http"
        );
    }

    #[tokio::test]
    async fn composite_provider_errors_when_provider_is_unavailable() {
        use std::sync::Arc;

        struct MockHttp;
        #[async_trait]
        impl ExecutionProvider for MockHttp {
            async fn execute(&self, _request: ExecutionRequest) -> Result<ExecutionResponse> {
                Ok(ExecutionResponse {
                    output: serde_json::json!({}),
                    stdout: String::new(),
                    stderr: String::new(),
                    exit_code: 0,
                    execution_time_ms: 0,
                    timed_out: false,
                    http_status: Some(200),
                })
            }
        }

        let no_docker = CompositeProvider::new(None, Some(Arc::new(MockHttp)));
        let code_request = ExecutionRequest {
            caller: ExecutionCaller::Skill {
                skill_id: Uuid::new_v4(),
                runtime: "python311".to_string(),
            },
            payload: ExecutionPayload::Code {
                source: "x=1".to_string(),
                language: "python311".to_string(),
            },
            constraints: ExecutionConstraints::default(),
            input: serde_json::json!({}),
            context: ExecutionContext::default(),
        };
        assert!(no_docker
            .execute(code_request)
            .await
            .unwrap_err()
            .to_string()
            .contains("Docker sandbox provider is not available"));

        struct MockDocker;
        #[async_trait]
        impl ExecutionProvider for MockDocker {
            async fn execute(&self, _request: ExecutionRequest) -> Result<ExecutionResponse> {
                Ok(ExecutionResponse {
                    output: serde_json::json!({}),
                    stdout: String::new(),
                    stderr: String::new(),
                    exit_code: 0,
                    execution_time_ms: 0,
                    timed_out: false,
                    http_status: None,
                })
            }
        }

        let no_http = CompositeProvider::new(Some(Arc::new(MockDocker)), None);
        let http_request = ExecutionRequest {
            caller: ExecutionCaller::McpTool {
                tool_id: Uuid::new_v4(),
            },
            payload: ExecutionPayload::HttpProxy {
                url: "http://example.com".to_string(),
                method: "GET".to_string(),
                timeout_ms: None,
            },
            constraints: ExecutionConstraints::default(),
            input: serde_json::json!({}),
            context: ExecutionContext::default(),
        };
        assert!(no_http
            .execute(http_request)
            .await
            .unwrap_err()
            .to_string()
            .contains("HTTP proxy provider is not available"));
    }
}
