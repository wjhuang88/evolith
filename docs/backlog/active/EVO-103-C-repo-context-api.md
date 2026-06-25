# EVO-103-C Repo Context API

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- 父 Epic: [EVO-103](EVO-103-repo-context-and-smart-http.md)
- 祖父 Epic: [EVO-100](EVO-100-git-centric-platform-foundation.md)
- 硬依赖: [EVO-103-A](EVO-103-A-repo-crud.md)（Ready；A 完成前 C 维持 Proposed）
- [ADR-0004 Git-Centric Storage](../../decisions/ADR-0004-git-centric-storage.md)

## Summary

- 类型：feature / api
- 优先级：P0
- 状态：Proposed
- 父 Epic：EVO-103（祖父 EVO-100）

## Problem Or Outcome

作为平台用户和前端 UI，我需要能通过 HTTP API 查询 git 仓库的文件树、blob 内容、commit 历史和 diff，以便在 Web UI 中浏览代码、查看变更历史和对比版本差异。

## Goal And Non-Goals

- **Goal**：
  1. 实现 Repo Context GET API：
     - `GET /repos/{id}/file-tree` — 通过 `gix::Tree::traverse` 遍历文件树。
     - `GET /repos/{id}/blobs/{sha}` — 通过 `gix::Object::detach` 获取 blob 内容。
     - `GET /repos/{id}/commits` — 通过 `gix::revwalk` 获取 commit 列表。
     - `GET /repos/{id}/diff` — 通过 `gix::diff::tree` 计算两 ref 间 diff。
  2. 所有 GET endpoints 接受 `?ref=` 参数（默认 `main`）。
  3. RBAC：tenant 作用域；公开 repo 跨 tenant list 可见，但 blobs 需权限校验。
- **Non-goals**：
  - 不实现 repo CRUD（→ EVO-103-A）。
  - 不实现 git smart HTTP 协议（→ EVO-103-B）。
  - 不实现 commit / promote（→ EVO-105）。
  - 不实现跨 tenant 公开 repo discover（Phase 5+）。

## Dependencies And Blockers

- 硬依赖 EVO-103-A（Proposed）：repo 必须先通过 CRUD 创建并初始化裸仓。
- EVO-103-B 不是硬 API 依赖，但真实内容需要 push 后才能查询；测试可通过 gix 直接 seed 内容。
- **阻塞**：EVO-103-A 未完成前无法进入 Ready。

## Governing ADRs, Specs Or Decisions

- [ADR-0004 Git-Centric Storage](../../decisions/ADR-0004-git-centric-storage.md)

## Acceptance Criteria

- [ ] `GET /repos/{id}/file-tree` 返回指定 ref 的文件树结构
- [ ] `GET /repos/{id}/blobs/{sha}` 返回指定 SHA 的 blob 内容
- [ ] `GET /repos/{id}/commits` 返回 commit 列表（按时间倒序）
- [ ] `GET /repos/{id}/diff` 返回两 ref 间的 diff
- [ ] 所有 GET endpoints 接受 `?ref=` 参数，默认 `main`
- [ ] 跨 tenant 访问私有 repo → 403
- [ ] 公开 repo 跨 tenant list 可见，但 blobs 需权限校验
- [ ] `cargo test --workspace` 与 `cargo clippy --workspace --all-targets -- -D warnings` 全绿

### BDD 验收场景

**场景 1：file-tree 返回指定 ref 的树**
```gherkin
Given repo R 已存在且 main 分支包含 3 个文件
When 用户 GET /repos/{R.id}/file-tree?ref=main
Then 返回包含 3 个文件条目的树结构
And 每个条目包含 path、type（blob/tree）、size 字段
```

**场景 2：blobs 按 SHA 获取内容**
```gherkin
Given repo R 的 main 分支包含文件 README.md，其 blob SHA 为 abc123
When 用户 GET /repos/{R.id}/blobs/abc123
Then 返回 README.md 的文本内容
```

**场景 3：commits 列表**
```gherkin
Given repo R 的 main 分支有 5 个 commit
When 用户 GET /repos/{R.id}/commits?ref=main
Then 返回 5 个 commit 条目，按时间倒序
And 每个条目包含 sha、author、message、timestamp
```

**场景 4：diff 两 ref 间对比**
```gherkin
Given repo R 的 main 分支和 feature 分支有不同内容
When 用户 GET /repos/{R.id}/diff?ref=main&compare=feature
Then 返回两分支间的 diff 内容
```

**场景 5：?ref= 默认 main**
```gherkin
Given repo R 已存在
When 用户 GET /repos/{R.id}/file-tree（不带 ?ref= 参数）
Then 返回 main 分支的文件树
```

**场景 6：跨 tenant 私有 repo → 403**
```gherkin
Given 用户 A 属于 tenant T1，私有 repo R 属于 tenant T2
When 用户 A GET /repos/{R.id}/file-tree
Then 返回 403 Forbidden
```

## Validation Evidence Required

- API 测试覆盖 file-tree / blobs / commits / diff 全部端点。
- BDD 场景 1-6 全部有对应测试用例。
- 性能基准：file-tree（100 文件）P95 < 100ms；list（1000 repos）P95 < 200ms。
- `cargo test --workspace` 全绿。
- `cargo clippy --workspace --all-targets -- -D warnings` 0 errors。

## Residual Work Destination

- 跨 tenant 公开 repo discover（Phase 5+）。
- Commit / promote → EVO-105。

## Source Snapshot

- Source: EVO-103 拆分（2026-06-25），按端到端价值切片拆为 A/B/C。
- Decision context: Repo Context API 提供 Web UI 浏览仓库所需的只读查询能力，依赖 EVO-103-A 的裸仓初始化。
- 关键依赖：`gix::Tree::traverse`、`gix::Object::detach`、`gix::revwalk`、`gix::diff::tree`。
