# EVO-121-F Retire Legacy Registry UI

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- [Parent Epic](EVO-121-product-experience-convergence.md)
- [ADR-0008](../../decisions/ADR-0008-repo-centric-interaction-architecture.md)
- [EVO-109 Discovery](EVO-109-discovery-api-and-pages-ui.md)
- [ADR-0009 No Pre-launch Registry Compatibility](../../decisions/ADR-0009-no-prelaunch-registry-compatibility.md)
- [EVO-122 Backend Retirement](EVO-122-retire-prelaunch-registry-backend.md)

## Summary

- 类型：chore / frontend
- 优先级：P1
- 状态：Proposed / paused
- 父 Epic：`EVO-121`
- 影响范围：frontend（routes、pages、API clients if orphaned、i18n、tests）

## Problem Or Outcome

旧 Tools/Skills/Interfaces CRUD 页面和 Legacy disclosure 会继续向开发者暗示 Registry 是一级产品。项目未上线，没有必要投入 UI 兼容或迁移体验。

为了让实际代码与 ADR-0008 一致，维护者需要在 Discover/Resources 可用后删除旧 Registry UI surface，同时保留后端兼容工作的独立治理边界。

## Goal And Non-goals

- Goal：删除 `/tools*`、`/skills*`、`/interfaces*`、`/snippets*` UI 路由和页面；删除 Legacy navigation 与只服务这些页面的 i18n/client/component；修复所有内部链接。
- Non-goals：不删除 backend API、DB table 或 index compatibility；不建设 redirect、deprecated page 或数据迁移 wizard。

## Dependencies And Blockers

- EVO-109 提供最终用户的 Repo Resources / Discover 入口。
- EVO-121-A 提供不含 Legacy 的目标 Shell。
- 删除前用引用搜索区分 orphan 与仍被 Repo-derived capability 使用的共享代码。

## Acceptance Criteria

```gherkin
Scenario: 用户使用目标产品导航
  Given Repo Resources 和 Discover 已可用
  When 用户浏览所有主导航与页面操作
  Then 没有入口指向旧 Registry CRUD 页面
  And capability 入口显示 Repo provenance

Scenario: 旧 UI 路径不再构成产品合约
  Given 产品尚未公开上线
  When 用户访问旧 Tools/Skills/Interfaces 路径
  Then 路由按普通 not-found 处理
  And 应用不维护专用兼容或迁移页面
```

## Validation Evidence Required

- `rg` 证明 main router、Header、landing、onboarding 和 i18n 无旧 CRUD 用户文案；历史/类型/API 名称命中需分类。
- `bun run type-check`、`bun run build`、`bun run lint`。
- 路由测试证明目标 routes 可用、旧 routes 不再注册。
- Playwright 冒烟覆盖 Dashboard -> Repo -> Resources/Discover，不经过 legacy UI。

## Residual Work Destination

- Backend Repo-derived 承接与旧表/API 删除归 EVO-122；Sandbox removal 保持 EVO-111，不随 UI 删除扩大范围。
