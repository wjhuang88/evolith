use async_trait::async_trait;
use chrono::{DateTime, Utc};
use common::error::{AppError, Result};
use domain::repository::SkillRepository;
use domain::skill::{NewSkill, Runtime, Skill, SkillFilter, UpdateSkill, Visibility};
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
        let tags_str = serde_json::to_string(&skill.tags)
            .map_err(|e| AppError::ValidationError(format!("Failed to serialize tags: {}", e)))?;
        let permissions_str = skill
            .permissions
            .as_ref()
            .map(serde_json::to_string)
            .transpose()
            .map_err(|e| {
                AppError::ValidationError(format!("Failed to serialize permissions: {}", e))
            })?;
        let disable_model_invocation = if skill.disable_model_invocation { 1 } else { 0 };
        let user_invocable = if skill.user_invocable { 1 } else { 0 };

        sqlx::query(
            r#"
            INSERT INTO skills (
                id, name, version, description, skill_md, code_package_path,
                runtime, dependencies, visibility, owner_id, tenant_id,
                author, tags, skill_type, execution, entrypoint, timeout, memory_mb,
                permissions, license, compatibility, disable_model_invocation,
                user_invocable, argument_hint,
                created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
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
        .bind(&skill.author)
        .bind(&tags_str)
        .bind(&skill.skill_type)
        .bind(&skill.execution)
        .bind(&skill.entrypoint)
        .bind(skill.timeout)
        .bind(skill.memory_mb)
        .bind(&permissions_str)
        .bind(&skill.license)
        .bind(&skill.compatibility)
        .bind(disable_model_invocation)
        .bind(user_invocable)
        .bind(&skill.argument_hint)
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
            author: skill.author,
            tags: skill.tags,
            skill_type: skill.skill_type,
            execution: skill.execution,
            entrypoint: skill.entrypoint,
            timeout: skill.timeout,
            memory_mb: skill.memory_mb,
            permissions: skill.permissions,
            license: skill.license,
            compatibility: skill.compatibility,
            disable_model_invocation: skill.disable_model_invocation,
            user_invocable: skill.user_invocable,
            argument_hint: skill.argument_hint,
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

        let mut query = sqlx::query_as::<_, SkillRow>(sqlx::AssertSqlSafe(sql.as_str()));
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
        let author = skill.author.or(existing.author);
        let tags = skill.tags.unwrap_or(existing.tags);
        let skill_type = skill.skill_type.unwrap_or(existing.skill_type);
        let execution = skill.execution.unwrap_or(existing.execution);
        let entrypoint = skill.entrypoint.or(existing.entrypoint);
        let timeout = skill.timeout.unwrap_or(existing.timeout);
        let memory_mb = skill.memory_mb.unwrap_or(existing.memory_mb);
        let permissions = skill.permissions.or(existing.permissions);
        let license = skill.license.or(existing.license);
        let compatibility = skill.compatibility.or(existing.compatibility);
        let disable_model_invocation = skill
            .disable_model_invocation
            .unwrap_or(existing.disable_model_invocation);
        let user_invocable = skill.user_invocable.unwrap_or(existing.user_invocable);
        let argument_hint = skill.argument_hint.or(existing.argument_hint);

        let runtime_str = match runtime {
            Runtime::Python311 => "python311",
            Runtime::Node20 => "node20",
            Runtime::Wasm => "wasm",
        };
        let visibility_str = match visibility {
            Visibility::Public => "public",
            Visibility::Private => "private",
        };
        let dependencies_str = serde_json::to_string(&dependencies).map_err(|e| {
            AppError::ValidationError(format!("Failed to serialize dependencies: {}", e))
        })?;
        let tags_str = serde_json::to_string(&tags)
            .map_err(|e| AppError::ValidationError(format!("Failed to serialize tags: {}", e)))?;
        let permissions_str = permissions
            .as_ref()
            .map(serde_json::to_string)
            .transpose()
            .map_err(|e| {
                AppError::ValidationError(format!("Failed to serialize permissions: {}", e))
            })?;
        let disable_model_invocation_int = if disable_model_invocation { 1 } else { 0 };
        let user_invocable_int = if user_invocable { 1 } else { 0 };

        sqlx::query(
            r#"
            UPDATE skills SET
                name = ?, version = ?, description = ?, skill_md = ?,
                runtime = ?, dependencies = ?, visibility = ?,
                author = ?, tags = ?, skill_type = ?, execution = ?, entrypoint = ?,
                timeout = ?, memory_mb = ?, permissions = ?, license = ?,
                compatibility = ?, disable_model_invocation = ?, user_invocable = ?,
                argument_hint = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&name)
        .bind(&version)
        .bind(&description)
        .bind(&skill_md)
        .bind(runtime_str)
        .bind(&dependencies_str)
        .bind(visibility_str)
        .bind(&author)
        .bind(&tags_str)
        .bind(&skill_type)
        .bind(&execution)
        .bind(&entrypoint)
        .bind(timeout)
        .bind(memory_mb)
        .bind(&permissions_str)
        .bind(&license)
        .bind(&compatibility)
        .bind(disable_model_invocation_int)
        .bind(user_invocable_int)
        .bind(&argument_hint)
        .bind(now.to_rfc3339())
        .bind(id.to_string())
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
            author,
            tags,
            skill_type,
            execution,
            entrypoint,
            timeout,
            memory_mb,
            permissions,
            license,
            compatibility,
            disable_model_invocation,
            user_invocable,
            argument_hint,
            created_at: existing.created_at,
            updated_at: now,
        })
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
    author: Option<String>,
    tags: Option<String>,
    skill_type: Option<String>,
    execution: Option<String>,
    entrypoint: Option<String>,
    timeout: Option<i32>,
    memory_mb: Option<i32>,
    permissions: Option<String>,
    license: Option<String>,
    compatibility: Option<String>,
    disable_model_invocation: Option<i32>,
    user_invocable: Option<i32>,
    argument_hint: Option<String>,
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
            author: row.author,
            tags: row
                .tags
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default(),
            skill_type: row.skill_type.unwrap_or_else(|| "instruction".to_string()),
            execution: row.execution.unwrap_or_else(|| "client".to_string()),
            entrypoint: row.entrypoint,
            timeout: row.timeout.unwrap_or(30),
            memory_mb: row.memory_mb.unwrap_or(256),
            permissions: row.permissions.and_then(|s| serde_json::from_str(&s).ok()),
            license: row.license,
            compatibility: row.compatibility,
            disable_model_invocation: row.disable_model_invocation.unwrap_or(0) != 0,
            user_invocable: row.user_invocable.unwrap_or(1) != 0,
            argument_hint: row.argument_hint,
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
