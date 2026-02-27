//! Application constants

/// API version
pub const API_VERSION: &str = "v1";

/// Default pagination size
pub const DEFAULT_PAGE_SIZE: u32 = 20;

/// Maximum pagination size
pub const MAX_PAGE_SIZE: u32 = 100;

/// Default JWT expiration time
pub const DEFAULT_JWT_EXPIRATION: &str = "24h";

/// Sandbox default timeout in seconds
pub const DEFAULT_SANDBOX_TIMEOUT: u32 = 30;

/// Sandbox default memory limit in MB
pub const DEFAULT_SANDBOX_MEMORY: u32 = 256;
