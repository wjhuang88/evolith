//! Health check handlers

use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;

use crate::state::AppState;

const GIT_STORAGE_PROBE_BYTES: &[u8] = b"evolith-git-storage-readiness-v1\n";

#[derive(Serialize)]
struct HealthCheckResponse {
    status: &'static str,
    version: &'static str,
}

#[derive(Serialize)]
struct LivenessResponse {
    status: &'static str,
    version: &'static str,
}

#[derive(Serialize)]
struct ReadinessResponse {
    status: &'static str,
    version: &'static str,
    checks: HealthChecks,
}

#[derive(Serialize)]
struct HealthChecks {
    database: bool,
    git_storage: bool,
}

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(HealthCheckResponse {
        status: "healthy",
        version: env!("CARGO_PKG_VERSION"),
    })
}

pub async fn liveness() -> impl Responder {
    HttpResponse::Ok().json(LivenessResponse {
        status: "ok",
        version: env!("CARGO_PKG_VERSION"),
    })
}

pub async fn readiness(state: web::Data<AppState>) -> impl Responder {
    let db_healthy = check_database(&state).await;
    let git_storage_path = state.git_storage_base_path.clone();
    let git_storage_healthy = web::block(move || check_git_storage(Path::new(&git_storage_path)))
        .await
        .unwrap_or(false);

    let healthy = db_healthy && git_storage_healthy;
    let response = ReadinessResponse {
        status: if healthy { "healthy" } else { "unavailable" },
        version: env!("CARGO_PKG_VERSION"),
        checks: HealthChecks {
            database: db_healthy,
            git_storage: git_storage_healthy,
        },
    };

    HttpResponse::build(readiness_status(db_healthy, git_storage_healthy)).json(response)
}

fn readiness_status(database_healthy: bool, git_storage_healthy: bool) -> StatusCode {
    if database_healthy && git_storage_healthy {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    }
}

async fn check_database(state: &AppState) -> bool {
    state
        .user_repo
        .find_by_id(uuid::Uuid::nil())
        .await
        .map(|_| true)
        .unwrap_or(false)
}

fn check_git_storage(base_path: &Path) -> bool {
    check_git_storage_with_readback(base_path, read_probe_exact)
}

fn check_git_storage_with_readback<F>(base_path: &Path, readback: F) -> bool
where
    F: FnOnce(&Path, &[u8]) -> io::Result<()>,
{
    let metadata = match fs::metadata(base_path) {
        Ok(metadata) => metadata,
        Err(_) => return false,
    };
    if !metadata.is_dir() || fs::read_dir(base_path).is_err() {
        return false;
    }

    let probe_dir = base_path.join(format!(".evolith-readiness-{}", uuid::Uuid::new_v4()));
    if fs::create_dir(&probe_dir).is_err() {
        return false;
    }

    let probe_file = probe_dir.join("probe");
    let probe_result = (|| -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&probe_file)?;
        file.write_all(GIT_STORAGE_PROBE_BYTES)?;
        file.sync_all()?;
        drop(file);

        readback(&probe_file, GIT_STORAGE_PROBE_BYTES)
    })();

    let cleanup_result = cleanup_probe(&probe_file, &probe_dir);
    probe_result.is_ok() && cleanup_result.is_ok()
}

fn read_probe_exact(probe_file: &Path, expected: &[u8]) -> io::Result<()> {
    let mut file = File::open(probe_file)?;
    let mut actual = Vec::new();
    file.read_to_end(&mut actual)?;
    if actual == expected {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Git storage readiness probe readback mismatch",
        ))
    }
}

fn cleanup_probe(probe_file: &Path, probe_dir: &Path) -> io::Result<()> {
    let file_result = remove_file_if_present(probe_file);
    let directory_result = remove_dir_if_present(probe_dir);
    file_result.and(directory_result)
}

fn remove_file_if_present(path: &Path) -> io::Result<()> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error),
    }
}

