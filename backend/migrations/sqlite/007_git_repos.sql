-- EVO-101: git_repos table — git-centric storage metadata index
-- This table stores metadata for git repositories hosted on Evolith.
-- Actual git objects live on the filesystem (bare repos under storage_path).

CREATE TABLE IF NOT EXISTS git_repos (
    id              TEXT PRIMARY KEY,
    tenant_id       TEXT NOT NULL,
    name            TEXT NOT NULL,
    description     TEXT NOT NULL DEFAULT '',
    default_branch  TEXT NOT NULL DEFAULT 'main',
    storage_path    TEXT NOT NULL,
    visibility      TEXT NOT NULL DEFAULT 'private',
    auto_merge      INTEGER NOT NULL DEFAULT 1,
    require_review  INTEGER NOT NULL DEFAULT 0,
    last_commit_sha TEXT,
    last_committed_at TEXT,
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at      TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (tenant_id) REFERENCES tenants(id) ON DELETE CASCADE,
    UNIQUE(tenant_id, name)
);

CREATE INDEX IF NOT EXISTS idx_git_repos_tenant ON git_repos(tenant_id);
