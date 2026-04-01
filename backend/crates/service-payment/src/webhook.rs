use crate::{PaymentConfig, PaymentError, Result};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use tracing::{debug, error, info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEvent {
    pub id: String,
    pub event_type: String,
    pub data: WebhookEventData,
    pub created: i64,
    pub livemode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEventData {
    pub object: serde_json::Value,
    pub previous_attributes: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedWebhook {
    pub event_id: String,
    pub event_type: String,
    pub tenant_id: Option<String>,
    pub action_taken: String,
    pub processed_at: chrono::DateTime<chrono::Utc>,
}

pub struct WebhookHandler {
    webhook_secret: String,
}

impl WebhookHandler {
    pub fn new(config: PaymentConfig) -> Self {
        Self {
            webhook_secret: config.stripe_webhook_secret,
        }
    }

    pub fn verify_signature(
        &self,
        payload: &[u8],
        signature: &str,
        timestamp: i64,
    ) -> Result<bool> {
        let current_time = chrono::Utc::now().timestamp();

        if (current_time - timestamp).abs() > 300 {
            warn!(
                "Webhook timestamp too old: {} (current: {})",
                timestamp, current_time
            );
            return Err(PaymentError::WebhookSignatureInvalid);
        }

        let signed_payload = format!("{}.{}", timestamp, String::from_utf8_lossy(payload));

        let mut mac = Hmac::<Sha256>::new_from_slice(self.webhook_secret.as_bytes())
            .map_err(|e| PaymentError::WebhookProcessingFailed(e.to_string()))?;

        mac.update(signed_payload.as_bytes());
        let expected_signature = hex::encode(mac.finalize().into_bytes());

        for part in signature.split(',') {
            if part.starts_with("v1=") {
                let provided_sig = part.strip_prefix("v1=").unwrap_or("");
                if provided_sig == expected_signature {
                    return Ok(true);
                }
            }
        }

        warn!("Webhook signature verification failed");
        Err(PaymentError::WebhookSignatureInvalid)
    }

    pub fn parse_event(&self, payload: &[u8]) -> Result<WebhookEvent> {
        // Parse the raw Stripe event JSON
        let raw_event: stripe::Event = serde_json::from_slice(payload).map_err(|e| {
            error!("Failed to parse Stripe event: {:?}", e);
            PaymentError::WebhookProcessingFailed(e.to_string())
        })?;

        // Convert the EventObject to serde_json::Value
        let object_value = serde_json::to_value(&raw_event.data.object).map_err(|e| {
            error!("Failed to serialize event object: {:?}", e);
            PaymentError::WebhookProcessingFailed(e.to_string())
        })?;

        // Convert previous_attributes from HashMap to Value
        let previous_attributes = raw_event
            .data
            .previous_attributes
            .as_ref()
            .map(|attrs| serde_json::to_value(attrs).unwrap_or(serde_json::Value::Null));

        Ok(WebhookEvent {
            id: raw_event.id.to_string(),
            event_type: raw_event.type_.to_string(),
            data: WebhookEventData {
                object: object_value,
                previous_attributes,
            },
            created: raw_event.created,
            livemode: raw_event.livemode,
        })
    }

    pub async fn process_event(&self, event: WebhookEvent) -> Result<ProcessedWebhook> {
        info!(
            "Processing Stripe webhook event: {} ({})",
            event.id, event.event_type
        );

        let action_taken = match event.event_type.as_str() {
            "customer.created" => self.handle_customer_created(&event).await?,
            "customer.updated" => self.handle_customer_updated(&event).await?,
            "customer.deleted" => self.handle_customer_deleted(&event).await?,
            "subscription.created" => self.handle_subscription_created(&event).await?,
            "subscription.updated" => self.handle_subscription_updated(&event).await?,
            "subscription.deleted" => self.handle_subscription_deleted(&event).await?,
            "invoice.paid" => self.handle_invoice_paid(&event).await?,
            "invoice.payment_failed" => self.handle_invoice_payment_failed(&event).await?,
            "payment_method.attached" => self.handle_payment_method_attached(&event).await?,
            "payment_method.detached" => self.handle_payment_method_detached(&event).await?,
            _ => {
                debug!("Unhandled webhook event type: {}", event.event_type);
                "ignored".to_string()
            }
        };

        let tenant_id = self.extract_tenant_id(&event);

        Ok(ProcessedWebhook {
            event_id: event.id,
            event_type: event.event_type,
            tenant_id,
            action_taken,
            processed_at: chrono::Utc::now(),
        })
    }

    fn extract_tenant_id(&self, event: &WebhookEvent) -> Option<String> {
        if let Some(obj) = event.data.object.as_object() {
            if let Some(metadata) = obj.get("metadata") {
                if let Some(tenant_id) = metadata.get("tenant_id") {
                    return tenant_id.as_str().map(|s| s.to_string());
                }
            }
        }
        None
    }

    async fn handle_customer_created(&self, _event: &WebhookEvent) -> Result<String> {
        debug!("Handling customer.created event");
        Ok("customer_created".to_string())
    }

    async fn handle_customer_updated(&self, _event: &WebhookEvent) -> Result<String> {
        debug!("Handling customer.updated event");
        Ok("customer_updated".to_string())
    }

    async fn handle_customer_deleted(&self, _event: &WebhookEvent) -> Result<String> {
        debug!("Handling customer.deleted event");
        Ok("customer_deleted".to_string())
    }

    async fn handle_subscription_created(&self, _event: &WebhookEvent) -> Result<String> {
        debug!("Handling subscription.created event");
        Ok("subscription_created".to_string())
    }

    async fn handle_subscription_updated(&self, _event: &WebhookEvent) -> Result<String> {
        debug!("Handling subscription.updated event");
        Ok("subscription_updated".to_string())
    }

    async fn handle_subscription_deleted(&self, _event: &WebhookEvent) -> Result<String> {
        debug!("Handling subscription.deleted event");
        Ok("subscription_deleted".to_string())
    }

    async fn handle_invoice_paid(&self, _event: &WebhookEvent) -> Result<String> {
        debug!("Handling invoice.paid event");
        Ok("invoice_paid".to_string())
    }

    async fn handle_invoice_payment_failed(&self, event: &WebhookEvent) -> Result<String> {
        warn!("Invoice payment failed for event: {}", event.id);
        Ok("invoice_payment_failed".to_string())
    }

    async fn handle_payment_method_attached(&self, _event: &WebhookEvent) -> Result<String> {
        debug!("Handling payment_method.attached event");
        Ok("payment_method_attached".to_string())
    }

    async fn handle_payment_method_detached(&self, _event: &WebhookEvent) -> Result<String> {
        debug!("Handling payment_method.detached event");
        Ok("payment_method_detached".to_string())
    }
}
