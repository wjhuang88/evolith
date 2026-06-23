# EVO-103 Repo CRUD + Smart HTTP + Repo Context API

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- [Git-Centric Platform Proposal](../../proposals/GIT-CENTRIC-PLATFORM.md)
- 父 Epic: [EVO-100](EVO-100-git-centric-platform-foundation.md)
- 依赖: [EVO-101](EVO-101-git-repos-schema.md)

## Summary

- 类型：feature / api
- 优先级：P0
- 状态：Proposed
- 父 Epic: EVO-100

## Problem Or Outcome

实现 Repo CRUD HTTP API、Smart HTTP git 协议（基于 `gix`）、Repo Context API（file-tree / blobs / commits / diff），让用户可在 Evolith 托管 git repo 并支持标准 git 客户端操作。

## Goal And Non-Goals

- **Goal**：
  1. Repo CRUD：`POST /api/v1/repos`、`GET /api/v1/repos/{id}`、`GET /api/v1/repos`、`PATCH /api/v1/repos/{id}`、`DELETE /api/v1/repos/{id}`
  2. Smart HTTP git 协议：`GET /repos/{id}/info/refs`、`POST /repos/{id}/git-upload-pack`、`POST /repos/{id}/git-receive-pack`（push 临时 git CLI subprocess）
  3. Repo Context API：`GET /repos/{id}/file-tree`、`GET /repos/{id}/blobs/{sha}`、`GET /repos/{id}/commits`、`GET /repos/{id}/diff`
- **Non-goals**：
  - 不实现 commit API 与 promote（EVO-105）
  - 不实现 SSH server（Phase 5）
  - 不实现 LFS（Phase 5）
  - 不实现 push 走纯 gix（`gix-push` 未到位；临时 subprocess；记 EVOLUTION）

## Dependencies And Blockers

- 依赖 EVO-101 `git_repos` 表
- 间接依赖 `gix` crate 已升级（EVO-086 Done）
- E2E 测试需 git CLI 客户端（项目已有）
- 单 tenant 内 UNIQUE name：删除后保留 N 天的 stub？

## Governing ADRs, Specs Or Decisions

- [ADR-0004 Git-Centric Storage](../../decisions/ADR-0004-git-centric-storage.md)

## Acceptance Criteria

- [ ] 6 个 REST endpoints（CRUD + list）实现并通过 RBAC 校验
- [ ] `POST /repos` 创建时自动 `gix::init` 裸仓到 `/srv/evolith/repos/{tenant_id}/{repo_id}.git`
- [ ] `POST /repos` 可选 `--seed-template` 参数，写入默认 `.evolith/agents.yaml` + `.evolith/policy.yaml` + README
- [ ] Smart HTTP `info/refs` 输出 `gix::protocol::ls_refs` 结果
- [ ] Smart HTTP `git-upload-pack` 流式输出 packfile（基于 `gix-pack`；参考 `gitserver` 项目）
- [ ] Smart HTTP `git-receive-pack` 临时调用 `git` CLI subprocess（`gix-push` 替代前）；记 EVOLUTION
- [ ] Repo Context API：file-tree 走 `gix::Tree::traverse`、blobs 走 `gix::Object::detach`、commits 走 `gix::revwalk`、diff 走 `gix::diff::tree`
- [ ] 所有 GET endpoints 接受 `?ref=` 参数（默认 main）
- [ ] RBAC：仅 tenant 内可访问；公开 repo 跨 tenant 仅 list 可见，blobs 需权限
- [ ] E2E 测试：`git clone http://localhost:8080/repos/{id}` 完整可用
- [ ] `cargo test --workspace` 与 `cargo clippy --workspace --all-targets -- -D warnings` 全绿

## Validation Evidence Required

- E2E 测试覆盖：create repo → push (含 SKILL.md) → clone → file-tree → blob → commit history → diff
- 性能基准：1000 个 repo list 查询 P95 < 200ms；file-tree (100 文件) P95 < 100ms（手工或 k6）
- Smart HTTP clone 在 Chromium devtools 中观察 handshake

## Residual Work Destination

- `gix-push` 合入后移除 subprocess 临时方案（独立 EVO）
- SSH（Phase 5）
- LFS（Phase 5）
- 跨 tenant 公开 repo discover（Phase 5+）

## Source Snapshot

- Source: 用户反馈 2026-06-23
- Decision context: 通用 git 托管服务的最小可用集合
- 关键依赖：`gix` crate（参考 `gitserver` 项目模式），`gix::protocol`、`gix-pack`、`gix::Tree`、`gix::Object`
- push 临时 subprocess 走 `git receive-pack` 命令（spec 完整）；记 EVOLUTION
