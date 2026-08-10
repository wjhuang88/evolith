//! Caller-aware rate limiting for anonymous, JWT, and API-key requests.

use std::collections::HashMap;
use std::hash::Hash;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::http::StatusCode;
use actix_web::middleware::Next;
use actix_web::{web, Error, HttpMessage, HttpResponse, ResponseError};
use domain::api_key::ApiKey;
use infra::config::RateLimitConfig;
use serde::Serialize;
use uuid::Uuid;

use crate::middleware::rbac::CurrentUser;

const MINUTE: Duration = Duration::from_secs(60);
const HOUR: Duration = Duration::from_secs(60 * 60);
const MAX_ACTIVE_BUCKETS: usize = 100_000;

#[derive(Debug, Clone, PartialEq, Eq)]
enum CallerIdentity {
    Anonymous(Option<IpAddr>),
    Jwt(Uuid),
    ApiKey { id: Uuid, hourly_limit: u32 },
}

impl CallerIdentity {
    fn from_request(req: &ServiceRequest) -> Self {
        if let Some(api_key) = req.extensions().get::<ApiKey>() {
            return Self::ApiKey {
                id: api_key.id,
                hourly_limit: api_key.rate_limit.max(1),
            };
        }
        if let Some(user) = req.extensions().get::<CurrentUser>() {
            return Self::Jwt(user.user_id);
        }
        Self::Anonymous(req.peer_addr().map(|address| address.ip()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum BucketKey {
    Anonymous(Option<IpAddr>),
    Jwt(Uuid),
    ApiKeyMinute(Uuid),
    ApiKeyHour(Uuid),
}

#[derive(Debug, Clone, Copy)]
struct BucketSpec {
    key: BucketKey,
    limit: u32,
    window: Duration,
}

#[derive(Debug, Clone, Copy)]
struct Bucket {
    started_at: Instant,
    count: u32,
    window: Duration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RateLimitExceeded {
    retry_after_seconds: u64,
}

#[derive(Debug, Serialize)]
struct RateLimitErrorBody {
    success: bool,
    error: RateLimitError,
}

#[derive(Debug, Serialize)]
struct RateLimitError {
    code: &'static str,
    message: &'static str,
}

impl std::fmt::Display for RateLimitExceeded {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Rate limit exceeded")
    }
}

impl ResponseError for RateLimitExceeded {
    fn status_code(&self) -> StatusCode {
        StatusCode::TOO_MANY_REQUESTS
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::TooManyRequests()
            .insert_header(("Retry-After", self.retry_after_seconds.to_string()))
            .json(RateLimitErrorBody {
                success: false,
                error: RateLimitError {
                    code: "RATE_LIMIT_EXCEEDED",
                    message: "Rate limit exceeded",
                },
            })
    }
}

#[derive(Clone)]
pub struct CallerRateLimiter {
    config: RateLimitConfig,
    buckets: Arc<Mutex<HashMap<BucketKey, Bucket>>>,
    minute_window: Duration,
    api_key_window: Duration,
}

impl CallerRateLimiter {
    pub fn new(config: RateLimitConfig) -> Self {
        Self::with_windows(config, MINUTE, HOUR)
    }

    fn with_windows(
        config: RateLimitConfig,
        minute_window: Duration,
        api_key_window: Duration,
    ) -> Self {
        Self {
            config,
            buckets: Arc::new(Mutex::new(HashMap::new())),
            minute_window,
            api_key_window,
        }
    }

    fn check_request(&self, req: &ServiceRequest) -> Result<(), RateLimitExceeded> {
        self.check_at(CallerIdentity::from_request(req), Instant::now())
    }

    fn check_at(&self, caller: CallerIdentity, now: Instant) -> Result<(), RateLimitExceeded> {
        let specs = self.bucket_specs(caller);
        let mut buckets = self
            .buckets
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);

        buckets.retain(|_, bucket| now.duration_since(bucket.started_at) < bucket.window);

        let new_bucket_count = specs
            .iter()
            .filter(|spec| !buckets.contains_key(&spec.key))
            .count();
        if buckets.len().saturating_add(new_bucket_count) > MAX_ACTIVE_BUCKETS {
            return Err(RateLimitExceeded {
                retry_after_seconds: 1,
            });
        }

        for spec in &specs {
            if let Some(bucket) = buckets.get(&spec.key) {
                if bucket.count >= spec.limit {
                    return Err(RateLimitExceeded {
                        retry_after_seconds: remaining_seconds(now, bucket),
                    });
                }
            }
        }

        for spec in specs {
            let bucket = buckets.entry(spec.key).or_insert(Bucket {
                started_at: now,
                count: 0,
                window: spec.window,
            });
            bucket.count = bucket.count.saturating_add(1);
        }
        Ok(())
    }

    fn bucket_specs(&self, caller: CallerIdentity) -> Vec<BucketSpec> {
        match caller {
            CallerIdentity::Anonymous(ip) => vec![BucketSpec {
                key: BucketKey::Anonymous(ip),
                limit: self.config.unauthenticated_rpm.max(1),
                window: self.minute_window,
            }],
            CallerIdentity::Jwt(user_id) => vec![BucketSpec {
                key: BucketKey::Jwt(user_id),
                limit: self.config.authenticated_rpm.max(1),
                window: self.minute_window,
            }],
            CallerIdentity::ApiKey { id, hourly_limit } => vec![
                BucketSpec {
                    key: BucketKey::ApiKeyMinute(id),
                    limit: self.config.api_key_rpm.max(1),
                    window: self.minute_window,
                },
                BucketSpec {
                    key: BucketKey::ApiKeyHour(id),
                    limit: hourly_limit.max(1),
                    window: self.api_key_window,
                },
            ],
        }
    }
}

fn remaining_seconds(now: Instant, bucket: &Bucket) -> u64 {
    bucket
        .window
        .saturating_sub(now.duration_since(bucket.started_at))
        .as_secs()
        .max(1)
}

pub async fn caller_rate_limit_middleware<B>(
    req: ServiceRequest,
    next: Next<B>,
) -> Result<ServiceResponse<B>, Error> {
    let limiter = req
        .app_data::<web::Data<CallerRateLimiter>>()
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("Rate limiter unavailable"))?;
    limiter.check_request(&req).map_err(Error::from)?;
    next.call(req).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test::TestRequest;
    use chrono::Utc;
    use domain::api_key::ApiKeyStatus;
    use domain::user::TenantRole;
    use std::net::{Ipv4Addr, SocketAddr};

    fn config(anonymous: u32, jwt: u32, api_key: u32) -> RateLimitConfig {
        RateLimitConfig {
            unauthenticated_rpm: anonymous,
            authenticated_rpm: jwt,
            api_key_rpm: api_key,
        }
    }

    fn current_user(user_id: Uuid) -> CurrentUser {
        CurrentUser {
            user_id,
            role: "user".to_string(),
            tenant_id: Uuid::new_v4(),
            tenant_role: TenantRole::Member,
        }
    }

    fn api_key(id: Uuid, rate_limit: u32) -> ApiKey {
        ApiKey {
            id,
            tenant_id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            name: "limited".to_string(),
            key_hash: "hash".to_string(),
            key_prefix: "evo_sk_test".to_string(),
            permissions: vec!["read".to_string()],
            status: ApiKeyStatus::Active,
            rate_limit,
            request_count: 0,
            last_used_at: None,
            expires_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn same_ip_callers_use_isolated_buckets() {
        let limiter = CallerRateLimiter::new(config(1, 1, 10));
        let peer = SocketAddr::from((Ipv4Addr::LOCALHOST, 4000));

        let anonymous = TestRequest::default().peer_addr(peer).to_srv_request();
        assert!(limiter.check_request(&anonymous).is_ok());
        assert!(limiter.check_request(&anonymous).is_err());

        let jwt_user = Uuid::new_v4();
        let jwt = TestRequest::default().peer_addr(peer).to_srv_request();
        jwt.extensions_mut().insert(current_user(jwt_user));
        assert!(limiter.check_request(&jwt).is_ok());
        assert!(limiter.check_request(&jwt).is_err());

        let second_jwt = TestRequest::default().peer_addr(peer).to_srv_request();
        second_jwt
            .extensions_mut()
            .insert(current_user(Uuid::new_v4()));
        assert!(limiter.check_request(&second_jwt).is_ok());

        let key_request = TestRequest::default().peer_addr(peer).to_srv_request();
        key_request
            .extensions_mut()
            .insert(api_key(Uuid::new_v4(), 5));
        assert!(limiter.check_request(&key_request).is_ok());
    }

    #[test]
    fn api_keys_on_same_ip_are_isolated_and_honor_per_key_hourly_limit() {
        let limiter = CallerRateLimiter::new(config(30, 300, 1000));
        let peer = SocketAddr::from((Ipv4Addr::LOCALHOST, 4000));

        let first = TestRequest::default().peer_addr(peer).to_srv_request();
        first.extensions_mut().insert(api_key(Uuid::new_v4(), 2));
        assert!(limiter.check_request(&first).is_ok());
        assert!(limiter.check_request(&first).is_ok());
        assert!(limiter.check_request(&first).is_err());

        let second = TestRequest::default().peer_addr(peer).to_srv_request();
        second.extensions_mut().insert(api_key(Uuid::new_v4(), 2));
        assert!(limiter.check_request(&second).is_ok());
    }

    #[test]
    fn api_key_global_minute_limit_is_enforced_independently() {
        let limiter = CallerRateLimiter::new(config(30, 300, 2));
        let caller = CallerIdentity::ApiKey {
            id: Uuid::new_v4(),
            hourly_limit: 100,
        };
        let now = Instant::now();

        assert!(limiter.check_at(caller.clone(), now).is_ok());
        assert!(limiter.check_at(caller.clone(), now).is_ok());
        assert!(limiter.check_at(caller, now).is_err());
    }

    #[test]
    fn expired_window_recovers_caller_quota() {
        let limiter = CallerRateLimiter::with_windows(
            config(1, 1, 1),
            Duration::from_secs(1),
            Duration::from_secs(2),
        );
        let caller = CallerIdentity::Jwt(Uuid::new_v4());
        let now = Instant::now();

        assert!(limiter.check_at(caller.clone(), now).is_ok());
        assert!(limiter.check_at(caller.clone(), now).is_err());
        assert!(limiter
            .check_at(caller, now + Duration::from_secs(1))
            .is_ok());
    }

    #[test]
    fn rate_limit_error_is_structured_and_includes_retry_after() {
        let response = RateLimitExceeded {
            retry_after_seconds: 12,
        }
        .error_response();
        assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
        assert_eq!(response.headers().get("Retry-After").unwrap(), "12");
    }
}
