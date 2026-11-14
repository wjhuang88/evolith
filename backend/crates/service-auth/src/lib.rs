//! Auth Service
//!
//! Handles authentication, authorization, and session management.

pub mod jwt;
pub mod login;
pub mod password;
pub mod rbac;
pub mod session;

pub use jwt::{Claims, JwtHandler};
pub use login::{LoginError, LoginOutcome, LoginService};
pub use password::Argon2Hasher;

// TODO: Implement and export RbacService and SessionManager
