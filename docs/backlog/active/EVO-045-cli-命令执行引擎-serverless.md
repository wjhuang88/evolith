# EVO-045 CLI 命令执行引擎（Serverless）

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- [Serverless runtime proposal](../../proposals/SERVERLESS-RUNTIME.md)

## Summary

| Field | Value |
| --- | --- |
| ID | EVO-045 |
| Type | feature |
| Status | Proposed |
| Priority | P0 |
| Parent Epic | None recorded |
| Source | 用户反馈 2026-05-29 |

## Problem Or Outcome

CLI 从纯文本记录升级为可执行命令接口，支持 serverless 执行环境或外部执行信息记录

## Goal And Non-Goals

- Goal: deliver the outcome described by the title and decision context without expanding scope during implementation.
- Non-goals: do not implement adjacent backlog items unless they are listed as hard dependencies in this file.

## Dependencies And Blockers

- No hard blocker recorded in the source backlog.

## Governing ADRs, Specs Or Decisions

- [Serverless runtime proposal](../../proposals/SERVERLESS-RUNTIME.md)

## Acceptance Criteria

- 类型：feature
- 优先级：P0
- 状态：Proposed
- 用户价值或技术目标：将 CLI 从纯代码文本记录升级为可执行的命令接口，让用户在 Agent 工作空间中发现、调用和管理 CLI 命令，获得结构化输入输出和错误语义。
- 核心设计：
  1. **结构化命令定义**：command、subcommands、参数 schema（类型、必填、默认值、枚举）、output schema、error model
  2. **Serverless 执行引擎**：
     - 本地版（近期）：复用 Phase 7 Docker sandbox 基础设施，按需启动容器执行命令，支持冷启动优化
     - Vercel 完整模式（远期）：函数部署、自动扩缩、按调用计费
  3. **外部执行支持**：CLI 也可仅记录外部执行 URL（如用户自建的 serverless 函数），平台做代理调用
  4. **Agent 集成**：CLI 命令可被 MCP tool 系统发现和调用，或通过独立 CLI endpoint 暴露
- 验收标准：
  - [ ] CLI 数据模型支持结构化命令定义（command、parameters、output schema、error model）
  - [ ] 后端提供 CLI 执行 endpoint（输入命令+参数 → 结构化结果 stdout/stderr/exit code）
  - [ ] 执行环境有资源限制（CPU/内存/超时）和安全隔离
  - [ ] 支持两种模式：平台 serverless 执行 + 外部执行 URL 代理
  - [ ] Agent 可通过 MCP tool 或 CLI endpoint 发现和调用已注册的 CLI 命令
  - [ ] 前端创建/编辑页支持结构化命令定义（不只是代码编辑器）
- 依赖或阻塞：EVO-048 Serverless 架构设计 Spike 完成；EVO-049 Phase 2（CLI 数据模型结构化字段就绪后执行引擎才能正确路由命令）
- 影响范围：backend（domain、service、API）/ frontend / db migration / docs
- 最小验证方式：后端集成测试覆盖命令注册+执行+调用全链路；前端创建页手工验证
- 不做：不实现本地 CLI 客户端（Rust CLI 另建 story）；不实现命令版本管理（EVO-028 范围）

## Validation Evidence Required

- Record real command output or manual evidence in the selected iteration before marking this item Done.
- Run checks matching the changed surface from [Testing SOP](../../sop/TESTING.md).

## Residual Work Destination

- Update this file and `docs/backlog/PRODUCT-BACKLOG.md` if scope, status, dependencies or Required Reads change.
- Move completed or deferred execution context to `docs/backlog/archive/<period>/` during backlog compaction.

## Source Snapshot

- Source: 用户反馈 2026-05-29
- Decision context: CLI 从纯文本记录升级为可执行命令接口，支持 serverless 执行环境或外部执行信息记录
