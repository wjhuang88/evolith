//! Member management handlers
//! Handles tenant member invitation, role management, and removal

use std::sync::{Arc, Mutex};

use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::dto::common::ApiResponse;
use crate::dto::member_dto::*;

/// In-memory invitation store for development
pub struct InvitationStore {
    invitations: Mutex<Vec<StoredInvitation>>,
}

#[derive(Clone)]
pub struct StoredInvitation {
    pub id: String,
    pub tenant_id: String,
    pub email: String,
    pub role: String,
    pub token: String,
    pub expires_at: i64,
    pub accepted_at: Option<i64>,
    pub created_by: String,
    pub created_at: i64,
}

impl Default for InvitationStore {
    fn default() -> Self {
        Self {
            invitations: Mutex::new(Vec::new()),
        }
    }
}

/// Member state
#[derive(Clone)]
pub struct MemberState {
    pub invitation_store: Arc<InvitationStore>,
}

impl MemberState {
    pub fn new() -> Self {
        Self {
            invitation_store: Arc::new(InvitationStore::default()),
        }
    }
}

impl Default for MemberState {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple token generation
fn generate_invite_token() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let random: u64 = {
        use std::collections::hash_map::RandomState;
        use std::hash::{BuildHasher, Hasher};
        RandomState::new().build_hasher().finish()
    };
    format!("inv_{:x}{:016x}", timestamp, random)
}

/// Get tenant members
pub async fn get_members(
    _tenant_id: web::Path<String>,
    _state: web::Data<MemberState>,
) -> impl Responder {
    // In production, fetch from database
    // For demo, return mock data structure
    HttpResponse::Ok().json(ApiResponse::<MemberListResponse>::success(
        MemberListResponse {
            members: vec![],
            total: 0,
        },
    ))
}

/// Invite a new member to the tenant
pub async fn invite_member(
    body: web::Json<InviteMemberRequest>,
    _state: web::Data<MemberState>,
) -> impl Responder {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "VALIDATION_ERROR",
            &errors.to_string(),
        ));
    }

    let email = body.email.to_lowercase();
    let role = body.role.clone();
    let message = body.message.clone();

    // Generate invitation token
    let token = generate_invite_token();
    let now = Utc::now().timestamp();
    let expires_7d = now + (7 * 24 * 60 * 60); // 7 days

    // In production:
    // 1. Check if user already exists in tenant
    // 2. Create invitation record in database
    // 3. Send invitation email

    // For demo, return the invitation info
    let response = InviteMemberResponse {
        id: Uuid::new_v4().to_string(),
        email: email.clone(),
        role: role.clone(),
        invite_url: format!("/join?token={}", token),
        expires_at: chrono::DateTime::from_timestamp(expires_7d, 0)
            .unwrap_or_default()
            .to_rfc3339(),
    };

    // Log for demo
    println!(
        "[DEMO] Invitation sent to {} with role '{}'. Message: {:?}",
        email, role, message
    );

    HttpResponse::Ok().json(ApiResponse::<InviteMemberResponse>::success(response))
}

/// Accept invitation and create account
pub async fn accept_invitation(
    body: web::Json<AcceptInviteRequest>,
    _state: web::Data<MemberState>,
) -> impl Responder {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "VALIDATION_ERROR",
            &errors.to_string(),
        ));
    }

    let token = body.token.clone();
    let password = body.password.clone();
    let username = body.username.clone();

    // In production:
    // 1. Validate invitation token
    // 2. Check if token expired
    // 3. Check if email already registered
    // 4. Create user account
    // 5. Mark invitation as accepted

    // For demo, return success
    HttpResponse::Ok().json(ApiResponse::<serde_json::Value>::success(
        serde_json::json!({
            "message": "Account created successfully",
            "username": username
        }),
    ))
}

/// Update member role
pub async fn update_member_role(
    _tenant_id: web::Path<String>,
    _member_id: web::Path<String>,
    body: web::Json<UpdateMemberRoleRequest>,
    _state: web::Data<MemberState>,
) -> impl Responder {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "VALIDATION_ERROR",
            &errors.to_string(),
        ));
    }

    let new_role = body.role.clone();

    // Validate role
    if !["admin", "member", "viewer"].contains(&new_role.as_str()) {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "INVALID_ROLE",
            "Role must be admin, member, or viewer",
        ));
    }

    // In production:
    // 1. Check permissions (only owner/admin can change roles)
    // 2. Cannot change owner's role
    // 3. Update user's tenant_role in database

    HttpResponse::Ok().json(ApiResponse::<serde_json::Value>::success(
        serde_json::json!({
            "message": "Role updated successfully",
            "role": new_role
        }),
    ))
}

/// Remove member from tenant
pub async fn remove_member(
    _tenant_id: web::Path<String>,
    _member_id: web::Path<String>,
    _state: web::Data<MemberState>,
) -> impl Responder {
    // In production:
    // 1. Check permissions (only owner/admin can remove)
    // 2. Cannot remove owner
    // 3. Delete user or update to unlinked state

    HttpResponse::Ok().json(ApiResponse::<serde_json::Value>::success(
        serde_json::json!({
            "message": "Member removed successfully"
        }),
    ))
}

/// Get pending invitations
pub async fn get_invitations(
    _tenant_id: web::Path<String>,
    _state: web::Data<MemberState>,
) -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::<InvitationListResponse>::success(
        InvitationListResponse {
            invitations: vec![],
            total: 0,
        },
    ))
}

/// Cancel invitation
pub async fn cancel_invitation(
    _tenant_id: web::Path<String>,
    _invitation_id: web::Path<String>,
    _state: web::Data<MemberState>,
) -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::<serde_json::Value>::success(
        serde_json::json!({
            "message": "Invitation cancelled successfully"
        }),
    ))
}
