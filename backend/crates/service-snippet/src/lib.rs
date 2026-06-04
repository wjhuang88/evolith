//! CLI interface service compatibility layer
//!
//! The crate name remains `service-snippet` while the legacy snippet model is
//! migrated. New product work should use CLI interface terminology.

pub mod parser;

pub use domain::SnippetRepository;
pub use parser::{CliInterfaceDocument, CliInterfaceMetadata, CliInterfaceParser};
