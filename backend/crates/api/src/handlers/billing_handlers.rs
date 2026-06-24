//! Billing handlers
//! Handles plans, subscriptions, and usage endpoints
//!
//! NOTE: All endpoints are hardcoded stubs for now. Real billing integration
//! will be implemented in a future phase.

use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;

use crate::dto::billing_dto::*;
use crate::dto::common::ApiResponse;
use crate::state::AppState;

/// Hardcoded plan data
fn plan_catalog() -> Vec<PlanInfo> {
    vec![
        PlanInfo {
            id: "plan-free".to_string(),
            name: "free".to_string(),
            display_name: "免费版".to_string(),
            description: Some("适合个人开发者的免费套餐".to_string()),
            monthly_price: 0.0,
            yearly_price: Some(0.0),
            max_users: 3,
            max_repos: 3,
            max_api_calls_per_month: 1000,
            max_storage_mb: 100,
            features: serde_json::json!({
                "custom_domain": false,
                "sso": false,
                "priority_support": false,
                "audit_logs": false
            }),
            is_builtin: true,
        },
        PlanInfo {
            id: "plan-starter".to_string(),
            name: "starter".to_string(),
            display_name: "基础版".to_string(),
            description: Some("适合小团队的入门套餐".to_string()),
            monthly_price: 29.0,
            yearly_price: Some(290.0),
            max_users: 10,
            max_repos: 20,
            max_api_calls_per_month: 10000,
            max_storage_mb: 1024,
            features: serde_json::json!({
                "custom_domain": false,
                "sso": false,
                "priority_support": false,
                "audit_logs": true
            }),
            is_builtin: true,
        },
        PlanInfo {
            id: "plan-pro".to_string(),
            name: "pro".to_string(),
            display_name: "专业版".to_string(),
            description: Some("适合成长中团队的专业套餐".to_string()),
            monthly_price: 99.0,
            yearly_price: Some(990.0),
            max_users: 50,
            max_repos: 100,
            max_api_calls_per_month: 100000,
            max_storage_mb: 10240,
            features: serde_json::json!({
                "custom_domain": true,
                "sso": false,
                "priority_support": true,
                "audit_logs": true
            }),
            is_builtin: true,
        },
        PlanInfo {
            id: "plan-enterprise".to_string(),
            name: "enterprise".to_string(),
            display_name: "企业版".to_string(),
            description: Some("适合大型企业的定制套餐".to_string()),
            monthly_price: 0.0,
            yearly_price: None,
            max_users: -1,
            max_repos: -1,
            max_api_calls_per_month: -1,
            max_storage_mb: -1,
            features: serde_json::json!({
                "custom_domain": true,
                "sso": true,
                "priority_support": true,
                "audit_logs": true,
                "dedicated_support": true,
                "sla": "99.99%"
            }),
            is_builtin: true,
        },
    ]
}

/// Get available plans
pub async fn get_plans(_state: web::Data<AppState>) -> impl Responder {
    let plans = plan_catalog();
    HttpResponse::Ok().json(ApiResponse::<PlanListResponse>::success(PlanListResponse {
        plans,
    }))
}

/// Get current subscription for a tenant
pub async fn get_current_plan(
    _tenant_id: web::Path<String>,
    _state: web::Data<AppState>,
) -> impl Responder {
    // Return stub subscription data
    let plans = plan_catalog();
    let pro_plan = match plans.iter().find(|p| p.id == "plan-pro").cloned() {
        Some(plan) => plan,
        None => {
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "PLAN_NOT_FOUND",
                "Default plan configuration missing",
            ))
        }
    };

    let now = Utc::now();
    let period_end = now + chrono::Duration::days(30);

    let subscription = SubscriptionInfo {
        id: "sub_demo_123".to_string(),
        plan: pro_plan,
        status: "active".to_string(),
        billing_cycle: "monthly".to_string(),
        current_period_start: now.to_rfc3339(),
        current_period_end: period_end.to_rfc3339(),
        trial_end_at: None,
        cancel_at_period_end: false,
    };

    HttpResponse::Ok().json(ApiResponse::<GetSubscriptionResponse>::success(
        GetSubscriptionResponse {
            subscription: Some(subscription),
        },
    ))
}

/// Get usage statistics for a tenant
pub async fn get_usage(
    _tenant_id: web::Path<String>,
    _state: web::Data<AppState>,
) -> impl Responder {
    let now = Utc::now();
    let period_start = now - chrono::Duration::days(15);
    let period_end = now + chrono::Duration::days(15);

    // Hardcoded usage data
    let usage = vec![
        ResourceUsage {
            resource_type: "api_calls".to_string(),
            used: 45000,
            limit: 100000,
            percent: 45.0,
            remaining: Some(55000),
            overage: None,
        },
        ResourceUsage {
            resource_type: "users".to_string(),
            used: 8,
            limit: 50,
            percent: 16.0,
            remaining: Some(42),
            overage: None,
        },
        ResourceUsage {
            resource_type: "tools".to_string(),
            used: 45,
            limit: 100,
            percent: 45.0,
            remaining: Some(55),
            overage: None,
        },
        ResourceUsage {
            resource_type: "skills".to_string(),
            used: 120,
            limit: 200,
            percent: 60.0,
            remaining: Some(80),
            overage: None,
        },
        ResourceUsage {
            resource_type: "storage_mb".to_string(),
            used: 2500,
            limit: 10240,
            percent: 24.4,
            remaining: Some(7740),
            overage: None,
        },
    ];

    let response = UsageResponse {
        period: UsagePeriod {
            start: period_start.to_rfc3339(),
            end: period_end.to_rfc3339(),
            remaining_days: 15,
        },
        resources: usage,
    };

    HttpResponse::Ok().json(ApiResponse::<UsageResponse>::success(response))
}

/// Create a checkout session for subscription
pub async fn create_checkout_session(
    _tenant_id: web::Path<String>,
    body: web::Json<CreateSubscriptionRequest>,
    _state: web::Data<AppState>,
) -> impl Responder {
    let plan_id = body.plan_id.clone();
    let billing_cycle = body
        .billing_cycle
        .clone()
        .unwrap_or_else(|| "monthly".to_string());

    // Find the plan
    let plans = plan_catalog();
    let plan = match plans.iter().find(|p| p.id == plan_id) {
        Some(p) => p.clone(),
        None => {
            return HttpResponse::BadRequest()
                .json(ApiResponse::<()>::error("PLAN_NOT_FOUND", "Plan not found"));
        }
    };

    // Return stub checkout session
    HttpResponse::Ok().json(ApiResponse::<serde_json::Value>::success(
        serde_json::json!({
            "checkout_url": format!("https://billing.example.com/checkout/{}", plan.id),
            "session_id": format!("cs_{}", uuid::Uuid::new_v4()),
            "plan": plan,
            "billing_cycle": billing_cycle
        }),
    ))
}

/// Manage subscription (upgrade, downgrade, cancel)
pub async fn manage_subscription(
    _tenant_id: web::Path<String>,
    _state: web::Data<AppState>,
) -> impl Responder {
    // Return stub management URL
    HttpResponse::Ok().json(ApiResponse::<serde_json::Value>::success(
        serde_json::json!({
            "management_url": "https://billing.example.com/manage",
            "message": "Redirect to billing portal to manage subscription"
        }),
    ))
}
