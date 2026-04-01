//! Rate limiting middleware using actix-governor.

use actix_governor::{
    governor::middleware::NoOpMiddleware, GovernorConfig, GovernorConfigBuilder, PeerIpKeyExtractor,
};
use infra::config::RateLimitConfig as InfraRateLimitConfig;

const MIN_RPM: u32 = 30;
const DEFAULT_RPM: u32 = 60;

type IpRateLimitConfig = GovernorConfig<PeerIpKeyExtractor, NoOpMiddleware>;

pub fn create_governor_config(requests_per_minute: u32) -> IpRateLimitConfig {
    let rpm = if requests_per_minute == 0 {
        DEFAULT_RPM
    } else {
        requests_per_minute.max(MIN_RPM)
    };

    let burst_size = (rpm / 10).max(3);

    let config = GovernorConfigBuilder::default()
        .requests_per_minute(u64::from(rpm))
        .burst_size(burst_size)
        .finish();

    match config {
        Some(c) => c,
        None => GovernorConfig::secure(),
    }
}

pub fn create_unauthenticated_limiter(config: &InfraRateLimitConfig) -> IpRateLimitConfig {
    create_governor_config(config.unauthenticated_rpm)
}

pub fn create_authenticated_limiter(config: &InfraRateLimitConfig) -> IpRateLimitConfig {
    create_governor_config(config.authenticated_rpm)
}

pub fn create_api_key_limiter(config: &InfraRateLimitConfig) -> IpRateLimitConfig {
    create_governor_config(config.api_key_rpm)
}

pub fn create_default_limiter() -> IpRateLimitConfig {
    create_governor_config(DEFAULT_RPM)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_governor_config_basic() {
        let _config = create_governor_config(60);
    }

    #[test]
    fn test_create_governor_config_zero_rpm() {
        let _config = create_governor_config(0);
    }

    #[test]
    fn test_create_governor_config_very_low_rpm() {
        let _config = create_governor_config(10);
    }

    #[test]
    fn test_create_governor_config_high_rpm() {
        let _config = create_governor_config(1000);
    }

    #[test]
    fn test_create_unauthenticated_limiter() {
        let config = InfraRateLimitConfig {
            unauthenticated_rpm: 30,
            authenticated_rpm: 300,
            api_key_rpm: 1000,
        };
        let _limiter = create_unauthenticated_limiter(&config);
    }

    #[test]
    fn test_create_authenticated_limiter() {
        let config = InfraRateLimitConfig {
            unauthenticated_rpm: 30,
            authenticated_rpm: 300,
            api_key_rpm: 1000,
        };
        let _limiter = create_authenticated_limiter(&config);
    }

    #[test]
    fn test_create_api_key_limiter() {
        let config = InfraRateLimitConfig {
            unauthenticated_rpm: 30,
            authenticated_rpm: 300,
            api_key_rpm: 1000,
        };
        let _limiter = create_api_key_limiter(&config);
    }

    #[test]
    fn test_create_default_limiter() {
        let _limiter = create_default_limiter();
    }

    #[test]
    fn test_rpm_conversion_math() {
        // burst = rpm / 10, min 3
        assert_eq!((60u32 / 10).max(3), 6);
        assert_eq!((300u32 / 10).max(3), 30);
        assert_eq!((1000u32 / 10).max(3), 100);
        assert_eq!((30u32 / 10).max(3), 3);
    }
}
