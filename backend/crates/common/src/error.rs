//! Error handling
//!
//! Comprehensive error types for the Evolith application.

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Result type alias for convenience
pub type Result<T> = std::result::Result<T, AppError>;

/// Application-wide error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppError {
    // ============ Core Errors ============
    /// Database error
    DatabaseError(String),
    /// Not found error
    NotFoundError(String),
    /// Validation error (input validation)
    ValidationError(String),
    /// Authentication error
    AuthenticationError(String),
    /// Authorization error
    AuthorizationError(String),
    /// Internal server error
    InternalError(String),
    /// Configuration error
    ConfigError(String),

    // ============ Business Errors ============
    /// External service error (MCP tools, skill execution, etc.)
    ExternalServiceError {
        /// Service name that failed
        service: String,
        /// Error message
        message: String,
    },
    /// Storage error (MinIO/S3 operations)
    StorageError(String),
    /// Rate limit exceeded
    RateLimitError {
        /// Retry after seconds
        retry_after: u64,
        /// Error message
        message: String,
    },
    /// Token error (JWT specific)
    TokenError(String),
    /// Business logic error
    BusinessError(String),

    // ============ Resource Errors ============
    /// Resource already exists
    ConflictError(String),
    /// Resource temporarily unavailable
    ServiceUnavailableError(String),
}

/// Error code mapping for client-side handling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorCode {
    /// Error code string
    pub code: String,
    /// HTTP status code
    pub status: u16,
    /// Whether this error is retryable
    pub retryable: bool,
}

