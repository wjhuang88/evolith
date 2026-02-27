//! Snippet Service
//!
//! Handles code snippet management, search, and reference generation.

pub mod repository;
pub mod search;
pub mod parser;
pub mod reference;

pub use repository::SnippetRepository;
pub use search::SnippetSearch;
pub use reference::ReferenceGenerator;
