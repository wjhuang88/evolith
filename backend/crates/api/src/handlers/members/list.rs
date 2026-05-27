//! List members and invitations handlers

use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use uuid::Uuid;

use crate::dto::common::ApiResponse;
use crate::dto::member_dto::*;
use crate::middleware::auth::AuthenticatedUser;
use crate::state::AppState;

/// Get tenant members
pub async fn list_members(
    tenant_id: web::Path<Uuid>,
    user: AuthenticatedUser,
    state: web::Data<AppState>,
) -> impl Responder {
    let tenant_id = tenant_id.into_inner();

    if user.tenant_id != tenant_id {
        return HttpResponse::Forbidden().json(ApiResponse::<()>::error(
            "FORBIDDEN",
            "You do not have access to this tenant",
        ));
    }

    match state.user_repo.find_by_tenant(tenant_id).await {
        Ok(users) => {
            let members: Vec<MemberInfo> = users
                .into_iter()
                .map(|u| MemberInfo {
                    id: u.id.to_string(),
                    email: u.email,
                    username: u.username,
                    full_name: None,
                    role: match u.role {
                        domain::user::UserRole::Admin => "admin".to_string(),
                        domain::user::UserRole::User => "user".to_string(),
                    },
                    tenant_role: match u.tenant_role {
                        domain::user::TenantRole::Owner => "owner".to_string(),
                        domain::user::TenantRole::Admin => "admin".to_string(),
                        domain::user::TenantRole::Member => "member".to_string(),
                    },
                    status: if u.email_verified {
                        "active".to_string()
                    } else {
                        "pending".to_string()
                    },
                    joined_at: Some(u.created_at.to_rfc3339()),
                    last_login_at: Some(u.updated_at.to_rfc3339()),
                    avatar_url: None,
                })
                .collect();
            let total = members.len();
            HttpResponse::Ok().json(ApiResponse::<MemberListResponse>::success(
                MemberListResponse { members, total },
            ))
        }
        Err(e) => {
            tracing::error!("Failed to list members: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to list members",
            ))
        }
    }
}

/// Get pending invitations for a tenant
pub async fn list_invitations(
    tenant_id: web::Path<Uuid>,
    user: AuthenticatedUser,
    state: web::Data<AppState>,
) -> impl Responder {
    let tenant_id = tenant_id.into_inner();

    // Verify user has access to this tenant
    if user.tenant_id != tenant_id {
        return HttpResponse::Forbidden().json(ApiResponse::<()>::error(
            "FORBIDDEN",
            "You do not have access to this tenant",
        ));
    }

    // Fetch invitations from repository
    match state.invitation_repo.find_by_tenant(tenant_id).await {
        Ok(invitations) => {
            let invitation_list: Vec<InvitationInfo> = invitations
                .into_iter()
                .filter(|i| i.accepted_at.is_none()) // Only pending invitations
                .map(|i| InvitationInfo {
                    id: i.id.to_string(),
                    email: i.email,
                    role: i.role,
                    status: if i.expires_at < Utc::now() {
                        "expired"
                    } else {
                        "pending"
                    }
                    .to_string(),
                    invited_by: i.created_by.to_string(),
                    expires_at: i.expires_at.to_rfc3339(),
                    created_at: i.created_at.to_rfc3339(),
                })
                .collect();

            let total = invitation_list.len();
            HttpResponse::Ok().json(ApiResponse::<InvitationListResponse>::success(
                InvitationListResponse {
                    invitations: invitation_list,
                    total,
                },
            ))
        }
        Err(e) => {
            tracing::error!("Failed to list invitations: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to list invitations",
            ))
        }
    }
}
