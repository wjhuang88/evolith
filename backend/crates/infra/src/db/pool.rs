//! Database pool management

use sqlx::mysql::MySqlPoolOptions;
use sqlx::postgres::PgPoolOptions;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{MySql, Postgres, SqlitePool};

use crate::config::DatabaseConfig;
use common::error::{AppError, Result};

pub enum DatabasePool {
    Sqlite(SqlitePool),
    Postgres(sqlx::Pool<Postgres>),
    MySql(sqlx::Pool<MySql>),
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
            let pool = MySqlPoolOptions::new()
                .max_connections(config.max_connections)
                .connect(&config.url)
                .await
                .map_err(|e| {
                    AppError::DatabaseError(format!("Failed to connect to MySQL: {}", e))
                })?;
            Ok(DatabasePool::MySql(pool))
        }
        _ => Err(AppError::ConfigError(format!(
            "Unsupported database type: {}",
            config.database_type
        ))),
    }
}
