//! Tool Service
//!
//! Handles MCP tool registration, discovery, and execution.

pub mod discovery;
pub mod egress;
pub mod executor;
pub mod http_proxy_provider;
pub mod mcp;

pub use egress::{DnsResolver, EgressError, EgressPolicy, SafeHttpClient, SafeHttpResponse};
pub use executor::{HttpToolExecutor, ToolExecutor, ToolExecutorAdapter};
pub use http_proxy_provider::HttpProxyProvider;
