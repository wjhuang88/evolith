#!/usr/bin/env bash
set -Eeuo pipefail

SUPPORTED_BACKUP_FORMAT_VERSION=1
EXPECTED_GIT_STORAGE_LAYOUT=tenant-uuid/repo-uuid.git
DATABASE_URL="${DATABASE_URL:-}"
GIT_STORAGE_PATH="${GIT_STORAGE_PATH:-}"
EXPECTED_APP_VERSION="${EVOLITH_RESTORE_EXPECTED_APP_VERSION:-}"
archive_path="${1:-}"
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
database_empty_query="$script_dir/postgres-user-object-count.sql"
uuid_re='[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}'
storage_path_re="^${uuid_re}/${uuid_re}\\.git$"

log() {
    printf '[restore] %s\n' "$*"
}

fail() {
    printf '[restore] ERROR: %s\n' "$*" >&2
    exit 1
}

require_command() {
    command -v "$1" >/dev/null 2>&1 || fail "required command not found: $1"
}

sha256_verify() {
    if command -v sha256sum >/dev/null 2>&1; then
        sha256sum -c SHA256SUMS
    else
        shasum -a 256 -c SHA256SUMS
    fi
}

validate_tar_listing() {
    local tar_file="$1"
    local entry
    local verbose_entry
    tar -tzf "$tar_file" >/dev/null
    while IFS= read -r entry; do
        case "$entry" in
            /*|../*|*/../*|*/..)
                fail "archive contains unsafe path: $entry"
                ;;
        esac
    done < <(tar -tzf "$tar_file")
    while IFS= read -r verbose_entry; do
        case "${verbose_entry:0:1}" in
            -|d) ;;
            *) fail "archive contains a link or special file entry" ;;
        esac
    done < <(tar -tvzf "$tar_file")
}

manifest_value() {
    local key="$1"
    local manifest="$2"
    local count
    count="$(grep -c "^${key}=" "$manifest" || true)"
    [[ "$count" == "1" ]] || fail "manifest key must appear exactly once: $key"
    awk -F= -v wanted="$key" '$1 == wanted {sub(/^[^=]*=/, ""); print; exit}' "$manifest"
}

database_user_object_count() {
    psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -Atq -f "$database_empty_query"
}

reset_postgres_to_empty() {
    psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -q >/dev/null <<'SQL'
-- Subscriptions are database-local but can own remote replication slots. Make
-- them inert and detach slot ownership before dropping them so rollback never
-- attempts a remote connection.
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

-- Extensions must be removed before their member schemas/objects. plpgsql is
-- the fresh-database baseline installed by PostgreSQL itself.
SELECT pg_catalog.format('DROP EXTENSION %I CASCADE;', extname)
FROM pg_catalog.pg_extension
WHERE oid >= 16384
ORDER BY extname
\gexec

SELECT pg_catalog.format('DROP PUBLICATION %I CASCADE;', pubname)
FROM pg_catalog.pg_publication
WHERE oid >= 16384
ORDER BY pubname
\gexec

SELECT pg_catalog.format('DROP SERVER %I CASCADE;', srvname)
FROM pg_catalog.pg_foreign_server
WHERE oid >= 16384
ORDER BY srvname
\gexec

SELECT pg_catalog.format('DROP FOREIGN DATA WRAPPER %I CASCADE;', fdwname)
FROM pg_catalog.pg_foreign_data_wrapper AS f
WHERE f.oid >= 16384
ORDER BY fdwname
\gexec

SELECT pg_catalog.lo_unlink(oid)
FROM pg_catalog.pg_largeobject_metadata
WHERE oid >= 16384;

-- Keep event triggers active through schema removal. The durability suite uses
-- one to deterministically prove that a failed cleanup remains non-zero and is
-- escalated to CRITICAL instead of being reported as an empty rollback.
DO $evolith$
DECLARE
    user_schema TEXT;
BEGIN
    FOR user_schema IN
        SELECT nspname
        FROM pg_catalog.pg_namespace
        WHERE nspname <> 'information_schema'
          AND nspname !~ '^pg_'
        ORDER BY nspname
    LOOP
        EXECUTE pg_catalog.format('DROP SCHEMA %I CASCADE', user_schema);
    END LOOP;
END
$evolith$;

-- Remove any database-level programmable objects that were not already
-- removed through schema or extension dependencies.
SELECT pg_catalog.format(
    'DROP TRANSFORM FOR %s LANGUAGE %I CASCADE;',
    pg_catalog.format_type(t.trftype, NULL),
    l.lanname
)
FROM pg_catalog.pg_transform AS t
JOIN pg_catalog.pg_language AS l ON l.oid = t.trflang
WHERE t.oid >= 16384
ORDER BY 1
\gexec

SELECT pg_catalog.format(
    'DROP CAST (%s AS %s) CASCADE;',
    pg_catalog.format_type(c.castsource, NULL),
    pg_catalog.format_type(c.casttarget, NULL)
)
FROM pg_catalog.pg_cast AS c
WHERE c.oid >= 16384
ORDER BY 1
\gexec

SELECT pg_catalog.format('DROP ACCESS METHOD %I CASCADE;', amname)
FROM pg_catalog.pg_am AS a
WHERE a.oid >= 16384
ORDER BY amname
\gexec

SELECT pg_catalog.format('DROP LANGUAGE %I CASCADE;', lanname)
FROM pg_catalog.pg_language AS l
WHERE l.oid >= 16384
ORDER BY lanname
\gexec

SELECT pg_catalog.format('DROP EVENT TRIGGER %I;', evtname)
FROM pg_catalog.pg_event_trigger
WHERE oid >= 16384
ORDER BY evtname
\gexec

CREATE SCHEMA public;
SQL
}

