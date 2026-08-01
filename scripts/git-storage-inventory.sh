#!/usr/bin/env bash
set -Eeuo pipefail

DATABASE_URL="${DATABASE_URL:-}"
GIT_STORAGE_PATH="${GIT_STORAGE_PATH:-}"
EXPECTED_REFS_FILE="${EXPECTED_REFS_FILE:-}"
uuid_re='[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}'
storage_path_re="^${uuid_re}/${uuid_re}\\.git$"

log() {
    printf '[inventory] %s\n' "$*"
}

fail() {
    printf '[inventory] ERROR: %s\n' "$*" >&2
    exit 1
}

require_command() {
    command -v "$1" >/dev/null 2>&1 || fail "required command not found: $1"
}

[[ -n "$DATABASE_URL" ]] || fail "DATABASE_URL is required"
[[ -n "$GIT_STORAGE_PATH" ]] || fail "GIT_STORAGE_PATH is required"
[[ -d "$GIT_STORAGE_PATH" ]] || fail "Git storage path does not exist or is not a directory"
[[ ! -L "$GIT_STORAGE_PATH" ]] || fail "Git storage base path must not be a symlink"
[[ -z "$EXPECTED_REFS_FILE" || ( -f "$EXPECTED_REFS_FILE" && ! -L "$EXPECTED_REFS_FILE" ) ]] || \
    fail "EXPECTED_REFS_FILE does not exist or is a symlink"

for command_name in psql git find sort grep awk mktemp cut uniq wc tr; do
    require_command "$command_name"
done

work_dir="$(mktemp -d "${TMPDIR:-/tmp}/evolith-inventory.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

db_rows="$work_dir/db.tsv"
db_paths="$work_dir/db-paths.txt"
disk_paths="$work_dir/disk-paths.txt"
errors=0

psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -At -F $'\t' -c \
    "SELECT storage_path, default_branch, COALESCE(last_commit_sha, '') FROM git_repos ORDER BY storage_path" \
    >"$db_rows"

cut -f1 "$db_rows" | LC_ALL=C sort >"$db_paths"
if [[ -n "$(uniq -d "$db_paths")" ]]; then
    printf '[inventory] ERROR: duplicate storage_path values in database:\n%s\n' "$(uniq -d "$db_paths")" >&2
    errors=$((errors + 1))
fi

if find "$GIT_STORAGE_PATH" -mindepth 1 -type l -print -quit | grep -q .; then
    printf '[inventory] ERROR: symlinks are not allowed in Git storage\n' >&2
    errors=$((errors + 1))
fi

