use async_trait::async_trait;
use chrono::{DateTime, Utc};
use common::error::{AppError, Result};
use domain::repository::ToolRepository;
use domain::tool::{HandlerConfig, HandlerType, NewTool, Tool, ToolFilter, UpdateTool, Visibility};
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct SqliteToolRepository {
    pool: SqlitePool,
}

impl SqliteToolRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ToolRepository for SqliteToolRepository {
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
        let input_schema_str = serde_json::to_string(&tool.input_schema).map_err(|e| {
            AppError::ValidationError(format!("Failed to serialize input_schema: {}", e))
        })?;
        let output_schema_str = tool
            .output_schema
            .as_ref()
            .map(serde_json::to_string)
            .transpose()
            .map_err(|e| {
                AppError::ValidationError(format!("Failed to serialize output_schema: {}", e))
            })?;

        sqlx::query(
            r#"
            INSERT INTO tools (
                id, name, description, input_schema, output_schema,
                handler_type, handler_url, handler_method, handler_timeout,
                visibility, owner_id, tenant_id, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(id.to_string())
        .bind(&tool.name)
        .bind(&tool.description)
        .bind(&input_schema_str)
        .bind(&output_schema_str)
        .bind(handler_type_str)
        .bind(&tool.handler.url)
        .bind(&tool.handler.method)
        .bind(tool.handler.timeout.map(|t| t as i32))
        .bind(visibility_str)
        .bind(owner_id.to_string())
        .bind(tenant_id.to_string())
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
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
        let row = sqlx::query_as::<_, ToolRow>(r#"SELECT * FROM tools WHERE id = ?"#)
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| r.into()))
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<Tool>> {
        let row = sqlx::query_as::<_, ToolRow>(r#"SELECT * FROM tools WHERE name = ?"#)
            .bind(name)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| r.into()))
    }

