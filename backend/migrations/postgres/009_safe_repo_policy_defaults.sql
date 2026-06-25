-- EVO-113: align git_repos defaults with the Git-Centric safety policy.

ALTER TABLE git_repos
    ALTER COLUMN auto_merge SET DEFAULT FALSE,
    ALTER COLUMN require_review SET DEFAULT TRUE;
