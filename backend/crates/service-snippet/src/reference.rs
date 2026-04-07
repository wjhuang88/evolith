//! Reference generator

use common::error::Result;

/// Reference generator for code snippets
pub struct ReferenceGenerator;

impl Default for ReferenceGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl ReferenceGenerator {
    pub fn new() -> Self {
        Self
    }

    pub async fn generate(&self, _snippet_id: &str) -> Result<String> {
        Ok("// Reference generation not implemented".to_string())
    }
}
