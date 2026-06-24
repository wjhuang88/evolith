-- EVO-101 ST-1: Add git-centric quota columns to tenants table
-- Old columns (max_tools, max_skills, max_snippets, current_tools, current_skills, current_snippets)
-- remain in the DB for backward compatibility but are no longer used by the application.

ALTER TABLE tenants ADD COLUMN max_repos INTEGER NOT NULL DEFAULT 10;
ALTER TABLE tenants ADD COLUMN current_repos INTEGER NOT NULL DEFAULT 0;
