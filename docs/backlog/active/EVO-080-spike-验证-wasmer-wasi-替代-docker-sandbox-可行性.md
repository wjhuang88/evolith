# EVO-080 Spike: 验证 Wasmer/WASI 替代 Docker sandbox 可行性

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- [Serverless runtime proposal](../../proposals/SERVERLESS-RUNTIME.md)
- [Skill format](../../reference/formats/SKILL-FORMAT.md)
- [CLI interface format](../../reference/formats/CLI-INTERFACE-FORMAT.md)

## Summary

| Field | Value |
| --- | --- |
| ID | EVO-080 |
| Type | spike |
| Status | Ready |
| Priority | P1 |
| Parent Epic | None recorded |
| Source | 用户反馈 2026-06-05 / Docker 依赖反思 |

## Problem Or Outcome

评估 Wasmer/Wasmtime/WASI 能否承接 Skill/CLI/MCP 执行；输出 ADR 或提案更新，不直接替换运行时

## Goal And Non-Goals

- Goal: deliver the outcome described by the title and decision context without expanding scope during implementation.
- Non-goals: do not implement adjacent backlog items unless they are listed as hard dependencies in this file.

## Dependencies And Blockers

- No hard blocker recorded in the source backlog.

## Governing ADRs, Specs Or Decisions

- [Serverless runtime proposal](../../proposals/SERVERLESS-RUNTIME.md)
- [Skill format](../../reference/formats/SKILL-FORMAT.md)
- [CLI interface format](../../reference/formats/CLI-INTERFACE-FORMAT.md)

## Acceptance Criteria

- 类型：spike
- 优先级：P1
- 状态：Ready
- 父 Epic：EVO-045 / EVO-047 执行基础设施方向参考
- Story 形态：Spike
- 问题：
  - Wasmer / WASI 是否能让 Evolith 的 Skill、CLI 和 MCP tool 执行摆脱 Docker daemon 依赖？
  - 如果可行，现有 Python/Node sandbox、ExecutionProvider、权限模型、文件/网络隔离和执行协议需要怎样改造？
- 时间盒：1 个微迭代。
- 候选方案：
  - Wasmer / WASIX 嵌入 Rust 后端，作为 `ExecutionProvider` 的新实现。
  - Wasmtime / WASI 作为对照方案。
  - 保留 Docker provider，仅把 lite/local 默认关闭作为当前稳定路径。
- 输出：
  - 更新 `docs/proposals/SERVERLESS-RUNTIME.md` 或新增 ADR，给出采用、暂缓或放弃 Wasmer 的结论。
  - 明确 PoC 需要执行的最小 guest 程序、权限边界、依赖打包方式和性能/安全验证。
  - 如可行，拆出一个不超过 0.5-2 天的实现 Story；如不可行，记录阻断条件。
- 不做：
  - 不直接替换 Docker provider。
  - 不改生产执行路径。
  - 不承诺 Python/Node 代码无需编译即可在 Wasm 中运行。
- 验收标准：
  - [ ] 明确 Wasmer 是否能覆盖当前执行需求；不能覆盖时列出具体缺口。
  - [ ] 至少比较 Wasmer、Wasmtime 和现有 Docker provider 的隔离边界、冷启动、语言支持和运维依赖。
  - [ ] 输出 ADR/提案更新和下一步 backlog 归口。
- 依赖或阻塞：EVO-045-A 已完成 ExecutionProvider 基线；需要查阅 Wasmer/Wasmtime 当前官方能力。
- 解锁内容：决定是否建立非 Docker 本地执行路线。
- 影响范围：docs / backend execution architecture。
- 最小验证方式：官方文档调研；必要时最小 Wasm/WASI PoC；`git diff --check`。

## Validation Evidence Required

- Record real command output or manual evidence in the selected iteration before marking this item Done.
- Run checks matching the changed surface from [Testing SOP](../../sop/TESTING.md).

## Residual Work Destination

- Update this file and `docs/backlog/PRODUCT-BACKLOG.md` if scope, status, dependencies or Required Reads change.
- Move completed or deferred execution context to `docs/backlog/archive/<period>/` during backlog compaction.

## Source Snapshot

- Source: 用户反馈 2026-06-05 / Docker 依赖反思
- Decision context: 评估 Wasmer/Wasmtime/WASI 能否承接 Skill/CLI/MCP 执行；输出 ADR 或提案更新，不直接替换运行时
