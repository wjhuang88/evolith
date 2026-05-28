use actix_web::http::header;
use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web_rust_embed_responder::{Compress, IntoResponse};
use rust_embed_for_web::RustEmbed;

#[derive(RustEmbed)]
#[folder = "../frontend/dist/"]
struct FrontendAssets;

fn is_hashed_asset(path: &str) -> bool {
    let filename = path.rsplit('/').next().unwrap_or("");
    filename
        .rsplit_once('.')
        .is_some_and(|(stem, _)| stem.contains('-'))
}

pub async fn spa_fallback(req: HttpRequest) -> HttpResponse {
    let path = req.path().trim_start_matches('/');
    let path = if path.is_empty() { "index.html" } else { path };

    let file = FrontendAssets::get(path).or_else(|| {
        (!path.contains('.'))
            .then(|| FrontendAssets::get("index.html"))
            .flatten()
    });

    let Some(file) = file else {
        return HttpResponse::NotFound().body("Asset not found");
    };

    let mut response = file
        .into_response()
        .use_compression(Compress::IfPrecompressed)
        .respond_to(&req);

    if is_hashed_asset(path) {
        response.headers_mut().insert(
            header::CACHE_CONTROL,
            header::HeaderValue::from_static("public, max-age=31536000, immutable"),
        );
    }

    response
}

pub async fn config_js(_req: HttpRequest) -> HttpResponse {
    let public_url =
        std::env::var("APP__PUBLIC_URL").unwrap_or_else(|_| "http://localhost:3001".to_string());
    let cors_origin = std::env::var("CORS__ALLOWED_ORIGIN")
        .unwrap_or_else(|_| "http://localhost:3001".to_string());

    let body = format!(
        r#"window.__EVOLITH_CONFIG__ = {{
  apiBaseUrl: "{cors_origin}/api/v1",
  publicUrl: "{public_url}",
  wsUrl: "{cors_origin}",
}};"#
    );

    HttpResponse::Ok()
        .content_type("application/javascript")
        .body(body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test::TestRequest;

    #[test]
    fn test_hashed_asset_detection() {
        assert!(is_hashed_asset("assets/index-abc123.js"));
        assert!(is_hashed_asset("assets/main-def456.css"));
        assert!(is_hashed_asset("logo-xyz789.svg"));
        assert!(!is_hashed_asset("index.html"));
        assert!(!is_hashed_asset("favicon.ico"));
        assert!(!is_hashed_asset("config.js"));
        assert!(!is_hashed_asset("robots.txt"));
    }

    fn make_request(uri: &str) -> HttpRequest {
        TestRequest::default().uri(uri).to_http_request()
    }

    #[test]
    fn serve_index_html() {
        let rt = tokio::runtime::Runtime::new().expect("create runtime");
        let resp = rt.block_on(spa_fallback(make_request("/")));
        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
    }

    #[test]
    fn serve_index_html_explicit() {
        let rt = tokio::runtime::Runtime::new().expect("create runtime");
        let resp = rt.block_on(spa_fallback(make_request("/index.html")));
        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
    }

    #[test]
    fn serve_nonexistent_asset() {
        let rt = tokio::runtime::Runtime::new().expect("create runtime");
        let resp = rt.block_on(spa_fallback(make_request("/assets/nonexistent.js")));
        assert_eq!(resp.status(), actix_web::http::StatusCode::NOT_FOUND);
    }
}
