use async_graphql::{Context, Object, SimpleObject};
use domain::user::TenantRole;

use super::context::GraphQLRequestContext;

#[derive(Debug, Clone, SimpleObject)]
pub struct GraphQLFoundationInfo {
    pub service: String,
    pub transport: String,
    pub version: String,
}

/// Public representation of the resolved caller's session claims.
///
/// Never includes credentials or tokens. A null `currentSession` means the transport
/// supplied no authenticated principal, not that a credential was valid but expired.
#[derive(Debug, Clone, SimpleObject)]
pub struct GraphQLSessionInfo {
    pub user_id: String,
    pub tenant_id: String,
    pub role: String,
    pub tenant_role: String,
}

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Minimal application-API smoke field. Product domains migrate in later EVO-127 slices.
    async fn graphql_foundation(&self) -> GraphQLFoundationInfo {
        GraphQLFoundationInfo {
            service: "evolith".to_string(),
            transport: "graphql".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    /// Correlation identifier for the current HTTP request.
    async fn request_id(&self, ctx: &Context<'_>) -> async_graphql::Result<String> {
        Ok(ctx.data::<GraphQLRequestContext>()?.request_id.clone())
    }

    /// Authenticated caller claims, or null when no credentials were supplied.
    async fn current_session(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<Option<GraphQLSessionInfo>> {
        let request = ctx.data::<GraphQLRequestContext>()?;
        Ok(request.principal.as_ref().map(|principal| GraphQLSessionInfo {
            user_id: principal.user_id.to_string(),
            tenant_id: principal.tenant_id.to_string(),
            role: principal.role.clone(),
            tenant_role: match &principal.tenant_role {
                TenantRole::Owner => "owner",
                TenantRole::Admin => "admin",
                TenantRole::Member => "member",
            }
            .to_owned(),
        }))
    }
}
