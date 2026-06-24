//! Docker sandbox provider with container pool.
//!
//! Implements `ExecutionProvider` for `Code` and `Command` payloads using a
//! pool of pre-warmed Docker containers.

// This entire provider is a deprecated code path that will be removed in EVO-111.
#![allow(deprecated)]

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use bollard::exec::{CreateExecOptions, StartExecResults};
use bollard::models::{ContainerCreateBody, HostConfig};
use bollard::query_parameters::{CreateContainerOptions, RemoveContainerOptions};
use bollard::Docker;
use common::error::{AppError, Result};
use common::execution::{ExecutionPayload, ExecutionProvider, ExecutionRequest, ExecutionResponse};
use futures_util::StreamExt;
use tokio::sync::{Mutex, OwnedSemaphorePermit, Semaphore};
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::sandbox::SandboxConfig;

// ---------------------------------------------------------------------------
// Pool configuration
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct PoolConfig {
    pub min_idle: usize,
    pub max_total: usize,
    pub idle_timeout_secs: u64,
    pub max_lifetime_secs: u64,
    pub wait_timeout_secs: u64,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            min_idle: 1,
            max_total: 5,
            idle_timeout_secs: 300,
            max_lifetime_secs: 3600,
            wait_timeout_secs: 30,
        }
    }
}

