#!/usr/bin/env bash
set -Eeuo pipefail

POSTGRES_BASE_URL="${POSTGRES_BASE_URL:-postgres://evolith:evolith@localhost:5432}"
POSTGRES_ADMIN_URL="${POSTGRES_ADMIN_URL:-$POSTGRES_BASE_URL/postgres}"
EVOLITH_BINARY="${EVOLITH_BINARY:-backend/target/debug/evolith}"
EVOLITH_APP_VERSION="${EVOLITH_APP_VERSION:-application-recovery-test}"
SOURCE_PORT="${SOURCE_PORT:-18080}"
TARGET_PORT="${TARGET_PORT:-18081}"
run_id="${RANDOM}_$$"
source_db="evolith_app_source_${run_id}"
target_db="evolith_app_target_${run_id}"
source_url="$POSTGRES_BASE_URL/$source_db"
target_url="$POSTGRES_BASE_URL/$target_db"
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
work_dir="$(mktemp -d "${TMPDIR:-/tmp}/evolith-app-recovery.XXXXXX")"
source_git="$work_dir/source-git"
target_git="$work_dir/target-git"
backup_dir="$work_dir/backups"
source_log="$work_dir/source-backend.log"
target_log="$work_dir/target-backend.log"
backend_pid=""
email="durability-${run_id}@example.invalid"
username="durability${run_id}"
password="TestPassword123!"

log() {
    printf '[application-recovery] %s\n' "$*"
}

fail() {
    printf '[application-recovery] ERROR: %s\n' "$*" >&2
    if [[ -f "$source_log" ]]; then
        printf '%s\n' '--- source backend log ---' >&2
        tail -n 80 "$source_log" >&2 || true
    fi
    if [[ -f "$target_log" ]]; then
        printf '%s\n' '--- target backend log ---' >&2
        tail -n 80 "$target_log" >&2 || true
    fi
    exit 1
}

stop_backend() {
    if [[ -n "$backend_pid" ]] && kill -0 "$backend_pid" >/dev/null 2>&1; then
        kill "$backend_pid" >/dev/null 2>&1 || true
        wait "$backend_pid" >/dev/null 2>&1 || true
    fi
    backend_pid=""
}

cleanup() {
    stop_backend
    dropdb --if-exists --force --maintenance-db="$POSTGRES_ADMIN_URL" "$source_db" >/dev/null 2>&1 || true
    dropdb --if-exists --force --maintenance-db="$POSTGRES_ADMIN_URL" "$target_db" >/dev/null 2>&1 || true
    rm -rf "$work_dir"
}
trap cleanup EXIT INT TERM

wait_for_ready() {
    local port="$1"
    local log_file="$2"
    local attempt
    for attempt in $(seq 1 90); do
        if [[ -n "$backend_pid" ]] && ! kill -0 "$backend_pid" >/dev/null 2>&1; then
            tail -n 100 "$log_file" >&2 || true
            fail "backend exited before readiness"
        fi
        if curl --silent --show-error --fail "http://127.0.0.1:${port}/health/ready" >/dev/null 2>&1; then
            return 0
        fi
        sleep 1
    done
    fail "backend did not become ready on port $port"
}

start_backend() {
    local database_url="$1"
    local git_path="$2"
    local port="$3"
    local log_file="$4"

    stop_backend
    mkdir -p "$git_path"
    (
        cd "$repo_root/backend"
        exec env \
            ENVIRONMENT=development \
            APP__PUBLIC_URL="http://127.0.0.1:${port}" \
            SERVER__HOST=127.0.0.1 \
            SERVER__PORT="$port" \
            DATABASE__DATABASE_TYPE=postgres \
            DATABASE__URL="$database_url" \
            DATABASE__SEED_DATABASE=false \
            GIT_STORAGE__BASE_PATH="$git_path" \
            JWT__SECRET=application-recovery-test-jwt-secret-32chars \
            JWT__EXPIRATION=1h \
            SANDBOX__ENABLED=false \
            SMTP__ENABLED=false \
            LOG__LEVEL=info \
            "$repo_root/$EVOLITH_BINARY"
    ) >"$log_file" 2>&1 &
    backend_pid=$!
    wait_for_ready "$port" "$log_file"
}

json_auth_fields() {
    local file="$1"
    python3 - "$file" <<'PY'
import json
import sys

with open(sys.argv[1], encoding="utf-8") as handle:
    payload = json.load(handle)
if not payload.get("success") or not payload.get("data"):
    raise SystemExit(f"authentication failed: {payload}")
data = payload["data"]
print(data["token"], data["tenant"]["id"])
PY
}

