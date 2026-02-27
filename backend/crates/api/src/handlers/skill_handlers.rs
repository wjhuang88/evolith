//! Skill handlers

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::dto::common::{ApiResponse, PaginationMeta};

/// Skill response
#[derive(Debug, Serialize, Clone)]
pub struct SkillResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub content: String,
    pub category: String,
    pub tags: Vec<String>,
    pub runtime: String,
    pub is_public: bool,
    pub owner_id: String,
    pub tenant_id: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Create skill request
#[derive(Debug, Deserialize, Validate)]
pub struct CreateSkillRequest {
    #[validate(length(min = 1, max = 128))]
    pub name: String,

    #[validate(length(min = 1))]
    pub description: String,

    #[validate(length(min = 1))]
    pub version: String,

    #[validate(length(min = 1))]
    pub content: String,

    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub runtime: Option<String>,
    pub is_public: Option<bool>,
}

/// Update skill request
#[derive(Debug, Deserialize)]
pub struct UpdateSkillRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub content: Option<String>,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub runtime: Option<String>,
    pub is_public: Option<bool>,
}

/// Skill store
pub struct SkillStore {
    skills: Mutex<HashMap<String, StoredSkill>>,
}

#[derive(Clone)]
pub struct StoredSkill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub content: String,
    pub category: String,
    pub tags: Vec<String>,
    pub runtime: String,
    pub is_public: bool,
    pub owner_id: String,
    pub tenant_id: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Default for SkillStore {
    fn default() -> Self {
        let mut store = Self {
            skills: Mutex::new(HashMap::new()),
        };

        // Seed data
        let seed_skills = vec![
            (
                "660e8400-e29b-41d4-a716-446655440001",
                "data-analyzer",
                "Analyze CSV/JSON data and generate statistical reports",
                "1.0.0",
                "data",
                vec!["data", "analysis", "csv", "json", "statistics"],
                true,
            ),
            (
                "660e8400-e29b-41d4-a716-446655440002",
                "code-review",
                "Automated code review with best practices",
                "1.2.0",
                "development",
                vec!["code", "review", "quality", "linting"],
                true,
            ),
            (
                "660e8400-e29b-41d4-a716-446655440003",
                "api-doc-generator",
                "Generate API documentation from code",
                "2.0.0",
                "documentation",
                vec!["api", "docs", "openapi", "swagger"],
                false,
            ),
            (
                "660e8400-e29b-41d4-a716-446655440004",
                "test-generator",
                "Generate unit tests from code",
                "1.5.0",
                "testing",
                vec!["testing", "unit-test", "coverage", "mock"],
                true,
            ),
            (
                "660e8400-e29b-41d4-a716-446655440005",
                "data-visualizer",
                "Create visualizations from data",
                "1.0.0",
                "visualization",
                vec!["charts", "graphs", "visualization", "data"],
                true,
            ),
        ];

        let now = Utc::now().timestamp();

        for (id, name, desc, ver, cat, tags, public) in seed_skills {
            let content = format!("---\nname: {}\nversion: {}\ndescription: {}\n---\n\n# {}\n\nThis skill provides {} functionality.", name, ver, desc, name, cat);
            
            let skill = StoredSkill {
                id: id.to_string(),
                name: name.to_string(),
                description: desc.to_string(),
                version: ver.to_string(),
                content,
                category: cat.to_string(),
                tags: tags.iter().map(|s| s.to_string()).collect(),
                runtime: "javascript".to_string(),
                is_public: public,
                owner_id: "00000000-0000-0000-0000-000000000001".to_string(),
                tenant_id: "00000000-0000-0000-0000-000000000001".to_string(),
                created_at: now,
                updated_at: now,
            };

            store.skills.lock().unwrap().insert(id.to_string(), skill);
        }

        store
    }
}

/// Skill state
#[derive(Clone)]
pub struct SkillState {
    pub store: Arc<SkillStore>,
}

