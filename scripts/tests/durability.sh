#!/usr/bin/env bash
set -Eeuo pipefail

POSTGRES_BASE_URL="${POSTGRES_BASE_URL:-postgres://evolith:evolith@localhost:5432}"
POSTGRES_ADMIN_URL="${POSTGRES_ADMIN_URL:-$POSTGRES_BASE_URL/postgres}"
run_id="${RANDOM}_$$"
source_db="evolith_durability_source_${run_id}"
target_db="evolith_durability_target_${run_id}"
reject_db="evolith_durability_reject_${run_id}"
source_url="$POSTGRES_BASE_URL/$source_db"
target_url="$POSTGRES_BASE_URL/$target_db"
reject_url="$POSTGRES_BASE_URL/$reject_db"
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
work_dir="$(mktemp -d "${TMPDIR:-/tmp}/evolith-durability-test.XXXXXX")"
source_git="$work_dir/source-git"
target_git="$work_dir/target-git"
reject_git="$work_dir/reject-git"
backup_dir="$work_dir/backups"
package_dir="$work_dir/package"
tenant_id="11111111-1111-4111-8111-111111111111"
repo_id="22222222-2222-4222-8222-222222222222"
storage_path="$tenant_id/$repo_id.git"

log() {
    printf '[durability-test] %s\n' "$*"
}

