//! JWT handling for authentication

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use common::error::{AppError, Result};
use infra::config::JwtConfig;

/// Claims stored in JWT token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,
    /// User role
    pub role: String,
    /// Tenant ID
    pub tenant_id: String,
    /// Tenant role (owner, admin, member)
    pub tenant_role: String,
    /// Issued at
    pub iat: i64,
    /// Expiration time
    pub exp: i64,
}

/// JWT token data
#[derive(Debug, Clone)]
pub struct TokenData {
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub tenant_role: String,
    pub role: String,
    pub expires_at: i64,
}

/// JWT handler for token generation and validation
pub struct JwtHandler {
    secret: Vec<u8>,
    expiration: Duration,
}

impl JwtHandler {
    /// Create a new JWT handler
    pub fn new(config: &JwtConfig) -> Self {
        Self {
            secret: config.secret.as_bytes().to_vec(),
            expiration: parse_duration(&config.expiration),
        }
    }

    /// Generate a new JWT token for a user
    pub fn generate_token(
        &self,
        user_id: Uuid,
        role: &str,
        tenant_id: Uuid,
        tenant_role: &str,
    ) -> Result<(String, i64)> {
        let now = Utc::now();
        let exp = now + self.expiration;

        let claims = Claims {
            sub: user_id.to_string(),
            role: role.to_string(),
            tenant_id: tenant_id.to_string(),
            tenant_role: tenant_role.to_string(),
            iat: now.timestamp(),
            exp: exp.timestamp(),
        };

        let token = encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(&self.secret),
        )
        .map_err(|e| AppError::InternalError(format!("Failed to generate token: {}", e)))?;

        Ok((token, exp.timestamp()))
    }

    /// Validate a JWT token and extract claims
    pub fn validate_token(&self, token: &str) -> Result<TokenData> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(&self.secret),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|e| AppError::AuthenticationError(format!("Invalid token: {}", e)))?;

        let user_id = Uuid::parse_str(&token_data.claims.sub).map_err(|e| {
            AppError::AuthenticationError(format!("Invalid user ID in token: {}", e))
        })?;

        let tenant_id = Uuid::parse_str(&token_data.claims.tenant_id).unwrap_or(Uuid::nil());

        Ok(TokenData {
            user_id,
            tenant_id,
            tenant_role: token_data.claims.tenant_role,
            role: token_data.claims.role,
            expires_at: token_data.claims.exp,
        })
    }

    /// Check if a token is expired
    pub fn is_expired(&self, token_data: &TokenData) -> bool {
        let now = Utc::now().timestamp();
        now >= token_data.expires_at
    }
}

/// Parse duration string like "24h", "7d", "30m"
fn parse_duration(s: &str) -> Duration {
    let s = s.trim();
    if s.is_empty() {
        return Duration::hours(24);
    }

    let num: u64 = s[..s.len() - 1].parse().unwrap_or(24);

    match s.chars().last() {
        Some('h') => Duration::hours(num as i64),
        Some('d') => Duration::days(num as i64),
        Some('w') => Duration::weeks(num as i64),
        Some('m') => Duration::minutes(num as i64),
        Some('s') => Duration::seconds(num as i64),
        _ => Duration::hours(24),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_handler() -> JwtHandler {
        JwtHandler::new(&JwtConfig {
            secret: "test_secret_key_for_testing".to_string(),
            expiration: "1h".to_string(),
        })
    }

    #[test]
    fn test_generate_and_validate_token() {
        let handler = create_test_handler();
        let user_id = Uuid::new_v4();
        let role = "user";
        let tenant_id = Uuid::nil();
        let tenant_role = "owner";

        let (token, exp) = handler
            .generate_token(user_id, role, tenant_id, tenant_role)
            .unwrap();
        assert!(!token.is_empty());
        assert!(exp > Utc::now().timestamp());

        let token_data = handler.validate_token(&token).unwrap();
        assert_eq!(token_data.user_id, user_id);
        assert_eq!(token_data.role, role);
    }

    #[test]
    fn test_invalid_token() {
        let handler = create_test_handler();

        let result = handler.validate_token("invalid_token");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_duration() {
        assert_eq!(parse_duration("24h"), Duration::hours(24));
        assert_eq!(parse_duration("7d"), Duration::days(7));
        assert_eq!(parse_duration("30m"), Duration::minutes(30));
        assert_eq!(parse_duration("1w"), Duration::weeks(1));
    }
}
