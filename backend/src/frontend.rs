use std::collections::HashMap;
use std::io::{Cursor, Read};
use std::sync::Arc;

use actix_web::http::header;
use actix_web::{web, HttpRequest, HttpResponse};
use futures_util::stream;
use zip::ZipArchive;

const FRONTEND_ZIP: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/frontend.zip"));

struct FileEntry {
    index: usize,
    uncompressed_size: u64,
    crc32: u32,
}

struct FrontendIndex {
    files: HashMap<String, FileEntry>,
}

static FRONTEND_INDEX: std::sync::OnceLock<Result<Arc<FrontendIndex>, String>> =
    std::sync::OnceLock::new();

fn build_index() -> Result<Arc<FrontendIndex>, String> {
    let archive = ZipArchive::new(Cursor::new(FRONTEND_ZIP))
        .map_err(|e| format!("Failed to parse embedded frontend ZIP: {}", e))?;

    let len = archive.len();
    let mut files = HashMap::with_capacity(len);

    for i in 0..len {
        let name = archive
            .name_for_index(i)
            .ok_or_else(|| format!("Invalid index {}", i))?;

        let normalized = name.strip_prefix("./").unwrap_or(name).to_string();

        let mut tmp = ZipArchive::new(Cursor::new(FRONTEND_ZIP))
            .map_err(|e| format!("Failed to reopen ZIP for index {}: {}", i, e))?;
        let file = tmp
            .by_index(i)
            .map_err(|e| format!("Failed to read entry {}: {}", i, e))?;

        files.insert(
            normalized,
            FileEntry {
                index: i,
                uncompressed_size: file.size(),
                crc32: file.crc32(),
            },
        );
    }

    Ok(Arc::new(FrontendIndex { files }))
}

