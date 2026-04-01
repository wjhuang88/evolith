//! PostgreSQL Tenant Repository implementation

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use common::error::{AppError, Result};
use domain::repository::TenantRepository;
use domain::tenant::{
    CreateTenantRequest, PlanStatus, Tenant, TenantPlan, TenantQuotas, TenantStatus, TenantUsage,
};
use sqlx::PgPool;
use uuid::Uuid;

pub struct PgTenantRepository {
    pool: PgPool,
}

impl PgTenantRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TenantRepository for PgTenantRepository {
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
                current_users, status, settings, features, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)
            "#,
        )
        .bind(id)
        .bind(&tenant.name)
        .bind(&tenant.slug)
        .bind(owner_id)
        .bind("free")
        .bind("active")
        .bind(quotas.max_users as i32)
        .bind(quotas.max_tools as i32)
        .bind(quotas.max_skills as i32)
        .bind(quotas.max_snippets as i32)
        .bind(quotas.max_api_calls_per_month as i32)
        .bind(quotas.max_storage_mb as i32)
        .bind(1i32)
        .bind("active")
        .bind(serde_json::json!({}))
        .bind(serde_json::json!({}))
        .bind(now)
        .bind(now)
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
        let row = sqlx::query_as::<_, PgTenantRow>(r#"SELECT * FROM tenants WHERE id = $1"#)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| r.into()))
    }

    async fn find_by_slug(&self, slug: &str) -> Result<Option<Tenant>> {
        let row = sqlx::query_as::<_, PgTenantRow>(r#"SELECT * FROM tenants WHERE slug = $1"#)
            .bind(slug)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| r.into()))
    }

    async fn find_by_domain(&self, domain: &str) -> Result<Option<Tenant>> {
        let row = sqlx::query_as::<_, PgTenantRow>(r#"SELECT * FROM tenants WHERE domain = $1"#)
            .bind(domain)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| r.into()))
    }

    async fn update(&self, id: Uuid, tenant: CreateTenantRequest) -> Result<Tenant> {
        let now = Utc::now();

        sqlx::query(r#"UPDATE tenants SET name = $1, slug = $2, updated_at = $3 WHERE id = $4"#)
            .bind(&tenant.name)
            .bind(&tenant.slug)
            .bind(now)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        self.find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFoundError(format!("Tenant {} not found", id)))
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        let now = Utc::now();
        sqlx::query(r#"UPDATE tenants SET status = $1, deleted_at = $2 WHERE id = $3"#)
            .bind("deleted")
            .bind(now)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn update_usage(&self, id: Uuid) -> Result<()> {
        let now = Utc::now();

        sqlx::query(
            r#"
            UPDATE tenants SET
                current_users = (SELECT COUNT(*) FROM users WHERE tenant_id = $1),
                current_tools = (SELECT COUNT(*) FROM tools WHERE tenant_id = $2),
                current_skills = (SELECT COUNT(*) FROM skills WHERE tenant_id = $3),
                current_snippets = (SELECT COUNT(*) FROM snippets WHERE tenant_id = $4),
                updated_at = $5
            WHERE id = $6
            "#,
        )
        .bind(id)
        .bind(id)
        .bind(id)
        .bind(id)
        .bind(now)
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct PgTenantRow {
    id: Uuid,
    name: String,
    slug: String,
    domain: Option<String>,
    owner_id: Uuid,
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
    settings: serde_json::Value,
    features: serde_json::Value,
    status: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    deleted_at: Option<DateTime<Utc>>,
}

impl From<PgTenantRow> for Tenant {
    fn from(row: PgTenantRow) -> Self {
        let plan = match row.plan.as_str() {
            "starter" => TenantPlan::Starter,
            "pro" => TenantPlan::Pro,
            "enterprise" => TenantPlan::Enterprise,
            _ => TenantPlan::Free,
        };

        Tenant {
            id: row.id,
            name: row.name,
            slug: row.slug,
            domain: row.domain,
            owner_id: row.owner_id,
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
            settings: row.settings,
            features: row.features,
            status: match row.status.as_str() {
                "suspended" => TenantStatus::Suspended,
                "deleted" => TenantStatus::Deleted,
                _ => TenantStatus::Active,
            },
            created_at: row.created_at,
            updated_at: row.updated_at,
            deleted_at: row.deleted_at,
        }
    }
}
