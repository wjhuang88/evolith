//! PostgreSQL Tool Repository implementation

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use common::error::{AppError, Result};
use domain::repository::ToolRepository;
use domain::tool::{HandlerConfig, HandlerType, NewTool, Tool, ToolFilter, UpdateTool, Visibility};
use sqlx::PgPool;
use uuid::Uuid;

pub struct PgToolRepository {
    pool: PgPool,
}

impl PgToolRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ToolRepository for PgToolRepository {
    async fn create(&self, tool: NewTool, owner_id: Uuid, tenant_id: Uuid) -> Result<Tool> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let visibility = tool.visibility.unwrap_or_default();
        let visibility_str = match &visibility {
            Visibility::Public => "public",
            Visibility::Private => "private",
        };
        let handler_type_str = match tool.handler.handler_type {
            HandlerType::Http => "http",
            HandlerType::Function => "function",
        };

        sqlx::query(
            r#"
            INSERT INTO tools (
                id, name, description, input_schema, output_schema,
                handler_type, handler_url, handler_method, handler_timeout,
                visibility, owner_id, tenant_id, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            "#,
        )
        .bind(id)
        .bind(&tool.name)
        .bind(&tool.description)
        .bind(&tool.input_schema)
        .bind(&tool.output_schema)
        .bind(handler_type_str)
        .bind(&tool.handler.url)
        .bind(&tool.handler.method)
        .bind(tool.handler.timeout.map(|t| t as i32))
        .bind(visibility_str)
        .bind(owner_id)
        .bind(tenant_id)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(Tool {
            id,
            name: tool.name,
            description: tool.description,
            input_schema: tool.input_schema,
            output_schema: tool.output_schema,
            handler: tool.handler,
            owner_id,
            tenant_id,
            visibility,
            created_at: now,
            updated_at: now,
        })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Tool>> {
        let row = sqlx::query_as::<_, PgToolRow>(r#"SELECT * FROM tools WHERE id = $1"#)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| r.into()))
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<Tool>> {
        let row = sqlx::query_as::<_, PgToolRow>(r#"SELECT * FROM tools WHERE name = $1"#)
            .bind(name)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| r.into()))
    }

    async fn find_all(&self, filter: ToolFilter) -> Result<Vec<Tool>> {
        let mut sql = String::from("SELECT * FROM tools WHERE 1=1");
        let mut bindings: Vec<String> = Vec::new();
        let mut param_idx = 1usize;

        if let Some(tenant_id) = filter.tenant_id {
            sql.push_str(&format!(" AND tenant_id = ${}", param_idx));
            param_idx += 1;
            bindings.push(tenant_id.to_string());
        }

        if let Some(ref search) = filter.search {
            sql.push_str(&format!(
                " AND (name LIKE ${} OR description LIKE ${})",
                param_idx,
                param_idx + 1
            ));
            let search_pattern = format!("%{}%", search);
            bindings.push(search_pattern.clone());
            bindings.push(search_pattern);
            param_idx += 2;
        }

        if let Some(ref visibility) = filter.visibility {
            sql.push_str(&format!(" AND visibility = ${}", param_idx));
            let visibility_str = match visibility {
                Visibility::Public => "public",
                Visibility::Private => "private",
            };
            bindings.push(visibility_str.to_string());
            param_idx += 1;
        }

        if let Some(owner_id) = filter.owner_id {
            sql.push_str(&format!(" AND owner_id = ${}", param_idx));
            bindings.push(owner_id.to_string());
        }

        sql.push_str(" ORDER BY created_at DESC");

        let page = filter.page.unwrap_or(1);
        let per_page = filter.per_page.unwrap_or(20);
        let offset = (page.saturating_sub(1)) * per_page;
        sql.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));

        let mut query = sqlx::query_as::<_, PgToolRow>(&sql);
        for binding in &bindings {
            query = query.bind(binding);
        }

        let rows = query
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn count(&self, filter: &ToolFilter) -> Result<u32> {
        let mut sql = String::from("SELECT COUNT(*) as count FROM tools WHERE 1=1");
        let mut bindings: Vec<String> = Vec::new();
        let mut param_idx = 1usize;

        if let Some(tenant_id) = filter.tenant_id {
            sql.push_str(&format!(" AND tenant_id = ${}", param_idx));
            param_idx += 1;
            bindings.push(tenant_id.to_string());
        }

        if let Some(ref search) = filter.search {
            sql.push_str(&format!(
                " AND (name LIKE ${} OR description LIKE ${})",
                param_idx,
                param_idx + 1
            ));
            let search_pattern = format!("%{}%", search);
            bindings.push(search_pattern.clone());
            bindings.push(search_pattern);
            param_idx += 2;
        }

        if let Some(ref visibility) = filter.visibility {
            sql.push_str(&format!(" AND visibility = ${}", param_idx));
            let visibility_str = match visibility {
                Visibility::Public => "public",
                Visibility::Private => "private",
            };
            bindings.push(visibility_str.to_string());
            param_idx += 1;
        }

        if let Some(owner_id) = filter.owner_id {
            sql.push_str(&format!(" AND owner_id = ${}", param_idx));
            bindings.push(owner_id.to_string());
        }

        let mut query = sqlx::query_as::<_, CountRow>(&sql);
        for binding in &bindings {
            query = query.bind(binding);
        }

        let row = query
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.count as u32)
    }