json_repo_id() {
    local file="$1"
    python3 - "$file" <<'PY'
import json
import sys

with open(sys.argv[1], encoding="utf-8") as handle:
    payload = json.load(handle)
if not payload.get("success") or not payload.get("data"):
    raise SystemExit(f"repository creation failed: {payload}")
print(payload["data"]["id"])
PY
}

assert_repo_list() {
    local file="$1"
    local expected_repo_id="$2"
    python3 - "$file" "$expected_repo_id" <<'PY'
import json
import sys

with open(sys.argv[1], encoding="utf-8") as handle:
    payload = json.load(handle)
if not payload.get("success") or not payload.get("data"):
    raise SystemExit(f"repository list failed: {payload}")
repos = payload["data"].get("repos", [])
if not any(repo.get("id") == sys.argv[2] and repo.get("name") == "durability" for repo in repos):
    raise SystemExit(f"restored repository missing from list: {payload}")
PY
}

for command_name in curl python3 psql createdb dropdb git gzip tar; do
    command -v "$command_name" >/dev/null 2>&1 || fail "required command not found: $command_name"
done
[[ -x "$repo_root/$EVOLITH_BINARY" ]] || fail "Evolith binary not found or not executable: $EVOLITH_BINARY"

createdb --maintenance-db="$POSTGRES_ADMIN_URL" "$source_db"
createdb --maintenance-db="$POSTGRES_ADMIN_URL" "$target_db"
mkdir -p "$source_git" "$target_git" "$backup_dir"

log "starting source application and creating user, tenant and repository"
start_backend "$source_url" "$source_git" "$SOURCE_PORT" "$source_log"

register_json="$work_dir/register.json"
curl --silent --show-error --fail-with-body \
    -H 'Content-Type: application/json' \
    -d "{\"email\":\"$email\",\"username\":\"$username\",\"password\":\"$password\"}" \
    "http://127.0.0.1:${SOURCE_PORT}/api/v1/auth/register" >"$register_json"
read -r source_token tenant_id < <(json_auth_fields "$register_json")
[[ -n "$source_token" && -n "$tenant_id" ]] || fail "register response did not contain auth data"

create_json="$work_dir/create-repo.json"
curl --silent --show-error --fail-with-body \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $source_token" \
    -d '{"name":"durability","description":"DATA-01 recovery drill","seed_template":false}' \
    "http://127.0.0.1:${SOURCE_PORT}/api/v1/tenant/${tenant_id}/repos" >"$create_json"
repo_id="$(json_repo_id "$create_json")"
[[ -n "$repo_id" ]] || fail "repository response did not contain an id"
remote_url="http://127.0.0.1:${SOURCE_PORT}/repos/${repo_id}"

log "pushing multiple commits, an extra branch and a tag through Smart HTTP"
work_repo="$work_dir/work-repo"
git init -b main "$work_repo" >/dev/null
git -C "$work_repo" config user.name "Application Recovery Test"
git -C "$work_repo" config user.email "application-recovery@example.invalid"
printf 'first\n' >"$work_repo/README.md"
git -C "$work_repo" add README.md
git -C "$work_repo" commit -m first >/dev/null
git -C "$work_repo" branch feature/recovery
printf 'second\n' >>"$work_repo/README.md"
git -C "$work_repo" commit -am second >/dev/null
git -C "$work_repo" tag v1.0.0
expected_sha="$(git -C "$work_repo" rev-parse HEAD)"
git -C "$work_repo" remote add origin "$remote_url"
git -C "$work_repo" -c "http.extraHeader=Authorization: Bearer $source_token" push origin --all >/dev/null
git -C "$work_repo" -c "http.extraHeader=Authorization: Bearer $source_token" push origin --tags >/dev/null

source_list_json="$work_dir/source-list.json"
curl --silent --show-error --fail-with-body \
    -H "Authorization: Bearer $source_token" \
    "http://127.0.0.1:${SOURCE_PORT}/api/v1/tenant/${tenant_id}/repos" >"$source_list_json"
assert_repo_list "$source_list_json" "$repo_id"

log "stopping writes and creating a joint PostgreSQL plus Git backup"
stop_backend
EVOLITH_BACKUP_QUIESCED=true \
DATABASE_URL="$source_url" \
GIT_STORAGE_PATH="$source_git" \
BACKUP_DIR="$backup_dir" \
EVOLITH_APP_VERSION="$EVOLITH_APP_VERSION" \
    "$repo_root/scripts/backup.sh"
