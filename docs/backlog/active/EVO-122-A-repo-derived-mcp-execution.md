# EVO-122-A Repo-derived MCP Execution

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- [Parent Epic](EVO-122-retire-prelaunch-registry-backend.md)
- [ADR-0009](../../decisions/ADR-0009-no-prelaunch-registry-compatibility.md)
- [API Contract](../../reference/API-CONTRACT.md)
- [Permissions](../../reference/PERMISSIONS.md)
- [Security Review](../../sop/SECURITY-REVIEW.md)

## Summary

- 类型：feature / api / security
- 优先级：P0
- 状态：Proposed / paused
- 父 Epic：`EVO-122`
- 影响范围：backend、API contract、tests

## Problem Or Outcome

MCP execute 当前可依赖旧 `tools` 行。删除旧 Registry 前，执行身份必须收敛为 Git provenance，并保持已经关闭的 Typed Capability、tenant isolation 和 Egress/SSRF 门禁。

作为获授权的 MCP client 或 Agent，我希望通过 Repo、Ref/Commit 和 manifest Path 调用能力，以便执行结果能追溯到真实代码版本，而不是可漂移的旧 DB 记录。

## Goal And Non-goals

- Goal：Contract-first 定义 Repo-derived execute identity；从 immutable commit/path 读取并校验 manifest；复用受控 executor/Egress；记录 Repo/Commit/Path/caller audit。
- Non-goals：不接受旧 tool UUID fallback；不迁移旧行；不放宽 API Key/Agent Token capability。

## Dependencies And Blockers

- EVO-108/109 提供 index 与 capability metadata。
- EVO-118-B/C 的授权与 Egress 契约不可回退。
- 跨层安全变更强制 Driver/Navigator。

## Acceptance Criteria

```gherkin
Scenario: 授权调用者执行固定 Commit 的 MCP Tool
  Given caller 对 tenant Repo 有 read 和 execute capability
  And manifest path 在指定 Commit 中解析为允许的 MCP Tool
  When caller 提交符合 schema 的 input
  Then executor 使用该 Commit 的 manifest 执行
  And audit 记录 tenant/repo/commit/path/caller/result

Scenario: caller 尝试跨 Repo 或 Path scope
  Given token 只允许 Repo A 或指定 path
  When caller 请求 Repo B 或 scope 外 manifest
  Then 请求被拒绝且不泄露目标存在性

Scenario: Ref 在解析后发生变化
  Given caller 使用 branch Ref 发起请求
  When server 解析执行身份
  Then server 固定为解析出的 Commit 并在响应/审计返回该 Commit
  And 执行过程不跟随后续 Ref 移动
```

## Validation Evidence Required

- API Contract first；handler/repository/executor tests。
- API Key/Agent Token/role/tenant/Repo/Ref/Path 负向矩阵。
- SSRF/Egress regression suite 与审计字段断言。
- `cargo fmt/check/clippy/test` baseline；Navigator security conclusion。

## Residual Work Destination

- 非 HTTP/FaaS capability execution 按独立 ADR/Story 进入，不复活 Sandbox 或 legacy row identity。
