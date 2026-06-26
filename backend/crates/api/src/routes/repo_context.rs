use actix_web::web;

// C routes are merged into repos::configure to avoid scope shadowing.
pub fn configure(_cfg: &mut web::ServiceConfig) {}
