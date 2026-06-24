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
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TenantPlan {
    #[default]
    Free,
    Starter,
    Pro,
    Enterprise,
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
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PlanStatus {
    #[default]
    Active,
    PastDue,
    Cancelled,
}

/// Tenant status
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TenantStatus {
    #[default]
    Active,
    Suspended,
    Deleted,
}

/// Tenant quotas (limits)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantQuotas {
    pub max_users: u32,
    /// Maximum number of git repos (git-centric primary quota).
    pub max_repos: u32,
    /// Total storage for all git repos in MB.
    pub max_storage_mb: u32,
    pub max_api_calls_per_month: u32,
}

impl Default for TenantQuotas {
    fn default() -> Self {
        Self {
            max_users: 5,
            max_repos: 10,
            max_storage_mb: 500,
            max_api_calls_per_month: 10000,
        }
    }
}

impl TenantQuotas {
    /// Get quotas for a specific plan
    pub fn for_plan(plan: &TenantPlan) -> Self {
        match plan {
            TenantPlan::Free => Self {
                max_users: 3,
                max_repos: 3,
                max_storage_mb: 100,
                max_api_calls_per_month: 1000,
            },
            TenantPlan::Starter => Self {
                max_users: 10,
                max_repos: 20,
                max_storage_mb: 500,
                max_api_calls_per_month: 10000,
            },
            TenantPlan::Pro => Self {
                max_users: 50,
                max_repos: 100,
                max_storage_mb: 5000,
                max_api_calls_per_month: 100000,
            },
            TenantPlan::Enterprise => Self {
                max_users: u32::MAX,
                max_repos: u32::MAX,
                max_storage_mb: u32::MAX,
                max_api_calls_per_month: u32::MAX,
            },
        }
    }
}

/// Tenant usage statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TenantUsage {
    pub current_users: u32,
    pub current_repos: u32,
    pub current_api_calls: u32,
    pub current_storage_mb: u32,
}

