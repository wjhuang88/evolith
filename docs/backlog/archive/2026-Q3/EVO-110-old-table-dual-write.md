# EVO-110 旧表（skills / snippets / tools）双写适配

> **Decision update (2026-08-08): Dropped.** 项目尚未上线，[ADR-0009](../../../decisions/ADR-0009-no-prelaunch-registry-compatibility.md) 决定不建设旧表双写、回填或旧 API 兼容层。以下内容保留为被否决方案的历史上下文；Repo-derived 承接与直接清理归 [EVO-122](../../active/EVO-122-retire-prelaunch-registry-backend.md)。

## Required Reads

- [Product Backlog](../../PRODUCT-BACKLOG.md)
- [Git-Centric Platform Proposal](../../../proposals/GIT-CENTRIC-PLATFORM.md)
- 父 Epic: [EVO-100](../../active/EVO-100-git-centric-platform-foundation.md)
- 依赖: [EVO-108](../../active/EVO-108-skill-cli-mcp-indexer.md)

## Summary

- 类型：tech-debt / compatibility
- 优先级：P1
- 状态：Dropped
- 父 Epic: EVO-100

## Problem Or Outcome

旧 `skills` / `snippets` / `tools` 三张表是当前 API 的数据源（`GET /skills/{id}` 等）。迁移到 git-centric 架构后，这些表的写入路径必须收敛到 `POST /repos/{id}/files`，由 indexer（EVO-108）触发双写，保证旧 API 在迁移期内仍可用。

## Goal And Non-Goals

- **Goal**：
  1. 旧表写入路径收敛：禁止任何代码路径直接 INSERT/UPDATE/DELETE skills/snippets/tools 行；所有变更必须经过 git 文件写入 + indexer
  2. Indexer 双写：indexer 在更新 index 表时同步 upsert 旧表行
  3. 旧 API 行为不变：`GET /skills/{id}` 仍返回正确数据（从旧表读，或从 git 读后 fallback）
  4. 回填脚本：现存旧表数据 → 创建仓库 → 写入文件 → commit → 回填指针
- **Non-goals**：
  - 不实现旧表 → index 表数据迁移（迁移期 index 表可为空，旧表保留完整数据）
  - 不删除旧表（独立 EVO 在 Phase 5+）
  - 不改旧表 schema（独立 EVO 在 Phase 5+ 评估）

## Dependencies And Blockers

- 依赖 EVO-108 indexer（双写逻辑挂载在 indexer 中）
- 依赖 EVO-105 commit API（写入路径收敛点）
- 软依赖 EVO-111（sandbox 删除前必须完成，否则旧 execute API 仍可被调用）

## Governing ADRs, Specs Or Decisions

- [ADR-0004 Git-Centric Storage](../../../decisions/ADR-0004-git-centric-storage.md)

## Acceptance Criteria

- [ ] 旧表 INSERT/UPDATE/DELETE 仅允许在 `service-git` crate 中调用（其他 crate 编译时通过 linter 或 code review 阻止）
- [ ] Indexer 在更新 `skill_index` / `cli_index` / `mcp_tool_index` 时同步 upsert `skills` / `snippets` / `tools` 表行
- [ ] 旧表字段映射：
  - `skills.skill_md` ← git blob 内容
  - `skills.tags` ← frontmatter `tags`
  - `snippets.content` + `snippets.code` ← interface.yaml + handler.{ext}
  - `tools.input_schema` / `output_schema` ← tool.yaml frontmatter
- [ ] 回填脚本 `scripts/migrate_content_to_git.sh` 实现：
  - 遍历旧表所有行
  - 每行创建对应 repo（不存在时）或选择已有 repo
  - 写入 SKILL.md / interface.yaml / tool.yaml
  - commit
  - 触发 indexer 更新 + 双写
- [ ] 回填脚本 `--dry-run` 模式：仅打印计划，不实际写
- [ ] 旧 API 行为回归测试：`GET /skills/{id}` / `GET /tools/{id}` / `GET /snippets/{id}` 在迁移后行为不变
- [ ] `cargo test --workspace` 与 `cargo clippy --workspace --all-targets -- -D warnings` 全绿

## Validation Evidence Required

- 单元测试：indexer 双写逻辑（mock DB，验证 skills 表同步更新）
- 集成测试：完整迁移一个 skill → 旧 API `GET /skills/{id}` 返回与迁移前一致
- 回填脚本 SHA-256 校验：commit 后从 git 读取的内容与原 TEXT 列的 SHA-256 一致
- 双轨期无脏数据：旧表与 index 表内容一致（`diff` 脚本验证）

## Residual Work Destination

- 旧表删除（Phase 5+，独立 EVO）
- 旧表 schema 评估（是否需要 `git_repo_id` 等列；Phase 5+）
- 写入路径 lint 自动化（独立 EVO）
- 旧 API deprecation 公告（独立 EVO）

## Source Snapshot

- Source: 用户反馈 2026-06-23
- Decision context: 迁移期不破坏现有 API；保持向后兼容
- 关键约束：迁移期写入路径必须收敛，否则会出现"git 与 DB 不一致"状态
