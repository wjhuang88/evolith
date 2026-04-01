pub mod config;
pub mod customer;
pub mod subscription;
pub mod webhook;
pub mod usage;

pub use config::PaymentConfig;
pub use customer::StripeCustomerService;
pub use subscription::StripeSubscriptionService;
pub use webhook::WebhookHandler;
pub use usage::{
    UsageTracker, UsageRecord, UsageReport, ResourceType, OverageRate,
    OverageInvoice, InvoiceLineItem, InvoiceStatus, InvoiceGenerator,
};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum PaymentError {
    #[error("Stripe API error: {0}")]
    StripeApi(String),

    #[error("Customer not found: {0}")]
    CustomerNotFound(String),

    #[error("Subscription not found: {0}")]
    SubscriptionNotFound(String),

    #[error("Payment method not found: {0}")]
    PaymentMethodNotFound(String),

    #[error("Webhook signature verification failed")]
    WebhookSignatureInvalid,

    #[error("Webhook event processing failed: {0}")]
    WebhookProcessingFailed(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Usage tracking error: {0}")]
    UsageTrackingError(String),

    #[error("Overage calculation error: {0}")]
    OverageCalculationError(String),
}

pub type Result<T> = std::result::Result<T, PaymentError>;