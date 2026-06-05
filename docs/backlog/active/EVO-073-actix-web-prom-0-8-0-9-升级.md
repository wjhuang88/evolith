# EVO-073 actix-web-prom 0.8→0.9+ 升级

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)

## Summary

| Field | Value |
| --- | --- |
| ID | EVO-073 |
| Type | tech-debt |
| Status | Proposed |
| Priority | P3 |
| Parent Epic | None recorded |
| Source | Iteration 032 依赖审计暂缓项 |

## Problem Or Outcome

0.9 起改 `PrometheusMetricsBuilder` API；metrics endpoint 路径需调整；0.10 latest

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
- 用户/工程价值：0.9 起改 `PrometheusMetricsBuilder` API（命名空间、registry 配置变化）；0.10 latest。
- 范围：升级 `actix-web-prom = "0.8"` → `"0.10"`；`api/src/lib.rs` 构造 + metrics endpoint 路径同步。
- 不做：不引入 OpenMetrics exporter；不改 metric 标签体系。
- 验收标准：`cargo check / test --workspace` 0 error；`/metrics` endpoint 仍返回 Prometheus 格式。
- 影响范围：`api`。
- 最小验证方式：`cargo test -p api`；`curl /metrics` 返回非空 Prometheus 文本。

## Validation Evidence Required

- Record real command output or manual evidence in the selected iteration before marking this item Done.
- Run checks matching the changed surface from [Testing SOP](../../sop/TESTING.md).

## Residual Work Destination

- Update this file and `docs/backlog/PRODUCT-BACKLOG.md` if scope, status, dependencies or Required Reads change.
- Move completed or deferred execution context to `docs/backlog/archive/<period>/` during backlog compaction.

## Source Snapshot

- Source: Iteration 032 依赖审计暂缓项
- Decision context: 0.9 起改 `PrometheusMetricsBuilder` API；metrics endpoint 路径需调整；0.10 latest
