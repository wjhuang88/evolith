//! Skill Service
//!
//! Handles skill registration, parsing, and execution.

pub mod docker_executor;
pub mod docker_sandbox_provider;
pub mod executor;
pub mod parser;
pub mod sandbox;

pub use docker_executor::DockerExecutor;
pub use docker_sandbox_provider::{DockerSandboxProvider, PoolConfig};
pub use executor::{DefaultSkillExecutor, ExecuteRequest, ExecuteResponse, SkillExecutor, SkillExecutorAdapter};
pub use parser::{SkillDocument, SkillMetadata, SkillParser, SkillPermissions};
pub use sandbox::SandboxConfig;
