//! Password hashing and verification utilities
//!
//! Uses Argon2id algorithm for secure password hashing.

use argon2::{
    password_hash::SaltString, Algorithm, Argon2, Params, PasswordHash, PasswordHasher,
    PasswordVerifier, Version,
};
use common::error::Result;
use rand::rngs::OsRng;
use thiserror::Error;

/// Password-related errors
#[derive(Debug, Error)]
pub enum PasswordError {
    #[error("Password hashing failed: {0}")]
    HashFailed(String),

    #[error("Password verification failed: {0}")]
    VerifyFailed(String),

    #[error("Invalid password hash format")]
    InvalidHashFormat,

    #[error("Password does not meet requirements")]
    InvalidPassword,
}

impl From<PasswordError> for common::error::AppError {
    fn from(err: PasswordError) -> Self {
        common::error::AppError::InternalError(err.to_string())
    }
}

/// Password hasher using Argon2id algorithm
///
/// Argon2id is the recommended algorithm for password hashing as of 2024,
/// providing resistance against GPU cracking attacks and side-channel attacks.
#[derive(Clone)]
pub struct Argon2Hasher {
    hasher: Argon2<'static>,
}

impl Default for Argon2Hasher {
    fn default() -> Self {
        Self::new()
    }
}

impl Argon2Hasher {
    /// Create a new password hasher with secure defaults
    ///
    /// Default parameters:
    /// - Memory cost: 64 MB (65536 KiB)
    /// - Time cost: 3 iterations
    /// - Parallelism: 4 threads
    /// - Output length: 32 bytes
    pub fn new() -> Self {
        let params = Params::new(
            65536,    // m_cost: 64MB memory
            3,        // t_cost: 3 iterations
            4,        // p_cost: 4 threads parallelism
            Some(32), // output length
        )
        .expect("Failed to create Argon2 params");

        let hasher = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

        Self { hasher }
    }

    /// Create a password hasher with custom parameters
    ///
    /// # Arguments
    /// * `memory_cost` - Memory cost in KiB (default: 65536 = 64MB)
    /// * `time_cost` - Number of iterations (default: 3)
    /// * `parallelism` - Degree of parallelism (default: 4)
    pub fn with_params(memory_cost: u32, time_cost: u32, parallelism: u32) -> Result<Self> {
        let params = Params::new(memory_cost, time_cost, parallelism, Some(32))
            .map_err(|e| PasswordError::HashFailed(e.to_string()))?;

        let hasher = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

        Ok(Self { hasher })
    }

    /// Hash a password using Argon2id
    ///
    /// Returns a PHC (Password Hashing Competition) format string:
    /// `$argon2id$v=19$m=65536,t=3,p=4$<salt>$<hash>`
    ///
    /// # Arguments
    /// * `password` - The plaintext password to hash
    ///
    /// # Example
    /// ```ignore
    /// let hasher = Argon2Hasher::new();
    /// ```
    /// let hasher = Argon2Hasher::new();
    /// let hash = hasher.hash_password("my_secure_password")?;
    /// ```
    pub fn hash_password(&self, password: &str) -> Result<String> {
        // Generate a random salt
        let salt = self.generate_salt();

        // Hash the password
        let hash = self
            .hasher
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| PasswordError::HashFailed(e.to_string()))?;

