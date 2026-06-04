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

    /// Author name
    #[serde(default)]
    pub author: Option<String>,

    /// Tags
    #[serde(default)]
    pub tags: Vec<String>,

    /// Skill type: instruction, agent
    #[serde(default = "default_skill_type")]
    pub skill_type: String,

    /// Execution mode: client, server
    #[serde(default = "default_execution")]
    pub execution: String,

    /// Entrypoint file
    #[serde(default)]
    pub entrypoint: Option<String>,

    /// Timeout in seconds
    #[serde(default = "default_timeout")]
    pub timeout: i32,

    /// Memory limit in MB
    #[serde(default = "default_memory_mb")]
    pub memory_mb: i32,

    /// Permissions JSON
    #[serde(default)]
    pub permissions: Option<serde_json::Value>,

    /// License
    #[serde(default)]
    pub license: Option<String>,

    /// Compatibility string
    #[serde(default)]
    pub compatibility: Option<String>,

    /// Disable model invocation
    #[serde(default)]
    pub disable_model_invocation: bool,

    /// User invocable
    #[serde(default = "default_true")]
    pub user_invocable: bool,

    /// Argument hint
    #[serde(default)]
    pub argument_hint: Option<String>,
}

fn default_skill_type() -> String {
    "instruction".to_string()
}

fn default_execution() -> String {
    "client".to_string()
}

fn default_timeout() -> i32 {
    30
}

fn default_memory_mb() -> i32 {
    256
}

fn default_true() -> bool {
    true
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
    pub author: Option<String>,
    pub tags: Option<Vec<String>>,
    pub skill_type: Option<String>,
    pub execution: Option<String>,
    pub entrypoint: Option<String>,
    pub timeout: Option<i32>,
    pub memory_mb: Option<i32>,
    pub permissions: Option<serde_json::Value>,
    pub license: Option<String>,
    pub compatibility: Option<String>,
    pub disable_model_invocation: Option<bool>,
    pub user_invocable: Option<bool>,
    pub argument_hint: Option<String>,
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
    /// Skill category (mapped from skill_type)
    pub category: String,
    /// Tags from domain
    pub tags: Vec<String>,
    pub owner_id: String,
    pub tenant_id: String,
    /// Author name
    pub author: Option<String>,
    /// Skill type
    pub skill_type: String,
    /// Execution mode
    pub execution: String,
    /// Entrypoint file
    pub entrypoint: Option<String>,
    /// Timeout in seconds
    pub timeout: i32,
    /// Memory limit in MB
    pub memory_mb: i32,
    /// Permissions JSON
    pub permissions: Option<serde_json::Value>,
    /// License
    pub license: Option<String>,
    /// Compatibility string
    pub compatibility: Option<String>,
    /// Disable model invocation
    pub disable_model_invocation: bool,
    /// User invocable
    pub user_invocable: bool,
    /// Argument hint
    pub argument_hint: Option<String>,
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
            category: skill.skill_type.clone(),
            tags: skill.tags.clone(),
            owner_id: skill.owner_id.to_string(),
            tenant_id: skill.tenant_id.to_string(),
            author: skill.author,
            skill_type: skill.skill_type,
            execution: skill.execution,
            entrypoint: skill.entrypoint,
            timeout: skill.timeout,
            memory_mb: skill.memory_mb,
            permissions: skill.permissions,
            license: skill.license,
            compatibility: skill.compatibility,
            disable_model_invocation: skill.disable_model_invocation,
            user_invocable: skill.user_invocable,
            argument_hint: skill.argument_hint,
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
