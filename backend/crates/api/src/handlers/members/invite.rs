//! Invite member and accept invitation handlers

use actix_web::cookie::{time::Duration, Cookie, SameSite};
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use uuid::Uuid;
use validator::Validate;

use crate::dto::auth_dto::{AuthResponseData, TenantInfo, UserInfo};
use crate::dto::common::ApiResponse;
use crate::dto::member_dto::*;
use crate::middleware::auth::AuthenticatedUser;
use crate::middleware::csrf::CSRF_COOKIE_NAME;
use crate::middleware::rbac::JWT_COOKIE_NAME;
use crate::state::AppState;
use domain::repository::NewInvitation;
use domain::user::{NewUser, TenantRole};

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
            let tenant_name = match state.tenant_repo.find_by_id(tenant_id).await {
                Ok(Some(tenant)) => tenant.name,
                Ok(None) => "your workspace".to_string(),
                Err(e) => {
                    tracing::warn!("Failed to load tenant for invitation email: {}", e);
                    "your workspace".to_string()
                }
            };
            let inviter_name = match state.user_repo.find_by_id(user.user_id).await {
                Ok(Some(inviter)) => inviter.username,
                Ok(None) => "A workspace admin".to_string(),
                Err(e) => {
                    tracing::warn!("Failed to load inviter for invitation email: {}", e);
                    "A workspace admin".to_string()
                }
            };
            let public_url = state.config.app.public_url.trim_end_matches('/');
            if let Err(e) = state
                .mailer
                .send_invitation_email(
                    &invitation.email,
                    &inviter_name,
                    &tenant_name,
                    &invitation.token,
                    public_url,
                )
                .await
            {
                tracing::warn!("Failed to send invitation email: {}", e);
            }

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
    state: web::Data<AppState>,
) -> impl Responder {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "VALIDATION_ERROR",
            &errors.to_string(),
        ));
    }

    let invitation = match state.invitation_repo.find_by_token(&body.token).await {
        Ok(Some(inv)) => inv,
        Ok(None) => {
            return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                "INVALID_TOKEN",
                "Invalid invitation token",
            ));
        }
        Err(e) => {
            tracing::error!("Failed to find invitation: {}", e);
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to accept invitation",
            ));
        }
    };

    if invitation.accepted_at.is_some() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "TOKEN_USED",
            "Invitation has already been used",
        ));
    }

    if Utc::now() > invitation.expires_at {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "TOKEN_EXPIRED",
            "Invitation has expired",
        ));
    }

    match state.user_repo.find_by_email(&invitation.email).await {
        Ok(Some(_)) => {
            return HttpResponse::Conflict().json(ApiResponse::<()>::error(
                "EMAIL_EXISTS",
                "An account with this email already exists",
            ));
        }
        Ok(None) => {}
        Err(e) => {
            tracing::error!("Failed to check invited user email: {}", e);
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to accept invitation",
            ));
        }
    }

    if let Err(e) = state.hasher.validate_strength(&body.password) {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "WEAK_PASSWORD",
            &format!("Password does not meet requirements: {}", e),
        ));
    }

    let password_hash = match state.hasher.hash_password(&body.password) {
        Ok(hash) => hash,
        Err(e) => {
            tracing::error!("Failed to hash password: {}", e);
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to accept invitation",
            ));
        }
    };

    let new_user = NewUser {
        username: body.username.clone(),
        email: invitation.email.clone(),
        password: password_hash,
    };

    let tenant_role = match invitation.role.as_str() {
        "admin" => TenantRole::Admin,
        _ => TenantRole::Member,
    };

    let user = match state
        .user_repo
        .create(new_user, invitation.tenant_id, tenant_role)
        .await
    {
        Ok(user) => user,
        Err(e) => {
            tracing::error!("Failed to create user from invitation: {}", e);
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to create account",
            ));
        }
    };

    if let Err(e) = state.invitation_repo.accept(invitation.id).await {
        tracing::warn!("Failed to mark invitation as accepted: {}", e);
    }

    let tenant = match state.tenant_repo.find_by_id(invitation.tenant_id).await {
        Ok(Some(t)) => t,
        Ok(None) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Tenant not found",
            ));
        }
        Err(e) => {
            tracing::error!("Failed to find tenant: {}", e);
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to accept invitation",
            ));
        }
    };

    let role_str = serde_json::to_value(&user.role)
        .ok()
        .and_then(|v| v.as_str().map(String::from))
        .unwrap_or_else(|| "user".to_string());
    let tenant_role_str = serde_json::to_value(&user.tenant_role)
        .ok()
        .and_then(|v| v.as_str().map(String::from))
        .unwrap_or_else(|| "member".to_string());

    let (token, expires_at) =
        match state
            .jwt
            .generate_token(user.id, &role_str, tenant.id, &tenant_role_str)
        {
            Ok(result) => result,
            Err(e) => {
                tracing::error!("Failed to generate token: {}", e);
                return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                    "INTERNAL_ERROR",
                    "Account created but login failed",
                ));
            }
        };

    let is_production = state.config.is_production();
    let csrf_token = Uuid::new_v4().to_string();

    let jwt_cookie = Cookie::build(JWT_COOKIE_NAME, &token)
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .secure(is_production)
        .max_age(Duration::seconds(
            expires_at - chrono::Utc::now().timestamp(),
        ))
        .finish();

    let csrf_cookie = Cookie::build(CSRF_COOKIE_NAME, &csrf_token)
        .path("/")
        .http_only(false)
        .same_site(SameSite::Lax)
        .secure(is_production)
        .max_age(Duration::seconds(
            expires_at - chrono::Utc::now().timestamp(),
        ))
        .finish();

    HttpResponse::Created()
        .cookie(jwt_cookie)
        .cookie(csrf_cookie)
        .json(ApiResponse::success(AuthResponseData {
            token,
            expires_at,
            user: UserInfo {
                id: user.id.to_string(),
                email: user.email,
                username: user.username,
                role: role_str,
                tenant_id: tenant.id.to_string(),
                tenant_role: tenant_role_str,
                email_verified: user.email_verified,
            },
            tenant: TenantInfo {
                id: tenant.id.to_string(),
                name: tenant.name,
                slug: tenant.slug,
                plan: format!("{}", tenant.plan),
            },
        }))
}
