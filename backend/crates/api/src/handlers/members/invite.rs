//! Invite member and accept invitation handlers

use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use uuid::Uuid;
use validator::Validate;

use crate::dto::common::ApiResponse;
use crate::dto::member_dto::*;
use crate::middleware::auth::AuthenticatedUser;
use crate::state::AppState;
use domain::repository::NewInvitation;

/// Generate a random invitation token
fn generate_invite_token() -> String {
    format!("inv_{}", Uuid::new_v4().simple())
}

/// Invite a new member to the tenant
pub async fn invite_member(
    tenant_id: web::Path<Uuid>,
    user: AuthenticatedUser,
    body: web::Json<InviteMemberRequest>,
    state: web::Data<AppState>,
) -> impl Responder {
    let tenant_id = tenant_id.into_inner();

    // Verify user has admin access to this tenant
    if user.tenant_id != tenant_id {
        return HttpResponse::Forbidden().json(ApiResponse::<()>::error(
            "FORBIDDEN",
            "You do not have access to this tenant",
        ));
    }

    // Only admins can invite members
    if !user.is_admin() {
        return HttpResponse::Forbidden().json(ApiResponse::<()>::error(
            "FORBIDDEN",
            "Only admins can invite members",
        ));
    }

    // Validate request
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "VALIDATION_ERROR",
            &errors.to_string(),
        ));
    }

    let email = body.email.to_lowercase();
    let role = body.role.clone();

    // Validate role
    if !["admin", "member"].contains(&role.as_str()) {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "INVALID_ROLE",
            "Role must be 'admin' or 'member'",
        ));
    }

    // Check if email is already invited
    match state.invitation_repo.find_by_email(tenant_id, &email).await {
        Ok(Some(_)) => {
            return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                "ALREADY_INVITED",
                "This email is already invited to the tenant",
            ));
        }
        Ok(None) => {}
        Err(e) => {
            tracing::error!("Failed to check existing invitation: {}", e);
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to check existing invitation",
            ));
        }
    }

    // Generate invitation token
    let token = generate_invite_token();
    let expires_at = Utc::now() + chrono::Duration::days(7);

    // Create invitation
    let new_invitation = NewInvitation::new(
        tenant_id,
        email.clone(),
        role.clone(),
        token.clone(),
        expires_at,
        user.user_id,
    );

    match state.invitation_repo.create(new_invitation).await {
        Ok(invitation) => {
            let response = InviteMemberResponse {
                id: invitation.id.to_string(),
                email: invitation.email,
                role: invitation.role,
                invite_url: format!("/join?token={}", invitation.token),
                expires_at: invitation.expires_at.to_rfc3339(),
            };

            HttpResponse::Ok().json(ApiResponse::<InviteMemberResponse>::success(response))
        }
        Err(e) => {
            tracing::error!("Failed to create invitation: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to create invitation",
            ))
        }
    }
}

/// Accept invitation and create account
pub async fn accept_invitation(
    body: web::Json<AcceptInviteRequest>,
    _state: web::Data<AppState>,
) -> impl Responder {
    // Validate request
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "VALIDATION_ERROR",
            &errors.to_string(),
        ));
    }

    // This is a complex operation that requires:
    // 1. Validate invitation token
    // 2. Check if token expired
    // 3. Check if email already registered
    // 4. Create user account with hashed password
    // 5. Mark invitation as accepted
    //
    // For now, return 501 Not Implemented
    HttpResponse::NotImplemented().json(ApiResponse::<()>::error(
        "NOT_IMPLEMENTED",
        "Accept invitation is not yet implemented",
    ))
}
