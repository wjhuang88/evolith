//! SKILL.md parser.
//!
//! Parses YAML frontmatter + Markdown body from SKILL.md files,
//! following the Agent Skills compatible format.

use std::collections::BTreeMap;

use common::error::{AppError, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Parsed skill document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkillDocument {
    pub metadata: SkillMetadata,
    pub body: String,
}

/// Machine-readable metadata from the YAML frontmatter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkillMetadata {
    // Required
    pub name: String,
    pub description: String,
    // Optional with defaults
    pub version: Option<String>,
    pub author: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    // Execution config
    #[serde(rename = "type", default = "default_skill_type")]
    pub skill_type: String,
    #[serde(default = "default_execution")]
    pub execution: String,
    pub runtime: Option<String>,
    pub entrypoint: Option<String>,
    #[serde(default = "default_timeout")]
    pub timeout: u32,
    #[serde(default = "default_memory")]
    pub memory: u32,
    // Dependencies
    #[serde(default)]
    pub dependencies: Vec<String>,
    // Permissions
    pub permissions: Option<SkillPermissions>,
    // Other
    pub license: Option<String>,
    pub compatibility: Option<String>,
    // Catch-all for unknown fields (e.g. metadata, disable-model-invocation, etc.)
    #[serde(flatten)]
    pub metadata_extra: BTreeMap<String, Value>,
}

/// Permission declarations for skill execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkillPermissions {
    pub filesystem: Option<String>,
    pub network: Option<String>,
    #[serde(default)]
    pub environment: Vec<String>,
}

/// Parser for the SKILL.md format.
pub struct SkillParser;

impl Default for SkillParser {
    fn default() -> Self {
        Self::new()
    }
}

impl SkillParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, content: &str) -> Result<SkillDocument> {
        let (frontmatter, body) = split_frontmatter(content)?;
        let metadata: SkillMetadata = serde_yaml::from_str(frontmatter)
            .map_err(|err| AppError::ValidationError(format!("Invalid skill YAML: {err}")))?;

        validate_metadata(&metadata)?;

        Ok(SkillDocument {
            metadata,
            body: body.trim().to_string(),
        })
    }
}

fn default_skill_type() -> String {
    "instruction".to_string()
}

fn default_execution() -> String {
    "client".to_string()
}

fn default_timeout() -> u32 {
    30
}

fn default_memory() -> u32 {
    256
}

fn split_frontmatter(content: &str) -> Result<(&str, &str)> {
    let content = content.trim_start();
    if !content.starts_with("---\n") {
        return Err(AppError::ValidationError(
            "SKILL.md must start with YAML frontmatter".to_string(),
        ));
    }

    let rest = &content[4..];
    let end = rest.find("\n---").ok_or_else(|| {
        AppError::ValidationError("SKILL.md frontmatter is not closed".to_string())
    })?;

    let frontmatter = &rest[..end];
    let after_marker = &rest[end + 4..];
    let body = after_marker.strip_prefix('\n').unwrap_or(after_marker);

    Ok((frontmatter, body))
}

