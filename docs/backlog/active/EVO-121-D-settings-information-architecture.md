# EVO-121-D Settings Information Architecture

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- [Parent Epic](EVO-121-product-experience-convergence.md)
- [Product Interaction Architecture](../../design/PRODUCT-INTERACTION-ARCHITECTURE.md)
- [Permissions](../../reference/PERMISSIONS.md)
- [API Key Authorization](../../reference/API-KEY-AUTHORIZATION.md)

## Summary

- 类型：feature / frontend
- 优先级：P1
- 状态：Done / Complete（Iteration 065 Closed / Complete）
- 父 Epic：`EVO-121`
- 影响范围：frontend（routes、settings layout、links、i18n、RBAC states）

## Problem Or Outcome

个人资料和 tenant 管理分散在 `/profile` 与 `/tenant/*`，Settings 同时占据一级导航，用户无法从信息架构理解个人设置、Workspace 设置和管理员能力的边界。

作为用户或管理员，我希望在一个二级 Settings 结构中只看到我有权管理的项目，以便完成管理任务而不干扰开发主流程。

## Goal And Non-goals

- Goal：目标路由为 `/settings/profile|workspace|members|api-keys|billing`；共享 Settings layout；用户菜单进入；权限不足显示正确 hidden/forbidden 语义。
- Non-goals：不改变后端 RBAC；不新增计费能力；不维护旧 `/tenant/*` UI 兼容体验。

## Dependencies And Blockers

- 硬依赖：EVO-121-C（Done / Complete），提供统一认证与 deep-link 入口边界。
- EVO-118-E / DEPLOY-01 是最终发布 Gate，不是本 Story 的开发前置条件；父 Epic 子项表中原有的 `EVO-118-E` 依赖已按 ADR-0010 校准。
- 权限行为必须与 PERMISSIONS/API-KEY-AUTHORIZATION 一致，不以隐藏导航替代后端授权。

## Governing ADRs, Specs Or Decisions

- [ADR-0008](../../decisions/ADR-0008-repo-centric-interaction-architecture.md)：Settings 使用 `/settings/*` 二级结构并从用户菜单进入；不建设旧 UI 路由兼容层。
- [ADR-0010](../../decisions/ADR-0010-final-production-convergence-after-product-completion.md)：EVO-118-E 是产品开发与清理后的最终发布 Gate，不阻塞本 Story。
- 角色矩阵：`profile` 所有角色；`workspace` owner/admin；`members` 所有角色可查看且写操作继续由页面/API 授权；`api-keys` owner/admin；`billing` owner。无权入口从二级导航隐藏，直接 deep link 显示 forbidden。

## Acceptance Criteria

```gherkin
Scenario: 普通成员查看 Settings
  Given 用户为 member
  When 用户从 user menu 进入 Settings
  Then 用户可访问 profile
  And 管理员专属入口按权限契约隐藏或显示 forbidden

Scenario: 管理员深链访问 API Keys
  Given 用户有管理 API Key 权限
  When 用户刷新 /settings/api-keys
  Then Settings layout 与页面保持可用
  And active state 指向 API Keys

Scenario: 普通成员直接访问管理员 Settings
  Given 用户为 member
  When 用户直接打开 /settings/api-keys
  Then 页面显示 forbidden 状态
  And 不挂载 API Key 业务页或发起管理请求
```

## Validation Evidence Required

- `bun run type-check`、`bun run build`、`bun run lint`。
- RBAC route matrix：owner/admin/member 的可见入口与直接 deep-link 结果。
- Playwright 桌面/移动 settings navigation、forbidden、loading/error。
- `rg` 检查内部链接不再指向 `/tenant/*` 或 `/profile`。

## Completion Evidence

- 纯策略测试：`bun test tests/settings-policy.test.ts`，4 pass / 0 fail；owner/admin/member 的导航与 direct-route 判定共用同一矩阵。
- 前端门禁：`bun run type-check`、`bun run build`、`bun run lint` 全部通过；locale parity 730 keys match。
- 链接/文档：旧内部 UI 链接 `rg` 无输出；Markdown 293 files 通过；`git diff --check` 通过。
- 浏览器角色矩阵：owner 5 项；admin 4 项且 Billing forbidden；member 2 项且 API Keys forbidden；member Dashboard 不显示 API Keys/Workspace/Billing。
- 状态证据：API Keys 500 显示 Error + Retry 且不同时显示 Empty，Retry 恢复；Workspace 明确只读且 `enabledControls=0`。
- 响应式/deep link：1440px 与 390px 均无横向 overflow；active state、刷新、键盘焦点通过；`/profile` 与 `/tenant/settings` 返回 SPA 404。
- 截图：[Iteration 065 evidence](../../iterations/screenshots/iter-065/)。
- Navigator check：no blocking findings after fixing false empty state, fake Workspace actions, and unauthorized Dashboard management links. Residual risk: backend authorization remains the final boundary and was not changed by this frontend Story.

## Residual Work Destination

- Settings 内各业务页未完成能力继续由原 backlog owner 管理，本 Story 只收敛 IA 与路由。
- 租户设置保存继续归 EVO-012；Stripe/计费写路径继续归 EVO-014 / Iteration 026，不在本 Story 中用假成功补齐。
