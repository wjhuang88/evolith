//! Tool Service
//!
//! Handles MCP tool registration, discovery, and execution.

pub mod discovery;
pub mod executor;
pub mod mcp;

pub use executor::ToolExecutor;
