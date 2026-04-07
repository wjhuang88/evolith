#!/usr/bin/env bash
set -euo pipefail

BACKUP_DIR="${BACKUP_DIR:-./backups}"
RETENTION_DAYS="${RETENTION_DAYS:-30}"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_FILE="${BACKUP_DIR}/evolith_${TIMESTAMP}.sql.gz"

mkdir -p "${BACKUP_DIR}"

usage() {
    echo "Usage: $0 [docker|host]"
    echo ""
    echo "  docker  - Backup from PostgreSQL running in docker-compose (default)"
    echo "  host    - Backup from PostgreSQL on host using pg_dump directly"
    echo ""
    echo "Environment variables:"
    echo "  BACKUP_DIR       - Backup directory (default: ./backups)"
    echo "  RETENTION_DAYS   - Days to keep backups (default: 30)"
    echo "  POSTGRES_USER    - Database user (default: evolith)"
    echo "  POSTGRES_DB      - Database name (default: evolith)"
    echo "  PGPASSWORD       - Database password (host mode only)"
    echo "  PGHOST           - Database host (host mode, default: localhost)"
    echo "  PGPORT           - Database port (host mode, default: 5432)"
    exit 1
}

MODE="${1:-docker}"

case "${MODE}" in
    docker)
        echo "[backup] Dumping PostgreSQL from docker container..."
        docker compose -f docker-compose.prod.yml exec -T postgres \
            pg_dump -U "${POSTGRES_USER:-evolith}" -d "${POSTGRES_DB:-evolith}" \
            --no-owner --no-privileges \
            | gzip > "${BACKUP_FILE}"
        ;;
    host)
        echo "[backup] Dumping PostgreSQL from host..."
        PGHOST="${PGHOST:-localhost}" PGPORT="${PGPORT:-5432}" \
            pg_dump -U "${POSTGRES_USER:-evolith}" -d "${POSTGRES_DB:-evolith}" \
            --no-owner --no-privileges \
            | gzip > "${BACKUP_FILE}"
        ;;
    *)
        usage
        ;;
esac

BACKUP_SIZE=$(du -h "${BACKUP_FILE}" | cut -f1)
echo "[backup] Created: ${BACKUP_FILE} (${BACKUP_SIZE})"

DELETED=$(find "${BACKUP_DIR}" -name "evolith_*.sql.gz" -type f -mtime +${RETENTION_DAYS} -print -delete | wc -l)
if [ "${DELETED}" -gt 0 ]; then
    echo "[backup] Cleaned up ${DELETED} backup(s) older than ${RETENTION_DAYS} days"
fi

TOTAL=$(find "${BACKUP_DIR}" -name "evolith_*.sql.gz" -type f | wc -l)
echo "[backup] Total backups: ${TOTAL}"
echo "[backup] Done."
