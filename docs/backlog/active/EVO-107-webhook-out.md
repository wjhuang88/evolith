# EVO-107 Webhook Out（push / promote → external agent engine）

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- [Git-Centric Platform Proposal](../../proposals/GIT-CENTRIC-PLATFORM.md)
- 父 Epic: [EVO-100](EVO-100-git-centric-platform-foundation.md)
- 依赖: [EVO-105](EVO-105-commit-and-promote-api.md), [EVO-106](EVO-106-agent-session-and-scoped-token.md)

## Summary

- 类型：feature / api
- 优先级：P1
- 状态：Proposed
- 父 Epic: EVO-100

## Problem Or Outcome

当 git push、commit promote、PR（可选）等事件发生时，Evolith 通过 webhook POST 到外部 agent engine 的 callback URL，让 agent 能被动响应 repo 事件（如自动 review、auto-fix、CI 触发）。失败 retry 与记录。

## Goal And Non-Goals

- **Goal**：
  1. `webhook_deliveries` 表：id、tenant_id、repo_id、session_id（可选）、event_type、target_url、payload、status、attempt_count、last_attempt_at、next_retry_at、response_status、response_body
  2. 触发事件：`repo.push`（commit 创建后）、`repo.promote`（分支合并到 default_branch 后）
  3. 重试策略：指数退避（1s / 5s / 30s / 5min / 30min），最多 5 次
  4. 失败记录：连续 5 次失败后状态 `failed`，不再重试
  5. 签名验证：每个 webhook 携带 `X-Evolith-Signature` HMAC-SHA256 头
- **Non-goals**：
  - 不实现 webhook 接收端（外部系统提供；Evolith 只发不发收）
  - 不实现 webhook UI 配置页（EVO-104 Vibe Coding UI 后续）
  - 不实现 webhook filter（按 path / branch 过滤；Phase 5+）
  - 不实现 PR 事件（Phase 5+；MVP 无 PR UI）

## Dependencies And Blockers

- 依赖 EVO-105 commit/promote 触发事件
- 依赖 EVO-106 agent session 用于 session_id 关联
- 依赖 EVO-118-H-C 的 typed durable event/worker contract；Webhook 外发必须作为后续
  subscriber 复用 Outbox，不在 HTTP handler 中直接 spawn 或把出站失败伪装成功。

## Governing ADRs, Specs Or Decisions

- [ADR-0004 Git-Centric Storage](../../decisions/ADR-0004-git-centric-storage.md)

## Acceptance Criteria

- [ ] `webhook_deliveries` 表 + Repository trait + 双实现 + migration
- [ ] `webhook_delivery_attempts` 表（每次 attempt 一行）或 attempt 嵌入 JSON 字段（设计决策）
- [ ] 触发逻辑：commit / promote 完成后入队 webhook delivery（异步任务，避免阻塞 HTTP 响应）
- [ ] HTTP POST 实现：携带 `X-Evolith-Signature`、`X-Evolith-Event`、`X-Evolith-Delivery` 头 + JSON payload
- [ ] HMAC-SHA256 签名：payload + secret → 16 进制
- [ ] 重试 worker（tokio task）：扫描 `next_retry_at <= now AND status = pending`，执行 POST
- [ ] 失败处理：response status >= 500 或 timeout（10s）→ retry；4xx → 立即 failed（不重试）
- [ ] `cargo test --workspace` 与 `cargo clippy --workspace --all-targets -- -D warnings` 全绿
- [ ] E2E：mock agent 接收 webhook → 校验 signature → 校验 payload

## Validation Evidence Required

- E2E 测试：commit → webhook 在 5s 内送达 → mock agent 验签成功
- 重试测试：mock agent 返回 500 → 验证 5 次 attempt + 指数退避 + 最终 failed
- 4xx 测试：mock agent 返回 400 → 立即 failed 不重试
- 签名测试：篡改 payload → mock agent 验签失败

## Residual Work Destination

- webhook UI 配置页（EVO-104 Vibe Coding UI 后续）
- webhook filter（path / branch 过滤；Phase 5+）
- PR 事件支持（Phase 5+）
- 多 webhook target per event（独立 EVO）

## Source Snapshot

- Source: 用户反馈 2026-06-23
- Decision context: webhook 是 git-centric 平台与外部 agent engine 通信的主通道
- 参考模式：GitHub Webhooks、GitLab Webhooks、Linear Agent events
