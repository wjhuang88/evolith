#!/usr/bin/env bash
set -Eeuo pipefail

POSTGRES_BASE_URL="${POSTGRES_BASE_URL:-postgres://evolith:evolith@localhost:5432}"
POSTGRES_ADMIN_URL="${POSTGRES_ADMIN_URL:-$POSTGRES_BASE_URL/postgres}"
EVOLITH_BINARY="${EVOLITH_BINARY:-backend/target/debug/evolith}"
PORT="${BACKEND_CONTAINER_PORT:-18082}"
run_id="${RANDOM}_$$"
db_name="evolith_container_${run_id}"
db_url="$POSTGRES_BASE_URL/$db_name"
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
work_dir="$(mktemp -d "${TMPDIR:-/tmp}/evolith-backend-container.XXXXXX")"
image="evolith-backend-recreation:${run_id}"
volume="evolith-git-${run_id}"
container=""
email="container-${run_id}@example.invalid"
username="container${run_id}"
password="TestPassword123!"

log() { printf '[backend-container] %s\n' "$*"; }
fail() {
    printf '[backend-container] ERROR: %s\n' "$*" >&2
    [[ -z "$container" ]] || docker logs "$container" >&2 2>/dev/null || true
    exit 1
}
stop_container() {
    [[ -z "$container" ]] || docker rm -f "$container" >/dev/null 2>&1 || true
    container=""
}
cleanup() {
    stop_container
    dropdb --if-exists --force --maintenance-db="$POSTGRES_ADMIN_URL" "$db_name" >/dev/null 2>&1 || true
    docker volume rm -f "$volume" >/dev/null 2>&1 || true
    docker image rm -f "$image" >/dev/null 2>&1 || true
    rm -rf "$work_dir"
}
trap cleanup EXIT INT TERM

wait_ready() {
    for _ in $(seq 1 90); do
        docker inspect -f '{{.State.Running}}' "$container" 2>/dev/null | grep -q true || fail "Backend container exited"
        curl -sf "http://127.0.0.1:${PORT}/health/ready" >/dev/null 2>&1 && return 0
        sleep 1
    done
    fail "Backend container did not become ready"
}
start_container() {
    local suffix="$1"
    stop_container
    container="evolith-backend-${suffix}-${run_id}"
    docker run -d --name "$container" --network host \
        --mount "type=volume,source=$volume,target=/var/lib/evolith/git" \
        -e ENVIRONMENT=development \
        -e APP__PUBLIC_URL="http://127.0.0.1:${PORT}" \
        -e SERVER__HOST=127.0.0.1 -e SERVER__PORT="$PORT" \
        -e DATABASE__DATABASE_TYPE=postgres -e DATABASE__URL="$db_url" \
        -e DATABASE__SEED_DATABASE=false \
        -e GIT_STORAGE__BASE_PATH=/var/lib/evolith/git \
        -e JWT__SECRET=backend-container-recreation-jwt-secret-32chars \
        -e JWT__EXPIRATION=1h -e SANDBOX__ENABLED=false -e SMTP__ENABLED=false \
        "$image" >/dev/null
    actual_volume="$(docker inspect -f '{{range .Mounts}}{{if eq .Destination "/var/lib/evolith/git"}}{{.Name}}{{end}}{{end}}' "$container")"
    [[ "$actual_volume" == "$volume" ]] || fail "Backend did not mount expected Git volume"
    wait_ready
}
auth_fields() {
    python3 - "$1" <<'PY'
import json, sys
p=json.load(open(sys.argv[1], encoding='utf-8'))
assert p.get('success') and p.get('data'), p
print(p['data']['token'], p['data']['tenant']['id'])
PY
}
repo_id_from() {
    python3 - "$1" <<'PY'
import json, sys
p=json.load(open(sys.argv[1], encoding='utf-8'))
assert p.get('success') and p.get('data'), p
print(p['data']['id'])
PY
}
assert_list() {
    python3 - "$1" "$2" <<'PY'
import json, sys
p=json.load(open(sys.argv[1], encoding='utf-8'))
assert p.get('success') and p.get('data'), p
assert any(r.get('id') == sys.argv[2] for r in p['data'].get('repos', [])), p
PY
}

