//! Authenticated GraphQL viewer read model.
//!
//! Repository access uses the established Evolith user repository; GraphQL is only the
//! transport adapter. Never return password hashes, reset/verification tokens or cookies.

use std::sync::Arc;

use async_graphql::{Context, Error, ErrorExtensions, SimpleObject};
use domain::{
    repository::UserRepository,
    user::{TenantRole, UserRole},
};

use super::context::GraphQLRequestContext;

#[derive(Debug, Clone, SimpleObject)]
pub struct GraphQLViewer {
    pub id: String,
    pub email: String,
    pub username: String,
    pub role: String,
    pub tenant_id: String,
    pub tenant_role: String,
    pub email_verified: bool,
}

fn auth_error(code: &'static str, message: &'static str) -> Error {
    Error::new(message).extend_with(|_, extensions| extensions.set("code", code))
}

pub async fn viewer(ctx: &Context<'_>) -> async_graphql::Result<GraphQLViewer> {
    let principal = ctx
        .data::<GraphQLRequestContext>()?
        .principal
        .as_ref()
        .ok_or_else(|| auth_error("UNAUTHORIZED", "Authentication required"))?;

    let repo = ctx.data::<Arc<dyn UserRepository>>()?;
    let user = repo
        .find_by_id(principal.user_id)
        .await
        .map_err(|error| {
            tracing::error!("GraphQL viewer repository lookup failed: {}", error);
            auth_error("INTERNAL_ERROR", "Failed to fetch user information")
        })?
        .ok_or_else(|| auth_error("USER_NOT_FOUND", "User not found"))?;

    // Server-issued claims may be stale after a tenant change. Do not use stale claims
    // to disclose another tenant's persisted profile or to override database truth.
    if user.tenant_id != principal.tenant_id {
        return Err(auth_error("UNAUTHORIZED", "Authentication required"));
    }

    Ok(GraphQLViewer {
        id: user.id.to_string(),
        email: user.email,
        username: user.username,
        role: match user.role {
            UserRole::Admin => "admin",
            UserRole::User => "user",
        }
        .to_owned(),
        tenant_id: user.tenant_id.to_string(),
        tenant_role: match user.tenant_role {
            TenantRole::Owner => "owner",
            TenantRole::Admin => "admin",
            TenantRole::Member => "member",
        }
        .to_owned(),
        email_verified: user.email_verified,
    })
}
