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
    use crate::middleware::rbac::CurrentUser;
    use async_graphql::{Object, Request};
    use domain::user::TenantRole;

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
                Request::new("{ graphqlFoundation { service transport version } requestId }")
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

    #[actix_rt::test]
    async fn current_session_is_null_for_anonymous_callers() {
        let response = build_schema()
            .execute(
                Request::new("{ currentSession { userId tenantId role tenantRole } }")
                    .data(context()),
            )
            .await;

        assert!(response.errors.is_empty(), "{:?}", response.errors);
        let data = response
            .data
            .into_json()
            .expect("serializable GraphQL data");
        assert!(data["currentSession"].is_null());
    }

    #[actix_rt::test]
    async fn current_session_reads_only_the_authenticated_principal() {
        let user_id =
            uuid::Uuid::parse_str("11111111-1111-1111-1111-111111111111").expect("valid user ID");
        let tenant_id =
            uuid::Uuid::parse_str("22222222-2222-2222-2222-222222222222").expect("valid tenant ID");
        let mut request_context = context();
        request_context.principal = Some(CurrentUser {
            user_id,
            tenant_id,
            role: "user".to_string(),
            tenant_role: TenantRole::Owner,
        });

        let response = build_schema()
            .execute(
                Request::new("{ currentSession { userId tenantId role tenantRole } }")
                    .data(request_context),
            )
            .await;

        assert!(response.errors.is_empty(), "{:?}", response.errors);
        let data = response
            .data
            .into_json()
            .expect("serializable GraphQL data");
        assert_eq!(data["currentSession"]["userId"], user_id.to_string());
        assert_eq!(data["currentSession"]["tenantId"], tenant_id.to_string());
        assert_eq!(data["currentSession"]["role"], "user");
        assert_eq!(data["currentSession"]["tenantRole"], "owner");
    }

    #[actix_rt::test]
    async fn session_schema_does_not_expose_credentials() {
        let response = build_schema()
            .execute(Request::new("{ currentSession { token } }").data(context()))
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
