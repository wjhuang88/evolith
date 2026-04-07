#!/usr/bin/env bash
set -euo pipefail

COMPOSE_FILE="${COMPOSE_FILE:-docker-compose.prod.yml}"
HEALTH_URL="${HEALTH_URL:-http://localhost:80/health}"
HEALTH_RETRIES="${HEALTH_RETRIES:-30}"
HEALTH_INTERVAL="${HEALTH_INTERVAL:-2}"

usage() {
    echo "Usage: $0 [deploy|rollback|status]"
    echo ""
    echo "  deploy   - Pull images and deploy (default)"
    echo "  rollback - Roll back to previous images"
    echo "  status   - Show service status and health"
    echo ""
    echo "Environment variables:"
    echo "  COMPOSE_FILE     - Compose file path (default: docker-compose.prod.yml)"
    echo "  HEALTH_URL       - Health check URL (default: http://localhost:80/health)"
    echo "  HEALTH_RETRIES   - Health check retry count (default: 30)"
    echo "  HEALTH_INTERVAL  - Seconds between retries (default: 2)"
    exit 1
}

health_check() {
    echo "[deploy] Waiting for health check..."
    for i in $(seq 1 "${HEALTH_RETRIES}"); do
        if curl -sf "${HEALTH_URL}" > /dev/null 2>&1; then
            echo "[deploy] Health check passed (attempt ${i}/${HEALTH_RETRIES})"
            return 0
        fi
        echo "[deploy] Health check attempt ${i}/${HEALTH_RETRIES} failed, retrying in ${HEALTH_INTERVAL}s..."
        sleep "${HEALTH_INTERVAL}"
    done
    echo "[deploy] Health check failed after ${HEALTH_RETRIES} attempts"
    return 1
}

do_deploy() {
    echo "[deploy] Pulling latest images..."
    docker compose -f "${COMPOSE_FILE}" pull

    echo "[deploy] Saving current image digests for rollback..."
    docker compose -f "${COMPOSE_FILE}" config --images > /tmp/evolith-previous-images.txt 2>/dev/null || true

    echo "[deploy] Starting services..."
    docker compose -f "${COMPOSE_FILE}" up -d --remove-orphans

    if health_check; then
        echo "[deploy] Deployment successful"
        docker compose -f "${COMPOSE_FILE}" ps
    else
        echo "[deploy] Deployment failed — health check did not pass"
        echo "[deploy] Run '$0 rollback' to revert, or check logs with:"
        echo "  docker compose -f ${COMPOSE_FILE} logs --tail=50"
        exit 1
    fi
}

do_rollback() {
    if [ ! -f /tmp/evolith-previous-images.txt ]; then
        echo "[rollback] No previous deployment found — nothing to roll back to"
        exit 1
    fi

    echo "[rollback] Rolling back to previous deployment..."
    docker compose -f "${COMPOSE_FILE}" down

    echo "[rollback] Restarting with previous configuration..."
    docker compose -f "${COMPOSE_FILE}" up -d

    if health_check; then
        echo "[rollback] Rollback successful"
    else
        echo "[rollback] Rollback failed — manual intervention required"
        echo "  docker compose -f ${COMPOSE_FILE} logs --tail=100"
        exit 1
    fi
}

do_status() {
    echo "[status] Service status:"
    docker compose -f "${COMPOSE_FILE}" ps
    echo ""
    echo "[status] Health check:"
    if curl -sf "${HEALTH_URL}" > /dev/null 2>&1; then
        echo "  OK"
    else
        echo "  FAIL"
    fi
}

ACTION="${1:-deploy}"

case "${ACTION}" in
    deploy)  do_deploy ;;
    rollback) do_rollback ;;
    status)  do_status ;;
    *)       usage ;;
esac
