use async_graphql::Object;

#[derive(Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Foundation-only mutation used to keep a stable root while domain mutations migrate later.
    async fn graphql_foundation_noop(&self) -> bool {
        true
    }
}
