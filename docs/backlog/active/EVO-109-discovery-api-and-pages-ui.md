# EVO-109 Discovery API + Pages 式发现 UI

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- [Git-Centric Platform Proposal](../../proposals/GIT-CENTRIC-PLATFORM.md)
- [Product Interaction Architecture](../../design/PRODUCT-INTERACTION-ARCHITECTURE.md)
- [ADR-0008 Repo-centric Interaction Architecture](../../decisions/ADR-0008-repo-centric-interaction-architecture.md)
- 父 Epic: [EVO-100](EVO-100-git-centric-platform-foundation.md)
- 依赖: [EVO-108](EVO-108-skill-cli-mcp-indexer.md)

## Summary

- 类型：feature / api / frontend
- 优先级：P1
- 状态：Proposed
- 父 Epic: EVO-100

## Problem Or Outcome

提供跨 repo 搜索 skill / cli / mcp_tool 的 API + 在 Repo 详情页展示 "此仓包含 X skill / Y CLI / Z MCP tool" 的 Pages 式发现 UI，让 vibe coding 副产品可被发现、可消费。

## Goal And Non-Goals

- **Goal**：
  1. `GET /api/v1/skills?q=pdf&repo_id=&tags=` 跨仓搜索 `skill_index`
  2. `GET /api/v1/cli-interfaces?q=&repo_id=` 跨仓搜索 `cli_index`
  3. `GET /api/v1/mcp-tools?q=&repo_id=` 跨仓搜索 `mcp_tool_index`
  4. `GET /api/v1/repos/{id}/skills`、`/cli-interfaces`、`/mcp-tools` 列出本仓所有资源
  5. Repo 详情页：在 Repo "Files" tab 旁增加 "Resources" tab，展示上述资源 + 跳转链接
- **Non-goals**：
  - 不实现全局搜索（跨类型聚合搜索；Phase 5+）
  - 不实现全文搜索 SKILL.md body（仅元数据 + 简单 LIKE；全文搜索 Phase 5+）
  - 不实现资源详情页 UI（独立 EVO；MVP 跳转 git blob 视图）
  - 不实现资源评分 / 评论（EVO-050 已 Dropped）

## Dependencies And Blockers

- 依赖 EVO-108 indexer 表 + 解析
- 软依赖 EVO-104 Vibe Coding UI 设计风格

## Governing ADRs, Specs Or Decisions

- [ADR-0004 Git-Centric Storage](../../decisions/ADR-0004-git-centric-storage.md)
- [Design System — Figma tokens](../../reference/DESIGN.md)
- [ADR-0008 Repo-centric Interaction Architecture](../../decisions/ADR-0008-repo-centric-interaction-architecture.md)

## UX Decisions

- [x] **U-14 Repo 详情页资源展示形态** — 单一 Resources Tab，按 Skill / CLI / MCP 分组的高密度列表；移动端折叠为分组 section，不拆成三个顶级 Tab。
- [x] **U-15 跨仓搜索结果展示** — 每条结果必须显示 Repo、Ref、Path、Commit、更新时间和类型；主链接打开 source blob，Repo 名链接到 Overview。
- [x] **U-16 资源链接跳转行为** — MVP 打开 git blob 视图；不提供尚无权限/执行契约的 MCP 调用或 CLI 执行入口。

## Acceptance Criteria

- [ ] 3 个搜索 GET endpoints 实现：分页 + tenant scope + 公开 repo 可见
- [ ] 3 个 repo 内资源列表 endpoints
- [ ] 模糊匹配 name + description（LIKE %q%）
- [ ] 标签 / 关键字过滤（`?tags=ai,llm`）
- [ ] Repo 详情页 Resources tab：按类型分组的列表展示 name、description、version、Repo/Ref/Path/Commit provenance 与 blob 链接
- [ ] 所有 UI 颜色 / 字号使用 `docs/reference/DESIGN.md` token
- [ ] `cargo test --workspace` 与 `cargo clippy --workspace --all-targets -- -D warnings` 全绿
- [ ] `bun run build` 0 errors，前端 tsc 通过

## Validation Evidence Required

- E2E 测试：跨仓搜索 → 命中预期资源
- E2E 测试：Repo 详情页 → Resources tab 显示正确
- 手工截图：搜索结果页、Repo 详情页 Resources tab（桌面 + 移动）
- Performance：1000 条 index 搜索 P95 < 200ms

## Residual Work Destination

- 全文搜索（Phase 5+）
- 跨类型聚合搜索（独立 EVO）
- 资源评分 / 评论（独立 EVO）
- 资源详情页 UI（独立 EVO）

## Source Snapshot

- Source: 用户反馈 2026-06-23
- Decision context: Pages 式发现是 git-centric 平台的"分发"层
- 类比：GitHub Pages 的站点列表 / GitHub Marketplace 的 tool 列表
