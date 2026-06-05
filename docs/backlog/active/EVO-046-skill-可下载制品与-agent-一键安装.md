# EVO-046 Skill 可下载制品与 Agent 一键安装

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)

## Summary

| Field | Value |
| --- | --- |
| ID | EVO-046 |
| Type | product-change |
| Status | Proposed |
| Priority | P1 |
| Parent Epic | None recorded |
| Source | 用户反馈 2026-05-29 |

## Problem Or Outcome

Skill 从服务端执行改为可下载制品（ClawHub 模式），支持搜索、下载和一键安装到 Agent 工作空间

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
- 用户价值或技术目标：Skill 从服务端 Docker sandbox 执行改为可下载制品（参考 ClawHub 模式），用户可搜索、下载 Skill 并一键安装到自己的 Agent 工作空间中使用。
- 核心设计：
  1. **制品市场**：Skill 作为可搜索、可下载的制品包（SKILL.md + 代码 + 资源文件），类似 npm package
  2. **一键安装**：用户选择 Skill → 安装到自己的 Agent 工作空间 → Agent 可在本地使用该 Skill
  3. **版本与来源追踪**：记录安装来源、版本、安装时间，支持更新检查
  4. **Docker sandbox 迁移**：Phase 7 的 sandbox 执行器从 Skill 服务中移除，挪给 CLI/MCP 使用
- 验收标准：
  - [ ] Skill 列表页支持搜索、筛选、预览制品详情
  - [ ] 提供 Skill 下载 API（打包为 ZIP 或 tar.gz）
  - [ ] 一键安装到 Agent 工作空间（记录安装关系，Agent 可引用已安装 Skill）
  - [ ] 已安装 Skill 列表管理（查看、更新、卸载）
  - [ ] Skill 服务移除 Docker sandbox 执行逻辑，sandbox 代码迁移到 CLI/MCP 服务
- 依赖或阻塞：EVO-049 Phase 3（Skill 制品打包 API 就绪后才能实现下载和安装）
- 影响范围：backend（service-skill 重构、新增安装服务）/ frontend / db migration / docs
- 最小验证方式：下载 API 返回有效制品包；安装后 Agent 工作空间可见已安装 Skill
- 不做：不实现 Skill 评分/评论（远期市场功能）；不实现 Skill 自动执行

## Validation Evidence Required

- Record real command output or manual evidence in the selected iteration before marking this item Done.
- Run checks matching the changed surface from [Testing SOP](../../sop/TESTING.md).

## Residual Work Destination

- Update this file and `docs/backlog/PRODUCT-BACKLOG.md` if scope, status, dependencies or Required Reads change.
- Move completed or deferred execution context to `docs/backlog/archive/<period>/` during backlog compaction.

## Source Snapshot

- Source: 用户反馈 2026-05-29
- Decision context: Skill 从服务端执行改为可下载制品（ClawHub 模式），支持搜索、下载和一键安装到 Agent 工作空间