impl PoolConfig {
    pub fn from_env() -> Self {
        Self {
            min_idle: std::env::var("SANDBOX__POOL_MIN_IDLE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(1),
            max_total: std::env::var("SANDBOX__POOL_MAX_TOTAL")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(5),
            idle_timeout_secs: std::env::var("SANDBOX__POOL_IDLE_TIMEOUT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(300),
            max_lifetime_secs: std::env::var("SANDBOX__POOL_MAX_LIFETIME")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(3600),
            wait_timeout_secs: std::env::var("SANDBOX__POOL_WAIT_TIMEOUT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(30),
        }
    }
}

// ---------------------------------------------------------------------------
// Container state
// ---------------------------------------------------------------------------

struct PooledContainer {
    container_id: String,
    #[allow(dead_code)]
    runtime: String,
    created_at: Instant,
    last_used: Instant,
}

impl PooledContainer {
    fn is_expired(&self, config: &PoolConfig) -> bool {
        let idle = self.last_used.elapsed().as_secs();
        let lifetime = self.created_at.elapsed().as_secs();
        idle > config.idle_timeout_secs || lifetime > config.max_lifetime_secs
    }
}

struct RuntimePool {
    idle: Vec<PooledContainer>,
    semaphore: Arc<Semaphore>,
}

impl RuntimePool {
    fn new(max_total: usize) -> Self {
        Self {
            idle: Vec::new(),
            semaphore: Arc::new(Semaphore::new(max_total)),
        }
    }
}

// ---------------------------------------------------------------------------
// DockerSandboxProvider
// ---------------------------------------------------------------------------

pub struct DockerSandboxProvider {
    docker: Docker,
    sandbox_config: SandboxConfig,
    pool_config: PoolConfig,
    pools: Mutex<HashMap<String, RuntimePool>>,
}

impl DockerSandboxProvider {
    pub fn new(sandbox_config: SandboxConfig, pool_config: PoolConfig) -> Result<Self> {
        let docker = Docker::connect_with_local_defaults().map_err(|e| {
            AppError::external_service("docker", format!("Failed to connect: {}", e))
        })?;

        Ok(Self {
            docker,
            sandbox_config,
            pool_config,
            pools: Mutex::new(HashMap::new()),
        })
    }

    pub async fn prewarm(&self) -> Result<()> {
        let runtimes = ["python311", "node20"];
        for runtime in &runtimes {
            for _ in 0..self.pool_config.min_idle {
                self.create_and_pool_container(runtime).await?;
            }
        }
        info!(
            "Docker sandbox pool prewarmed: {} runtimes x {} idle containers",
            runtimes.len(),
            self.pool_config.min_idle
        );
        Ok(())
    }

    async fn create_and_pool_container(&self, runtime: &str) -> Result<()> {
        let container_id = self.create_container(runtime).await?;
        let mut pools = self.pools.lock().await;
        let pool = pools
            .entry(runtime.to_string())
            .or_insert_with(|| RuntimePool::new(self.pool_config.max_total));
        pool.idle.push(PooledContainer {
            container_id,
            runtime: runtime.to_string(),
            created_at: Instant::now(),
            last_used: Instant::now(),
        });
        Ok(())
    }

    async fn borrow_container(
        &self,
        runtime: &str,
    ) -> Result<(PooledContainer, OwnedSemaphorePermit)> {
        let sem = {
            let pools = self.pools.lock().await;
            let pool = pools.get(runtime).ok_or_else(|| {
                AppError::external_service("docker", format!("Runtime pool not found: {}", runtime))
            })?;
            pool.semaphore.clone()
        };

        let permit = tokio::time::timeout(
            Duration::from_secs(self.pool_config.wait_timeout_secs),
            sem.acquire_owned(),
        )
        .await
        .map_err(|_| {
            AppError::external_service("docker", "Timeout waiting for container slot".to_string())
        })?
        .map_err(|_| AppError::external_service("docker", "Pool semaphore closed".to_string()))?;

        let mut pools = self.pools.lock().await;
        let pool = pools.get_mut(runtime).ok_or_else(|| {
            AppError::external_service("docker", format!("Runtime pool not found: {}", runtime))
        })?;

        pool.idle.retain(|c| !c.is_expired(&self.pool_config));

        if let Some(container) = pool.idle.pop() {
            Ok((container, permit))
        } else {
            let container_id = self.create_container(runtime).await?;
            Ok((
                PooledContainer {
                    container_id,
                    runtime: runtime.to_string(),
                    created_at: Instant::now(),
                    last_used: Instant::now(),
                },
                permit,
            ))
        }
    }

    async fn return_container(
        &self,
        runtime: &str,
        container: PooledContainer,
        permit: OwnedSemaphorePermit,
    ) {
        drop(permit);

        if container.is_expired(&self.pool_config) {
            let _ = self.remove_container(&container.container_id).await;
            return;
        }

        let mut pools = self.pools.lock().await;
        let pool = pools
            .entry(runtime.to_string())
            .or_insert_with(|| RuntimePool::new(self.pool_config.max_total));
        pool.idle.push(PooledContainer {
            last_used: Instant::now(),
            ..container
        });
    }

    async fn create_container(&self, runtime: &str) -> Result<String> {
        let image = match runtime {
            "python311" => "evolith-python-sandbox:latest",
            "node20" => "evolith-node-sandbox:latest",
            _ => {
                return Err(AppError::ValidationError(format!(
                    "Unsupported runtime: {}",
                    runtime
                )))
            }
        };

        let mut labels: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();
        labels.insert("evolith.sandbox".to_string(), "true".to_string());
        labels.insert("evolith.pool".to_string(), "true".to_string());

        let network_mode = if self.sandbox_config.network_enabled {
            "bridge"
        } else {
            "none"
        };

        let host_config = HostConfig {
            memory: Some((self.sandbox_config.memory_mb as i64) * 1024 * 1024),
            cpu_shares: Some(self.sandbox_config.cpu_shares),
            pids_limit: Some(self.sandbox_config.pids_limit),
            network_mode: Some(network_mode.to_string()),
            security_opt: Some(vec!["no-new-privileges".to_string()]),
            cap_drop: Some(vec![
                "NET_RAW".to_string(),
                "SYS_ADMIN".to_string(),
                "MKNOD".to_string(),
            ]),
            ..Default::default()
        };

        let container_config = ContainerCreateBody {
            image: Some(image.to_string()),
            cmd: Some(vec![
                "tail".to_string(),
                "-f".to_string(),
                "/dev/null".to_string(),
            ]),
            working_dir: Some("/workspace".to_string()),
            user: Some("sandbox".to_string()),
            host_config: Some(host_config),
            labels: Some(labels),
            ..Default::default()
        };

        let sandbox_id = Uuid::new_v4();
        let container_name = format!("evolith-pool-{}-{}", runtime, sandbox_id);

        let response = self
            .docker
            .create_container(
                Some(CreateContainerOptions {
                    name: Some(container_name),
                    platform: String::new(),
                }),
                container_config,
            )
            .await
            .map_err(|e| {
                AppError::external_service("docker", format!("Failed to create container: {}", e))
            })?;

        self.docker
            .start_container(&response.id, None)
            .await
            .map_err(|e| {
                AppError::external_service("docker", format!("Failed to start container: {}", e))
            })?;

        debug!(
            "Created pooled container: {} for runtime {}",
            response.id, runtime
        );
        Ok(response.id)
    }

    async fn remove_container(&self, container_id: &str) -> Result<()> {
        let options = RemoveContainerOptions {
            force: true,
            ..Default::default()
        };

        self.docker
            .remove_container(container_id, Some(options))
            .await
            .map_err(|e| {
                AppError::external_service("docker", format!("Failed to remove container: {}", e))
            })?;

        debug!("Removed container: {}", container_id);
        Ok(())
    }

    async fn exec_in_container(
        &self,
        container_id: &str,
        command: &[String],
        timeout_secs: u32,
        max_output: usize,
    ) -> Result<(String, String, i64, bool)> {
        let timeout_duration = Duration::from_secs(timeout_secs as u64);

        let exec_result = tokio::time::timeout(timeout_duration, async {
            self.exec_inner(container_id, command, max_output).await
        })
        .await;

        match exec_result {
            Ok(Ok(result)) => Ok((result.0, result.1, result.2, false)),
            Ok(Err(e)) => {
                let is_timeout = e.to_string().contains("timed out");
                Ok((String::new(), e.to_string(), -1, is_timeout))
            }
            Err(_) => Ok((
                String::new(),
                format!("Execution timed out after {} seconds", timeout_secs),
                -1,
                true,
            )),
        }
    }

    async fn exec_inner(
        &self,
        container_id: &str,
        command: &[String],
        max_output: usize,
    ) -> Result<(String, String, i64)> {
        let exec_config = CreateExecOptions {
            cmd: Some(command.to_vec()),
            attach_stdout: Some(true),
            attach_stderr: Some(true),
            ..Default::default()
        };

        let exec_response = self
            .docker
            .create_exec(container_id, exec_config)
            .await
            .map_err(|e| {
                AppError::external_service("docker", format!("Failed to create exec: {}", e))
            })?;

        let exec_result = self
            .docker
            .start_exec(&exec_response.id, None)
            .await
            .map_err(|e| {
                AppError::external_service("docker", format!("Failed to start exec: {}", e))
            })?;

        let mut stdout = String::new();
        let mut stderr = String::new();

        if let StartExecResults::Attached { mut output, .. } = exec_result {
            while let Some(msg) = output.next().await {
                match msg {
                    Ok(log_output) => match log_output {
                        bollard::container::LogOutput::StdOut { message } => {
                            let data = String::from_utf8_lossy(&message);
                            if stdout.len() + data.len() <= max_output {
                                stdout.push_str(&data);
                            } else if stdout.len() < max_output {
                                let remaining = max_output - stdout.len();
                                stdout.push_str(&data.chars().take(remaining).collect::<String>());
                            }
                        }
                        bollard::container::LogOutput::StdErr { message } => {
                            let data = String::from_utf8_lossy(&message);
                            if stderr.len() + data.len() <= max_output {
                                stderr.push_str(&data);
                            } else if stderr.len() < max_output {
                                let remaining = max_output - stderr.len();
                                stderr.push_str(&data.chars().take(remaining).collect::<String>());
                            }
                        }
                        _ => {}
                    },
                    Err(e) => {
                        warn!("Error reading exec output: {}", e);
                        break;
                    }
                }
            }
        }

        let exec_inspect = self
            .docker
            .inspect_exec(&exec_response.id)
            .await
            .map_err(|e| {
                AppError::external_service("docker", format!("Failed to inspect exec: {}", e))
            })?;

        let exit_code = exec_inspect.exit_code.unwrap_or(-1);
        Ok((stdout, stderr, exit_code))
    }

    fn resolve_runtime(&self, language: &str) -> Result<String> {
        match language {
            "python311" | "python" | "py" => Ok("python311".to_string()),
            "node20" | "node" | "javascript" | "js" => Ok("node20".to_string()),
            _ => Err(AppError::ValidationError(format!(
                "Unsupported runtime: {}",
                language
            ))),
        }
    }

    fn build_command(runtime: &str, source: &str) -> Vec<String> {
        match runtime {
            "python311" => vec!["python3".to_string(), "-c".to_string(), source.to_string()],
            "node20" => vec!["node".to_string(), "-e".to_string(), source.to_string()],
            _ => vec!["sh".to_string(), "-c".to_string(), source.to_string()],
        }
    }
}

#[async_trait]
impl ExecutionProvider for DockerSandboxProvider {
    async fn execute(&self, request: ExecutionRequest) -> Result<ExecutionResponse> {
        let start = Instant::now();

        let (runtime, command) = match &request.payload {
            ExecutionPayload::Code { source, language } => {
                let runtime = self.resolve_runtime(language)?;
                let command = Self::build_command(&runtime, source);
                (runtime, command)
            }
            ExecutionPayload::Command { command, args } => {
                let mut full_command = vec![command.clone()];
                full_command.extend(args.clone());
                ("python311".to_string(), full_command)
            }
            ExecutionPayload::HttpProxy { .. } => {
                return Err(AppError::ValidationError(
                    "DockerSandboxProvider does not handle HttpProxy payloads".to_string(),
                ))
            }
        };

        let (container, permit) = self.borrow_container(&runtime).await?;

        let (stdout, stderr, exit_code, timed_out) = self
            .exec_in_container(
                &container.container_id,
                &command,
                request.constraints.timeout_seconds,
                request.constraints.max_output_bytes,
            )
            .await?;

        let elapsed = start.elapsed().as_millis() as u64;

        self.return_container(&runtime, container, permit).await;

        let output = if exit_code == 0 {
            serde_json::json!({ "output": stdout, "errors": stderr })
        } else {
            serde_json::json!({ "error": stderr, "exit_code": exit_code })
        };

        Ok(ExecutionResponse {
            output,
            stdout,
            stderr,
            exit_code,
            execution_time_ms: elapsed,
            timed_out,
            http_status: None,
        })
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;

    #[test]
    fn pool_config_default() {
        let cfg = PoolConfig::default();
        assert_eq!(cfg.min_idle, 1);
        assert_eq!(cfg.max_total, 5);
        assert_eq!(cfg.idle_timeout_secs, 300);
        assert_eq!(cfg.max_lifetime_secs, 3600);
        assert_eq!(cfg.wait_timeout_secs, 30);
    }

    #[test]
    fn pooled_container_expires_by_idle_timeout() {
        let container = PooledContainer {
            container_id: "abc".to_string(),
            runtime: "python311".to_string(),
            created_at: Instant::now(),
            last_used: Instant::now() - Duration::from_secs(400),
        };
        let config = PoolConfig::default();
        assert!(container.is_expired(&config));
    }

    #[test]
    fn pooled_container_expires_by_max_lifetime() {
        let container = PooledContainer {
            container_id: "abc".to_string(),
            runtime: "python311".to_string(),
            created_at: Instant::now() - Duration::from_secs(4000),
            last_used: Instant::now(),
        };
        let config = PoolConfig::default();
        assert!(container.is_expired(&config));
    }

    #[test]
    fn pooled_container_not_expired() {
        let container = PooledContainer {
            container_id: "abc".to_string(),
            runtime: "python311".to_string(),
            created_at: Instant::now(),
            last_used: Instant::now(),
        };
        let config = PoolConfig::default();
        assert!(!container.is_expired(&config));
    }

    #[test]
    fn build_command_python() {
        let cmd = DockerSandboxProvider::build_command("python311", "print('hi')");
        assert_eq!(cmd, vec!["python3", "-c", "print('hi')"]);
    }

    #[test]
    fn resolve_runtime_node() {
        let cmd = DockerSandboxProvider::build_command("node20", "console.log('hi')");
        assert_eq!(cmd, vec!["node", "-e", "console.log('hi')"]);
    }

    #[test]
    fn build_command_fallback() {
        let cmd = DockerSandboxProvider::build_command("unknown", "echo hi");
        assert_eq!(cmd, vec!["sh", "-c", "echo hi"]);
    }

    #[test]
    fn docker_provider_new_fails_without_docker() {
        let sandbox_config = SandboxConfig::default();
        let result = DockerSandboxProvider::new(sandbox_config, PoolConfig::default());
        assert!(
            result.is_err(),
            "DockerSandboxProvider::new should fail without Docker daemon"
        );
        let err = match result {
            Ok(_) => panic!("expected error"),
            Err(e) => e,
        };
        assert!(err.to_string().contains("docker") || err.to_string().contains("Docker"));
    }
}
