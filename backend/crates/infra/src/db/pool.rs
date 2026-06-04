//! Database pool management

use sqlx::postgres::PgPoolOptions;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Postgres, SqlitePool};

use crate::config::DatabaseConfig;
use common::error::{AppError, Result};

pub enum DatabasePool {
    Sqlite(SqlitePool),
    Postgres(sqlx::Pool<Postgres>),
}

pub async fn create_pool(config: &DatabaseConfig) -> Result<DatabasePool> {
    match config.database_type.to_lowercase().as_str() {
        "sqlite" => {
            let pool = SqlitePoolOptions::new()
                .max_connections(config.max_connections)
                .connect(&config.url)
                .await
                .map_err(|e| {
                    AppError::DatabaseError(format!("Failed to connect to SQLite: {}", e))
                })?;
            Ok(DatabasePool::Sqlite(pool))
        }
        "postgres" | "postgresql" => {
            let pool = PgPoolOptions::new()
                .max_connections(config.max_connections)
                .connect(&config.url)
                .await
                .map_err(|e| {
                    AppError::DatabaseError(format!("Failed to connect to PostgreSQL: {}", e))
                })?;
            Ok(DatabasePool::Postgres(pool))
        }
        "mysql" => {
            Err(AppError::ConfigError(
                "MySQL repositories are not implemented; use sqlite for development or postgres for production"
                    .to_string(),
            ))
        }
        _ => Err(AppError::ConfigError(format!(
            "Unsupported database type: {}",
            config.database_type
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn mysql_fails_before_opening_pool() {
        let config = DatabaseConfig {
            database_type: "mysql".to_string(),
            url: "mysql://127.0.0.1:1/evolith".to_string(),
            max_connections: 1,
            seed_database: false,
        };

        let err = match create_pool(&config).await {
            Ok(_) => panic!("mysql should fail fast"),
            Err(err) => err,
        };

        assert!(matches!(err, AppError::ConfigError(_)));
        assert!(err
            .to_string()
            .contains("MySQL repositories are not implemented"));
    }
}
