//! API Layer
//!
//! This crate handles HTTP routing, request handling, and response formatting.

pub mod handlers;
pub mod middleware;
pub mod routes;
pub mod dto;

pub use routes::configure_routes;
