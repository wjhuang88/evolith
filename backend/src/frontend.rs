use std::io::{Cursor, Read};

use actix_web::{HttpRequest, HttpResponse};
use zip::ZipArchive;

const FRONTEND_ZIP: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/frontend.zip"));

fn open_zip() -> Result<ZipArchive<Cursor<&'static [u8]>>, HttpResponse> {
    ZipArchive::new(Cursor::new(FRONTEND_ZIP)).map_err(|e| {
        tracing::error!("Failed to open embedded frontend ZIP: {}", e);
        HttpResponse::InternalServerError().body("Failed to load frontend assets")
    })
}

fn get_content_type(path: &str) -> &'static str {
    match path.rsplit('.').next().unwrap_or("") {
        "html" => "text/html; charset=utf-8",
        "js" => "application/javascript",
        "css" => "text/css",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "ico" => "image/x-icon",
        "woff2" => "font/woff2",
        "json" => "application/json",
        "map" => "application/json",
        "txt" => "text/plain",
        "webp" => "image/webp",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        _ => "application/octet-stream",
    }
}

fn serve_file(path: &str) -> Result<HttpResponse, HttpResponse> {
    let path = path.trim_start_matches('/');
    let path = if path.is_empty() || path == "/" {
        "index.html"
    } else {
        path
    };

    let is_spa_route = !path.contains('.');

    let mut archive = open_zip()?;

    let file_path = if is_spa_route {
        if archive.by_name(path).is_ok() {
            path.to_string()
        } else {
            "index.html".to_string()
        }
    } else {
        path.to_string()
    };

    let mut file = archive
        .by_name(&file_path)
        .map_err(|_| HttpResponse::NotFound().body("Asset not found"))?;

    let mut buf = Vec::new();
    file.read_to_end(&mut buf).map_err(|e| {
        tracing::error!("Failed to read asset {}: {}", file_path, e);
        HttpResponse::InternalServerError().body("Failed to read asset")
    })?;

    let ct = get_content_type(&file_path);

    Ok(HttpResponse::Ok()
        .content_type(ct)
        .body(buf))
}

pub async fn spa_fallback(req: HttpRequest) -> HttpResponse {
    let path = req.path();
    match serve_file(path) {
        Ok(resp) => resp,
        Err(resp) => resp,
    }
}

pub async fn config_js(_req: HttpRequest) -> HttpResponse {
    let public_url = std::env::var("APP__PUBLIC_URL").unwrap_or_else(|_| "http://localhost:3001".to_string());
    let cors_origin = std::env::var("CORS__ALLOWED_ORIGIN").unwrap_or_else(|_| "http://localhost:3001".to_string());

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

    #[test]
    fn content_type_js() {
        assert_eq!(get_content_type("index-abc.js"), "application/javascript");
    }

    #[test]
    fn content_type_css() {
        assert_eq!(get_content_type("index-def.css"), "text/css");
    }

    #[test]
    fn content_type_svg() {
        assert_eq!(get_content_type("logo.svg"), "image/svg+xml");
    }

    #[test]
    fn content_type_png() {
        assert_eq!(get_content_type("logo.png"), "image/png");
    }

    #[test]
    fn content_type_ico() {
        assert_eq!(get_content_type("favicon.ico"), "image/x-icon");
    }

    #[test]
    fn content_type_woff2() {
        assert_eq!(get_content_type("fonts/inter.woff2"), "font/woff2");
    }

    #[test]
    fn content_type_json() {
        assert_eq!(get_content_type("manifest.json"), "application/json");
    }

    #[test]
    fn content_type_map() {
        assert_eq!(get_content_type("index-abc.js.map"), "application/json");
    }

    #[test]
    fn content_type_txt() {
        assert_eq!(get_content_type("robots.txt"), "text/plain");
    }

    #[test]
    fn content_type_webp() {
        assert_eq!(get_content_type("image.webp"), "image/webp");
    }

    #[test]
    fn content_type_jpg() {
        assert_eq!(get_content_type("photo.jpg"), "image/jpeg");
    }

    #[test]
    fn content_type_jpeg() {
        assert_eq!(get_content_type("photo.jpeg"), "image/jpeg");
    }

    #[test]
    fn content_type_gif() {
        assert_eq!(get_content_type("anim.gif"), "image/gif");
    }

    #[test]
    fn content_type_unknown() {
        assert_eq!(get_content_type("file.xyz"), "application/octet-stream");
    }

    #[test]
    fn content_type_html() {
        assert_eq!(get_content_type("index.html"), "text/html; charset=utf-8");
    }

    #[test]
    fn serve_index_html() {
        let resp = serve_file("/").expect("should serve index.html");
        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
    }

    #[test]
    fn serve_index_html_explicit() {
        let resp = serve_file("/index.html").expect("should serve index.html");
        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
    }

    #[test]
    fn serve_nonexistent_asset() {
        let resp = serve_file("/assets/nonexistent.js");
        assert!(resp.is_err());
        let resp = resp.expect_err("should return error");
        assert_eq!(resp.status(), actix_web::http::StatusCode::NOT_FOUND);
    }
}
