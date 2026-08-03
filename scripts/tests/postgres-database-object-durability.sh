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
subscription_name="evolith_sensitive_subscription"
legacy_subscription_name="legacy_sensitive_subscription"
subscription_secret="evolith-subscription-secret-${run_id}"
hostile_psqlrc="$work_dir/hostile.psqlrc"

log() {
    printf '[database-object-test] %s\n' "$*"
}

fail() {
    printf '[database-object-test] ERROR: %s\n' "$*" >&2
    exit 1
}

run_psql() {
    local database_url="$1"
    shift
    psql -X -v ON_ERROR_STOP=1 "$@" "$database_url"
}

drop_all_test_subscriptions() {
    local database_url="$1"
    run_psql "$database_url" -q >/dev/null 2>&1 <<'SQL' || true
SELECT pg_catalog.format('ALTER SUBSCRIPTION %I DISABLE;', subname)
FROM pg_catalog.pg_subscription
WHERE subdbid = (SELECT oid FROM pg_catalog.pg_database WHERE datname = current_database())
ORDER BY subname
\gexec
SELECT pg_catalog.format('ALTER SUBSCRIPTION %I SET (slot_name = NONE);', subname)
FROM pg_catalog.pg_subscription
WHERE subdbid = (SELECT oid FROM pg_catalog.pg_database WHERE datname = current_database())
ORDER BY subname
\gexec
SELECT pg_catalog.format('DROP SUBSCRIPTION %I;', subname)
FROM pg_catalog.pg_subscription
WHERE subdbid = (SELECT oid FROM pg_catalog.pg_database WHERE datname = current_database())
ORDER BY subname
\gexec
SQL
}

cleanup() {
    drop_all_test_subscriptions "$source_url"
    drop_all_test_subscriptions "$target_url"
    drop_all_test_subscriptions "$reject_url"
    dropdb --if-exists --force --maintenance-db="$POSTGRES_ADMIN_URL" "$source_db" >/dev/null 2>&1 || true
    dropdb --if-exists --force --maintenance-db="$POSTGRES_ADMIN_URL" "$target_db" >/dev/null 2>&1 || true
    dropdb --if-exists --force --maintenance-db="$POSTGRES_ADMIN_URL" "$reject_db" >/dev/null 2>&1 || true
    rm -rf "$work_dir"
}
trap cleanup EXIT INT TERM

recreate_reject_target() {
    drop_all_test_subscriptions "$reject_url"
    dropdb --if-exists --force --maintenance-db="$POSTGRES_ADMIN_URL" "$reject_db" >/dev/null 2>&1 || true
    createdb --maintenance-db="$POSTGRES_ADMIN_URL" "$reject_db"
    rm -rf "$reject_git"
    mkdir -p "$reject_git"
}

database_user_object_count() {
    local database_url="$1"
    run_psql "$database_url" -Atq -f "$database_empty_query"
}

publication_exists() {
    local database_url="$1"
    local name="$2"
    run_psql "$database_url" -Atq -v publication_name="$name" <<'SQL'
SELECT EXISTS (
    SELECT 1
    FROM pg_catalog.pg_publication
    WHERE pubname = :'publication_name'
);
SQL
}

subscription_exists() {
    local database_url="$1"
    local name="$2"
    run_psql "$database_url" -Atq -v subscription_name="$name" <<'SQL'
SELECT EXISTS (
    SELECT 1
    FROM pg_catalog.pg_subscription
    WHERE subdbid = (SELECT oid FROM pg_catalog.pg_database WHERE datname = current_database())
      AND subname = :'subscription_name'
);
SQL
}

