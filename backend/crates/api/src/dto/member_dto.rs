//! Member management DTOs
//! Defines request/response structures for tenant member management

use serde::{Deserialize, Serialize};
use validator::Validate;

/// Invite member request
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct InviteMemberRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 1, max = 50))]
    pub role: String,

    /// Optional message to include in the invitation
    pub message: Option<String>,
}

/// Accept invitation request
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct AcceptInviteRequest {
    pub token: String,

    #[validate(length(min = 8, max = 128))]
    pub password: String,

    /// Username for the new user
    #[validate(length(min = 3, max = 50))]
    pub username: String,
}

/// Update member role request
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateMemberRoleRequest {
    #[validate(length(min = 1, max = 50))]
    pub role: String,
}

/// Member info response
#[derive(Debug, Serialize)]
pub struct MemberInfo {
    pub id: String,
    pub email: String,
    pub username: String,
    pub full_name: Option<String>,
    pub role: String,
    pub tenant_role: String,
    pub status: String,
    pub joined_at: Option<String>,
    pub last_login_at: Option<String>,
    pub avatar_url: Option<String>,
}

/// Invitation info response
#[derive(Debug, Serialize)]
pub struct InvitationInfo {
    pub id: String,
    pub email: String,
    pub role: String,
    pub status: String,
    pub invited_by: String,
    pub expires_at: String,
    pub created_at: String,
}

/// Invite member response
#[derive(Debug, Serialize)]
pub struct InviteMemberResponse {
    pub id: String,
    pub email: String,
    pub role: String,
    pub invite_url: String,
    pub expires_at: String,
}

/// Member list response
#[derive(Debug, Serialize)]
pub struct MemberListResponse {
    pub members: Vec<MemberInfo>,
    pub total: usize,
}

/// Invitation list response
#[derive(Debug, Serialize)]
pub struct InvitationListResponse {
    pub invitations: Vec<InvitationInfo>,
    pub total: usize,
}
