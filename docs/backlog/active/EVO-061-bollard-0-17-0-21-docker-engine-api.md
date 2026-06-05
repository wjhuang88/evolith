# EVO-061 bollard 0.17→0.21 Docker Engine API 升级

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)

## Summary

| Field | Value |
| --- | --- |
| ID | EVO-061 |
| Type | tech-debt |
| Status | Proposed |
| Priority | P3 |
| Parent Epic | None recorded |
| Source | Iteration 032 依赖审计暂缓项 |

## Problem Or Outcome

bollard 跨 4 个 minor 的 Docker Engine API 演进（0.18 起 `Docker::connect_with_*` API 调整）；当前 `bollard 0.17.1` + Phase 7 sandbox（service-skill）零运行时问题；迁移需重写 `service-skill/src/executor.rs` + 验证 sandbox 镜像兼容性

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
- 用户/工程价值：bollard 0.18 起 `Docker::connect_with_*` API 调整、0.19+ 引入 `exec` 异步流式 API、0.20+ 改 `ContainerStats` 返回类型；当前 0.17.1 在 Phase 7 sandbox 验证通过但 4 个 minor 的 bugfix 缺失。
- 范围：升级 `bollard = "0.17"` → `"0.21"`；重写 `service-skill/src/executor.rs` 的容器启动/停止/日志流；验证 `backend/sandbox/{python,node}/Dockerfile` 在新 bollard API 下行为不变。
- 不做：升级 `service-skill` Phase 7 之外的 bollard 使用方（实际仅 executor.rs）。
- 验收标准：`cargo check / clippy / test --workspace` 0 error；Phase 7 沙箱 Python/Node 示例技能可执行。
- 影响范围：`service-skill` + sandbox 镜像。
- 最小验证方式：起 Phase 7 测试环境跑 `python_skill:hello` + `node_skill:hello` 两类示例技能。

## Validation Evidence Required

- Record real command output or manual evidence in the selected iteration before marking this item Done.
- Run checks matching the changed surface from [Testing SOP](../../sop/TESTING.md).

## Residual Work Destination

- Update this file and `docs/backlog/PRODUCT-BACKLOG.md` if scope, status, dependencies or Required Reads change.
- Move completed or deferred execution context to `docs/backlog/archive/<period>/` during backlog compaction.

## Source Snapshot

- Source: Iteration 032 依赖审计暂缓项
- Decision context: bollard 跨 4 个 minor 的 Docker Engine API 演进（0.18 起 `Docker::connect_with_*` API 调整）；当前 `bollard 0.17.1` + Phase 7 sandbox（service-skill）零运行时问题；迁移需重写 `service-skill/src/executor.rs` + 验证 sandbox 镜像兼容性
