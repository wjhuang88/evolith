//! Billing DTOs
//! Defines request/response structures for plans, subscriptions, and billing

use serde::{Deserialize, Serialize};

/// Plan info response
#[derive(Debug, Clone, Serialize)]
pub struct PlanInfo {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub monthly_price: f64,
    pub yearly_price: Option<f64>,
    pub max_users: i32,
    pub max_tools: i32,
    pub max_skills: i32,
    pub max_snippets: i32,
    pub max_api_calls_per_month: i32,
    pub max_storage_mb: i32,
    pub features: serde_json::Value,
    pub is_builtin: bool,
}

/// Plan list response
#[derive(Debug, Clone, Serialize)]
pub struct PlanListResponse {
    pub plans: Vec<PlanInfo>,
}

/// Subscription info response
#[derive(Debug, Clone, Serialize)]
pub struct SubscriptionInfo {
    pub id: String,
    pub plan: PlanInfo,
    pub status: String,
    pub billing_cycle: String,
    pub current_period_start: String,
    pub current_period_end: String,
    pub trial_end_at: Option<String>,
    pub cancel_at_period_end: bool,
}

/// Get subscription response
#[derive(Debug, Clone, Serialize)]
pub struct GetSubscriptionResponse {
    pub subscription: Option<SubscriptionInfo>,
}

/// Create subscription request
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateSubscriptionRequest {
    pub plan_id: String,
    pub billing_cycle: Option<String>, // "monthly" or "yearly"
    pub payment_method_id: Option<String>,
}

/// Update subscription request
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateSubscriptionRequest {
    pub plan_id: Option<String>,
    pub billing_cycle: Option<String>,
    pub proration: Option<String>, // "immediate" or "next_billing_cycle"
}

/// Cancel subscription request
#[derive(Debug, Deserialize, Serialize)]
pub struct CancelSubscriptionRequest {
    pub reason: Option<String>,
    pub feedback: Option<String>,
}

/// Invoice info
#[derive(Debug, Clone, Serialize)]
pub struct InvoiceInfo {
    pub id: String,
    pub invoice_number: String,
    pub status: String,
    pub amount: f64,
    pub currency: String,
    pub period_start: String,
    pub period_end: String,
    pub pdf_url: Option<String>,
    pub created_at: String,
    pub paid_at: Option<String>,
}

/// Invoice list response
#[derive(Debug, Clone, Serialize)]
pub struct InvoiceListResponse {
    pub invoices: Vec<InvoiceInfo>,
    pub total: usize,
}

/// Usage info for a resource type
#[derive(Debug, Clone, Serialize)]
pub struct ResourceUsage {
    pub resource_type: String,
    pub used: i64,
    pub limit: i64,
    pub percent: f64,
    pub remaining: Option<i64>,
    pub overage: Option<f64>,
}

/// Usage period info
#[derive(Debug, Clone, Serialize)]
pub struct UsagePeriod {
    pub start: String,
    pub end: String,
    pub remaining_days: i32,
}

/// Get usage response
#[derive(Debug, Clone, Serialize)]
pub struct UsageResponse {
    pub period: UsagePeriod,
    pub resources: Vec<ResourceUsage>,
}
