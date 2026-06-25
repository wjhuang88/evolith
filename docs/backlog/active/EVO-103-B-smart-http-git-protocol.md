# EVO-103-B Smart HTTP git 协议

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- 父 Epic: [EVO-103](EVO-103-repo-context-and-smart-http.md)
- 祖父 Epic: [EVO-100](EVO-100-git-centric-platform-foundation.md)
- 硬依赖: [EVO-103-A](EVO-103-A-repo-crud.md)（Ready；A 完成前 B 维持 Proposed）
- [ADR-0004 Git-Centric Storage](../../decisions/ADR-0004-git-centric-storage.md)

## Summary

- 类型：feature / api
- 优先级：P0
- 状态：Proposed
- 父 Epic：EVO-103（祖父 EVO-100）

## Problem Or Outcome

作为 git 用户，我需要能通过标准 git 客户端（`git clone` / `git push` / `git pull`）与 Evolith 托管的仓库交互，以便使用现有 git 工作流而无需迁移。

## Goal And Non-Goals

- **Goal**：
  1. 实现 Smart HTTP git 协议端点：
     - `GET /repos/{id}/info/refs` — 通过 `gix::protocol::ls_refs` 输出 refs。
     - `POST /repos/{id}/git-upload-pack` — 通过 `gix-pack` 流式输出 packfile。
     - `POST /repos/{id}/git-receive-pack` — 临时调用 `git` CLI subprocess（`gix-push` 未就绪前）。
  2. 支持标准 git 客户端 clone / push / pull 操作。
- **Non-goals**：
  - 不实现 repo CRUD（→ EVO-103-A）。
  - 不实现 file-tree / blobs / commits / diff API（→ EVO-103-C）。
  - 不实现 SSH server（Phase 5）。
  - 不实现 LFS（Phase 5）。
  - 不实现纯 gix push（`gix-push` 未合入；临时 subprocess 记 EVOLUTION）。

## Dependencies And Blockers

- 硬依赖 EVO-103-A（Proposed）：repo 必须先通过 CRUD 创建并初始化裸仓。
- 间接依赖 `gix` crate（EVO-086 Done）。
- E2E 测试需 git CLI 客户端（项目已有）。
- **阻塞**：EVO-103-A 未完成前无法进入 Ready。

## Governing ADRs, Specs Or Decisions

- [ADR-0004 Git-Centric Storage](../../decisions/ADR-0004-git-centric-storage.md)

## Acceptance Criteria

> **BDD 不适用原因**：Smart HTTP 是协议级 git smart HTTP 实现，Given/When/Then 场景不自然。采用等价技术验收（命令级 E2E 验证）。

- [ ] `GET /repos/{id}/info/refs` 输出 `gix::protocol::ls_refs` 结果，git 客户端可解析
- [ ] `POST /repos/{id}/git-upload-pack` 流式输出 packfile（基于 `gix-pack`）
- [ ] `POST /repos/{id}/git-receive-pack` 临时调用 `git` CLI subprocess 完成 push（`gix-push` 替代前）
- [ ] `git clone http://localhost:8080/repos/{id}` 成功克隆裸仓
- [ ] `git push` 成功推送 commits 到远程仓库
- [ ] `git pull` 成功拉取远程更新
- [ ] info/refs handshake 可在 Chromium devtools 或 tcpdump 中观察
- [ ] `cargo test --workspace` 与 `cargo clippy --workspace --all-targets -- -D warnings` 全绿

### 等价技术验收（替代 BDD）

1. **E2E clone**：`git clone http://localhost:8080/repos/{id} /tmp/test-clone` 成功退出码 0，`.git` 目录存在。
2. **E2E push**：在克隆仓库中创建 commit 后 `git push` 成功，远程仓库包含新 commit。
3. **E2E pull**：另一克隆仓库 `git pull` 可获取 push 的内容。
4. **协议观察**：info/refs 请求返回 `application/x-git-upload-pack-advertisement` 或 `application/x-git-receive-pack-advertisement` content-type。

## Validation Evidence Required

- E2E git CLI clone/push/pull 针对已创建的 repo 完整通过。
- Smart HTTP handshake 在 devtools 或抓包中可观察。
- `cargo test --workspace` 全绿。
- `cargo clippy --workspace --all-targets -- -D warnings` 0 errors。

## Residual Work Destination

- `gix-push` 合入后移除 git CLI subprocess 临时方案（独立 EVO 跟踪，记 EVOLUTION）。
- SSH server（Phase 5）。
- LFS（Phase 5）。

## Source Snapshot

- Source: EVO-103 拆分（2026-06-25），按端到端价值切片拆为 A/B/C。
- Decision context: Smart HTTP 是 git 协议层实现，依赖 EVO-103-A 的裸仓初始化。
- 关键依赖：`gix::protocol::ls_refs`、`gix-pack`、临时 `git receive-pack` subprocess。
- push 临时 subprocess 走 `git receive-pack` 命令（spec 完整）；记 EVOLUTION。
