//! Billing handlers
//! Handles plans, subscriptions, invoices, and usage

use std::sync::{Arc, Mutex};

use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use uuid::Uuid;

use crate::dto::billing_dto::*;
use crate::dto::common::ApiResponse;

/// In-memory store for plans and subscriptions (for development)
pub struct BillingStore {
    pub plans: Mutex<Vec<PlanInfo>>,
    pub subscriptions: Mutex<Vec<SubscriptionInfo>>,
    pub invoices: Mutex<Vec<InvoiceInfo>>,
    pub usage: Mutex<Vec<ResourceUsage>>,
}

impl Default for BillingStore {
    fn default() -> Self {
        let plans = vec![
            PlanInfo {
                id: "plan-free".to_string(),
                name: "free".to_string(),
                display_name: "免费版".to_string(),
                description: Some("适合个人开发者的免费套餐".to_string()),
                monthly_price: 0.0,
                yearly_price: Some(0.0),
                max_users: 3,
                max_tools: 5,
                max_skills: 10,
                max_snippets: 50,
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
                max_tools: 20,
                max_skills: 50,
                max_snippets: 200,
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
                max_tools: 100,
                max_skills: 200,
                max_snippets: 1000,
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
                monthly_price: 0.0, // Custom pricing
                yearly_price: None,
                max_users: -1, // Unlimited
                max_tools: -1,
                max_skills: -1,
                max_snippets: -1,
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
        ];

        Self {
            plans: Mutex::new(plans),
            subscriptions: Mutex::new(Vec::new()),
            invoices: Mutex::new(Vec::new()),
            usage: Mutex::new(Vec::new()),
        }
    }
}

/// Billing state
#[derive(Clone)]
pub struct BillingState {
    pub store: Arc<BillingStore>,
}

impl BillingState {
    pub fn new() -> Self {
        Self {
            store: Arc::new(BillingStore::default()),
        }
    }
}

impl Default for BillingState {
    fn default() -> Self {
        Self::new()
    }
}

/// Get all available plans
pub async fn get_plans(state: web::Data<BillingState>) -> impl Responder {
    let plans = state.store.plans.lock().unwrap().clone();
    HttpResponse::Ok().json(ApiResponse::<PlanListResponse>::success(PlanListResponse {
        plans,
    }))
}

/// Get current subscription
pub async fn get_subscription(
    _tenant_id: web::Path<String>,
    state: web::Data<BillingState>,
) -> impl Responder {
    let subscriptions = state.store.subscriptions.lock().unwrap();

    // For demo, return default subscription if none exists
    let subscription = subscriptions.first().cloned().map(|s| {
        let plans = state.store.plans.lock().unwrap();
        let plan = plans.iter().find(|p| p.id == "plan-pro").cloned().unwrap();
        SubscriptionInfo {
            id: s.id,
            plan,
            status: s.status,
            billing_cycle: s.billing_cycle,
            current_period_start: s.current_period_start,
            current_period_end: s.current_period_end,
            trial_end_at: None,
            cancel_at_period_end: false,
        }
    });

    HttpResponse::Ok().json(ApiResponse::<GetSubscriptionResponse>::success(
        GetSubscriptionResponse { subscription },
    ))
}

/// Create a new subscription
pub async fn create_subscription(
    _tenant_id: web::Path<String>,
    body: web::Json<CreateSubscriptionRequest>,
    state: web::Data<BillingState>,
) -> impl Responder {
    let plan_id = body.plan_id.clone();
    let billing_cycle = body
        .billing_cycle
        .clone()
        .unwrap_or_else(|| "monthly".to_string());

    // Find plan
    let plans = state.store.plans.lock().unwrap();
    let plan = match plans.iter().find(|p| p.id == plan_id) {
        Some(p) => p.clone(),
        None => {
            return HttpResponse::BadRequest()
                .json(ApiResponse::<()>::error("PLAN_NOT_FOUND", "Plan not found"));
        }
    };
    drop(plans);

    let now = Utc::now();
    let period_days = if billing_cycle == "yearly" { 365 } else { 30 };
    let period_end = now + chrono::Duration::days(period_days);

    let subscription = SubscriptionInfo {
        id: Uuid::new_v4().to_string(),
        plan,
        status: "active".to_string(),
        billing_cycle,
        current_period_start: now.to_rfc3339(),
        current_period_end: period_end.to_rfc3339(),
        trial_end_at: None,
        cancel_at_period_end: false,
    };

    // Store subscription
    {
        let mut subscriptions = state.store.subscriptions.lock().unwrap();
        subscriptions.push(subscription.clone());
    }

    HttpResponse::Ok().json(ApiResponse::<SubscriptionInfo>::success(subscription))
}

/// Update subscription (change plan)
pub async fn update_subscription(
    _tenant_id: web::Path<String>,
    body: web::Json<UpdateSubscriptionRequest>,
    state: web::Data<BillingState>,
) -> impl Responder {
    // In production:
    // 1. Validate new plan
    // 2. Calculate proration
    // 3. Update subscription in database

    HttpResponse::Ok().json(ApiResponse::<serde_json::Value>::success(
        serde_json::json!({
            "message": "Subscription updated successfully"
        }),
    ))
}

/// Cancel subscription
pub async fn cancel_subscription(
    _tenant_id: web::Path<String>,
    body: web::Json<CancelSubscriptionRequest>,
    state: web::Data<BillingState>,
) -> impl Responder {
    let reason = body.reason.clone();
    let feedback = body.feedback.clone();

    // In production:
    // 1. Check permissions
    // 2. Mark subscription to cancel at period end

    println!(
        "[DEMO] Subscription cancelled. Reason: {:?}, Feedback: {:?}",
        reason, feedback
    );

    HttpResponse::Ok().json(ApiResponse::<serde_json::Value>::success(
        serde_json::json!({
            "message": "Subscription will be cancelled at the end of billing period",
            "status": "canceled"
        }),
    ))
}

/// Get invoices
pub async fn get_invoices(
    _tenant_id: web::Path<String>,
    state: web::Data<BillingState>,
) -> impl Responder {
    let invoices = state.store.invoices.lock().unwrap().clone();
    HttpResponse::Ok().json(ApiResponse::<InvoiceListResponse>::success(
        InvoiceListResponse {
            total: invoices.len(),
            invoices,
        },
    ))
}

/// Get usage statistics
pub async fn get_usage(
    _tenant_id: web::Path<String>,
    state: web::Data<BillingState>,
) -> impl Responder {
    let now = Utc::now();
    let period_start = now - chrono::Duration::days(30);
    let period_end = now + chrono::Duration::days(15); // Days remaining in period

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
