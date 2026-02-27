# Database Migrations

This directory contains SQL migration files for the Evolith database.

## Migration Files

| File | Description |
|------|-------------|
| `001_initial_schema.sql` | Creates initial database tables |
| `002_seed_data.sql` | Seeds development data (only in dev) |

## Running Migrations

### Using sqlx-cli

```bash
# Install sqlx-cli
cargo install sqlx-cli --features sqlite,postgres

# Create database
sqlx database create

# Run migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert
```

### SQLite (Development)

```bash
# Create database file
sqlite3 dev.db < migrations/001_initial_schema.sql
sqlite3 dev.db < migrations/002_seed_data.sql
```

### PostgreSQL (Production)

```bash
# Using psql
psql -U evolith -d evolith -f migrations/001_initial_schema.sql
```

## Schema Notes

### Multi-Database Compatibility

The migration files are designed to work with:
- SQLite (development)
- PostgreSQL (production)
- MySQL (alternative)

For PostgreSQL-specific optimizations, additional migration files can be created.

### UUID Handling

- SQLite: Uses TEXT for UUID storage
- PostgreSQL: Can use native UUID type (migration needed)
- MySQL: Uses CHAR(36) or BINARY(16)

### JSON Handling

- SQLite: Uses TEXT with JSON functions
- PostgreSQL: Uses native JSONB type (migration needed)
- MySQL: Uses JSON type
