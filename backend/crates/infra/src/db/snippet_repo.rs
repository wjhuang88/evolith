use async_trait::async_trait;
use chrono::{DateTime, Utc};
use common::error::{AppError, Result};
use domain::repository::SnippetRepository;
use domain::snippet::{NewSnippet, Snippet, SnippetFilter, UpdateSnippet, Visibility};
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct SqliteSnippetRepository {
    pool: SqlitePool,
}

impl SqliteSnippetRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SnippetRepository for SqliteSnippetRepository {
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
        let tags_str = serde_json::to_string(&snippet.tags)
            .map_err(|e| AppError::ValidationError(format!("Failed to serialize tags: {}", e)))?;
        let dependencies_str = serde_json::to_string(&snippet.dependencies).map_err(|e| {
            AppError::ValidationError(format!("Failed to serialize dependencies: {}", e))
        })?;
        let subcommands_str = serde_json::to_string(&snippet.subcommands).map_err(|e| {
            AppError::ValidationError(format!("Failed to serialize subcommands: {}", e))
        })?;
        let inputs_str = serde_json::to_string(&snippet.inputs)
            .map_err(|e| AppError::ValidationError(format!("Failed to serialize inputs: {}", e)))?;
        let output_str = snippet
            .output
            .as_ref()
            .map(serde_json::to_string)
            .transpose()
            .map_err(|e| AppError::ValidationError(format!("Failed to serialize output: {}", e)))?;
        let examples_str = serde_json::to_string(&snippet.examples).map_err(|e| {
            AppError::ValidationError(format!("Failed to serialize examples: {}", e))
        })?;
        let error_model_str = snippet
            .error_model
            .as_ref()
            .map(serde_json::to_string)
            .transpose()
            .map_err(|e| {
                AppError::ValidationError(format!("Failed to serialize error_model: {}", e))
            })?;

        let estimated_tokens = Self::estimate_tokens(&snippet.content, &snippet.code);

        sqlx::query(
            r#"
            INSERT INTO snippets (
                id, name, language, framework, tags, content, code,
                dependencies, estimated_tokens, visibility, owner_id, tenant_id,
                version, summary, command, subcommands, inputs, output, examples, error_model,
                created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(id.to_string())
        .bind(&snippet.name)
        .bind(&snippet.language)
        .bind(&snippet.framework)
        .bind(&tags_str)
        .bind(&snippet.content)
        .bind(&snippet.code)
        .bind(&dependencies_str)
        .bind(estimated_tokens as i32)
        .bind(visibility_str)
        .bind(owner_id.to_string())
        .bind(tenant_id.to_string())
        .bind(&snippet.version)
        .bind(&snippet.summary)
        .bind(&snippet.command)
        .bind(&subcommands_str)
        .bind(&inputs_str)
        .bind(&output_str)
        .bind(&examples_str)
        .bind(&error_model_str)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
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
        let row = sqlx::query_as::<_, SnippetRow>(r#"SELECT * FROM snippets WHERE id = ?"#)
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| r.into()))
    }

