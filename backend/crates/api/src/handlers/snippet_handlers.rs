//! Snippet handlers

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::dto::common::{ApiResponse, PaginationMeta};

/// Snippet response
#[derive(Debug, Serialize, Clone)]
pub struct SnippetResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub language: String,
    pub framework: Option<String>,
    pub tags: Vec<String>,
    pub content: String,
    pub code: String,
    pub estimated_tokens: u32,
    pub is_public: bool,
    pub owner_id: String,
    pub tenant_id: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Create snippet request
#[derive(Debug, Deserialize, Validate)]
pub struct CreateSnippetRequest {
    #[validate(length(min = 1, max = 128))]
    pub name: String,

    #[validate(length(min = 1))]
    pub description: String,

    #[validate(length(min = 1, max = 32))]
    pub language: String,

    pub framework: Option<String>,
    pub tags: Option<Vec<String>>,

    #[validate(length(min = 1))]
    pub content: String,

    #[validate(length(min = 1))]
    pub code: String,

    pub is_public: Option<bool>,
}

/// Update snippet request
#[derive(Debug, Deserialize)]
pub struct UpdateSnippetRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub language: Option<String>,
    pub framework: Option<String>,
    pub tags: Option<Vec<String>>,
    pub content: Option<String>,
    pub code: Option<String>,
    pub is_public: Option<bool>,
}

/// Snippet store
pub struct SnippetStore {
    snippets: Mutex<HashMap<String, StoredSnippet>>,
}