archive="$(find "$backup_dir" -maxdepth 1 -type f -name 'evolith-backup-*.tar.gz' -print -quit)"
[[ -n "$archive" ]] || fail "joint backup archive was not created"

log "restoring into a new empty PostgreSQL database and Git directory"
EVOLITH_RESTORE_QUIESCED=true \
DATABASE_URL="$target_url" \
GIT_STORAGE_PATH="$target_git" \
EVOLITH_RESTORE_EXPECTED_APP_VERSION="$EVOLITH_APP_VERSION" \
    "$repo_root/scripts/restore.sh" "$archive"

log "starting restored application, logging in and listing repositories"
start_backend "$target_url" "$target_git" "$TARGET_PORT" "$target_log"
login_json="$work_dir/login.json"
curl --silent --show-error --fail-with-body \
    -H 'Content-Type: application/json' \
    -d "{\"email\":\"$email\",\"password\":\"$password\"}" \
    "http://127.0.0.1:${TARGET_PORT}/api/v1/auth/login" >"$login_json"
read -r restored_token restored_tenant_id < <(json_auth_fields "$login_json")
[[ "$restored_tenant_id" == "$tenant_id" ]] || fail "restored login returned a different tenant"

restored_list_json="$work_dir/restored-list.json"
curl --silent --show-error --fail-with-body \
    -H "Authorization: Bearer $restored_token" \
    "http://127.0.0.1:${TARGET_PORT}/api/v1/tenant/${tenant_id}/repos" >"$restored_list_json"
assert_repo_list "$restored_list_json" "$repo_id"

log "cloning restored repository and verifying Commit, Branch and Tag"
restored_remote="http://127.0.0.1:${TARGET_PORT}/repos/${repo_id}"
clone_dir="$work_dir/restored-clone"
git -c "http.extraHeader=Authorization: Bearer $restored_token" \
    clone --branch main "$restored_remote" "$clone_dir" >/dev/null
[[ "$(git -C "$clone_dir" rev-parse HEAD)" == "$expected_sha" ]] || fail "restored clone SHA mismatch"
ls_remote="$work_dir/ls-remote.txt"
git -c "http.extraHeader=Authorization: Bearer $restored_token" \
    ls-remote "$restored_remote" >"$ls_remote"
grep -Fq "$expected_sha"$'\t''refs/heads/main' "$ls_remote" || fail "restored main ref mismatch"
grep -Fq $'\t''refs/heads/feature/recovery' "$ls_remote" || fail "restored feature branch missing"
grep -Fq $'\t''refs/tags/v1.0.0' "$ls_remote" || fail "restored tag missing"

log "injecting an actual read-only Git Storage readiness failure"
chmod 500 "$target_git"
read_only_status="$(curl --silent --show-error -o "$work_dir/read-only-ready.json" -w '%{http_code}' \
    "http://127.0.0.1:${TARGET_PORT}/health/ready")"
[[ "$read_only_status" == "503" ]] || fail "read-only Git storage did not return readiness 503"
chmod 700 "$target_git"
wait_for_ready "$TARGET_PORT" "$target_log"

log "restarting Backend against restored data and cloning again"
stop_backend
start_backend "$target_url" "$target_git" "$TARGET_PORT" "$target_log"
second_login_json="$work_dir/second-login.json"
curl --silent --show-error --fail-with-body \
    -H 'Content-Type: application/json' \
    -d "{\"email\":\"$email\",\"password\":\"$password\"}" \
    "http://127.0.0.1:${TARGET_PORT}/api/v1/auth/login" >"$second_login_json"
read -r second_token second_tenant_id < <(json_auth_fields "$second_login_json")
[[ "$second_tenant_id" == "$tenant_id" ]] || fail "tenant changed after Backend restart"
second_clone="$work_dir/restarted-clone"
git -c "http.extraHeader=Authorization: Bearer $second_token" \
    clone --branch main "http://127.0.0.1:${TARGET_PORT}/repos/${repo_id}" "$second_clone" >/dev/null
[[ "$(git -C "$second_clone" rev-parse HEAD)" == "$expected_sha" ]] || fail \
    "clone SHA changed after Backend restart"

DATABASE_URL="$target_url" GIT_STORAGE_PATH="$target_git" \
    "$repo_root/scripts/git-storage-inventory.sh"

log "application-level recovery drill passed: login, list, clone, refs, readiness and Backend restart"
