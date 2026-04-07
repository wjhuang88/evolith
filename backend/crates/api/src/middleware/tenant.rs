//! Tenant middleware for multi-tenant support

use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use std::future::{ready, Ready};
use uuid::Uuid;

use domain::{TenantContext, TenantPlan, TenantStatus};

/// Extract tenant from request
/// Priority: subdomain > X-Tenant-ID header > default tenant
pub fn extract_tenant(req: &ServiceRequest) -> Result<TenantContext, Error> {
    // Try to get tenant from subdomain first
    if let Some(host) = req.headers().get("host").and_then(|h| h.to_str().ok()) {
        if let Some(slug) = extract_subdomain(host) {
            // In production, we would look up the tenant in the database
            // For now, return a mock tenant context
            return Ok(TenantContext {
                tenant_id: Uuid::new_v5(&Uuid::NAMESPACE_DNS, slug.as_bytes()),
                tenant_slug: slug.to_string(),
                tenant_name: format!("{} Tenant", slug),
                tenant_plan: TenantPlan::Pro,
                tenant_status: TenantStatus::Active,
            });
        }
    }

    // Try X-Tenant-ID header
    if let Some(tenant_id) = req
        .headers()
        .get("X-Tenant-ID")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| Uuid::parse_str(s).ok())
    {
        return Ok(TenantContext {
            tenant_id,
            tenant_slug: format!("tenant-{}", tenant_id),
            tenant_name: "Custom Tenant".to_string(),
            tenant_plan: TenantPlan::Pro,
            tenant_status: TenantStatus::Active,
        });
    }

    // Try X-Tenant-Slug header
    if let Some(slug) = req
        .headers()
        .get("X-Tenant-Slug")
        .and_then(|h| h.to_str().ok())
    {
        return Ok(TenantContext {
            tenant_id: Uuid::new_v5(&Uuid::NAMESPACE_DNS, slug.as_bytes()),
            tenant_slug: slug.to_string(),
            tenant_name: format!("{} Tenant", slug),
            tenant_plan: TenantPlan::Pro,
            tenant_status: TenantStatus::Active,
        });
    }

    // Return default tenant for development
    Ok(TenantContext::default_tenant())
}

/// Extract subdomain from host
fn extract_subdomain(host: &str) -> Option<String> {
    // Remove port if present
    let host = host.split(':').next()?;

    // Skip localhost and IP addresses
    if host == "localhost" || host.parse::<std::net::IpAddr>().is_ok() {
        return None;
    }

    // Extract subdomain from evolith.io domains
    if host.ends_with(".evolith.io") {
        let subdomain = host.strip_suffix(".evolith.io").unwrap_or(host);
        // Skip 'app' and 'www' subdomains
        if subdomain != "app" && subdomain != "www" && !subdomain.is_empty() {
            return Some(subdomain.to_string());
        }
    }

    None
}

/// Actix-web middleware for tenant extraction
pub struct TenantMiddleware;

impl<S> actix_web::dev::Transform<S, actix_web::dev::ServiceRequest> for TenantMiddleware
where
    S: actix_web::dev::Service<
        actix_web::dev::ServiceRequest,
        Response = actix_web::dev::ServiceResponse<actix_web::body::BoxBody>,
        Error = Error,
    >,
    S::Future: 'static,
{
    type Response = actix_web::dev::ServiceResponse<actix_web::body::BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = TenantMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(TenantMiddlewareService { service }))
    }
}

pub struct TenantMiddlewareService<S> {
    service: S,
}

impl<S> actix_web::dev::Service<actix_web::dev::ServiceRequest> for TenantMiddlewareService<S>
where
    S: actix_web::dev::Service<
        actix_web::dev::ServiceRequest,
        Response = actix_web::dev::ServiceResponse<actix_web::body::BoxBody>,
        Error = Error,
    >,
    S::Future: 'static,
{
    type Response = actix_web::dev::ServiceResponse<actix_web::body::BoxBody>;
    type Error = Error;
    type Future =
        std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: actix_web::dev::ServiceRequest) -> Self::Future {
        // Extract tenant context
        match extract_tenant(&req) {
            Ok(tenant_ctx) => {
                // Insert tenant context into request extensions
                req.extensions_mut().insert(tenant_ctx);

                let fut = self.service.call(req);
                Box::pin(fut)
            }
            Err(e) => Box::pin(async move { Err(e) }),
        }
    }
}

/// Extension trait to get tenant context from request
pub trait TenantContextExt {
    fn get_tenant_context(&self) -> Option<TenantContext>;
}

impl TenantContextExt for actix_web::HttpRequest {
    fn get_tenant_context(&self) -> Option<TenantContext> {
        self.extensions().get::<TenantContext>().cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_subdomain() {
        assert_eq!(
            extract_subdomain("acme.evolith.io"),
            Some("acme".to_string())
        );
        assert_eq!(extract_subdomain("app.evolith.io"), None);
        assert_eq!(extract_subdomain("localhost:8080"), None);
        assert_eq!(extract_subdomain("127.0.0.1:8080"), None);
    }
}
