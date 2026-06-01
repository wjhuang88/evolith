//! Auth Service
//!
//! Handles authentication, authorization, and session management.

pub mod jwt;
pub mod password;
pub mod rbac;
pub mod session;

pub use jwt::{Claims, JwtHandler};
pub use password::Argon2Hasher;

// TODO: Implement and export RbacService and SessionManager
