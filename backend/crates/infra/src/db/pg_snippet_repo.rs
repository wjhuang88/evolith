//! PostgreSQL Snippet Repository implementation

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use common::error::{AppError, Result};
use domain::repository::SnippetRepository;
use domain::snippet::{NewSnippet, Snippet, SnippetFilter, UpdateSnippet, Visibility};
use sqlx::PgPool;
use uuid::Uuid;

pub struct PgSnippetRepository {
    pool: PgPool,
}

impl PgSnippetRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn estimate_tokens(content: &str, code: &str) -> u32 {
        let total_chars = content.chars().count() + code.chars().count();
        (total_chars / 4) as u32
    }
}

#[async_trait]
impl SnippetRepository for PgSnippetRepository {
    async fn create(
        &self,
        snippet: NewSnippet,
        owner_id: Uuid,
        tenant_id: Uuid,
    ) -> Result<Snippet> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let visibility = snippet.visibility.unwrap_or_default();
        let visibility_str = match &visibility {
            Visibility::Public => "public",
            Visibility::Private => "private",
        };

        let estimated_tokens = Self::estimate_tokens(&snippet.content, &snippet.code);

        sqlx::query(
            r#"
            INSERT INTO snippets (
                id, name, language, framework, tags, content, code,
                dependencies, estimated_tokens, visibility, owner_id, tenant_id,
                version, summary, command, subcommands, inputs, output, examples, error_model,
                created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22)
            "#,
        )
        .bind(id)
        .bind(&snippet.name)
        .bind(&snippet.language)
        .bind(&snippet.framework)
        .bind(serde_json::to_value(&snippet.tags).map_err(|e| {
            AppError::ValidationError(format!("Failed to serialize tags: {}", e))
        })?)
        .bind(&snippet.content)
        .bind(&snippet.code)
        .bind(serde_json::to_value(&snippet.dependencies).map_err(|e| {
            AppError::ValidationError(format!("Failed to serialize dependencies: {}", e))
        })?)
        .bind(estimated_tokens as i32)
        .bind(visibility_str)
        .bind(owner_id)
        .bind(tenant_id)
        .bind(&snippet.version)
        .bind(&snippet.summary)
        .bind(&snippet.command)
        .bind(serde_json::to_value(&snippet.subcommands).map_err(|e| {
            AppError::ValidationError(format!("Failed to serialize subcommands: {}", e))
        })?)
        .bind(serde_json::to_value(&snippet.inputs).map_err(|e| {
            AppError::ValidationError(format!("Failed to serialize inputs: {}", e))
        })?)
        .bind(snippet.output.as_ref().map(serde_json::to_value).transpose().map_err(|e| {
            AppError::ValidationError(format!("Failed to serialize output: {}", e))
        })?)
        .bind(serde_json::to_value(&snippet.examples).map_err(|e| {
            AppError::ValidationError(format!("Failed to serialize examples: {}", e))
        })?)
        .bind(snippet.error_model.as_ref().map(serde_json::to_value).transpose().map_err(|e| {
            AppError::ValidationError(format!("Failed to serialize error_model: {}", e))
        })?)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(Snippet {
            id,
            name: snippet.name,
            language: snippet.language,
            framework: snippet.framework,
            tags: snippet.tags,
            content: snippet.content,
            code: snippet.code,
            dependencies: snippet.dependencies,
            estimated_tokens,
            owner_id,
            tenant_id,
            visibility,
            version: snippet.version,
            summary: snippet.summary,
            command: snippet.command,
            subcommands: snippet.subcommands,
            inputs: snippet.inputs,
            output: snippet.output,
            examples: snippet.examples,
            error_model: snippet.error_model,
            created_at: now,
            updated_at: now,
        })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Snippet>> {
        let row = sqlx::query_as::<_, PgSnippetRow>(r#"SELECT * FROM snippets WHERE id = $1"#)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| r.into()))
    }

    async fn find_all(&self, filter: SnippetFilter) -> Result<Vec<Snippet>> {
        let mut sql = String::from("SELECT * FROM snippets WHERE 1=1");
        let mut bindings: Vec<String> = Vec::new();
        let mut param_idx = 1usize;

        if let Some(tenant_id) = filter.tenant_id {
            sql.push_str(&format!(" AND tenant_id = ${}", param_idx));
            param_idx += 1;
            bindings.push(tenant_id.to_string());
        }

        if let Some(ref search) = filter.search {
            sql.push_str(&format!(
                " AND (name LIKE ${} OR code LIKE ${} OR content LIKE ${})",
                param_idx,
                param_idx + 1,
                param_idx + 2
            ));
            let search_pattern = format!("%{}%", search);
            bindings.push(search_pattern.clone());
            bindings.push(search_pattern.clone());
            bindings.push(search_pattern);
            param_idx += 3;
        }

        if let Some(ref language) = filter.language {
            sql.push_str(&format!(" AND language = ${}", param_idx));
            param_idx += 1;
            bindings.push(language.clone());
        }

        if let Some(ref framework) = filter.framework {
            sql.push_str(&format!(" AND framework = ${}", param_idx));
            param_idx += 1;
            bindings.push(framework.clone());
        }

        if !filter.tags.is_empty() {
            for tag in &filter.tags {
                sql.push_str(&format!(" AND tags::text LIKE ${}", param_idx));
                param_idx += 1;
                bindings.push(format!("%\"{}\"", tag));
            }
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

        let mut query = sqlx::query_as::<_, PgSnippetRow>(&sql);
        for binding in &bindings {
            query = query.bind(binding);
        }

        let rows = query
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn count(&self, filter: &SnippetFilter) -> Result<u32> {
        let mut sql = String::from("SELECT COUNT(*) as count FROM snippets WHERE 1=1");
        let mut bindings: Vec<String> = Vec::new();
        let mut param_idx = 1usize;

        if let Some(tenant_id) = filter.tenant_id {
            sql.push_str(&format!(" AND tenant_id = ${}", param_idx));
            param_idx += 1;
            bindings.push(tenant_id.to_string());
        }

        if let Some(ref search) = filter.search {
            sql.push_str(&format!(
                " AND (name LIKE ${} OR code LIKE ${} OR content LIKE ${})",
                param_idx,
                param_idx + 1,
                param_idx + 2
            ));
            let search_pattern = format!("%{}%", search);
            bindings.push(search_pattern.clone());
            bindings.push(search_pattern.clone());
            bindings.push(search_pattern);
            param_idx += 3;
        }

        if let Some(ref language) = filter.language {
            sql.push_str(&format!(" AND language = ${}", param_idx));
            param_idx += 1;
            bindings.push(language.clone());
        }

        if let Some(ref framework) = filter.framework {
            sql.push_str(&format!(" AND framework = ${}", param_idx));
            param_idx += 1;
            bindings.push(framework.clone());
        }

        if !filter.tags.is_empty() {
            for tag in &filter.tags {
                sql.push_str(&format!(" AND tags::text LIKE ${}", param_idx));
                param_idx += 1;
                bindings.push(format!("%\"{}\"", tag));
            }
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

    async fn update(&self, id: Uuid, snippet: UpdateSnippet) -> Result<Snippet> {
        let existing = self.find_by_id(id).await?;
        let existing = existing.ok_or_else(|| {
            AppError::NotFoundError(format!("Snippet with id {} not found", id))
        })?;

        let now = Utc::now();

        let name = snippet.name.unwrap_or(existing.name);
        let language = snippet.language.unwrap_or(existing.language);
        let framework = snippet.framework.or(existing.framework);
        let tags = snippet.tags.unwrap_or(existing.tags);
        let content = snippet.content.unwrap_or(existing.content);
        let code = snippet.code.unwrap_or(existing.code);
        let dependencies = snippet.dependencies.unwrap_or(existing.dependencies);
        let estimated_tokens = snippet.estimated_tokens.unwrap_or(existing.estimated_tokens);
        let visibility = snippet.visibility.unwrap_or(existing.visibility);
        let version = snippet.version.unwrap_or(existing.version);
        let summary = snippet.summary.or(existing.summary);
        let command = snippet.command.or(existing.command);
        let subcommands = snippet.subcommands.unwrap_or(existing.subcommands);
        let inputs = snippet.inputs.unwrap_or(existing.inputs);
        let output = snippet.output.or(existing.output);
        let examples = snippet.examples.unwrap_or(existing.examples);
        let error_model = snippet.error_model.or(existing.error_model);

        let visibility_str = match &visibility {
            Visibility::Public => "public",
            Visibility::Private => "private",
        };

        sqlx::query(
            r#"
            UPDATE snippets SET
                name = $1, language = $2, framework = $3, tags = $4, content = $5, code = $6,
                dependencies = $7, estimated_tokens = $8, visibility = $9,
                version = $10, summary = $11, command = $12, subcommands = $13, inputs = $14,
                output = $15, examples = $16, error_model = $17, updated_at = $18
            WHERE id = $19
            "#,
        )
        .bind(&name)
        .bind(&language)
        .bind(&framework)
        .bind(serde_json::to_value(&tags).map_err(|e| {
            AppError::ValidationError(format!("Failed to serialize tags: {}", e))
        })?)
        .bind(&content)
        .bind(&code)
        .bind(serde_json::to_value(&dependencies).map_err(|e| {
            AppError::ValidationError(format!("Failed to serialize dependencies: {}", e))
        })?)
        .bind(estimated_tokens as i32)
        .bind(visibility_str)
        .bind(&version)
        .bind(&summary)
        .bind(&command)
        .bind(serde_json::to_value(&subcommands).map_err(|e| {
            AppError::ValidationError(format!("Failed to serialize subcommands: {}", e))
        })?)
        .bind(serde_json::to_value(&inputs).map_err(|e| {
            AppError::ValidationError(format!("Failed to serialize inputs: {}", e))
        })?)
        .bind(output.as_ref().map(serde_json::to_value).transpose().map_err(|e| {
            AppError::ValidationError(format!("Failed to serialize output: {}", e))
        })?)
        .bind(serde_json::to_value(&examples).map_err(|e| {
            AppError::ValidationError(format!("Failed to serialize examples: {}", e))
        })?)
        .bind(error_model.as_ref().map(serde_json::to_value).transpose().map_err(|e| {
            AppError::ValidationError(format!("Failed to serialize error_model: {}", e))
        })?)
        .bind(now)
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(Snippet {
            id,
            name,
            language,
            framework,
            tags,
            content,
            code,
            dependencies,
            estimated_tokens,
            owner_id: existing.owner_id,
            tenant_id: existing.tenant_id,
            visibility,
            version,
            summary,
            command,
            subcommands,
            inputs,
            output,
            error_model,
            examples,
            created_at: existing.created_at,
            updated_at: now,
        })
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query(r#"DELETE FROM snippets WHERE id = $1"#)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct PgSnippetRow {
    id: Uuid,
    name: String,
    language: String,
    framework: Option<String>,
    tags: Option<serde_json::Value>,
    content: String,
    code: String,
    dependencies: Option<serde_json::Value>,
    estimated_tokens: i32,
    visibility: String,
    owner_id: Uuid,
    tenant_id: Uuid,
    version: Option<String>,
    summary: Option<String>,
    command: Option<String>,
    subcommands: Option<serde_json::Value>,
    inputs: Option<serde_json::Value>,
    output: Option<serde_json::Value>,
    examples: Option<serde_json::Value>,
    error_model: Option<serde_json::Value>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<PgSnippetRow> for Snippet {
    fn from(row: PgSnippetRow) -> Self {
        Snippet {
            id: row.id,
            name: row.name,
            language: row.language,
            framework: row.framework,
            tags: row
                .tags
                .map(|v| serde_json::from_value(v).unwrap_or_default())
                .unwrap_or_default(),
            content: row.content,
            code: row.code,
            dependencies: row
                .dependencies
                .map(|v| serde_json::from_value(v).unwrap_or_default())
                .unwrap_or_default(),
            estimated_tokens: row.estimated_tokens as u32,
            owner_id: row.owner_id,
            tenant_id: row.tenant_id,
            visibility: match row.visibility.as_str() {
                "public" => Visibility::Public,
                _ => Visibility::Private,
            },
            version: row.version.unwrap_or_else(|| "1.0.0".to_string()),
            summary: row.summary,
            command: row.command,
            subcommands: row
                .subcommands
                .map(|v| serde_json::from_value(v).unwrap_or_default())
                .unwrap_or_default(),
            inputs: row
                .inputs
                .map(|v| serde_json::from_value(v).unwrap_or_default())
                .unwrap_or_default(),
            output: row.output,
            examples: row
                .examples
                .map(|v| serde_json::from_value(v).unwrap_or_default())
                .unwrap_or_default(),
            error_model: row.error_model,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(sqlx::FromRow)]
struct CountRow {
    count: i64,
}
