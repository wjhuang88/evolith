-- EVO-113: align git_repos defaults with the Git-Centric safety policy.
-- SQLite cannot alter column defaults in place, so rebuild the table.

PRAGMA foreign_keys = OFF;

CREATE TABLE git_repos_new (
    id              TEXT PRIMARY KEY,
    tenant_id       TEXT NOT NULL,
    name            TEXT NOT NULL,
    description     TEXT NOT NULL DEFAULT '',
    default_branch  TEXT NOT NULL DEFAULT 'main',
    storage_path    TEXT NOT NULL,
    visibility      TEXT NOT NULL DEFAULT 'private',
    auto_merge      INTEGER NOT NULL DEFAULT 0,
    require_review  INTEGER NOT NULL DEFAULT 1,
    last_commit_sha TEXT,
    last_committed_at TEXT,
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at      TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (tenant_id) REFERENCES tenants(id) ON DELETE CASCADE,
    UNIQUE(tenant_id, name)
);

INSERT INTO git_repos_new (
    id, tenant_id, name, description, default_branch, storage_path,
    visibility, auto_merge, require_review, last_commit_sha, last_committed_at,
    created_at, updated_at
)
SELECT
    id, tenant_id, name, description, default_branch, storage_path,
    visibility, auto_merge, require_review, last_commit_sha, last_committed_at,
    created_at, updated_at
FROM git_repos;

DROP TABLE git_repos;
ALTER TABLE git_repos_new RENAME TO git_repos;

CREATE INDEX IF NOT EXISTS idx_git_repos_tenant ON git_repos(tenant_id);

PRAGMA foreign_keys = ON;