fn validate_metadata(metadata: &SkillMetadata) -> Result<()> {
    // Validate name: required, 1-64 chars, lowercase alphanumeric + hyphens only,
    // cannot start/end with hyphen, no consecutive hyphens
    let name = &metadata.name;
    if name.is_empty() || name.len() > 64 {
        return Err(AppError::ValidationError(
            "Skill name must be 1-64 characters".to_string(),
        ));
    }
    if !name
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    {
        return Err(AppError::ValidationError(
            "Skill name must contain only lowercase letters, numbers, and hyphens".to_string(),
        ));
    }
    if name.starts_with('-') || name.ends_with('-') {
        return Err(AppError::ValidationError(
            "Skill name cannot start or end with a hyphen".to_string(),
        ));
    }
    if name.contains("--") {
        return Err(AppError::ValidationError(
            "Skill name cannot contain consecutive hyphens".to_string(),
        ));
    }

    // Validate description: required, 1-1024 chars
    if metadata.description.is_empty() || metadata.description.len() > 1024 {
        return Err(AppError::ValidationError(
            "Skill description must be 1-1024 characters".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_skill_md() {
        let input = r#"---
name: data-analyzer
version: 1.0.0
description: Analyze data files (CSV, JSON, Excel) and generate statistical reports with visualizations.
author: evolith-team
tags: [data, analysis, statistics, visualization]

type: hybrid
execution: server
runtime: python3.11
entrypoint: src/main.py
timeout: 60
memory: 512

dependencies:
  - pandas>=2.0.0
  - numpy>=1.24.0

permissions:
  filesystem: read
  network: none
  environment: []

disable-model-invocation: false
user-invocable: true
---

# Data Analyzer Skill

A powerful skill for analyzing data files.
"#;

        let parser = SkillParser::new();
        let doc = parser.parse(input).expect("document should parse");

        assert_eq!(doc.metadata.name, "data-analyzer");
        assert_eq!(doc.metadata.version, Some("1.0.0".to_string()));
        assert_eq!(
            doc.metadata.description,
            "Analyze data files (CSV, JSON, Excel) and generate statistical reports with visualizations."
        );
        assert_eq!(doc.metadata.author, Some("evolith-team".to_string()));
        assert_eq!(
            doc.metadata.tags,
            vec!["data", "analysis", "statistics", "visualization"]
        );
        assert_eq!(doc.metadata.skill_type, "hybrid");
        assert_eq!(doc.metadata.execution, "server");
        assert_eq!(doc.metadata.runtime, Some("python3.11".to_string()));
        assert_eq!(doc.metadata.entrypoint, Some("src/main.py".to_string()));
        assert_eq!(doc.metadata.timeout, 60);
        assert_eq!(doc.metadata.memory, 512);
        assert_eq!(
            doc.metadata.dependencies,
            vec!["pandas>=2.0.0", "numpy>=1.24.0"]
        );

        let perms = doc
            .metadata
            .permissions
            .as_ref()
            .expect("permissions should exist");
        assert_eq!(perms.filesystem, Some("read".to_string()));
        assert_eq!(perms.network, Some("none".to_string()));
        assert!(perms.environment.is_empty());

        // Check flattened extra fields
        assert!(doc
            .metadata
            .metadata_extra
            .contains_key("disable-model-invocation"));
        assert!(doc.metadata.metadata_extra.contains_key("user-invocable"));

        assert!(doc.body.contains("# Data Analyzer Skill"));
    }

    #[test]
    fn parses_minimal_skill_md() {
        let input = r#"---
name: simple-skill
description: A minimal skill with only required fields.
---

Body content here.
"#;

        let parser = SkillParser::new();
        let doc = parser.parse(input).expect("document should parse");

        assert_eq!(doc.metadata.name, "simple-skill");
        assert_eq!(
            doc.metadata.description,
            "A minimal skill with only required fields."
        );
        assert_eq!(doc.metadata.skill_type, "instruction");
        assert_eq!(doc.metadata.execution, "client");
        assert_eq!(doc.metadata.timeout, 30);
        assert_eq!(doc.metadata.memory, 256);
        assert!(doc.metadata.tags.is_empty());
        assert!(doc.metadata.dependencies.is_empty());
        assert!(doc.body.contains("Body content here."));
    }

    #[test]
    fn rejects_missing_frontmatter() {
        let parser = SkillParser::new();
        let err = parser
            .parse("name: missing-frontmatter")
            .expect_err("missing frontmatter should fail");

        assert!(err.to_string().contains("frontmatter"));
    }

    #[test]
    fn rejects_invalid_name() {
        // Uppercase letters
        let parser = SkillParser::new();
        let err = parser
            .parse(
                r#"---
name: Invalid-Name
description: A skill with uppercase.
---
Body
"#,
            )
            .expect_err("uppercase name should fail");
        assert!(err.to_string().contains("lowercase"));

        // Starts with hyphen
        let err = parser
            .parse(
                r#"---
name: -bad-name
description: A skill starting with hyphen.
---
Body
"#,
            )
            .expect_err("name starting with hyphen should fail");
        assert!(err.to_string().contains("start or end"));

        // Contains spaces
        let err = parser
            .parse(
                r#"---
name: bad name
description: A skill with spaces.
---
Body
"#,
            )
            .expect_err("name with spaces should fail");
        assert!(err.to_string().contains("lowercase"));
    }

    #[test]
    fn rejects_missing_required_fields() {
        // Missing name
        let parser = SkillParser::new();
        let err = parser
            .parse(
                r#"---
description: Missing name field.
---
Body
"#,
            )
            .expect_err("missing name should fail");
        assert!(err.to_string().contains("name"));

        // Missing description
        let err = parser
            .parse(
                r#"---
name: valid-name
---
Body
"#,
            )
            .expect_err("missing description should fail");
        assert!(err.to_string().contains("description"));
    }

    #[test]
    fn rejects_description_too_long() {
        let long_desc = "x".repeat(1025);
        let input = format!(
            r#"---
name: valid-name
description: {}
---
Body
"#,
            long_desc
        );

        let parser = SkillParser::new();
        let err = parser
            .parse(&input)
            .expect_err("description too long should fail");

        assert!(err.to_string().contains("1024"));
    }
}
