# EVO-116 EVO-103 验收硬化：API key scope、Context API 边界、元数据同步与合约补齐

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- 父 Epic: [EVO-100](EVO-100-git-centric-platform-foundation.md)
- 验收输入: [EVO-103 acceptance](../../review/EVO-103-acceptance.md)
- 架构评审输入: [EVO-103 architecture review](../../review/EVO-103-architecture-review.md)
- 相关 Story: [EVO-103](EVO-103-repo-context-and-smart-http.md)
- 相关子项: [EVO-103-B-2](EVO-103-B-2-smart-http-endpoints.md), [EVO-103-C](EVO-103-C-repo-context-api.md), [EVO-106](EVO-106-agent-session-and-scoped-token.md), [EVO-108](EVO-108-skill-cli-mcp-indexer.md), [EVO-112](EVO-112-repo-management-ui.md)
- ADR: [ADR-0004 Git-Centric Storage](../../decisions/ADR-0004-git-centric-storage.md), [ADR-0006 Smart HTTP via git subprocess](../../decisions/ADR-0006-smart-http-via-git-subprocess.md)
- 合约归口: [API Contract](../../reference/API-CONTRACT.md)
- 验证 SOP: [Testing](../../sop/TESTING.md)

## Summary

- 类型：technical / security / performance hardening
- 优先级：P0
- 状态：Done
- 父 Epic：EVO-100
- 所属迭代：Iteration 049
- 来源：2026-06-26 架构组对 EVO-103 交付验收的 Conditional Accept

## Problem Or Outcome

EVO-103 已完成 Repo CRUD、Smart HTTP 和 Repo Context API 的核心功能，但架构组验收发现
若干发布前必须硬化的问题：

1. API key 通过全局 RBAC 被注入为 tenant Member，但普通 REST handler 未统一检查
   `ApiKey.permissions`，scope 模型只在 Smart HTTP handler 局部补齐。
2. Repo Context API 的 blob/tree/diff 没有输出大小、entry 数量、计算时间和错误映射边界。
3. EVO-103-C item file 要求的性能基准未形成验收证据。
4. `git-receive-pack` push 成功后没有同步 `git_repos.last_commit_sha` /
   `last_committed_at`，Repo 列表和后续 indexer 触发会读到陈旧元数据。
5. `API-CONTRACT.md` 缺失 EVO-103 的 Repo CRUD、Context API 和 Smart HTTP 路由说明。

本 Story 的目标是在继续 Phase E' 后续 UI / agent / indexer 工作前，把这些安全、性能和
合约缺口收口，避免后续功能建立在不稳定接口之上。

## Goal And Non-Goals

- **Goal**：
  1. 建立 API key scope 的统一门禁，至少覆盖 Repo CRUD、Repo Context、Smart HTTP 和
     API key 管理路由，避免 read-only 或 execute-only key 越权调用 REST 写接口。
  2. 为 Repo Context API 增加可配置或常量化的资源边界：blob 最大响应字节数、
     file-tree/diff 最大 entry 数、blocking 读取超时和清晰的 413/422/504 错误映射。
  3. `git-receive-pack` 成功后同步 repo last commit metadata，并提供失败时不破坏 git
     push 结果的降级策略和日志。
  4. 补齐 EVO-103 公开 API 合约文档，说明 auth、CSRF 豁免、错误码和响应结构。
  5. 补齐性能验收证据：file-tree（100 files）P95 < 100ms；repo list（1000 repos）
     P95 < 200ms；若本机环境不稳定，记录 benchmark 方法、样本和限制。
- **Non-goals**：
  - 不实现 EVO-106 scoped token 新数据模型或 agent session。
  - 不实现资源级 ACL、公开 repo discover、SSH、LFS。
  - 不迁移 Smart HTTP 到纯 gix。
  - 不实现 EVO-105 Commit API 或 EVO-108 Indexer。
  - 不做 UI 改动。

## Dependencies And Blockers

- 硬依赖 EVO-103 A/B/C Done。
- 软关联 EVO-106：本轮只修现有 API key scope，不引入完整 agent scoped token。
- 软关联 EVO-108：本轮只同步 repo metadata，不实现 indexer 触发。
- 无外部阻塞；需要实现者在动手前确认是否采用常量边界还是配置项。若新增配置键，必须同步
  [CONFIG](../../reference/CONFIG.md)。

## Acceptance Criteria

### API key scope

