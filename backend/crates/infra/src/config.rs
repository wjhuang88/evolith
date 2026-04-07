//! Configuration management

use config::{Config, ConfigError, Environment};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub jwt: JwtConfig,
    pub storage: StorageConfig,
    pub sandbox: SandboxConfig,
    pub log: LogConfig,
    pub stripe: StripeConfig,
    pub smtp: SmtpConfig,
    pub rate_limit: RateLimitConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    /// Database type: sqlite, postgres, mysql
    pub database_type: String,
    /// Database URL
    pub url: String,
    /// Maximum connections in pool
    pub max_connections: u32,
    /// Enable seed data (development only)
    pub seed_database: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RedisConfig {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StorageConfig {
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
    pub use_ssl: bool,
    pub bucket: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SandboxConfig {
    pub enabled: bool,
    pub timeout_seconds: u32,
    pub memory_mb: u32,
    /// CPU shares for Docker container (relative weight, default 512)
    pub cpu_shares: i64,
    /// Maximum number of PIDs in container (default 256)
    pub pids_limit: i64,
    /// Whether network access is enabled (default false for security)
    pub network_enabled: bool,
    /// Maximum output bytes to capture from stdout/stderr (default 10MB)
    pub max_output_bytes: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LogConfig {
    pub level: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StripeConfig {
    pub secret_key: String,
    pub webhook_secret: String,
    pub api_version: Option<String>,
    pub test_mode: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub from_address: String,
    pub from_name: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RateLimitConfig {
    pub unauthenticated_rpm: u32,
    pub authenticated_rpm: u32,
    pub api_key_rpm: u32,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let config = Config::builder()
            // Set defaults
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 8080)?
            .set_default("database.database_type", "sqlite")?
            .set_default("database.url", ":memory:")?
            .set_default("database.max_connections", 10)?
            .set_default("database.seed_database", false)?
            .set_default("redis.url", "redis://localhost:6379")?
            .set_default("jwt.secret", "dev_secret_key_change_in_production")?
            .set_default("jwt.expiration", "24h")?
            .set_default("storage.endpoint", "localhost:9000")?
            .set_default("storage.access_key", "minioadmin")?
            .set_default("storage.secret_key", "minioadmin")?
            .set_default("storage.use_ssl", false)?
            .set_default("storage.bucket", "evolith")?
            .set_default("sandbox.enabled", true)?
            .set_default("sandbox.timeout_seconds", 30)?
            .set_default("sandbox.memory_mb", 256)?
            .set_default("sandbox.cpu_shares", 512)?
            .set_default("sandbox.pids_limit", 256)?
            .set_default("sandbox.network_enabled", false)?
            .set_default("sandbox.max_output_bytes", 10485760i64)?
            .set_default("log.level", "info")?
            .set_default("stripe.secret_key", "sk_test_placeholder")?
            .set_default("stripe.webhook_secret", "whsec_placeholder")?
            .set_default("stripe.api_version", None::<String>)?
            .set_default("stripe.test_mode", true)?
            .set_default("smtp.host", "localhost")?
            .set_default("smtp.port", 1025)?
            .set_default("smtp.username", "")?
            .set_default("smtp.password", "")?
            .set_default("smtp.from_address", "noreply@evolith.io")?
            .set_default("smtp.from_name", "Evolith")?
            .set_default("smtp.enabled", false)?
            .set_default("rate_limit.unauthenticated_rpm", 30)?
            .set_default("rate_limit.authenticated_rpm", 300)?
            .set_default("rate_limit.api_key_rpm", 1000)?
            // Add environment variables
            .add_source(Environment::default().separator("__"))
            .build()?;

        let app_config: AppConfig = config.try_deserialize()?;
        app_config.validate()?;
        Ok(app_config)
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), ConfigError> {
        // Validate server config
        if self.server.port == 0 {
            return Err(ConfigError::Message("Server port cannot be 0".to_string()));
        }

        // Validate database config
        if !matches!(
            self.database.database_type.to_lowercase().as_str(),
            "sqlite" | "postgres" | "postgresql" | "mysql"
        ) {
            return Err(ConfigError::Message(format!(
                "Unsupported database type: {}",
                self.database.database_type
            )));
        }

        // Validate JWT secret in production
        if std::env::var("ENVIRONMENT").unwrap_or_default() == "production" {
            if self.jwt.secret == "dev_secret_key_change_in_production" {
                return Err(ConfigError::Message(
                    "JWT secret must be changed in production!".to_string(),
                ));
            }
            if self.jwt.secret.len() < 32 {
                return Err(ConfigError::Message(
                    "JWT secret must be at least 32 characters in production".to_string(),
                ));
            }
        }

        // Validate sandbox config
        if self.sandbox.enabled {
            if self.sandbox.timeout_seconds == 0 {
                return Err(ConfigError::Message(
                    "Sandbox timeout must be greater than 0".to_string(),
                ));
            }
            if self.sandbox.memory_mb < 64 {
                return Err(ConfigError::Message(
                    "Sandbox memory must be at least 64MB".to_string(),
                ));
            }
            if self.sandbox.cpu_shares < 2 {
                return Err(ConfigError::Message(
                    "Sandbox cpu_shares must be at least 2".to_string(),
                ));
            }
            if self.sandbox.pids_limit < 16 {
                return Err(ConfigError::Message(
                    "Sandbox pids_limit must be at least 16".to_string(),
                ));
            }
            if self.sandbox.max_output_bytes < 1024 {
                return Err(ConfigError::Message(
                    "Sandbox max_output_bytes must be at least 1024".to_string(),
                ));
            }
        }

        // Validate Stripe config in production
        if std::env::var("ENVIRONMENT").unwrap_or_default() == "production" {
            if self.stripe.secret_key.starts_with("sk_test_") {
                return Err(ConfigError::Message(
                    "Stripe test key cannot be used in production!".to_string(),
                ));
            }
            if self.stripe.webhook_secret == "whsec_placeholder" {
                return Err(ConfigError::Message(
                    "Stripe webhook secret must be set in production!".to_string(),
                ));
            }
        }

        Ok(())
    }

    /// Check if running in development mode
    pub fn is_development(&self) -> bool {
        std::env::var("ENVIRONMENT").unwrap_or_default() != "production"
    }

    /// Check if running in production mode
    pub fn is_production(&self) -> bool {
        !self.is_development()
    }
}
