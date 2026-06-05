# EVO-067 jsonschema 0.17→0.46 升级

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)

## Summary

| Field | Value |
| --- | --- |
| ID | EVO-067 |
| Type | tech-debt |
| Status | Proposed |
| Priority | P3 |
| Parent Epic | None recorded |
| Source | Iteration 032 依赖审计暂缓项 |

## Problem Or Outcome

跨 28 个 minor 的 schema draft / API 演进；`service-tool` 内部使用 0.17 即可；升级涉及 `validator` / `draft-7` / `draft-2020` API 切换

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
- 用户/工程价值：跨 28 个 minor 的 schema draft 支持演进（draft-07 → draft 2019-09 / 2020-12）；新 `Retrieval` / `Validator` trait 抽象。
- 范围：升级 `jsonschema = "0.17"` → `"0.46"`；重写 `service-tool` 的 schema 校验入口（如果有 trait 改动）。
- 不做：暂不升级 draft 语义（仍用 draft-07 校验 MCP tool 的 input_schema）。
- 验收标准：`cargo check / test --workspace` 0 error；MCP `tools/call` 校验路径不退化。
- 影响范围：`service-tool`。
- 最小验证方式：`cargo test -p service-tool`；MCP tools/call 测试用例 `mcp_tool_execution_tests` 仍 4 通过。

## Validation Evidence Required

- Record real command output or manual evidence in the selected iteration before marking this item Done.
- Run checks matching the changed surface from [Testing SOP](../../sop/TESTING.md).

## Residual Work Destination

- Update this file and `docs/backlog/PRODUCT-BACKLOG.md` if scope, status, dependencies or Required Reads change.
- Move completed or deferred execution context to `docs/backlog/archive/<period>/` during backlog compaction.

## Source Snapshot

- Source: Iteration 032 依赖审计暂缓项
- Decision context: 跨 28 个 minor 的 schema draft / API 演进；`service-tool` 内部使用 0.17 即可；升级涉及 `validator` / `draft-7` / `draft-2020` API 切换
