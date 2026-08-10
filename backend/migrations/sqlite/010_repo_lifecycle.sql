-- EVO-118-F: make DB/Filesystem lifecycle observable and recoverable.
ALTER TABLE git_repos ADD COLUMN lifecycle_status TEXT NOT NULL DEFAULT 'ACTIVE';
CREATE INDEX IF NOT EXISTS idx_git_repos_lifecycle_status ON git_repos(lifecycle_status);