    async fn find_all(&self, filter: SnippetFilter) -> Result<Vec<Snippet>> {
        let mut sql = String::from("SELECT * FROM snippets WHERE 1=1");
        let mut bindings: Vec<String> = Vec::new();

        if let Some(tenant_id) = filter.tenant_id {
            sql.push_str(" AND tenant_id = ?");
            bindings.push(tenant_id.to_string());
        }

        if let Some(ref search) = filter.search {
            sql.push_str(" AND (name LIKE ? OR code LIKE ? OR content LIKE ?)");
            let search_pattern = format!("%{}%", search);
            bindings.push(search_pattern.clone());
            bindings.push(search_pattern.clone());
            bindings.push(search_pattern);
        }

        if let Some(ref language) = filter.language {
            sql.push_str(" AND language = ?");
            bindings.push(language.clone());
        }

        if let Some(ref framework) = filter.framework {
            sql.push_str(" AND framework = ?");
            bindings.push(framework.clone());
        }

        if !filter.tags.is_empty() {
            for tag in &filter.tags {
                sql.push_str(" AND tags LIKE ?");
                bindings.push(format!("%\"{}\"", tag));
            }
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

        let mut query = sqlx::query_as::<_, SnippetRow>(sqlx::AssertSqlSafe(sql.as_str()));
        for binding in bindings {
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

        if let Some(tenant_id) = filter.tenant_id {
            sql.push_str(" AND tenant_id = ?");
            bindings.push(tenant_id.to_string());
        }

        if let Some(ref search) = filter.search {
            sql.push_str(" AND (name LIKE ? OR code LIKE ? OR content LIKE ?)");
            let search_pattern = format!("%{}%", search);
            bindings.push(search_pattern.clone());
            bindings.push(search_pattern.clone());
            bindings.push(search_pattern);
        }

        if let Some(ref language) = filter.language {
            sql.push_str(" AND language = ?");
            bindings.push(language.clone());
        }

        if let Some(ref framework) = filter.framework {
            sql.push_str(" AND framework = ?");
            bindings.push(framework.clone());
        }

        if !filter.tags.is_empty() {
            for tag in &filter.tags {
                sql.push_str(" AND tags LIKE ?");
                bindings.push(format!("%\"{}\"", tag));
            }
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

    async fn update(&self, id: Uuid, snippet: UpdateSnippet) -> Result<Snippet> {
        let existing = self.find_by_id(id).await?;
        let existing = existing
            .ok_or_else(|| AppError::NotFoundError(format!("Snippet with id {} not found", id)))?;

        let now = Utc::now();

        let name = snippet.name.unwrap_or(existing.name);
        let language = snippet.language.unwrap_or(existing.language);
        let framework = snippet.framework.or(existing.framework);
        let tags = snippet.tags.unwrap_or(existing.tags);
        let content = snippet.content.unwrap_or(existing.content);
        let code = snippet.code.unwrap_or(existing.code);
        let dependencies = snippet.dependencies.unwrap_or(existing.dependencies);
        let estimated_tokens = snippet
            .estimated_tokens
            .unwrap_or(existing.estimated_tokens);
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
        let tags_str = serde_json::to_string(&tags)
            .map_err(|e| AppError::ValidationError(format!("Failed to serialize tags: {}", e)))?;
        let dependencies_str = serde_json::to_string(&dependencies).map_err(|e| {
            AppError::ValidationError(format!("Failed to serialize dependencies: {}", e))
        })?;
        let subcommands_str = serde_json::to_string(&subcommands).map_err(|e| {
            AppError::ValidationError(format!("Failed to serialize subcommands: {}", e))
        })?;
        let inputs_str = serde_json::to_string(&inputs)
            .map_err(|e| AppError::ValidationError(format!("Failed to serialize inputs: {}", e)))?;
        let output_str = output
            .as_ref()
            .map(serde_json::to_string)
            .transpose()
            .map_err(|e| AppError::ValidationError(format!("Failed to serialize output: {}", e)))?;
        let examples_str = serde_json::to_string(&examples).map_err(|e| {
            AppError::ValidationError(format!("Failed to serialize examples: {}", e))
        })?;
        let error_model_str = error_model
            .as_ref()
            .map(serde_json::to_string)
            .transpose()
            .map_err(|e| {
                AppError::ValidationError(format!("Failed to serialize error_model: {}", e))
            })?;

        sqlx::query(
            r#"
            UPDATE snippets SET
                name = ?, language = ?, framework = ?, tags = ?, content = ?, code = ?,
                dependencies = ?, estimated_tokens = ?, visibility = ?,
                version = ?, summary = ?, command = ?, subcommands = ?, inputs = ?,
                output = ?, examples = ?, error_model = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&name)
        .bind(&language)
        .bind(&framework)
        .bind(&tags_str)
        .bind(&content)
        .bind(&code)
        .bind(&dependencies_str)
        .bind(estimated_tokens as i32)
        .bind(visibility_str)
        .bind(&version)
        .bind(&summary)
        .bind(&command)
        .bind(&subcommands_str)
        .bind(&inputs_str)
        .bind(&output_str)
        .bind(&examples_str)
        .bind(&error_model_str)
        .bind(now.to_rfc3339())
        .bind(id.to_string())
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
            examples,
            error_model,
            created_at: existing.created_at,
            updated_at: now,
        })
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query(r#"DELETE FROM snippets WHERE id = ?"#)
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }
}

impl SqliteSnippetRepository {
    fn estimate_tokens(content: &str, code: &str) -> u32 {
        let total_chars = content.chars().count() + code.chars().count();
        (total_chars / 4) as u32
    }
}

#[derive(sqlx::FromRow)]
struct SnippetRow {
    id: String,
    name: String,
    language: String,
    framework: Option<String>,
    tags: Option<String>,
    content: String,
    code: String,
    dependencies: Option<String>,
    estimated_tokens: i32,
    visibility: String,
    owner_id: String,
    tenant_id: Option<String>,
    version: Option<String>,
    summary: Option<String>,
    command: Option<String>,
    subcommands: Option<String>,
    inputs: Option<String>,
    output: Option<String>,
    examples: Option<String>,
    error_model: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<SnippetRow> for Snippet {
    fn from(row: SnippetRow) -> Self {
        Snippet {
            id: row.id.parse().unwrap_or_default(),
            name: row.name,
            language: row.language,
            framework: row.framework,
            tags: row
                .tags
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default(),
            content: row.content,
            code: row.code,
            dependencies: row
                .dependencies
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default(),
            estimated_tokens: row.estimated_tokens as u32,
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
            version: row.version.unwrap_or_else(|| "1.0.0".to_string()),
            summary: row.summary,
            command: row.command,
            subcommands: row
                .subcommands
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default(),
            inputs: row
                .inputs
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default(),
            output: row.output.and_then(|s| serde_json::from_str(&s).ok()),
            examples: row
                .examples
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default(),
            error_model: row.error_model.and_then(|s| serde_json::from_str(&s).ok()),
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