/// Request to create a new tenant
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateTenantRequest {
    #[validate(length(min = 1, max = 128))]
    pub name: String,

    #[validate(length(min = 1, max = 64), custom(function = "validate_slug"))]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tenant_plan_default() {
        let plan = TenantPlan::default();
        assert_eq!(plan, TenantPlan::Free);
    }

    #[test]
    fn test_tenant_plan_display() {
        assert_eq!(TenantPlan::Free.to_string(), "free");
        assert_eq!(TenantPlan::Starter.to_string(), "starter");
        assert_eq!(TenantPlan::Pro.to_string(), "pro");
        assert_eq!(TenantPlan::Enterprise.to_string(), "enterprise");
    }

    #[test]
    fn test_plan_status_default() {
        let status = PlanStatus::default();
        assert_eq!(status, PlanStatus::Active);
    }

    #[test]
    fn test_tenant_status_default() {
        let status = TenantStatus::default();
        assert_eq!(status, TenantStatus::Active);
    }

    #[test]
    fn test_tenant_quotas_default() {
        let quotas = TenantQuotas::default();
        assert_eq!(quotas.max_users, 5);
        assert_eq!(quotas.max_repos, 10);
    }

    #[test]
    fn test_tenant_quotas_for_free_plan() {
        let quotas = TenantQuotas::for_plan(&TenantPlan::Free);
        assert_eq!(quotas.max_users, 3);
        assert_eq!(quotas.max_repos, 3);
        assert_eq!(quotas.max_api_calls_per_month, 1000);
    }

    #[test]
    fn test_tenant_quotas_for_starter_plan() {
        let quotas = TenantQuotas::for_plan(&TenantPlan::Starter);
        assert_eq!(quotas.max_users, 10);
        assert_eq!(quotas.max_repos, 20);
    }

    #[test]
    fn test_tenant_quotas_for_pro_plan() {
        let quotas = TenantQuotas::for_plan(&TenantPlan::Pro);
        assert_eq!(quotas.max_users, 50);
        assert_eq!(quotas.max_repos, 100);
        assert_eq!(quotas.max_api_calls_per_month, 100000);
    }

    #[test]
    fn test_tenant_quotas_for_enterprise_plan() {
        let quotas = TenantQuotas::for_plan(&TenantPlan::Enterprise);
        assert_eq!(quotas.max_users, u32::MAX);
        assert_eq!(quotas.max_repos, u32::MAX);
    }

    #[test]
    fn test_tenant_context_default() {
        let ctx = TenantContext::default_tenant();
        assert_eq!(ctx.tenant_id, Uuid::nil());
        assert_eq!(ctx.tenant_slug, "default");
        assert_eq!(ctx.tenant_name, "Default Tenant");
        assert_eq!(ctx.tenant_plan, TenantPlan::Pro);
        assert_eq!(ctx.tenant_status, TenantStatus::Active);
    }

    #[test]
    fn test_tenant_serialization() {
        let tenant = Tenant {
            id: Uuid::new_v4(),
            name: "Test Company".to_string(),
            slug: "test-company".to_string(),
            domain: Some("test.com".to_string()),
            owner_id: Uuid::new_v4(),
            billing_email: Some("billing@test.com".to_string()),
            plan: TenantPlan::Pro,
            plan_status: PlanStatus::Active,
            quotas: TenantQuotas::default(),
            usage: TenantUsage::default(),
            settings: serde_json::json!({"key": "value"}),
            features: serde_json::json!(["feature1", "feature2"]),
            status: TenantStatus::Active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        let json = serde_json::to_string(&tenant).unwrap();
        assert!(json.contains("Test Company"));
        assert!(json.contains("pro"));
    }

    #[test]
    fn test_tenant_invitation_serialization() {
        let invitation = TenantInvitation {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            email: "user@example.com".to_string(),
            role: TenantRole::Admin,
            token: "invite-token-123".to_string(),
            expires_at: Utc::now() + chrono::Duration::days(7),
            accepted_at: None,
            created_by: Uuid::new_v4(),
            created_at: Utc::now(),
        };

        let json = serde_json::to_string(&invitation).unwrap();
        assert!(json.contains("user@example.com"));
        assert!(json.contains("admin"));
    }

    #[test]
    fn test_invite_user_request_validation() {
        let req = InviteUserRequest {
            email: "newuser@example.com".to_string(),
            role: TenantRole::Member,
        };

        assert!(req.validate().is_ok());
    }

    #[test]
    fn test_invite_user_request_invalid_email() {
        let req = InviteUserRequest {
            email: "invalid-email".to_string(),
            role: TenantRole::Member,
        };

        assert!(req.validate().is_err());
    }

    #[test]
    fn test_create_tenant_request_validation() {
        let req = CreateTenantRequest {
            name: "Test Company".to_string(),
            slug: "test-company".to_string(),
            owner_email: "owner@example.com".to_string(),
            owner_username: "owner".to_string(),
            owner_password: "Password123!".to_string(),
        };

        assert!(req.validate().is_ok());
    }

    #[test]
    fn test_create_tenant_request_invalid_slug() {
        let req = CreateTenantRequest {
            name: "Test Company".to_string(),
            slug: "Test_Company".to_string(), // Invalid: contains underscore
            owner_email: "owner@example.com".to_string(),
            owner_username: "owner".to_string(),
            owner_password: "Password123!".to_string(),
        };

        assert!(req.validate().is_err());
    }

    #[test]
    fn test_create_tenant_request_slug_starts_with_hyphen() {
        let req = CreateTenantRequest {
            name: "Test Company".to_string(),
            slug: "-test-company".to_string(), // Invalid: starts with hyphen
            owner_email: "owner@example.com".to_string(),
            owner_username: "owner".to_string(),
            owner_password: "Password123!".to_string(),
        };

        assert!(req.validate().is_err());
    }
}
