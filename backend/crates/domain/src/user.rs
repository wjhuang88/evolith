//! User domain model

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub role: UserRole,
    pub tenant_id: Uuid,
    pub tenant_role: TenantRole,
    pub email_verified: bool,
    pub verify_token: Option<String>,
    pub verified_at: Option<DateTime<Utc>>,
    pub reset_token: Option<String>,
    pub reset_expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    User,
}

/// User role within a tenant
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TenantRole {
    Owner,
    Admin,
    Member,
}

impl Default for TenantRole {
    fn default() -> Self {
        TenantRole::Member
    }
}

impl Default for UserRole {
    fn default() -> Self {
        UserRole::User
    }
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct NewUser {
    #[validate(length(min = 3, max = 64))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateUser {
    #[validate(length(min = 3, max = 64))]
    pub username: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoginResponse {
    pub user: UserInfo,
    pub token: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserInfo {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub role: UserRole,
    pub tenant_id: Uuid,
    pub tenant_role: TenantRole,
    pub email_verified: bool,
}

/// ========== Tenant Invitation Model ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invitation {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub email: String,
    pub role: String,  // admin, member
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub accepted_at: Option<DateTime<Utc>>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    /// Helper to create a valid NewUser for testing
    fn valid_new_user() -> NewUser {
        NewUser {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "TestPass123!".to_string(),
        }
    }

    /// Helper to create a valid LoginRequest for testing
    fn valid_login_request() -> LoginRequest {
        LoginRequest {
            email: "test@example.com".to_string(),
            password: "TestPass123!".to_string(),
        }
    }

    #[test]
    fn test_valid_new_user_passes_validation() {
        let user = valid_new_user();
        assert!(user.validate().is_ok());
    }

    #[test]
    fn test_new_user_username_too_short() {
        let mut user = valid_new_user();
        user.username = "ab".to_string(); // Min 3 chars
        let result = user.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_new_user_username_too_long() {
        let mut user = valid_new_user();
        user.username = "a".repeat(65); // Max 64 chars
        let result = user.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_new_user_invalid_email() {
        let mut user = valid_new_user();
        user.email = "not-an-email".to_string();
        let result = user.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_new_user_password_too_short() {
        let mut user = valid_new_user();
        user.password = "short".to_string(); // Min 8 chars
        let result = user.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_valid_login_request_passes_validation() {
        let req = valid_login_request();
        assert!(req.validate().is_ok());
    }

    #[test]
    fn test_login_request_invalid_email() {
        let mut req = valid_login_request();
        req.email = "invalid".to_string();
        assert!(req.validate().is_err());
    }

    #[test]
    fn test_login_request_empty_password() {
        let mut req = valid_login_request();
        req.password = "".to_string();
        assert!(req.validate().is_err());
    }

    #[test]
    fn test_invitation_creation() {
        let now = Utc::now();
        let invitation = Invitation {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            email: "invitee@example.com".to_string(),
            role: "admin".to_string(),
            token: "test-token-123".to_string(),
            expires_at: now + chrono::Duration::days(7),
            accepted_at: None,
            created_by: Uuid::new_v4(),
            created_at: now,
        };

        assert_eq!(invitation.role, "admin");
        assert!(invitation.accepted_at.is_none());
    }

    #[test]
    fn test_invitation_can_be_accepted() {
        let now = Utc::now();
        let invitation = Invitation {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            email: "invitee@example.com".to_string(),
            role: "member".to_string(),
            token: "test-token-123".to_string(),
            expires_at: now + chrono::Duration::days(7),
            accepted_at: Some(now),
            created_by: Uuid::new_v4(),
            created_at: now,
        };

        assert!(invitation.accepted_at.is_some());
    }

    #[test]
    fn test_tenant_role_default() {
        let role = TenantRole::default();
        assert_eq!(role, TenantRole::Member);
    }

    #[test]
    fn test_user_role_default() {
        let role = UserRole::default();
        assert_eq!(role, UserRole::User);
    }

    #[test]
    fn test_user_info_serialization() {
        let user_info = UserInfo {
            id: Uuid::new_v4(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            role: UserRole::Admin,
            tenant_id: Uuid::new_v4(),
            tenant_role: TenantRole::Owner,
            email_verified: true,
        };

        // Test serialization to JSON
        let json = serde_json::to_string(&user_info).unwrap();
        assert!(json.contains("testuser"));
        assert!(json.contains("owner")); // tenant_role serialized as lowercase
    }

    #[test]
    fn test_login_response_serialization() {
        let login_response = LoginResponse {
            user: UserInfo {
                id: Uuid::new_v4(),
                username: "testuser".to_string(),
                email: "test@example.com".to_string(),
                role: UserRole::User,
                tenant_id: Uuid::new_v4(),
                tenant_role: TenantRole::Member,
                email_verified: false,
            },
            token: "test-token".to_string(),
            expires_at: Utc::now(),
        };

        let json = serde_json::to_string(&login_response).unwrap();
        assert!(json.contains("token"));
        assert!(json.contains("user"));
        assert!(json.contains("expires_at"));
    }
}
