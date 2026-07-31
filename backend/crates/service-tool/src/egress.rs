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
pub const MAX_REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
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
            allow_http: false,
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
            allow_http: true,
            allow_private_networks: true,
            ..Self::default()
        }
    }

    pub fn validate_url(&self, raw_url: &str) -> Result<Url, EgressError> {
        let mut url = Url::parse(raw_url).map_err(|_| EgressError::InvalidTarget)?;
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

        let hostname = canonical_host(&url)?;
        if is_metadata_hostname(&hostname) {
            return Err(EgressError::TargetNotAllowed);
        }
        match hostname.parse::<IpAddr>() {
            Ok(ip) => self.validate_ip(ip)?,
            Err(_) => {
                if url.host_str() != Some(hostname.as_str()) {
                    url.set_host(Some(&hostname))
                        .map_err(|_| EgressError::InvalidTarget)?;
                }
            }
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
    #[error("HTTP tool timeout must be between 1 and 30000 milliseconds")]
    InvalidTimeout,
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

    /// Validate and resolve a target without opening a network connection.
    pub async fn validate_target(&self, raw_url: &str) -> Result<(), EgressError> {
        let url = self.policy.validate_url(raw_url)?;
        tokio::time::timeout(self.policy.connect_timeout, self.resolve_and_validate(&url))
            .await
            .map_err(|_| EgressError::RequestTimedOut)??;
        Ok(())
    }

    pub async fn execute(
        &self,
        method: Method,
        raw_url: &str,
        input: &serde_json::Value,
        timeout: Duration,
    ) -> Result<SafeHttpResponse, EgressError> {
        if timeout.is_zero() || timeout > MAX_REQUEST_TIMEOUT {
            return Err(EgressError::InvalidTimeout);
        }
        let _permit = self
            .semaphore
            .clone()
            .try_acquire_owned()
            .map_err(|_| EgressError::ConcurrencyLimit)?;

        tokio::time::timeout(
            timeout,
            self.execute_within_deadline(method, raw_url, input),
        )
        .await
        .map_err(|_| EgressError::RequestTimedOut)?
    }

    async fn execute_within_deadline(
        &self,
        method: Method,
        raw_url: &str,
        input: &serde_json::Value,
    ) -> Result<SafeHttpResponse, EgressError> {
        let mut current_url = self.policy.validate_url(raw_url)?;

        for hop in 0..=self.policy.max_redirects {
            let (host, addresses) = self.resolve_and_validate(&current_url).await?;
            let mut builder = reqwest::Client::builder()
                .no_proxy()
                .redirect(reqwest::redirect::Policy::none())
                .referer(false)
                .connect_timeout(self.policy.connect_timeout)
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
                current_url = redirect_target(&self.policy, &current_url, location)?;
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
        let port = url
            .port_or_known_default()
            .ok_or(EgressError::InvalidTarget)?;
        let host = canonical_host(url)?;

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

fn redirect_target(
    policy: &EgressPolicy,
    current_url: &Url,
    location: &str,
) -> Result<Url, EgressError> {
    let target = current_url
        .join(location)
        .map_err(|_| EgressError::RedirectRejected)?;
    policy.validate_url(target.as_str())
}

fn canonical_host(url: &Url) -> Result<String, EgressError> {
    let raw_host = url.host_str().ok_or(EgressError::InvalidTarget)?;
    let unbracketed = raw_host
        .strip_prefix('[')
        .and_then(|host| host.strip_suffix(']'))
        .unwrap_or(raw_host);
    Ok(unbracketed.trim_end_matches('.').to_ascii_lowercase())
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
    let [first, second, ..] = ip.segments();
    match first {
        0x2001 => {
            matches!(
                second,
                0x0200..=0x0fff
                    | 0x1200..=0x4dff
                    | 0x5000..=0x5fff
                    | 0x8000..=0xbfff
            ) && second != 0x0db8
        }
        0x2003 => second <= 0x3fff,
        0x2400..=0x241f
        | 0x2600..=0x260f
        | 0x2630..=0x263f
        | 0x2800..=0x280f
        | 0x2a00..=0x2a1f
        | 0x2c00..=0x2c0f => true,
        0x2610 | 0x2620 => second <= 0x01ff,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use tokio::sync::Notify;

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

    struct HangingResolver;

    #[async_trait]
    impl DnsResolver for HangingResolver {
        async fn resolve(&self, _host: &str, _port: u16) -> Result<Vec<SocketAddr>, EgressError> {
            std::future::pending().await
        }
    }

    struct SignalingHangingResolver {
        entered: Arc<Notify>,
    }

    #[async_trait]
    impl DnsResolver for SignalingHangingResolver {
        async fn resolve(&self, _host: &str, _port: u16) -> Result<Vec<SocketAddr>, EgressError> {
            self.entered.notify_one();
            std::future::pending().await
        }
    }

    #[test]
    fn rejects_disallowed_schemes_and_credentials_before_dns() {
        let policy = EgressPolicy::default();
        assert_eq!(
            policy.validate_url("http://93.184.216.34").unwrap_err(),
            EgressError::SchemeNotAllowed
        );
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
    fn normalizes_domain_host_before_resolution_and_pinning() {
        let url = EgressPolicy::default()
            .validate_url("https://EXAMPLE.COM./path")
            .unwrap();
        assert_eq!(url.host_str(), Some("example.com"));
    }

    #[test]
    fn rejects_private_metadata_and_special_purpose_addresses() {
        let policy = EgressPolicy::default();
        for url in [
            "https://127.0.0.1",
            "https://10.0.0.1",
            "https://169.254.169.254/latest/meta-data",
            "https://[::]",
            "https://[::1]",
            "https://[fe80::1]",
            "https://[fc00::1]",
            "https://[ff02::1]",
            "https://[::ffff:127.0.0.1]",
            "https://[2001:2::1]",
            "https://[2001:10::1]",
            "https://[2001:20::1]",
            "https://[2001:1000::1]",
            "https://[2001:db8::1]",
            "https://[2002:c000:0201::1]",
            "https://[2d00::1]",
            "https://[2e00::1]",
            "https://[3000::1]",
            "https://[3ffe::1]",
            "https://[3fff::1]",
        ] {
            assert_eq!(
                policy.validate_url(url).unwrap_err(),
                EgressError::TargetNotAllowed,
                "{url} should be rejected"
            );
        }
    }

    #[test]
    fn accepts_representative_allocated_global_ipv4_and_ipv6() {
        let policy = EgressPolicy::default();
        for url in [
            "https://93.184.216.34",
            "https://[2001:4860:4860::8888]",
            "https://[2404:6800:4001::200e]",
            "https://[2606:4700:4700::1111]",
            "https://[2804:14d:1::1]",
            "https://[2a00:1450:4001::200e]",
        ] {
            assert!(
                policy.validate_url(url).is_ok(),
                "{url} should be accepted"
            );
        }
    }

    #[test]
    fn private_redirect_is_rejected_before_dns_or_connection() {
        let policy = EgressPolicy::default();
        let current = policy.validate_url("https://93.184.216.34/start").unwrap();
        let error = redirect_target(&policy, &current, "https://127.0.0.1/private").unwrap_err();
        assert_eq!(error, EgressError::TargetNotAllowed);
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

    #[tokio::test]
    async fn execution_deadline_includes_dns_resolution() {
        let client =
            SafeHttpClient::with_resolver(EgressPolicy::default(), Arc::new(HangingResolver));
        let error = client
            .execute(
                Method::GET,
                "https://example.test/",
                &serde_json::json!({}),
                Duration::from_millis(20),
            )
            .await
            .unwrap_err();
        assert_eq!(error, EgressError::RequestTimedOut);
    }

    #[tokio::test]
    async fn total_deadline_is_shared_across_redirect_hops() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            loop {
                let Ok((mut socket, _)) = listener.accept().await else {
                    return;
                };
                tokio::spawn(async move {
                    let mut request = [0_u8; 4096];
                    let _ = socket.read(&mut request).await;
                    tokio::time::sleep(Duration::from_millis(40)).await;
                    let response = b"HTTP/1.1 302 Found\r\nLocation: /slow-redirect\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
                    let _ = socket.write_all(response).await;
                });
            }
        });

        let client = SafeHttpClient::new(EgressPolicy::for_test_allow_private_networks());
        let error = client
            .execute(
                Method::GET,
                &format!("http://{address}/slow-redirect"),
                &serde_json::json!({}),
                Duration::from_millis(70),
            )
            .await
            .unwrap_err();
        assert_eq!(error, EgressError::RequestTimedOut);

        server.abort();
        let _ = server.await;
    }

    #[tokio::test]
    async fn response_header_limit_is_enforced() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut request = [0_u8; 4096];
            let _ = socket.read(&mut request).await;
            let response = format!(
                "HTTP/1.1 200 OK\r\nX-Large: {}\r\nContent-Length: 2\r\nConnection: close\r\n\r\nok",
                "x".repeat(128)
            );
            socket.write_all(response.as_bytes()).await.unwrap();
        });

        let policy = EgressPolicy {
            max_header_bytes: 64,
            ..EgressPolicy::for_test_allow_private_networks()
        };
        let client = SafeHttpClient::new(policy);
        let error = client
            .execute(
                Method::GET,
                &format!("http://{address}/headers"),
                &serde_json::json!({}),
                Duration::from_secs(1),
            )
            .await
            .unwrap_err();
        assert_eq!(error, EgressError::ResponseHeadersTooLarge);
        server.await.unwrap();
    }

    #[tokio::test]
    async fn concurrency_limit_fails_closed_without_waiting() {
        let entered = Arc::new(Notify::new());
        let resolver = Arc::new(SignalingHangingResolver {
            entered: entered.clone(),
        });
        let policy = EgressPolicy {
            max_concurrency: 1,
            ..EgressPolicy::default()
        };
        let client = SafeHttpClient::with_resolver(policy, resolver);
        let first_client = client.clone();
        let first = tokio::spawn(async move {
            first_client
                .execute(
                    Method::GET,
                    "https://first.example.test/",
                    &serde_json::json!({}),
                    Duration::from_secs(1),
                )
                .await
        });
        entered.notified().await;

        let error = client
            .execute(
                Method::GET,
                "https://second.example.test/",
                &serde_json::json!({}),
                Duration::from_secs(1),
            )
            .await
            .unwrap_err();
        assert_eq!(error, EgressError::ConcurrencyLimit);

        first.abort();
        let _ = first.await;
    }

    #[tokio::test]
    async fn configuration_validation_bounds_hanging_dns() {
        let policy = EgressPolicy {
            connect_timeout: Duration::from_millis(20),
            ..EgressPolicy::default()
        };
        let client = SafeHttpClient::with_resolver(policy, Arc::new(HangingResolver));
        let error = client
            .validate_target("https://example.test/")
            .await
            .unwrap_err();
        assert_eq!(error, EgressError::RequestTimedOut);
    }

    #[tokio::test]
    async fn rejects_timeout_above_runtime_limit() {
        let client = SafeHttpClient::default();
        let error = client
            .execute(
                Method::GET,
                "https://example.com/",
                &serde_json::json!({}),
                Duration::from_millis(30_001),
            )
            .await
            .unwrap_err();
        assert_eq!(error, EgressError::InvalidTimeout);
    }
}
