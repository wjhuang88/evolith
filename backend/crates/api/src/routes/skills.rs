//! Skill routes

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/skills", web::get().to(list_skills))
        .route("/skills", web::post().to(create_skill))
        .route("/skills/{id}", web::get().to(get_skill))
        .route("/skills/{id}", web::delete().to(delete_skill))
        .route("/skills/{id}/load", web::get().to(load_skill))
        .route("/skills/{id}/execute", web::post().to(execute_skill));
}

async fn list_skills() -> actix_web::HttpResponse {
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

async fn create_skill() -> actix_web::HttpResponse {
    actix_web::HttpResponse::NotImplemented().json(serde_json::json!({
        "success": false,
        "error": {
            "code": "NOT_IMPLEMENTED",
            "message": "Skill creation not implemented yet"
        }
    }))
}

async fn get_skill() -> actix_web::HttpResponse {
    actix_web::HttpResponse::NotImplemented().json(serde_json::json!({
        "success": false,
        "error": {
            "code": "NOT_IMPLEMENTED",
            "message": "Get skill not implemented yet"
        }
    }))
}

async fn delete_skill() -> actix_web::HttpResponse {
    actix_web::HttpResponse::NotImplemented().json(serde_json::json!({
        "success": false,
        "error": {
            "code": "NOT_IMPLEMENTED",
            "message": "Delete skill not implemented yet"
        }
    }))
}

async fn load_skill() -> actix_web::HttpResponse {
    actix_web::HttpResponse::NotImplemented().json(serde_json::json!({
        "success": false,
        "error": {
            "code": "NOT_IMPLEMENTED",
            "message": "Load skill not implemented yet"
        }
    }))
}

async fn execute_skill() -> actix_web::HttpResponse {
    actix_web::HttpResponse::NotImplemented().json(serde_json::json!({
        "success": false,
        "error": {
            "code": "NOT_IMPLEMENTED",
            "message": "Execute skill not implemented yet"
        }
    }))
}