        Ok(hash.to_string())
    }

    /// Verify a password against a stored hash
    ///
    /// # Arguments
    /// * `password` - The plaintext password to verify
    /// * `hash` - The stored PHC format hash string
    ///
    /// # Returns
    /// * `Ok(true)` - Password matches
    /// * `Ok(false)` - Password does not match
    /// * `Err` - Hash format is invalid or verification failed
    ///
    /// # Example
    /// ```ignore
    /// let hasher = Argon2Hasher::new();
    /// let hash = hasher.hash_password("password")?;
    /// assert!(hasher.verify_password("password", &hash)?);
    /// assert!(!hasher.verify_password("wrong_password", &hash)?);
    /// ```
    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash).map_err(|_| PasswordError::InvalidHashFormat)?;

        match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Generate a cryptographically secure random salt
    ///
    /// Returns a base64-encoded salt string suitable for Argon2
    fn generate_salt(&self) -> SaltString {
        SaltString::generate(&mut OsRng)
    }

    /// Validate password strength
    ///
    /// Requirements:
    /// - Minimum 8 characters
    /// - Maximum 128 characters
    /// - Contains at least one lowercase letter
    /// - Contains at least one uppercase letter
    /// - Contains at least one digit
    /// - Contains at least one special character
    pub fn validate_strength(&self, password: &str) -> Result<()> {
        if password.len() < 8 {
            return Err(PasswordError::InvalidPassword.into());
        }

        if password.len() > 128 {
            return Err(PasswordError::InvalidPassword.into());
        }

        let has_lowercase = password.chars().any(|c| c.is_lowercase());
        let has_uppercase = password.chars().any(|c| c.is_uppercase());
        let has_digit = password.chars().any(|c| c.is_numeric());
        let has_special = password.chars().any(|c| !c.is_alphanumeric());

        if !(has_lowercase && has_uppercase && has_digit && has_special) {
            return Err(PasswordError::InvalidPassword.into());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify_password() {
        let hasher = Argon2Hasher::new();
        let password = "TestPassword123!";

        let hash = hasher.hash_password(password).expect("Hashing failed");
        assert!(hash.starts_with("$argon2id$"));

        assert!(hasher
            .verify_password(password, &hash)
            .expect("Verification failed"));
        assert!(!hasher
            .verify_password("WrongPassword", &hash)
            .expect("Verification failed"));
    }

    #[test]
    fn test_password_validation() {
        let hasher = Argon2Hasher::new();

        // Valid passwords
        assert!(hasher.validate_strength("TestPassword123!").is_ok());
        assert!(hasher.validate_strength("ComplexP@ssw0rd").is_ok());

        // Invalid passwords
        assert!(hasher.validate_strength("short").is_err());
        assert!(hasher.validate_strength("noupper123!").is_err());
        assert!(hasher.validate_strength("NOLOWER123!").is_err());
        assert!(hasher.validate_strength("NoDigits!").is_err());
        assert!(hasher.validate_strength("NoSpecial123").is_err());
    }

    #[test]
    fn test_unique_hashes() {
        let hasher = Argon2Hasher::new();
        let password = "SamePassword123!";

        let hash1 = hasher.hash_password(password).expect("Hashing failed");
        let hash2 = hasher.hash_password(password).expect("Hashing failed");

        // Same password should produce different hashes due to random salt
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_password_length_boundaries() {
        let hasher = Argon2Hasher::new();

        // Min length is 8
        assert!(hasher.validate_strength("Abcdefg1!").is_ok()); // Exactly 8
        assert!(hasher.validate_strength("Abcdef12!").is_ok()); // 9 chars

        // Invalid: too long (> 128 chars)
        let too_long = "Aa1!".repeat(33); // 4 * 33 = 132 chars
        assert!(hasher.validate_strength(&too_long).is_err());
    }

    #[test]
    fn test_password_special_characters() {
        let hasher = Argon2Hasher::new();

        // Valid special characters
        assert!(hasher.validate_strength("PassWord1!").is_ok());
        assert!(hasher.validate_strength("PassWord1@").is_ok());
        assert!(hasher.validate_strength("PassWord1#").is_ok());
        assert!(hasher.validate_strength("PassWord1$").is_ok());
        assert!(hasher.validate_strength("PassWord1%").is_ok());
        assert!(hasher.validate_strength("PassWord1^").is_ok());
        assert!(hasher.validate_strength("PassWord1&").is_ok());
        assert!(hasher.validate_strength("PassWord1*").is_ok());
    }

    #[test]
    fn test_verify_invalid_hash_format() {
        let hasher = Argon2Hasher::new();

        // Invalid hash formats
        assert!(hasher.verify_password("test", "invalid-hash").is_err());
        assert!(hasher.verify_password("test", "").is_err());
    }

    #[test]
    fn test_empty_password() {
        let hasher = Argon2Hasher::new();

        // Empty password should fail validation
        let result = hasher.validate_strength("");
        assert!(result.is_err());
    }

    #[test]
    fn test_whitespace_password() {
        let hasher = Argon2Hasher::new();

        // Password with only whitespace should fail
        let result = hasher.validate_strength("    ");
        assert!(result.is_err());
    }

    #[test]
    fn test_unicode_password() {
        let hasher = Argon2Hasher::new();

        // Unicode characters are allowed in passwords
        let result = hasher.hash_password("密码123ABC!");
        assert!(result.is_ok());
    }

    #[test]
    fn test_password_with_spaces() {
        let hasher = Argon2Hasher::new();

        // Password with spaces should work (special char check allows non-alphanumeric)
        let result = hasher.validate_strength("Pass Word1!");
        assert!(result.is_ok());
    }
}
