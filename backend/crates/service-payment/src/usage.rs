use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageRecord {
    pub tenant_id: Uuid,
    pub resource_type: ResourceType,
    pub quantity: u64,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub overage_units: u64,
    pub overage_cost: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ResourceType {
    ApiCalls,
    StorageMb,
    Tools,
    Skills,
    Snippets,
    Users,
}

impl std::fmt::Display for ResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceType::ApiCalls => write!(f, "api_calls"),
            ResourceType::StorageMb => write!(f, "storage_mb"),
            ResourceType::Tools => write!(f, "tools"),
            ResourceType::Skills => write!(f, "skills"),
            ResourceType::Snippets => write!(f, "snippets"),
            ResourceType::Users => write!(f, "users"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverageRate {
    pub resource_type: ResourceType,
    pub unit_price: f64,
    pub billing_unit: u64,
}

impl Default for OverageRate {
    fn default() -> Self {
        Self {
            resource_type: ResourceType::ApiCalls,
            unit_price: 0.001,
            billing_unit: 1000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotaCheck {
    pub resource_type: ResourceType,
    pub limit: u64,
    pub used: u64,
    pub remaining: u64,
    pub allowed: bool,
    pub overage_rate: Option<f64>,
}

pub struct UsageTracker {
    overage_rates: std::collections::HashMap<ResourceType, OverageRate>,
}

impl UsageTracker {
    pub fn new() -> Self {
        let mut rates = std::collections::HashMap::new();

        rates.insert(
            ResourceType::ApiCalls,
            OverageRate {
                resource_type: ResourceType::ApiCalls,
                unit_price: 0.001,
                billing_unit: 1000,
            },
        );

        rates.insert(
            ResourceType::StorageMb,
            OverageRate {
                resource_type: ResourceType::StorageMb,
                unit_price: 0.01,
                billing_unit: 100,
            },
        );

        Self {
            overage_rates: rates,
        }
    }

    pub fn check_quota(&self, resource_type: ResourceType, limit: u64, used: u64) -> QuotaCheck {
        if limit == u64::MAX {
            return QuotaCheck {
                resource_type,
                limit,
                used,
                remaining: u64::MAX,
                allowed: true,
                overage_rate: None,
            };
        }

        let remaining = limit.saturating_sub(used);
        let allowed = remaining > 0;

        let overage_rate = if !allowed && self.overage_rates.contains_key(&resource_type) {
            Some(self.overage_rates[&resource_type].unit_price)
        } else {
            None
        };

        QuotaCheck {
            resource_type,
            limit,
            used,
            remaining,
            allowed: allowed || overage_rate.is_some(),
            overage_rate,
        }
    }

    pub fn calculate_overage(&self, _resource_type: ResourceType, limit: u64, used: u64) -> u64 {
        used.saturating_sub(limit)
    }

    pub fn calculate_overage_cost(&self, resource_type: ResourceType, overage_units: u64) -> f64 {
        if let Some(rate) = self.overage_rates.get(&resource_type) {
            (overage_units as f64 / rate.billing_unit as f64) * rate.unit_price
        } else {
            0.0
        }
    }

    pub fn get_overage_rates(&self) -> &std::collections::HashMap<ResourceType, OverageRate> {
        &self.overage_rates
    }

    pub fn set_overage_rate(&mut self, rate: OverageRate) {
        self.overage_rates.insert(rate.resource_type, rate);
    }
}

impl Default for UsageTracker {
    fn default() -> Self {
        Self::new()
    }
}

pub struct UsageReport {
    pub tenant_id: Uuid,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub resources: Vec<UsageRecord>,
    pub total_overage_cost: f64,
}

impl UsageTracker {
    pub fn generate_usage_report(
        &self,
        tenant_id: Uuid,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
        usage_data: Vec<(ResourceType, u64, u64)>,
    ) -> UsageReport {
        let mut resources = Vec::new();
        let mut total_overage_cost = 0.0;

        for (resource_type, limit, used) in usage_data {
            let overage_units = self.calculate_overage(resource_type, limit, used);
            let overage_cost = self.calculate_overage_cost(resource_type, overage_units);

            resources.push(UsageRecord {
                tenant_id,
                resource_type,
                quantity: used,
                period_start,
                period_end,
                overage_units,
                overage_cost,
            });

            total_overage_cost += overage_cost;
        }

        UsageReport {
            tenant_id,
            period_start,
            period_end,
            resources,
            total_overage_cost,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverageInvoice {
    pub invoice_number: String,
    pub tenant_id: Uuid,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub line_items: Vec<InvoiceLineItem>,
    pub subtotal: f64,
    pub tax: f64,
    pub total: f64,
    pub currency: String,
    pub status: InvoiceStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceLineItem {
    pub description: String,
    pub resource_type: ResourceType,
    pub quantity: u64,
    pub unit_price: f64,
    pub amount: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum InvoiceStatus {
    Draft,
    Issued,
    Paid,
    Void,
}

pub struct InvoiceGenerator {
    tax_rate: f64,
    currency: String,
}

impl InvoiceGenerator {
    pub fn new(tax_rate: f64, currency: String) -> Self {
        Self { tax_rate, currency }
    }

    pub fn generate_overage_invoice(&self, usage_report: &UsageReport) -> Option<OverageInvoice> {
        if usage_report.total_overage_cost <= 0.0 {
            return None;
        }

        let line_items: Vec<InvoiceLineItem> = usage_report
            .resources
            .iter()
            .filter(|r| r.overage_units > 0)
            .map(|r| InvoiceLineItem {
                description: format!("{} Overage", r.resource_type),
                resource_type: r.resource_type,
                quantity: r.overage_units,
                unit_price: r.overage_cost / r.overage_units as f64,
                amount: r.overage_cost,
            })
            .collect();

        let subtotal = usage_report.total_overage_cost;
        let tax = subtotal * self.tax_rate / 100.0;
        let total = subtotal + tax;

        Some(OverageInvoice {
            invoice_number: format!(
                "OVR-{}-{}",
                usage_report
                    .tenant_id
                    .to_string()
                    .split('-')
                    .next()
                    .unwrap_or(""),
                Utc::now().format("%Y%m%d%H%M%S")
            ),
            tenant_id: usage_report.tenant_id,
            period_start: usage_report.period_start,
            period_end: usage_report.period_end,
            line_items,
            subtotal,
            tax,
            total,
            currency: self.currency.clone(),
            status: InvoiceStatus::Draft,
            created_at: Utc::now(),
        })
    }
}

impl Default for InvoiceGenerator {
    fn default() -> Self {
        Self::new(0.0, "CNY".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage_tracker_new() {
        let tracker = UsageTracker::new();
        assert!(tracker.overage_rates.contains_key(&ResourceType::ApiCalls));
        assert!(tracker.overage_rates.contains_key(&ResourceType::StorageMb));
    }

    #[test]
    fn test_check_quota_within_limit() {
        let tracker = UsageTracker::new();
        let check = tracker.check_quota(ResourceType::ApiCalls, 1000, 500);

        assert!(check.allowed);
        assert_eq!(check.remaining, 500);
        assert!(check.overage_rate.is_none());
    }

    #[test]
    fn test_check_quota_at_limit() {
        let tracker = UsageTracker::new();
        // Use a resource type without overage rates so "at limit" truly blocks
        let check = tracker.check_quota(ResourceType::Tools, 10, 10);

        assert!(!check.allowed);
        assert_eq!(check.remaining, 0);
        assert!(check.overage_rate.is_none());
    }

    #[test]
    fn test_check_quota_exceeded_with_overage() {
        let tracker = UsageTracker::new();
        let check = tracker.check_quota(ResourceType::ApiCalls, 1000, 1500);

        assert!(check.allowed);
        assert_eq!(check.remaining, 0);
        assert!(check.overage_rate.is_some());
    }

    #[test]
    fn test_check_quota_unlimited() {
        let tracker = UsageTracker::new();
        let check = tracker.check_quota(ResourceType::ApiCalls, u64::MAX, 1000000);

        assert!(check.allowed);
        assert_eq!(check.remaining, u64::MAX);
        assert!(check.overage_rate.is_none());
    }

    #[test]
    fn test_calculate_overage() {
        let tracker = UsageTracker::new();

        assert_eq!(
            tracker.calculate_overage(ResourceType::ApiCalls, 1000, 500),
            0
        );
        assert_eq!(
            tracker.calculate_overage(ResourceType::ApiCalls, 1000, 1000),
            0
        );
        assert_eq!(
            tracker.calculate_overage(ResourceType::ApiCalls, 1000, 1500),
            500
        );
    }

    #[test]
    fn test_calculate_overage_cost() {
        let tracker = UsageTracker::new();

        let cost = tracker.calculate_overage_cost(ResourceType::ApiCalls, 5000);
        assert!((cost - 0.005).abs() < 0.0001);

        let cost = tracker.calculate_overage_cost(ResourceType::StorageMb, 200);
        assert!((cost - 0.02).abs() < 0.0001);
    }

    #[test]
    fn test_generate_usage_report() {
        let tracker = UsageTracker::new();
        let tenant_id = Uuid::new_v4();
        let now = Utc::now();

        let usage_data = vec![
            (ResourceType::ApiCalls, 1000, 1500),
            (ResourceType::StorageMb, 100, 150),
            (ResourceType::Users, 10, 8),
        ];

        let report = tracker.generate_usage_report(tenant_id, now, now, usage_data);

        assert_eq!(report.tenant_id, tenant_id);
        assert_eq!(report.resources.len(), 3);
        assert!(report.total_overage_cost > 0.0);
    }

    #[test]
    fn test_invoice_generator_no_overage() {
        let generator = InvoiceGenerator::new(10.0, "CNY".to_string());
        let tracker = UsageTracker::new();
        let tenant_id = Uuid::new_v4();
        let now = Utc::now();

        let usage_data = vec![
            (ResourceType::ApiCalls, 1000, 500),
            (ResourceType::StorageMb, 100, 50),
        ];

        let report = tracker.generate_usage_report(tenant_id, now, now, usage_data);
        let invoice = generator.generate_overage_invoice(&report);

        assert!(invoice.is_none());
    }

    #[test]
    fn test_invoice_generator_with_overage() {
        let generator = InvoiceGenerator::new(10.0, "CNY".to_string());
        let tracker = UsageTracker::new();
        let tenant_id = Uuid::new_v4();
        let now = Utc::now();

        let usage_data = vec![
            (ResourceType::ApiCalls, 1000, 2000),
            (ResourceType::StorageMb, 100, 200),
        ];

        let report = tracker.generate_usage_report(tenant_id, now, now, usage_data);
        let invoice = generator.generate_overage_invoice(&report);

        assert!(invoice.is_some());
        let invoice = invoice.unwrap();

        assert_eq!(invoice.tenant_id, tenant_id);
        assert_eq!(invoice.status, InvoiceStatus::Draft);
        assert_eq!(invoice.currency, "CNY");
        assert!(invoice.invoice_number.starts_with("OVR-"));
        assert_eq!(invoice.line_items.len(), 2);

        let expected_tax = invoice.subtotal * 0.1;
        assert!((invoice.tax - expected_tax).abs() < 0.0001);
        assert!((invoice.total - (invoice.subtotal + invoice.tax)).abs() < 0.0001);
    }

    #[test]
    fn test_resource_type_display() {
        assert_eq!(ResourceType::ApiCalls.to_string(), "api_calls");
        assert_eq!(ResourceType::StorageMb.to_string(), "storage_mb");
        assert_eq!(ResourceType::Tools.to_string(), "tools");
    }

    #[test]
    fn test_overage_rate_default() {
        let rate = OverageRate::default();
        assert_eq!(rate.resource_type, ResourceType::ApiCalls);
        assert_eq!(rate.unit_price, 0.001);
        assert_eq!(rate.billing_unit, 1000);
    }
}
