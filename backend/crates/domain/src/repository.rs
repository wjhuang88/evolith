//! Repository traits for database abstraction

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::user::{TenantRole, UpdateUser};
use crate::audit::AuditLog;
use crate::Invitation;
use crate::{
    NewSkill, NewSnippet, NewTool, NewUser, Skill, SkillFilter, Snippet, SnippetFilter, Tool,
    ToolFilter, UpdateTool, User,
};
use common::error::Result;


/// User repository trait
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: NewUser, tenant_id: Uuid, tenant_role: TenantRole)
        -> Result<User>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>>;
    async fn find_by_username(&self, username: &str, tenant_id: Uuid) -> Result<Option<User>>;
    async fn update(&self, id: Uuid, user: UpdateUser) -> Result<User>;
    async fn delete(&self, id: Uuid) -> Result<()>;
    async fn verify_email(&self, id: Uuid) -> Result<()>;
    async fn set_verify_token(&self, id: Uuid, token: &str) -> Result<()>;
    async fn find_by_verify_token(&self, token: &str) -> Result<Option<User>>;
    async fn set_reset_token(&self, id: Uuid, token: &str, expires_at: DateTime<Utc>)
        -> Result<()>;
    async fn find_by_reset_token(&self, token: &str) -> Result<Option<User>>;
    async fn clear_reset_token(&self, id: Uuid) -> Result<()>;
    async fn update_password(&self, id: Uuid, password_hash: &str) -> Result<()>;
}

/// Tool repository trait
#[async_trait]
pub trait ToolRepository: Send + Sync {
    async fn create(&self, tool: NewTool, owner_id: Uuid) -> Result<Tool>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Tool>>;
    async fn find_by_name(&self, name: &str) -> Result<Option<Tool>>;
    async fn find_all(&self, filter: ToolFilter) -> Result<Vec<Tool>>;
    async fn count(&self, filter: &ToolFilter) -> Result<u32>;
    async fn update(&self, id: Uuid, tool: UpdateTool) -> Result<Tool>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}

/// Skill repository trait
#[async_trait]
pub trait SkillRepository: Send + Sync {
    async fn create(&self, skill: NewSkill, owner_id: Uuid) -> Result<Skill>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Skill>>;
    async fn find_by_name_and_version(&self, name: &str, version: &str) -> Result<Option<Skill>>;
    async fn find_all(&self, filter: SkillFilter) -> Result<Vec<Skill>>;
    async fn count(&self, filter: &SkillFilter) -> Result<u32>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}

/// Snippet repository trait
#[async_trait]
pub trait SnippetRepository: Send + Sync {
    async fn create(&self, snippet: NewSnippet, owner_id: Uuid) -> Result<Snippet>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Snippet>>;
    async fn find_all(&self, filter: SnippetFilter) -> Result<Vec<Snippet>>;
    async fn count(&self, filter: &SnippetFilter) -> Result<u32>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}

/// Tenant repository trait
#[async_trait]
pub trait TenantRepository: Send + Sync {
    async fn create(
        &self,
        tenant: crate::CreateTenantRequest,
        owner_id: Uuid,
    ) -> Result<crate::Tenant>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<crate::Tenant>>;
    async fn find_by_slug(&self, slug: &str) -> Result<Option<crate::Tenant>>;
    async fn find_by_domain(&self, domain: &str) -> Result<Option<crate::Tenant>>;
    async fn update(&self, id: Uuid, tenant: crate::CreateTenantRequest) -> Result<crate::Tenant>;
    async fn delete(&self, id: Uuid) -> Result<()>;
    async fn update_usage(&self, id: Uuid) -> Result<()>;
}

/// Tenant invitation repository trait
#[async_trait]
pub trait InvitationRepository: Send + Sync {
    async fn create(&self, invitation: NewInvitation) -> Result<Invitation>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Invitation>>;
    async fn find_by_token(&self, token: &str) -> Result<Option<Invitation>>;
    /// Find pending invitations for a tenant
    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<Invitation>>;
    /// Check if email is already invited in the tenant
    async fn find_by_email(&self, tenant_id: Uuid, email: &str) -> Result<Option<Invitation>>;
    /// Mark invitation as accepted
    async fn accept(&self, id: Uuid) -> Result<()>;
    /// Delete/cancel invitation
    async fn delete(&self, id: Uuid) -> Result<()>;
}

/// Struct for creating new invitations
#[derive(Debug, Clone)]
pub struct NewInvitation {
    pub tenant_id: Uuid,
    pub email: String,
    pub role: String,  // admin, member
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub created_by: Uuid,
}

impl NewInvitation {
    pub fn new(
        tenant_id: Uuid,
        email: String,
        role: String, // admin, member
        token: String, 
        expires_at: DateTime<Utc>,
        created_by: Uuid,
    ) -> Self {
        Self {
            tenant_id,
            email,
            role,
            token,
            expires_at,
            created_by,
        }
    }
}

/// Audit repository trait
#[async_trait]
pub trait AuditRepository: Send + Sync {
    async fn create(&self, log: AuditLog) -> Result<()>;
    async fn find_by_tenant(&self, tenant_id: Uuid, limit: usize, offset: usize) -> Result<Vec<AuditLog>>;
    async fn find_by_user(&self, user_id: Uuid, limit: usize, offset: usize) -> Result<Vec<AuditLog>>;
    async fn find_by_action(&self, action: &str, limit: usize, offset: usize) -> Result<Vec<AuditLog>>;
}
