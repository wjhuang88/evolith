use crate::{PaymentConfig, PaymentError, Result};
use std::str::FromStr;
use stripe::{Client, CreateCustomer, Customer, CustomerId, UpdateCustomer};
use tracing::{debug, error, info};
use uuid::Uuid;

pub struct StripeCustomerService {
    client: Client,
    _config: PaymentConfig,
}

impl StripeCustomerService {
    pub fn new(config: PaymentConfig) -> Self {
        let client = Client::new(config.stripe_secret_key.as_str());
        Self {
            client,
            _config: config,
        }
    }

    pub async fn create_customer(
        &self,
        email: &str,
        name: Option<&str>,
        tenant_id: Uuid,
    ) -> Result<String> {
        info!("Creating Stripe customer for tenant: {}", tenant_id);

        let mut metadata = std::collections::HashMap::new();
        metadata.insert("tenant_id".to_string(), tenant_id.to_string());

        let mut create_params = CreateCustomer::new();
        create_params.email = Some(email);
        create_params.name = name;
        create_params.metadata = Some(metadata);

        let customer = Customer::create(&self.client, create_params)
            .await
            .map_err(|e: stripe::StripeError| {
                error!("Failed to create Stripe customer: {:?}", e);
                PaymentError::StripeApi(e.to_string())
            })?;

        debug!("Created Stripe customer: {}", customer.id);
        Ok(customer.id.to_string())
    }

    pub async fn get_customer(&self, customer_id: &str) -> Result<Option<Customer>> {
        debug!("Fetching Stripe customer: {}", customer_id);

        let cid = CustomerId::from_str(customer_id)
            .map_err(|e| PaymentError::CustomerNotFound(e.to_string()))?;

        match Customer::retrieve(&self.client, &cid, &[]).await {
            Ok(customer) => Ok(Some(customer)),
            Err(e) => {
                let error_str = e.to_string().to_lowercase();
                if error_str.contains("not found") || error_str.contains("404") {
                    Ok(None)
                } else {
                    error!("Failed to retrieve Stripe customer: {:?}", e);
                    Err(PaymentError::StripeApi(e.to_string()))
                }
            }
        }
    }

    pub async fn update_customer(
        &self,
        customer_id: &str,
        email: Option<&str>,
        name: Option<&str>,
    ) -> Result<()> {
        debug!("Updating Stripe customer: {}", customer_id);

        let cid = CustomerId::from_str(customer_id)
            .map_err(|e| PaymentError::CustomerNotFound(e.to_string()))?;

        let mut update_params = UpdateCustomer::new();
        update_params.email = email;
        update_params.name = name;

        Customer::update(&self.client, &cid, update_params)
            .await
            .map_err(|e: stripe::StripeError| {
                error!("Failed to update Stripe customer: {:?}", e);
                PaymentError::StripeApi(e.to_string())
            })?;

        Ok(())
    }

    pub async fn delete_customer(&self, customer_id: &str) -> Result<()> {
        info!("Deleting Stripe customer: {}", customer_id);

        let cid = CustomerId::from_str(customer_id)
            .map_err(|e| PaymentError::CustomerNotFound(e.to_string()))?;

        Customer::delete(&self.client, &cid)
            .await
            .map_err(|e: stripe::StripeError| {
                error!("Failed to delete Stripe customer: {:?}", e);
                PaymentError::StripeApi(e.to_string())
            })?;

        Ok(())
    }
}
