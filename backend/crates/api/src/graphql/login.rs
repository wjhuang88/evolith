//! GraphQL login transport adapter. Credential verification happens only in service-auth.

use std::sync::{Arc, Mutex};

use async_graphql::{Context, Error, ErrorExtensions, SimpleObject};
use service_auth::{LoginError, LoginService};
use validator::Validate;

use crate::dto::auth_dto::LoginRequest;

pub struct LoginCookieIntent {
    pub token: String,
    pub expires_at: i64,
}

/// One request's pending browser cookie update, committed by the Actix HTTP transport.
///
/// GraphQL execution and cookie writes are separate: a failed operation never sets cookies.
#[derive(Default)]
pub struct LoginCookieIntentStore(Mutex<Option<LoginCookieIntent>>);

impl LoginCookieIntentStore {
    pub fn schedule(&self, token: String, expires_at: i64) -> async_graphql::Result<()> {
        let mut pending = self
            .0
            .lock()
            .map_err(|_| auth_error("INTERNAL_ERROR", "Login failed"))?;
        if pending.is_some() {
            return Err(auth_error(
                "LOGIN_ALREADY_ISSUED",
                "Only one session may be issued per request",
            ));
        }
        *pending = Some(LoginCookieIntent { token, expires_at });
        Ok(())
    }

    pub fn take(&self) -> Option<LoginCookieIntent> {
        self.0.lock().ok()?.take()
    }
}

fn auth_error(code: &'static str, message: &'static str) -> Error {
    Error::new(message).extend_with(|_, extensions| extensions.set("code", code))
}

#[derive(Debug, Clone, SimpleObject)]
pub struct GraphQLLoginUser {
    pub id: String,
    pub email: String,
    pub username: String,
    pub role: String,
    pub tenant_id: String,
    pub tenant_role: String,
    pub email_verified: bool,
}

#[derive(Debug, Clone, SimpleObject)]
pub struct GraphQLLoginTenant {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub plan: String,
}

#[derive(Debug, Clone, SimpleObject)]
pub struct GraphQLLoginPayload {
    /// JWT for compatibility with the current browser auth consumer.
    pub token: String,
    /// Unix expiry timestamp in seconds, encoded as String to avoid GraphQL Int32 overflow.
    pub expires_at: String,
    pub user: GraphQLLoginUser,
    pub tenant: GraphQLLoginTenant,
}

pub async fn login(
    ctx: &Context<'_>,
    email: String,
    password: String,
) -> async_graphql::Result<GraphQLLoginPayload> {
    let input = LoginRequest { email, password };
    input
        .validate()
        .map_err(|_| auth_error("VALIDATION_ERROR", "Invalid login input"))?;

    let service = ctx.data::<LoginService>()?;
    let outcome = service
        .authenticate(&input.email, &input.password)
        .await
        .map_err(|error| match error {
            LoginError::InvalidCredentials => {
                auth_error("INVALID_CREDENTIALS", "Invalid email or password")
            }
            LoginError::Internal => auth_error("INTERNAL_ERROR", "Login failed"),
        })?;

    // The resolver stages an intent, never directly manipulates an HTTP response.
    ctx.data::<Arc<LoginCookieIntentStore>>()?
        .schedule(outcome.token.clone(), outcome.expires_at)?;

    Ok(GraphQLLoginPayload {
        token: outcome.token,
        expires_at: outcome.expires_at.to_string(),
        user: GraphQLLoginUser {
            id: outcome.user.id,
            email: outcome.user.email,
            username: outcome.user.username,
            role: outcome.user.role,
            tenant_id: outcome.user.tenant_id,
            tenant_role: outcome.user.tenant_role,
            email_verified: outcome.user.email_verified,
        },
        tenant: GraphQLLoginTenant {
            id: outcome.tenant.id,
            name: outcome.tenant.name,
            slug: outcome.tenant.slug,
            plan: outcome.tenant.plan,
        },
    })
}