[[ -n "$archive_path" ]] || fail "usage: scripts/restore.sh <evolith-backup.tar.gz>"
[[ -f "$archive_path" && ! -L "$archive_path" ]] || fail "backup archive not found or is a symlink: $archive_path"
[[ "${EVOLITH_RESTORE_QUIESCED:-false}" == "true" ]] || fail \
    "EVOLITH_RESTORE_QUIESCED=true is required; stop application writes before restore"
[[ -n "$DATABASE_URL" ]] || fail "DATABASE_URL is required"
[[ -n "$GIT_STORAGE_PATH" ]] || fail "GIT_STORAGE_PATH is required"
[[ -f "$database_empty_query" && ! -L "$database_empty_query" ]] || fail \
    "database emptiness query is missing or unsafe: $database_empty_query"

for command_name in psql gzip tar git find grep awk sort uniq cmp mktemp; do
    require_command "$command_name"
done

stage_dir="$(mktemp -d "${TMPDIR:-/tmp}/evolith-restore.XXXXXX")"
package_dir="$stage_dir/package"
staged_git="$stage_dir/git"
mkdir -p "$package_dir" "$staged_git"
restore_started=false
git_path_created=false

reset_empty_target() {
    local cleanup_failed=false
    local remaining_objects

    if ! reset_postgres_to_empty >/dev/null 2>&1; then
        cleanup_failed=true
    elif ! remaining_objects="$(database_user_object_count 2>/dev/null)"; then
        cleanup_failed=true
    elif [[ "$remaining_objects" != "0" ]]; then
        cleanup_failed=true
    fi

    if [[ -e "$GIT_STORAGE_PATH" ]]; then
        if [[ -d "$GIT_STORAGE_PATH" && ! -L "$GIT_STORAGE_PATH" ]]; then
            if ! find "$GIT_STORAGE_PATH" -mindepth 1 -maxdepth 1 -exec rm -rf -- {} + 2>/dev/null; then
                cleanup_failed=true
            fi
            if find "$GIT_STORAGE_PATH" -mindepth 1 -print -quit 2>/dev/null | grep -q .; then
                cleanup_failed=true
            fi
        else
            cleanup_failed=true
        fi
    fi

    if [[ "$git_path_created" == "true" && -e "$GIT_STORAGE_PATH" ]] && \
        ! rmdir "$GIT_STORAGE_PATH" 2>/dev/null; then
        cleanup_failed=true
    fi

    [[ "$cleanup_failed" == "false" ]]
}

cleanup() {
    local status=$?
    trap - EXIT INT TERM
    if (( status != 0 )) && [[ "$restore_started" == "true" ]]; then
        printf '[restore] restore failed; reverting the previously empty target\n' >&2
        if ! reset_empty_target; then
            printf '[restore] CRITICAL: automatic rollback was incomplete; keep the application stopped and inspect both targets\n' >&2
        fi
    fi
    rm -rf "$stage_dir"
    exit "$status"
}
trap cleanup EXIT INT TERM

validate_tar_listing "$archive_path"
tar --no-same-owner --no-same-permissions -C "$package_dir" -xzf "$archive_path"

required_files=(database.sql.gz git-storage.tar.gz git-refs.tsv manifest.env SHA256SUMS)
for required_file in "${required_files[@]}"; do
    [[ -f "$package_dir/$required_file" && ! -L "$package_dir/$required_file" ]] || fail \
        "backup is incomplete or unsafe: $required_file"
