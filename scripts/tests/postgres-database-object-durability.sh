#!/usr/bin/env bash
set -Eeuo pipefail

POSTGRES_BASE_URL="${POSTGRES_BASE_URL:-postgres://evolith:evolith@localhost:5432}"
POSTGRES_ADMIN_URL="${POSTGRES_ADMIN_URL:-$POSTGRES_BASE_URL/postgres}"
run_id="${RANDOM}_$$"
source_db="evolith_database_object_source_${run_id}"
target_db="evolith_database_object_target_${run_id}"
reject_db="evolith_database_object_reject_${run_id}"
source_url="$POSTGRES_BASE_URL/$source_db"
target_url="$POSTGRES_BASE_URL/$target_db"
reject_url="$POSTGRES_BASE_URL/$reject_db"
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
database_empty_query="$repo_root/scripts/postgres-user-object-count.sql"
work_dir="$(mktemp -d "${TMPDIR:-/tmp}/evolith-database-object-test.XXXXXX")"
source_git="$work_dir/source-git"
target_git="$work_dir/target-git"
reject_git="$work_dir/reject-git"
backup_dir="$work_dir/backups"
package_dir="$work_dir/package"
tenant_id="66666666-6666-4666-8666-666666666666"
repo_id="77777777-7777-4777-8777-777777777777"
storage_path="$tenant_id/$repo_id.git"
publication_name="evolith_database_level_publication"
legacy_publication_name="legacy_publication"

log() {
    printf '[database-object-test] %s\n' "$*"
}

fail() {
    printf '[database-object-test] ERROR: %s\n' "$*" >&2
    exit 1
}

cleanup() {
    dropdb --if-exists --force --maintenance-db="$POSTGRES_ADMIN_URL" "$source_db" >/dev/null 2>&1 || true
    dropdb --if-exists --force --maintenance-db="$POSTGRES_ADMIN_URL" "$target_db" >/dev/null 2>&1 || true
    dropdb --if-exists --force --maintenance-db="$POSTGRES_ADMIN_URL" "$reject_db" >/dev/null 2>&1 || true
    rm -rf "$work_dir"
}
trap cleanup EXIT INT TERM

recreate_reject_target() {
    dropdb --if-exists --force --maintenance-db="$POSTGRES_ADMIN_URL" "$reject_db" >/dev/null 2>&1 || true
    createdb --maintenance-db="$POSTGRES_ADMIN_URL" "$reject_db"
    rm -rf "$reject_git"
    mkdir -p "$reject_git"
}

database_user_object_count() {
    local database_url="$1"
    psql "$database_url" -v ON_ERROR_STOP=1 -Atq -f "$database_empty_query"
}

publication_exists() {
    local database_url="$1"
    local name="$2"
    psql "$database_url" -v ON_ERROR_STOP=1 -Atq \
        -v publication_name="$name" \
        -c "SELECT EXISTS (SELECT 1 FROM pg_catalog.pg_publication WHERE pubname = :'publication_name')"
}

assert_no_restore_success() {
    local output="$1"
    if grep -Fq "restore completed successfully" "$output"; then
        fail "failed restore printed a success message"
    fi
}

assert_git_empty() {
    local git_path="$1"
    if find "$git_path" -mindepth 1 -print -quit | grep -q .; then
        fail "failed restore left Git data behind"
    fi
}

rewrite_checksums() {
    local dir="$1"
    (
        cd "$dir"
        sha256sum database.sql.gz git-storage.tar.gz git-refs.tsv manifest.env >SHA256SUMS
    )
}

