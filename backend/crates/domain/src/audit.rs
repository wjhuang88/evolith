//! Audit models and types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: Uuid,
    pub tenant_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub action: String,
    pub resource_type: Option<String>,
    pub resource_id: Option<String>,
    pub details: serde_json::Value,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_log_creation() {
        let now = Utc::now();
        let log = AuditLog {
            id: Uuid::new_v4(),
            tenant_id: Some(Uuid::new_v4()),
            user_id: Some(Uuid::new_v4()),
            action: "user.login".to_string(),
            resource_type: Some("auth".to_string()),
            resource_id: Some("session-123".to_string()),
            details: serde_json::json!({"ip": "127.0.0.1"}),
            ip_address: Some("127.0.0.1".to_string()),
            user_agent: Some("Mozilla/5.0".to_string()),
            created_at: now,
        };

        assert_eq!(log.action, "user.login");
        assert!(log.tenant_id.is_some());
        assert!(log.user_id.is_some());
    }

    #[test]
    fn test_audit_log_serialization() {
        let now = Utc::now();
        let log = AuditLog {
            id: Uuid::new_v4(),
            tenant_id: Some(Uuid::new_v4()),
            user_id: Some(Uuid::new_v4()),
            action: "tool.create".to_string(),
            resource_type: Some("tool".to_string()),
            resource_id: Some("tool-uuid".to_string()),
            details: serde_json::json!({"name": "http_request"}),
            ip_address: None,
            user_agent: None,
            created_at: now,
        };

        let json = serde_json::to_string(&log).unwrap();
        assert!(json.contains("tool.create"));
        assert!(json.contains("tool"));
    }

    #[test]
    fn test_audit_log_with_null_tenant() {
        let now = Utc::now();
        let log = AuditLog {
            id: Uuid::new_v4(),
            tenant_id: None, // System-level audit log
            user_id: None,
            action: "system.startup".to_string(),
            resource_type: None,
            resource_id: None,
            details: serde_json::json!({"version": "1.0.0"}),
            ip_address: None,
            user_agent: None,
            created_at: now,
        };

        assert!(log.tenant_id.is_none());
        assert!(log.user_id.is_none());
    }

    #[test]
    fn test_audit_log_complex_details() {
        let now = Utc::now();
        let log = AuditLog {
            id: Uuid::new_v4(),
            tenant_id: Some(Uuid::new_v4()),
            user_id: Some(Uuid::new_v4()),
            action: "skill.execute".to_string(),
            resource_type: Some("skill".to_string()),
            resource_id: Some("skill-uuid".to_string()),
            details: serde_json::json!({
                "skill_name": "file_reader",
                "arguments": {
                    "path": "/tmp/test.txt",
                    "max_size": 1024
                },
                "result": {
                    "success": true,
                    "lines_read": 42
                }
            }),
            ip_address: Some("192.168.1.100".to_string()),
            user_agent: Some("Evolith-CLI/1.0".to_string()),
            created_at: now,
        };

        // Verify complex JSON details are properly serialized
        let json = serde_json::to_string(&log).unwrap();
        assert!(json.contains("file_reader"));
        assert!(json.contains("lines_read"));
    }
}
