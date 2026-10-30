//! Optional authentication bridge for the canonical GraphQL application endpoint.
//!
//! `/graphql` contains both public and authenticated operations, so the global RBAC middleware
//! deliberately treats the route as public at the transport layer. This middleware still resolves
//! any credentials supplied by the caller before caller-aware rate limiting and schema execution.
//! Resolvers consume `CurrentUser` from request context and never parse JWT/API-key credentials.

use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    error::ErrorInternalServerError,
    middleware::Next,
    web, Error, HttpMessage,
};
use domain::user::TenantRole;

use crate::middleware::rbac::{extract_token, resolve_api_key, CurrentUser, RbacError};
use crate::state::AppState;

const GRAPHQL_PATH: &str = "/graphql";

fn tenant_role(role: &str) -> TenantRole {
    match role {
        "owner" => TenantRole::Owner,
        "admin" => TenantRole::Admin,
        _ => TenantRole::Member,
    }
}

/// Resolve optional GraphQL credentials before the caller-aware rate limiter runs.
///
/// Missing credentials are valid because the schema may expose public operations. Supplied but
/// invalid credentials fail closed instead of silently degrading to an anonymous caller.
pub async fn graphql_optional_auth_middleware<B>(
    req: ServiceRequest,
    next: Next<B>,
) -> Result<ServiceResponse<B>, Error> {
    if req.path() != GRAPHQL_PATH || req.extensions().get::<CurrentUser>().is_some() {
        return next.call(req).await;
    }

    let Some((kind, token)) = extract_token(&req) else {
        return next.call(req).await;
    };

    let state = req
        .app_data::<web::Data<AppState>>()
        .ok_or_else(|| ErrorInternalServerError("Application state unavailable"))?;

    if kind == "token" {
        let token_data = state
            .jwt
            .validate_token(&token)
            .map_err(|_| actix_web::Error::from(RbacError::Unauthorized))?;

        req.extensions_mut().insert(CurrentUser {
            user_id: token_data.user_id,
            role: token_data.role,
            tenant_id: token_data.tenant_id,
            tenant_role: tenant_role(&token_data.tenant_role),
        });
        return next.call(req).await;
    }

    let api_key = resolve_api_key(&token, &*state.api_key_repo)
        .await
        .map_err(|_| actix_web::Error::from(RbacError::Unauthorized))?;
    let user = CurrentUser {
        user_id: api_key.user_id,
        role: "api_key".to_string(),
        tenant_id: api_key.tenant_id,
        tenant_role: TenantRole::Member,
    };
    req.extensions_mut().insert(api_key);
    req.extensions_mut().insert(user);
    next.call(req).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::middleware::rbac::{default_public_paths, RbacMiddleware};

    #[test]
    fn graphql_stays_public_at_global_rbac_boundary() {
        assert!(RbacMiddleware::is_public_path(
            GRAPHQL_PATH,
            &default_public_paths()
        ));
    }

    #[test]
    fn unknown_tenant_roles_fail_down_to_member() {
        assert_eq!(tenant_role("owner"), TenantRole::Owner);
        assert_eq!(tenant_role("admin"), TenantRole::Admin);
        assert_eq!(tenant_role("member"), TenantRole::Member);
        assert_eq!(tenant_role("unexpected"), TenantRole::Member);
    }
}
