use crate::{PaymentConfig, PaymentError, Result};
use reqwest::{Method, RequestBuilder, StatusCode};
use serde_json::Value;
use tracing::{debug, error, info};
use uuid::Uuid;

const STRIPE_API_BASE: &str = "https://api.stripe.com/v1";

pub struct StripeCustomerService {
    client: reqwest::Client,
    config: PaymentConfig,
}

impl StripeCustomerService {
    pub fn new(config: PaymentConfig) -> Self {
        let client = reqwest::Client::new();
        Self { client, config }
    }

    pub async fn create_customer(
        &self,
        email: &str,
        name: Option<&str>,
        tenant_id: Uuid,
    ) -> Result<String> {
        info!("Creating Stripe customer for tenant: {}", tenant_id);

        let mut form = vec![
            ("email", email.to_string()),
            ("metadata[tenant_id]", tenant_id.to_string()),
        ];
        if let Some(name) = name {
            form.push(("name", name.to_string()));
        }

        let customer = self
            .stripe_json(self.stripe_request(Method::POST, "customers").form(&form))
            .await?;

        let customer_id = customer
            .get("id")
            .and_then(Value::as_str)
            .ok_or_else(|| PaymentError::StripeApi("Stripe customer id missing".to_string()))?;

        debug!("Created Stripe customer: {}", customer_id);
        Ok(customer_id.to_string())
    }

    pub async fn get_customer(&self, customer_id: &str) -> Result<Option<Value>> {
        debug!("Fetching Stripe customer: {}", customer_id);

        let response = self
            .stripe_request(Method::GET, &format!("customers/{}", customer_id))
            .send()
            .await
            .map_err(|e| PaymentError::StripeApi(e.to_string()))?;

        if response.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }

        Ok(Some(self.parse_stripe_response(response).await?))
    }

    pub async fn update_customer(
        &self,
        customer_id: &str,
        email: Option<&str>,
        name: Option<&str>,
    ) -> Result<()> {
        debug!("Updating Stripe customer: {}", customer_id);

        let mut form = Vec::new();
        if let Some(email) = email {
            form.push(("email", email.to_string()));
        }
        if let Some(name) = name {
            form.push(("name", name.to_string()));
        }

        self.stripe_json(
            self.stripe_request(Method::POST, &format!("customers/{}", customer_id))
                .form(&form),
        )
        .await
        .map_err(|e| {
            error!("Failed to update Stripe customer: {:?}", e);
            e
        })?;

        Ok(())
    }

    pub async fn delete_customer(&self, customer_id: &str) -> Result<()> {
        info!("Deleting Stripe customer: {}", customer_id);

        self.stripe_json(
            self.stripe_request(Method::DELETE, &format!("customers/{}", customer_id)),
        )
        .await
        .map_err(|e| {
            error!("Failed to delete Stripe customer: {:?}", e);
            e
        })?;

        Ok(())
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
