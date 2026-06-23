# EVO-101 git_repos 表与双轨 migration

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- [Git-Centric Platform Proposal](../../proposals/GIT-CENTRIC-PLATFORM.md)
- [ADR-0004 Git-Centric Storage](../../decisions/ADR-0004-git-centric-storage.md)
- 父 Epic: [EVO-100](EVO-100-git-centric-platform-foundation.md)

## Summary

- 类型：tech-debt / data-model
- 优先级：P0
- 状态：Proposed
- 父 Epic: EVO-100

## Problem Or Outcome

新增 `git_repos` 第一类实体表，作为 git 托管的 metadata 索引；建立 SQLite/PostgreSQL 双轨 migration；为后续 Repo CRUD API（EVO-103）提供底层 schema。

## Goal And Non-Goals

- **Goal**：完成 `git_repos` 表 schema 设计 + Repository trait + Sqlite/Pg 双实现 + migration 文件 + 集成测试。
- **Non-goals**：
  - 不实现 Repo CRUD HTTP API（EVO-103）。
  - 不实现 Repo 磁盘创建 / smart HTTP 协议（EVO-103）。
  - 不实现 `skill_index` / `cli_index` / `mcp_tool_index`（EVO-108）。
  - 不实现旧表迁移（EVO-110）。

## Dependencies And Blockers

- 无硬依赖；可作为 Phase 1 第一个 Story 启动。
- 间接依赖 ADR-0004 决策生效。

## Governing ADRs, Specs Or Decisions

- [ADR-0004 Git-Centric Storage](../../decisions/ADR-0004-git-centric-storage.md)

## Acceptance Criteria

- [ ] `backend/migrations/sqlite/007_git_repos.sql` 与 `backend/migrations/postgres/007_git_repos.sql` 同步落地
- [ ] `git_repos` 表包含字段：`id` (UUID PK)、`tenant_id` (FK)、`name`、`description`、`default_branch`、`storage_path`、`visibility`、`auto_merge`、`require_review`、`last_commit_sha`、`created_at`、`updated_at`
- [ ] `(tenant_id, name)` UNIQUE 约束
- [ ] `idx_git_repos_tenant` 索引
- [ ] `GitRepoRepository` trait 定义在 `backend/crates/domain/src/repository.rs`
- [ ] `SqliteGitRepoRepository` 与 `PgGitRepoRepository` 双实现位于 `backend/crates/infra/src/db/`
- [ ] `cargo test --workspace` 全绿
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` 0 errors
- [ ] Repository 集成测试覆盖 create / find_by_id / find_by_tenant / update / delete（参考现有 SqliteSkillRepository 测试模式）
- [ ] 双轨 migration 在 SQLite 内存库与 PostgreSQL 测试容器上分别跑通

## Validation Evidence Required

- `cargo test -p infra --test git_repo_tests` 通过（新建测试文件）
- `cargo sqlx migrate run --source backend/migrations/sqlite` 与 `--source backend/migrations/postgres` 成功
- 手工 psql / sqlite3 查询验证表结构与索引存在
- 集成测试覆盖 CRUD + tenant scope + UNIQUE 冲突场景

## Residual Work Destination

- 字段调整（如新增 `webhook_secret`）后续 EVO 增量补
- `skill_index` / `cli_index` / `mcp_tool_index` 由 EVO-108 处理

## Source Snapshot

- Source: 用户反馈 2026-06-23（开发目标变更 / Git-Centric Platform）
- Decision context: 见 `GIT-CENTRIC-PLATFORM.md` §4.1
- Prior discussion: 2026-06-23 per-resource 仓模型已弃用，本表不绑 `resource_type` / `resource_id`
