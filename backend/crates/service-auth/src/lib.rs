//! Auth Service
//!
//! Handles authentication, authorization, and session management.

pub mod jwt;
pub mod rbac;
pub mod session;
pub mod password;

pub use jwt::JwtHandler;
pub use password::Argon2Hasher;

// TODO: Implement and export RbacService and SessionManager
