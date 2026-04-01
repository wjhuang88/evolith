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

    // Verify user has access to this tenant
    if user.tenant_id != tenant_id {
        return HttpResponse::Forbidden().json(ApiResponse::<()>::error(
            "FORBIDDEN",
            "You do not have access to this tenant",
        ));
    }

    // Note: UserRepository doesn't have find_by_tenant method yet
    // For now, return empty list until the repository trait is updated
    // In production, this would call: state.user_repo.find_by_tenant(tenant_id).await

    HttpResponse::Ok().json(ApiResponse::<MemberListResponse>::success(
        MemberListResponse {
            members: vec![],
            total: 0,
        },
    ))
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
