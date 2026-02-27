//! Skill domain model

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: Uuid,
    pub name: String,
    pub version: String,
    pub description: String,
    pub skill_md: String,
    pub code_package_path: Option<String>,
    pub runtime: Runtime,
    pub dependencies: Vec<Dependency>,
    pub owner_id: Uuid,
    pub visibility: Visibility,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Runtime {
    Python311,
    Node20,
    Wasm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct NewSkill {
    #[validate(length(min = 1, max = 128))]
    pub name: String,
    #[validate(length(min = 1, max = 32))]
    pub version: String,
    #[validate(length(min = 1))]
    pub description: String,
    pub skill_md: String,
    pub code_package_path: Option<String>,
    pub runtime: Runtime,
    pub dependencies: Vec<Dependency>,
    pub visibility: Option<Visibility>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SkillFilter {
    pub search: Option<String>,
    pub runtime: Option<Runtime>,
    pub visibility: Option<Visibility>,
    pub owner_id: Option<Uuid>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

impl Default for SkillFilter {
    fn default() -> Self {
        Self {
            search: None,
            runtime: None,
            visibility: None,
            owner_id: None,
            page: Some(1),
            per_page: Some(20),
        }
    }
}

// Re-export Visibility from tool module
pub use crate::tool::Visibility;
