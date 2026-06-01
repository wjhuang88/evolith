use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct PaymentConfig {
    pub stripe_secret_key: String,
    pub stripe_webhook_secret: String,
    pub stripe_api_version: Option<String>,
    pub test_mode: bool,
}

impl PaymentConfig {
    pub fn from_env() -> Self {
        Self {
            stripe_secret_key: std::env::var("STRIPE__SECRET_KEY")
                .unwrap_or_else(|_| "sk_test_placeholder".to_string()),
            stripe_webhook_secret: std::env::var("STRIPE__WEBHOOK_SECRET")
                .unwrap_or_else(|_| "whsec_placeholder".to_string()),
            stripe_api_version: std::env::var("STRIPE__API_VERSION").ok(),
            test_mode: std::env::var("STRIPE__TEST_MODE")
                .map(|v| v == "true")
                .unwrap_or(true),
        }
    }

    pub fn is_live_mode(&self) -> bool {
        !self.test_mode && self.stripe_secret_key.starts_with("sk_live_")
    }
}

impl Default for PaymentConfig {
    fn default() -> Self {
        Self::from_env()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payment_config_defaults() {
        let config = PaymentConfig::default();
        assert!(config.test_mode);
        assert!(
            config.stripe_secret_key.contains("test")
                || config.stripe_secret_key.contains("placeholder")
        );
    }

    #[test]
    fn test_is_live_mode() {
        let mut config = PaymentConfig {
            test_mode: true,
            ..Default::default()
        };
        config.stripe_secret_key = "sk_test_xxx".to_string();
        assert!(!config.is_live_mode());

        config.test_mode = false;
        config.stripe_secret_key = "sk_live_xxx".to_string();
        assert!(config.is_live_mode());

        config.test_mode = false;
        config.stripe_secret_key = "sk_test_xxx".to_string();
        assert!(!config.is_live_mode());
    }
}