for cmd in curl python3 createdb dropdb git docker; do command -v "$cmd" >/dev/null || fail "missing command: $cmd"; done
docker info >/dev/null 2>&1 || fail "Docker daemon unavailable"
[[ -x "$repo_root/$EVOLITH_BINARY" ]] || fail "missing Evolith binary: $EVOLITH_BINARY"

mkdir -p "$work_dir/image"
cp "$repo_root/$EVOLITH_BINARY" "$work_dir/image/evolith"
chmod 755 "$work_dir/image/evolith"
cat >"$work_dir/image/Dockerfile" <<'DOCKERFILE'
FROM ubuntu:24.04
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends ca-certificates curl git libssl3 libsqlite3-0 passwd && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY evolith /app/evolith
RUN useradd --uid 1000 --create-home --shell /usr/sbin/nologin evolith && mkdir -p /var/lib/evolith/git && chown -R evolith:evolith /app /var/lib/evolith
USER 1000:1000
ENTRYPOINT ["/app/evolith"]
DOCKERFILE
docker build -q -t "$image" "$work_dir/image" >/dev/null
docker volume create "$volume" >/dev/null
createdb --maintenance-db="$POSTGRES_ADMIN_URL" "$db_name"

log "starting first Evolith Backend container"
start_container first
curl -sfS -H 'Content-Type: application/json' \
    -d "{\"email\":\"$email\",\"username\":\"$username\",\"password\":\"$password\"}" \
    "http://127.0.0.1:${PORT}/api/v1/auth/register" >"$work_dir/register.json"
read -r token tenant_id < <(auth_fields "$work_dir/register.json")
curl -sfS -H 'Content-Type: application/json' -H "Authorization: Bearer $token" \
    -d '{"name":"container-durability","seed_template":false}' \
    "http://127.0.0.1:${PORT}/api/v1/tenant/${tenant_id}/repos" >"$work_dir/create.json"
repo_id="$(repo_id_from "$work_dir/create.json")"
remote="http://127.0.0.1:${PORT}/repos/${repo_id}"

git init -b main "$work_dir/repo" >/dev/null
git -C "$work_dir/repo" config user.name "Container Test"
git -C "$work_dir/repo" config user.email "container@example.invalid"
printf 'first\n' >"$work_dir/repo/README.md"
git -C "$work_dir/repo" add README.md
git -C "$work_dir/repo" commit -m first >/dev/null
git -C "$work_dir/repo" branch feature/recreation
printf 'second\n' >>"$work_dir/repo/README.md"
git -C "$work_dir/repo" commit -am second >/dev/null
git -C "$work_dir/repo" tag v1.0.0
expected_sha="$(git -C "$work_dir/repo" rev-parse HEAD)"
git -C "$work_dir/repo" remote add origin "$remote"
git -C "$work_dir/repo" -c "http.extraHeader=Authorization: Bearer $token" push origin --all >/dev/null
git -C "$work_dir/repo" -c "http.extraHeader=Authorization: Bearer $token" push origin --tags >/dev/null

log "deleting Backend container and recreating with the same Git volume"
stop_container
start_container second
curl -sfS -H 'Content-Type: application/json' \
    -d "{\"email\":\"$email\",\"password\":\"$password\"}" \
    "http://127.0.0.1:${PORT}/api/v1/auth/login" >"$work_dir/login.json"
read -r token2 tenant2 < <(auth_fields "$work_dir/login.json")
[[ "$tenant2" == "$tenant_id" ]] || fail "tenant changed after recreation"
curl -sfS -H "Authorization: Bearer $token2" \
    "http://127.0.0.1:${PORT}/api/v1/tenant/${tenant_id}/repos" >"$work_dir/list.json"
assert_list "$work_dir/list.json" "$repo_id"
git -c "http.extraHeader=Authorization: Bearer $token2" clone --branch main "$remote" "$work_dir/clone" >/dev/null
[[ "$(git -C "$work_dir/clone" rev-parse HEAD)" == "$expected_sha" ]] || fail "main SHA changed"
git -c "http.extraHeader=Authorization: Bearer $token2" ls-remote "$remote" >"$work_dir/refs"
grep -Fq $'\trefs/heads/feature/recreation' "$work_dir/refs" || fail "feature branch missing"
grep -Fq $'\trefs/tags/v1.0.0' "$work_dir/refs" || fail "tag missing"
log "real Backend container recreation preserved login, metadata, clone and refs"