    async fn find_all(&self, filter: ToolFilter) -> Result<Vec<Tool>> {
        let mut sql = String::from("SELECT * FROM tools WHERE 1=1");
        let mut bindings: Vec<String> = Vec::new();

        if let Some(tenant_id) = filter.tenant_id {
            sql.push_str(" AND tenant_id = ?");
            bindings.push(tenant_id.to_string());
        }

        if let Some(ref search) = filter.search {
            sql.push_str(" AND (name LIKE ? OR description LIKE ?)");
            let search_pattern = format!("%{}%", search);
            bindings.push(search_pattern.clone());
            bindings.push(search_pattern);
        }

        if let Some(ref visibility) = filter.visibility {
            sql.push_str(" AND visibility = ?");
            let visibility_str = match visibility {
                Visibility::Public => "public",
                Visibility::Private => "private",
            };
            bindings.push(visibility_str.to_string());
        }

        if let Some(owner_id) = filter.owner_id {
            sql.push_str(" AND owner_id = ?");
            bindings.push(owner_id.to_string());
        }

        sql.push_str(" ORDER BY created_at DESC");

        let page = filter.page.unwrap_or(1);
        let per_page = filter.per_page.unwrap_or(20);
        let offset = (page.saturating_sub(1)) * per_page;
        sql.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));

        let mut query = sqlx::query_as::<_, ToolRow>(sqlx::AssertSqlSafe(sql.as_str()));
        for binding in bindings {
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

        if let Some(tenant_id) = filter.tenant_id {
            sql.push_str(" AND tenant_id = ?");
            bindings.push(tenant_id.to_string());
        }

        if let Some(ref search) = filter.search {
            sql.push_str(" AND (name LIKE ? OR description LIKE ?)");
            let search_pattern = format!("%{}%", search);
            bindings.push(search_pattern.clone());
            bindings.push(search_pattern);
        }

        if let Some(ref visibility) = filter.visibility {
            sql.push_str(" AND visibility = ?");
            let visibility_str = match visibility {
                Visibility::Public => "public",
                Visibility::Private => "private",
            };
            bindings.push(visibility_str.to_string());
        }

        if let Some(owner_id) = filter.owner_id {
            sql.push_str(" AND owner_id = ?");
            bindings.push(owner_id.to_string());
        }

        let mut query = sqlx::query_as::<_, CountRow>(sqlx::AssertSqlSafe(sql.as_str()));
        for binding in bindings {
            query = query.bind(binding);
        }

        let row = query
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.count as u32)
    }

    async fn update(&self, id: Uuid, tool: UpdateTool) -> Result<Tool> {
        let now = Utc::now().to_rfc3339();

        if let Some(name) = &tool.name {
            sqlx::query(r#"UPDATE tools SET name = ?, updated_at = ? WHERE id = ?"#)
                .bind(name)
                .bind(&now)
                .bind(id.to_string())
                .execute(&self.pool)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        }

        if let Some(description) = &tool.description {
            sqlx::query(r#"UPDATE tools SET description = ?, updated_at = ? WHERE id = ?"#)
                .bind(description)
                .bind(&now)
                .bind(id.to_string())
                .execute(&self.pool)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        }

        if let Some(input_schema) = &tool.input_schema {
            let input_schema_str = serde_json::to_string(input_schema).map_err(|e| {
                AppError::ValidationError(format!("Failed to serialize input_schema: {}", e))
            })?;
            sqlx::query(r#"UPDATE tools SET input_schema = ?, updated_at = ? WHERE id = ?"#)
                .bind(&input_schema_str)
                .bind(&now)
                .bind(id.to_string())
                .execute(&self.pool)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        }

        if let Some(output_schema) = &tool.output_schema {
            let output_schema_str = serde_json::to_string(output_schema).map_err(|e| {
                AppError::ValidationError(format!("Failed to serialize output_schema: {}", e))
            })?;
            sqlx::query(r#"UPDATE tools SET output_schema = ?, updated_at = ? WHERE id = ?"#)
                .bind(&output_schema_str)
                .bind(&now)
                .bind(id.to_string())
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
                r#"UPDATE tools SET handler_type = ?, handler_url = ?, handler_method = ?, handler_timeout = ?, updated_at = ? WHERE id = ?"#
            )
            .bind(handler_type_str)
            .bind(&handler.url)
            .bind(&handler.method)
            .bind(handler.timeout.map(|t| t as i32))
            .bind(&now)
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        }

        if let Some(visibility) = &tool.visibility {
            let visibility_str = match visibility {
                Visibility::Public => "public",
                Visibility::Private => "private",
            };
            sqlx::query(r#"UPDATE tools SET visibility = ?, updated_at = ? WHERE id = ?"#)
                .bind(visibility_str)
                .bind(&now)
                .bind(id.to_string())
                .execute(&self.pool)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        }

        self.find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFoundError(format!("Tool {} not found", id)))
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query(r#"DELETE FROM tools WHERE id = ?"#)
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct ToolRow {
    id: String,
    name: String,
    description: String,
    input_schema: String,
    output_schema: Option<String>,
    handler_type: String,
    handler_url: Option<String>,
    handler_method: Option<String>,
    handler_timeout: Option<i32>,
    visibility: String,
    owner_id: String,
    tenant_id: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<ToolRow> for Tool {
    fn from(row: ToolRow) -> Self {
        Tool {
            id: row.id.parse().unwrap_or_default(),
            name: row.name,
            description: row.description,
            input_schema: serde_json::from_str(&row.input_schema).unwrap_or_default(),
            output_schema: row
                .output_schema
                .and_then(|s| serde_json::from_str(&s).ok()),
            handler: HandlerConfig {
                handler_type: match row.handler_type.as_str() {
                    "http" => HandlerType::Http,
                    _ => HandlerType::Function,
                },
                url: row.handler_url,
                method: row.handler_method,
                timeout: row.handler_timeout.map(|t| t as u32),
            },
            owner_id: row.owner_id.parse().unwrap_or_default(),
            tenant_id: row
                .tenant_id
                .unwrap_or_default()
                .parse()
                .unwrap_or_default(),
            visibility: match row.visibility.as_str() {
                "public" => Visibility::Public,
                _ => Visibility::Private,
            },
            created_at: DateTime::parse_from_rfc3339(&row.created_at)
                .unwrap_or_else(|_| Utc::now().into())
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&row.updated_at)
                .unwrap_or_else(|_| Utc::now().into())
                .with_timezone(&Utc),
        }
    }
}

#[derive(sqlx::FromRow)]
struct CountRow {
    count: i64,
}
