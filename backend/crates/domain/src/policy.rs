use common::error::{AppError, Result};
use serde::{Deserialize, Serialize};

/// Parsed `.evolith/policy.yaml` content.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EvolithPolicy {
    #[serde(default = "default_version")]
    pub version: u32,
    #[serde(default)]
    pub default_action: DefaultAction,
    #[serde(default)]
    pub protected_paths: Vec<String>,
    #[serde(default)]
    pub agents: Vec<AgentPolicy>,
}

fn default_version() -> u32 {
    1
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DefaultAction {
    AutoMerge,
    #[default]
    RequireReview,
    Block,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgentPolicy {
    pub name: String,
    #[serde(default)]
    pub scopes: Vec<Scope>,
    #[serde(default)]
    pub auto_merge: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Scope {
    Read,
    CommitAll,
    CommitPath(String),
    Promote,
}

impl Serialize for Scope {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Scope::Read => serializer.serialize_str("read"),
            Scope::CommitAll => serializer.serialize_str("commit"),
            Scope::CommitPath(path) => serializer.serialize_str(&format!("commit:{}", path)),
            Scope::Promote => serializer.serialize_str("promote"),
        }
    }
}

impl<'de> Deserialize<'de> for Scope {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "read" => Ok(Scope::Read),
            "commit" => Ok(Scope::CommitAll),
            "promote" => Ok(Scope::Promote),
            _ if s.starts_with("commit:") => {
                Ok(Scope::CommitPath(s["commit:".len()..].to_string()))
            }
            _ => Err(serde::de::Error::custom(format!(
                "Unknown scope: {}. Expected 'read', 'commit', 'commit:<path>', or 'promote'",
                s
            ))),
        }
    }
}

impl EvolithPolicy {
    /// Parse a YAML string into an `EvolithPolicy`.
    ///
    /// Missing `version` field is treated as v1 for backward compatibility.
    /// Unrecognised fields are ignored.
    pub fn parse(yaml: &str) -> Result<Self> {
        serde_norway::from_str::<EvolithPolicy>(yaml).map_err(|e| {
            AppError::ValidationError(format!("Failed to parse .evolith/policy.yaml: {}", e))
        })
    }

    /// Parse a YAML string, falling back to defaults if the input is empty.
    pub fn parse_or_default(yaml: &str) -> Self {
        if yaml.trim().is_empty() {
            return Self::default();
        }
        Self::parse(yaml).unwrap_or_default()
    }
}

impl Default for EvolithPolicy {
    fn default() -> Self {
        Self {
            version: 1,
            default_action: DefaultAction::RequireReview,
            protected_paths: vec![],
            agents: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_auto_merge() {
        let yaml = r#"
version: 1
default_action: auto_merge
protected_paths:
  - SKILL.md
  - mcp/tool.yaml
agents:
  - name: "ci-bot"
    scopes: ["read", "commit:src/"]
    auto_merge: true
"#;
        let policy = EvolithPolicy::parse(yaml).unwrap();
        assert_eq!(policy.version, 1);
        assert_eq!(policy.default_action, DefaultAction::AutoMerge);
        assert_eq!(policy.protected_paths.len(), 2);
        assert_eq!(policy.protected_paths[0], "SKILL.md");
        assert_eq!(policy.agents.len(), 1);
        assert_eq!(policy.agents[0].name, "ci-bot");
        assert_eq!(policy.agents[0].auto_merge, Some(true));
    }

    #[test]
    fn test_parse_require_review() {
        let yaml = "default_action: require_review";
        let policy = EvolithPolicy::parse(yaml).unwrap();
        assert_eq!(policy.default_action, DefaultAction::RequireReview);
        assert_eq!(policy.version, 1);
        assert!(policy.protected_paths.is_empty());
        assert!(policy.agents.is_empty());
    }

    #[test]
    fn test_parse_block() {
        let yaml = "default_action: block";
        let policy = EvolithPolicy::parse(yaml).unwrap();
        assert_eq!(policy.default_action, DefaultAction::Block);
    }

    #[test]
    fn test_parse_invalid_action() {
        let yaml = "default_action: invalid_value";
        let result = EvolithPolicy::parse(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_empty_string() {
        let policy = EvolithPolicy::parse_or_default("");
        assert_eq!(policy.default_action, DefaultAction::RequireReview);
        assert_eq!(policy.version, 1);
    }

    #[test]
    fn test_parse_whitespace_only() {
        let policy = EvolithPolicy::parse_or_default("  \n  ");
        assert_eq!(policy.default_action, DefaultAction::RequireReview);
    }

    #[test]
    fn test_parse_missing_version_defaults_to_v1() {
        let yaml = "default_action: auto_merge";
        let policy = EvolithPolicy::parse(yaml).unwrap();
        assert_eq!(policy.version, 1);
    }

    #[test]
    fn test_parse_multiple_agents() {
        let yaml = r#"
default_action: require_review
agents:
  - name: "reviewer"
    scopes: ["read"]
  - name: "deployer"
    scopes: ["read", "promote"]
    auto_merge: true
"#;
        let policy = EvolithPolicy::parse(yaml).unwrap();
        assert_eq!(policy.agents.len(), 2);
        assert_eq!(policy.agents[0].name, "reviewer");
        assert!(policy.agents[0].auto_merge.is_none());
        assert_eq!(policy.agents[1].name, "deployer");
        assert_eq!(policy.agents[1].auto_merge, Some(true));
    }

    #[test]
    fn test_parse_protected_paths_glob() {
        let yaml = r#"
default_action: block
protected_paths:
  - "*.md"
  - "mcp/**"
  - "cli/?.yaml"
"#;
        let policy = EvolithPolicy::parse(yaml).unwrap();
        assert_eq!(policy.protected_paths.len(), 3);
        assert!(policy.protected_paths.contains(&"*.md".to_string()));
        assert!(policy.protected_paths.contains(&"mcp/**".to_string()));
        assert!(policy.protected_paths.contains(&"cli/?.yaml".to_string()));
    }

    #[test]
    fn test_parse_unknown_fields_ignored() {
        let yaml = r#"
default_action: auto_merge
unknown_field: should_be_ignored
nested:
  also: ignored
"#;
        let policy = EvolithPolicy::parse(yaml).unwrap();
        assert_eq!(policy.default_action, DefaultAction::AutoMerge);
    }

    #[test]
    fn test_parse_scopes_variants() {
        let yaml = r#"
default_action: require_review
agents:
  - name: "read-only"
    scopes: ["read"]
  - name: "path-committer"
    scopes: ["commit:docs/"]
"#;
        let policy = EvolithPolicy::parse(yaml).unwrap();
        assert_eq!(policy.agents[0].scopes.len(), 1);
        assert_eq!(policy.agents[1].scopes.len(), 1);
    }
}
