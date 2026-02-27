//! Common utilities and shared functionality

pub mod constants;
pub mod error;
pub mod log;
pub mod utils;

pub use error::{AppError, Result};
pub use log::{generate_request_id, LogConfig, LogLevel, RequestContext};
