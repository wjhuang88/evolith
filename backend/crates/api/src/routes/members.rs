//! Member management routes
//! Handles tenant member endpoints

use crate::handlers::members;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/members")
            // Get members list - requires tenant membership
            .route("", web::get().to(members::list_members))
            // Invite new member - requires admin or owner
            .route("/invite", web::post().to(members::invite_member))
            // Get pending invitations - requires tenant membership
            .route("/invitations", web::get().to(members::list_invitations))
            // Remove member - requires admin or owner
            .route("/{member_id}", web::delete().to(members::remove_member))
            // Accept invitation (public)
            .route("/join", web::post().to(members::accept_invitation)),
    );
}
