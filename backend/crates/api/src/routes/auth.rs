//! Auth routes

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/auth/register", web::post().to(register))
        .route("/auth/login", web::post().to(login))
        .route("/auth/refresh", web::post().to(refresh_token));
}

async fn register() -> actix_web::HttpResponse {
    actix_web::HttpResponse::NotImplemented().json(serde_json::json!({
        "success": false,
        "error": {
            "code": "NOT_IMPLEMENTED",
            "message": "Registration not implemented yet"
        }
    }))
}

async fn login() -> actix_web::HttpResponse {
    actix_web::HttpResponse::NotImplemented().json(serde_json::json!({
        "success": false,
        "error": {
            "code": "NOT_IMPLEMENTED",
            "message": "Login not implemented yet"
        }
    }))
}

async fn refresh_token() -> actix_web::HttpResponse {
    actix_web::HttpResponse::NotImplemented().json(serde_json::json!({
        "success": false,
        "error": {
            "code": "NOT_IMPLEMENTED",
            "message": "Token refresh not implemented yet"
        }
    }))
}
