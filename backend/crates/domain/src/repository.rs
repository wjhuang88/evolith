//! Repository traits for database abstraction

use async_trait::async_trait;

use crate::{
    NewSkill, NewSnippet, NewTool, NewUser, Skill, SkillFilter, Snippet, SnippetFilter, Tool,
    ToolFilter, UpdateTool, User,
};
use common::error::Result;

/// User repository trait
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: NewUser) -> Result<User>;
    async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<User>>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>>;
    async fn update(&self, id: uuid::Uuid, user: NewUser) -> Result<User>;
    async fn delete(&self, id: uuid::Uuid) -> Result<()>;
}

/// Tool repository trait
#[async_trait]
pub trait ToolRepository: Send + Sync {
    async fn create(&self, tool: NewTool, owner_id: uuid::Uuid) -> Result<Tool>;
    async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<Tool>>;
    async fn find_by_name(&self, name: &str) -> Result<Option<Tool>>;
    async fn find_all(&self, filter: ToolFilter) -> Result<Vec<Tool>>;
    async fn count(&self, filter: &ToolFilter) -> Result<u32>;
    async fn update(&self, id: uuid::Uuid, tool: UpdateTool) -> Result<Tool>;
    async fn delete(&self, id: uuid::Uuid) -> Result<()>;
}

/// Skill repository trait
#[async_trait]
pub trait SkillRepository: Send + Sync {
    async fn create(&self, skill: NewSkill, owner_id: uuid::Uuid) -> Result<Skill>;
    async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<Skill>>;
    async fn find_by_name_and_version(&self, name: &str, version: &str) -> Result<Option<Skill>>;
    async fn find_all(&self, filter: SkillFilter) -> Result<Vec<Skill>>;
    async fn count(&self, filter: &SkillFilter) -> Result<u32>;
    async fn delete(&self, id: uuid::Uuid) -> Result<()>;
}

/// Snippet repository trait
#[async_trait]
pub trait SnippetRepository: Send + Sync {
    async fn create(&self, snippet: NewSnippet, owner_id: uuid::Uuid) -> Result<Snippet>;
    async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<Snippet>>;
    async fn find_all(&self, filter: SnippetFilter) -> Result<Vec<Snippet>>;
    async fn count(&self, filter: &SnippetFilter) -> Result<u32>;
    async fn delete(&self, id: uuid::Uuid) -> Result<()>;
}

/// Tenant repository trait
#[async_trait]
pub trait TenantRepository: Send + Sync {
    async fn create(&self, tenant: crate::CreateTenantRequest, owner_id: uuid::Uuid) -> Result<crate::Tenant>;
    async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<crate::Tenant>>;
    async fn find_by_slug(&self, slug: &str) -> Result<Option<crate::Tenant>>;
    async fn find_by_domain(&self, domain: &str) -> Result<Option<crate::Tenant>>;
    async fn update(&self, id: uuid::Uuid, tenant: crate::CreateTenantRequest) -> Result<crate::Tenant>;
    async fn delete(&self, id: uuid::Uuid) -> Result<()>;
    async fn update_usage(&self, id: uuid::Uuid) -> Result<()>;
}
