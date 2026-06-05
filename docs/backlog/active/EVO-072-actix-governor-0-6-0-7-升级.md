# EVO-072 actix-governor 0.6→0.7+ 升级

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)

## Summary

| Field | Value |
| --- | --- |
| ID | EVO-072 |
| Type | tech-debt |
| Status | Proposed |
| Priority | P2 |
| Parent Epic | None recorded |
| Source | Iteration 032 依赖审计暂缓项 |

## Problem Or Outcome

0.7 起 `Governor` 中间件构造改 actix-web 4.x 新 `KeyExtractor` trait；影响 `api/src/middleware/rate_limit.rs`；0.10 latest

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
- 用户/工程价值：0.7 起 `Governor` 中间件构造改 actix-web 4.x 新 `KeyExtractor` trait；0.10 latest 加入 `cfg` 模块化配置。
- 范围：升级 `actix-governor = "0.6"` → `"0.10"`；`api/src/middleware/rate_limit.rs` 迁移 KeyExtractor 构造。
- 不做：不调整 rate limit 阈值（保留当前 unauthenticated/authenticated/api_key 区分）。
- 验收标准：`cargo check / test --workspace` 0 error；rate limit middleware 单元测试通过。
- 影响范围：`api/src/middleware/rate_limit.rs`。
- 最小验证方式：`cargo test -p api`；`/api/v1/auth/login` 错误密码 30+ 次触发 429。

## Validation Evidence Required

- Record real command output or manual evidence in the selected iteration before marking this item Done.
- Run checks matching the changed surface from [Testing SOP](../../sop/TESTING.md).

## Residual Work Destination

- Update this file and `docs/backlog/PRODUCT-BACKLOG.md` if scope, status, dependencies or Required Reads change.
- Move completed or deferred execution context to `docs/backlog/archive/<period>/` during backlog compaction.

## Source Snapshot

- Source: Iteration 032 依赖审计暂缓项
- Decision context: 0.7 起 `Governor` 中间件构造改 actix-web 4.x 新 `KeyExtractor` trait；影响 `api/src/middleware/rate_limit.rs`；0.10 latest
