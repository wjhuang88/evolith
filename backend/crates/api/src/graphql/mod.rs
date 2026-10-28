//! Canonical GraphQL application API foundation.
//!
//! GraphQL owns browser/application business operations. Protocol-native Git Smart HTTP, MCP,
//! health/readiness and callback endpoints remain outside this module.

mod context;
mod error;
mod mutation;
mod query;
mod schema;

use std::env;

use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use async_graphql::{http::GraphiQLSource, BatchRequest};
use async_graphql_actix_web::GraphQLResponse;

use crate::middleware::{CurrentUser, RequestIdExt};
use crate::state::AppState;

pub use context::GraphQLRequestContext;
pub use schema::{
    build_schema, EvolithSchema, GRAPHQL_MAX_BODY_BYTES, GRAPHQL_MAX_COMPLEXITY,
    GRAPHQL_MAX_DEPTH, GRAPHQL_MAX_DIRECTIVES,
};

use error::{enrich_response_errors, GraphQLTransportError};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/graphql")
            .app_data(web::Data::new(build_schema()))
            .app_data(web::JsonConfig::default().limit(GRAPHQL_MAX_BODY_BYTES))
            .route(web::post().to(graphql_post))
            .route(web::get().to(graphiql)),
    );
}

async fn graphql_post(
    schema: web::Data<EvolithSchema>,
    http_request: HttpRequest,
    payload: web::Json<BatchRequest>,
) -> Result<GraphQLResponse, GraphQLTransportError> {
    let request_id = http_request
        .get_request_id()
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    let principal = http_request.extensions().get::<CurrentUser>().cloned();
    let request_context = GraphQLRequestContext {
        request_id: request_id.clone(),
        principal,
    };

    let request = payload
        .into_inner()
        .into_single()
        .map_err(|_| GraphQLTransportError::BatchNotSupported {
            request_id: request_id.clone(),
        })?
        .data(request_context);

    let mut response = schema.execute(request).await;
    enrich_response_errors(&mut response, &request_id);
    Ok(response.into())
}

async fn graphiql(state: web::Data<AppState>) -> HttpResponse {
    if !graphiql_allowed(
        state.config.is_development(),
        env::var("GRAPHQL_GRAPHIQL_ENABLED").ok().as_deref(),
    ) {
        return HttpResponse::NotFound().finish();
    }

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/graphql").finish())
}

fn graphiql_allowed(is_development: bool, configured: Option<&str>) -> bool {
    is_development
        && configured.is_some_and(|value| {
            matches!(value.trim().to_ascii_lowercase().as_str(), "1" | "true" | "yes")
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::middleware::request_id::RequestIdMiddleware;
    use actix_web::{http::StatusCode, test, App};
    use serde_json::{json, Value};

    #[actix_rt::test]
    async fn http_smoke_query_returns_data_and_request_id() {
        let app = test::init_service(
            App::new()
                .wrap(RequestIdMiddleware::new())
                .configure(configure),
        )
        .await;
        let request = test::TestRequest::post()
            .uri("/graphql")
            .set_json(json!({
                "query": "{ graphqlFoundation { service transport version } requestId }"
            }))
            .to_request();
        let response = test::call_service(&app, request).await;
        assert_eq!(response.status(), StatusCode::OK);
        assert!(response.headers().contains_key("x-request-id"));
        let body: Value = test::read_body_json(response).await;
        assert_eq!(body["data"]["graphqlFoundation"]["service"], "evolith");
        assert!(body["data"]["requestId"].is_string());
    }

    #[actix_rt::test]
    async fn batch_requests_are_rejected() {
        let app = test::init_service(
            App::new()
                .wrap(RequestIdMiddleware::new())
                .configure(configure),
        )
        .await;
        let request = test::TestRequest::post()
            .uri("/graphql")
            .set_json(json!([{ "query": "{ graphqlFoundation { service } }" }]))
            .to_request();
        let response = test::call_service(&app, request).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body: Value = test::read_body_json(response).await;
        assert_eq!(
            body["errors"][0]["extensions"]["code"],
            "GRAPHQL_BATCH_NOT_SUPPORTED"
        );
    }

    #[actix_rt::test]
    async fn schema_errors_receive_stable_extensions() {
        let app = test::init_service(
            App::new()
                .wrap(RequestIdMiddleware::new())
                .configure(configure),
        )
        .await;
        let request = test::TestRequest::post()
            .uri("/graphql")
            .insert_header(("X-Request-ID", "known-request"))
            .set_json(json!({ "query": "{ missingField }" }))
            .to_request();
        let response = test::call_service(&app, request).await;
        assert_eq!(response.status(), StatusCode::OK);
        let body: Value = test::read_body_json(response).await;
        assert_eq!(body["errors"][0]["extensions"]["code"], "GRAPHQL_ERROR");
        assert_eq!(
            body["errors"][0]["extensions"]["requestId"],
            "known-request"
        );
        assert_eq!(body["errors"][0]["extensions"]["retryable"], false);
    }

    #[actix_rt::test]
    async fn graphiql_is_explicit_dev_only() {
        assert!(!graphiql_allowed(false, Some("true")));
        assert!(!graphiql_allowed(true, None));
        assert!(!graphiql_allowed(true, Some("false")));
        assert!(graphiql_allowed(true, Some("true")));
        assert!(graphiql_allowed(true, Some("1")));
    }
}
