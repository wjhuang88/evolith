# EVO-106 Agent Session API + Scoped Token

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- [Git-Centric Platform Proposal](../../proposals/GIT-CENTRIC-PLATFORM.md)
- 父 Epic: [EVO-100](EVO-100-git-centric-platform-foundation.md)
- 依赖: [EVO-103](EVO-103-repo-context-and-smart-http.md)

## Summary

- 类型：feature / api
- 优先级：P0
- 状态：Proposed
- 父 Epic: EVO-100

## Problem Or Outcome

为外部 agent engine 提供 session 抽象 + scoped token，让 agent 可以 Evolith 用户的身份对单个 repo 执行受控操作（read + commit + promote）。所有操作可审计、可重放、可撤销。

## Goal And Non-Goals

- **Goal**：
  1. `agent_sessions` 表：id、tenant_id、repo_id、external_engine_url、scoped_token_id、status、created_at、updated_at、completed_at
  2. `POST /api/v1/agent-sessions`：分配短期 scoped token（24h TTL）+ 返回 session_id + token
  3. Token 权限模型：`read_repo` + `create_branch` + `commit` + `open_pr`（可选）；**禁止** `force_push` / `delete_branch` / `admin`
  4. Session 状态机：`created` → `active` → `completed` | `failed` | `expired`
  5. Session 事件日志（append-only）：用于审计与回放
- **Non-goals**：
  - 不实现 agent 内部消息协议（外部 engine 提供）
  - 不实现 multi-repo session（每个 session 绑一个 repo）
  - 不实现 session 间的消息总线（独立 EVO）
  - 不实现 token refresh（一次性 24h，到期重发）

## Dependencies And Blockers

- 依赖 EVO-103 Repo Context API（agent 通过此读取 repo 内容）
- 依赖 EVO-105 Commit API（agent 通过此写 commit）
- 不依赖外部 agent engine 实现（mock 即可走通 Evolith 侧）

## Governing ADRs, Specs Or Decisions

- [ADR-0004 Git-Centric Storage](../../decisions/ADR-0004-git-centric-storage.md)
- [ADR-0007 Agent 写入授权与生产交付边界](../../decisions/ADR-0007-agent-write-and-production-delivery-boundaries.md)

## Acceptance Criteria

- [ ] `agent_sessions` 表 + Repository trait + 双实现 + migration
- [ ] 独立 `scoped_tokens` 表：存储 token hash、typed capabilities、session/repo/tenant 绑定、expires_at、revoked_at
- [ ] `POST /api/v1/agent-sessions` 实现：一次性返回 opaque token；后续请求使用 `X-Agent-Session`，服务端检查 hash、状态、有效期和资源绑定
- [ ] Token 校验 middleware：`X-Agent-Session` header 携带 token；校验 scope 是否允许操作
- [ ] Session 创建需 RBAC 校验：用户对 repo 有 owner/admin 权限才可创建 session
- [ ] Session 状态机转换规则：`completed` / `failed` / `expired` 不可回退为 `active`
- [ ] Session 事件 append-only log：每次 agent 操作（commit / read / promote）追加一条 event，含 session_id、event_type、payload、timestamp
- [ ] `cargo test --workspace` 与 `cargo clippy --workspace --all-targets -- -D warnings` 全绿
- [ ] 安全：scope 校验失败时返回 403 + 记录 audit log

## Validation Evidence Required

- E2E 测试：用户创建 session → 模拟 agent 用 token 调 commit API → session 记录 event → session expire
- 安全测试：尝试用 token 调 admin API → 403
- Scope 粒度测试：`read_repo` token 不能 commit；`commit` token 不能 promote
- Session 状态机单元测试覆盖所有合法 / 非法转换

## Residual Work Destination

- Multi-repo session（独立 EVO）
- Token refresh（独立 EVO）
- Session 间消息总线（独立 EVO）
- Session 终止时自动 promote 策略（独立 EVO）

## Source Snapshot

- Source: 用户反馈 2026-06-23
- Decision context: agent engine 外部构建；Evolith 仅暴露 integration surface
- 参考模式：OpenHands SDK Agent/Conversation/Workspace、Open SWE 沙箱模式、Linear Agent 权限模型
