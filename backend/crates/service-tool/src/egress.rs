//! Controlled outbound HTTP boundary for tenant-configured tools.

use std::collections::BTreeSet;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use futures_util::StreamExt;
use reqwest::header::LOCATION;
use reqwest::{Method, StatusCode, Url};
use tokio::sync::Semaphore;

pub const DEFAULT_MAX_RESPONSE_BYTES: usize = 1_048_576;
const DEFAULT_MAX_HEADER_BYTES: usize = 64 * 1024;
const DEFAULT_MAX_REDIRECTS: usize = 5;
const DEFAULT_MAX_CONCURRENCY: usize = 32;
const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Debug, Clone)]
pub struct EgressPolicy {
    allow_http: bool,
    allow_private_networks: bool,
    max_redirects: usize,
    max_response_bytes: usize,
    max_header_bytes: usize,
    max_concurrency: usize,
    connect_timeout: Duration,
}

impl Default for EgressPolicy {
    fn default() -> Self {
        Self {
            // HTTP remains supported for compatibility, but only to globally routable targets.
            allow_http: true,
            allow_private_networks: false,
            max_redirects: DEFAULT_MAX_REDIRECTS,
            max_response_bytes: DEFAULT_MAX_RESPONSE_BYTES,
            max_header_bytes: DEFAULT_MAX_HEADER_BYTES,
            max_concurrency: DEFAULT_MAX_CONCURRENCY,
            connect_timeout: DEFAULT_CONNECT_TIMEOUT,
        }
    }
}

impl EgressPolicy {
    #[doc(hidden)]
    pub fn for_test_allow_private_networks() -> Self {
        Self {
            allow_private_networks: true,
            ..Self::default()
        }
    }

    pub fn validate_url(&self, raw_url: &str) -> Result<Url, EgressError> {
        let url = Url::parse(raw_url).map_err(|_| EgressError::InvalidTarget)?;
        match url.scheme() {
            "https" => {}
            "http" if self.allow_http => {}
            _ => return Err(EgressError::SchemeNotAllowed),
        }

        if url.host().is_none() {
            return Err(EgressError::InvalidTarget);
        }
        if !url.username().is_empty() || url.password().is_some() {
            return Err(EgressError::CredentialsNotAllowed);
        }

        let hostname = url
            .host_str()
            .ok_or(EgressError::InvalidTarget)?
            .trim_end_matches('.')
            .to_ascii_lowercase();
        if is_metadata_hostname(&hostname) {
            return Err(EgressError::TargetNotAllowed);
        }

        if let Ok(ip) = hostname.parse::<IpAddr>() {
            self.validate_ip(ip)?;
        }

        Ok(url)
    }

    pub fn validate_ip(&self, ip: IpAddr) -> Result<(), EgressError> {
        if self.allow_private_networks || is_public_ip(ip) {
            Ok(())
        } else {
            Err(EgressError::TargetNotAllowed)
        }
    }
}

#[derive(Debug, thiserror::Error, Clone, Copy, PartialEq, Eq)]
pub enum EgressError {
    #[error("HTTP tool target is invalid")]
    InvalidTarget,
    #[error("HTTP tool URL scheme is not allowed")]
    SchemeNotAllowed,
    #[error("HTTP tool URL credentials are not allowed")]
    CredentialsNotAllowed,
    #[error("HTTP tool target is not allowed")]
    TargetNotAllowed,
    #[error("HTTP tool target could not be resolved safely")]
    DnsResolutionFailed,
    #[error("HTTP tool redirect was rejected")]
    RedirectRejected,
    #[error("HTTP tool redirect limit exceeded")]
    TooManyRedirects,
    #[error("HTTP tool concurrency limit reached")]
    ConcurrencyLimit,
    #[error("HTTP tool request timed out")]
    RequestTimedOut,
    #[error("HTTP tool request failed")]
    RequestFailed,
    #[error("HTTP tool response headers exceeded the limit")]
    ResponseHeadersTooLarge,
    #[error("HTTP tool response body exceeded the limit")]
    ResponseBodyTooLarge,
}

#[derive(Debug)]
pub struct SafeHttpResponse {
    pub status: StatusCode,
    pub body: Vec<u8>,
}

#[async_trait]
pub trait DnsResolver: Send + Sync {
    async fn resolve(&self, host: &str, port: u16) -> Result<Vec<SocketAddr>, EgressError>;
}

#[derive(Debug, Default)]
pub struct SystemDnsResolver;

