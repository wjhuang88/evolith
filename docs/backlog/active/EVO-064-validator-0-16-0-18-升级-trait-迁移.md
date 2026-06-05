# EVO-064 validator 0.16→0.18+ 升级（trait 迁移）

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)

## Summary

| Field | Value |
| --- | --- |
| ID | EVO-064 |
| Type | tech-debt |
| Status | Proposed |
| Priority | P2 |
| Parent Epic | None recorded |
| Source | Iteration 032 依赖审计暂缓项 |

## Problem Or Outcome

0.18 起 `validate` 方法改成 `#[derive(Validate)]` + `validator::Validate` trait；9 个 domain 模型 + DTO 需适配；breaking 跨多个 minor

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
- 用户/工程价值：0.18 起 `validate` 方法从 `#[validate(...)]` 改为 `#[derive(Validate)]` + `validator::Validate` trait 调用；与 serde 生态更协调，错误类型统一为 `ValidationErrors`。
- 范围：升级 `validator = "0.16"` → `"0.18"`（或最新 0.20）；重写 9 个 domain / DTO 的 `validate` 调用路径；handler 错误映射同步更新。
- 不做：不升 0.19+（API 仍在微调）；不改校验规则（regex 长度等）。
- 验收标准：`cargo check / test --workspace` 0 error；register / login / forgot_password 等 handler 校验路径在 DTO 失败时仍返 400。
- 影响范围：domain 模型 + 8 个 DTO + auth handler 错误映射。
- 最小验证方式：`cargo test --workspace`；手测注册接口 `POST /api/v1/auth/register` 传 `password=short` 返 400。

## Validation Evidence Required

- Record real command output or manual evidence in the selected iteration before marking this item Done.
- Run checks matching the changed surface from [Testing SOP](../../sop/TESTING.md).

## Residual Work Destination

- Update this file and `docs/backlog/PRODUCT-BACKLOG.md` if scope, status, dependencies or Required Reads change.
- Move completed or deferred execution context to `docs/backlog/archive/<period>/` during backlog compaction.

## Source Snapshot

- Source: Iteration 032 依赖审计暂缓项
- Decision context: 0.18 起 `validate` 方法改成 `#[derive(Validate)]` + `validator::Validate` trait；9 个 domain 模型 + DTO 需适配；breaking 跨多个 minor
