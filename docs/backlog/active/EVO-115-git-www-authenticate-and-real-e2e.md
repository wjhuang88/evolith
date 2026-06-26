# EVO-115 git Smart HTTP WWW-Authenticate + 真实 git-client E2E

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- 父 Epic: [EVO-103-B](EVO-103-B-smart-http-git-protocol.md)（祖父 EVO-103，曾祖 EVO-100）
- 祖父 Epic: [EVO-103](EVO-103-repo-context-and-smart-http.md)
- 曾祖 Epic: [EVO-100](EVO-100-git-centric-platform-foundation.md)
- 依赖: [EVO-103-B-2](EVO-103-B-2-smart-http-endpoints.md)（Review；B-2 实现完成，真实 E2E 待本项解锁）
- [EVO-103-B-1](EVO-103-B-1-git-client-auth-infra.md)（Done；Basic-Auth 基础设施）
- [ADR-0004 Git-Centric Storage](../../decisions/ADR-0004-git-centric-storage.md)

## Summary

- 类型：feature/test-debt
- 优先级：P1
- 状态：Proposed
- 父 Epic：EVO-103-B（祖父 EVO-103，曾祖 EVO-100）

## Problem Or Outcome

EVO-103-B-2 实现了 3 个 Smart HTTP 端点并通过 handler 级测试验证（auth 解析 + git subprocess + content-type），但真实 git-client E2E（`git clone`/`git push`/`git pull`）失败。

**根因**：git 客户端在克隆 `http://user:apikey@host/repos/{id}` 时，部分 git 版本不会主动发送 URL 中的凭证，而是等待服务器返回 `WWW-Authenticate: Basic` challenge。当前服务器 401（来自 rbac_middleware）缺少 `WWW-Authenticate: Basic` header，导致 git 报告 "Authentication failed"。标准 git 服务器（Gitea/GitLab）在 401 时均返回 `WWW-Authenticate: Basic`。

## Goal And Non-Goals

- **Goal**：
  1. 使服务器在 `/repos/` 路径的 401 响应中返回 `WWW-Authenticate: Basic realm="evolith"` header（针对性修改 rbac_middleware 或 B-2 特定 401 builder），以便 git 客户端收到 challenge 后使用 URL 凭证重试 Basic auth。
  2. 解禁 `#[ignore]`d 的 `test_git_clone_push_pull_e2e` 测试，使真实 `git clone`/`git push`/`git pull` 针对服务器通过。
- **Non-goals**：
  - 不改变现有 RBAC 权限模型或 API key 认证逻辑。
  - 不实现 Digest auth 或其他认证方式。
  - 不实现 SSH 或 LFS（Phase 5）。

## Dependencies And Blockers

- 依赖 EVO-103-B-2（Review）：3 个 Smart HTTP 端点已实现，handler 级测试通过。
- 依赖 EVO-103-B-1（Done）：Basic-Auth 基础设施已就绪。

## Governing ADRs, Specs Or Decisions

- [ADR-0004 Git-Centric Storage](../../decisions/ADR-0004-git-centric-storage.md)

## Acceptance Criteria

- [ ] `/repos/` 路径 401 响应包含 `WWW-Authenticate: Basic realm="evolith"` header
- [ ] `test_git_clone_push_pull_e2e` 解禁（移除 `#[ignore]`）并通过
- [ ] `git clone http://user:apikey@host/repos/{id}` 成功（exit 0）
- [ ] `git push` 成功推送 commits
- [ ] `git pull` 成功拉取远程更新
- [ ] `cargo test --workspace` 全绿
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` 0 errors

## Validation Evidence Required

- 真实 git CLI clone/push/pull E2E 针对已创建的 repo 完整通过。
- `cargo test --workspace` 全绿（含解禁的 E2E 测试）。
- `cargo clippy --workspace --all-targets -- -D warnings` 0 errors。

## Residual Work Destination

- 无（本项完成后 EVO-103-B-2 可转 Done）。

## Source Snapshot

- Source: EVO-103-B-2 Review 发现的残余工作（2026-06-26）。
- Decision context: git Smart HTTP 协议要求服务端 401 返回 `WWW-Authenticate: Basic`；标准 git server（Gitea/GitLab）均遵循此行为。
- 关键依赖：rbac_middleware 或 B-2 端点 401 builder 修改。
- 创建日期：2026-06-26。
