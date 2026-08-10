# EVO-122-B Remove Legacy Registry Runtime

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- [Parent Epic](EVO-122-retire-prelaunch-registry-backend.md)
- [ADR-0009](../../decisions/ADR-0009-no-prelaunch-registry-compatibility.md)
- [EVO-122-A Repo-derived MCP Execution](EVO-122-A-repo-derived-mcp-execution.md)
- [EVO-111 Sandbox Removal](EVO-111-deprecate-sandbox-runtime.md)
- [API Contract](../../reference/API-CONTRACT.md)

## Summary

- 类型：refactor / tech-debt / backend
- 优先级：P1
- 状态：Proposed / paused
- 父 Epic：`EVO-122`
- 影响范围：backend、API contract、tests

## Problem Or Outcome

Repo-derived read/execute 可用后，继续注册 legacy CRUD routes、DTO、services 和 repositories 只会维持第二模型并扩大误用面。

为了让 runtime 真正只有 Git-centric 能力路径，维护者需要删除旧 Registry consumer，并以编译与路由证据证明没有 fallback。

## Goal And Non-goals

- Goal：删除 legacy `/tools*`、`/skills*`、`/snippets*` CRUD/execute routes 及仅服务它们的 handler/DTO/service/repository/domain wiring；更新 API Contract；清理 orphan tests/config。
- Non-goals：本 Story 不 drop DB tables；不删除仍被 Repo-derived parser/executor 复用的通用代码。

## Dependencies And Blockers

- EVO-122-A、EVO-111、EVO-121-F Done。
- 删除前 consumer inventory 必须区分 legacy domain 名与仍合法的 MCP protocol `tools/list`、`tools/call`。

## Acceptance Criteria

- [ ] main API router 不再注册 legacy Registry CRUD/load/execute routes。
- [ ] API Contract 只发布 Repo-derived capability read/execute。
- [ ] 编译依赖图与 `rg` 证明无 legacy repository trait/adapter runtime consumer。
- [ ] MCP protocol `tools/list` / `tools/call` 仍通过 Repo-derived contract 工作。
- [ ] 未授权、跨 tenant、Egress 和 audit tests 不退化。

## Validation Evidence Required

- 旧 route 404 与目标 route success 的集成测试。
- `cargo fmt/check/clippy/test` 全量 baseline。
- dead-code/dependency inventory 和 Navigator security review。
- `git diff --check` 与 API Contract link check。

## Residual Work Destination

- 旧表物理删除归 EVO-122-C。
