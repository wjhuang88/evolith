# EVO-113 Direction Pivot Review Remediation

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- [Git-Centric Platform Proposal](../../proposals/GIT-CENTRIC-PLATFORM.md)
- [ADR-0004 Git-Centric Storage](../../decisions/ADR-0004-git-centric-storage.md)
- [ADR-0005 Deprecate Sandbox Runtime](../../decisions/ADR-0005-deprecate-sandbox-runtime.md)
- [Iteration 042](../../iterations/ITERATION-042.md)
- [Iteration 043](../../iterations/ITERATION-043.md)

## Summary

- 类型：bug / governance
- 优先级：P0
- 状态：Done
- 父 Epic：EVO-100

## Problem Or Outcome

2026-06-25 方向变更全面评审发现 Git-Centric 主线存在若干会影响后续 Agent 执行和安全语义的漂移：

1. `git_repos` 与 repository 创建路径默认 `auto_merge=true` / `require_review=false`，与新方向“默认 require review”冲突。
2. EVO-101 / EVO-102 已由 Iteration 042 完成，但 backlog 主表和 EVO-100 子项表仍标 `Proposed`。
3. Iterations README 将 EVO-104 UX 调研门禁错误套到 EVO-103 backend story。
4. API / Architecture / Config reference 仍把 sandbox execute 描述为当前稳定能力。
5. 方向变更相关文档存在断链。
6. `.evolith/policy.yaml` 规范与 parser 默认值语义不一致。
7. Manifest risk gates 未吸收 Git-Centric 主线的新硬约束。

## Goal And Non-Goals

**目标**：

- 将 repo 默认策略改为安全默认：`auto_merge=false`、`require_review=true`。
- 追加 SQLite/PostgreSQL 双轨 migration 修正 DB default。
- 同步 backlog / Epic / Board / Roadmap / Iteration 目录状态。
- 修复方向变更相关断链。
- 将 sandbox execute 标为 legacy / pending removal，避免新工作误读为当前主线能力。
- 同步 manifest risk gates 和 policy spec/parser 默认语义。

**不做**：

- 不实现 EVO-103 Repo CRUD / Smart HTTP。
- 不删除 sandbox 代码（EVO-111 范围）。
- 不完成 EVO-104 UX 决策。
- 不重写已关闭 Iteration 042 的计划基线，只追加或同步状态事实。

## Acceptance Criteria

- [x] 新建 SQLite/PostgreSQL 009 migration，将 `git_repos` 默认改为 `auto_merge=false` / `require_review=true`。
- [x] Repository create 默认值与 migration 一致，相关测试更新。
- [x] EVO-101 / EVO-102 在 PRODUCT-BACKLOG.md 和 EVO-100 子项表同步为 Done。
- [x] Board / Roadmap / Iterations README 不再把 EVO-104 UX 门禁错误阻塞 EVO-103。
- [x] Markdown 相对链接检查 0 missing links。
- [x] API / Architecture / Config reference 中 sandbox execute 明确为 legacy / pending removal，不作为 Git-Centric 新主线能力。
- [x] Policy spec 与 parser 对缺省 `default_action`、`agents[].scopes` 的语义一致。
- [x] Manifest risk gates 包含 Git-Centric 主线新增硬约束。

## Validation Evidence Required

- `cargo test -p infra --test git_repo_repo_tests --quiet`
- `cargo test -p domain policy --quiet`
- `cargo check --workspace --quiet`
- Markdown link scan
- `git diff --check`

## Residual Work Destination

- EVO-103：Repo CRUD + Smart HTTP + Repo Context API。
- EVO-104：Vibe Coding UI UX 决策与实现。
- EVO-111：sandbox 整层删除。

## Closure Ledger

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 修复方向变更全面评审发现的 P0/P1 漂移和默认策略风险 |
| 产物 | migration、repository 默认值、测试、backlog / roadmap / board / reference / manifest 文档 |
| 状态同步归口 | PRODUCT-BACKLOG.md、EVO-100、EVO-101、EVO-102、EVO-113、iterations README、BOARD、roadmap、reference docs、manifest |
| 验证证据 | Rust 局部测试、workspace check、Markdown link scan、diff check |
| 残余工作归口 | EVO-103 / EVO-104 / EVO-111 |

## Review

- 请求结果：Complete。
- 已实施：
  - 新增 SQLite/PostgreSQL migration 009，将 `git_repos` 默认策略修正为 `auto_merge=false` / `require_review=true`。
  - 同步 SQLite/Postgres repository create 默认值和 git repo tests。
  - 修复 sandbox legacy 测试对“当前环境没有 Docker”的错误假设。
  - 同步 PRODUCT-BACKLOG、EVO-100、BOARD、Roadmap、Iterations README 的 EVO-101/102/EVO-103/EVO-113 状态与顺序。
  - 修复方向变更文档断链；更新 API/Architecture/Config/Project Map/AGENTS 中 sandbox legacy 标注。
  - 刷新 `.agent-governance/manifest.yaml` 的 Git-Centric 风险门禁。
  - 同步 `.evolith/policy.yaml` 规范与 parser 默认语义。
- 验证证据：
  - `cargo fmt --all -- --check` → pass
  - `cargo test -p infra --test git_repo_repo_tests --quiet` → 15 passed
  - `cargo test -p domain policy --quiet` → 11 passed
  - `cargo test -p infra --quiet` → pass
  - `cargo test -p service-skill --lib --quiet` → 17 passed
  - `cargo check --workspace --quiet` → pass
  - `cargo clippy --workspace --all-targets -- -D warnings` → pass
  - `cargo test --workspace --quiet` → pass
  - Markdown link scan → all markdown links exist
  - `git diff --check` → pass
- 残余工作归口：
  - EVO-103：Repo CRUD + Smart HTTP + Repo Context API。
  - EVO-104：Vibe Coding UI UX 决策与实现。
  - EVO-111：sandbox 整层删除。
- 迭代记录：[Iteration 043](../../iterations/ITERATION-043.md)。

## Source Snapshot

- Source: 2026-06-25 方向变更全面评审。
- Decision context: Git-Centric Platform 已成为主线；旧 sandbox runtime 进入废弃路径。
