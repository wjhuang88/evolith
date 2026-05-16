//! CLI interface service compatibility layer
//!
//! The crate name remains `service-snippet` while the legacy snippet model is
//! migrated. New product work should use CLI interface terminology.

pub mod parser;
pub mod reference;
pub mod repository;
pub mod search;

pub use domain::SnippetRepository;
pub use parser::{CliInterfaceDocument, CliInterfaceMetadata, CliInterfaceParser};
pub use reference::ReferenceGenerator;
pub use search::SnippetSearch;
