#!/bin/bash
# =============================================================================
# Evolith Development Environment Launcher
# =============================================================================
#
# Usage:
#   ./scripts/dev.sh start   - Start infrastructure + backend + frontend
#   ./scripts/dev.sh stop    - Stop all services and clean up
#   ./scripts/dev.sh infra   - Start only infrastructure (PostgreSQL, Redis, MinIO)
#   ./scripts/dev.sh backend - Start only backend (assumes infra is running or SQLite)
#   ./scripts/dev.sh status  - Show status of all services
#   ./scripts/dev.sh logs    - Tail backend and frontend logs
#   ./scripts/dev.sh clean   - Stop all and remove Docker volumes
#
# Environment:
#   Uses .env.development by default. Override with ENV_FILE=.env.custom
# =============================================================================

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ENV_FILE="${ENV_FILE:-$PROJECT_ROOT/.env.development}"
PID_DIR="$PROJECT_ROOT/.dev-pids"
LOG_DIR="$PROJECT_ROOT/.dev-logs"
COMPOSE_FILE="$PROJECT_ROOT/docker-compose.yml"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info()  { echo -e "${BLUE}[INFO]${NC}  $*"; }
log_ok()    { echo -e "${GREEN}[OK]${NC}    $*"; }
log_warn()  { echo -e "${YELLOW}[WARN]${NC}  $*"; }
log_error() { echo -e "${RED}[ERROR]${NC} $*"; }

# Ensure directories exist
ensure_dirs() {
    mkdir -p "$PID_DIR" "$LOG_DIR"
}

# Load environment variables from file
load_env() {
    if [ -f "$ENV_FILE" ]; then
        set -a
        # shellcheck disable=SC1090
        source <(grep -v '^\s*#' "$ENV_FILE" | grep -v '^\s*$')
        set +a
        log_info "Loaded environment from $(basename "$ENV_FILE")"
    else
        log_warn "No env file found at $ENV_FILE — using defaults"
    fi
}

# Check if a process is running by PID file
is_running() {
    local pidfile="$PID_DIR/$1.pid"
    if [ -f "$pidfile" ]; then
        local pid
        pid=$(cat "$pidfile")
        if kill -0 "$pid" 2>/dev/null; then
            return 0
        fi
        # Stale PID file
        rm -f "$pidfile"
    fi
    return 1
}

# Start infrastructure services (Docker Compose)
start_infra() {
    log_info "Starting infrastructure services (PostgreSQL, Redis, MinIO)..."
    docker compose -f "$COMPOSE_FILE" up -d postgres redis minio 2>&1 | while read -r line; do
        echo "  $line"
    done

    log_info "Waiting for services to be healthy..."
    local max_wait=60
    local waited=0
    while [ $waited -lt $max_wait ]; do
        local all_healthy=true
        for svc in postgres redis; do
            local status
            status=$(docker compose -f "$COMPOSE_FILE" ps --format json "$svc" 2>/dev/null | grep -o '"Health":"[^"]*"' | head -1 || echo "")
            if [[ "$status" != *'"Health":"healthy"'* ]]; then
                all_healthy=false
                break
            fi
        done
        if $all_healthy; then
            break
        fi
        sleep 2
        waited=$((waited + 2))
    done

    if [ $waited -ge $max_wait ]; then
        log_warn "Some services may not be fully healthy yet (waited ${max_wait}s)"
    else
        log_ok "Infrastructure services are healthy"
    fi
}

# Stop infrastructure services
stop_infra() {
    log_info "Stopping infrastructure services..."
    docker compose -f "$COMPOSE_FILE" down 2>&1 | while read -r line; do
        echo "  $line"
    done
    log_ok "Infrastructure stopped"
}

# Start backend
start_backend() {
    if is_running "backend"; then
        log_warn "Backend is already running (PID $(cat "$PID_DIR/backend.pid"))"
        return 0
    fi

    log_info "Building and starting Rust backend..."
    cd "$PROJECT_ROOT/backend"

    # Build first (fail fast if compilation errors)
    if ! cargo build 2>"$LOG_DIR/backend-build.log"; then
        log_error "Backend build failed! Check $LOG_DIR/backend-build.log"
        return 1
    fi

    # Run in background
    cargo run > "$LOG_DIR/backend.log" 2>&1 &
    local pid=$!
    echo "$pid" > "$PID_DIR/backend.pid"
    cd "$PROJECT_ROOT"

    # Wait for backend to be ready
    log_info "Waiting for backend to start (PID $pid)..."
    local max_wait=30
    local waited=0
    while [ $waited -lt $max_wait ]; do
        if curl -sf http://localhost:8080/health/live >/dev/null 2>&1; then
            log_ok "Backend is running at http://localhost:8080"
            return 0
        fi
        if ! kill -0 "$pid" 2>/dev/null; then
            log_error "Backend process died. Check $LOG_DIR/backend.log"
            rm -f "$PID_DIR/backend.pid"
            return 1
        fi
        sleep 1
        waited=$((waited + 1))
    done

    log_warn "Backend started but health check not responding yet (PID $pid)"
    log_warn "Check $LOG_DIR/backend.log for details"
}

