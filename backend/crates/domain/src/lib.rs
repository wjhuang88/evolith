//! Domain Layer
//!
//! Contains core domain models, traits, and business rules.

pub mod api_key;
pub mod audit;
pub mod errors;
pub mod repository;
pub mod skill;
pub mod snippet;
pub mod tenant;
pub mod tool;
pub mod user;

pub use api_key::{ApiKey, ApiKeyStatus, NewApiKey};
pub use errors::DomainError;
pub use repository::{
    ApiKeyRepository, AuditRepository, InvitationRepository, SkillRepository, SnippetRepository,
    TenantRepository, ToolRepository, UserRepository,
};

// Re-export commonly used types
pub use skill::{Dependency as SkillDependency, NewSkill, Runtime, Skill, SkillFilter, UpdateSkill};
pub use snippet::{Dependency as SnippetDependency, NewSnippet, Snippet, SnippetFilter};
pub use tenant::{
    CreateTenantRequest, InviteUserRequest, PlanStatus, Tenant, TenantContext, TenantInvitation,
    TenantPlan, TenantQuotas, TenantStatus, TenantUsage,
};
pub use tool::{HandlerConfig, HandlerType, NewTool, Tool, ToolFilter, UpdateTool, Visibility};
pub use user::{
    Invitation, LoginRequest, LoginResponse, NewUser, TenantRole, UpdateUser, User, UserInfo,
    UserRole,
};
