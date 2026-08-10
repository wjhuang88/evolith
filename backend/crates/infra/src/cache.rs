//! Cache layer for Evolith infrastructure
//!
//! Provides a unified cache interface with two implementations:
//! - `InMemoryCache`: Thread-safe in-memory cache for development/testing
//! - `RedisCache`: Production-ready Redis-backed cache

use async_trait::async_trait;
use common::error::{AppError, Result};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

use crate::config::RedisConfig;

/// Cache entry for in-memory storage
#[derive(Debug, Clone)]
struct CacheEntry {
    value: String,
    expires_at: Option<Instant>,
}

impl CacheEntry {
    fn new(value: String, ttl: Option<Duration>) -> Self {
        let expires_at = ttl.map(|duration| Instant::now() + duration);
        Self { value, expires_at }
    }

    fn is_expired(&self) -> bool {
        match self.expires_at {
            Some(expires_at) => Instant::now() >= expires_at,
            None => false,
        }
    }
}

/// Cache trait for async key-value storage operations
///
/// All methods are designed to be object-safe for dynamic dispatch.
#[async_trait]
pub trait Cache: Send + Sync {
    /// Get a value from the cache
    ///
    /// Returns `Ok(None)` if the key doesn't exist or has expired.
    async fn get(&self, key: &str) -> Result<Option<String>>;

    /// Set a value in the cache without expiration
    async fn set(&self, key: &str, value: &str) -> Result<()>;

    /// Set a value in the cache with a time-to-live
    async fn set_with_ttl(&self, key: &str, value: &str, ttl: Duration) -> Result<()>;

    /// Delete a key from the cache
    ///
    /// Returns `Ok(true)` if the key was deleted, `Ok(false)` if it didn't exist.
    async fn delete(&self, key: &str) -> Result<bool>;

    /// Check if a key exists in the cache
    async fn exists(&self, key: &str) -> Result<bool>;

    /// Increment a counter by the given delta
    ///
    /// If the key doesn't exist, it's initialized to 0 before incrementing.
    /// Returns the new value after incrementing.
    async fn increment(&self, key: &str, delta: i64) -> Result<i64>;

    /// Set expiration time on an existing key
    ///
    /// Returns `Ok(true)` if the expiration was set, `Ok(false)` if the key doesn't exist.
    async fn expire(&self, key: &str, ttl: Duration) -> Result<bool>;
}

/// In-memory cache implementation using HashMap with TTL support
///
/// Thread-safe via `RwLock`. TTL is checked lazily on read operations.
#[derive(Debug, Default)]
pub struct InMemoryCache {
    data: RwLock<HashMap<String, CacheEntry>>,
}

