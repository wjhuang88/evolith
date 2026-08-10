//! Application state shared across handlers

use std::sync::Arc;

use common::execution::ExecutionProvider;
use domain::repository::{
    ApiKeyRepository, AuditRepository, GitRepoRepository, InvitationRepository, OutboxRepository,
    SkillRepository, SnippetRepository, TenantRepository, ToolRepository, UserRepository,
};
use infra::cache::Cache;
use infra::config::AppConfig;
use infra::mailer::Mailer;
use service_auth::{Argon2Hasher, JwtHandler};
use service_skill::executor::SkillExecutor;
use service_tool::executor::ToolExecutor;

/// Application state containing all shared resources
pub struct AppState {
    /// Application configuration
    pub config: AppConfig,
    /// JWT handler for token generation/validation
    pub jwt: JwtHandler,
    /// Password hasher using Argon2id
    pub hasher: Argon2Hasher,
    /// User repository for user operations
    pub user_repo: Arc<dyn UserRepository>,
    /// Tenant repository for tenant operations
    pub tenant_repo: Arc<dyn TenantRepository>,
    /// Tool repository for tool operations
    pub tool_repo: Arc<dyn ToolRepository>,
    /// Skill repository for skill operations
    pub skill_repo: Arc<dyn SkillRepository>,
    /// Snippet repository for snippet operations
    pub snippet_repo: Arc<dyn SnippetRepository>,
    /// Audit repository for audit logging
    pub audit_repo: Arc<dyn AuditRepository>,
    /// Invitation repository for tenant invitations
    pub invitation_repo: Arc<dyn InvitationRepository>,
    /// API Key repository for API key management
    pub api_key_repo: Arc<dyn ApiKeyRepository>,
    /// Git repo repository for git repo operations
    pub git_repo_repo: Arc<dyn GitRepoRepository>,
    /// Durable outbox for business events emitted by request handlers
    pub outbox_repo: Arc<dyn OutboxRepository>,
    /// Cache for session data, rate limiting counters, etc.
    pub cache: Arc<dyn Cache>,
    /// Mailer for sending transactional emails
    pub mailer: Arc<dyn Mailer>,
    /// Unified execution provider (routes by payload type)
    pub execution_provider: Arc<dyn ExecutionProvider>,
    /// Skill executor for running code in sandboxed environment (legacy facade)
    pub skill_executor: Arc<dyn SkillExecutor>,
    /// Tool executor for executing HTTP-based MCP tools (legacy facade)
    pub tool_executor: Arc<dyn ToolExecutor>,
    /// Base path for git repo storage on disk
    pub git_storage_base_path: String,
}