#[async_trait]
impl DnsResolver for SystemDnsResolver {
    async fn resolve(&self, host: &str, port: u16) -> Result<Vec<SocketAddr>, EgressError> {
        tokio::net::lookup_host((host, port))
            .await
            .map(|addresses| addresses.collect())
            .map_err(|_| EgressError::DnsResolutionFailed)
    }
}

#[derive(Clone)]
pub struct SafeHttpClient {
    policy: EgressPolicy,
    resolver: Arc<dyn DnsResolver>,
    semaphore: Arc<Semaphore>,
}

impl Default for SafeHttpClient {
    fn default() -> Self {
        Self::new(EgressPolicy::default())
    }
}

impl SafeHttpClient {
    pub fn new(policy: EgressPolicy) -> Self {
        Self::with_resolver(policy, Arc::new(SystemDnsResolver))
    }

    pub fn with_resolver(policy: EgressPolicy, resolver: Arc<dyn DnsResolver>) -> Self {
        let semaphore = Arc::new(Semaphore::new(policy.max_concurrency));
        Self {
            policy,
            resolver,
            semaphore,
        }
    }

    pub fn policy(&self) -> &EgressPolicy {
        &self.policy
    }

    pub async fn execute(
        &self,
        method: Method,
        raw_url: &str,
        input: &serde_json::Value,
        timeout: Duration,
    ) -> Result<SafeHttpResponse, EgressError> {
        let _permit = self
            .semaphore
            .clone()
            .try_acquire_owned()
            .map_err(|_| EgressError::ConcurrencyLimit)?;

        let mut current_url = self.policy.validate_url(raw_url)?;

        for hop in 0..=self.policy.max_redirects {
            let (host, addresses) = self.resolve_and_validate(&current_url).await?;
            let mut builder = reqwest::Client::builder()
                .no_proxy()
                .redirect(reqwest::redirect::Policy::none())
                .referer(false)
                .connect_timeout(self.policy.connect_timeout)
                .timeout(timeout)
                .pool_max_idle_per_host(0);

            if let Some(host) = host.as_deref() {
                builder = builder.resolve_to_addrs(host, &addresses);
            }

            let client = builder.build().map_err(|_| EgressError::RequestFailed)?;
            let response = client
                .request(method.clone(), current_url.clone())
                .json(input)
                .send()
                .await
                .map_err(map_reqwest_error)?;

            let header_bytes = response
                .headers()
                .iter()
                .map(|(name, value)| name.as_str().len() + value.as_bytes().len())
                .sum::<usize>();
            if header_bytes > self.policy.max_header_bytes {
                return Err(EgressError::ResponseHeadersTooLarge);
            }

            if response.status().is_redirection() {
                if hop == self.policy.max_redirects {
                    return Err(EgressError::TooManyRedirects);
                }
                let location = response
                    .headers()
                    .get(LOCATION)
                    .and_then(|value| value.to_str().ok())
                    .ok_or(EgressError::RedirectRejected)?;
                current_url = current_url
                    .join(location)
                    .map_err(|_| EgressError::RedirectRejected)?;
                self.policy.validate_url(current_url.as_str())?;
                continue;
            }

            let status = response.status();
            let mut body = Vec::new();
            let mut stream = response.bytes_stream();
            while let Some(chunk) = stream.next().await {
                let chunk = chunk.map_err(|_| EgressError::RequestFailed)?;
                if body.len() + chunk.len() > self.policy.max_response_bytes {
                    return Err(EgressError::ResponseBodyTooLarge);
                }
                body.extend_from_slice(&chunk);
            }

            return Ok(SafeHttpResponse { status, body });
        }

        Err(EgressError::TooManyRedirects)
    }

    async fn resolve_and_validate(
        &self,
        url: &Url,
    ) -> Result<(Option<String>, Vec<SocketAddr>), EgressError> {
        let port = url.port_or_known_default().ok_or(EgressError::InvalidTarget)?;
        let host = url
            .host_str()
            .ok_or(EgressError::InvalidTarget)?
            .trim_end_matches('.')
            .to_ascii_lowercase();

        if let Ok(ip) = host.parse::<IpAddr>() {
            self.policy.validate_ip(ip)?;
            return Ok((None, vec![SocketAddr::new(ip, port)]));
        }

        if is_metadata_hostname(&host) {
            return Err(EgressError::TargetNotAllowed);
        }
        let addresses = self.resolver.resolve(&host, port).await?;
        if addresses.is_empty() {
            return Err(EgressError::DnsResolutionFailed);
        }

        let mut unique = BTreeSet::new();
        for address in addresses {
            self.policy.validate_ip(address.ip())?;
            unique.insert(address);
        }
        Ok((Some(host), unique.into_iter().collect()))
    }
}

