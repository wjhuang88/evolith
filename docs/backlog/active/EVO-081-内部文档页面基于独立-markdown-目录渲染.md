# EVO-081 内部文档页面基于独立 Markdown 目录渲染

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- [Product requirements](../../reference/product/REQUIREMENTS.md)

## Summary

| Field | Value |
| --- | --- |
| ID | EVO-081 |
| Type | feature |
| Status | Ready |
| Priority | P1 |
| Parent Epic | None recorded |
| Source | 用户反馈 2026-06-05 |

## Problem Or Outcome

新增内部文档页面，文档源放独立 md 目录，前端根据目录渲染文档列表和详情

## Goal And Non-Goals

- Goal: deliver the outcome described by the title and decision context without expanding scope during implementation.
- Non-goals: do not implement adjacent backlog items unless they are listed as hard dependencies in this file.

## Dependencies And Blockers

- No hard blocker recorded in the source backlog.

## Governing ADRs, Specs Or Decisions

- [Product requirements](../../reference/product/REQUIREMENTS.md)

## Acceptance Criteria

- 类型：feature
- 优先级：P1
- 状态：Ready
- 父 Epic：无
- Story 形态：Product / User Story
- 用户故事：
  - 作为 Evolith 平台用户，
  - 我希望在应用内查看内部文档页面，
  - 以便不用离开平台即可浏览团队维护的 Markdown 文档。
- 范围：
  - 新增一个独立文档源目录，专门存放内部文档 Markdown。
  - 新增前端内部文档页面，从该目录生成/读取文档列表并渲染 Markdown 详情。
  - 支持至少标题、正文、基础 Markdown 排版和空状态。
  - 在导航或路由中接入文档页面。
- 不做：
  - 不实现在线编辑、权限分级、版本历史或全文搜索。
  - 不混用 `docs/` 治理文档作为应用内文档源。
  - 不新增后端数据库模型，除非实现时证明静态/构建时加载不可行。
- 验收场景：
  - Given 独立文档目录中存在至少一篇 Markdown 文档
    When 用户打开内部文档页面
    Then 页面展示文档列表，并能打开对应详情查看渲染后的内容
  - Given 独立文档目录为空
    When 用户打开内部文档页面
    Then 页面展示空状态，不报错
  - Given Markdown 包含标题、列表和代码块
    When 用户查看详情
    Then 基础格式正确渲染且前端构建通过
- 依赖或阻塞：需在实现前确认文档源目录命名和路由命名；默认可采用 `frontend/content/docs/` 与 `/docs` 或 `/internal-docs`。
- 解锁内容：后续可扩展内部知识库、搜索和权限。
- 影响范围：frontend / docs。
- 最小验证方式：`bun run build`；必要时 Playwright 打开文档页面验证列表和详情。

## Validation Evidence Required

- Record real command output or manual evidence in the selected iteration before marking this item Done.
- Run checks matching the changed surface from [Testing SOP](../../sop/TESTING.md).

## Residual Work Destination

- Update this file and `docs/backlog/PRODUCT-BACKLOG.md` if scope, status, dependencies or Required Reads change.
- Move completed or deferred execution context to `docs/backlog/archive/<period>/` during backlog compaction.

## Source Snapshot

- Source: 用户反馈 2026-06-05
- Decision context: 新增内部文档页面，文档源放独立 md 目录，前端根据目录渲染文档列表和详情
