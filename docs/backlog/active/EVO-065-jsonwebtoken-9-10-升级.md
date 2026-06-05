# EVO-065 jsonwebtoken 9→10 升级

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)

## Summary

| Field | Value |
| --- | --- |
| ID | EVO-065 |
| Type | tech-debt |
| Status | Proposed |
| Priority | P2 |
| Parent Epic | None recorded |
| Source | Iteration 032 依赖审计暂缓项 |

## Problem Or Outcome

9→10 改 `Header` / `EncodingKey` / `decode` 签名（with/without validation 合并）；影响 `service-auth/src/jwt.rs`；10.x 已稳定（10.4.0）

## Goal And Non-Goals

- Goal: deliver the outcome described by the title and decision context without expanding scope during implementation.
- Non-goals: do not implement adjacent backlog items unless they are listed as hard dependencies in this file.

## Dependencies And Blockers

- No hard blocker recorded in the source backlog.

## Governing ADRs, Specs Or Decisions

- None recorded beyond the product backlog and item file.

## Acceptance Criteria

- 类型：tech-debt
- 优先级：P2
- 状态：Proposed
- 来源：Iteration 032 依赖审计暂缓项 / 2026-06-01
- 用户/工程价值：9→10 合并 `decode` with/without validation、`Header` 改 `JoseHeader`、`EncodingKey::from_secret` 不变；与 openssl 0.10+ / RustCrypto 升级链对齐。
- 范围：升级 `jsonwebtoken = "9.2"` → `"10"`；重写 `service-auth/src/jwt.rs` 的 `encode` / `decode` 调用与 `Header` 构造。
- 不做：JWT payload 结构不变（`sub` / `exp` / `tenant_id` / `role`）；不改 token 有效期策略。
- 验收标准：`cargo check / test --workspace` 0 error；登录/登出/CsrfMiddleware token 解析路径不退化。
- 影响范围：`service-auth` + `api/src/middleware/csrf.rs` + `api/src/middleware/auth.rs`。
- 最小验证方式：`cargo test --workspace`；本地起服登录拿 token 访问 `/api/v1/auth/profile`。

## Validation Evidence Required

- Record real command output or manual evidence in the selected iteration before marking this item Done.
- Run checks matching the changed surface from [Testing SOP](../../sop/TESTING.md).

## Residual Work Destination

- Update this file and `docs/backlog/PRODUCT-BACKLOG.md` if scope, status, dependencies or Required Reads change.
- Move completed or deferred execution context to `docs/backlog/archive/<period>/` during backlog compaction.

## Source Snapshot

- Source: Iteration 032 依赖审计暂缓项
- Decision context: 9→10 改 `Header` / `EncodingKey` / `decode` 签名（with/without validation 合并）；影响 `service-auth/src/jwt.rs`；10.x 已稳定（10.4.0）