subscription_contains_secret() {
    local database_url="$1"
    local name="$2"
    local secret="$3"
    run_psql "$database_url" -Atq -v subscription_name="$name" -v subscription_secret="$secret" <<'SQL'
SELECT EXISTS (
    SELECT 1
    FROM pg_catalog.pg_subscription
    WHERE subdbid = (SELECT oid FROM pg_catalog.pg_database WHERE datname = current_database())
      AND subname = :'subscription_name'
      AND pg_catalog.strpos(subconninfo, :'subscription_secret') > 0
);
SQL
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

schema_snapshot() {
    local database_url="$1"
    local output="$2"
    pg_dump --schema-only --no-owner --no-privileges --no-subscriptions "$database_url" |
        sed -e '/^\\restrict /d' -e '/^\\unrestrict /d' >"$output"
}

for command_name in psql pg_dump createdb dropdb git gzip tar sha256sum grep find sed cmp; do
    command -v "$command_name" >/dev/null 2>&1 || fail "required command not found: $command_name"
done
[[ -f "$database_empty_query" && ! -L "$database_empty_query" ]] || fail \
    "database emptiness query is missing or unsafe"
grep -Fq 'psql -X -v ON_ERROR_STOP=1' "$repo_root/scripts/restore.sh" || \
    fail "restore production psql wrapper is missing -X or ON_ERROR_STOP"
grep -Fq 'psql -X -v ON_ERROR_STOP=1' "$repo_root/scripts/git-storage-inventory.sh" || \
    fail "inventory production psql wrapper is missing -X or ON_ERROR_STOP"
if grep -nE '^[[:space:]]*psql[[:space:]]' \
    "$repo_root/scripts/restore.sh" "$repo_root/scripts/git-storage-inventory.sh" |
    grep -vF 'psql -X -v ON_ERROR_STOP=1'; then
    fail "production automation contains a non-interactive psql invocation outside the -X wrapper"
fi

log "creating source and empty restore targets"
createdb --maintenance-db="$POSTGRES_ADMIN_URL" "$source_db"
createdb --maintenance-db="$POSTGRES_ADMIN_URL" "$target_db"
createdb --maintenance-db="$POSTGRES_ADMIN_URL" "$reject_db"
mkdir -p "$source_git" "$target_git" "$reject_git" "$backup_dir"

run_psql "$source_url" <<SQL
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
CREATE SUBSCRIPTION "$subscription_name"
    CONNECTION 'host=upstream.invalid port=5432 dbname=publisher user=replicator password=$subscription_secret'
    PUBLICATION upstream_publication
    WITH (connect = false, slot_name = NONE);
SQL

[[ "$(subscription_exists "$source_url" "$subscription_name")" == "t" ]] || \
    fail "source Subscription was not created"
[[ "$(subscription_contains_secret "$source_url" "$subscription_name" "$subscription_secret")" == "t" ]] || \
    fail "source Subscription conninfo does not contain the credential sentinel"
[[ "$(database_user_object_count "$source_url")" != "0" ]] || \
    fail "shared helper treated a database containing a Subscription as empty"

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

run_psql "$source_url" -v tenant_id="$tenant_id" -v repo_id="$repo_id" \
    -v storage_path="$storage_path" -v main_sha="$main_sha" <<'SQL'
INSERT INTO public.git_repos (
    id, tenant_id, name, default_branch, storage_path, last_commit_sha
) VALUES (
    :'repo_id', :'tenant_id', 'database-object-durability', 'main', :'storage_path', :'main_sha'
);
SQL

log "creating a production backup that retains Publication but excludes Subscription credentials"
backup_output="$work_dir/backup.log"
EVOLITH_BACKUP_QUIESCED=true DATABASE_URL="$source_url" GIT_STORAGE_PATH="$source_git" \
BACKUP_DIR="$backup_dir" EVOLITH_APP_VERSION="database-object-test-head" \
    "$repo_root/scripts/backup.sh" >"$backup_output" 2>&1
archive="$(find "$backup_dir" -maxdepth 1 -type f -name 'evolith-backup-*.tar.gz' -print -quit)"
[[ -n "$archive" ]] || fail "backup archive was not created"
if grep -Fq "$subscription_secret" "$backup_output"; then
    fail "backup stdout/stderr leaked the Subscription credential sentinel"
fi

rm -rf "$package_dir" && mkdir -p "$package_dir"
tar -C "$package_dir" -xzf "$archive"
gzip -dc "$package_dir/database.sql.gz" >"$package_dir/database.sql"
grep -Fq "CREATE PUBLICATION $publication_name" "$package_dir/database.sql" || \
    fail "production database dump did not contain the Publication"
if grep -Fq "$subscription_secret" "$package_dir/database.sql"; then
    fail "decompressed production database dump contains the Subscription credential sentinel"
fi
if grep -Eq '^[[:space:]]*CREATE SUBSCRIPTION[[:space:]]' "$package_dir/database.sql"; then
    fail "decompressed production database dump contains CREATE SUBSCRIPTION"
fi
if grep -Fq "$subscription_name" "$package_dir/database.sql"; then
    fail "decompressed production database dump contains the excluded Subscription name"
fi
for public_component in manifest.env git-refs.tsv SHA256SUMS; do
    if grep -Fq "$subscription_secret" "$package_dir/$public_component"; then
        fail "backup component $public_component leaked the Subscription credential sentinel"
    fi
done
tar -tzf "$archive" >"$work_dir/outer-archive-list.txt"
if grep -Fq "$subscription_secret" "$work_dir/outer-archive-list.txt"; then
    fail "outer archive listing leaked the Subscription credential sentinel"
fi

log "proving the Publication, database state and Git inventory restore without the Subscription"
restore_success_output="$work_dir/restore-success.log"
EVOLITH_RESTORE_QUIESCED=true DATABASE_URL="$target_url" GIT_STORAGE_PATH="$target_git" \
EVOLITH_RESTORE_EXPECTED_APP_VERSION="database-object-test-head" \
    "$repo_root/scripts/restore.sh" "$archive" >"$restore_success_output" 2>&1
grep -Fq "restore completed successfully" "$restore_success_output" || \
    fail "successful production restore did not report completion"
if grep -Fq "$subscription_secret" "$restore_success_output"; then
    fail "restore stdout/stderr leaked the Subscription credential sentinel"
fi
[[ "$(publication_exists "$target_url" "$publication_name")" == "t" ]] || \
    fail "Publication was not restored"
[[ "$(subscription_exists "$target_url" "$subscription_name")" == "f" ]] || \
    fail "excluded Subscription was restored"
[[ "$(run_psql "$target_url" -Atq -c 'SELECT value FROM public.durable_marker')" == "database-level-restored" ]] || \
    fail "database marker was not restored"
[[ "$(git --git-dir="$target_git/$storage_path" rev-parse refs/heads/main)" == "$main_sha" ]] || \
    fail "Git commit was not restored"
DATABASE_URL="$target_url" GIT_STORAGE_PATH="$target_git" \
    "$repo_root/scripts/git-storage-inventory.sh" >/dev/null

log "proving a Publication-only target is rejected before writes"
recreate_reject_target
run_psql "$reject_url" -c "CREATE PUBLICATION $legacy_publication_name" >/dev/null
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
[[ "$(run_psql "$reject_url" -Atq -c "SELECT to_regclass('public.durable_marker') IS NULL")" == "t" ]] || \
    fail "Publication-only rejection wrote schema objects"
assert_git_empty "$reject_git"

log "proving a Subscription-only target is rejected and preserved"
recreate_reject_target
run_psql "$reject_url" <<SQL
CREATE SUBSCRIPTION "$legacy_subscription_name"
    CONNECTION 'host=upstream.invalid port=5432 dbname=publisher user=replicator password=legacy-secret-${run_id}'
    PUBLICATION upstream_publication
    WITH (connect = false, slot_name = NONE);
SQL
[[ "$(database_user_object_count "$reject_url")" != "0" ]] || \
    fail "shared helper treated a Subscription-only target as empty"
subscription_reject_output="$work_dir/subscription-only-reject.log"
if EVOLITH_RESTORE_QUIESCED=true DATABASE_URL="$reject_url" GIT_STORAGE_PATH="$reject_git" \
    "$repo_root/scripts/restore.sh" "$archive" >"$subscription_reject_output" 2>&1; then
    fail "restore unexpectedly accepted a Subscription-only target"
fi
grep -Fq "target database is not empty" "$subscription_reject_output" || \
    fail "Subscription-only target did not reach the database emptiness gate"
if grep -Fq "restoring PostgreSQL into empty target" "$subscription_reject_output"; then
    fail "Subscription-only target was rejected after writes started"
fi
assert_no_restore_success "$subscription_reject_output"
[[ "$(subscription_exists "$reject_url" "$legacy_subscription_name")" == "t" ]] || \
    fail "Subscription-only rejection did not preserve the existing Subscription"
assert_git_empty "$reject_git"

log "proving hostile PSQLRC cannot mutate a legitimate non-empty preflight target"
recreate_reject_target
run_psql "$reject_url" <<'SQL'
CREATE TABLE public.restore_preflight_marker (value TEXT PRIMARY KEY);
INSERT INTO public.restore_preflight_marker(value) VALUES ('preexisting-marker');
SQL
cat >"$hostile_psqlrc" <<'PSQLRC'
\set ON_ERROR_STOP off
\echo HOSTILE_PSQLRC_EXECUTED
CREATE TABLE public.psqlrc_sentinel (value TEXT);
PSQLRC
before_object_count="$(database_user_object_count "$reject_url")"
schema_snapshot "$reject_url" "$work_dir/preflight-before.sql"
psqlrc_reject_output="$work_dir/hostile-psqlrc-reject.log"
if PSQLRC="$hostile_psqlrc" \
    EVOLITH_RESTORE_QUIESCED=true DATABASE_URL="$reject_url" GIT_STORAGE_PATH="$reject_git" \
    "$repo_root/scripts/restore.sh" "$archive" >"$psqlrc_reject_output" 2>&1; then
    fail "restore unexpectedly accepted the non-empty target under hostile PSQLRC"
fi
grep -Fq "target database is not empty" "$psqlrc_reject_output" || \
    fail "hostile PSQLRC test did not reach the non-empty database preflight gate"
if grep -Fq "HOSTILE_PSQLRC_EXECUTED" "$psqlrc_reject_output"; then
    fail "restore executed the hostile psql startup file"
fi
for forbidden_phase in \
    "restoring PostgreSQL into empty target" \
    "installing staged Git storage" \
    "running DB/Git inventory and ref verification"; do
    if grep -Fq "$forbidden_phase" "$psqlrc_reject_output"; then
        fail "hostile PSQLRC test entered forbidden phase: $forbidden_phase"
    fi
done
assert_no_restore_success "$psqlrc_reject_output"
[[ "$(run_psql "$reject_url" -Atq -c 'SELECT value FROM public.restore_preflight_marker')" == "preexisting-marker" ]] || \
    fail "hostile PSQLRC test changed the original marker"
[[ "$(run_psql "$reject_url" -Atq -c "SELECT to_regclass('public.psqlrc_sentinel') IS NULL")" == "t" ]] || \
    fail "hostile PSQLRC created public.psqlrc_sentinel"
after_object_count="$(database_user_object_count "$reject_url")"
[[ "$after_object_count" == "$before_object_count" ]] || \
    fail "hostile PSQLRC test changed the shared user-object count"
schema_snapshot "$reject_url" "$work_dir/preflight-after.sql"
cmp -s "$work_dir/preflight-before.sql" "$work_dir/preflight-after.sql" || \
    fail "hostile PSQLRC test changed database schema state"
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
[[ "$(subscription_exists "$reject_url" "$subscription_name")" == "f" ]] || \
    fail "rollback target unexpectedly contains the excluded Subscription"
[[ "$(database_user_object_count "$reject_url")" == "0" ]] || \
    fail "shared helper did not report zero after Publication rollback"
[[ "$(run_psql "$reject_url" -Atq -c "SELECT to_regclass('public.durable_marker') IS NULL")" == "t" ]] || \
    fail "rollback left schema state behind"
assert_git_empty "$reject_git"

log "all database-level PostgreSQL object durability tests passed"
