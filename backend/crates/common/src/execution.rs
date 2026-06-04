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

/// Identifies which subsystem is requesting execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionCaller {
    /// Skill code execution (from Skill handler)
    Skill {
        skill_id: Uuid,
        runtime: String,
    },
    /// CLI command execution (from CLI interface handler)
    Cli {
        snippet_id: Uuid,
        command: String,
    },
    /// MCP tool execution (from MCP handler)
    McpTool { tool_id: Uuid },
}

/// The actual work to be executed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionPayload {
    /// Execute source code (Skill / Serverless Function)
    Code {
        source: String,
        language: String,
    },
    /// Execute a shell command (CLI interface)
    Command {
        command: String,
        args: Vec<String>,
    },
    /// Forward to an external HTTP endpoint (MCP HTTP tool)
    HttpProxy {
        url: String,
        method: String,
    },
}

/// Resource and safety constraints for an execution.
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

/// Audit and routing context attached to every execution.
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

/// Unified execution request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionRequest {
    pub caller: ExecutionCaller,
    pub payload: ExecutionPayload,
    pub constraints: ExecutionConstraints,
    /// Caller-supplied parameters (JSON).
    pub input: serde_json::Value,
    pub context: ExecutionContext,
}

// ---------------------------------------------------------------------------
// Response type
// ---------------------------------------------------------------------------

/// Unified execution response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResponse {
    /// Structured output on success.
    pub output: serde_json::Value,
    /// Captured stdout.
    pub stdout: String,
    /// Captured stderr.
    pub stderr: String,
    /// Exit code: 0 = success, >0 = application error, -1 = infrastructure error.
    pub exit_code: i64,
    /// Execution time in milliseconds.
    pub execution_time_ms: u64,
    /// Whether the execution timed out.
    pub timed_out: bool,
    /// HTTP status code (only meaningful for HttpProxy).
    pub http_status: Option<u16>,
}

// ---------------------------------------------------------------------------
// Trait
// ---------------------------------------------------------------------------

/// Unified execution provider.
///
/// Implementations route based on `ExecutionCaller` + `ExecutionPayload` to the
/// appropriate backend (Docker sandbox, HTTP proxy, etc.).
#[async_trait]
pub trait ExecutionProvider: Send + Sync {
    async fn execute(&self, request: ExecutionRequest) -> Result<ExecutionResponse>;
}

// ---------------------------------------------------------------------------
// Composite provider that routes by caller/payload
// ---------------------------------------------------------------------------

/// Routes execution requests to the appropriate provider based on payload type.
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

    fn route(&self, request: &ExecutionRequest) -> Result<&dyn ExecutionProvider> {
        match &request.payload {
            ExecutionPayload::Code { .. } | ExecutionPayload::Command { .. } => {
                self.docker_sandbox
                    .as_ref()
                    .map(|p| p.as_ref() as &dyn ExecutionProvider)
                    .ok_or_else(|| {
                        AppError::ConfigError(
                            "Docker sandbox provider is not available".to_string(),
                        )
                    })
            }
            ExecutionPayload::HttpProxy { .. } => self
                .http_proxy
                .as_ref()
                .map(|p| p.as_ref() as &dyn ExecutionProvider)
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

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;

    #[test]
    fn execution_constraints_default() {
        let c = ExecutionConstraints::default();
        assert_eq!(c.timeout_seconds, 30);
        assert_eq!(c.memory_mb, 256);
        assert!(!c.network_enabled);
    }

    #[test]
    fn execution_context_default() {
        let ctx = ExecutionContext::default();
        assert_eq!(ctx.tenant_id, Uuid::nil());
        assert_eq!(ctx.user_id, Uuid::nil());
        assert!(ctx.request_id.is_empty());
    }

    #[test]
    fn execution_request_can_be_built() {
        let req = ExecutionRequest {
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
        assert!(matches!(req.caller, ExecutionCaller::Skill { .. }));
        assert!(matches!(req.payload, ExecutionPayload::Code { .. }));
    }

    #[test]
    fn execution_response_serializes() {
        let resp = ExecutionResponse {
            output: serde_json::json!({"ok": true}),
            stdout: "hello".to_string(),
            stderr: String::new(),
            exit_code: 0,
            execution_time_ms: 42,
            timed_out: false,
            http_status: None,
        };
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("hello"));
        assert!(json.contains("42"));
    }

    #[tokio::test]
    async fn composite_provider_routes_code_to_docker() {
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

        let composite = CompositeProvider::new(
            Some(Arc::new(MockDocker)),
            Some(Arc::new(MockHttp)),
        );

        // Code payload → docker
        let req = ExecutionRequest {
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
        let resp = composite.execute(req).await.unwrap();
        assert_eq!(resp.output["from"], "docker");

        // HttpProxy payload → http
        let req = ExecutionRequest {
            caller: ExecutionCaller::McpTool {
                tool_id: Uuid::new_v4(),
            },
            payload: ExecutionPayload::HttpProxy {
                url: "http://example.com".to_string(),
                method: "GET".to_string(),
            },
            constraints: ExecutionConstraints::default(),
            input: serde_json::json!({}),
            context: ExecutionContext::default(),
        };
        let resp = composite.execute(req).await.unwrap();
        assert_eq!(resp.output["from"], "http");
    }

    #[tokio::test]
    async fn composite_provider_errors_when_docker_unavailable() {
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

        // No docker provider
        let composite = CompositeProvider::new(None, Some(Arc::new(MockHttp)));

        let req = ExecutionRequest {
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
        let err = composite.execute(req).await.unwrap_err();
        assert!(err.to_string().contains("Docker sandbox provider is not available"));
    }

    #[tokio::test]
    async fn composite_provider_errors_when_http_unavailable() {
        use std::sync::Arc;

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

        // No http provider
        let composite = CompositeProvider::new(Some(Arc::new(MockDocker)), None);

        let req = ExecutionRequest {
            caller: ExecutionCaller::McpTool {
                tool_id: Uuid::new_v4(),
            },
            payload: ExecutionPayload::HttpProxy {
                url: "http://example.com".to_string(),
                method: "GET".to_string(),
            },
            constraints: ExecutionConstraints::default(),
            input: serde_json::json!({}),
            context: ExecutionContext::default(),
        };
        let err = composite.execute(req).await.unwrap_err();
        assert!(err.to_string().contains("HTTP proxy provider is not available"));
    }
}