make_inventory_failure_archive() {
    local source_archive="$1"
    local output_archive="$2"

    rm -rf "$package_dir"
    mkdir -p "$package_dir"
    tar -C "$package_dir" -xzf "$source_archive"
    gzip -dc "$package_dir/database.sql.gz" >"$package_dir/database.sql"
    cat >>"$package_dir/database.sql" <<'SQL'
INSERT INTO public.git_repos (
    id, tenant_id, name, description, default_branch, storage_path,
    visibility, auto_merge, require_review, created_at, updated_at
) VALUES (
    '88888888-8888-4888-8888-888888888888',
    '66666666-6666-4666-8666-666666666666',
    'orphan-publication-test', '', 'main',
    '66666666-6666-4666-8666-666666666666/88888888-8888-4888-8888-888888888888.git',
    'private', false, true, NOW(), NOW()
);
SQL
    gzip -c "$package_dir/database.sql" >"$package_dir/database.sql.gz"
    rm -f "$package_dir/database.sql"
    rewrite_checksums "$package_dir"
    tar -C "$package_dir" -czf "$output_archive" .
}

for command_name in psql pg_dump createdb dropdb git gzip tar sha256sum grep find; do
    command -v "$command_name" >/dev/null 2>&1 || fail "required command not found: $command_name"
done
[[ -f "$database_empty_query" && ! -L "$database_empty_query" ]] || fail \
    "database emptiness query is missing or unsafe"

log "creating source and empty restore targets"
createdb --maintenance-db="$POSTGRES_ADMIN_URL" "$source_db"
createdb --maintenance-db="$POSTGRES_ADMIN_URL" "$target_db"
createdb --maintenance-db="$POSTGRES_ADMIN_URL" "$reject_db"
mkdir -p "$source_git" "$target_git" "$reject_git" "$backup_dir"

