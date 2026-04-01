//! Docker-based sandbox executor for skill code execution

use async_trait::async_trait;
use bollard::container::{Config as ContainerConfig, RemoveContainerOptions};
use bollard::exec::{CreateExecOptions, StartExecResults};
use bollard::models::HostConfig;
use bollard::Docker;
use common::error::{AppError, Result};
use futures_util::StreamExt;
use std::collections::HashMap;
use std::time::Duration;
use tracing::{debug, info, warn};

use crate::executor::{ExecuteRequest, ExecuteResponse, SkillExecutor};
use crate::sandbox::SandboxConfig;

pub struct DockerExecutor {
    docker: Docker,
    config: SandboxConfig,
}

impl DockerExecutor {
    pub fn new(config: SandboxConfig) -> Result<Self> {
        let docker = Docker::connect_with_local_defaults().map_err(|e| {
            AppError::external_service("docker", format!("Failed to connect: {}", e))
        })?;

        Ok(Self { docker, config })
    }

    pub async fn check_docker_connection(&self) -> Result<()> {
        self.docker
            .ping()
            .await
            .map_err(|e| AppError::external_service("docker", format!("Ping failed: {}", e)))?;
        Ok(())
    }

    async fn create_container(&self, sandbox_id: &str, runtime: &str) -> Result<String> {
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

        let mut labels: HashMap<&str, &str> = HashMap::new();
        labels.insert("evolith.sandbox", "true");
        labels.insert("evolith.sandbox-id", sandbox_id);

        let network_mode = if self.config.network_enabled {
            "bridge"
        } else {
            "none"
        };

        let host_config = HostConfig {
            memory: Some((self.config.memory_mb as i64) * 1024 * 1024),
            cpu_shares: Some(self.config.cpu_shares),
            pids_limit: Some(self.config.pids_limit),
            network_mode: Some(network_mode.to_string()),
            security_opt: Some(vec!["no-new-privileges".to_string()]),
            cap_drop: Some(vec![
                "NET_RAW".to_string(),
                "SYS_ADMIN".to_string(),
                "MKNOD".to_string(),
            ]),
            ..Default::default()
        };

        let container_config = ContainerConfig {
            image: Some(image),
            cmd: Some(vec!["tail", "-f", "/dev/null"]),
            working_dir: Some("/workspace"),
            user: Some("sandbox"),
            host_config: Some(host_config),
            labels: Some(labels),
            ..Default::default()
        };

        let container_name = format!("evolith-sandbox-{}", sandbox_id);

        let response = self
            .docker
            .create_container(
                Some(bollard::container::CreateContainerOptions {
                    name: &container_name,
                    platform: None,
                }),
                container_config,
            )
            .await
            .map_err(|e| {
                AppError::external_service("docker", format!("Failed to create container: {}", e))
            })?;

        self.docker
            .start_container::<String>(&response.id, None)
            .await
            .map_err(|e| {
                AppError::external_service("docker", format!("Failed to start container: {}", e))
            })?;

        info!(
            "Created sandbox container: {} for runtime {}",
            response.id, runtime
        );
        Ok(response.id)
    }

    async fn execute_in_container(
        &self,
        container_id: &str,
        command: &[String],
    ) -> Result<(String, String, i64)> {
        let timeout_duration = Duration::from_secs(self.config.timeout_seconds as u64);

        let exec_result = tokio::time::timeout(timeout_duration, async {
            self.execute_in_container_inner(container_id, command).await
        })
        .await;

        match exec_result {
            Ok(Ok(result)) => Ok(result),
            Ok(Err(e)) => Err(e),
            Err(_) => Err(AppError::external_service(
                "docker",
                format!(
                    "Execution timed out after {} seconds",
                    self.config.timeout_seconds
                ),
            )),
        }
    }

    async fn execute_in_container_inner(
        &self,
        container_id: &str,
        command: &[String],
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

        match exec_result {
            StartExecResults::Attached { mut output, .. } => {
                while let Some(msg) = output.next().await {
                    match msg {
                        Ok(log_output) => match log_output {
                            bollard::container::LogOutput::StdOut { message } => {
                                let data = String::from_utf8_lossy(&message);
                                if stdout.len() + data.len() <= self.config.max_output_bytes {
                                    stdout.push_str(&data);
                                } else if stdout.len() < self.config.max_output_bytes {
                                    let remaining = self.config.max_output_bytes - stdout.len();
                                    stdout.push_str(
                                        &data.chars().take(remaining).collect::<String>(),
                                    );
                                }
                            }
                            bollard::container::LogOutput::StdErr { message } => {
                                let data = String::from_utf8_lossy(&message);
                                if stderr.len() + data.len() <= self.config.max_output_bytes {
                                    stderr.push_str(&data);
                                } else if stderr.len() < self.config.max_output_bytes {
                                    let remaining = self.config.max_output_bytes - stderr.len();
                                    stderr.push_str(
                                        &data.chars().take(remaining).collect::<String>(),
                                    );
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
            StartExecResults::Detached => {
                warn!("Exec started in detached mode, cannot capture output");
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

        debug!(
            "Exec completed in container {}: exit_code={}",
            container_id, exit_code
        );

        Ok((stdout, stderr, exit_code))
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
}

#[async_trait]
impl SkillExecutor for DockerExecutor {
    async fn execute(&self, request: ExecuteRequest) -> Result<ExecuteResponse> {
        let sandbox_id = uuid::Uuid::new_v4().to_string();
        let start = std::time::Instant::now();

        let runtime = match request.language.as_str() {
            "python311" | "python" | "py" => "python311",
            "node20" | "node" | "javascript" | "js" => "node20",
            _ => {
                return Err(AppError::ValidationError(format!(
                    "Unsupported runtime: {}",
                    request.language
                )))
            }
        };

        let container_result = self.create_container(&sandbox_id, runtime).await;

        let container_id = match container_result {
            Ok(id) => id,
            Err(e) => {
                return Ok(ExecuteResponse {
                    result: serde_json::json!({ "error": e.to_string() }),
                    stdout: String::new(),
                    stderr: e.to_string(),
                    exit_code: -1,
                    execution_time_ms: start.elapsed().as_millis() as u64,
                    timed_out: false,
                });
            }
        };

        let command = match runtime {
            "python311" => vec![
                "python3".to_string(),
                "-c".to_string(),
                request.code.clone(),
            ],
            "node20" => vec!["node".to_string(), "-e".to_string(), request.code.clone()],
            _ => unreachable!("Runtime was validated above"),
        };

        let exec_result = self.execute_in_container(&container_id, &command).await;

        let _ = self.remove_container(&container_id).await;

        let elapsed = start.elapsed().as_millis() as u64;

        match exec_result {
            Ok((stdout, stderr, exit_code)) => Ok(ExecuteResponse {
                result: serde_json::json!({
                    "output": stdout,
                    "errors": stderr,
                }),
                stdout,
                stderr,
                exit_code,
                execution_time_ms: elapsed,
                timed_out: false,
            }),
            Err(e) => {
                let is_timeout = e.to_string().contains("timed out");
                Ok(ExecuteResponse {
                    result: serde_json::json!({ "error": e.to_string() }),
                    stdout: String::new(),
                    stderr: e.to_string(),
                    exit_code: -1,
                    execution_time_ms: elapsed,
                    timed_out: is_timeout,
                })
            }
        }
    }
}
