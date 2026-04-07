//! SQLite Tenant Repository implementation

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use common::error::{AppError, Result};
use domain::repository::TenantRepository;
use domain::tenant::{
    CreateTenantRequest, PlanStatus, Tenant, TenantPlan, TenantQuotas, TenantStatus, TenantUsage,
};
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct SqliteTenantRepository {
    pool: SqlitePool,
}

impl SqliteTenantRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TenantRepository for SqliteTenantRepository {
    async fn create(&self, tenant: CreateTenantRequest, owner_id: Uuid) -> Result<Tenant> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let quotas = TenantQuotas::for_plan(&TenantPlan::Free);

        sqlx::query(
            r#"
            INSERT INTO tenants (
                id, name, slug, owner_id, plan, plan_status,
                max_users, max_tools, max_skills, max_snippets, 
                max_api_calls_per_month, max_storage_mb,
                current_users, status, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(id.to_string())
        .bind(&tenant.name)
        .bind(&tenant.slug)
        .bind(owner_id.to_string())
        .bind("free")
        .bind("active")
        .bind(quotas.max_users)
        .bind(quotas.max_tools)
        .bind(quotas.max_skills)
        .bind(quotas.max_snippets)
        .bind(quotas.max_api_calls_per_month)
        .bind(quotas.max_storage_mb)
        .bind(1i32) // current_users = 1 (the owner)
        .bind("active")
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(Tenant {
            id,
            name: tenant.name,
            slug: tenant.slug,
            domain: None,
            owner_id,
            billing_email: Some(tenant.owner_email),
            plan: TenantPlan::Free,
            plan_status: PlanStatus::Active,
            quotas,
            usage: TenantUsage {
                current_users: 1,
                ..Default::default()
            },
            settings: serde_json::json!({}),
            features: serde_json::json!({}),
            status: TenantStatus::Active,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Tenant>> {
        let row = sqlx::query_as::<_, TenantRow>(r#"SELECT * FROM tenants WHERE id = ?"#)
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| r.into()))
    }

    async fn find_by_slug(&self, slug: &str) -> Result<Option<Tenant>> {
        let row = sqlx::query_as::<_, TenantRow>(r#"SELECT * FROM tenants WHERE slug = ?"#)
            .bind(slug)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| r.into()))
    }

    async fn find_by_domain(&self, domain: &str) -> Result<Option<Tenant>> {
        let row = sqlx::query_as::<_, TenantRow>(r#"SELECT * FROM tenants WHERE domain = ?"#)
            .bind(domain)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| r.into()))
    }

    async fn update(&self, id: Uuid, tenant: CreateTenantRequest) -> Result<Tenant> {
        let now = Utc::now().to_rfc3339();

        sqlx::query(r#"UPDATE tenants SET name = ?, slug = ?, updated_at = ? WHERE id = ?"#)
            .bind(&tenant.name)
            .bind(&tenant.slug)
            .bind(&now)
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        self.find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFoundError(format!("Tenant {} not found", id)))
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(r#"UPDATE tenants SET status = ?, deleted_at = ? WHERE id = ?"#)
            .bind("deleted")
            .bind(&now)
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn update_usage(&self, id: Uuid) -> Result<()> {
        // Recalculate usage from related tables
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            r#"
            UPDATE tenants SET
                current_users = (SELECT COUNT(*) FROM users WHERE tenant_id = ?),
                current_tools = (SELECT COUNT(*) FROM tools WHERE tenant_id = ?),
                current_skills = (SELECT COUNT(*) FROM skills WHERE tenant_id = ?),
                current_snippets = (SELECT COUNT(*) FROM snippets WHERE tenant_id = ?),
                updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(id.to_string())
        .bind(id.to_string())
        .bind(id.to_string())
        .bind(id.to_string())
        .bind(&now)
        .bind(id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}

#[derive(sqlx::FromRow)]
#[allow(dead_code)]
struct TenantRow {
    id: String,
    name: String,
    slug: String,
    domain: Option<String>,
    owner_id: String,
    billing_email: Option<String>,
    plan: String,
    plan_status: String,
    max_users: i32,
    max_tools: i32,
    max_skills: i32,
    max_snippets: i32,
    max_api_calls_per_month: i32,
    max_storage_mb: i32,
    current_users: i32,
    current_tools: i32,
    current_skills: i32,
    current_snippets: i32,
    current_api_calls: i32,
    current_storage_mb: i32,
    settings: String,
    features: String,
    status: String,
    created_at: String,
    updated_at: String,
    deleted_at: Option<String>,
}

impl From<TenantRow> for Tenant {
    fn from(row: TenantRow) -> Self {
        let plan = match row.plan.as_str() {
            "starter" => TenantPlan::Starter,
            "pro" => TenantPlan::Pro,
            "enterprise" => TenantPlan::Enterprise,
            _ => TenantPlan::Free,
        };

        Tenant {
            id: row.id.parse().unwrap_or_default(),
            name: row.name,
            slug: row.slug,
            domain: row.domain,
            owner_id: row.owner_id.parse().unwrap_or_default(),
            billing_email: row.billing_email,
            plan: plan.clone(),
            plan_status: match row.plan_status.as_str() {
                "past_due" => PlanStatus::PastDue,
                "cancelled" => PlanStatus::Cancelled,
                _ => PlanStatus::Active,
            },
            quotas: TenantQuotas::for_plan(&plan),
            usage: TenantUsage {
                current_users: row.current_users as u32,
                current_tools: row.current_tools as u32,
                current_skills: row.current_skills as u32,
                current_snippets: row.current_snippets as u32,
                current_api_calls: row.current_api_calls as u32,
                current_storage_mb: row.current_storage_mb as u32,
            },
            settings: serde_json::from_str(&row.settings).unwrap_or_default(),
            features: serde_json::from_str(&row.features).unwrap_or_default(),
            status: match row.status.as_str() {
                "suspended" => TenantStatus::Suspended,
                "deleted" => TenantStatus::Deleted,
                _ => TenantStatus::Active,
            },
            created_at: DateTime::parse_from_rfc3339(&row.created_at)
                .unwrap_or_else(|_| Utc::now().into())
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&row.updated_at)
                .unwrap_or_else(|_| Utc::now().into())
                .with_timezone(&Utc),
            deleted_at: row.deleted_at.and_then(|s| {
                DateTime::parse_from_rfc3339(&s)
                    .ok()
                    .map(|dt| dt.with_timezone(&Utc))
            }),
        }
    }
}
