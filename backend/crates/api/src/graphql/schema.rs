use async_graphql::{EmptySubscription, Schema};

use super::{mutation::MutationRoot, query::QueryRoot};

pub const GRAPHQL_MAX_DEPTH: usize = 15;
pub const GRAPHQL_MAX_COMPLEXITY: usize = 1_000;
pub const GRAPHQL_MAX_DIRECTIVES: usize = 20;
pub const GRAPHQL_MAX_BODY_BYTES: usize = 256 * 1024;

pub type EvolithSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn build_schema() -> EvolithSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .limit_depth(GRAPHQL_MAX_DEPTH)
        .limit_complexity(GRAPHQL_MAX_COMPLEXITY)
        .limit_directives(GRAPHQL_MAX_DIRECTIVES)
        .finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphql::context::GraphQLRequestContext;
    use async_graphql::{Object, Request};

    fn context() -> GraphQLRequestContext {
        GraphQLRequestContext {
            request_id: "schema-test".to_string(),
            principal: None,
        }
    }

    #[actix_rt::test]
    async fn foundation_query_is_available() {
        let response = build_schema()
            .execute(
                Request::new(
                    "{ graphqlFoundation { service transport version } requestId }",
                )
                .data(context()),
            )
            .await;

        assert!(response.errors.is_empty(), "{:?}", response.errors);
    }

    #[actix_rt::test]
    async fn production_schema_rejects_excessive_complexity() {
        let selections = (0..=GRAPHQL_MAX_COMPLEXITY)
            .map(|index| format!("field{index}: graphqlFoundation {{ service }}"))
            .collect::<Vec<_>>()
            .join(" ");
        let response = build_schema()
            .execute(Request::new(format!("{{ {selections} }}")).data(context()))
            .await;

        assert!(!response.errors.is_empty());
    }

    struct DepthNode;

    #[Object]
    impl DepthNode {
        async fn child(&self) -> DepthNode {
            DepthNode
        }

        async fn value(&self) -> bool {
            true
        }
    }

    struct DepthQuery;

    #[Object]
    impl DepthQuery {
        async fn root(&self) -> DepthNode {
            DepthNode
        }
    }

    #[actix_rt::test]
    async fn configured_depth_limit_rejects_deep_queries() {
        let schema = Schema::build(DepthQuery, MutationRoot, EmptySubscription)
            .limit_depth(GRAPHQL_MAX_DEPTH)
            .finish();
        let mut selection = "value".to_string();
        for _ in 0..=GRAPHQL_MAX_DEPTH {
            selection = format!("child {{ {selection} }}");
        }
        let response = schema
            .execute(Request::new(format!("{{ root {{ {selection} }} }}")))
            .await;

        assert!(!response.errors.is_empty());
    }
}
