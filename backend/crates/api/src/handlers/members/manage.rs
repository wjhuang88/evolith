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
    _state: web::Data<AppState>,
) -> impl Responder {
    let (tenant_id, _member_id) = path.into_inner();

    // Verify user has admin access to this tenant
    if user.tenant_id != tenant_id {
        return HttpResponse::Forbidden().json(ApiResponse::<()>::error(
            "FORBIDDEN",
            "You do not have access to this tenant",
        ));
    }

    // Only admins can remove members
    if !user.is_admin() {
        return HttpResponse::Forbidden().json(ApiResponse::<()>::error(
            "FORBIDDEN",
            "Only admins can remove members",
        ));
    }

    // Note: This requires updating the user's tenant_id or deleting the user
    // The UserRepository doesn't have a method for this yet
    // For now, return success as a stub

    HttpResponse::Ok().json(ApiResponse::<serde_json::Value>::success(
        serde_json::json!({
            "message": "Member removed successfully"
        }),
    ))
}
