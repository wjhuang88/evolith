use async_trait::async_trait;
use common::error::{AppError, Result};
use domain::repository::OutboxRepository;
use domain::{OutboxEvent, OutboxHandler, OutboxRuntime, OutboxWorker};
use infra::config::{AppConfig, OutboxWorkerConfig};
use infra::db::pool::DatabasePool;
use infra::db::{create_pool, PgOutboxRepository, SqliteOutboxRepository};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

const PROBE_EVENT_TYPE: &str = "system.outbox.probe";

#[derive(Debug, PartialEq, Eq)]
enum Command {
    Run { once: bool },
    Replay { event_id: Uuid },
    Help,
}

struct ProbeHandler;

#[async_trait]
impl OutboxHandler for ProbeHandler {
    async fn deliver(&self, event: &OutboxEvent) -> Result<()> {
        if event.event_type == PROBE_EVENT_TYPE {
            Ok(())
        } else {
            Err(AppError::ServiceUnavailableError(
                "no registered outbox handler".to_string(),
            ))
        }
    }
}

#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        eprintln!("outbox worker failed: {}", error.error_code_info().code);
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let command = parse_args(std::env::args().skip(1))?;
    if command == Command::Help {
        print_usage();
        return Ok(());
    }

    let app_config = AppConfig::from_env()?;
    let worker_config = OutboxWorkerConfig::from_env()?;
    init_tracing(&app_config);
    let pool = create_pool(&app_config.database).await?;

    match pool {
        DatabasePool::Sqlite(pool) => {
            sqlx::migrate!("./migrations/sqlite")
                .run(&pool)
                .await
                .map_err(|error| AppError::DatabaseError(error.to_string()))?;
            execute(command, SqliteOutboxRepository::new(pool), &worker_config).await
        }
        DatabasePool::Postgres(pool) => {
            sqlx::migrate!("./migrations/postgres")
                .run(&pool)
                .await
                .map_err(|error| AppError::DatabaseError(error.to_string()))?;
            execute(command, PgOutboxRepository::new(pool), &worker_config).await
        }
    }
}

async fn execute<R>(command: Command, repository: R, config: &OutboxWorkerConfig) -> Result<()>
where
    R: OutboxRepository,
{
    if let Command::Replay { event_id } = command {
        let event = repository
            .replay_dead_letter(event_id, chrono::Utc::now())
            .await?;
        println!("replayed event {} ({})", event.id, event.event_type);
        return Ok(());
    }

    let once = matches!(command, Command::Run { once: true });
    let worker = OutboxWorker::new(
        repository,
        ProbeHandler,
        config.batch_size,
        config.base_backoff_seconds,
        config.lease_seconds,
        config.delivery_timeout_seconds,
    )?;
    if once {
        let result = worker.process_once().await?;
        println!(
            "batch complete: claimed={}, delivered={}, failed={}",
            result.claimed, result.delivered, result.failed
        );
        return Ok(());
    }

    let runtime = OutboxRuntime::new(worker, config.poll_interval_ms)?;
    info!("outbox worker started");
    let stats = runtime
        .run_until(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await?;
    println!(
        "shutdown complete: batches={}, claimed={}, delivered={}, failed={}",
        stats.batches, stats.claimed, stats.delivered, stats.failed
    );
    Ok(())
}

fn parse_args(args: impl IntoIterator<Item = String>) -> Result<Command> {
    let args: Vec<String> = args.into_iter().collect();
    match args.as_slice() {
        [] => Ok(Command::Help),
        [arg] if arg == "--help" || arg == "-h" => Ok(Command::Help),
        [command] if command == "run" => Ok(Command::Run { once: false }),
        [command, flag] if command == "run" && flag == "--once" => Ok(Command::Run { once: true }),
        [command, event_id, confirm] if command == "replay" && confirm == "--confirm" => {
            let event_id = event_id.parse().map_err(|_| {
                AppError::ValidationError("replay event id must be a UUID".to_string())
            })?;
            Ok(Command::Replay { event_id })
        }
        [command, ..] if command == "replay" => Err(AppError::ValidationError(
            "replay requires <event-id> --confirm".to_string(),
        )),
        _ => Err(AppError::ValidationError(
            "invalid outbox worker command".to_string(),
        )),
    }
}

fn print_usage() {
    println!("Usage: outbox-worker run [--once] | replay <event-id> --confirm");
}

fn init_tracing(config: &AppConfig) {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&config.log.level));
    let _ = tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .try_init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replay_requires_explicit_confirmation() {
        let event_id = Uuid::new_v4();
        assert!(parse_args(["replay".to_string(), event_id.to_string()]).is_err());
        assert_eq!(
            parse_args([
                "replay".to_string(),
                event_id.to_string(),
                "--confirm".to_string()
            ])
            .expect("confirmed replay"),
            Command::Replay { event_id }
        );
    }

    #[test]
    fn only_supported_run_shapes_are_accepted() {
        assert_eq!(
            parse_args(["run".to_string()]).expect("continuous run"),
            Command::Run { once: false }
        );
        assert_eq!(
            parse_args(["run".to_string(), "--once".to_string()]).expect("one batch"),
            Command::Run { once: true }
        );
        assert!(parse_args(["run".to_string(), "--forever".to_string()]).is_err());
        assert!(parse_args([
            "replay".to_string(),
            "not-a-uuid".to_string(),
            "--confirm".to_string()
        ])
        .is_err());
    }
}
