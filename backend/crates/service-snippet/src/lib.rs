//! Snippet Service
//!
//! Handles code snippet management, search, and reference generation.

pub mod parser;
pub mod reference;
pub mod repository;
pub mod search;

pub use domain::SnippetRepository;
pub use reference::ReferenceGenerator;
pub use search::SnippetSearch;