while IFS= read -r -d '' entry; do
    relative_path="${entry#"$GIT_STORAGE_PATH"/}"
    if [[ "$relative_path" == */* ]]; then
        if [[ ! -d "$entry" || ! "$relative_path" =~ $storage_path_re ]]; then
            printf '[inventory] ERROR: unexpected second-level Git storage entry: %s\n' "$relative_path" >&2
            errors=$((errors + 1))
        fi
    elif [[ ! -d "$entry" || ! "$relative_path" =~ ^${uuid_re}$ ]]; then
        printf '[inventory] ERROR: unexpected top-level Git storage entry: %s\n' "$relative_path" >&2
        errors=$((errors + 1))
    fi
done < <(find "$GIT_STORAGE_PATH" -mindepth 1 -maxdepth 2 -print0)

while IFS= read -r -d '' tenant_dir; do
    if ! find "$tenant_dir" -mindepth 1 -maxdepth 1 -type d -name '*.git' -print -quit | grep -q .; then
        printf '[inventory] ERROR: orphan or empty tenant directory: %s\n' "${tenant_dir#"$GIT_STORAGE_PATH"/}" >&2
        errors=$((errors + 1))
    fi
done < <(find "$GIT_STORAGE_PATH" -mindepth 1 -maxdepth 1 -type d -print0)

: >"$disk_paths"
while IFS= read -r -d '' disk_repo; do
    relative_path="${disk_repo#"$GIT_STORAGE_PATH"/}"
    printf '%s\n' "$relative_path" >>"$disk_paths"
done < <(find "$GIT_STORAGE_PATH" -mindepth 2 -maxdepth 2 -type d -name '*.git' -print0)
LC_ALL=C sort -o "$disk_paths" "$disk_paths"

while IFS=$'\t' read -r storage_path default_branch last_commit_sha extra; do
    [[ -z "${extra:-}" ]] || {
        printf '[inventory] ERROR: malformed database inventory row\n' >&2
        errors=$((errors + 1))
        continue
    }
    [[ -n "$storage_path" ]] || continue
    if [[ ! "$storage_path" =~ $storage_path_re ]]; then
        printf '[inventory] ERROR: invalid storage_path format: %s\n' "$storage_path" >&2
        errors=$((errors + 1))
        continue
    fi
    if ! git check-ref-format "refs/heads/$default_branch" >/dev/null 2>&1; then
        printf '[inventory] ERROR: invalid default branch for %s: %s\n' "$storage_path" "$default_branch" >&2
        errors=$((errors + 1))
        continue
    fi
    if [[ -n "$last_commit_sha" && ! "$last_commit_sha" =~ ^([0-9a-fA-F]{40}|[0-9a-fA-F]{64})$ ]]; then
        printf '[inventory] ERROR: invalid last_commit_sha format for %s: %s\n' "$storage_path" "$last_commit_sha" >&2
        errors=$((errors + 1))
        continue
    fi

    repo_path="$GIT_STORAGE_PATH/$storage_path"
    if [[ ! -d "$repo_path" ]]; then
        printf '[inventory] ERROR: DB-only repository, Git directory missing: %s\n' "$storage_path" >&2
        errors=$((errors + 1))
        continue
    fi
    if [[ -L "$repo_path" ]]; then
        printf '[inventory] ERROR: repository path is a symlink: %s\n' "$storage_path" >&2
        errors=$((errors + 1))
        continue
    fi
    if [[ "$(git --git-dir="$repo_path" rev-parse --is-bare-repository 2>/dev/null || true)" != "true" ]]; then
        printf '[inventory] ERROR: invalid bare repository: %s\n' "$storage_path" >&2
        errors=$((errors + 1))
        continue
    fi
    if ! git --git-dir="$repo_path" fsck --full >/dev/null; then
        printf '[inventory] ERROR: git fsck failed: %s\n' "$storage_path" >&2
        errors=$((errors + 1))
    fi

    if git --git-dir="$repo_path" show-ref >/dev/null 2>&1; then
        if ! git --git-dir="$repo_path" show-ref --verify --quiet "refs/heads/$default_branch"; then
            printf '[inventory] ERROR: default branch ref missing for %s: %s\n' "$storage_path" "$default_branch" >&2
            errors=$((errors + 1))
        fi
    fi

    if [[ -n "$last_commit_sha" ]] && \
        ! git --git-dir="$repo_path" cat-file -e "${last_commit_sha}^{commit}" 2>/dev/null; then
        printf '[inventory] ERROR: last_commit_sha is missing for %s: %s\n' "$storage_path" "$last_commit_sha" >&2
        errors=$((errors + 1))
    fi
done <"$db_rows"

while IFS= read -r disk_path; do
    [[ -n "$disk_path" ]] || continue
    if ! grep -Fqx "$disk_path" "$db_paths"; then
        printf '[inventory] ERROR: disk-only repository, DB row missing: %s\n' "$disk_path" >&2
        errors=$((errors + 1))
    fi
done <"$disk_paths"

if [[ -n "$EXPECTED_REFS_FILE" ]]; then
    if [[ -n "$(LC_ALL=C sort "$EXPECTED_REFS_FILE" | uniq -d)" ]]; then
        printf '[inventory] ERROR: expected ref inventory contains duplicate entries\n' >&2
        errors=$((errors + 1))
    fi
    while IFS=$'\t' read -r storage_path expected_sha ref_name extra; do
        [[ -z "${extra:-}" ]] || {
            printf '[inventory] ERROR: malformed expected ref inventory row\n' >&2
            errors=$((errors + 1))
            continue
        }
        [[ -n "$storage_path" && -n "$expected_sha" && -n "$ref_name" ]] || continue
        if [[ ! "$storage_path" =~ $storage_path_re ]]; then
            printf '[inventory] ERROR: invalid storage path in expected refs: %s\n' "$storage_path" >&2
            errors=$((errors + 1))
            continue
        fi
        if [[ ! "$expected_sha" =~ ^([0-9a-fA-F]{40}|[0-9a-fA-F]{64})$ ]]; then
            printf '[inventory] ERROR: invalid object id in expected refs for %s\n' "$storage_path" >&2
            errors=$((errors + 1))
            continue
        fi
        if ! git check-ref-format "$ref_name" >/dev/null 2>&1; then
            printf '[inventory] ERROR: invalid ref name in expected refs: %s\n' "$ref_name" >&2
            errors=$((errors + 1))
            continue
        fi
        repo_path="$GIT_STORAGE_PATH/$storage_path"
        actual_sha="$(git --git-dir="$repo_path" show-ref --verify --hash "$ref_name" 2>/dev/null || true)"
        if [[ "$actual_sha" != "$expected_sha" ]]; then
            printf '[inventory] ERROR: ref mismatch for %s %s: expected %s, got %s\n' \
                "$storage_path" "$ref_name" "$expected_sha" "${actual_sha:-missing}" >&2
            errors=$((errors + 1))
        fi
    done <"$EXPECTED_REFS_FILE"
fi

if (( errors > 0 )); then
    printf '[inventory] FAILED: %d inconsistency item(s) detected\n' "$errors" >&2
    exit 1
fi

repo_count="$(wc -l <"$db_paths" | tr -d ' ')"
log "inventory consistent: $repo_count repository record(s)"
