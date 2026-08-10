//! API Layer
//!
//! This crate handles HTTP routing, request handling, and response formatting.

pub mod dto;
pub mod handlers;
pub mod middleware;
pub mod routes;
pub mod services;
pub mod state;

pub use routes::configure_routes;
