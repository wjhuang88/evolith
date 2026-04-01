//! Database module

// SQLite repositories
pub mod api_key_repo;
pub mod audit_repo;
pub mod invitation_repo;
pub mod skill_repo;
pub mod snippet_repo;
pub mod tenant_repo;
pub mod tool_repo;
pub mod user_repo;

// PostgreSQL repositories
pub mod pg_api_key_repo;
pub mod pg_audit_repo;
pub mod pg_invitation_repo;
pub mod pg_skill_repo;
pub mod pg_snippet_repo;
pub mod pg_tenant_repo;
pub mod pg_tool_repo;
pub mod pg_user_repo;

// Database drivers
pub mod mysql;
pub mod pool;
pub mod postgres;
pub mod sqlite;

// SQLite re-exports
pub use api_key_repo::SqliteApiKeyRepository;
pub use audit_repo::SqliteAuditRepository;
pub use invitation_repo::SqliteInvitationRepository;
pub use pool::create_pool;
pub use skill_repo::SqliteSkillRepository;
pub use snippet_repo::SqliteSnippetRepository;
pub use tenant_repo::SqliteTenantRepository;
pub use tool_repo::SqliteToolRepository;
pub use user_repo::SqliteUserRepository;

// PostgreSQL re-exports
pub use pg_api_key_repo::PgApiKeyRepository;
pub use pg_audit_repo::PgAuditRepository;
pub use pg_invitation_repo::PgInvitationRepository;
pub use pg_skill_repo::PgSkillRepository;
pub use pg_snippet_repo::PgSnippetRepository;
pub use pg_tenant_repo::PgTenantRepository;
pub use pg_tool_repo::PgToolRepository;
pub use pg_user_repo::PgUserRepository;
