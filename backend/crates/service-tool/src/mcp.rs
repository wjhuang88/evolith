//! MCP protocol implementation
//!
//! Implements the Model Context Protocol (MCP) for tool invocation.
//! Reference: https://spec.modelcontextprotocol.io/

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// MCP JSON-RPC version
pub const MCP_JSONRPC_VERSION: &str = "2.0";

/// MCP protocol version
pub const MCP_PROTOCOL_VERSION: &str = "2024-11-05";

/// MCP server version
pub const SERVER_VERSION: &str = "1.0.0";

/// MCP JSON-RPC request
#[derive(Debug, Deserialize)]
pub struct McpRequest {
    pub jsonrpc: String,
    pub id: Value,
    pub method: String,
    #[serde(default)]
    pub params: Option<Value>,
}

/// MCP JSON-RPC response
#[derive(Debug, Serialize)]
pub struct McpResponse {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<McpError>,
}

/// MCP error
#[derive(Debug, Serialize)]
pub struct McpError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

/// MCP error codes
pub mod error_codes {
    pub const PARSE_ERROR: i32 = -32700;
    pub const INVALID_REQUEST: i32 = -32600;
    pub const METHOD_NOT_FOUND: i32 = -32601;
    pub const INVALID_PARAMS: i32 = -32602;
    pub const INTERNAL_ERROR: i32 = -32603;
}

/// MCP initialize params
#[derive(Debug, Deserialize)]
pub struct InitializeParams {
    #[serde(rename = "protocolVersion")]
    pub protocol_version: Option<String>,
    #[serde(default)]
    pub capabilities: Value,
    #[serde(rename = "clientInfo")]
    pub client_info: Option<ClientInfo>,
}

/// Client info
#[derive(Debug, Deserialize)]
pub struct ClientInfo {
    pub name: String,
    pub version: String,
}

/// MCP initialize result
#[derive(Debug, Serialize)]
pub struct InitializeResult {
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
    pub capabilities: Capabilities,
    #[serde(rename = "serverInfo")]
    pub server_info: ServerInfo,
}

/// Server capabilities
#[derive(Debug, Serialize)]
pub struct Capabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Value>,
}

/// Server info
#[derive(Debug, Serialize)]
#[serde(rename = "serverInfo")]
pub struct ServerInfo {
    pub name: String,
    pub version: String,
}

/// Tool in MCP format
#[derive(Debug, Serialize)]
pub struct McpTool {
    pub name: String,
    pub description: String,
    #[serde(rename = "inputSchema")]
    pub input_schema: Value,
}

/// Tools list result
#[derive(Debug, Serialize)]
pub struct ToolsListResult {
    pub tools: Vec<McpTool>,
}

/// Tool call params
#[derive(Debug, Deserialize)]
pub struct ToolCallParams {
    pub name: String,
    #[serde(default)]
    pub arguments: Value,
}

/// Tool call result
#[derive(Debug, Serialize)]
pub struct ToolCallResult {
    pub content: Vec<ToolContent>,
}

/// Tool content block
#[derive(Debug, Serialize)]
pub struct ToolContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: String,
}

impl McpResponse {
    /// Create a success response
    pub fn success(id: Value, result: Value) -> Self {
        Self {
            jsonrpc: MCP_JSONRPC_VERSION.to_string(),
            id: Some(id),
            result: Some(result),
            error: None,
        }
    }

    /// Create an error response
    pub fn error(id: Value, code: i32, message: String, data: Option<Value>) -> Self {
        Self {
            jsonrpc: MCP_JSONRPC_VERSION.to_string(),
            id: Some(id),
            result: None,
            error: Some(McpError {
                code,
                message,
                data,
            }),
        }
    }

    /// Parse error
    pub fn parse_error(id: Value) -> Self {
        Self::error(
            id,
            error_codes::PARSE_ERROR,
            "Parse error".to_string(),
            None,
        )
    }

    /// Invalid request error
    pub fn invalid_request(id: Value, message: &str) -> Self {
        Self::error(id, error_codes::INVALID_REQUEST, message.to_string(), None)
    }

    /// Method not found error
    pub fn method_not_found(id: Value) -> Self {
        Self::error(
            id,
            error_codes::METHOD_NOT_FOUND,
            "Method not found".to_string(),
            None,
        )
    }

    /// Invalid params error
    pub fn invalid_params(id: Value, message: &str) -> Self {
        Self::error(id, error_codes::INVALID_PARAMS, message.to_string(), None)
    }

    /// Internal error
    pub fn internal_error(id: Value, message: &str) -> Self {
        Self::error(id, error_codes::INTERNAL_ERROR, message.to_string(), None)
    }
}

/// Build initialize result
pub fn build_initialize_result() -> InitializeResult {
    InitializeResult {
        protocol_version: MCP_PROTOCOL_VERSION.to_string(),
        capabilities: Capabilities {
            tools: Some(json!({})),
        },
        server_info: ServerInfo {
            name: "evolith".to_string(),
            version: SERVER_VERSION.to_string(),
        },
    }
}

/// Validate arguments against a JSON Schema
///
/// Returns Ok(()) if validation passes, or an error message if validation fails.
pub fn validate_arguments(schema: &Value, arguments: &Value) -> Result<(), String> {
    // If schema is empty or not an object, skip validation
    if !schema.is_object() || schema.as_object().map_or(true, |o| o.is_empty()) {
        return Ok(());
    }

    // Create a JSON Schema validator
    let compiled =
        jsonschema::JSONSchema::compile(schema).map_err(|e| format!("Invalid schema: {}", e))?;

    // Validate the arguments
    let result = compiled.validate(arguments);

    if let Err(errors) = result {
        let error_messages: Vec<String> = errors
            .map(|e| {
                let path = e.instance_path.to_string();
                if path.is_empty() {
                    format!("{}", e)
                } else {
                    format!("at '{}': {}", path, e)
                }
            })
            .collect();
        return Err(format!("Validation failed: {}", error_messages.join(", ")));
    }

    Ok(())
}
