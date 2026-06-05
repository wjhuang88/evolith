# EVO-047 MCP 工具 Serverless 执行

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- [Serverless runtime proposal](../../proposals/SERVERLESS-RUNTIME.md)

## Summary

| Field | Value |
| --- | --- |
| ID | EVO-047 |
| Type | feature |
| Status | Proposed |
| Priority | P1 |
| Parent Epic | None recorded |
| Source | 用户反馈 2026-05-29 |

## Problem Or Outcome

MCP 工具支持 serverless 执行环境或外部执行信息记录，与 CLI 共享执行基础设施

## Goal And Non-Goals

- Goal: deliver the outcome described by the title and decision context without expanding scope during implementation.
- Non-goals: do not implement adjacent backlog items unless they are listed as hard dependencies in this file.

## Dependencies And Blockers

- No hard blocker recorded in the source backlog.

## Governing ADRs, Specs Or Decisions

- [Serverless runtime proposal](../../proposals/SERVERLESS-RUNTIME.md)

## Acceptance Criteria

- 类型：feature
- 优先级：P1
- 状态：Proposed
- 用户价值或技术目标：MCP 工具支持 serverless 执行环境（与 CLI 共享基础设施），或仅记录外部执行 URL 由平台代理调用。
- 核心设计：
  1. **Serverless 执行**：MCP 工具处理函数部署到 serverless runtime，按需调用
  2. **外部执行代理**：MCP 工具记录外部 HTTP endpoint，平台做代理调用（当前 HTTP executor 增强）
  3. **与 CLI 共享基础设施**：复用 EVO-045/048 的 serverless runtime
- 验收标准：
  - [ ] MCP 工具支持 serverless 执行模式（工具处理函数部署到 runtime）
  - [ ] MCP 工具支持外部执行模式（记录 URL，代理调用）
  - [ ] 执行环境有资源限制和安全隔离
  - [ ] 与 CLI 共享 serverless runtime 基础设施
- 依赖或阻塞：EVO-045（CLI serverless）+ EVO-048（架构设计）
- 影响范围：backend（service-tool 增强）/ db migration / docs
- 最小验证方式：MCP tool 注册 + serverless 执行 + 调用全链路测试
- 不做：不实现 MCP 协议完整服务端（当前只做 tool 执行层）

## Validation Evidence Required

- Record real command output or manual evidence in the selected iteration before marking this item Done.
- Run checks matching the changed surface from [Testing SOP](../../sop/TESTING.md).

## Residual Work Destination

- Update this file and `docs/backlog/PRODUCT-BACKLOG.md` if scope, status, dependencies or Required Reads change.
- Move completed or deferred execution context to `docs/backlog/archive/<period>/` during backlog compaction.

## Source Snapshot

- Source: 用户反馈 2026-05-29
- Decision context: MCP 工具支持 serverless 执行环境或外部执行信息记录，与 CLI 共享执行基础设施
