//! Skill DTOs

use serde::{Deserialize, Serialize};
use validator::Validate;

use domain::skill::{Dependency, Runtime, Skill, Visibility};

/// Request to create a new skill
#[derive(Debug, Deserialize, Validate)]
pub struct CreateSkillRequest {
    /// Skill name (1-128 chars)
    #[validate(length(min = 1, max = 128))]
    pub name: String,

    /// Semantic version string
    #[validate(length(min = 1, max = 32))]
    pub version: String,

    /// Human-readable description
    #[validate(length(min = 1))]
    pub description: String,

    /// Skill markdown content (SKILL.md format)
    #[validate(length(min = 1))]
    pub content: String,

    /// Runtime environment: python311, node20, wasm
    pub runtime: String,

    /// Package dependencies
    #[serde(default)]
    pub dependencies: Vec<DependencyInput>,

    /// Public visibility (default: false = private)
    #[serde(default)]
    pub is_public: bool,
}

/// Dependency input for skill creation
#[derive(Debug, Clone, Deserialize)]
pub struct DependencyInput {
    /// Package name
    pub name: String,
    /// Version constraint
    pub version: String,
}

impl DependencyInput {
    /// Convert to domain Dependency
    pub fn to_domain(&self) -> Dependency {
        Dependency {
            name: self.name.clone(),
            version: self.version.clone(),
        }
    }
}

/// Request to update an existing skill
#[derive(Debug, Deserialize)]
pub struct UpdateSkillRequest {
    pub name: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub runtime: Option<String>,
    pub dependencies: Option<Vec<DependencyInput>>,
    pub is_public: Option<bool>,
}

/// Skill response DTO
#[derive(Debug, Serialize)]
pub struct SkillResponse {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    /// Skill markdown content
    pub content: String,
    /// Runtime environment
    pub runtime: String,
    /// Package dependencies
    pub dependencies: Vec<DependencyResponse>,
    /// Public visibility
    pub is_public: bool,
    /// Not in domain, defaulting to empty
    #[serde(default)]
    pub category: String,
    /// Not in domain, defaulting to empty
    #[serde(default)]
    pub tags: Vec<String>,
    pub owner_id: String,
    pub tenant_id: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Dependency in response
#[derive(Debug, Serialize)]
pub struct DependencyResponse {
    pub name: String,
    pub version: String,
}

impl From<Dependency> for DependencyResponse {
    fn from(dep: Dependency) -> Self {
        Self {
            name: dep.name,
            version: dep.version,
        }
    }
}

impl From<Skill> for SkillResponse {
    fn from(skill: Skill) -> Self {
        Self {
            id: skill.id.to_string(),
            name: skill.name,
            version: skill.version,
            description: skill.description,
            content: skill.skill_md,
            runtime: runtime_to_string(&skill.runtime),
            dependencies: skill
                .dependencies
                .into_iter()
                .map(DependencyResponse::from)
                .collect(),
            is_public: matches!(skill.visibility, Visibility::Public),
            category: String::new(),
            tags: Vec::new(),
            owner_id: skill.owner_id.to_string(),
            tenant_id: skill.tenant_id.to_string(),
            created_at: skill.created_at.to_rfc3339(),
            updated_at: skill.updated_at.to_rfc3339(),
        }
    }
}

/// Convert Runtime enum to string
pub fn runtime_to_string(runtime: &Runtime) -> String {
    match runtime {
        Runtime::Python311 => "python311".to_string(),
        Runtime::Node20 => "node20".to_string(),
        Runtime::Wasm => "wasm".to_string(),
    }
}

/// Parse runtime string to Runtime enum
pub fn parse_runtime(s: &str) -> Result<Runtime, String> {
    match s.to_lowercase().as_str() {
        "python311" | "python" | "py" => Ok(Runtime::Python311),
        "node20" | "node" | "javascript" | "js" => Ok(Runtime::Node20),
        "wasm" | "webassembly" => Ok(Runtime::Wasm),
        _ => Err(format!("Unknown runtime: {}", s)),
    }
}

/// Response for skill loading (minimal info for execution)
#[derive(Debug, Serialize)]
pub struct SkillLoadResponse {
    pub id: String,
    pub name: String,
    pub version: String,
    /// Raw markdown content
    pub content: String,
    pub runtime: String,
}

impl From<Skill> for SkillLoadResponse {
    fn from(skill: Skill) -> Self {
        Self {
            id: skill.id.to_string(),
            name: skill.name,
            version: skill.version,
            content: skill.skill_md,
            runtime: runtime_to_string(&skill.runtime),
        }
    }
}

/// Request to execute a skill
#[derive(Debug, Deserialize)]
pub struct ExecuteSkillRequest {
    /// Input parameters as JSON
    #[serde(default = "default_params")]
    pub parameters: serde_json::Value,
    /// Override code (optional — if not provided, use skill's stored code)
    pub code: Option<String>,
}

fn default_params() -> serde_json::Value {
    serde_json::json!({})
}

/// Skill execution response
#[derive(Debug, Serialize)]
pub struct SkillExecutionResponse {
    pub status: String,
    pub skill_id: String,
    pub skill_name: String,
    pub runtime: String,
    pub output: String,
    pub errors: String,
    pub exit_code: i64,
    pub execution_time_ms: u64,
    pub timed_out: bool,
}
