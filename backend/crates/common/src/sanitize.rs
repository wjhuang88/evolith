//! Log sanitization utilities
//!
//! Provides functions to redact sensitive information from log output.
//! Replaces values of known sensitive fields with "[REDACTED]".

/// List of field names that should be redacted in log output
const SENSITIVE_FIELDS: &[&str] = &[
    "password",
    "token",
    "secret",
    "api_key",
    "apikey",
    "authorization",
    "cookie",
    "credential",
    "private_key",
    "access_token",
    "refresh_token",
];

/// Sanitize a key-value pair for logging.
/// If the key matches a known sensitive field (case-insensitive), the value is replaced with "[REDACTED]".
///
/// # Examples
/// ```
/// use common::sanitize::sanitize_value;
///
/// assert_eq!(sanitize_value("username", "alice"), "alice");
/// assert_eq!(sanitize_value("password", "s3cret"), "[REDACTED]");
/// assert_eq!(sanitize_value("API_KEY", "abc123"), "[REDACTED]");
/// ```
pub fn sanitize_value<'a>(key: &str, value: &'a str) -> &'a str {
    let key_lower = key.to_lowercase();
    for field in SENSITIVE_FIELDS {
        if key_lower.contains(field) {
            return "[REDACTED]";
        }
    }
    value
}

/// Sanitize a JSON-like string by replacing values of sensitive fields.
/// This performs simple pattern matching — not full JSON parsing — suitable for log output.
///
/// Matches patterns like `"password":"value"` and `"password": "value"` and replaces
/// the value with `"[REDACTED]"`.
pub fn sanitize_log_string(input: &str) -> String {
    let mut result = input.to_string();
    for field in SENSITIVE_FIELDS {
        let patterns = [
            format!("\"{}\":\"", field),
            format!("\"{}\": \"", field),
            format!("\"{}\" : \"", field),
        ];
        for pattern in &patterns {
            while let Some(start) = result.to_lowercase().find(&pattern.to_lowercase()) {
                let value_start = start + pattern.len();
                if let Some(end) = result[value_start..].find('"') {
                    let before = &result[..start];
                    let key_part = &result[start..value_start];
                    let after = &result[value_start + end..];
                    result = format!("{}{}[REDACTED]{}", before, key_part, after);
                } else {
                    break;
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_value_safe_field() {
        assert_eq!(sanitize_value("username", "alice"), "alice");
        assert_eq!(sanitize_value("email", "a@b.com"), "a@b.com");
    }

    #[test]
    fn test_sanitize_value_sensitive_field() {
        assert_eq!(sanitize_value("password", "s3cret"), "[REDACTED]");
        assert_eq!(sanitize_value("api_key", "abc123"), "[REDACTED]");
        assert_eq!(sanitize_value("Authorization", "Bearer xyz"), "[REDACTED]");
        assert_eq!(sanitize_value("JWT_SECRET", "hmac"), "[REDACTED]");
        assert_eq!(sanitize_value("access_token", "tok"), "[REDACTED]");
    }

    #[test]
    fn test_sanitize_value_case_insensitive() {
        assert_eq!(sanitize_value("PASSWORD", "s3cret"), "[REDACTED]");
        assert_eq!(sanitize_value("Api_Key", "abc"), "[REDACTED]");
        assert_eq!(sanitize_value("TOKEN", "xyz"), "[REDACTED]");
    }

    #[test]
    fn test_sanitize_log_string() {
        let input = r#"{"username":"alice","password":"s3cret","email":"a@b.com"}"#;
        let output = sanitize_log_string(input);
        assert!(output.contains("\"username\":\"alice\""));
        assert!(output.contains("[REDACTED]"));
        assert!(!output.contains("s3cret"));
    }

    #[test]
    fn test_sanitize_log_string_with_spaces() {
        let input = r#"{"token": "mytoken123"}"#;
        let output = sanitize_log_string(input);
        assert!(output.contains("[REDACTED]"));
        assert!(!output.contains("mytoken123"));
    }

    #[test]
    fn test_sanitize_log_string_no_sensitive() {
        let input = r#"{"name":"test","count":42}"#;
        let output = sanitize_log_string(input);
        assert_eq!(input, output);
    }
}