# Start frontend
start_frontend() {
    if is_running "frontend"; then
        log_warn "Frontend is already running (PID $(cat "$PID_DIR/frontend.pid"))"
        return 0
    fi

    log_info "Starting Next.js frontend..."
    cd "$PROJECT_ROOT/frontend"

    # Install dependencies if needed
    if [ ! -d "node_modules" ]; then
        log_info "Installing frontend dependencies..."
        npm install > "$LOG_DIR/frontend-install.log" 2>&1
    fi

    npm run dev > "$LOG_DIR/frontend.log" 2>&1 &
    local pid=$!
    echo "$pid" > "$PID_DIR/frontend.pid"
    cd "$PROJECT_ROOT"

    # Wait for frontend to be ready
    log_info "Waiting for frontend to start (PID $pid)..."
    local max_wait=30
    local waited=0
    while [ $waited -lt $max_wait ]; do
        if curl -sf http://localhost:3000 >/dev/null 2>&1; then
            log_ok "Frontend is running at http://localhost:3000"
            return 0
        fi
        if ! kill -0 "$pid" 2>/dev/null; then
            log_error "Frontend process died. Check $LOG_DIR/frontend.log"
            rm -f "$PID_DIR/frontend.pid"
            return 1
        fi
        sleep 1
        waited=$((waited + 1))
    done

    log_warn "Frontend started but not responding yet (PID $pid)"
    log_warn "Check $LOG_DIR/frontend.log for details"
}

# Stop a service by PID file
stop_service() {
    local name="$1"
    local pidfile="$PID_DIR/$name.pid"

    if [ -f "$pidfile" ]; then
        local pid
        pid=$(cat "$pidfile")
        if kill -0 "$pid" 2>/dev/null; then
            log_info "Stopping $name (PID $pid)..."
            kill "$pid" 2>/dev/null || true
            # Wait for graceful shutdown
            local waited=0
            while kill -0 "$pid" 2>/dev/null && [ $waited -lt 10 ]; do
                sleep 1
                waited=$((waited + 1))
            done
            if kill -0 "$pid" 2>/dev/null; then
                log_warn "Force killing $name (PID $pid)..."
                kill -9 "$pid" 2>/dev/null || true
            fi
            log_ok "$name stopped"
        else
            log_info "$name is not running (stale PID file)"
        fi
        rm -f "$pidfile"
    else
        log_info "$name is not running"
    fi
}

# Show status
show_status() {
    echo ""
    echo "=== Evolith Development Environment Status ==="
    echo ""

    # Infrastructure
    echo "Infrastructure:"
    for svc in postgres redis minio; do
        local status
        status=$(docker compose -f "$COMPOSE_FILE" ps --format "{{.Status}}" "$svc" 2>/dev/null || echo "Not running")
        if [[ "$status" == *"Up"* ]]; then
            echo -e "  ${GREEN}●${NC} $svc: $status"
        else
            echo -e "  ${RED}●${NC} $svc: ${status:-Not running}"
        fi
    done

    echo ""
    echo "Application:"

    # Backend
    if is_running "backend"; then
        echo -e "  ${GREEN}●${NC} Backend:  Running (PID $(cat "$PID_DIR/backend.pid")) — http://localhost:8080"
    else
        echo -e "  ${RED}●${NC} Backend:  Not running"
    fi

    # Frontend
    if is_running "frontend"; then
        echo -e "  ${GREEN}●${NC} Frontend: Running (PID $(cat "$PID_DIR/frontend.pid")) — http://localhost:3000"
    else
        echo -e "  ${RED}●${NC} Frontend: Not running"
    fi

    echo ""
}

# Tail logs
tail_logs() {
    log_info "Tailing backend and frontend logs (Ctrl+C to stop)..."
    tail -f "$LOG_DIR/backend.log" "$LOG_DIR/frontend.log" 2>/dev/null || log_warn "No log files found"
}

# Print usage
usage() {
    echo "Usage: $0 {start|stop|infra|backend|status|logs|clean}"
    echo ""
    echo "Commands:"
    echo "  start    Start infrastructure + backend + frontend"
    echo "  stop     Stop all services (backend, frontend, infrastructure)"
    echo "  infra    Start only infrastructure (PostgreSQL, Redis, MinIO)"
    echo "  backend  Start only backend (assumes infra or SQLite)"
    echo "  status   Show status of all services"
    echo "  logs     Tail backend and frontend logs"
    echo "  clean    Stop everything and remove Docker volumes"
}

# =============================================================================
# Main
# =============================================================================

case "${1:-}" in
    start)
        ensure_dirs
        load_env
        start_infra
        start_backend
        start_frontend
        echo ""
        echo "========================================="
        echo "  Evolith Development Environment Ready"
        echo "========================================="
        echo ""
        echo "  Frontend: http://localhost:3000"
        echo "  Backend:  http://localhost:8080"
        echo "  Health:   http://localhost:8080/health"
        echo ""
        echo "  Stop with:  ./scripts/dev.sh stop"
        echo "  Logs with:  ./scripts/dev.sh logs"
        echo "  Status:     ./scripts/dev.sh status"
        echo ""
        ;;
    stop)
        ensure_dirs
        stop_service "frontend"
        stop_service "backend"
        stop_infra
        log_ok "All services stopped"
        ;;
    infra)
        ensure_dirs
        load_env
        start_infra
        ;;
    backend)
        ensure_dirs
        load_env
        start_backend
        ;;
    status)
        ensure_dirs
        show_status
        ;;
    logs)
        tail_logs
        ;;
    clean)
        ensure_dirs
        stop_service "frontend"
        stop_service "backend"
        log_info "Stopping infrastructure and removing volumes..."
        docker compose -f "$COMPOSE_FILE" down -v 2>&1 | while read -r line; do
            echo "  $line"
        done
        rm -rf "$PID_DIR" "$LOG_DIR"
        log_ok "Cleaned up everything (Docker volumes removed)"
        ;;
    *)
        usage
        exit 1
        ;;
esac
