//! Member management routes
//! Handles tenant member endpoints

use crate::handlers::member_handlers::*;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/members")
            // Get members list
            .route("", web::get().to(get_members))
            // Invite new member
            .route("/invite", web::post().to(invite_member))
            // Get pending invitations
            .route("/invitations", web::get().to(get_invitations))
            // Accept invitation (public)
            .route("/join", web::post().to(accept_invitation)),
    );
}
