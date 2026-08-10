# EVO-108 Skill / CLI / MCP Indexer

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- [Git-Centric Platform Proposal](../../proposals/GIT-CENTRIC-PLATFORM.md)
- 父 Epic: [EVO-100](EVO-100-git-centric-platform-foundation.md)
- 依赖: [EVO-103](EVO-103-repo-context-and-smart-http.md)

## Summary

- 类型：feature / technical
- 优先级：P0
- 状态：Proposed
- 父 Epic: EVO-100

## Problem Or Outcome

实现 git push 触发的 Indexer 服务：扫描 repo 变更路径中的 `SKILL.md` / `interface.yaml` / `tool.yaml` 文件，解析 YAML frontmatter，写入 `skill_index` / `cli_index` / `mcp_tool_index` 表。让 vibe coding 的副产品自动注册为可发现资源。

## Goal And Non-Goals

- **Goal**：
  1. `skill_index` / `cli_index` / `mcp_tool_index` 三张表 + Repository trait + 双实现 + migration
  2. Indexer 服务：监听 git push 事件（来自 EVO-105 commit API 触发）→ 列出变更 paths → 匹配 pattern → 解析 frontmatter → 写入 / 更新 / 删除 index 行
  3. Path pattern 识别：
     - `SKILL.md`（精确匹配）+ `**/SKILL.md`（任意层级）
     - `interface.yaml` + `cli/**/interface.yaml`
     - `tool.yaml` + `mcp/**/tool.yaml`
  4. Frontmatter 解析：使用现有 `serde_norway`（EVO-076 Done）+ Markdown body 分离
  5. 失败处理：单个文件解析失败不阻塞其他；错误入 indexer log
- **Non-goals**：
  - 不实现全仓定期 reconcile（Phase 5+ 评估必要性）
  - 不实现跨仓 federation（独立 EVO）
  - 不实现 schema 验证（仅解析 frontmatter；schema 验证留给调用方）
  - 不实现 UI（EVO-109）

## Dependencies And Blockers

- 依赖 EVO-103 smart HTTP 写事件
- 依赖 EVO-105 commit API 触发点
- 依赖 EVO-118-H-C 的 `repo.push.completed.v1` durable producer/consumer boundary；Indexer
  订阅必须保留 Repo/Ref/Path/Commit provenance，并支持重复 delivery 幂等。
- 间接依赖现有 `service-skill/src/parser.rs` / `service-snippet/src/parser.rs` / `service-tool/src/mcp.rs` 的 frontmatter 解析逻辑（可复用或迁移）

## Governing ADRs, Specs Or Decisions

- [ADR-0004 Git-Centric Storage](../../decisions/ADR-0004-git-centric-storage.md)
- [SKILL.md 格式规范](../../reference/formats/SKILL-FORMAT.md)
- [CLI 友好接口格式规范](../../reference/formats/CLI-INTERFACE-FORMAT.md)

## Acceptance Criteria

- [ ] 三张 index 表 + Repository trait + 双实现 + migration
- [ ] Indexer service 启动后订阅 git push 事件（事件总线或 DB poll，二选一）
- [ ] Path pattern 匹配单元测试覆盖各种情况（root、nested、deleted file）
- [ ] Frontmatter 解析：复用现有 parser；indexer 不重新实现解析逻辑
- [ ] 写入：commit 中新增 SKILL.md → index 表新增一行；修改 → 更新行；删除 → 删除行
- [ ] UNIQUE 约束 `(repo_id, ref, path)` 冲突时正确 upsert
- [ ] 单文件解析失败不影响其他文件处理
- [ ] indexer 性能：单 commit 处理 100 个匹配文件 < 1s
- [ ] `cargo test --workspace` 与 `cargo clippy --workspace --all-targets -- -D warnings` 全绿

## Validation Evidence Required

- E2E 测试：push 一个含 SKILL.md 的 commit → 5s 内 skill_index 出现对应行
- E2E 测试：删除 SKILL.md → skill_index 行消失
- E2E 测试：修改 SKILL.md frontmatter → skill_index 更新 name/version/description
- 错误注入测试：损坏的 YAML frontmatter → indexer 记录错误日志但不崩溃

## Residual Work Destination

- 全仓 reconcile job（Phase 5+）
- 跨仓 federation（独立 EVO）
- indexer 可视化（debug 工具；Phase 5+）

## Source Snapshot

- Source: 用户反馈 2026-06-23
- Decision context: 仓库是 substrate，skill/CLI/MCP 是 Pages 式衍生能力；indexer 是 Pages 引擎
- 类比：GitHub Pages 检测 `index.html`；Evolith indexer 检测 `SKILL.md` / `interface.yaml` / `tool.yaml`