fn get_index() -> Result<Arc<FrontendIndex>, HttpResponse> {
    FRONTEND_INDEX
        .get_or_init(build_index)
        .clone()
        .map_err(|e| {
            tracing::error!("Frontend index initialization failed: {}", e);
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

fn is_hashed_asset(path: &str) -> bool {
    let filename = path.rsplit('/').next().unwrap_or("");
    let parts: Vec<&str> = filename.rsplitn(2, '.').collect();
    if parts.len() != 2 {
        return false;
    }
    let stem = parts[1];
    stem.contains('-')
}

fn compute_etag(path: &str, entry: &FileEntry) -> String {
    if is_hashed_asset(path) {
        let filename = path.rsplit('/').next().unwrap_or("");
        let stem = filename.rsplit_once('.').map(|(s, _)| s).unwrap_or("");
        stem.rsplit('-').next().unwrap_or("").to_string()
    } else {
        format!("{:08x}", entry.crc32)
    }
}

async fn read_file_from_zip(index: usize) -> Result<Vec<u8>, String> {
    web::block(move || {
        let mut archive = ZipArchive::new(Cursor::new(FRONTEND_ZIP))
            .map_err(|e| format!("Failed to open embedded frontend ZIP: {}", e))?;

        let mut file = archive
            .by_index(index)
            .map_err(|e| format!("Failed to find entry at index {}: {}", index, e))?;

        let mut buf = Vec::with_capacity(file.size() as usize);
        let mut chunk = [0u8; 16384];
        loop {
            let n = file
                .read(&mut chunk)
                .map_err(|e| format!("Failed to read asset: {}", e))?;
            if n == 0 {
                break;
            }
            buf.extend_from_slice(&chunk[..n]);
        }

        Ok(buf)
    })
    .await
    .map_err(|e| format!("Blocking error: {}", e))?
}

fn resolve_path(path: &str, index: &FrontendIndex) -> String {
    let path = path.trim_start_matches('/');
    let path = if path.is_empty() || path == "/" {
        "index.html"
    } else {
        path
    };

    let is_spa_route = !path.contains('.');

    if is_spa_route {
        if index.files.contains_key(path) {
            path.to_string()
        } else {
            "index.html".to_string()
        }
    } else {
        path.to_string()
    }
}

fn check_if_none_match(req: &HttpRequest, etag: &str) -> bool {
    if let Some(if_none_match) = req.headers().get(header::IF_NONE_MATCH) {
        if let Ok(if_none_match_str) = if_none_match.to_str() {
            return if_none_match_str
                .split(',')
                .map(|s| s.trim())
                .any(|s| {
                    let s = s.trim();
                    if s == "*" {
                        return true;
                    }
                    let s = s.strip_prefix("W/").unwrap_or(s);
                    let s = s.strip_prefix('"').unwrap_or(s);
                    let s = s.strip_suffix('"').unwrap_or(s);
                    s == etag
                });
        }
    }
    false
}

fn not_modified_response(etag: String) -> HttpResponse {
    HttpResponse::NotModified()
        .insert_header(header::ETag(header::EntityTag::new(false, etag)))
        .finish()
}

fn ok_response(
    resolved_path: String,
    entry: &FileEntry,
    bytes: Vec<u8>,
) -> HttpResponse {
    let etag = compute_etag(&resolved_path, entry);
    let content_type = get_content_type(&resolved_path);
    let content_length = entry.uncompressed_size as usize;

    let cache_control = if is_hashed_asset(&resolved_path) {
        header::CacheControl(vec![
            header::CacheDirective::Public,
            header::CacheDirective::MaxAge(31536000),
            header::CacheDirective::Extension("immutable".to_string(), None),
        ])
    } else {
        header::CacheControl(vec![header::CacheDirective::NoCache])
    };

    HttpResponse::Ok()
        .content_type(content_type)
        .insert_header(cache_control)
        .insert_header(header::ContentLength(content_length))
        .insert_header(header::ETag(header::EntityTag::new(false, etag)))
        .streaming(stream::once(async move {
            Ok::<_, actix_web::Error>(web::Bytes::from(bytes))
        }))
}

async fn serve_file_inner(path: &str, req: Option<&HttpRequest>) -> HttpResponse {
    let index = match get_index() {
        Ok(idx) => idx,
        Err(resp) => return resp,
    };

    let resolved_path = resolve_path(path, &index);

    let entry = match index.files.get(&resolved_path) {
        Some(e) => e,
        None => {
            return HttpResponse::NotFound().body("Asset not found");
        }
    };

    let etag = compute_etag(&resolved_path, entry);

    if let Some(req) = req {
        if check_if_none_match(req, &etag) {
            return not_modified_response(etag);
        }
    }

    let entry_index = entry.index;
    let bytes = match read_file_from_zip(entry_index).await {
        Ok(b) => b,
        Err(e) => {
            tracing::error!("Failed to read file: {}", e);
            return HttpResponse::InternalServerError().body("Failed to read asset");
        }
    };

    ok_response(resolved_path, entry, bytes)
}

pub async fn spa_fallback(req: HttpRequest) -> HttpResponse {
    let path = req.path().trim_start_matches('/').to_string();
    let path = if path.is_empty() {
        "index.html".to_string()
    } else {
        path
    };

    serve_file_inner(&path, Some(&req)).await
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

    async fn serve_file(path: &str) -> HttpResponse {
        serve_file_inner(path, None).await
    }

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
    fn test_index_initialization() {
        let index = get_index().expect("index should initialize");
        assert!(
            index.files.contains_key("index.html"),
            "index should contain index.html"
        );
    }

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

    #[test]
    fn test_etag_generation() {
        let entry = FileEntry {
            index: 0,
            uncompressed_size: 100,
            crc32: 0x12345678,
        };
        let etag = compute_etag("assets/index-abc123.js", &entry);
        assert_eq!(etag, "abc123");

        let etag = compute_etag("index.html", &entry);
        assert_eq!(etag, "12345678");
    }

    #[test]
    fn serve_index_html() {
        let rt = tokio::runtime::Runtime::new().expect("create runtime");
        let resp = rt.block_on(serve_file("/"));
        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
    }

    #[test]
    fn serve_index_html_explicit() {
        let rt = tokio::runtime::Runtime::new().expect("create runtime");
        let resp = rt.block_on(serve_file("/index.html"));
        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
    }

    #[test]
    fn serve_nonexistent_asset() {
        let rt = tokio::runtime::Runtime::new().expect("create runtime");
        let resp = rt.block_on(serve_file("/assets/nonexistent.js"));
        assert_eq!(resp.status(), actix_web::http::StatusCode::NOT_FOUND);
    }
}