#[derive(Clone)]
pub struct StoredSnippet {
    pub id: String,
    pub name: String,
    pub description: String,
    pub language: String,
    pub framework: Option<String>,
    pub tags: Vec<String>,
    pub content: String,
    pub code: String,
    pub estimated_tokens: u32,
    pub is_public: bool,
    pub owner_id: String,
    pub tenant_id: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Default for SnippetStore {
    fn default() -> Self {
        let mut store = Self {
            snippets: Mutex::new(HashMap::new()),
        };

        // Seed data
        let seed_snippets = vec![
            (
                "770e8400-e29b-41d4-a716-446655440001",
                "react-useState",
                "React hook for state management",
                "typescript",
                Some("react"),
                vec!["react", "hooks", "state"],
                true,
            ),
            (
                "770e8400-e29b-41d4-a716-446655440002",
                "python-debounce",
                "Debounce function for Python",
                "python",
                None,
                vec!["python", "debounce", "utility"],
                true,
            ),
            (
                "770e8400-e29b-41d4-a716-446655440003",
                "go-error-handling",
                "Go error handling pattern",
                "go",
                None,
                vec!["go", "error", "pattern"],
                true,
            ),
            (
                "770e8400-e29b-41d4-a716-446655440004",
                "rust-result",
                "Rust Result handling",
                "rust",
                None,
                vec!["rust", "result", "error"],
                true,
            ),
            (
                "770e8400-e29b-41d4-a716-446655440005",
                "sql-join",
                "SQL JOIN examples",
                "sql",
                None,
                vec!["sql", "join", "database"],
                true,
            ),
            (
                "770e8400-e29b-41d4-a716-446655440006",
                "ts-interface",
                "TypeScript interface example",
                "typescript",
                None,
                vec!["typescript", "interface", "types"],
                true,
            ),
        ];

        let now = Utc::now().timestamp();

        for (id, name, desc, lang, fw, tags, public) in seed_snippets {
            let code = format!(
                "// {} code snippet for {}\nfunction example() {{\n  // implementation\n}}",
                lang, name
            );
            let content = format!("# {}\n\n{}", name, desc);
            let estimated_tokens = (code.len() as u32) / 4;

            let snippet = StoredSnippet {
                id: id.to_string(),
                name: name.to_string(),
                description: desc.to_string(),
                language: lang.to_string(),
                framework: fw.map(|s| s.to_string()),
                tags: tags.iter().map(|s| s.to_string()).collect(),
                content,
                code,
                estimated_tokens,
                is_public: public,
                owner_id: "00000000-0000-0000-0000-000000000001".to_string(),
                tenant_id: "00000000-0000-0000-0000-000000000001".to_string(),
                created_at: now,
                updated_at: now,
            };

            store
                .snippets
                .lock()
                .unwrap()
                .insert(id.to_string(), snippet);
        }

        store
    }
}

/// Snippet state
#[derive(Clone)]
pub struct SnippetState {
    pub store: Arc<SnippetStore>,
}

impl SnippetState {
    pub fn new() -> Self {
        Self {
            store: Arc::new(SnippetStore::default()),
        }
    }
}

impl Default for SnippetState {
    fn default() -> Self {
        Self::new()
    }
}

/// List snippets handler
pub async fn list_snippets(state: web::Data<SnippetState>) -> impl Responder {
    let snippets = state.store.snippets.lock().unwrap();
    let total = snippets.len() as u32;
    let snippets: Vec<SnippetResponse> = snippets
        .values()
        .map(|s| SnippetResponse {
            id: s.id.clone(),
            name: s.name.clone(),
            description: s.description.clone(),
            language: s.language.clone(),
            framework: s.framework.clone(),
            tags: s.tags.clone(),
            content: s.content.clone(),
            code: s.code.clone(),
            estimated_tokens: s.estimated_tokens,
            is_public: s.is_public,
            owner_id: s.owner_id.clone(),
            tenant_id: s.tenant_id.clone(),
            created_at: chrono::DateTime::from_timestamp(s.created_at, 0)
                .unwrap_or_default()
                .to_rfc3339(),
            updated_at: chrono::DateTime::from_timestamp(s.updated_at, 0)
                .unwrap_or_default()
                .to_rfc3339(),
        })
        .collect();

    HttpResponse::Ok().json(ApiResponse::<Vec<SnippetResponse>>::success_with_meta(
        snippets,
        PaginationMeta {
            page: 1,
            per_page: 20,
            total,
        },
    ))
}

/// Get snippet handler
pub async fn get_snippet(id: web::Path<String>, state: web::Data<SnippetState>) -> impl Responder {
    let snippets = state.store.snippets.lock().unwrap();

    if let Some(snippet) = snippets.get(&id.to_string()) {
        let response = SnippetResponse {
            id: snippet.id.clone(),
            name: snippet.name.clone(),
            description: snippet.description.clone(),
            language: snippet.language.clone(),
            framework: snippet.framework.clone(),
            tags: snippet.tags.clone(),
            content: snippet.content.clone(),
            code: snippet.code.clone(),
            estimated_tokens: snippet.estimated_tokens,
            is_public: snippet.is_public,
            owner_id: snippet.owner_id.clone(),
            tenant_id: snippet.tenant_id.clone(),
            created_at: chrono::DateTime::from_timestamp(snippet.created_at, 0)
                .unwrap_or_default()
                .to_rfc3339(),
            updated_at: chrono::DateTime::from_timestamp(snippet.updated_at, 0)
                .unwrap_or_default()
                .to_rfc3339(),
        };

        HttpResponse::Ok().json(ApiResponse::<SnippetResponse>::success(response))
    } else {
        HttpResponse::NotFound().json(ApiResponse::<()>::error("NOT_FOUND", "Snippet not found"))
    }
}

/// Create snippet handler
pub async fn create_snippet(
    body: web::Json<CreateSnippetRequest>,
    state: web::Data<SnippetState>,
) -> impl Responder {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "VALIDATION_ERROR",
            &errors.to_string(),
        ));
    }

    let now = Utc::now().timestamp();
    let id = Uuid::new_v4().to_string();
    let estimated_tokens = (body.code.len() as u32) / 4;

    let snippet = StoredSnippet {
        id: id.clone(),
        name: body.name.clone(),
        description: body.description.clone(),
        language: body.language.clone(),
        framework: body.framework.clone(),
        tags: body.tags.clone().unwrap_or_default(),
        content: body.content.clone(),
        code: body.code.clone(),
        estimated_tokens,
        is_public: body.is_public.unwrap_or(false),
        owner_id: "00000000-0000-0000-0000-000000000001".to_string(),
        tenant_id: "00000000-0000-0000-0000-000000000001".to_string(),
        created_at: now,
        updated_at: now,
    };

    {
        let mut snippets = state.store.snippets.lock().unwrap();
        snippets.insert(id.clone(), snippet.clone());
    }

    let response = SnippetResponse {
        id: snippet.id,
        name: snippet.name,
        description: snippet.description,
        language: snippet.language,
        framework: snippet.framework,
        tags: snippet.tags,
        content: snippet.content,
        code: snippet.code,
        estimated_tokens: snippet.estimated_tokens,
        is_public: snippet.is_public,
        owner_id: snippet.owner_id,
        tenant_id: snippet.tenant_id,
        created_at: chrono::DateTime::from_timestamp(snippet.created_at, 0)
            .unwrap_or_default()
            .to_rfc3339(),
        updated_at: chrono::DateTime::from_timestamp(snippet.updated_at, 0)
            .unwrap_or_default()
            .to_rfc3339(),
    };

    HttpResponse::Ok().json(ApiResponse::<SnippetResponse>::success(response))
}

