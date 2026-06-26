# EVO-103-B-2 Smart HTTP Endpoints（git subprocess）

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- 父 Epic: [EVO-103-B](EVO-103-B-smart-http-git-protocol.md)（祖父 EVO-103，曾祖 EVO-100）
- 祖父 Epic: [EVO-103](EVO-103-repo-context-and-smart-http.md)
- 曾祖 Epic: [EVO-100](EVO-100-git-centric-platform-foundation.md)
- 硬依赖: [EVO-103-B-1](EVO-103-B-1-git-client-auth-infra.md)（Ready；B-1 完成前 B-2 维持 Proposed）
- 依赖: [EVO-103-A](EVO-103-A-repo-crud.md)（Done，提供 repo 上下文）
- [ADR-0004 Git-Centric Storage](../../decisions/ADR-0004-git-centric-storage.md)

## Summary

- 类型：feature / api
- 优先级：P0
- 状态：Review
- 父 Epic：EVO-103-B（祖父 EVO-103，曾祖 EVO-100）

## Problem Or Outcome

作为 git 用户，我需要能通过标准 git 客户端（`git clone` / `git push` / `git pull`）与 Evolith 托管的仓库交互，以便使用现有 git 工作流而无需迁移。

**关键技术纠正**：已发布的 `gix` crate（v0.78.0）**没有任何服务端协议支持**。所有 3 个 Smart HTTP 端点当前必须通过 `git` 二进制 subprocess 实现。gix 服务端 upload-pack 仅在未合并 PR #2465 中；gix 维护者明确表示服务端 receive-pack 无计划（推荐 shelling out to git）。所有生产级 Rust git 服务器（如 loom）均使用 `git --stateless-rpc` subprocess 实现全部端点。

## Goal And Non-Goals

- **Goal**：
  1. 实现 3 个 Smart HTTP 端点（**全部通过 `git` CLI subprocess**，非 gix）：
     - `GET /repos/{id}/info/refs?service=git-upload-pack|git-receive-pack` → 运行 `git upload-pack|receive-pack --advertise-refs --stateless-rpc <repo_path>`，前置 `001e# service=git-<svc>\n0000` 广告头，content-type `application/x-git-<svc>-advertisement`，`Cache-Control: no-cache`。
     - `POST /repos/{id}/git-upload-pack` → 将请求体（`web::Payload`）流式传入 `git upload-pack --stateless-rpc <repo_path>` stdin，将 stdout 流式返回，content-type `application/x-git-upload-pack-result`。
     - `POST /repos/{id}/git-receive-pack` → 同上模式，使用 `git receive-pack --stateless-rpc <repo_path>`，content-type `application/x-git-receive-pack-result`。
  2. **安全 subprocess**：`Command::new("git")`（NO shell），固定参数向量，repo_path 通过 DB 按 repo id 查找（绝不从 URL 读取），不透明 body 传入 stdin，`tokio::time::timeout` 防止挂起。service 参数验证为枚举 {upload-pack, receive-pack}。
  3. **Docker 运行时**：在 `backend/Dockerfile` 的 `debian:bookworm-slim` 运行时阶段添加 `git`（`apt-get install git`）。这是影响发布的镜像变更——需更新 `docs/reference/SCRIPTS-RELEASE-NOTES.md`（按 AGENTS.md 规则）。
  4. **Actix 流式**：响应通过 `HttpResponse::Ok().insert_header(...).streaming(stream)`（返回 `HttpResponse<BoxBody>`）；请求通过 `web::Payload`。Actix-web 4.13.0。
  5. **RBAC**：repo 属于调用者的 tenant（tenant-scoped；使用 B-1 的 Basic-Auth 身份）。
  6. 支持标准 git 客户端 clone / push / pull 操作。
- **Non-goals**：
  - 不实现 auth 基础设施（→ EVO-103-B-1）。
  - 不使用 gix 服务端（已发布 crate 中不存在）。
  - 不实现 SSH server（Phase 5）。
  - 不实现 LFS（Phase 5）。
  - 不实现纯 gix push（gix 维护者：receive-pack 无计划）。

## Dependencies And Blockers