- [x] read-only API key 可以 clone/fetch 和读取允许的 Repo Context 端点。
- [x] read-only API key 调用 `git-receive-pack`、Repo CRUD 写接口或 API key 管理接口返回 403。
- [x] execute-only / unrelated permission key 不能访问 Repo Context 和 Repo CRUD。
- [x] JWT 用户原有 Repo CRUD / Context 行为不回归。
- [x] 权限判断不散落为重复字符串判断；至少有共享 helper 或清晰的单一归口。

### Context API resource bounds

- [x] 超过 blob 最大字节数时不把完整内容读入并返回给客户端；响应 413 或明确错误码。
- [x] file-tree / diff 超过 entry 上限时返回截断标记或 413/422，行为写入 API contract。
- [x] gix blocking 读取有超时保护；超时返回 504 或明确错误码。
- [x] 无效 ref / sha 返回 400/404 类错误，不再统一映射为 500。

### Smart HTTP metadata

- [x] `git push` 成功后 `git_repos.last_commit_sha` 与 `last_committed_at` 更新为默认分支最新 commit。
- [x] push 到非默认分支时行为明确：不更新默认分支 metadata，或记录分支 metadata 延后到独立 Story。
- [x] metadata 更新失败不破坏已成功的 git push，但必须记录 warn/error 并有测试覆盖。

### Contract and evidence

- [x] `docs/reference/API-CONTRACT.md` 补齐 Repo CRUD、Smart HTTP、Repo Context API。
- [x] EVO-103-C 性能基准有命令、样本规模、结果和限制说明。
- [x] `docs/review/EVO-103-acceptance.md` 或 Iteration 049 Review 记录 Conditional Accept 的关闭证据。

## BDD / Technical Acceptance Scenarios

### 场景 1：read-only API key 不能写仓库

Given tenant T 有 repo R，API key K 只有 `repo:read`  
When 客户端用 K 调用 `POST /repos/{R.id}/git-receive-pack` 或 PATCH/DELETE repo  
Then 请求返回 403  
And repo 内容、repo metadata 和 API key 状态不变

### 场景 2：execute-only API key 不能读取 repo context

Given API key K 只有 `execute` 权限  
When 客户端用 K 调用 file-tree、blob、commits 或 diff  
Then 请求返回 403  
And 不暴露 repo 路径、ref 解析错误或 blob 内容

### 场景 3：Context API 大对象被边界保护

Given repo R 含一个超过 blob 响应上限的 blob  
When 用户请求 `GET /repos/{R.id}/blobs/{sha}`  
Then 服务返回边界错误（413 或 API contract 中定义的错误码）  
And 响应不包含完整 blob 内容

### 场景 4：push 成功后 repo metadata 更新

Given repo R 的 default_branch 为 `main`  
When 用户通过 Smart HTTP push 一个新的 main commit  
Then `GET /api/v1/tenant/{tenant_id}/repos/{R.id}` 返回新的 `last_commit_sha`  
And `last_committed_at` 对应新 commit 时间

### 场景 5：性能基准可复现

Given 一个含 100 个文件的 repo 和一个含 1000 条 repo metadata 的 tenant  
When 运行本 Story 定义的 benchmark 命令  
Then file-tree P95 < 100ms  
And repo list P95 < 200ms  
And 结果记录到 Iteration 049 Review

## Impacted Areas