done
while IFS= read -r -d '' package_entry; do
    package_name="${package_entry#"$package_dir"/}"
    [[ ! -L "$package_entry" ]] || fail "backup contains a top-level symlink: $package_name"
    case "$package_name" in
        database.sql.gz|git-storage.tar.gz|git-refs.tsv|manifest.env|SHA256SUMS) ;;
        *) fail "backup contains unexpected top-level entry: $package_name" ;;
    esac
done < <(find "$package_dir" -mindepth 1 -maxdepth 1 -print0)

manifest_line_count="$(awk 'END {print NR}' "$package_dir/manifest.env")"
[[ "$manifest_line_count" == "7" ]] || fail "manifest must contain exactly seven entries"

format_version="$(manifest_value backup_format_version "$package_dir/manifest.env")"
[[ "$format_version" == "$SUPPORTED_BACKUP_FORMAT_VERSION" ]] || fail \
    "unsupported backup format version: $format_version"

actual_app_version="$(manifest_value application_version "$package_dir/manifest.env")"
[[ -n "$actual_app_version" ]] || fail "application_version must not be empty"

created_at="$(manifest_value created_at "$package_dir/manifest.env")"
[[ "$created_at" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z$ ]] || fail \
    "invalid manifest created_at: $created_at"

[[ "$(manifest_value consistency "$package_dir/manifest.env")" == "quiesced-maintenance-window" ]] || fail \
    "backup does not declare the required consistency boundary"
[[ "$(manifest_value database_format "$package_dir/manifest.env")" == "postgresql-plain-sql-gzip" ]] || fail \
    "unsupported database backup format"
[[ "$(manifest_value git_format "$package_dir/manifest.env")" == "tar-gzip" ]] || fail \
    "unsupported Git backup format"
[[ "$(manifest_value git_storage_layout "$package_dir/manifest.env")" == "$EXPECTED_GIT_STORAGE_LAYOUT" ]] || fail \
    "unsupported Git storage layout"

if [[ -n "$EXPECTED_APP_VERSION" ]]; then
    [[ "$actual_app_version" == "$EXPECTED_APP_VERSION" ]] || fail \
        "application version mismatch: expected $EXPECTED_APP_VERSION, got $actual_app_version"
fi

checksum_count=0
seen_database=false
seen_git_storage=false
seen_git_refs=false
seen_manifest=false
while read -r checksum checksum_file extra; do
    [[ -z "${extra:-}" ]] || fail "invalid checksum entry with extra fields"
    checksum_file="${checksum_file#\*}"
    [[ "$checksum" =~ ^[0-9a-fA-F]{64}$ ]] || fail "invalid checksum entry"
    case "$checksum_file" in
        database.sql.gz)
            [[ "$seen_database" == "false" ]] || fail "duplicate checksum entry: database.sql.gz"
            seen_database=true
            ;;
        git-storage.tar.gz)
            [[ "$seen_git_storage" == "false" ]] || fail "duplicate checksum entry: git-storage.tar.gz"
            seen_git_storage=true
            ;;
        git-refs.tsv)
            [[ "$seen_git_refs" == "false" ]] || fail "duplicate checksum entry: git-refs.tsv"
            seen_git_refs=true
            ;;
        manifest.env)
            [[ "$seen_manifest" == "false" ]] || fail "duplicate checksum entry: manifest.env"
            seen_manifest=true
            ;;
        *) fail "checksum file list contains unexpected path: $checksum_file" ;;
    esac
    checksum_count=$((checksum_count + 1))
done <"$package_dir/SHA256SUMS"
[[ "$checksum_count" == "4" ]] || fail "checksum file must contain exactly four entries"
[[ "$seen_database" == "true" && "$seen_git_storage" == "true" && \
    "$seen_git_refs" == "true" && "$seen_manifest" == "true" ]] || fail \
    "checksum file must cover each required backup component exactly once"

log "verifying checksums and archive structure before target writes"
(
    cd "$package_dir"
    sha256_verify >/dev/null
)
gzip -t "$package_dir/database.sql.gz"
validate_tar_listing "$package_dir/git-storage.tar.gz"
tar --no-same-owner --no-same-permissions -C "$staged_git" -xzf "$package_dir/git-storage.tar.gz"

if find "$staged_git" -mindepth 1 -type l -print -quit | grep -q .; then
    fail "Git archive contains symlinks"
