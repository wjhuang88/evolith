//! Logging utilities with request tracing

use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use uuid::Uuid;

pub const REQUEST_ID_HEADER: &str = "X-Request-ID";
const REQUEST_ID_LENGTH: usize = 16;

pub fn generate_request_id() -> String {
    Uuid::new_v4().to_string()[..REQUEST_ID_LENGTH].to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestContext {
    pub request_id: String,
    pub path: Option<String>,
    pub method: Option<String>,
    pub user_id: Option<String>,
    pub client_ip: Option<String>,
}

impl RequestContext {
    pub fn new(request_id: String) -> Self {
        Self {
            request_id,
            path: None,
            method: None,
            user_id: None,
            client_ip: None,
        }
    }
    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }
    pub fn with_method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into());
        self
    }
    pub fn with_user(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }
    pub fn with_ip(mut self, ip: impl Into<String>) -> Self {
        self.client_ip = Some(ip.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogConfig {
    pub level: LogLevel,
    pub log_requests: bool,
    pub log_request_body: bool,
    pub log_response_body: bool,
    pub slow_request_threshold_ms: u64,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            log_requests: true,
            log_request_body: false,
            log_response_body: false,
            slow_request_threshold_ms: 1000,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "TRACE"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

#[derive(Clone)]
pub struct RequestTiming {
    start: Instant,
}

impl RequestTiming {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
    pub fn elapsed_ms(&self) -> u64 {
        self.elapsed().as_millis() as u64
    }
}

impl Default for RequestTiming {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub request_id: Option<String>,
    pub message: String,
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<serde_json::Value>,
}

impl LogEntry {
    pub fn new(level: LogLevel, message: impl Into<String>) -> Self {
        Self {
            timestamp: chrono::Utc::now().to_rfc3339(),
            level: level.to_string(),
            request_id: None,
            message: message.into(),
            target: None,
            fields: None,
        }
    }
    pub fn with_request_id(mut self, id: impl Into<String>) -> Self {
        self.request_id = Some(id.into());
        self
    }
    pub fn with_target(mut self, target: impl Into<String>) -> Self {
        self.target = Some(target.into());
        self
    }
    pub fn with_fields(mut self, fields: serde_json::Value) -> Self {
        self.fields = Some(fields);
        self
    }
}

pub fn init_tracing(config: &LogConfig) {
    use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(config.level.to_string()));
    let fmt_layer = fmt::layer()
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true);
    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .init();
    tracing::info!("Logging initialized at {} level", config.level);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_generate_request_id() {
        let id1 = generate_request_id();
        let id2 = generate_request_id();
        assert_eq!(id1.len(), REQUEST_ID_LENGTH);
        assert_eq!(id2.len(), REQUEST_ID_LENGTH);
        assert_ne!(id1, id2);
    }
    #[test]
    fn test_request_context() {
        let ctx = RequestContext::new("test123".to_string())
            .with_path("/api/users")
            .with_method("GET")
            .with_user("user1")
            .with_ip("192.168.1.1");
        assert_eq!(ctx.request_id, "test123");
        assert_eq!(ctx.path, Some("/api/users".to_string()));
        assert_eq!(ctx.method, Some("GET".to_string()));
    }
    #[test]
    fn test_log_entry() {
        let entry = LogEntry::new(LogLevel::Info, "Test message")
            .with_request_id("req123")
            .with_target("test_module")
            .with_fields(serde_json::json!({"key": "value"}));
        assert_eq!(entry.level, "INFO");
        assert_eq!(entry.message, "Test message");
    }
    #[test]
    fn test_log_config_default() {
        let config = LogConfig::default();
        assert_eq!(config.level, LogLevel::Info);
        assert!(config.log_requests);
    }
    #[test]
    fn test_request_timing() {
        let timing = RequestTiming::new();
        std::thread::sleep(Duration::from_millis(10));
        assert!(timing.elapsed_ms() >= 10);
    }
}
