//! Skill Service
//!
//! Handles skill registration, parsing, and execution.

pub mod docker_executor;
pub mod executor;
pub mod parser;
pub mod sandbox;

pub use docker_executor::DockerExecutor;
pub use executor::{DefaultSkillExecutor, ExecuteRequest, ExecuteResponse, SkillExecutor};
pub use parser::{SkillDocument, SkillMetadata, SkillParser, SkillPermissions};
pub use sandbox::SandboxConfig;