- 硬依赖 EVO-103-B-1（Ready → In Progress；B-1 完成前 B-2 维持 Proposed）：Basic-Auth + RBAC 挂载 + CSRF 豁免必须先就绪。
- 依赖 EVO-103-A（Done）：repo 必须先通过 CRUD 创建并初始化裸仓。
- E2E 测试需 git CLI 客户端（项目已有）。
- **阻塞**：EVO-103-B-1 未完成前无法进入 Ready。

## Governing ADRs, Specs Or Decisions

- [ADR-0004 Git-Centric Storage](../../decisions/ADR-0004-git-centric-storage.md)

## Acceptance Criteria

> **BDD 不适用原因**：Smart HTTP 是协议级 git smart HTTP 实现，Given/When/Then 场景不自然。采用等价技术验收（命令级 E2E 验证）。

- [x] `GET /repos/{id}/info/refs` 返回正确 content-type（`application/x-git-*-advertisement`），git 客户端可解析（handler 级测试验证：auth 解析 + git subprocess + content-type 200）
- [ ] `POST /repos/{id}/git-upload-pack` 流式返回 packfile 结果（`application/x-git-upload-pack-result`）— 端点已实现，真实 git-client E2E 待 WWW-Authenticate（→ EVO-115）
- [ ] `POST /repos/{id}/git-receive-pack` 流式返回 receive-pack 结果（`application/x-git-receive-pack-result`）— 端点已实现，真实 git-client E2E 待 WWW-Authenticate（→ EVO-115）
- [ ] `git clone http://host/repos/{id}` 成功克隆（exit 0）— 真实 git-client E2E 待 WWW-Authenticate（→ EVO-115）
- [ ] `git push` 成功推送 commits 到远程仓库 — 真实 git-client E2E 待 WWW-Authenticate（→ EVO-115）
- [ ] `git pull` 成功拉取远程更新 — 真实 git-client E2E 待 WWW-Authenticate（→ EVO-115）
- [x] 跨 tenant repo → 403/404（RBAC middleware 覆盖）
- [x] subprocess timeout enforced（`tokio::time::timeout`）
- [x] `cargo test --workspace` 与 `cargo clippy --workspace --all-targets -- -D warnings` 全绿
- [x] `docs/reference/SCRIPTS-RELEASE-NOTES.md` 已更新（Docker 镜像新增 git 包）

### 等价技术验收（替代 BDD）

1. **E2E clone**：`git clone http://localhost:8080/repos/{id} /tmp/test-clone` 成功退出码 0，`.git` 目录存在。
2. **E2E push**：在克隆仓库中创建 commit 后 `git push` 成功，远程仓库包含新 commit。
3. **E2E pull**：另一克隆仓库 `git pull` 可获取 push 的内容。
4. **协议观察**：info/refs 请求返回 `application/x-git-upload-pack-advertisement` 或 `application/x-git-receive-pack-advertisement` content-type。
5. **跨租户隔离**：非所属 tenant 的 repo 访问 → 403/404。
6. **超时强制**：subprocess 超时后请求被终止，不挂起服务。

## Validation Evidence Required

- E2E git CLI clone/push/pull 针对已创建的 repo 完整通过。
- Smart HTTP handshake 在 devtools 或抓包中可观察。
- `cargo test --workspace` 全绿。
- `cargo clippy --workspace --all-targets -- -D warnings` 0 errors。

## Residual Work Destination

- 当 gix PR #2465（服务端 upload-pack）合并后，可将 info/refs + upload-pack subprocess 替换为纯 gix——跟踪为未来 EVO；**receive-pack 永久保持 subprocess**（无 gix 服务端 receive-pack 计划，gix 维护者推荐 shelling out）。
- 将 subprocess 决策写入 EVOLUTION.md。
- SSH server（Phase 5）。
- LFS（Phase 5）。

## Source Snapshot

- Source: EVO-103-B 拆分（2026-06-25），将协议端点实现从 auth 基础设施分离。
- Decision context: 已发布 gix crate 无服务端协议支持；所有 3 个端点必须使用 `git --stateless-rpc` subprocess；gix PR #2465（upload-pack）未合并；receive-pack subprocess 为永久方案。
- 关键依赖：`git` CLI subprocess（`Command::new("git")`）、Actix-web 流式（`web::Payload` + `HttpResponse::streaming()`）、`tokio::time::timeout`。
- 拆分日期：2026-06-25。
