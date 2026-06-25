# EVO-103-A Repo CRUD（gix::init + RBAC）

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- 父 Epic: [EVO-103](EVO-103-repo-context-and-smart-http.md)
- 祖父 Epic: [EVO-100](EVO-100-git-centric-platform-foundation.md)
- 依赖: [EVO-101](EVO-101-git-repos-schema.md)（Done）
- [ADR-0004 Git-Centric Storage](../../decisions/ADR-0004-git-centric-storage.md)

## Summary

- 类型：feature / api
- 优先级：P0
- 状态：Done
- 父 Epic：EVO-103（祖父 EVO-100）

## Problem Or Outcome

作为平台用户，我需要通过 HTTP API 创建、查询、更新和删除 git 仓库，以便在 Evolith 上托管代码并启动标准 git 工作流。

## Goal And Non-Goals

- **Goal**：
  1. 实现 Repo CRUD REST API：`POST /api/v1/tenant/{tenant_id}/repos`、`GET /api/v1/tenant/{tenant_id}/repos`（list）、`GET /api/v1/tenant/{tenant_id}/repos/{id}`、`PATCH /api/v1/tenant/{tenant_id}/repos/{id}`、`DELETE /api/v1/tenant/{tenant_id}/repos/{id}`。
  2. 创建时自动 `gix::init` 裸仓到 `/srv/evolith/repos/{tenant_id}/{repo_id}.git`。
  3. 可选 `--seed-template`（或请求字段）写入默认 `.evolith/agents.yaml` + `.evolith/policy.yaml` + README。
  4. RBAC：tenant 作用域；跨 tenant 访问拒绝（公开 repo 跨 tenant list 可见性归 EVO-103-C）。
  5. 校验：name 在 tenant 内唯一（UNIQUE(tenant_id, name)）。
- **Non-goals**：
  - 不实现 git smart HTTP 协议（→ EVO-103-B）。
  - 不实现 file-tree / blobs / commits / diff API（→ EVO-103-C）。
  - 不实现 commit / promote（→ EVO-105）。
  - 不实现 SSH / LFS（Phase 5）。

## Dependencies And Blockers

- 硬依赖 EVO-101（Done）：`git_repos` 表 + 双轨 migration。
- 间接依赖 `gix` crate（EVO-086 Done）。
- 无阻塞项。

## Governing ADRs, Specs Or Decisions

- [ADR-0004 Git-Centric Storage](../../decisions/ADR-0004-git-centric-storage.md)

## Acceptance Criteria

- [x] `POST /api/v1/tenant/{tenant_id}/repos` 创建 repo → 201 + DB 记录 + 磁盘裸仓 `{base_path}/{tenant_id}/{repo_id}.git`
- [x] `POST /api/v1/tenant/{tenant_id}/repos` 可选 `seed_template: true` 写入 README
- [x] `GET /api/v1/tenant/{tenant_id}/repos` 返回 caller tenant 内的 repo 列表
- [x] `GET /api/v1/tenant/{tenant_id}/repos/{id}` 返回单个 repo 详情
- [x] `PATCH /api/v1/tenant/{tenant_id}/repos/{id}` 更新 repo 元数据（name、description、visibility）
- [x] `DELETE /api/v1/tenant/{tenant_id}/repos/{id}` 硬删除 repo（DB 记录 + 磁盘裸仓一并移除）
- [x] 跨 tenant 访问 → 403
- [x] 同 tenant 内重复 name → 409
- [x] RBAC：tenant-scoped，通过现有 `AuthenticatedUser` + RBAC middleware 校验
- [x] `cargo test --workspace` 与 `cargo clippy --workspace --all-targets -- -D warnings` 全绿

### BDD 验收场景

**场景 1：创建 repo → 201 + 裸仓**
```gherkin
Given 用户已登录且属于 tenant T
When 用户 POST /api/v1/tenant/{T.id}/repos 携带 { name: "my-repo", seed_template: true }
Then 返回 201 且响应包含 repo id 和 name
And 磁盘上存在 {base_path}/{tenant_id}/{repo_id}.git 裸仓
And 裸仓内包含 README
```

**场景 2：list 仅返回 caller tenant**
```gherkin
Given 用户 A 属于 tenant T1，用户 B 属于 tenant T2
And T1 和 T2 各有 2 个 repo
When 用户 A GET /api/v1/tenant/{T1.id}/repos
Then 仅返回 T1 的 2 个 repo，不包含 T2 的任何 repo
```

**场景 3：跨 tenant 访问 → 403**
```gherkin
Given 用户 A 属于 tenant T1，repo R 属于 tenant T2
When 用户 A GET /api/v1/tenant/{T1.id}/repos/{R.id}
Then 返回 403 Forbidden
```

**场景 4：重复 name → 409**
```gherkin
Given 用户已在 tenant T 内创建 name 为 "my-repo" 的 repo
When 用户再次 POST /api/v1/tenant/{T.id}/repos 携带 { name: "my-repo" }
Then 返回 409 Conflict
```

**场景 5：更新 repo**
```gherkin
Given 用户已在 tenant T 内创建 repo R
When 用户 PATCH /api/v1/tenant/{T.id}/repos/{R.id} 携带 { description: "updated" }
Then 返回 200 且 repo description 已更新
```

**场景 6：删除 repo（硬删除）**
```gherkin
Given 用户已在 tenant T 内创建 repo R
When 用户 DELETE /api/v1/tenant/{T.id}/repos/{R.id}
Then 返回 204 No Content
And DB 中 repo 记录已删除
And 磁盘裸仓 /srv/evolith/repos/{tenant_id}/{repo_id}.git 已移除
```

## Validation Evidence Required

- API 集成测试覆盖 repo 生命周期（create → list → get → update → delete）。
- BDD 场景 1-6 全部有对应测试用例。
- `cargo test --workspace` 全绿。
- `cargo clippy --workspace --all-targets -- -D warnings` 0 errors。

## Residual Work Destination

- Smart HTTP git 协议 → EVO-103-B。
- Repo Context API（file-tree / blobs / commits / diff）→ EVO-103-C。
- Commit / promote → EVO-105。
- 跨 tenant 公开 repo discover → Phase 5+。
- 软删除 repo（保留 stub N 天）→ Phase 5+ ops 特性（当前为硬删除）。

## Source Snapshot

- Source: EVO-103 拆分（2026-06-25），按端到端价值切片拆为 A/B/C。
- Decision context: 原始 EVO-103 范围（CRUD + Smart HTTP + Context API）超过 0.5-2 天窗口；拆分为独立可验收的纵向切片。
- 关键依赖：`gix::init` 创建裸仓；UNIQUE(tenant_id, name) 约束。
