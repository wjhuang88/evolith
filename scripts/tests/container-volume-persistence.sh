#!/usr/bin/env bash
set -Eeuo pipefail

image="${GIT_VOLUME_TEST_IMAGE:-alpine:3.20}"
volume_name="evolith-git-persistence-${RANDOM}-$$"
work_dir="$(mktemp -d "${TMPDIR:-/tmp}/evolith-volume-test.XXXXXX")"
tenant_id="11111111-1111-4111-8111-111111111111"
repo_id="22222222-2222-4222-8222-222222222222"
storage_path="$tenant_id/$repo_id.git"

log() {
    printf '[volume-test] %s\n' "$*"
}

fail() {
    printf '[volume-test] ERROR: %s\n' "$*" >&2
    exit 1
}

cleanup() {
    docker volume rm -f "$volume_name" >/dev/null 2>&1 || true
    rm -rf "$work_dir"
}
trap cleanup EXIT INT TERM

command -v docker >/dev/null 2>&1 || fail "docker is required"
docker info >/dev/null 2>&1 || fail "docker daemon is unavailable"
docker volume create "$volume_name" >/dev/null

log "creating Commit, Branch and Tag in the first container"
docker run --rm \
    --mount "type=volume,source=$volume_name,target=/var/lib/evolith/git" \
    --mount "type=bind,source=$work_dir,target=/evidence" \
    --entrypoint sh \
    "$image" -ec '
        apk add --no-cache git >/dev/null
        tenant_id="11111111-1111-4111-8111-111111111111"
        repo_id="22222222-2222-4222-8222-222222222222"
        storage_path="$tenant_id/$repo_id.git"
        mkdir -p "/var/lib/evolith/git/$tenant_id"
        git init --bare "/var/lib/evolith/git/$storage_path" >/dev/null
        git init -b main /tmp/work >/dev/null
        git -C /tmp/work config user.name "Volume Test"
        git -C /tmp/work config user.email "volume-test@example.invalid"
        printf "first\n" >/tmp/work/README.md
        git -C /tmp/work add README.md
        git -C /tmp/work commit -m first >/dev/null
        git -C /tmp/work branch feature/recovery
        printf "second\n" >>/tmp/work/README.md
        git -C /tmp/work commit -am second >/dev/null
        git -C /tmp/work tag v1.0.0
        git -C /tmp/work remote add origin "/var/lib/evolith/git/$storage_path"
        git -C /tmp/work push origin --all >/dev/null
        git -C /tmp/work push origin --tags >/dev/null
        git --git-dir="/var/lib/evolith/git/$storage_path" symbolic-ref HEAD refs/heads/main
        git -C /tmp/work rev-parse HEAD >/evidence/expected-main-sha
    '

[[ -s "$work_dir/expected-main-sha" ]] || fail "first container did not record the expected SHA"

log "removing the first container and verifying the same volume in a new container"
docker run --rm \
    --mount "type=volume,source=$volume_name,target=/var/lib/evolith/git" \
    --mount "type=bind,source=$work_dir,target=/evidence,readonly" \
    --entrypoint sh \
    "$image" -ec '
        apk add --no-cache git >/dev/null
        tenant_id="11111111-1111-4111-8111-111111111111"
        repo_id="22222222-2222-4222-8222-222222222222"
        storage_path="$tenant_id/$repo_id.git"
        expected_sha="$(cat /evidence/expected-main-sha)"
        repo_path="/var/lib/evolith/git/$storage_path"
        test "$(git --git-dir="$repo_path" rev-parse refs/heads/main)" = "$expected_sha"
        git --git-dir="$repo_path" show-ref --verify --quiet refs/heads/feature/recovery
        git --git-dir="$repo_path" show-ref --verify --quiet refs/tags/v1.0.0
        git --git-dir="$repo_path" fsck --full >/dev/null
        git clone "$repo_path" /tmp/clone >/dev/null
        test "$(git -C /tmp/clone rev-parse HEAD)" = "$expected_sha"
    '

log "named volume preserved Commit, Branch, Tag and clone behavior across container recreation"
