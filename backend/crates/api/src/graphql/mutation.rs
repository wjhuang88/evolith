use async_graphql::{Context, Object};

#[derive(Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Foundation-only mutation used to keep a stable root while domain mutations migrate later.
    async fn graphql_foundation_noop(&self) -> bool {
        true
    }

    /// Authenticate with email and password and issue a browser session.
    async fn login(
        &self,
        ctx: &Context<'_>,
        email: String,
        password: String,
    ) -> async_graphql::Result<super::login::GraphQLLoginPayload> {
        super::login::login(ctx, email, password).await
    }
}
