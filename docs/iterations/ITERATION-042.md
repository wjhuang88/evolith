# Iteration 042: Phase E'-1 Git Service 基础（git_repos + policy.yaml + 双轨 schema）

> 文档状态：Planned
> 计划发布日期：2026-06-24
> 计划目标：在 Evoith 后端建立 git-centric 平台的最小可运行底层——`git_repos` schema + 双轨 migration、`.evolith/policy.yaml` 解析器、Tenant quota 适配 git-centric 模型、ExecutionProvider 简化准备。
>
> 基线保护：本文件一旦提交，以下「发布计划基线」内容不可因实施或改线而覆写；同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

完成 Phase E'-1 全部落地事项：

- 建立 `git_repos` 表（第一类实体，无 `resource_type` 字段）+ SQLite/PostgreSQL 双轨 migration
- 实现 `GitRepoRepository` trait 与 Sqlite/Pg 双实现 + 集成测试
- 实现 `.evolith/policy.yaml` 规范文档（`docs/reference/formats/POLICY-YAML-FORMAT.md`）+ Rust 解析器
- 适配 Tenant quota 模型（`max_repos` + storage 含义重定义）
- ExecutionProvider 简化准备（标 deprecated 死路径；不删除；EVO-111 收口）

为 Phase E'-2（Smart HTTP + Agent 集成）提供 schema / 解析 / quota 底层。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|-----------|--------|---------------|
| [EVO-101](../backlog/active/EVO-101-git-repos-schema.md) | `git_repos` 表 + 双轨 migration（含 ST-1 Tenant quota + ST-2 ExecutionProvider 简化准备） | EVO-100 | P0 | 无（Phase E'-1 首 Story） |
| [EVO-102](../backlog/active/EVO-102-evolith-policy-yaml.md) | `.evolith/policy.yaml` 规范 + 解析器 | EVO-100 | P1 | 无（与 EVO-101 独立，可同迭代推进） |

**EVO-103 (Smart HTTP) 推迟到下一轮**——smart HTTP 涉及 protocol v2 / pkt-line / pack streaming，实现面较大，单独成轮更适合验收。Phase E'-1 只完成 schema + parser 层。

## 3. 发布计划基线：不做事项

- 不实现 `skill_index` / `cli_index` / `mcp_tool_index` 表（EVO-108）
- 不实现 Repo CRUD HTTP API（EVO-103）
- 不实现 Smart HTTP git 协议（EVO-103）
- 不实现 Agent Session / Scoped Token（EVO-106）
- 不实现 Vibe Coding Web UI（EVO-104，需 UX 调研 design doc `docs/design/vibe-coding-ui-decisions.md` U-01~U-05 先决策）
- 不删除 service-skill 沙箱执行层（EVO-111 收口）
- 不修改 ExecutionProvider 公开 API（仅标 deprecated 死路径，不破坏 Phase 7 已通过的 309 tests）
- 不实现 TenantStorage 文件系统监控（Phase 5+ LFS HTTP API 评估）

## 4. 发布计划基线：计划验收标准

### Story 格式与 BDD 适用性

- [x] EVO-101 标 Technical 形态（数据模型 + Repository trait + 集成测试）；使用命令级技术验收
- [x] EVO-102 标 Governance 形态（规范文档 + parser）；使用一致性 + 命令级混合验收
- [ ] 行为类工作无（schema 改动是 internal；user-facing 行为变化在 Phase E'-2 体现）

### EVO-101 验收（来自 item file）

- [ ] migration 007（SQLite + PostgreSQL）落地
- [ ] `git_repos` 表 11 个字段 + UNIQUE + idx
- [ ] `GitRepoRepository` trait + Sqlite/Pg 双实现 + 集成测试覆盖 CRUD + tenant scope + UNIQUE 冲突
- [ ] Tenant quota 子任务：新增 `max_repos`；`max_storage_mb` 含义重定义；`max_tools/skills/snippets` 处置
- [ ] ExecutionProvider 子任务：Skill / Cli caller 与 Code / Command payload 加 `#[deprecated]`；`CompositeProvider::route()` 编译通过
- [ ] `cargo test --workspace` 全绿
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` 0 errors
- [ ] `bun run build` 0 errors（frontend 无变更应仍通过）

### EVO-102 验收（来自 item file）

- [ ] `.evolith/policy.yaml` 规范文档落地于 `docs/reference/formats/POLICY-YAML-FORMAT.md`
- [ ] `EvolithPolicy::parse(&str) -> Result<EvolithPolicy, AppError>` 实现
- [ ] `default_action` / `protected_paths` / `agents[].scopes` 解析单元测试覆盖 ≥10 用例
- [ ] 缺失 `policy.yaml` 时 fallback 到 `git_repos.auto_merge` / `require_review` 默认值
- [ ] SPEC 文档含向后兼容策略（`version` 字段缺失按 v1 处理）

### 全局门禁

- [ ] 所有命令级验收可重复执行
- [ ] 跨 SQLite + PostgreSQL 双轨 schema 一致
- [ ] 文档同步：PRODUCT-BACKLOG.md（EVO-101/102 状态 Ready → In Progress → Done）/ EVOLUTION.md（如果遇到陷阱）

## 5. 发布计划基线：计划验证

```bash
# backend — schema 双轨 migration
cargo sqlx migrate run --source backend/migrations/sqlite
cargo sqlx migrate run --source backend/migrations/postgres

# backend — 编译 + 测试
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings

