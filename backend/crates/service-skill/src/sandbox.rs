//! Sandbox execution configuration

/// Sandbox execution configuration for Docker-based skill execution
#[derive(Debug, Clone)]
pub struct SandboxConfig {
    pub enabled: bool,
    pub timeout_seconds: u32,
    pub memory_mb: u32,
    pub cpu_shares: i64,
    pub pids_limit: i64,
    pub network_enabled: bool,
    pub max_output_bytes: usize,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            timeout_seconds: 30,
            memory_mb: 256,
            cpu_shares: 512,
            pids_limit: 256,
            network_enabled: false,
            max_output_bytes: 10 * 1024 * 1024,
        }
    }
}

impl SandboxConfig {
    pub fn from_infra(config: &infra::config::SandboxConfig) -> Self {
        Self {
            enabled: config.enabled,
            timeout_seconds: config.timeout_seconds,
            memory_mb: config.memory_mb,
            cpu_shares: config.cpu_shares,
            pids_limit: config.pids_limit,
            network_enabled: config.network_enabled,
            max_output_bytes: config.max_output_bytes,
        }
    }
}