impl AppError {
    /// Get error code information
    pub fn error_code_info(&self) -> ErrorCode {
        match self {
            AppError::DatabaseError(_) => ErrorCode {
                code: "DATABASE_ERROR".to_string(),
                status: 500,
                retryable: false,
            },
            AppError::NotFoundError(_) => ErrorCode {
                code: "NOT_FOUND".to_string(),
                status: 404,
                retryable: false,
            },
            AppError::ValidationError(_) => ErrorCode {
                code: "VALIDATION_ERROR".to_string(),
                status: 400,
                retryable: false,
            },
            AppError::AuthenticationError(_) => ErrorCode {
                code: "UNAUTHORIZED".to_string(),
                status: 401,
                retryable: false,
            },
            AppError::AuthorizationError(_) => ErrorCode {
                code: "FORBIDDEN".to_string(),
                status: 403,
                retryable: false,
            },
            AppError::InternalError(_) => ErrorCode {
                code: "INTERNAL_ERROR".to_string(),
                status: 500,
                retryable: false,
            },
            AppError::ConfigError(_) => ErrorCode {
                code: "CONFIG_ERROR".to_string(),
                status: 500,
                retryable: false,
            },
            AppError::ExternalServiceError { .. } => ErrorCode {
                code: "EXTERNAL_SERVICE_ERROR".to_string(),
                status: 502,
                retryable: true,
            },
            AppError::StorageError(_) => ErrorCode {
                code: "STORAGE_ERROR".to_string(),
                status: 500,
                retryable: true,
            },
            AppError::RateLimitError { .. } => ErrorCode {
                code: "RATE_LIMIT_ERROR".to_string(),
                status: 429,
                retryable: true,
            },
            AppError::TokenError(_) => ErrorCode {
                code: "TOKEN_ERROR".to_string(),
                status: 401,
                retryable: false,
            },
            AppError::BusinessError(_) => ErrorCode {
                code: "BUSINESS_ERROR".to_string(),
                status: 400,
                retryable: false,
            },
            AppError::ConflictError(_) => ErrorCode {
                code: "CONFLICT_ERROR".to_string(),
                status: 409,
                retryable: false,
            },
            AppError::ServiceUnavailableError(_) => ErrorCode {
                code: "SERVICE_UNAVAILABLE".to_string(),
                status: 503,
                retryable: true,
            },
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AppError::NotFoundError(msg) => write!(f, "Not found: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::AuthenticationError(msg) => write!(f, "Authentication error: {}", msg),
            AppError::AuthorizationError(msg) => write!(f, "Authorization error: {}", msg),
            AppError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            AppError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            AppError::ExternalServiceError { service, message } => {
                write!(f, "External service error [{}]: {}", service, message)
            }
            AppError::StorageError(msg) => write!(f, "Storage error: {}", msg),
            AppError::RateLimitError { message, .. } => write!(f, "Rate limit: {}", message),
            AppError::TokenError(msg) => write!(f, "Token error: {}", msg),
            AppError::BusinessError(msg) => write!(f, "Business error: {}", msg),
            AppError::ConflictError(msg) => write!(f, "Conflict: {}", msg),
            AppError::ServiceUnavailableError(msg) => write!(f, "Service unavailable: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_info = self.error_code_info();

        HttpResponse::build(status_code).json(serde_json::json!({
            "success": false,
            "error": {
                "code": error_info.code,
                "message": self.to_string(),
                "retryable": error_info.retryable,
                "details": self.get_details()
            }
        }))
    }

    fn status_code(&self) -> StatusCode {
        let info = self.error_code_info();
        StatusCode::from_u16(info.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl AppError {
    /// Get additional details for certain error types
    fn get_details(&self) -> Option<serde_json::Value> {
        match self {
            AppError::RateLimitError { retry_after, .. } => {
                Some(serde_json::json!({ "retry_after": retry_after }))
            }
            AppError::ExternalServiceError { service, .. } => {
                Some(serde_json::json!({ "service": service }))
            }
            _ => None,
        }
    }
}

// ============ From Implementations ============

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFoundError("Record not found".to_string()),
            _ => AppError::DatabaseError(err.to_string()),
        }
    }
}

impl From<config::ConfigError> for AppError {
    fn from(err: config::ConfigError) -> Self {
        AppError::ConfigError(err.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::InternalError(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::ValidationError(format!("JSON error: {}", err))
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(err: std::num::ParseIntError) -> Self {
        AppError::ValidationError(format!("Parse error: {}", err))
    }
}

impl From<std::num::ParseFloatError> for AppError {
    fn from(err: std::num::ParseFloatError) -> Self {
        AppError::ValidationError(format!("Parse error: {}", err))
    }
}

// ============ Helper Methods ============

impl AppError {
    /// Create a not found error
    pub fn not_found(entity: &str, id: &str) -> Self {
        AppError::NotFoundError(format!("{} with id '{}' not found", entity, id))
    }

    /// Create a validation error
    pub fn validation(message: impl Into<String>) -> Self {
        AppError::ValidationError(message.into())
    }

    /// Create an authentication error
    pub fn auth(message: impl Into<String>) -> Self {
        AppError::AuthenticationError(message.into())
    }

    /// Create an authorization error
    pub fn forbidden(message: impl Into<String>) -> Self {
        AppError::AuthorizationError(message.into())
    }

    /// Create an external service error
    pub fn external_service(service: impl Into<String>, message: impl Into<String>) -> Self {
        AppError::ExternalServiceError {
            service: service.into(),
            message: message.into(),
        }
    }

    /// Create a rate limit error
    pub fn rate_limit(retry_after: u64, message: impl Into<String>) -> Self {
        AppError::RateLimitError {
            retry_after,
            message: message.into(),
        }
    }

    /// Create a token error
    pub fn token(message: impl Into<String>) -> Self {
        AppError::TokenError(message.into())
    }

    /// Create a conflict error
    pub fn conflict(message: impl Into<String>) -> Self {
        AppError::ConflictError(message.into())
    }

    /// Create a business error
    pub fn business(message: impl Into<String>) -> Self {
        AppError::BusinessError(message.into())
    }
}

/// Extension trait for Option to convert to AppError
pub trait OptionExt<T> {
    fn not_found(self, entity: &str, id: &str) -> Result<T>;
}

impl<T> OptionExt<T> for Option<T> {
    fn not_found(self, entity: &str, id: &str) -> Result<T> {
        self.ok_or_else(|| AppError::not_found(entity, id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = AppError::not_found("User", "123");
        assert_eq!(err.to_string(), "Not found: User with id '123' not found");

        let err = AppError::validation("Invalid email format");
        assert_eq!(err.to_string(), "Validation error: Invalid email format");

        let err = AppError::external_service("mcp-server", "Connection failed");
        assert_eq!(
            err.to_string(),
            "External service error [mcp-server]: Connection failed"
        );
    }

    #[test]
    fn test_error_code_info() {
        let err = AppError::not_found("User", "123");
        let info = err.error_code_info();
        assert_eq!(info.code, "NOT_FOUND");
        assert_eq!(info.status, 404);
        assert!(!info.retryable);

        let err = AppError::rate_limit(60, "Too many requests");
        let info = err.error_code_info();
        assert_eq!(info.code, "RATE_LIMIT_ERROR");
        assert_eq!(info.status, 429);
        assert!(info.retryable);
    }

    #[test]
    fn test_helper_methods() {
        let err = AppError::not_found("Tool", "uuid");
        assert!(matches!(err, AppError::NotFoundError(_)));

        let err = AppError::validation("Invalid input");
        assert!(matches!(err, AppError::ValidationError(_)));

        let err = AppError::external_service("skill-executor", "Timeout");
        assert!(matches!(err, AppError::ExternalServiceError { .. }));

        let err = AppError::rate_limit(30, "Try again later");
        assert!(matches!(
            err,
            AppError::RateLimitError {
                retry_after: 30,
                ..
            }
        ));
    }

    #[test]
    fn test_option_ext() {
        let value: Option<i32> = Some(42);
        let result = value.not_found("Test", "1");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);

        let value: Option<i32> = None;
        let result = value.not_found("Test", "1");
        assert!(result.is_err());
    }
}
