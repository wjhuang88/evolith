use actix_web::{
    cookie::Cookie,
    http::{header, StatusCode},
    test, App,
};
use api::{
    graphql,
    middleware::{CsrfMiddleware, CSRF_COOKIE_NAME, CSRF_HEADER_NAME, JWT_COOKIE_NAME},
};
use serde_json::json;

fn foundation_query() -> serde_json::Value {
    json!({
        "query": "{ graphqlFoundation { service transport version } requestId }"
    })
}

#[actix_rt::test]
async fn graphql_cookie_session_without_csrf_fails_closed() {
    let app = test::init_service(
        App::new()
            .wrap(CsrfMiddleware::new())
            .configure(graphql::configure),
    )
    .await;
    let request = test::TestRequest::post()
        .uri("/graphql")
        .cookie(Cookie::new(JWT_COOKIE_NAME, "session-token"))
        .set_json(foundation_query())
        .to_request();

    let error = test::try_call_service(&app, request)
        .await
        .expect_err("cookie-authenticated GraphQL POST must require CSRF");

    assert_eq!(
        error.as_response_error().status_code(),
        StatusCode::FORBIDDEN
    );
}

#[actix_rt::test]
async fn graphql_cookie_session_with_double_submit_csrf_passes_transport_gate() {
    let app = test::init_service(
        App::new()
            .wrap(CsrfMiddleware::new())
            .configure(graphql::configure),
    )
    .await;
    let request = test::TestRequest::post()
        .uri("/graphql")
        .cookie(Cookie::new(JWT_COOKIE_NAME, "session-token"))
        .cookie(Cookie::new(CSRF_COOKIE_NAME, "csrf-value"))
        .insert_header((CSRF_HEADER_NAME, "csrf-value"))
        .set_json(foundation_query())
        .to_request();

    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), StatusCode::OK);
}

#[actix_rt::test]
async fn graphql_bearer_only_request_is_not_blocked_by_browser_csrf() {
    let app = test::init_service(
        App::new()
            .wrap(CsrfMiddleware::new())
            .configure(graphql::configure),
    )
    .await;
    let request = test::TestRequest::post()
        .uri("/graphql")
        .insert_header((header::AUTHORIZATION, "Bearer api-client-token"))
        .set_json(foundation_query())
        .to_request();

    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), StatusCode::OK);
}