fail() {
    printf '[durability-test] ERROR: %s\n' "$*" >&2
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

assert_reject_target_empty() {
    local objects
    objects="$(psql "$reject_url" -At -c \
        "SELECT
           (SELECT COUNT(*) FROM pg_catalog.pg_class c JOIN pg_catalog.pg_namespace n ON n.oid = c.relnamespace
            WHERE n.nspname = 'public' AND c.relkind IN ('r','p','v','m','S','f'))
           +
           (SELECT COUNT(*) FROM pg_catalog.pg_proc p JOIN pg_catalog.pg_namespace n ON n.oid = p.pronamespace
            WHERE n.nspname = 'public')
           +
           (SELECT COUNT(*) FROM pg_catalog.pg_type t JOIN pg_catalog.pg_namespace n ON n.oid = t.typnamespace
            WHERE n.nspname = 'public' AND t.typisdefined)")"
    [[ "$objects" == "0" ]] || fail "failed restore left database objects behind"
    if find "$reject_git" -mindepth 1 -print -quit | grep -q .; then
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

rewrite_git_archive() {
    local dir="$1"
    local mutation="$2"
    local git_payload="$dir/git-payload"
    rm -rf "$git_payload"
    mkdir -p "$git_payload"
    tar -C "$git_payload" -xzf "$dir/git-storage.tar.gz"
    "$mutation" "$git_payload"
    tar -C "$git_payload" -czf "$dir/git-storage.tar.gz" .
    rm -rf "$git_payload"
    rewrite_checksums "$dir"
}

make_modified_archive() {
    local source_archive="$1"
    local output_archive="$2"
    local mutation="$3"
    rm -rf "$package_dir"
    mkdir -p "$package_dir"
    tar -C "$package_dir" -xzf "$source_archive"
    "$mutation" "$package_dir"
    tar -C "$package_dir" -czf "$output_archive" .
}

mutation_remove_database() {
    rm -f "$1/database.sql.gz"
}

mutation_remove_git() {
    rm -f "$1/git-storage.tar.gz"
}

mutation_checksum_mismatch() {
    printf 'tampered\n' >>"$1/git-refs.tsv"
}

mutation_duplicate_checksum_entry() {
    local dir="$1"
    (
        cd "$dir"
        sha256sum database.sql.gz database.sql.gz git-storage.tar.gz manifest.env >SHA256SUMS
    )
}

mutation_version_mismatch() {
    sed -i 's/^backup_format_version=.*/backup_format_version=999/' "$1/manifest.env"
    rewrite_checksums "$1"
}

mutation_missing_manifest_key() {
    sed -i '/^git_storage_layout=/d' "$1/manifest.env"
    rewrite_checksums "$1"
}

mutation_layout_mismatch() {
    sed -i 's#^git_storage_layout=.*#git_storage_layout=tenant-name/repo-name.git#' "$1/manifest.env"
    rewrite_checksums "$1"
}

mutation_invalid_created_at() {
    sed -i 's/^created_at=.*/created_at=not-a-timestamp/' "$1/manifest.env"
    rewrite_checksums "$1"
}

mutation_incomplete_ref_inventory() {
    sed -i '$d' "$1/git-refs.tsv"
    rewrite_checksums "$1"
}

mutation_outer_symlink() {
    local dir="$1"
    rm -f "$dir/manifest.env"
    ln -s /etc/passwd "$dir/manifest.env"
}

add_inner_symlink() {
    local git_payload="$1"
    ln -s /etc/passwd "$git_payload/$tenant_id/unsafe-link"
}

mutation_inner_symlink() {
    rewrite_git_archive "$1" add_inner_symlink
}

add_invalid_layout() {
    local git_payload="$1"
    mkdir -p "$git_payload/not-a-tenant"
    printf 'unexpected\n' >"$git_payload/not-a-tenant/file"
}

mutation_invalid_git_layout() {
    rewrite_git_archive "$1" add_invalid_layout
}

mutation_database_sql_failure() {
    local dir="$1"
    gzip -dc "$dir/database.sql.gz" >"$dir/database.sql"
    cat >>"$dir/database.sql" <<'SQL'
CREATE TABLE should_be_rolled_back (value TEXT);
THIS IS NOT VALID SQL;
SQL
    gzip -c "$dir/database.sql" >"$dir/database.sql.gz"
    rm -f "$dir/database.sql"
    rewrite_checksums "$dir"
}

mutation_inventory_failure() {
    local dir="$1"
    gzip -dc "$dir/database.sql.gz" >"$dir/database.sql"
    cat >>"$dir/database.sql" <<'SQL'
INSERT INTO git_repos (
    id, tenant_id, name, description, default_branch, storage_path,
    visibility, auto_merge, require_review, created_at, updated_at
) VALUES (
    '33333333-3333-4333-8333-333333333333',
    '11111111-1111-4111-8111-111111111111',
    'orphan', '', 'main',
    '11111111-1111-4111-8111-111111111111/33333333-3333-4333-8333-333333333333.git',
    'private', false, true, NOW(), NOW()
);
SQL
    gzip -c "$dir/database.sql" >"$dir/database.sql.gz"
    rm -f "$dir/database.sql"
    rewrite_checksums "$dir"
}

for command_name in psql pg_dump createdb dropdb git gzip tar sha256sum mkfifo; do
    command -v "$command_name" >/dev/null 2>&1 || fail "required command not found: $command_name"
done

log "creating PostgreSQL source and empty restore targets"
createdb --maintenance-db="$POSTGRES_ADMIN_URL" "$source_db"
createdb --maintenance-db="$POSTGRES_ADMIN_URL" "$target_db"
createdb --maintenance-db="$POSTGRES_ADMIN_URL" "$reject_db"
mkdir -p "$source_git" "$target_git" "$reject_git" "$backup_dir"

psql "$source_url" -v ON_ERROR_STOP=1 <<'SQL'
CREATE TABLE durable_marker (value TEXT NOT NULL);
INSERT INTO durable_marker(value) VALUES ('postgres-restored');
CREATE TABLE git_repos (
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
SQL

log "creating real Git history with commit, branch and tag"
mkdir -p "$source_git/$tenant_id"
git init --bare "$source_git/$storage_path" >/dev/null
work_repo="$work_dir/work"
git init -b main "$work_repo" >/dev/null
git -C "$work_repo" config user.name "Durability Test"
git -C "$work_repo" config user.email "durability@example.invalid"
printf 'first\n' >"$work_repo/README.md"
git -C "$work_repo" add README.md
git -C "$work_repo" commit -m "first" >/dev/null
git -C "$work_repo" branch feature/recovery
printf 'second\n' >>"$work_repo/README.md"
git -C "$work_repo" commit -am "second" >/dev/null
git -C "$work_repo" tag v1.0.0
main_sha="$(git -C "$work_repo" rev-parse HEAD)"
git -C "$work_repo" remote add origin "$source_git/$storage_path"
git -C "$work_repo" push origin --all >/dev/null
git -C "$work_repo" push origin --tags >/dev/null
git --git-dir="$source_git/$storage_path" symbolic-ref HEAD refs/heads/main

psql "$source_url" -v ON_ERROR_STOP=1 -v tenant_id="$tenant_id" -v repo_id="$repo_id" \
    -v storage_path="$storage_path" -v main_sha="$main_sha" <<'SQL'
INSERT INTO git_repos (
    id, tenant_id, name, default_branch, storage_path, last_commit_sha
) VALUES (
    :'repo_id', :'tenant_id', 'durability', 'main', :'storage_path', :'main_sha'
);
SQL

log "proving backup refuses an unconfirmed consistency window"
if DATABASE_URL="$source_url" GIT_STORAGE_PATH="$source_git" BACKUP_DIR="$backup_dir" \
    "$repo_root/scripts/backup.sh" >/dev/null 2>&1; then
    fail "backup unexpectedly succeeded without EVOLITH_BACKUP_QUIESCED=true"
fi

log "proving backup requires a traceable application version"
if EVOLITH_BACKUP_QUIESCED=true DATABASE_URL="$source_url" GIT_STORAGE_PATH="$source_git" \
    BACKUP_DIR="$backup_dir" "$repo_root/scripts/backup.sh" >/dev/null 2>&1; then
    fail "backup unexpectedly succeeded without EVOLITH_APP_VERSION"
fi

log "proving backup rejects unsupported special files instead of producing an unrestorable archive"
unsafe_fifo="$source_git/$storage_path/unsafe-fifo"
mkfifo "$unsafe_fifo"
if EVOLITH_BACKUP_QUIESCED=true DATABASE_URL="$source_url" GIT_STORAGE_PATH="$source_git" \
    BACKUP_DIR="$backup_dir" EVOLITH_APP_VERSION="test-head" \
    "$repo_root/scripts/backup.sh" >/dev/null 2>&1; then
    fail "backup unexpectedly accepted a FIFO in Git storage"
fi
rm -f "$unsafe_fifo"
if find "$backup_dir" -maxdepth 1 -type f -name 'evolith-backup-*.tar.gz' -print -quit | grep -q .; then
    fail "failed special-file backup left a publishable archive"
fi

log "creating joint PostgreSQL + Git backup"
backup_output="$work_dir/backup-output.log"
EVOLITH_BACKUP_QUIESCED=true DATABASE_URL="$source_url" GIT_STORAGE_PATH="$source_git" \
BACKUP_DIR="$backup_dir" EVOLITH_APP_VERSION="test-head" \
    "$repo_root/scripts/backup.sh" | tee "$backup_output"
archive="$(find "$backup_dir" -maxdepth 1 -type f -name 'evolith-backup-*.tar.gz' -print -quit)"
[[ -n "$archive" ]] || fail "backup archive was not created"
! grep -Fq "$source_url" "$backup_output" || fail "backup log leaked DATABASE_URL"
rm -rf "$package_dir" && mkdir -p "$package_dir"
tar -C "$package_dir" -xzf "$archive"
! grep -Fq "$source_url" "$package_dir/manifest.env" || fail "manifest leaked DATABASE_URL"

log "restoring into an empty PostgreSQL database and Git directory"
EVOLITH_RESTORE_QUIESCED=true DATABASE_URL="$target_url" GIT_STORAGE_PATH="$target_git" \
EVOLITH_RESTORE_EXPECTED_APP_VERSION="test-head" \
    "$repo_root/scripts/restore.sh" "$archive"
[[ "$(psql "$target_url" -At -c 'SELECT value FROM durable_marker')" == "postgres-restored" ]] || \
    fail "PostgreSQL marker was not restored"
[[ "$(git --git-dir="$target_git/$storage_path" rev-parse refs/heads/main)" == "$main_sha" ]] || \
    fail "main branch SHA mismatch after restore"
git --git-dir="$target_git/$storage_path" show-ref --verify --quiet refs/heads/feature/recovery || \
    fail "feature branch missing after restore"
git --git-dir="$target_git/$storage_path" show-ref --verify --quiet refs/tags/v1.0.0 || \
    fail "tag missing after restore"
DATABASE_URL="$target_url" GIT_STORAGE_PATH="$target_git" \
    "$repo_root/scripts/git-storage-inventory.sh"

log "proving non-empty targets are rejected"
if EVOLITH_RESTORE_QUIESCED=true DATABASE_URL="$target_url" GIT_STORAGE_PATH="$target_git" \
    "$repo_root/scripts/restore.sh" "$archive" >/dev/null 2>&1; then
    fail "restore unexpectedly overwrote a non-empty target"
fi

log "proving incomplete, corrupt, malicious and incompatible backups are rejected before writes"
for mutation in \
    mutation_remove_database \
    mutation_remove_git \
    mutation_checksum_mismatch \
    mutation_duplicate_checksum_entry \
    mutation_version_mismatch \
    mutation_missing_manifest_key \
    mutation_layout_mismatch \
    mutation_invalid_created_at \
    mutation_incomplete_ref_inventory \
    mutation_outer_symlink \
    mutation_inner_symlink \
    mutation_invalid_git_layout; do
    recreate_reject_target
    bad_archive="$work_dir/${mutation}.tar.gz"
    make_modified_archive "$archive" "$bad_archive" "$mutation"
    if EVOLITH_RESTORE_QUIESCED=true DATABASE_URL="$reject_url" GIT_STORAGE_PATH="$reject_git" \
        "$repo_root/scripts/restore.sh" "$bad_archive" >/dev/null 2>&1; then
        fail "$mutation archive unexpectedly restored"
    fi
    assert_reject_target_empty
done

log "proving a database restore error leaves the previously empty target empty"
recreate_reject_target
sql_failure_archive="$work_dir/database-sql-failure.tar.gz"
make_modified_archive "$archive" "$sql_failure_archive" mutation_database_sql_failure
if EVOLITH_RESTORE_QUIESCED=true DATABASE_URL="$reject_url" GIT_STORAGE_PATH="$reject_git" \
    "$repo_root/scripts/restore.sh" "$sql_failure_archive" >/dev/null 2>&1; then
    fail "restore unexpectedly reported success after a SQL failure"
fi
assert_reject_target_empty

log "proving a post-write inventory failure rolls the empty target back"
recreate_reject_target
mid_failure_archive="$work_dir/mid-failure.tar.gz"
make_modified_archive "$archive" "$mid_failure_archive" mutation_inventory_failure
if EVOLITH_RESTORE_QUIESCED=true DATABASE_URL="$reject_url" GIT_STORAGE_PATH="$reject_git" \
    "$repo_root/scripts/restore.sh" "$mid_failure_archive" >/dev/null 2>&1; then
    fail "restore unexpectedly reported success after inventory failure"
fi
assert_reject_target_empty

log "proving DB-only, disk-only and invalid-layout inventory mismatches return non-zero"
mv "$target_git/$storage_path" "$work_dir/saved-repo.git"
if DATABASE_URL="$target_url" GIT_STORAGE_PATH="$target_git" \
    "$repo_root/scripts/git-storage-inventory.sh" >/dev/null 2>&1; then
    fail "inventory unexpectedly accepted DB-only repository"
fi
mkdir -p "$target_git/$tenant_id"
mv "$work_dir/saved-repo.git" "$target_git/$storage_path"
orphan_id="44444444-4444-4444-8444-444444444444"
git init --bare "$target_git/$tenant_id/$orphan_id.git" >/dev/null
if DATABASE_URL="$target_url" GIT_STORAGE_PATH="$target_git" \
    "$repo_root/scripts/git-storage-inventory.sh" >/dev/null 2>&1; then
    fail "inventory unexpectedly accepted disk-only repository"
fi
rm -rf "$target_git/$tenant_id/$orphan_id.git"
mkdir -p "$target_git/not-a-uuid"
if DATABASE_URL="$target_url" GIT_STORAGE_PATH="$target_git" \
    "$repo_root/scripts/git-storage-inventory.sh" >/dev/null 2>&1; then
    fail "inventory unexpectedly accepted invalid Git storage layout"
fi
rm -rf "$target_git/not-a-uuid"
DATABASE_URL="$target_url" GIT_STORAGE_PATH="$target_git" \
    "$repo_root/scripts/git-storage-inventory.sh" >/dev/null

log "all durability tests passed"
