//! CLI interface parser.
//!
//! This crate still lives under `service-snippet` for compatibility, but new
//! product work should use the CLI interface terminology.

use common::error::{AppError, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Parsed CLI-friendly interface document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CliInterfaceDocument {
    pub metadata: CliInterfaceMetadata,
    pub body: String,
}

/// Machine-readable metadata from the YAML frontmatter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CliInterfaceMetadata {
    pub name: String,
    pub version: String,
    pub summary: String,
    pub command: String,
    #[serde(default)]
    pub subcommands: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default = "default_visibility")]
    pub visibility: String,
    #[serde(default)]
    pub inputs: Vec<CliInterfaceParameter>,
    pub output: Option<CliInterfaceOutput>,
    #[serde(default)]
    pub examples: Vec<CliInterfaceExample>,
    #[serde(default)]
    pub error_model: Vec<CliInterfaceError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CliInterfaceParameter {
    pub name: String,
    #[serde(rename = "type")]
    pub parameter_type: String,
    #[serde(default)]
    pub required: bool,
    pub description: Option<String>,
    #[serde(default)]
    pub values: Vec<String>,
    pub default: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CliInterfaceOutput {
    #[serde(rename = "type")]
    pub output_type: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CliInterfaceExample {
    pub title: String,
    pub command: String,
    #[serde(default)]
    pub input: Value,
    #[serde(default)]
    pub output: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CliInterfaceError {
    pub code: String,
    pub message: String,
    #[serde(default)]
    pub retryable: bool,
}

/// Parser for the CLI interface format.
pub struct CliInterfaceParser;

impl Default for CliInterfaceParser {
    fn default() -> Self {
        Self::new()
    }
}

impl CliInterfaceParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, content: &str) -> Result<CliInterfaceDocument> {
        let (frontmatter, body) = split_frontmatter(content)?;
        let metadata: CliInterfaceMetadata =
            serde_norway::from_str(frontmatter).map_err(|err| {
                AppError::ValidationError(format!("Invalid CLI interface YAML: {err}"))
            })?;

        validate_metadata(&metadata)?;

        Ok(CliInterfaceDocument {
            metadata,
            body: body.trim().to_string(),
        })
    }
}

/// Backward-compatible alias while the crate and old imports are being migrated.
pub type SnippetParser = CliInterfaceParser;

fn default_visibility() -> String {
    "private".to_string()
}

fn split_frontmatter(content: &str) -> Result<(&str, &str)> {
    let content = content.trim_start();
    if !content.starts_with("---\n") {
        return Err(AppError::ValidationError(
            "CLI interface document must start with YAML frontmatter".to_string(),
        ));
    }

    let rest = &content[4..];
    let end = rest.find("\n---").ok_or_else(|| {
        AppError::ValidationError("CLI interface frontmatter is not closed".to_string())
    })?;

    let frontmatter = &rest[..end];
    let after_marker = &rest[end + 4..];
    let body = after_marker.strip_prefix('\n').unwrap_or(after_marker);

    Ok((frontmatter, body))
}

fn validate_metadata(metadata: &CliInterfaceMetadata) -> Result<()> {
    if metadata.name.trim().is_empty() {
        return Err(AppError::ValidationError(
            "CLI interface name is required".to_string(),
        ));
    }
    if metadata.version.trim().is_empty() {
        return Err(AppError::ValidationError(
            "CLI interface version is required".to_string(),
        ));
    }
    if metadata.summary.trim().is_empty() {
        return Err(AppError::ValidationError(
            "CLI interface summary is required".to_string(),
        ));
    }
    if metadata.command.trim().is_empty() {
        return Err(AppError::ValidationError(
            "CLI interface command is required".to_string(),
        ));
    }

    for input in &metadata.inputs {
        if input.name.trim().is_empty() {
            return Err(AppError::ValidationError(
                "CLI interface input name is required".to_string(),
            ));
        }
        if input.parameter_type.trim().is_empty() {
            return Err(AppError::ValidationError(format!(
                "CLI interface input '{}' type is required",
                input.name
            )));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_cli_interface_frontmatter() {
        let input = r#"---
name: deploy-service
version: 1.0.0
summary: Deploy a service.
command: evolith deploy
subcommands:
  - service
tags:
  - deploy
visibility: tenant
inputs:
  - name: service
    type: string
    required: true
    description: Service name.
output:
  type: object
  description: Deployment result.
examples:
  - title: Deploy staging
    command: evolith deploy service --service api --env staging
    input:
      service: api
    output:
      status: queued
error_model:
  - code: SERVICE_NOT_FOUND
    message: Service is not visible.
    retryable: false
---

## Usage

Deploy an existing service.
"#;

        let parser = CliInterfaceParser::new();
        let document = parser.parse(input).expect("document should parse");

        assert_eq!(document.metadata.name, "deploy-service");
        assert_eq!(document.metadata.command, "evolith deploy");
        assert_eq!(document.metadata.visibility, "tenant");
        assert_eq!(document.metadata.inputs.len(), 1);
        assert_eq!(document.metadata.inputs[0].parameter_type, "string");
        assert_eq!(document.metadata.examples.len(), 1);
        assert!(document.body.contains("Deploy an existing service."));
    }

    #[test]
    fn defaults_visibility_to_private() {
        let input = r#"---
name: inspect-tool
version: 0.1.0
summary: Inspect a tool.
command: evolith tool inspect
---
Body
"#;

        let parser = CliInterfaceParser::new();
        let document = parser.parse(input).expect("document should parse");

        assert_eq!(document.metadata.visibility, "private");
    }

    #[test]
    fn rejects_missing_frontmatter() {
        let parser = CliInterfaceParser::new();
        let err = parser
            .parse("name: missing-frontmatter")
            .expect_err("missing frontmatter should fail");

        assert!(err.to_string().contains("frontmatter"));
    }

    #[test]
    fn rejects_missing_required_command() {
        let input = r#"---
name: broken
version: 0.1.0
summary: Missing command.
---
Body
"#;

        let parser = CliInterfaceParser::new();
        let err = parser
            .parse(input)
            .expect_err("missing command should fail");

        assert!(err.to_string().contains("command"));
    }
}