impl InMemoryCache {
    /// Create a new empty in-memory cache
    pub fn new() -> Self {
        Self {
            data: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl Cache for InMemoryCache {
    async fn get(&self, key: &str) -> Result<Option<String>> {
        let read_guard = self.data.read().await;
        match read_guard.get(key) {
            Some(entry) if !entry.is_expired() => Ok(Some(entry.value.clone())),
            Some(_) => {
                // Entry is expired - drop read lock and acquire write lock for cleanup
                drop(read_guard);
                let mut write_guard = self.data.write().await;
                write_guard.remove(key);
                Ok(None)
            }
            None => Ok(None),
        }
    }

    async fn set(&self, key: &str, value: &str) -> Result<()> {
        let mut write_guard = self.data.write().await;
        write_guard.insert(key.to_string(), CacheEntry::new(value.to_string(), None));
        Ok(())
    }

    async fn set_with_ttl(&self, key: &str, value: &str, ttl: Duration) -> Result<()> {
        let mut write_guard = self.data.write().await;
        write_guard.insert(
            key.to_string(),
            CacheEntry::new(value.to_string(), Some(ttl)),
        );
        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<bool> {
        let mut write_guard = self.data.write().await;
        Ok(write_guard.remove(key).is_some())
    }

    async fn exists(&self, key: &str) -> Result<bool> {
        let read_guard = self.data.read().await;
        match read_guard.get(key) {
            Some(entry) if !entry.is_expired() => Ok(true),
            Some(_) => {
                // Entry is expired - doesn't count as existing
                Ok(false)
            }
            None => Ok(false),
        }
    }

    async fn increment(&self, key: &str, delta: i64) -> Result<i64> {
        let mut write_guard = self.data.write().await;

        // Check if key exists and is not expired
        let current = match write_guard.get(key) {
            Some(entry) if !entry.is_expired() => entry.value.parse::<i64>().map_err(|_| {
                AppError::ValidationError(format!("Value at key '{}' is not a valid integer", key))
            })?,
            Some(_) => {
                // Expired entry - remove it and start from 0
                write_guard.remove(key);
                0
            }
            None => 0,
        };

        let new_value = current + delta;
        write_guard.insert(
            key.to_string(),
            CacheEntry::new(new_value.to_string(), None),
        );
        Ok(new_value)
    }

    async fn expire(&self, key: &str, ttl: Duration) -> Result<bool> {
        let mut write_guard = self.data.write().await;
        match write_guard.get(key) {
            Some(entry) if !entry.is_expired() => {
                let value = entry.value.clone();
                write_guard.insert(key.to_string(), CacheEntry::new(value, Some(ttl)));
                Ok(true)
            }
            Some(_) => {
                // Expired entry - treat as non-existent
                write_guard.remove(key);
                Ok(false)
            }
            None => Ok(false),
        }
    }
}

/// Redis-backed cache implementation
///
/// Uses `ConnectionManager` for automatic connection pooling and reconnection.
pub struct RedisCache {
    conn: redis::aio::ConnectionManager,
}

impl RedisCache {
    /// Create a new Redis cache from an existing connection manager
    pub fn new(conn: redis::aio::ConnectionManager) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl Cache for RedisCache {
    async fn get(&self, key: &str) -> Result<Option<String>> {
        let mut conn = self.conn.clone();
        let result: Option<String> = redis::cmd("GET")
            .arg(key)
            .query_async(&mut conn)
            .await
            .map_err(|e| AppError::InternalError(format!("Redis GET error: {}", e)))?;
        Ok(result)
    }

    async fn set(&self, key: &str, value: &str) -> Result<()> {
        let mut conn = self.conn.clone();
        redis::cmd("SET")
            .arg(key)
            .arg(value)
            .query_async::<()>(&mut conn)
            .await
            .map_err(|e| AppError::InternalError(format!("Redis SET error: {}", e)))?;
        Ok(())
    }

    async fn set_with_ttl(&self, key: &str, value: &str, ttl: Duration) -> Result<()> {
        let mut conn = self.conn.clone();
        let ttl_secs = ttl.as_secs();
        redis::cmd("SETEX")
            .arg(key)
            .arg(ttl_secs)
            .arg(value)
            .query_async::<()>(&mut conn)
            .await
            .map_err(|e| AppError::InternalError(format!("Redis SETEX error: {}", e)))?;
        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<bool> {
        let mut conn = self.conn.clone();
        let deleted: i64 = redis::cmd("DEL")
            .arg(key)
            .query_async(&mut conn)
            .await
            .map_err(|e| AppError::InternalError(format!("Redis DEL error: {}", e)))?;
        Ok(deleted > 0)
    }

    async fn exists(&self, key: &str) -> Result<bool> {
        let mut conn = self.conn.clone();
        let exists: i64 = redis::cmd("EXISTS")
            .arg(key)
            .query_async(&mut conn)
            .await
            .map_err(|e| AppError::InternalError(format!("Redis EXISTS error: {}", e)))?;
        Ok(exists > 0)
    }

    async fn increment(&self, key: &str, delta: i64) -> Result<i64> {
        let mut conn = self.conn.clone();
        let result: i64 = redis::cmd("INCRBY")
            .arg(key)
            .arg(delta)
            .query_async(&mut conn)
            .await
            .map_err(|e| AppError::InternalError(format!("Redis INCRBY error: {}", e)))?;
        Ok(result)
    }

    async fn expire(&self, key: &str, ttl: Duration) -> Result<bool> {
        let mut conn = self.conn.clone();
        let ttl_secs = ttl.as_secs();
        let result: i64 = redis::cmd("EXPIRE")
            .arg(key)
            .arg(ttl_secs)
            .query_async(&mut conn)
            .await
            .map_err(|e| AppError::InternalError(format!("Redis EXPIRE error: {}", e)))?;
        Ok(result > 0)
    }
}

/// Create a Redis connection manager from configuration
///
/// # Errors
///
/// Returns an error if the Redis URL is invalid or connection fails.
pub async fn create_redis_connection(
    config: &RedisConfig,
) -> Result<redis::aio::ConnectionManager> {
    let client = redis::Client::open(config.url.as_str())
        .map_err(|e| AppError::ConfigError(format!("Invalid Redis URL: {}", e)))?;

    // Apply a 5-second timeout so lite mode (no Redis) doesn't hang forever
    let conn = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        client.get_connection_manager(),
    )
    .await
    .map_err(|_| {
        AppError::InternalError(format!(
            "Redis connection timed out after 5s ({})",
            config.url
        ))
    })?
    .map_err(|e| AppError::InternalError(format!("Failed to connect to Redis: {}", e)))?;

    Ok(conn)
}

/// Create a cache instance based on configuration
///
/// If the Redis URL is configured and reachable, returns a `RedisCache`.
/// Otherwise, returns an `InMemoryCache` for development/testing.
///
/// # Arguments
///
/// * `config` - Redis configuration containing the connection URL
///
/// # Returns
///
/// A boxed cache trait object. Redis is preferred for production;
/// in-memory cache is used as a fallback or for development.
pub async fn create_cache(config: &RedisConfig, production: bool) -> Result<Box<dyn Cache>> {
    match create_redis_connection(config).await {
        Ok(conn) => {
            tracing::info!("Connected to Redis cache at {}", config.url);
            Ok(Box::new(RedisCache::new(conn)) as Box<dyn Cache>)
        }
        Err(e) => {
            if production {
                Err(AppError::ConfigError(format!(
                    "Redis is required in production: {}",
                    e
                )))
            } else {
                tracing::warn!("Failed to connect to Redis in development: {}. Falling back to in-memory cache.", e);
                Ok(Box::new(InMemoryCache::new()) as Box<dyn Cache>)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_in_memory_cache_basic_operations() {
        let cache = InMemoryCache::new();

        // Test set and get
        cache.set("key1", "value1").await.expect("set failed");
        let result = cache.get("key1").await.expect("get failed");
        assert_eq!(result, Some("value1".to_string()));

        // Test exists
        assert!(cache.exists("key1").await.expect("exists failed"));
        assert!(!cache.exists("nonexistent").await.expect("exists failed"));

        // Test delete
        let deleted = cache.delete("key1").await.expect("delete failed");
        assert!(deleted);
        let result = cache.get("key1").await.expect("get failed");
        assert_eq!(result, None);

        // Delete non-existent key
        let deleted = cache.delete("key1").await.expect("delete failed");
        assert!(!deleted);
    }

    #[tokio::test]
    async fn test_in_memory_cache_ttl() {
        let cache = InMemoryCache::new();

        // Set with TTL
        cache
            .set_with_ttl("ttl_key", "ttl_value", Duration::from_millis(50))
            .await
            .expect("set_with_ttl failed");

        // Should exist immediately
        assert!(cache.exists("ttl_key").await.expect("exists failed"));

        // Wait for expiration
        sleep(Duration::from_millis(100)).await;

        // Should be expired now
        let result = cache.get("ttl_key").await.expect("get failed");
        assert_eq!(result, None);
        assert!(!cache.exists("ttl_key").await.expect("exists failed"));
    }

    #[tokio::test]
    async fn test_in_memory_cache_increment() {
        let cache = InMemoryCache::new();

        // Increment non-existent key (starts at 0)
        let result = cache
            .increment("counter", 5)
            .await
            .expect("increment failed");
        assert_eq!(result, 5);

        // Increment existing key
        let result = cache
            .increment("counter", 3)
            .await
            .expect("increment failed");
        assert_eq!(result, 8);

        // Decrement (negative delta)
        let result = cache
            .increment("counter", -2)
            .await
            .expect("increment failed");
        assert_eq!(result, 6);
    }

    #[tokio::test]
    async fn test_in_memory_cache_expire() {
        let cache = InMemoryCache::new();

        // Set a key without TTL
        cache.set("expire_key", "value").await.expect("set failed");

        // Set expiration
        let expired = cache
            .expire("expire_key", Duration::from_millis(50))
            .await
            .expect("expire failed");
        assert!(expired);

        // Should exist immediately
        assert!(cache.exists("expire_key").await.expect("exists failed"));

        // Wait for expiration
        sleep(Duration::from_millis(100)).await;

        // Should be expired
        assert!(!cache.exists("expire_key").await.expect("exists failed"));

        // Expire non-existent key
        let expired = cache
            .expire("nonexistent", Duration::from_secs(10))
            .await
            .expect("expire failed");
        assert!(!expired);
    }

    #[tokio::test]
    async fn test_in_memory_cache_increment_non_integer_value() {
        let cache = InMemoryCache::new();

        // Set a non-integer value
        cache
            .set("string_key", "not_a_number")
            .await
            .expect("set failed");

        // Increment should fail
        let result = cache.increment("string_key", 1).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_cache_trait_object() {
        // Test that the trait is object-safe
        let cache: Box<dyn Cache> = Box::new(InMemoryCache::new());

        cache.set("test", "value").await.expect("set failed");
        let result = cache.get("test").await.expect("get failed");
        assert_eq!(result, Some("value".to_string()));
    }

    #[tokio::test]
    async fn production_invalid_redis_fails_closed() {
        let config = RedisConfig {
            url: "not-a-redis-url".to_string(),
        };
        assert!(create_cache(&config, true).await.is_err());
    }

    #[tokio::test]
    async fn development_invalid_redis_falls_back_to_memory() {
        let config = RedisConfig {
            url: "not-a-redis-url".to_string(),
        };
        let cache = create_cache(&config, false)
            .await
            .expect("development fallback");
        cache.set("fallback", "ok").await.expect("set");
        assert_eq!(
            cache.get("fallback").await.expect("get").as_deref(),
            Some("ok")
        );
    }
}
