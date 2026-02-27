//! Snippet search

use common::error::Result;

/// Snippet search implementation
pub struct SnippetSearch;

impl SnippetSearch {
    pub fn new() -> Self {
        Self
    }

    pub async fn search(&self, _query: &str) -> Result<Vec<String>> {
        Ok(Vec::new())
    }
}
