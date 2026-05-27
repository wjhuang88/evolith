//! Member management handlers

use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

use crate::dto::common::ApiResponse;
use crate::middleware::auth::AuthenticatedUser;
use crate::state::AppState;

/// Remove member from tenant
pub async fn remove_member(
    path: web::Path<(Uuid, Uuid)>,
    user: AuthenticatedUser,
    state: web::Data<AppState>,
) -> impl Responder {
    let (tenant_id, member_id) = path.into_inner();

    if user.tenant_id != tenant_id {
        return HttpResponse::Forbidden().json(ApiResponse::<()>::error(
            "FORBIDDEN",
            "You do not have access to this tenant",
        ));
    }

    if !user.is_admin() {
        return HttpResponse::Forbidden().json(ApiResponse::<()>::error(
            "FORBIDDEN",
            "Only admins can remove members",
        ));
    }

    if member_id == user.user_id {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "BAD_REQUEST",
            "Cannot remove yourself from the tenant",
        ));
    }

    match state.user_repo.remove_from_tenant(member_id).await {
        Ok(()) => HttpResponse::Ok().json(ApiResponse::<serde_json::Value>::success(
            serde_json::json!({
                "message": "Member removed successfully"
            }),
        )),
        Err(e) => {
            tracing::error!("Failed to remove member: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "INTERNAL_ERROR",
                "Failed to remove member",
            ))
        }
    }
}
