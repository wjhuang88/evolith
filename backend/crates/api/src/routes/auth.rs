//! Auth routes
//! Handles user authentication endpoints

use crate::handlers::auth_handlers::*;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            // Authentication
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/logout", web::post().to(logout))
            .route("/refresh", web::post().to(refresh_token))
            .route("/me", web::get().to(get_current_user))
            // Email verification
            .route("/send-verify", web::post().to(send_verification_email))
            .route("/verify-email", web::post().to(verify_email))
            // Password reset
            .route("/forgot-password", web::post().to(forgot_password))
            .route("/reset-password", web::post().to(reset_password)),
    );
}
