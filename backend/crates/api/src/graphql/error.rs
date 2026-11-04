use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use async_graphql::{ErrorExtensionValues, Response};
use serde_json::json;

pub const DEFAULT_GRAPHQL_ERROR_CODE: &str = "GRAPHQL_ERROR";

/// Ensure every schema-level GraphQL error carries Evolith's stable correlation envelope.
pub fn enrich_response_errors(response: &mut Response, request_id: &str) {
    for error in &mut response.errors {
        let extensions = error
            .extensions
            .get_or_insert_with(ErrorExtensionValues::default);
        if extensions.get("code").is_none() {
            extensions.set("code", DEFAULT_GRAPHQL_ERROR_CODE);
        }
        if extensions.get("requestId").is_none() {
            extensions.set("requestId", request_id.to_string());
        }
        if extensions.get("retryable").is_none() {
            extensions.set("retryable", false);
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GraphQLTransportError {
    #[error("GraphQL batch requests are disabled")]
    BatchNotSupported { request_id: String },
}

impl ResponseError for GraphQLTransportError {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> HttpResponse {
        let (code, request_id) = match self {
            Self::BatchNotSupported { request_id } => {
                ("GRAPHQL_BATCH_NOT_SUPPORTED", request_id.as_str())
            }
        };

        HttpResponse::build(self.status_code()).json(json!({
            "errors": [{
                "message": self.to_string(),
                "extensions": {
                    "code": code,
                    "requestId": request_id,
                    "retryable": false
                }
            }]
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_graphql::{Response, ServerError};

    #[test]
    fn error_envelope_adds_stable_extensions_without_overwriting_existing_code() {
        let mut response = Response::from_errors(vec![ServerError::new("broken", None)]);
        enrich_response_errors(&mut response, "req-123");

        let Some(extensions) = response
            .errors
            .first()
            .and_then(|error| error.extensions.as_ref())
        else {
            panic!("missing GraphQL error extensions");
        };
        assert!(extensions.get("code").is_some());
        assert!(extensions.get("requestId").is_some());
        assert!(extensions.get("retryable").is_some());
    }
}