/// Update snippet handler
pub async fn update_snippet(
    id: web::Path<String>,
    body: web::Json<UpdateSnippetRequest>,
    state: web::Data<SnippetState>,
) -> impl Responder {
    let mut snippets = state.store.snippets.lock().unwrap();

    if let Some(snippet) = snippets.get_mut(&id.to_string()) {
        if let Some(name) = &body.name {
            snippet.name = name.clone();
        }
        if let Some(description) = &body.description {
            snippet.description = description.clone();
        }
        if let Some(language) = &body.language {
            snippet.language = language.clone();
        }
        if let Some(framework) = &body.framework {
            snippet.framework = Some(framework.clone());
        }
        if let Some(tags) = &body.tags {
            snippet.tags = tags.clone();
        }
        if let Some(content) = &body.content {
            snippet.content = content.clone();
        }
        if let Some(code) = &body.code {
            snippet.code = code.clone();
            snippet.estimated_tokens = (code.len() as u32) / 4;
        }
        if let Some(is_public) = body.is_public {
            snippet.is_public = is_public;
        }
        snippet.updated_at = Utc::now().timestamp();

        let response = SnippetResponse {
            id: snippet.id.clone(),
            name: snippet.name.clone(),
            description: snippet.description.clone(),
            language: snippet.language.clone(),
            framework: snippet.framework.clone(),
            tags: snippet.tags.clone(),
            content: snippet.content.clone(),
            code: snippet.code.clone(),
            estimated_tokens: snippet.estimated_tokens,
            is_public: snippet.is_public,
            owner_id: snippet.owner_id.clone(),
            tenant_id: snippet.tenant_id.clone(),
            created_at: chrono::DateTime::from_timestamp(snippet.created_at, 0)
                .unwrap_or_default()
                .to_rfc3339(),
            updated_at: chrono::DateTime::from_timestamp(snippet.updated_at, 0)
                .unwrap_or_default()
                .to_rfc3339(),
        };

        HttpResponse::Ok().json(ApiResponse::<SnippetResponse>::success(response))
    } else {
        HttpResponse::NotFound().json(ApiResponse::<()>::error("NOT_FOUND", "Snippet not found"))
    }
}

/// Delete snippet handler
pub async fn delete_snippet(
    id: web::Path<String>,
    state: web::Data<SnippetState>,
) -> impl Responder {
    let mut snippets = state.store.snippets.lock().unwrap();

    if snippets.remove(&id.to_string()).is_some() {
        HttpResponse::Ok().json(ApiResponse::<serde_json::Value>::success(
            serde_json::json!({ "message": "Snippet deleted successfully" }),
        ))
    } else {
        HttpResponse::NotFound().json(ApiResponse::<()>::error("NOT_FOUND", "Snippet not found"))
    }
}

/// Search snippets handler
pub async fn search_snippets(
    query: web::Query<std::collections::HashMap<String, String>>,
    state: web::Data<SnippetState>,
) -> impl Responder {
    let snippets = state.store.snippets.lock().unwrap();
    let search_term = query.get("q").map(|s| s.to_lowercase()).unwrap_or_default();

    let results: Vec<SnippetResponse> = snippets
        .values()
        .filter(|s| {
            if search_term.is_empty() {
                return true;
            }
            s.name.to_lowercase().contains(&search_term)
                || s.description.to_lowercase().contains(&search_term)
                || s.language.to_lowercase().contains(&search_term)
                || s.tags
                    .iter()
                    .any(|t| t.to_lowercase().contains(&search_term))
        })
        .map(|s| SnippetResponse {
            id: s.id.clone(),
            name: s.name.clone(),
            description: s.description.clone(),
            language: s.language.clone(),
            framework: s.framework.clone(),
            tags: s.tags.clone(),
            content: s.content.clone(),
            code: s.code.clone(),
            estimated_tokens: s.estimated_tokens,
            is_public: s.is_public,
            owner_id: s.owner_id.clone(),
            tenant_id: s.tenant_id.clone(),
            created_at: chrono::DateTime::from_timestamp(s.created_at, 0)
                .unwrap_or_default()
                .to_rfc3339(),
            updated_at: chrono::DateTime::from_timestamp(s.updated_at, 0)
                .unwrap_or_default()
                .to_rfc3339(),
        })
        .collect();

    let total = results.len() as u32;

    HttpResponse::Ok().json(ApiResponse::<Vec<SnippetResponse>>::success_with_meta(
        results,
        PaginationMeta {
            page: 1,
            per_page: 20,
            total,
        },
    ))
}

/// Get reference handler (for LLM consumption)
pub async fn get_reference(
    id: web::Path<String>,
    state: web::Data<SnippetState>,
) -> impl Responder {
    let snippets = state.store.snippets.lock().unwrap();

    if let Some(snippet) = snippets.get(&id.to_string()) {
        HttpResponse::Ok().json(ApiResponse::<serde_json::Value>::success(
            serde_json::json!({
                "reference": {
                    "name": snippet.name,
                    "language": snippet.language,
                    "framework": snippet.framework,
                    "code": snippet.code,
                    "estimated_tokens": snippet.estimated_tokens,
                }
            }),
        ))
    } else {
        HttpResponse::NotFound().json(ApiResponse::<()>::error("NOT_FOUND", "Snippet not found"))
    }
}
