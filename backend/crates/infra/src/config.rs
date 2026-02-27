//! Configuration management

use config::{Config, ConfigError, Environment, File};
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
}

#[derive(Debug, Clone, Deserialize)]
pub struct LogConfig {
    pub level: String,
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
            .set_default("log.level", "info")?
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
