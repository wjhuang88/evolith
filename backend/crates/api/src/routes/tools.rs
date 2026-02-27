//! Tool routes

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/tools", web::get().to(list_tools))
        .route("/tools", web::post().to(create_tool))
        .route("/tools/{id}", web::get().to(get_tool))
        .route("/tools/{id}", web::put().to(update_tool))
        .route("/tools/{id}", web::delete().to(delete_tool));
}

async fn list_tools() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": [],
        "meta": {
            "page": 1,
            "per_page": 20,
            "total": 0
        }
    }))
}

async fn create_tool() -> actix_web::HttpResponse {
    actix_web::HttpResponse::NotImplemented().json(serde_json::json!({
        "success": false,
        "error": {
            "code": "NOT_IMPLEMENTED",
            "message": "Tool creation not implemented yet"
        }
    }))
}

async fn get_tool() -> actix_web::HttpResponse {
    actix_web::HttpResponse::NotImplemented().json(serde_json::json!({
        "success": false,
        "error": {
            "code": "NOT_IMPLEMENTED",
            "message": "Get tool not implemented yet"
        }
    }))
}

async fn update_tool() -> actix_web::HttpResponse {
    actix_web::HttpResponse::NotImplemented().json(serde_json::json!({
        "success": false,
        "error": {
            "code": "NOT_IMPLEMENTED",
            "message": "Update tool not implemented yet"
        }
    }))
}

async fn delete_tool() -> actix_web::HttpResponse {
    actix_web::HttpResponse::NotImplemented().json(serde_json::json!({
        "success": false,
        "error": {
            "code": "NOT_IMPLEMENTED",
            "message": "Delete tool not implemented yet"
        }
    }))
}
