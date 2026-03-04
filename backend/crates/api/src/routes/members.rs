//! Member management routes
//! Handles tenant member endpoints

use crate::handlers::member_handlers::*;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/members")
            // Get members list - requires tenant membership
            .route("", web::get().to(get_members))
            // Invite new member - requires admin or owner
            .route("/invite", web::post().to(invite_member))
            // Get pending invitations - requires tenant membership
            .route("/invitations", web::get().to(get_invitations))
            // Cancel invitation - requires admin or owner
            .route(
                "/invitations/{invitation_id}",
                web::delete().to(cancel_invitation),
            )
            // Update member role - requires admin or owner
            .route("/{member_id}/role", web::patch().to(update_member_role))
            // Remove member - requires admin or owner
            .route("/{member_id}", web::delete().to(remove_member))
            // Transfer ownership - requires owner only (TODO: implement)
            // .route("/transfer-ownership", web::post().to(transfer_ownership))
            // Accept invitation (public)
            .route("/join", web::post().to(accept_invitation)),
    );
}