- backend/crates/api/src/middleware/rbac.rs
- backend/crates/api/src/middleware/auth.rs
- backend/crates/api/src/handlers/git_smart_http_handlers.rs
- backend/crates/api/src/handlers/repo_handlers.rs
- backend/crates/api/src/handlers/repo_context_handlers.rs
- backend/crates/service-git/src/lib.rs
- backend/crates/infra/src/config.rs（仅新增配置时）
- backend/crates/api/tests/*
- docs/reference/API-CONTRACT.md
- docs/reference/CONFIG.md（仅新增配置时）
- docs/review/EVO-103-acceptance.md 或 docs/iterations/ITERATION-049.md Review

## Validation Evidence Required

```bash
cd backend
cargo test -p api --test git_smart_http_e2e_tests
cargo test -p api --test repo_context_e2e_tests
cargo test -p api --test repo_e2e_tests
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

Additional required evidence:

- API key scope matrix test covering JWT, `repo:read`, `repo:write`, `execute`, invalid key.
- Context API bound tests for large blob, large tree or diff, invalid ref/sha, timeout path if practical.
- Metadata sync test using real git push or handler-level controlled bare repo push.
- Performance benchmark command and result table.
- `git diff --check`.
- Markdown link check from [START-ITERATION](../../sop/START-ITERATION.md) before closure.

## Residual Work Destination

- EVO-106 remains the owner for full agent scoped token/session model.
- EVO-108 remains the owner for indexer trigger and push event processing.
- Phase 5+ remains the owner for branch-level metadata, public repo discover, SSH and LFS.
- If resource bounds need more nuanced pagination/streaming UX, create a follow-up story before marking this one Done.

## Source Snapshot

- Source: 2026-06-26 architecture acceptance of EVO-103.
- Acceptance conclusion: Conditional Accept.
- Confirmed defects/gaps:
  - API key permissions not uniformly enforced outside Smart HTTP.
  - Context API resource boundaries and performance evidence missing.
  - Smart HTTP push does not update repo metadata.
  - API contract lacks EVO-103 public route documentation.

## 变更记录

| 日期 | 状态 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|
| 2026-06-26 | Done | 关闭 EVO-103 Conditional Accept 发布前阻断项 | 5 个验收面（API key scope、Context API 边界、Smart HTTP push metadata、API contract、性能证据）全部完成；架构组可基于此批准 EVO-103 关闭 | 不适用 |

## 实施摘要

### 关键代码变更

- 新增 `backend/crates/api/src/middleware/api_key_scope.rs` — 单一权限矩阵 + `api_key_allows_repo_read/write` 与 `api_key_allows_api_key_management` 共享 helper。
- 在 `repo_handlers` / `repo_context_handlers` / `api_key_handlers` 中通过 `forbid_if_api_key_lacks` helper 接入统一门禁。
- `git_smart_http_handlers::authorize_git_service` 改用共享 helper。
- `service-git` 新增 `BLOB_MAX_BYTES = 1 MiB` / `FILE_TREE_MAX_ENTRIES = 5000` / `DIFF_MAX_ENTRIES = 5000` / `CONTEXT_BLOCKING_TIMEOUT = 5s` 常量；`read_blob` / `read_file_tree` / `read_diff` 超限时返回 `ResourceExceeded`；无效 ref/sha 返回 `NotFound` / `InvalidInput`。
- `repo_context_handlers` 在 `web::block` 之上叠加 `tokio::time::timeout` 保护；新错误类型映射到 400/404/413/504/500。
- `git_smart_http_handlers::update_repo_metadata_after_push` 在 receive-pack 成功后通过 gix 解析 `refs/heads/{default_branch}`，调用 `git_repo_repo.update_last_commit`；任何失败仅记 WARN，不影响 push 响应。
- `service-git::resolve_ref` 接受 `refs/heads/X` 与 `X` 两种写法；返回 `(oid, timestamp)` 给 metadata sync 使用。

### 测试

- `api_key_scope_e2e_tests.rs`（5 个 E2E）覆盖 read-only / write / execute-only / admin 管理 API key 越权 / JWT 不回归。
- `repo_context_bounds_e2e_tests.rs`（4 个 E2E）覆盖无效 ref 404 / 无效 sha 400 / 404 / 超大 blob 413 / push metadata 同步。
- `repo_perf_benchmarks.rs`（2 个 benchmark）记录 file-tree 100 文件 P95 与 repo list 1000 条 P95。
- `repo_e2e_tests.rs` 7 项 / `repo_context_e2e_tests.rs` 6 项 / `git_smart_http_e2e_tests.rs` 3 项 / `api` 单元测试 22 项 / `service-git` 单元测试 23 项全部通过；`cargo test --workspace` 与 `cargo clippy --workspace --all-targets -- -D warnings` 全绿。

### 文档

- `docs/reference/API-CONTRACT.md` 新增 `Repos` / `Smart HTTP` / `Repo Context API` / `API key scope` / `Push metadata sync` 五个 section；CSRF 豁免加上 `git-upload-pack` / `git-receive-pack`；错误码新增 `INVALID_INPUT` / `RESOURCE_EXCEEDED` / `TIMEOUT` / `REPO_EXISTS`。
- `docs/reference/CONFIG.md` 暂未新增键（资源边界使用 `service-git` 常量；如未来需要按租户覆盖，回退为可配置项）。
- `docs/iterations/ITERATION-049.md` Review / Retrospective 已记录性能证据。
