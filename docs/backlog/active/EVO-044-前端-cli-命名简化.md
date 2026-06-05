# EVO-044 前端 CLI 命名简化

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)

## Summary

| Field | Value |
| --- | --- |
| ID | EVO-044 |
| Type | product-change |
| Status | Proposed |
| Priority | P1 |
| Parent Epic | None recorded |
| Source | 用户反馈 2026-05-29 |

## Problem Or Outcome

导航、页面、i18n 中 "CLI Interfaces" / "CLI 接口" 统一简化为 "CLI"

## Goal And Non-Goals

- Goal: deliver the outcome described by the title and decision context without expanding scope during implementation.
- Non-goals: do not implement adjacent backlog items unless they are listed as hard dependencies in this file.

## Dependencies And Blockers

- No hard blocker recorded in the source backlog.

## Governing ADRs, Specs Or Decisions

- None recorded beyond the product backlog and item file.

## Acceptance Criteria

- 类型：product-change
- 优先级：P1
- 状态：Proposed
- 用户价值或技术目标：缩短导航和页面中的 "CLI Interfaces" / "CLI 接口" 为 "CLI"，降低认知负担，与 Tools / Skills 保持同级简洁度。
- 验收标准：
  - [ ] 导航栏、页面标题、面包屑、空状态、按钮中 "CLI Interfaces" / "CLI 接口" 统一为 "CLI"
  - [ ] i18n 两个 locale 文件同步更新
  - [ ] `bun run build` 通过，无回归
- 依赖或阻塞：无
- 影响范围：frontend（i18n、Header、页面组件）
- 最小验证方式：`bun run build` 0 errors；视觉回归导航和页面标题
- 不做：不改后端 API 路径、不改数据模型字段名

## Validation Evidence Required

- Record real command output or manual evidence in the selected iteration before marking this item Done.
- Run checks matching the changed surface from [Testing SOP](../../sop/TESTING.md).

## Residual Work Destination

- Update this file and `docs/backlog/PRODUCT-BACKLOG.md` if scope, status, dependencies or Required Reads change.
- Move completed or deferred execution context to `docs/backlog/archive/<period>/` during backlog compaction.

## Source Snapshot

- Source: 用户反馈 2026-05-29
- Decision context: 导航、页面、i18n 中 "CLI Interfaces" / "CLI 接口" 统一简化为 "CLI"