fn remove_dir_if_present(path: &Path) -> io::Result<()> {
    match fs::remove_dir(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        check_git_storage, check_git_storage_with_readback, read_probe_exact, readiness_status,
        GIT_STORAGE_PROBE_BYTES,
    };
    use actix_web::http::StatusCode;
    use std::fs;

    fn assert_probe_artifacts_cleaned(path: &std::path::Path) {
        assert!(
            fs::read_dir(path)
                .expect("read tempdir")
                .next()
                .is_none(),
            "readiness probe must clean up its temporary artifacts"
        );
    }

    #[test]
    fn readiness_returns_503_when_database_is_unavailable() {
        assert_eq!(
            readiness_status(false, true),
            StatusCode::SERVICE_UNAVAILABLE
        );
    }

    #[test]
    fn readiness_returns_503_when_git_storage_is_unavailable() {
        assert_eq!(
            readiness_status(true, false),
            StatusCode::SERVICE_UNAVAILABLE
        );
    }

    #[test]
    fn readiness_returns_200_only_when_database_and_git_storage_are_healthy() {
        assert_eq!(readiness_status(true, true), StatusCode::OK);
    }

    #[test]
    fn git_storage_readiness_rejects_missing_path() {
        let temp = tempfile::tempdir().expect("tempdir");
        let missing = temp.path().join("missing");

        assert!(!check_git_storage(&missing));
    }

    #[test]
    fn git_storage_readiness_rejects_regular_file() {
        let temp = tempfile::tempdir().expect("tempdir");
        let file = temp.path().join("repos");
        fs::write(&file, b"not a directory").expect("write fixture");

        assert!(!check_git_storage(&file));
    }

    #[test]
    fn git_storage_readiness_reopens_reads_compares_and_cleans_up() {
        let temp = tempfile::tempdir().expect("tempdir");

        assert!(check_git_storage(temp.path()));
        assert_probe_artifacts_cleaned(temp.path());
    }

    #[test]
    fn git_storage_readback_requires_exact_probe_bytes() {
        let temp = tempfile::tempdir().expect("tempdir");
        let probe = temp.path().join("probe");
        fs::write(&probe, GIT_STORAGE_PROBE_BYTES).expect("write exact probe");

        assert!(read_probe_exact(&probe, GIT_STORAGE_PROBE_BYTES).is_ok());

        fs::write(&probe, b"evolith-git-storage-readiness-v1")
            .expect("write truncated probe");
        assert!(read_probe_exact(&probe, GIT_STORAGE_PROBE_BYTES).is_err());
    }

    #[test]
    fn git_storage_readiness_rejects_readback_mismatch_and_cleans_up() {
        let temp = tempfile::tempdir().expect("tempdir");

        let ready = check_git_storage_with_readback(temp.path(), |probe_file, expected| {
            fs::write(probe_file, b"corrupted")?;
            read_probe_exact(probe_file, expected)
        });

        assert!(!ready);
        assert_probe_artifacts_cleaned(temp.path());
    }

    #[test]
    fn git_storage_readiness_rejects_read_error_and_cleans_up() {
        let temp = tempfile::tempdir().expect("tempdir");

        let ready = check_git_storage_with_readback(temp.path(), |probe_file, expected| {
            fs::remove_file(probe_file)?;
            read_probe_exact(probe_file, expected)
        });

        assert!(!ready);
        assert_probe_artifacts_cleaned(temp.path());
    }

    #[cfg(unix)]
    #[test]
    fn git_storage_readiness_rejects_unwritable_directory() {
        use std::os::unix::fs::PermissionsExt;

        let temp = tempfile::tempdir().expect("tempdir");
        let original = fs::metadata(temp.path()).expect("metadata").permissions();
        let mut read_only = original.clone();
        read_only.set_mode(0o500);
        fs::set_permissions(temp.path(), read_only).expect("set read-only permissions");

        let ready = check_git_storage(temp.path());

        fs::set_permissions(temp.path(), original).expect("restore permissions");
        assert!(!ready);
    }
}
