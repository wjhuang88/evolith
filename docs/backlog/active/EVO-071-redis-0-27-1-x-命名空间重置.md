# EVO-071 redis 0.27→1.x 命名空间重置

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)

## Summary

| Field | Value |
| --- | --- |
| ID | EVO-071 |
| Type | tech-debt |
| Status | Proposed |
| Priority | P3 |
| Parent Epic | None recorded |
| Source | Iteration 032 依赖审计暂缓项 |

## Problem Or Outcome

0.x→1.0 是显式 breaking（trait 完全重写）；`service-payment` 暂未真实使用（infra 缓存层预留）

## Goal And Non-Goals

- Goal: deliver the outcome described by the title and decision context without expanding scope during implementation.
- Non-goals: do not implement adjacent backlog items unless they are listed as hard dependencies in this file.

## Dependencies And Blockers

- No hard blocker recorded in the source backlog.

## Governing ADRs, Specs Or Decisions

- None recorded beyond the product backlog and item file.

## Acceptance Criteria

- 类型：tech-debt
- 优先级：P3
- 状态：Proposed
- 来源：Iteration 032 依赖审计暂缓项 / 2026-06-01
- 用户/工程价值：redis crate 0.x→1.0 是显式 breaking（trait 完全重写、`AsyncCommands` 重命名、新 `Connection` 抽象）。
- 范围：升级 `redis = "0.27"` → `"1.2"`；重写 `infra/src/cache.rs` 的 `RedisCache` 实现（`service-payment` 暂未真实使用）。
- 不做：保留 `InMemoryCache` 作为 fallback；不引入 redis cluster / sentinel。
- 验收标准：`cargo check / test --workspace` 0 error；`RedisCache` 单元测试通过（如有）。
- 影响范围：`infra`。
- 最小验证方式：`cargo test -p infra`（mock 模式）；本地启动 docker compose redis 7-alpine 后集成测试。

## Validation Evidence Required

- Record real command output or manual evidence in the selected iteration before marking this item Done.
- Run checks matching the changed surface from [Testing SOP](../../sop/TESTING.md).

## Residual Work Destination

- Update this file and `docs/backlog/PRODUCT-BACKLOG.md` if scope, status, dependencies or Required Reads change.
- Move completed or deferred execution context to `docs/backlog/archive/<period>/` during backlog compaction.

## Source Snapshot

- Source: Iteration 032 依赖审计暂缓项
- Decision context: 0.x→1.0 是显式 breaking（trait 完全重写）；`service-payment` 暂未真实使用（infra 缓存层预留）
