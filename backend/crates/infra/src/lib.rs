//! Infrastructure Layer
//!
//! Contains database, cache, storage, and external service implementations.

pub mod cache;
pub mod config;
pub mod db;
pub mod mailer;
pub mod storage;

pub use cache::{create_cache, create_redis_connection, Cache, InMemoryCache, RedisCache};
pub use config::{AppConfig, JwtConfig};
pub use mailer::Mailer;
