//! Tool Service
//!
//! Handles MCP tool registration, discovery, and execution.

pub mod registry;
pub mod discovery;
pub mod executor;
pub mod mcp;

pub use registry::ToolRegistry;
pub use executor::ToolExecutor;
