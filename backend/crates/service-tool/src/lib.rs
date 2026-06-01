//! Tool Service
//!
//! Handles MCP tool registration, discovery, and execution.

pub mod discovery;
pub mod executor;
pub mod mcp;
pub mod registry;

pub use executor::ToolExecutor;
pub use registry::ToolRegistry;
