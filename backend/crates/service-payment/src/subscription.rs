use crate::{PaymentConfig, PaymentError, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use stripe::{
    CancelSubscription, Client, CreateSubscription, CreateSubscriptionItems, CustomerId,
    Subscription, SubscriptionId, SubscriptionStatus, UpdateSubscription,
};
use tracing::{debug, error, info};
use uuid::Uuid;

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
    client: Client,
    _config: PaymentConfig,
}

impl StripeSubscriptionService {
    pub fn new(config: PaymentConfig) -> Self {
        let client = Client::new(config.stripe_secret_key.as_str());
        Self {
            client,
            _config: config,
        }
    }

    pub async fn create_subscription(
        &self,
        customer_id: &str,
        price_id: &str,
        tenant_id: Uuid,
        trial_days: Option<u32>,
    ) -> Result<SubscriptionData> {
        info!("Creating Stripe subscription for customer: {}", customer_id);

        let cid = CustomerId::from_str(customer_id)
            .map_err(|e| PaymentError::CustomerNotFound(e.to_string()))?;

        let mut metadata = std::collections::HashMap::new();
        metadata.insert("tenant_id".to_string(), tenant_id.to_string());

        let mut create_params = CreateSubscription::new(cid);
        create_params.items = Some(vec![CreateSubscriptionItems {
            price: Some(price_id.to_string()),
            quantity: Some(1),
            ..Default::default()
        }]);
        create_params.metadata = Some(metadata);
        create_params.trial_period_days = trial_days;

        let subscription = Subscription::create(&self.client, create_params)
            .await
            .map_err(|e: stripe::StripeError| {
                error!("Failed to create Stripe subscription: {:?}", e);
                PaymentError::StripeApi(e.to_string())
            })?;

        Ok(self.subscription_to_data(&subscription))
    }

    pub async fn get_subscription(
        &self,
        subscription_id: &str,
    ) -> Result<Option<SubscriptionData>> {
        debug!("Fetching Stripe subscription: {}", subscription_id);

        let sub_id = SubscriptionId::from_str(subscription_id)
            .map_err(|e| PaymentError::SubscriptionNotFound(e.to_string()))?;

        match Subscription::retrieve(&self.client, &sub_id, &[]).await {
            Ok(subscription) => Ok(Some(self.subscription_to_data(&subscription))),
            Err(e) => {
                let error_str = e.to_string().to_lowercase();
                if error_str.contains("not found") || error_str.contains("404") {
                    Ok(None)
                } else {
                    error!("Failed to retrieve Stripe subscription: {:?}", e);
                    Err(PaymentError::StripeApi(e.to_string()))
                }
            }
        }
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

        let sub_id = SubscriptionId::from_str(subscription_id)
            .map_err(|e| PaymentError::SubscriptionNotFound(e.to_string()))?;

        let subscription = if immediately {
            Subscription::cancel(&self.client, &sub_id, CancelSubscription::new())
                .await
                .map_err(|e: stripe::StripeError| {
                    error!("Failed to cancel Stripe subscription: {:?}", e);
                    PaymentError::StripeApi(e.to_string())
                })?
        } else {
            let mut update_params = UpdateSubscription::new();
            update_params.cancel_at_period_end = Some(true);
            Subscription::update(&self.client, &sub_id, update_params)
                .await
                .map_err(|e: stripe::StripeError| {
                    error!("Failed to schedule subscription cancellation: {:?}", e);
                    PaymentError::StripeApi(e.to_string())
                })?
        };

        Ok(self.subscription_to_data(&subscription))
    }

    fn subscription_to_data(&self, subscription: &Subscription) -> SubscriptionData {
        let status = match subscription.status {
            SubscriptionStatus::Active => "active",
            SubscriptionStatus::PastDue => "past_due",
            SubscriptionStatus::Canceled => "canceled",
            SubscriptionStatus::Unpaid => "unpaid",
            SubscriptionStatus::Trialing => "trialing",
            SubscriptionStatus::Incomplete => "incomplete",
            SubscriptionStatus::IncompleteExpired => "incomplete_expired",
            SubscriptionStatus::Paused => "paused",
        };

        let price_id = subscription
            .items
            .data
            .first()
            .and_then(|item| item.price.as_ref())
            .map(|p| p.id.to_string())
            .unwrap_or_default();

        let customer_id = match &subscription.customer {
            stripe::Expandable::Id(id) => id.to_string(),
            stripe::Expandable::Object(customer) => customer.id.to_string(),
        };

        SubscriptionData {
            stripe_subscription_id: subscription.id.to_string(),
            stripe_customer_id: customer_id,
            status: status.to_string(),
            plan_id: price_id,
            billing_cycle: "monthly".to_string(),
            current_period_start: DateTime::from_timestamp(subscription.current_period_start, 0)
                .unwrap_or_else(Utc::now),
            current_period_end: DateTime::from_timestamp(subscription.current_period_end, 0)
                .unwrap_or_else(Utc::now),
            cancel_at_period_end: subscription.cancel_at_period_end,
            trial_end: subscription
                .trial_end
                .map(|ts| DateTime::from_timestamp(ts, 0).unwrap_or_else(Utc::now)),
        }
    }
}
