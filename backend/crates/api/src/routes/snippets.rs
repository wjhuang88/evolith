//! Snippet routes

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/snippets", web::get().to(list_snippets))
        .route("/snippets", web::post().to(create_snippet))
        .route("/snippets/search", web::get().to(search_snippets))
        .route("/snippets/{id}", web::get().to(get_snippet))
        .route("/snippets/{id}", web::delete().to(delete_snippet))
        .route("/snippets/{id}/reference", web::get().to(get_reference));
}

async fn list_snippets() -> actix_web::HttpResponse {
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

async fn create_snippet() -> actix_web::HttpResponse {
    actix_web::HttpResponse::NotImplemented().json(serde_json::json!({
        "success": false,
        "error": {
            "code": "NOT_IMPLEMENTED",
            "message": "Snippet creation not implemented yet"
        }
    }))
}

async fn search_snippets() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": [],
        "meta": {
            "query": "",
            "total": 0
        }
    }))
}

async fn get_snippet() -> actix_web::HttpResponse {
    actix_web::HttpResponse::NotImplemented().json(serde_json::json!({
        "success": false,
        "error": {
            "code": "NOT_IMPLEMENTED",
            "message": "Get snippet not implemented yet"
        }
    }))
}

async fn delete_snippet() -> actix_web::HttpResponse {
    actix_web::HttpResponse::NotImplemented().json(serde_json::json!({
        "success": false,
        "error": {
            "code": "NOT_IMPLEMENTED",
            "message": "Delete snippet not implemented yet"
        }
    }))
}

async fn get_reference() -> actix_web::HttpResponse {
    actix_web::HttpResponse::NotImplemented().json(serde_json::json!({
        "success": false,
        "error": {
            "code": "NOT_IMPLEMENTED",
            "message": "Get reference not implemented yet"
        }
    }))
}
