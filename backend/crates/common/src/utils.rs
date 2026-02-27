//! Utility functions

use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Generate a new UUID v4
pub fn generate_uuid() -> Uuid {
    Uuid::new_v4()
}

/// Get current UTC timestamp
pub fn now_utc() -> DateTime<Utc> {
    Utc::now()
}

/// Check if a string is not empty after trimming
pub fn is_not_empty(s: &str) -> bool {
    !s.trim().is_empty()
}
