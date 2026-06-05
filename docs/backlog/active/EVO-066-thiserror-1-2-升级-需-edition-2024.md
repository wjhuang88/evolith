# EVO-066 thiserror 1→2 升级（需 edition 2024）

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)

## Summary

| Field | Value |
| --- | --- |
| ID | EVO-066 |
| Type | tech-debt |
| Status | Proposed |
| Priority | P2 |
| Parent Epic | None recorded |
| Source | Iteration 032 依赖审计暂缓项 |

## Problem Or Outcome

1→2 需 `edition = "2024"`；影响 `common/src/error.rs` + 8 个 service error 类型；与 EVO-043 「不升级 Rust edition」约束冲突，须先解除 edition 升级前置

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
- 用户/工程价值：thiserror 2.0 需 `edition = "2024"`；与 Rust 1.85+ 强制规则同步；`#[from]` 自动 trait 路径推断。
- 范围：升级 `thiserror = "1.0"` → `"2"`；同步把 `[workspace.package] edition = "2021"` 升到 `"2024"`；`common/src/error.rs` + 8 个 service error 类型迁移。
- 不做：不引入 thiserror 2 的新 feature；保持现有 `AppError` enum 形状。
- 前置依赖：Rust edition 升级（Iteration 032 plan §3 明确「不升级 Rust edition」，本项激活时需先解除该约束）。
- 验收标准：`cargo check / test --workspace` 0 error；所有 `AppError` 变体仍能正确显示。
- 影响范围：所有 backend crate 的 `thiserror::Error` derive + Rust edition。
- 最小验证方式：`cargo test --workspace`；`grep -r "edition = " backend/Cargo.toml backend/crates/*/Cargo.toml` 全部 2024。

## Validation Evidence Required

- Record real command output or manual evidence in the selected iteration before marking this item Done.
- Run checks matching the changed surface from [Testing SOP](../../sop/TESTING.md).

## Residual Work Destination

- Update this file and `docs/backlog/PRODUCT-BACKLOG.md` if scope, status, dependencies or Required Reads change.
- Move completed or deferred execution context to `docs/backlog/archive/<period>/` during backlog compaction.

## Source Snapshot

- Source: Iteration 032 依赖审计暂缓项
- Decision context: 1→2 需 `edition = "2024"`；影响 `common/src/error.rs` + 8 个 service error 类型；与 EVO-043 「不升级 Rust edition」约束冲突，须先解除 edition 升级前置