fi
while IFS= read -r -d '' entry; do
    relative_path="${entry#"$staged_git"/}"
    if [[ "$relative_path" == */* ]]; then
        [[ -d "$entry" && "$relative_path" =~ $storage_path_re ]] || \
            fail "unexpected second-level Git archive entry: $relative_path"
    else
        [[ -d "$entry" && "$relative_path" =~ ^${uuid_re}$ ]] || \
            fail "unexpected top-level Git archive entry: $relative_path"
    fi
done < <(find "$staged_git" -mindepth 1 -maxdepth 2 -print0)

staged_refs_unsorted="$stage_dir/staged-refs.unsorted.tsv"
staged_refs="$stage_dir/staged-refs.tsv"
expected_refs_sorted="$stage_dir/expected-refs.tsv"
: >"$staged_refs_unsorted"

while IFS= read -r -d '' repo_path; do
    relative_path="${repo_path#"$staged_git"/}"
    [[ "$(git --git-dir="$repo_path" rev-parse --is-bare-repository 2>/dev/null || true)" == "true" ]] || \
        fail "invalid bare repository in backup: $relative_path"
    git --git-dir="$repo_path" fsck --full >/dev/null || fail "git fsck failed in backup: $relative_path"
    while read -r sha ref_name; do
        [[ -n "${sha:-}" && -n "${ref_name:-}" ]] || continue
        printf '%s\t%s\t%s\n' "$relative_path" "$sha" "$ref_name" >>"$staged_refs_unsorted"
    done < <(git --git-dir="$repo_path" show-ref)
done < <(find "$staged_git" -mindepth 2 -maxdepth 2 -type d -name '*.git' -print0)
LC_ALL=C sort "$staged_refs_unsorted" >"$staged_refs"
LC_ALL=C sort "$package_dir/git-refs.tsv" >"$expected_refs_sorted"

if [[ -n "$(uniq -d "$expected_refs_sorted")" ]]; then
    fail "ref inventory contains duplicate entries"
fi
while IFS=$'\t' read -r storage_path expected_sha ref_name extra; do
    [[ -z "${extra:-}" ]] || fail "invalid ref inventory entry with extra fields"
    [[ -n "$storage_path" && -n "$expected_sha" && -n "$ref_name" ]] || continue
    [[ "$storage_path" =~ $storage_path_re ]] || fail \
        "invalid storage path in ref inventory: $storage_path"
    [[ "$expected_sha" =~ ^([0-9a-fA-F]{40}|[0-9a-fA-F]{64})$ ]] || fail \
        "invalid object id in ref inventory"
    git check-ref-format "$ref_name" >/dev/null 2>&1 || fail \
        "invalid ref name in ref inventory: $ref_name"
    repo_path="$staged_git/$storage_path"
    actual_sha="$(git --git-dir="$repo_path" show-ref --verify --hash "$ref_name" 2>/dev/null || true)"
    [[ "$actual_sha" == "$expected_sha" ]] || fail \
        "backup ref mismatch for $storage_path $ref_name: expected $expected_sha, got ${actual_sha:-missing}"
done <"$package_dir/git-refs.tsv"

if ! cmp -s "$staged_refs" "$expected_refs_sorted"; then
    fail "Git archive refs do not exactly match git-refs.tsv"
fi

target_user_object_count="$(database_user_object_count)"
[[ "$target_user_object_count" =~ ^[0-9]+$ ]] || fail \
    "target database emptiness check returned an invalid result"
[[ "$target_user_object_count" == "0" ]] || fail \
    "target database is not empty; restore refuses to overwrite existing data"

if [[ -e "$GIT_STORAGE_PATH" ]]; then
    [[ -d "$GIT_STORAGE_PATH" && ! -L "$GIT_STORAGE_PATH" ]] || fail \
        "target Git storage must be a real directory"
    if find "$GIT_STORAGE_PATH" -mindepth 1 -print -quit | grep -q .; then
        fail "target Git storage is not empty; restore refuses to overwrite existing data"
    fi
else
    mkdir -p "$GIT_STORAGE_PATH"
    git_path_created=true
fi

restore_started=true
log "restoring PostgreSQL into empty target"
gzip -dc "$package_dir/database.sql.gz" | \
    psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -q --single-transaction

log "installing staged Git storage"
tar -C "$staged_git" -cf - . | tar --no-same-owner --no-same-permissions -C "$GIT_STORAGE_PATH" -xf -

log "running DB/Git inventory and ref verification"
DATABASE_URL="$DATABASE_URL" \
GIT_STORAGE_PATH="$GIT_STORAGE_PATH" \
EXPECTED_REFS_FILE="$package_dir/git-refs.tsv" \
    "$script_dir/git-storage-inventory.sh"

restore_started=false
log "restore completed successfully"
