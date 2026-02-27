//! Tenant domain model

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// Re-export TenantRole from user module
pub use crate::user::TenantRole;

/// Tenant entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tenant {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub domain: Option<String>,
    pub owner_id: Uuid,
    pub billing_email: Option<String>,
    pub plan: TenantPlan,
    pub plan_status: PlanStatus,
    pub quotas: TenantQuotas,
    pub usage: TenantUsage,
    pub settings: serde_json::Value,
    pub features: serde_json::Value,
    pub status: TenantStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Tenant subscription plan
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TenantPlan {
    Free,
    Starter,
    Pro,
    Enterprise,
}

impl Default for TenantPlan {
    fn default() -> Self {
        TenantPlan::Free
    }
}

impl std::fmt::Display for TenantPlan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TenantPlan::Free => write!(f, "free"),
            TenantPlan::Starter => write!(f, "starter"),
            TenantPlan::Pro => write!(f, "pro"),
            TenantPlan::Enterprise => write!(f, "enterprise"),
        }
    }
}

/// Plan status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PlanStatus {
    Active,
    PastDue,
    Cancelled,
}

impl Default for PlanStatus {
    fn default() -> Self {
        PlanStatus::Active
    }
}

/// Tenant status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TenantStatus {
    Active,
    Suspended,
    Deleted,
}

impl Default for TenantStatus {
    fn default() -> Self {
        TenantStatus::Active
    }
}

/// Tenant quotas (limits)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantQuotas {
    pub max_users: u32,
    pub max_tools: u32,
    pub max_skills: u32,
    pub max_snippets: u32,
    pub max_api_calls_per_month: u32,
    pub max_storage_mb: u32,
}

impl Default for TenantQuotas {
    fn default() -> Self {
        Self {
            max_users: 5,
            max_tools: 10,
            max_skills: 20,
            max_snippets: 100,
            max_api_calls_per_month: 10000,
            max_storage_mb: 500,
        }
    }
}

impl TenantQuotas {
    /// Get quotas for a specific plan
    pub fn for_plan(plan: &TenantPlan) -> Self {
        match plan {
            TenantPlan::Free => Self {
                max_users: 3,
                max_tools: 5,
                max_skills: 10,
                max_snippets: 50,
                max_api_calls_per_month: 1000,
                max_storage_mb: 100,
            },
            TenantPlan::Starter => Self {
                max_users: 10,
                max_tools: 20,
                max_skills: 50,
                max_snippets: 200,
                max_api_calls_per_month: 10000,
                max_storage_mb: 500,
            },
            TenantPlan::Pro => Self {
                max_users: 50,
                max_tools: 100,
                max_skills: 200,
                max_snippets: 1000,
                max_api_calls_per_month: 100000,
                max_storage_mb: 5000,
            },
            TenantPlan::Enterprise => Self {
                max_users: u32::MAX,
                max_tools: u32::MAX,
                max_skills: u32::MAX,
                max_snippets: u32::MAX,
                max_api_calls_per_month: u32::MAX,
                max_storage_mb: u32::MAX,
            },
        }
    }
}

/// Tenant usage statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TenantUsage {
    pub current_users: u32,
    pub current_tools: u32,
    pub current_skills: u32,
    pub current_snippets: u32,
    pub current_api_calls: u32,
    pub current_storage_mb: u32,
}

/// Request to create a new tenant
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateTenantRequest {
    #[validate(length(min = 1, max = 128))]
    pub name: String,

    #[validate(length(min = 1, max = 64), custom = "validate_slug")]
    pub slug: String,

    #[validate(email)]
    pub owner_email: String,

    #[validate(length(min = 3, max = 64))]
    pub owner_username: String,

    #[validate(length(min = 8))]
    pub owner_password: String,
}

/// Validate slug format (lowercase letters, numbers, and hyphens)
fn validate_slug(slug: &str) -> Result<(), validator::ValidationError> {
    if slug.is_empty() || slug.len() > 64 {
        return Err(validator::ValidationError::new("length"));
    }
    for c in slug.chars() {
        if !c.is_ascii_lowercase() && !c.is_ascii_digit() && c != '-' {
            return Err(validator::ValidationError::new("format"));
        }
    }
    if slug.starts_with('-') || slug.ends_with('-') {
        return Err(validator::ValidationError::new("format"));
    }
    Ok(())
}

/// Tenant invitation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantInvitation {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub email: String,
    pub role: TenantRole,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub accepted_at: Option<DateTime<Utc>>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
}

/// Request to invite a user to tenant
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct InviteUserRequest {
    #[validate(email)]
    pub email: String,
    pub role: TenantRole,
}

/// Tenant context extracted from request
#[derive(Debug, Clone)]
pub struct TenantContext {
    pub tenant_id: Uuid,
    pub tenant_slug: String,
    pub tenant_name: String,
    pub tenant_plan: TenantPlan,
    pub tenant_status: TenantStatus,
}

impl TenantContext {
    /// Create a default tenant context for development
    pub fn default_tenant() -> Self {
        Self {
            tenant_id: Uuid::nil(),
            tenant_slug: "default".to_string(),
            tenant_name: "Default Tenant".to_string(),
            tenant_plan: TenantPlan::Pro,
            tenant_status: TenantStatus::Active,
        }
    }
}
