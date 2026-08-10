# EVO-120 First-run Repo Onboarding

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- [Product Interaction Architecture](../../design/PRODUCT-INTERACTION-ARCHITECTURE.md)
- [ADR-0008 Repo-centric Interaction Architecture](../../decisions/ADR-0008-repo-centric-interaction-architecture.md)
- [EVO-112-A Repo UI Shell](EVO-112-A-repo-ui-shell.md)
- [EVO-118 Production Readiness](EVO-118-production-readiness-and-security-hardening.md)
- [Local Development SOP](../../sop/LOCAL-DEV.md)
- [Testing SOP](../../sop/TESTING.md)

## Summary

| Field | Value |
| --- | --- |
| ID | EVO-120 |
| Type | feature / frontend |
| Status | Done / Complete（Iteration 062 Closed） |
| Priority | P0 |
| Parent Epic | None recorded |
| Source | 2026-08-08 interaction architecture review |

## Problem Or Outcome

当前 onboarding 仍以创建旧 Registry Tool 为目标，与 Git-centric 产品方向冲突。新用户完成登录后没有一条真实、连续的路径到 Repo、代码上下文和 Workspace。

目标是让首次使用直接建立 Repo 上下文，并将成功落点统一为 `/repos/:id/overview`。

## Goal And Non-goals

### Goal

1. 登录后检测当前 tenant 是否已有 Repo。
2. 无 Repo 时展示创建 Repo，以及后端真实支持时的 import Repo 入口。
3. 创建成功后进入 `/repos/:id/overview`，始终提供 **Browse code**；EVO-104 可用后再显示 **Start Workspace**。
4. 已有 Repo 的用户跳过 onboarding，进入现有 `/dashboard`；Dashboard 的任务型内容重构仍归 EVO-121-B。
5. 从主 onboarding 中移除 Tool 创建和旧 Registry 产品文案。
6. API 失败时保留输入并显示可重试错误，不跳转、不显示假成功。

### Non-goals

- 不在没有后端能力时伪造 remote import。
- 不实现 Repo Detail、文件浏览或 Workspace 本体，分别归 EVO-112-B 和 EVO-104。
- 不重做完整 public landing page。
- 不删除 legacy route 或数据库兼容层。

## Dependencies And Blockers

- `EVO-112-A` Done：复用 Repo API、列表、创建和主导航基础。
- `EVO-103-A` Done：Repo 创建后端能力可用。
- `EVO-112-B` 提供目标 Overview 路由；两者可在同一产品批次编排，但不可留下无效跳转。
- 启动处置：EVO-112-B、EVO-118-F 与 G-B/C/D 已完成；H 的 Worker/业务接入 residual 不参与首次 Repo 查询/创建路径，G-A 的远端 Branch Protection residual 也不阻塞本地产品开发。Iteration 062 因此激活本 Story；EVO-118-E / DEPLOY-01 仍只门禁最终发布。

## Acceptance Criteria

```gherkin
Scenario: 新用户创建第一个 Repo
  Given 已登录用户所在 tenant 没有 Repo
  When 用户完成有效的 Repo 创建表单
  Then 系统创建真实 Repo
  And 浏览器进入 /repos/{id}/overview
  And 页面提供 Browse code
  And 仅当 Workspace 已真实可用时显示 Start Workspace

Scenario: 已有 Repo 的用户登录
  Given 用户所在 tenant 已有至少一个 Repo
  When 用户完成登录
  Then 系统跳过 first-run onboarding
  And 用户进入 /dashboard
  And Dashboard 的任务型内容由 EVO-121-B 独立交付

Scenario: 登录恢复受保护 deep link
  Given 未登录用户访问受保护页面后进入带 redirect 参数的登录页
  When 用户成功登录
  Then 浏览器优先恢复站内 redirect 目标
  And onboarding 的 Repo 判定不覆盖该目标

Scenario: Repo 创建失败
  Given 用户提交 Repo 创建表单
  When API 返回失败
  Then 页面保留用户输入并展示可操作错误
  And 不跳转到 Repo Overview
  And 不显示成功状态
```

### Technical Acceptance

- [x] first-run 判定基于真实 Repo 查询结果，并区分 loading / empty / error / forbidden。
- [x] `/onboarding` 不再创建 Tool 或调用 legacy Tool API。
- [x] `/onboarding` 受认证保护；已有 Repo 时 replace 到 `/dashboard`。
- [x] 普通登录无 redirect 时按 Repo 数量进入 `/onboarding` 或 `/dashboard`；安全的站内 redirect 优先。
- [x] create success 与 `/repos/:id/overview` 路由形成真实闭环。
- [x] 不渲染指向尚未注册 Workspace 路由的操作。
- [x] import 入口只有在 API Contract 与后端实现存在时才显示。
- [x] en / zh-CN 文案同步，主流程不出现旧 Registry 定位。
- [x] 键盘导航、焦点顺序、错误关联和移动布局通过检查。
- [x] `bun run type-check`、`bun run build` 通过。
- [x] 浏览器自动化覆盖桌面/移动的新用户、已有 Repo、创建失败和站内 redirect 流程，并归档截图。

## Completion Record

- 2026-08-09：Iteration 062 Closed / Complete；首次 Repo 创建显式使用 `seed_template: true`，真实 Initial Commit、默认分支、README 与 Overview/Browse code 闭环成立。
- 浏览器证据覆盖未认证保护、无 Repo 登录分流、已有 Repo 跳过 onboarding、站内 deep link、外部 redirect 拒绝、500/403/Retry、重复名失败保留输入、桌面及 390px 移动布局与键盘焦点顺序。
- Navigator 复核无未关闭 finding；完整 public/join/verification resolver、任务型 Dashboard、Workspace、Resources 与 legacy 删除仍按 Residual Work Destination 归口。

## Validation Evidence Required

1. type-check 与 production frontend build 输出。
2. Headless Chrome/Playwright 桌面与移动截图及流程断言。
3. 创建失败负向测试，证明没有假成功和错误跳转。
4. 从 login 到 Repo Overview 的完整手工 smoke 记录。
5. markdown link check 与 `git diff --check`。

## Residual Work Destination

- Repo Overview / Files / Commits / Settings 已由 EVO-112-B 完成；Resources 仍归 EVO-109。
- Agent Workspace：EVO-104。
- Task-first Dashboard：EVO-121-B；最终主导航：EVO-121-A。
- Public landing 与统一 entry resolver：EVO-121-C。
- Legacy UI route 删除：EVO-121-F。

## DoR And Activation Record

- Story 形态：Product / User Story；用户、目标、价值与 Given/When/Then 已具备。
- 单一可验收结果：首次登录到真实 Repo Overview 的创建闭环，以及已有 Repo 的跳过分流；预计 0.5-2 天。
- 影响范围：frontend、i18n、docs；不修改 backend/db/deploy。
- 重大取舍：无新增 ADR；沿用 ADR-0008，entry resolver 的全入口收敛仍归 EVO-121-C。
- 状态同步归口：EVO-120、Product Backlog、Iteration 062、Iteration index、Board、AGENTS/Readiness current execution。
- 残余归口：EVO-121-B/C/F、EVO-104、EVO-109。