fn map_reqwest_error(error: reqwest::Error) -> EgressError {
    if error.is_timeout() {
        EgressError::RequestTimedOut
    } else {
        EgressError::RequestFailed
    }
}

fn is_metadata_hostname(host: &str) -> bool {
    matches!(
        host,
        "metadata.google.internal"
            | "metadata.goog"
            | "instance-data"
            | "instance-data.ec2.internal"
            | "metadata.aws.internal"
            | "metadata.azure.internal"
    )
}

fn is_public_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(ip) => is_public_ipv4(ip),
        IpAddr::V6(ip) => is_public_ipv6(ip),
    }
}

fn is_public_ipv4(ip: Ipv4Addr) -> bool {
    let [a, b, c, d] = ip.octets();
    !matches!(
        (a, b, c, d),
        (0, _, _, _)
            | (10, _, _, _)
            | (100, 64..=127, _, _)
            | (127, _, _, _)
            | (169, 254, _, _)
            | (172, 16..=31, _, _)
            | (192, 0, 0, _)
            | (192, 0, 2, _)
            | (192, 88, 99, _)
            | (192, 168, _, _)
            | (198, 18..=19, _, _)
            | (198, 51, 100, _)
            | (203, 0, 113, _)
            | (224..=255, _, _, _)
    )
}

fn is_public_ipv6(ip: Ipv6Addr) -> bool {
    let segments = ip.segments();
    let globally_routable_unicast = (0x2000..=0x3fff).contains(&segments[0]);
    let documentation = segments[0] == 0x2001 && segments[1] == 0x0db8;
    globally_routable_unicast && !documentation
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct StaticResolver {
        calls: Arc<AtomicUsize>,
        addresses: Result<Vec<SocketAddr>, EgressError>,
    }

    #[async_trait]
    impl DnsResolver for StaticResolver {
        async fn resolve(&self, _host: &str, _port: u16) -> Result<Vec<SocketAddr>, EgressError> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            self.addresses.clone()
        }
    }

    #[test]
    fn rejects_disallowed_schemes_and_credentials_before_dns() {
        let policy = EgressPolicy::default();
        assert_eq!(
            policy.validate_url("file:///etc/passwd").unwrap_err(),
            EgressError::SchemeNotAllowed
        );
        assert_eq!(
            policy
                .validate_url("https://user:secret@example.com")
                .unwrap_err(),
            EgressError::CredentialsNotAllowed
        );
    }

    #[test]
    fn rejects_private_and_metadata_literal_addresses() {
        let policy = EgressPolicy::default();
        for url in [
            "http://127.0.0.1",
            "http://10.0.0.1",
            "http://169.254.169.254/latest/meta-data",
            "http://[::1]",
            "http://[fc00::1]",
            "http://[::ffff:127.0.0.1]",
        ] {
            assert_eq!(
                policy.validate_url(url).unwrap_err(),
                EgressError::TargetNotAllowed,
                "{url} should be rejected"
            );
        }
    }

    #[tokio::test]
    async fn mixed_dns_results_fail_closed_before_request() {
        let calls = Arc::new(AtomicUsize::new(0));
        let resolver = Arc::new(StaticResolver {
            calls: calls.clone(),
            addresses: Ok(vec![
                "93.184.216.34:443".parse().unwrap(),
                "127.0.0.1:443".parse().unwrap(),
            ]),
        });
        let client = SafeHttpClient::with_resolver(EgressPolicy::default(), resolver);

        let error = client
            .execute(
                Method::GET,
                "https://example.test/",
                &serde_json::json!({}),
                Duration::from_secs(1),
            )
            .await
            .unwrap_err();

        assert_eq!(error, EgressError::TargetNotAllowed);
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn dns_failure_does_not_fall_back_to_reqwest_resolution() {
        let calls = Arc::new(AtomicUsize::new(0));
        let resolver = Arc::new(StaticResolver {
            calls: calls.clone(),
            addresses: Err(EgressError::DnsResolutionFailed),
        });
        let client = SafeHttpClient::with_resolver(EgressPolicy::default(), resolver);

        let error = client
            .execute(
                Method::GET,
                "https://example.test/",
                &serde_json::json!({}),
                Duration::from_secs(1),
            )
            .await
            .unwrap_err();

        assert_eq!(error, EgressError::DnsResolutionFailed);
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
}