impl SkillState {
    pub fn new() -> Self {
        Self {
            store: Arc::new(SkillStore::default()),
        }
    }
}

impl Default for SkillState {
    fn default() -> Self {
        Self::new()
    }
}

/// List skills handler
pub async fn list_skills(state: web::Data<SkillState>) -> impl Responder {
    let skills = state.store.skills.lock().unwrap();
    let total = skills.len() as u32;
    let skills: Vec<SkillResponse> = skills
        .values()
        .map(|s| SkillResponse {
            id: s.id.clone(),
            name: s.name.clone(),
            description: s.description.clone(),
            version: s.version.clone(),
            content: s.content.clone(),
            category: s.category.clone(),
            tags: s.tags.clone(),
            runtime: s.runtime.clone(),
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

    HttpResponse::Ok().json(ApiResponse::<Vec<SkillResponse>>::success_with_meta(
        skills,
        PaginationMeta {
            page: 1,
            per_page: 20,
            total,
        },
    ))
}

/// Get skill handler
pub async fn get_skill(id: web::Path<String>, state: web::Data<SkillState>) -> impl Responder {
    let skills = state.store.skills.lock().unwrap();

    if let Some(skill) = skills.get(&id.to_string()) {
        let response = SkillResponse {
            id: skill.id.clone(),
            name: skill.name.clone(),
            description: skill.description.clone(),
            version: skill.version.clone(),
            content: skill.content.clone(),
            category: skill.category.clone(),
            tags: skill.tags.clone(),
            runtime: skill.runtime.clone(),
            is_public: skill.is_public,
            owner_id: skill.owner_id.clone(),
            tenant_id: skill.tenant_id.clone(),
            created_at: chrono::DateTime::from_timestamp(skill.created_at, 0)
                .unwrap_or_default()
                .to_rfc3339(),
            updated_at: chrono::DateTime::from_timestamp(skill.updated_at, 0)
                .unwrap_or_default()
                .to_rfc3339(),
        };

        HttpResponse::Ok().json(ApiResponse::<SkillResponse>::success(response))
    } else {
        HttpResponse::NotFound().json(ApiResponse::<()>::error("NOT_FOUND", "Skill not found"))
    }
}

/// Create skill handler
pub async fn create_skill(
    body: web::Json<CreateSkillRequest>,
    state: web::Data<SkillState>,
) -> impl Responder {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "VALIDATION_ERROR",
            &errors.to_string(),
        ));
    }

    let now = Utc::now().timestamp();
    let id = Uuid::new_v4().to_string();

    let skill = StoredSkill {
        id: id.clone(),
        name: body.name.clone(),
        description: body.description.clone(),
        version: body.version.clone(),
        content: body.content.clone(),
        category: body
            .category
            .clone()
            .unwrap_or_else(|| "custom".to_string()),
        tags: body.tags.clone().unwrap_or_default(),
        runtime: body
            .runtime
            .clone()
            .unwrap_or_else(|| "javascript".to_string()),
        is_public: body.is_public.unwrap_or(false),
        owner_id: "00000000-0000-0000-0000-000000000001".to_string(),
        tenant_id: "00000000-0000-0000-0000-000000000001".to_string(),
        created_at: now,
        updated_at: now,
    };

    {
        let mut skills = state.store.skills.lock().unwrap();
        skills.insert(id.clone(), skill.clone());
    }

    let response = SkillResponse {
        id: skill.id,
        name: skill.name,
        description: skill.description,
        version: skill.version,
        content: skill.content,
        category: skill.category,
        tags: skill.tags,
        runtime: skill.runtime,
        is_public: skill.is_public,
        owner_id: skill.owner_id,
        tenant_id: skill.tenant_id,
        created_at: chrono::DateTime::from_timestamp(skill.created_at, 0)
            .unwrap_or_default()
            .to_rfc3339(),
        updated_at: chrono::DateTime::from_timestamp(skill.updated_at, 0)
            .unwrap_or_default()
            .to_rfc3339(),
    };

    HttpResponse::Ok().json(ApiResponse::<SkillResponse>::success(response))
}