psql "$source_url" -v ON_ERROR_STOP=1 <<SQL
CREATE TABLE public.durable_marker (value TEXT NOT NULL);
INSERT INTO public.durable_marker(value) VALUES ('database-level-restored');
CREATE TABLE public.git_repos (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    default_branch TEXT NOT NULL DEFAULT 'main',
    storage_path TEXT NOT NULL,
    visibility TEXT NOT NULL DEFAULT 'private',
    auto_merge BOOLEAN NOT NULL DEFAULT FALSE,
    require_review BOOLEAN NOT NULL DEFAULT TRUE,
    last_commit_sha TEXT,
    last_committed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE PUBLICATION "$publication_name";
SQL

log "creating a matching bare Git repository"
mkdir -p "$source_git/$tenant_id"
git init --bare "$source_git/$storage_path" >/dev/null
work_repo="$work_dir/work"
git init -b main "$work_repo" >/dev/null
git -C "$work_repo" config user.name "Database Object Test"
git -C "$work_repo" config user.email "database-object@example.invalid"
printf 'database-level object durability\n' >"$work_repo/README.md"
git -C "$work_repo" add README.md
git -C "$work_repo" commit -m "database object fixture" >/dev/null
main_sha="$(git -C "$work_repo" rev-parse HEAD)"
git -C "$work_repo" remote add origin "$source_git/$storage_path"
git -C "$work_repo" push origin main >/dev/null
git --git-dir="$source_git/$storage_path" symbolic-ref HEAD refs/heads/main

psql "$source_url" -v ON_ERROR_STOP=1 -v tenant_id="$tenant_id" -v repo_id="$repo_id" \
    -v storage_path="$storage_path" -v main_sha="$main_sha" <<'SQL'
INSERT INTO public.git_repos (
    id, tenant_id, name, default_branch, storage_path, last_commit_sha
) VALUES (
    :'repo_id', :'tenant_id', 'database-object-durability', 'main', :'storage_path', :'main_sha'
);
SQL

log "creating a production backup that contains the Publication"
EVOLITH_BACKUP_QUIESCED=true DATABASE_URL="$source_url" GIT_STORAGE_PATH="$source_git" \
BACKUP_DIR="$backup_dir" EVOLITH_APP_VERSION="database-object-test-head" \
    "$repo_root/scripts/backup.sh" >/dev/null
archive="$(find "$backup_dir" -maxdepth 1 -type f -name 'evolith-backup-*.tar.gz' -print -quit)"
[[ -n "$archive" ]] || fail "backup archive was not created"
rm -rf "$package_dir" && mkdir -p "$package_dir"
tar -C "$package_dir" -xzf "$archive"
gzip -dc "$package_dir/database.sql.gz" | grep -Fq "CREATE PUBLICATION $publication_name" || \
    fail "production database dump did not contain the Publication"

log "proving the Publication is restored with the joint backup"
EVOLITH_RESTORE_QUIESCED=true DATABASE_URL="$target_url" GIT_STORAGE_PATH="$target_git" \
EVOLITH_RESTORE_EXPECTED_APP_VERSION="database-object-test-head" \
    "$repo_root/scripts/restore.sh" "$archive"
[[ "$(publication_exists "$target_url" "$publication_name")" == "t" ]] || \
    fail "Publication was not restored"
[[ "$(psql "$target_url" -Atq -c 'SELECT value FROM public.durable_marker')" == "database-level-restored" ]] || \
    fail "database marker was not restored"
[[ "$(git --git-dir="$target_git/$storage_path" rev-parse refs/heads/main)" == "$main_sha" ]] || \
    fail "Git commit was not restored"

log "proving a Publication-only target is rejected before writes"
recreate_reject_target
psql "$reject_url" -v ON_ERROR_STOP=1 -c "CREATE PUBLICATION $legacy_publication_name" >/dev/null
[[ "$(database_user_object_count "$reject_url")" != "0" ]] || \
    fail "shared helper treated a Publication-only target as empty"
publication_reject_output="$work_dir/publication-only-reject.log"
if EVOLITH_RESTORE_QUIESCED=true DATABASE_URL="$reject_url" GIT_STORAGE_PATH="$reject_git" \
    "$repo_root/scripts/restore.sh" "$archive" >"$publication_reject_output" 2>&1; then
    fail "restore unexpectedly accepted a Publication-only target"
fi
grep -Fq "target database is not empty" "$publication_reject_output" || \
    fail "Publication-only target did not reach the database emptiness gate"
if grep -Fq "restoring PostgreSQL into empty target" "$publication_reject_output"; then
    fail "Publication-only target was rejected after writes started"
fi
assert_no_restore_success "$publication_reject_output"
[[ "$(publication_exists "$reject_url" "$legacy_publication_name")" == "t" ]] || \
    fail "Publication-only rejection did not preserve the existing Publication"
[[ "$(database_user_object_count "$reject_url")" != "0" ]] || \
    fail "shared helper reported the preserved Publication target as empty"
[[ "$(psql "$reject_url" -Atq -c "SELECT to_regclass('public.durable_marker') IS NULL")" == "t" ]] || \
    fail "Publication-only rejection wrote schema objects"
assert_git_empty "$reject_git"

log "proving post-write failure removes the restored Publication and all other state"
recreate_reject_target
inventory_failure_archive="$work_dir/publication-inventory-failure.tar.gz"
make_inventory_failure_archive "$archive" "$inventory_failure_archive"
inventory_failure_output="$work_dir/publication-inventory-failure.log"
if EVOLITH_RESTORE_QUIESCED=true DATABASE_URL="$reject_url" GIT_STORAGE_PATH="$reject_git" \
    "$repo_root/scripts/restore.sh" "$inventory_failure_archive" >"$inventory_failure_output" 2>&1; then
    fail "restore unexpectedly reported success after Publication post-write failure"
fi
grep -Fq "restoring PostgreSQL into empty target" "$inventory_failure_output" || \
    fail "Publication rollback test did not restore the database"
grep -Fq "running DB/Git inventory and ref verification" "$inventory_failure_output" || \
    fail "Publication rollback test did not reach post-write inventory"
grep -Fq "reverting the previously empty target" "$inventory_failure_output" || \
    fail "Publication rollback test did not invoke rollback-to-empty"
assert_no_restore_success "$inventory_failure_output"
[[ "$(publication_exists "$reject_url" "$publication_name")" == "f" ]] || \
    fail "rollback left the restored Publication behind"
[[ "$(database_user_object_count "$reject_url")" == "0" ]] || \
    fail "shared helper did not report zero after Publication rollback"
[[ "$(psql "$reject_url" -Atq -c "SELECT to_regclass('public.durable_marker') IS NULL")" == "t" ]] || \
    fail "rollback left schema state behind"
assert_git_empty "$reject_git"

log "all database-level PostgreSQL object durability tests passed"