    async fn update(&self, id: Uuid, tool: UpdateTool) -> Result<Tool> {
        let now = Utc::now();

        if let Some(name) = &tool.name {
            sqlx::query(r#"UPDATE tools SET name = $1, updated_at = $2 WHERE id = $3"#)
                .bind(name)
                .bind(now)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        }

        if let Some(description) = &tool.description {
            sqlx::query(r#"UPDATE tools SET description = $1, updated_at = $2 WHERE id = $3"#)
                .bind(description)
                .bind(now)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        }

        if let Some(input_schema) = &tool.input_schema {
            sqlx::query(r#"UPDATE tools SET input_schema = $1, updated_at = $2 WHERE id = $3"#)
                .bind(input_schema)
                .bind(now)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        }

        if let Some(output_schema) = &tool.output_schema {
            sqlx::query(r#"UPDATE tools SET output_schema = $1, updated_at = $2 WHERE id = $3"#)
                .bind(output_schema)
                .bind(now)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        }

        if let Some(handler) = &tool.handler {
            let handler_type_str = match handler.handler_type {
                HandlerType::Http => "http",
                HandlerType::Function => "function",
            };
            sqlx::query(
                r#"UPDATE tools SET handler_type = $1, handler_url = $2, handler_method = $3, handler_timeout = $4, updated_at = $5 WHERE id = $6"#
            )
            .bind(handler_type_str)
            .bind(&handler.url)
            .bind(&handler.method)
            .bind(handler.timeout.map(|t| t as i32))
            .bind(now)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        }

        if let Some(visibility) = &tool.visibility {
            let visibility_str = match visibility {
                Visibility::Public => "public",
                Visibility::Private => "private",
            };
            sqlx::query(r#"UPDATE tools SET visibility = $1, updated_at = $2 WHERE id = $3"#)
                .bind(visibility_str)
                .bind(now)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        }

        self.find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFoundError(format!("Tool {} not found", id)))
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query(r#"DELETE FROM tools WHERE id = $1"#)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct PgToolRow {
    id: Uuid,
    name: String,
    description: String,
    input_schema: serde_json::Value,
    output_schema: Option<serde_json::Value>,
    handler_type: String,
    handler_url: Option<String>,
    handler_method: Option<String>,
    handler_timeout: Option<i32>,
    visibility: String,
    owner_id: Uuid,
    tenant_id: Uuid,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<PgToolRow> for Tool {
    fn from(row: PgToolRow) -> Self {
        Tool {
            id: row.id,
            name: row.name,
            description: row.description,
            input_schema: row.input_schema,
            output_schema: row.output_schema,
            handler: HandlerConfig {
                handler_type: match row.handler_type.as_str() {
                    "http" => HandlerType::Http,
                    _ => HandlerType::Function,
                },
                url: row.handler_url,
                method: row.handler_method,
                timeout: row.handler_timeout.map(|t| t as u32),
            },
            owner_id: row.owner_id,
            tenant_id: row.tenant_id,
            visibility: match row.visibility.as_str() {
                "public" => Visibility::Public,
                _ => Visibility::Private,
            },
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(sqlx::FromRow)]
struct CountRow {
    count: i64,
}