/// Update skill handler
pub async fn update_skill(
    id: web::Path<String>,
    body: web::Json<UpdateSkillRequest>,
    state: web::Data<SkillState>,
) -> impl Responder {
    let mut skills = state.store.skills.lock().unwrap();

    if let Some(skill) = skills.get_mut(&id.to_string()) {
        if let Some(name) = &body.name {
            skill.name = name.clone();
        }
        if let Some(description) = &body.description {
            skill.description = description.clone();
        }
        if let Some(version) = &body.version {
            skill.version = version.clone();
        }
        if let Some(content) = &body.content {
            skill.content = content.clone();
        }
        if let Some(category) = &body.category {
            skill.category = category.clone();
        }
        if let Some(tags) = &body.tags {
            skill.tags = tags.clone();
        }
        if let Some(runtime) = &body.runtime {
            skill.runtime = runtime.clone();
        }
        if let Some(is_public) = body.is_public {
            skill.is_public = is_public;
        }
        skill.updated_at = Utc::now().timestamp();

        let response = SkillResponse {
            id: skill.id.clone(),
            name: skill.name.clone(),
            description: skill.description.clone(),
            version: skill.version.clone(),
            content: skill.content.clone(),
            category: skill.category.clone(),
            tags: skill.tags.clone(),
            runtime: skill.runtime.clone(),
            is_public: skill.is_public,
            owner_id: skill.owner_id.clone(),
            tenant_id: skill.tenant_id.clone(),
            created_at: chrono::DateTime::from_timestamp(skill.created_at, 0)
                .unwrap_or_default()
                .to_rfc3339(),
            updated_at: chrono::DateTime::from_timestamp(skill.updated_at, 0)
                .unwrap_or_default()
                .to_rfc3339(),
        };

        HttpResponse::Ok().json(ApiResponse::<SkillResponse>::success(response))
    } else {
        HttpResponse::NotFound().json(ApiResponse::<()>::error("NOT_FOUND", "Skill not found"))
    }
}

/// Delete skill handler
pub async fn delete_skill(id: web::Path<String>, state: web::Data<SkillState>) -> impl Responder {
    let mut skills = state.store.skills.lock().unwrap();

    if skills.remove(&id.to_string()).is_some() {
        HttpResponse::Ok().json(ApiResponse::<serde_json::Value>::success(
            serde_json::json!({ "message": "Skill deleted successfully" }),
        ))
    } else {
        HttpResponse::NotFound().json(ApiResponse::<()>::error("NOT_FOUND", "Skill not found"))
    }
}

/// Load skill handler (for skill execution)
pub async fn load_skill(id: web::Path<String>, state: web::Data<SkillState>) -> impl Responder {
    let skills = state.store.skills.lock().unwrap();

    if let Some(skill) = skills.get(&id.to_string()) {
        HttpResponse::Ok().json(ApiResponse::<serde_json::Value>::success(
            serde_json::json!({
                "skill": {
                    "id": skill.id,
                    "name": skill.name,
                    "version": skill.version,
                    "content": skill.content,
                    "runtime": skill.runtime,
                }
            }),
        ))
    } else {
        HttpResponse::NotFound().json(ApiResponse::<()>::error("NOT_FOUND", "Skill not found"))
    }
}

/// Execute skill handler (placeholder)
pub async fn execute_skill(id: web::Path<String>, state: web::Data<SkillState>) -> impl Responder {
    let skills = state.store.skills.lock().unwrap();

    if let Some(skill) = skills.get(&id.to_string()) {
        HttpResponse::Ok().json(ApiResponse::<serde_json::Value>::success(
            serde_json::json!({
                "message": "Skill execution not implemented",
                "skill_id": skill.id,
                "skill_name": skill.name,
            }),
        ))
    } else {
        HttpResponse::NotFound().json(ApiResponse::<()>::error("NOT_FOUND", "Skill not found"))
    }
}
