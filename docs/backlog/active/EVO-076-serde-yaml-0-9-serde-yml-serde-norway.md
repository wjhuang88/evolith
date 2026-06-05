# EVO-076 serde_yaml 0.9 → serde_yml / serde_norway 迁移

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)

## Summary

| Field | Value |
| --- | --- |
| ID | EVO-076 |
| Type | tech-debt |
| Status | Proposed |
| Priority | P3 |
| Parent Epic | None recorded |
| Source | Iteration 032 依赖审计暂缓项 |

## Problem Or Outcome

上游 serde_yaml 0.9 已 deprecate；建议迁移到 `serde_yml`（社区 fork）或 `serde_norway`（纯 Rust 替代）；影响 `service-skill` + `service-snippet` 的 YAML 解析路径

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
- 用户/工程价值：serde_yaml 0.9 已 deprecate；社区 fork `serde_yml` 与纯 Rust 替代 `serde_norway` 提供活跃维护路径。
- 范围：`service-skill` + `service-snippet` 内的 `serde_yaml::from_str` / `to_string` 调用迁移到 `serde_yml`（推荐，社区活跃）或 `serde_norway`（纯 Rust 替代）。
- 不做：保留 YAML 格式本身（不切 JSON）；不改 SKILL.md / CLI 格式定义。
- 验收标准：`cargo check / test --workspace` 0 error；现有 SKILL.md parser 测试不退化。
- 影响范围：`service-skill` + `service-snippet`。
- 最小验证方式：`cargo test -p service-skill -p service-snippet`。

## Validation Evidence Required

- Record real command output or manual evidence in the selected iteration before marking this item Done.
- Run checks matching the changed surface from [Testing SOP](../../sop/TESTING.md).

## Residual Work Destination

- Update this file and `docs/backlog/PRODUCT-BACKLOG.md` if scope, status, dependencies or Required Reads change.
- Move completed or deferred execution context to `docs/backlog/archive/<period>/` during backlog compaction.

## Source Snapshot

- Source: Iteration 032 依赖审计暂缓项
- Decision context: 上游 serde_yaml 0.9 已 deprecate；建议迁移到 `serde_yml`（社区 fork）或 `serde_norway`（纯 Rust 替代）；影响 `service-skill` + `service-snippet` 的 YAML 解析路径