# 手工 psql / sqlite3 查询验证
sqlite3 :memory: < backend/migrations/sqlite/007_git_repos.sql
sqlite3 :memory: "SELECT name FROM sqlite_master WHERE type='table' AND name='git_repos'"
sqlite3 :memory: ".schema git_repos"

# frontend — sanity check（无变更应通过）
cd frontend && bun run build
```

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| SQLite 与 PostgreSQL 行为差异（JSON / UUID / TEXT vs JSONB） | 双轨 migration 各跑一次 + Repository 集成测试分别验证；不通过则阻断进入下一步 |
| Tenant quota 字段重定义影响现有测试 | 集成测试先更新到新结构；现有 billing 测试覆盖 `current_storage_mb` 字段的需同步调整 |
| ExecutionProvider 标 deprecated 触发新 warning 阻塞 `-D warnings` | 用 `#[allow(deprecated)]` 限定作用域（仅 `CompositeProvider::route()`）；不扩大到全 crate |
| policy.yaml parser 与现有 `service-snippet/src/parser.rs` 重复 | EVO-102 parser 是独立 crate（建议新建 `service-policy`）；不与 `service-snippet` 共享 frontmatter 抽象以保持改动局部化 |
| EVO-102 创建新 crate 触发 workspace 结构变更 | 文档同步（`docs/reference/PROJECT-MAP.md` + `backend/crates/*/Cargo.toml` 引用） |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | _实施时填_ |
| 产物 | migration 007 SQL × 2；`domain::repository.rs` `GitRepoRepository` trait；`infra/src/db/{sqlite,pg}_git_repo_repo.rs`；`domain::tenant.rs` quota 字段；`common/src/execution.rs` `#[deprecated]` 标注；`docs/reference/formats/POLICY-YAML-FORMAT.md`；policy parser crate |
| 状态同步归口 | PRODUCT-BACKLOG.md（EVO-101/102 状态从 Proposed → Ready → In Progress → Done）；EVO-100 Epic 子项表状态更新；README.md inventory |
| Story/BDD 归口 | EVO-101 / EVO-102 各自 item file 收口 |
| 验证证据 | `cargo test --workspace` 完整输出；`cargo clippy --workspace --all-targets -- -D warnings` 0 errors；migration 跑通截图 |
| 残余工作归口 | EVO-103 Smart HTTP（下一轮）+ EVO-108 Indexer（Phase E'-3）+ EVO-111 Sandbox 废弃（Phase E'-4） |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-06-24 | activation | _待执行时填入_ |

### 迭代启动前库存盘点（per [START-ITERATION.md](../sop/START-ITERATION.md)）

按 AGENTS.md / START-ITERATION SOP 要求，启动前先盘点既有 iteration 状态：

- **In Progress / Active**：无（BOARD.md `Now` 段为空；iterations/ 中无 Active 文档）
- **Review**：无
- **Blocked**：Iterations 018-020、025-027 已被 2026-06-23 方向调整 superseded（见 [BOARD.md](../board.md) Blocked Or Paused 段；本轮同步清理 README.md 库存段）
- **Recently Closed**：Iteration 041（EVO-045-A ExecutionProvider；2026-06-05 follow-up 已修复 sandbox 默认启用回归）

**disposition 决策**：
- Iterations 018-020（Skill 导入基础 / 多来源 / 版本验证）—— Superseded by EVO-100 Phase E'-3（EVO-108 Indexer）。不激活。
- Iteration 025（租户设置与审计详情补齐）—— 与 Phase E' 独立；维持 Blocked 状态。
- Iteration 026（Stripe Webhook）—— 与 Phase E' 独立；维持 Blocked 状态。
- Iteration 027（Skill 发现质量）—— Superseded by EVO-109 Discovery API + Pages 式发现 UI；不激活。
- Iteration 028（Rustfmt 基线）—— 已 Closed（per EVO-033 + EVO-059）；README.md 重复条目清理。
- Iteration 034（Skill/CLI 数据模型基线）—— 已 Closed（per EVO-049-A）；README.md 重复条目清理。

**重复条目 / 文档漂移修复**（本轮同步在 README.md）：
- 移除 未来计划 section 中"Ready for activation"标注 Iteration 028 / 034（实际已 Closed）
- 修正 非终态库存 section 中 Iteration 028 / 034 的重复行
- 将 Iteration 018-020 / 027 在 未来计划 section 标 Superseded

启动 ITERATION-042（Phase E'-1 首轮）。

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|
|  | _暂无_ | | | |

## 10. Review

- 完成：_收口时填_
- 未完成：_收口时填_
- 验证结果：_收口时填_
- 闭环状态：`Complete / Partial / Blocked`
- 残余归口：_收口时填_

## 11. Retrospective

- 做得好的：_收口时填_
- 需要调整的：_收口时填_
- 写入 EVOLUTION：_收口时填_

---

## 相关链接

- 父 Epic：[EVO-100 Git-Centric Platform Foundation](../backlog/active/EVO-100-git-centric-platform-foundation.md)
- 提案：[GIT-CENTRIC-PLATFORM](../proposals/GIT-CENTRIC-PLATFORM.md)
- ADR：[ADR-0004 Git-Centric Storage](../decisions/ADR-0004-git-centric-storage.md)
- 启动 SOP：[START-ITERATION](../sop/START-ITERATION.md)
- 闭环 SOP：[TASK-CLOSURE](../sop/TASK-CLOSURE.md)
- 下一轮候选：[EVO-103 Repo CRUD + Smart HTTP + Repo Context API](../backlog/active/EVO-103-repo-context-and-smart-http.md)
