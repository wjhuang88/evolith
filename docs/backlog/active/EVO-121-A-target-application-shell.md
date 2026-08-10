# EVO-121-A Target Application Shell

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- [Parent Epic](EVO-121-product-experience-convergence.md)
- [Product Interaction Architecture](../../design/PRODUCT-INTERACTION-ARCHITECTURE.md)
- [ADR-0008](../../decisions/ADR-0008-repo-centric-interaction-architecture.md)
- [Design System](../../reference/DESIGN.md)

## Summary

- 类型：feature / frontend
- 优先级：P0
- 状态：Proposed / paused
- 父 Epic：`EVO-121`
- 影响范围：frontend（layout、router、i18n、navigation tests）

## Problem Or Outcome

当前 Shell 的一级导航顺序是 Repos / Dashboard / Settings，并包含 Legacy disclosure。目标 Shell 应只承载开发主线，且不能在目标页面尚未可用时制造死链。

作为日常使用 Evolith 的开发者，我希望在桌面和移动端通过一致导航进入当前工作、Repo、Discover 和 Activity，以便快速恢复任务而不被管理项或旧 Registry 概念干扰。

## Goal And Non-goals

- Goal：一级导航固定为 Dashboard / Repositories / Discover / Activity；Settings 从用户菜单进入；桌面 sidebar 与移动 drawer 保持同一顺序和 active state；Repo 内二级导航由 Repo layout 管理。
- Non-goals：不实现各目标页面内容；不保留 Legacy disclosure；不新增全局 Workspace。

## Dependencies And Blockers

- 依赖 EVO-109 的 `/discover`、EVO-121-D 的 `/settings/*`、EVO-121-E 的 `/activity` 已真实注册。
- 未满足依赖前保持 Proposed，不用 disabled nav 或空页面规避死链。

## Acceptance Criteria

```gherkin
Scenario: 桌面用户通过主导航切换任务
  Given 用户已登录且目标页面均可用
  When 用户依次选择 Dashboard, Repositories, Discover, Activity
  Then 每个入口进入对应真实路由
  And 当前入口有唯一 active state
  And 导航中没有 Settings, Tools, Skills, Interfaces 或全局 Workspace

Scenario: 移动端导航保持同一结构
  Given 视口为移动宽度
  When 用户打开 navigation drawer 并选择入口
  Then 顺序与桌面一致
  And 选择后 drawer 关闭且焦点进入页面标题
```

## Validation Evidence Required

- `bun run type-check`、`bun run build`、`bun run lint`。
- 路由测试证明每个 nav target 已注册，且 active matching 不误命中相邻路径。
- Playwright 桌面 sidebar、移动 drawer、键盘导航与 deep-link refresh。
- `rg -n "legacyNavigation|nav\.legacy" frontend/src` 无用户可见主导航命中。

## Residual Work Destination

- 页面业务内容归依赖 Story；Repo 内导航归 EVO-112-B/EVO-104/EVO-109。
