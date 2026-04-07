use async_trait::async_trait;
use chrono::{DateTime, Utc};
use common::error::{AppError, Result};
use domain::repository::SnippetRepository;
use domain::snippet::{NewSnippet, Snippet, SnippetFilter, Visibility};
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

        let estimated_tokens = Self::estimate_tokens(&snippet.content, &snippet.code);

        sqlx::query(
            r#"
            INSERT INTO snippets (
                id, name, language, framework, tags, content, code,
                dependencies, estimated_tokens, visibility, owner_id, tenant_id, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
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

        let mut query = sqlx::query_as::<_, SnippetRow>(&sql);
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

        let mut query = sqlx::query_as::<_, CountRow>(&sql);
        for binding in bindings {
            query = query.bind(binding);
        }

        let row = query
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.count as u32)
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
