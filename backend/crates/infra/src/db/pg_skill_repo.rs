//! PostgreSQL Skill Repository implementation

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use common::error::{AppError, Result};
use domain::repository::SkillRepository;
use domain::skill::{NewSkill, Runtime, Skill, SkillFilter, UpdateSkill, Visibility};
use sqlx::PgPool;
use uuid::Uuid;

pub struct PgSkillRepository {
    pool: PgPool,
}

impl PgSkillRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SkillRepository for PgSkillRepository {
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

        sqlx::query(
            r#"
            INSERT INTO skills (
                id, name, version, description, skill_md, code_package_path,
                runtime, dependencies, visibility, owner_id, tenant_id, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            "#,
        )
        .bind(id)
        .bind(&skill.name)
        .bind(&skill.version)
        .bind(&skill.description)
        .bind(&skill.skill_md)
        .bind(&skill.code_package_path)
        .bind(runtime_str)
        .bind(serde_json::to_value(&skill.dependencies).map_err(|e| {
            AppError::ValidationError(format!("Failed to serialize dependencies: {}", e))
        })?)
        .bind(visibility_str)
        .bind(owner_id)
        .bind(tenant_id)
        .bind(now)
        .bind(now)
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
        let row = sqlx::query_as::<_, PgSkillRow>(r#"SELECT * FROM skills WHERE id = $1"#)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| r.into()))
    }

    async fn find_by_name_and_version(&self, name: &str, version: &str) -> Result<Option<Skill>> {
        let row = sqlx::query_as::<_, PgSkillRow>(
            r#"SELECT * FROM skills WHERE name = $1 AND version = $2"#,
        )
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

        if let Some(ref runtime) = filter.runtime {
            sql.push_str(&format!(" AND runtime = ${}", param_idx));
            let runtime_str = match runtime {
                Runtime::Python311 => "python311",
                Runtime::Node20 => "node20",
                Runtime::Wasm => "wasm",
            };
            bindings.push(runtime_str.to_string());
            param_idx += 1;
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

        let mut query = sqlx::query_as::<_, PgSkillRow>(&sql);
        for binding in &bindings {
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

        if let Some(ref runtime) = filter.runtime {
            sql.push_str(&format!(" AND runtime = ${}", param_idx));
            let runtime_str = match runtime {
                Runtime::Python311 => "python311",
                Runtime::Node20 => "node20",
                Runtime::Wasm => "wasm",
            };
            bindings.push(runtime_str.to_string());
            param_idx += 1;
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

    async fn update(&self, id: Uuid, skill: UpdateSkill) -> Result<Skill> {
        let existing = self.find_by_id(id).await?;
        let existing = existing
            .ok_or_else(|| AppError::NotFoundError(format!("Skill with id {} not found", id)))?;

        let now = Utc::now();

        let name = skill.name.unwrap_or(existing.name);
        let version = skill.version.unwrap_or(existing.version);
        let description = skill.description.unwrap_or(existing.description);
        let skill_md = skill.skill_md.unwrap_or(existing.skill_md);
        let runtime = skill.runtime.unwrap_or(existing.runtime);
        let dependencies = skill.dependencies.unwrap_or(existing.dependencies);
        let visibility = skill.visibility.unwrap_or(existing.visibility);

        let runtime_str = match runtime {
            Runtime::Python311 => "python311",
            Runtime::Node20 => "node20",
            Runtime::Wasm => "wasm",
        };
        let visibility_str = match visibility {
            Visibility::Public => "public",
            Visibility::Private => "private",
        };

        sqlx::query(
            r#"
            UPDATE skills SET
                name = $1, version = $2, description = $3, skill_md = $4,
                runtime = $5, dependencies = $6, visibility = $7, updated_at = $8
            WHERE id = $9
            "#,
        )
        .bind(&name)
        .bind(&version)
        .bind(&description)
        .bind(&skill_md)
        .bind(runtime_str)
        .bind(serde_json::to_value(&dependencies).map_err(|e| {
            AppError::ValidationError(format!("Failed to serialize dependencies: {}", e))
        })?)
        .bind(visibility_str)
        .bind(now)
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(Skill {
            id,
            name,
            version,
            description,
            skill_md,
            code_package_path: existing.code_package_path,
            runtime,
            dependencies,
            owner_id: existing.owner_id,
            tenant_id: existing.tenant_id,
            visibility,
            created_at: existing.created_at,
            updated_at: now,
        })
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query(r#"DELETE FROM skills WHERE id = $1"#)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct PgSkillRow {
    id: Uuid,
    name: String,
    version: String,
    description: String,
    skill_md: String,
    code_package_path: Option<String>,
    runtime: String,
    dependencies: Option<serde_json::Value>,
    visibility: String,
    owner_id: Uuid,
    tenant_id: Uuid,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<PgSkillRow> for Skill {
    fn from(row: PgSkillRow) -> Self {
        Skill {
            id: row.id,
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
                .map(|v| serde_json::from_value(v).unwrap_or_default())
                .unwrap_or_default(),
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
