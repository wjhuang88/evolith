-- EVO-101: git_repos table — git-centric storage metadata index
-- This table stores metadata for git repositories hosted on Evolith.
-- Actual git objects live on the filesystem (bare repos under storage_path).

CREATE TABLE IF NOT EXISTS git_repos (
    id              UUID PRIMARY KEY,
    tenant_id       UUID NOT NULL,
    name            TEXT NOT NULL,
    description     TEXT NOT NULL DEFAULT '',
    default_branch  TEXT NOT NULL DEFAULT 'main',
    storage_path    TEXT NOT NULL,
    visibility      TEXT NOT NULL DEFAULT 'private',
    auto_merge      BOOLEAN NOT NULL DEFAULT TRUE,
    require_review  BOOLEAN NOT NULL DEFAULT FALSE,
    last_commit_sha TEXT,
    last_committed_at TIMESTAMPTZ,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (tenant_id) REFERENCES tenants(id) ON DELETE CASCADE,
    UNIQUE(tenant_id, name)
);

CREATE INDEX IF NOT EXISTS idx_git_repos_tenant ON git_repos(tenant_id);
