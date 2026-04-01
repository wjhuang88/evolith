use async_trait::async_trait;
use chrono::{DateTime, Utc};
use common::error::{AppError, Result};
use domain::repository::SkillRepository;
use domain::skill::{Dependency, NewSkill, Runtime, Skill, SkillFilter, Visibility};
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct SqliteSkillRepository {
    pool: SqlitePool,
}

impl SqliteSkillRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SkillRepository for SqliteSkillRepository {
    async fn create(&self, skill: NewSkill, owner_id: Uuid, tenant_id: Uuid) -> Result<Skill> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let visibility = skill.visibility.unwrap_or_default();
        let visibility_str = match &visibility {
            Visibility::Public => "public",
            Visibility::Private => "private",
        };
        let runtime_str = match skill.runtime {
            Runtime::Python311 => "python311",
            Runtime::Node20 => "node20",
            Runtime::Wasm => "wasm",
        };
        let dependencies_str = serde_json::to_string(&skill.dependencies).map_err(|e| {
            AppError::ValidationError(format!("Failed to serialize dependencies: {}", e))
        })?;

        sqlx::query(
            r#"
            INSERT INTO skills (
                id, name, version, description, skill_md, code_package_path,
                runtime, dependencies, visibility, owner_id, tenant_id, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(id.to_string())
        .bind(&skill.name)
        .bind(&skill.version)
        .bind(&skill.description)
        .bind(&skill.skill_md)
        .bind(&skill.code_package_path)
        .bind(runtime_str)
        .bind(&dependencies_str)
        .bind(visibility_str)
        .bind(owner_id.to_string())
        .bind(tenant_id.to_string())
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(Skill {
            id,
            name: skill.name,
            version: skill.version,
            description: skill.description,
            skill_md: skill.skill_md,
            code_package_path: skill.code_package_path,
            runtime: skill.runtime,
            dependencies: skill.dependencies,
            owner_id,
            tenant_id,
            visibility,
            created_at: now,
            updated_at: now,
        })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Skill>> {
        let row = sqlx::query_as::<_, SkillRow>(r#"SELECT * FROM skills WHERE id = ?"#)
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| r.into()))
    }

    async fn find_by_name_and_version(&self, name: &str, version: &str) -> Result<Option<Skill>> {
        let row =
            sqlx::query_as::<_, SkillRow>(r#"SELECT * FROM skills WHERE name = ? AND version = ?"#)
                .bind(name)
                .bind(version)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| r.into()))
    }

    async fn find_all(&self, filter: SkillFilter) -> Result<Vec<Skill>> {
        let mut sql = String::from("SELECT * FROM skills WHERE 1=1");
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

        if let Some(ref runtime) = filter.runtime {
            sql.push_str(" AND runtime = ?");
            let runtime_str = match runtime {
                Runtime::Python311 => "python311",
                Runtime::Node20 => "node20",
                Runtime::Wasm => "wasm",
            };
            bindings.push(runtime_str.to_string());
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

        let mut query = sqlx::query_as::<_, SkillRow>(&sql);
        for binding in bindings {
            query = query.bind(binding);
        }

        let rows = query
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn count(&self, filter: &SkillFilter) -> Result<u32> {
        let mut sql = String::from("SELECT COUNT(*) as count FROM skills WHERE 1=1");
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

        if let Some(ref runtime) = filter.runtime {
            sql.push_str(" AND runtime = ?");
            let runtime_str = match runtime {
                Runtime::Python311 => "python311",
                Runtime::Node20 => "node20",
                Runtime::Wasm => "wasm",
            };
            bindings.push(runtime_str.to_string());
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
        sqlx::query(r#"DELETE FROM skills WHERE id = ?"#)
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct SkillRow {
    id: String,
    name: String,
    version: String,
    description: String,
    skill_md: String,
    code_package_path: Option<String>,
    runtime: String,
    dependencies: Option<String>,
    visibility: String,
    owner_id: String,
    tenant_id: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<SkillRow> for Skill {
    fn from(row: SkillRow) -> Self {
        Skill {
            id: row.id.parse().unwrap_or_default(),
            name: row.name,
            version: row.version,
            description: row.description,
            skill_md: row.skill_md,
            code_package_path: row.code_package_path,
            runtime: match row.runtime.as_str() {
                "python311" => Runtime::Python311,
                "node20" => Runtime::Node20,
                _ => Runtime::Wasm,
            },
            dependencies: row
                .dependencies
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default(),
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
