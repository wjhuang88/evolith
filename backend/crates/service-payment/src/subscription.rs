use crate::{PaymentConfig, PaymentError, Result};
use chrono::{DateTime, Utc};
use reqwest::{Method, RequestBuilder, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::{debug, error, info};
use uuid::Uuid;

const STRIPE_API_BASE: &str = "https://api.stripe.com/v1";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionData {
    pub stripe_subscription_id: String,
    pub stripe_customer_id: String,
    pub status: String,
    pub plan_id: String,
    pub billing_cycle: String,
    pub current_period_start: DateTime<Utc>,
    pub current_period_end: DateTime<Utc>,
    pub cancel_at_period_end: bool,
    pub trial_end: Option<DateTime<Utc>>,
}

pub struct StripeSubscriptionService {
    client: reqwest::Client,
    config: PaymentConfig,
}

impl StripeSubscriptionService {
    pub fn new(config: PaymentConfig) -> Self {
        let client = reqwest::Client::new();
        Self { client, config }
    }

    pub async fn create_subscription(
        &self,
        customer_id: &str,
        price_id: &str,
        tenant_id: Uuid,
        trial_days: Option<u32>,
    ) -> Result<SubscriptionData> {
        info!("Creating Stripe subscription for customer: {}", customer_id);

        let mut form = vec![
            ("customer", customer_id.to_string()),
            ("items[0][price]", price_id.to_string()),
            ("items[0][quantity]", "1".to_string()),
            ("metadata[tenant_id]", tenant_id.to_string()),
        ];
        if let Some(trial_days) = trial_days {
            form.push(("trial_period_days", trial_days.to_string()));
        }

        let subscription = self
            .stripe_json(
                self.stripe_request(Method::POST, "subscriptions")
                    .form(&form),
            )
            .await
            .map_err(|e| {
                error!("Failed to create Stripe subscription: {:?}", e);
                e
            })?;

        Ok(self.subscription_to_data(&subscription))
    }

    pub async fn get_subscription(
        &self,
        subscription_id: &str,
    ) -> Result<Option<SubscriptionData>> {
        debug!("Fetching Stripe subscription: {}", subscription_id);

        let response = self
            .stripe_request(Method::GET, &format!("subscriptions/{}", subscription_id))
            .send()
            .await
            .map_err(|e| PaymentError::StripeApi(e.to_string()))?;

        if response.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }

        Ok(Some(self.subscription_to_data(
            &self.parse_stripe_response(response).await?,
        )))
    }

    pub async fn cancel_subscription(
        &self,
        subscription_id: &str,
        immediately: bool,
    ) -> Result<SubscriptionData> {
        info!(
            "Cancelling Stripe subscription: {} (immediately: {})",
            subscription_id, immediately
        );

        let subscription = if immediately {
            self.stripe_json(self.stripe_request(
                Method::DELETE,
                &format!("subscriptions/{}", subscription_id),
            ))
            .await
            .map_err(|e| {
                error!("Failed to cancel Stripe subscription: {:?}", e);
                e
            })?
        } else {
            let form = [("cancel_at_period_end", "true")];
            self.stripe_json(
                self.stripe_request(Method::POST, &format!("subscriptions/{}", subscription_id))
                    .form(&form),
            )
            .await
            .map_err(|e| {
                error!("Failed to schedule subscription cancellation: {:?}", e);
                e
            })?
        };

        Ok(self.subscription_to_data(&subscription))
    }

    fn subscription_to_data(&self, subscription: &Value) -> SubscriptionData {
        let status = subscription
            .get("status")
            .and_then(Value::as_str)
            .unwrap_or("unknown");

        let price_id = subscription
            .pointer("/items/data")
            .and_then(Value::as_array)
            .and_then(|items| items.first())
            .and_then(|item| item.get("price"))
            .and_then(|price| {
                price
                    .as_str()
                    .or_else(|| price.get("id").and_then(Value::as_str))
            })
            .unwrap_or_default();

        let customer_id = subscription
            .get("customer")
            .and_then(|customer| {
                customer
                    .as_str()
                    .or_else(|| customer.get("id").and_then(Value::as_str))
            })
            .unwrap_or_default();

        let period_start = subscription
            .get("current_period_start")
            .and_then(Value::as_i64)
            .unwrap_or_else(|| Utc::now().timestamp());
        let period_end = subscription
            .get("current_period_end")
            .and_then(Value::as_i64)
            .unwrap_or_else(|| Utc::now().timestamp());
        let trial_end = subscription
            .get("trial_end")
            .and_then(Value::as_i64)
            .and_then(|ts| DateTime::from_timestamp(ts, 0));

        SubscriptionData {
            stripe_subscription_id: subscription
                .get("id")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string(),
            stripe_customer_id: customer_id.to_string(),
            status: status.to_string(),
            plan_id: price_id.to_string(),
            billing_cycle: "monthly".to_string(),
            current_period_start: DateTime::from_timestamp(period_start, 0)
                .unwrap_or_else(Utc::now),
            current_period_end: DateTime::from_timestamp(period_end, 0).unwrap_or_else(Utc::now),
            cancel_at_period_end: subscription
                .get("cancel_at_period_end")
                .and_then(Value::as_bool)
                .unwrap_or(false),
            trial_end,
        }
    }

    fn stripe_request(&self, method: Method, path: &str) -> RequestBuilder {
        let request = self
            .client
            .request(method, format!("{}/{}", STRIPE_API_BASE, path))
            .bearer_auth(&self.config.stripe_secret_key);

        if let Some(version) = &self.config.stripe_api_version {
            request.header("Stripe-Version", version)
        } else {
            request
        }
    }

    async fn stripe_json(&self, request: RequestBuilder) -> Result<Value> {
        let response = request
            .send()
            .await
            .map_err(|e| PaymentError::StripeApi(e.to_string()))?;
        self.parse_stripe_response(response).await
    }

    async fn parse_stripe_response(&self, response: reqwest::Response) -> Result<Value> {
        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| PaymentError::StripeApi(e.to_string()))?;

        if !status.is_success() {
            return Err(PaymentError::StripeApi(body));
        }

        serde_json::from_str(&body).map_err(|e| PaymentError::StripeApi(e.to_string()))
    }
}
