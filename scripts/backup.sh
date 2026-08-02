#!/usr/bin/env bash
set -Eeuo pipefail

BACKUP_FORMAT_VERSION=1
BACKUP_DIR="${BACKUP_DIR:-./backups}"
RETENTION_DAYS="${RETENTION_DAYS:-30}"
DATABASE_URL="${DATABASE_URL:-}"
GIT_STORAGE_PATH="${GIT_STORAGE_PATH:-}"
EVOLITH_APP_VERSION="${EVOLITH_APP_VERSION:-}"
safe_app_version="${EVOLITH_APP_VERSION//$'\n'/}"
safe_app_version="${safe_app_version//$'\r'/}"
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

log() {
    printf '[backup] %s\n' "$*"
}

fail() {
    printf '[backup] ERROR: %s\n' "$*" >&2
    exit 1
}

require_command() {
    command -v "$1" >/dev/null 2>&1 || fail "required command not found: $1"
}

sha256_write() {
    if command -v sha256sum >/dev/null 2>&1; then
        sha256sum "$@"
    else
        shasum -a 256 "$@"
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
            *) fail "archive contains a link or special file entry: $tar_file" ;;
        esac
    done < <(tar -tvzf "$tar_file")
}

[[ "${EVOLITH_BACKUP_QUIESCED:-false}" == "true" ]] || fail \
    "EVOLITH_BACKUP_QUIESCED=true is required; sequential pg_dump and Git archiving are not an online consistency guarantee"
[[ -n "$DATABASE_URL" ]] || fail "DATABASE_URL is required"
[[ -n "$GIT_STORAGE_PATH" ]] || fail "GIT_STORAGE_PATH is required"
[[ -n "$safe_app_version" ]] || fail \
    "EVOLITH_APP_VERSION must contain a traceable value after removing line breaks"
[[ -d "$GIT_STORAGE_PATH" ]] || fail "Git storage path does not exist or is not a directory"
[[ ! -L "$GIT_STORAGE_PATH" ]] || fail "Git storage base path must not be a symlink"
[[ "$RETENTION_DAYS" =~ ^[0-9]+$ ]] || fail "RETENTION_DAYS must be a non-negative integer"

for command_name in pg_dump gzip tar git find sort awk date mktemp mv; do
    require_command "$command_name"
done

mkdir -p "$BACKUP_DIR"
chmod 700 "$BACKUP_DIR" 2>/dev/null || true

lock_dir="$BACKUP_DIR/.evolith-backup.lock"
mkdir "$lock_dir" 2>/dev/null || fail "another backup appears to be running: $lock_dir"
stage_dir="$(mktemp -d "$BACKUP_DIR/.evolith-backup-stage.XXXXXX")"
cleanup() {
    rm -rf "$stage_dir" "$lock_dir"
}
trap cleanup EXIT INT TERM

created_at="$(date -u +'%Y-%m-%dT%H%M%SZ')"
timestamp="$(date -u +'%Y%m%dT%H%M%SZ')"
archive_path="$BACKUP_DIR/evolith-backup-${timestamp}-$$.tar.gz"
staged_archive="$stage_dir/evolith-backup.tar.gz"
refs_unsorted="$stage_dir/git-refs.unsorted.tsv"
refs_file="$stage_dir/git-refs.tsv"
: >"$refs_unsorted"

log "checking DB/Git inventory before snapshot"
DATABASE_URL="$DATABASE_URL" GIT_STORAGE_PATH="$GIT_STORAGE_PATH" \
    "$script_dir/git-storage-inventory.sh"

special_entry="$(find "$GIT_STORAGE_PATH" -mindepth 1 \
    ! -type d ! -type f ! -type l -print -quit)"
[[ -z "$special_entry" ]] || fail \
    "Git storage contains an unsupported special file: ${special_entry#"$GIT_STORAGE_PATH"/}"

log "capturing and validating Git refs"
while IFS= read -r -d '' repo_path; do
    relative_path="${repo_path#"$GIT_STORAGE_PATH"/}"
    [[ "$relative_path" != "$repo_path" ]] || fail "repository escaped Git storage base path"
    [[ "$(git --git-dir="$repo_path" rev-parse --is-bare-repository 2>/dev/null)" == "true" ]] || \
        fail "invalid bare repository: $relative_path"
    git --git-dir="$repo_path" fsck --full >/dev/null || fail "git fsck failed: $relative_path"
    while read -r sha ref_name; do
        [[ -n "${sha:-}" && -n "${ref_name:-}" ]] || continue
        printf '%s\t%s\t%s\n' "$relative_path" "$sha" "$ref_name" >>"$refs_unsorted"
    done < <(git --git-dir="$repo_path" show-ref)
done < <(find "$GIT_STORAGE_PATH" -mindepth 2 -maxdepth 2 -type d -name '*.git' -print0)
LC_ALL=C sort "$refs_unsorted" >"$refs_file"
rm -f "$refs_unsorted"

log "dumping PostgreSQL"
pg_dump --no-owner --no-privileges "$DATABASE_URL" | gzip -c >"$stage_dir/database.sql.gz"
gzip -t "$stage_dir/database.sql.gz"

log "archiving Git storage"
tar -C "$GIT_STORAGE_PATH" -czf "$stage_dir/git-storage.tar.gz" .
validate_tar_listing "$stage_dir/git-storage.tar.gz"

cat >"$stage_dir/manifest.env" <<EOF
backup_format_version=$BACKUP_FORMAT_VERSION
application_version=$safe_app_version
created_at=$created_at
consistency=quiesced-maintenance-window
database_format=postgresql-plain-sql-gzip
git_format=tar-gzip
git_storage_layout=tenant-uuid/repo-uuid.git
EOF

(
    cd "$stage_dir"
    sha256_write database.sql.gz git-storage.tar.gz git-refs.tsv manifest.env >SHA256SUMS
)

tar -C "$stage_dir" -czf "$staged_archive" \
    database.sql.gz git-storage.tar.gz git-refs.tsv manifest.env SHA256SUMS
validate_tar_listing "$staged_archive"
chmod 600 "$staged_archive"
mv "$staged_archive" "$archive_path"

if (( RETENTION_DAYS > 0 )); then
    find "$BACKUP_DIR" -maxdepth 1 -type f -name 'evolith-backup-*.tar.gz' \
        -mtime "+$RETENTION_DAYS" -delete
fi

log "backup completed: $archive_path"
