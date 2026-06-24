# EVO-101 git_repos 表与双轨 migration

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- [Git-Centric Platform Proposal](../../proposals/GIT-CENTRIC-PLATFORM.md)
- [ADR-0004 Git-Centric Storage](../../decisions/ADR-0004-git-centric-storage.md)
- 父 Epic: [EVO-100](EVO-100-git-centric-platform-foundation.md)

## Summary

- 类型：tech-debt / data-model
- 优先级：P0
- 状态：Done
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

## Phase E'-1 落地补充（来自 EVO-100 Epic 启动前的兼容性 review）

> 在 EVO-101 实施期间一并完成下列两项 governance 整理，确保 Phase E'-1 启动不引入 schema / trait 漂移。

### 子任务 ST-1：Tenant quota 适配 git-centric 模型

`backend/crates/domain/src/tenant.rs` 的 `TenantQuotas` 当前含 DB-centric quota（`max_tools` / `max_skills` / `max_snippets` / `max_storage_mb` 含义为 skill 包存储）。git-centric 下需调整：

- **新增** `max_repos: u32`（git-centric 主 quota；默认值与 plan 对照表同步）
- **重新定义** `max_storage_mb: u32` 为「所有 git 仓总存储」含义（不再是 skill 包存储）
- **移除** `max_tools` / `max_skills` / `max_snippets`（隐含在 repo 文件数量中；保留为软指标字段或从 `TenantUsage` 移除）
- `TenantQuotas::for_plan` 默认值重新计算（Free / Starter / Pro / Enterprise 各档 max_repos 与 max_storage_mb）
- 现有 `TenantUsage` 字段 `current_tools/skills/snippets` 同样处理（保留/移除二选一）

修改涉及 `tenant.rs` 的 schema 字段、`for_plan` 默认值、所有测试用例。`infra/src/db/pg_tenant_repo.rs` 与 `sqlite_tenant_repo.rs` 不需 schema migration（因为 quota / usage 字段为 JSONB / TEXT），但需保证兼容性。

### 子任务 ST-2：ExecutionProvider 简化准备（不彻底删除）

`backend/crates/common/src/execution.rs` 的 `ExecutionProvider` trait 当前含三个 `ExecutionCaller` 变体（Skill / Cli / McpTool）。EVO-111 整层删除 sandbox 时会一并清理，但本 Story 提前做"dead path 标注"以避免后续误用：

- 给 `ExecutionCaller::Skill { skill_id, runtime }` 和 `ExecutionCaller::Cli { snippet_id, command }` 变体加 `#[deprecated(since = "...", note = "will be removed in EVO-111; sandbox execution deprecated per ADR-0005")]` 标注
- 给 `ExecutionPayload::Code { source, language }` 和 `ExecutionPayload::Command { command, args }` 变体同样标注
- `CompositeProvider::route()` 匹配分支保留但加 `#[allow(deprecated)]` 注释（仍编译；dead path 警告可见）
- 不删除 DockerSandboxProvider（EVO-111 收口时统一删）
- 单元测试中关于 Code / Command payload 的 mock 路径保留（标 deprecated），等 EVO-111 收口

**目的**：让 Phase E'-1 启动后所有 ExecutionProvider 调用方都明确"只有 HttpProxy 是活路径"，但暂不破坏现有编译，避免 Phase E'-1 阻塞在 trait 重构。

## Acceptance Criteria

### EVO-101 主线

- [ ] `backend/migrations/sqlite/007_git_repos.sql` 与 `backend/migrations/postgres/007_git_repos.sql` 同步落地
- [ ] `git_repos` 表包含字段：`id` (UUID PK)、`tenant_id` (FK)、`name`、`description`、`default_branch`、`storage_path`、`visibility`、`auto_merge`、`require_review`、`last_commit_sha`、`created_at`、`updated_at`
- [ ] `(tenant_id, name)` UNIQUE 约束
- [ ] `idx_git_repos_tenant` 索引
- [ ] `GitRepoRepository` trait 定义在 `backend/crates/domain/src/repository.rs`
- [ ] `SqliteGitRepoRepository` 与 `PgGitRepoRepository` 双实现位于 `backend/crates/infra/src/db/`
- [ ] Repository 集成测试覆盖 create / find_by_id / find_by_tenant / update / delete（参考现有 SqliteSkillRepository 测试模式）
- [ ] 双轨 migration 在 SQLite 内存库与 PostgreSQL 测试容器上分别跑通

### 子任务 ST-1

- [ ] `TenantQuotas` 新增 `max_repos: u32` 字段并 `for_plan` 默认值合理（Free=3 / Starter=20 / Pro=100 / Enterprise=u32::MAX）
- [ ] `max_storage_mb` 含义从「skill 包存储」改为「所有 git 仓总存储」并更新 `CONFIG.md` 与 TECH-STACK.md 中相关引用
- [ ] `max_tools/skills/snippets` 字段处置二选一已记录并实施（推荐保留为软指标 JSON 字段，从 `TenantQuotas` struct 移除并迁入 `settings` JSONB）
- [ ] 现有 tenant repository 集成测试更新覆盖新字段

### 子任务 ST-2

- [ ] `ExecutionCaller::Skill` 和 `::Cli` 变体加 `#[deprecated]` 标注
- [ ] `ExecutionPayload::Code` 和 `::Command` 变体加 `#[deprecated]` 标注
- [ ] `CompositeProvider::route()` 编译通过（带 `#[allow(deprecated)]`）
- [ ] 现有 309 tests 通过（不破坏 Phase 7 已通过测试）

### 全局门禁

- [ ] `cargo test --workspace` 全绿
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` 0 errors
- [ ] `bun run build` 0 errors（frontend 未变更应仍通过）

## Validation Evidence Required

- `cargo test -p infra --test git_repo_tests` 通过（新建测试文件）
- `cargo sqlx migrate run --source backend/migrations/sqlite` 与 `--source backend/migrations/postgres` 成功
- 手工 psql / sqlite3 查询验证表结构与索引存在
- 集成测试覆盖 CRUD + tenant scope + UNIQUE 冲突场景
- ST-1: `cargo test -p domain` 验证 `TenantQuotas::for_plan` 默认值
- ST-2: `cargo build` 无 warning 升级（除 `#[deprecated]` 自身提示）

## Residual Work Destination

- 字段调整（如新增 `webhook_secret`）后续 EVO 增量补
- `skill_index` / `cli_index` / `mcp_tool_index` 由 EVO-108 处理

## Source Snapshot

- Source: 用户反馈 2026-06-23（开发目标变更 / Git-Centric Platform）
- Decision context: 见 `GIT-CENTRIC-PLATFORM.md` §4.1
- Prior discussion: 2026-06-23 per-resource 仓模型已弃用，本表不绑 `resource_type` / `resource_id`
