//! SKILL.md parser

use common::error::Result;
use serde::{Deserialize, Serialize};

/// Parsed skill metadata
#[derive(Debug, Serialize, Deserialize)]
pub struct SkillMetadata {
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: Option<String>,
    pub runtime: String,
}

/// Parser for SKILL.md format
pub struct SkillParser;

impl SkillParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, _content: &str) -> Result<SkillMetadata> {
        Ok(SkillMetadata {
            name: "unknown".to_string(),
            description: "Not implemented".to_string(),
            version: "0.0.1".to_string(),
            author: None,
            runtime: "javascript".to_string(),
        })
    }
}
