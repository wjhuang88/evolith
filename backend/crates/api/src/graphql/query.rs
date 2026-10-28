use async_graphql::{Context, Object, SimpleObject};

use super::context::GraphQLRequestContext;

#[derive(Debug, Clone, SimpleObject)]
pub struct GraphQLFoundationInfo {
    pub service: String,
    pub transport: String,
    pub version: String,
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
}
