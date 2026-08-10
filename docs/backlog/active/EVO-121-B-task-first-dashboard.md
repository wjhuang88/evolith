# EVO-121-B Task-first Dashboard

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- [Parent Epic](EVO-121-product-experience-convergence.md)
- [Product Interaction Architecture](../../design/PRODUCT-INTERACTION-ARCHITECTURE.md)
- [EVO-121-E Activity](EVO-121-E-auditable-activity-timeline.md)
- [EVO-112-B Repo Detail](EVO-112-B-repo-detail-read-only.md)
- [EVO-112-C Commit Evidence](EVO-112-C-repo-commit-evidence-detail.md)

## Summary

- 类型：feature / frontend
- 优先级：P0
- 状态：Proposed / paused
- 父 Epic：`EVO-121`
- 影响范围：frontend、必要的只读 API contract/client

## Problem Or Outcome

当前 Dashboard 用 Repo、成员和月度统计卡片填充首屏，并把 Team Members / API Keys 作为快捷操作。它回答“系统里有多少东西”，没有回答“我现在应该继续什么”。

作为开发者，我希望首屏展示 Active work、Review queue、Recent repos 和 Repo health，以便一次点击恢复会话、处理待审查结果或进入异常 Repo。

## Goal And Non-goals

- Goal：四个任务区按优先级组织；每条记录链接到真实 Repo/Session/Commit/Review；无 Repo 时进入 first-run CTA；管理项不占首屏。
- Non-goals：不在 Dashboard 执行成员、计费或 Key 管理；不伪造 commit 数、review 或 health。

## Dependencies And Blockers

- EVO-112-B/C 提供 Repo 与 Commit deep link。
- EVO-105/106 提供 Commit/Session 状态；EVO-121-E 提供统一 Activity read model。
- 若后端缺少聚合查询，本 Story 可新增最小只读 endpoint 与 API Contract，但不得用多个 `.catch(() => [])` 把系统错误伪装为空状态。

## Acceptance Criteria

```gherkin
Scenario: 用户恢复进行中的 Agent 工作
  Given 用户有一个 active session
  When 用户进入 Dashboard
  Then Active work 显示 Repo、branch、last event 和状态
  And 点击记录进入对应 /repos/{id}/workspace session

Scenario: 用户处理待审查结果
  Given Policy 产生 require_review 结果
  When 用户进入 Dashboard
  Then Review queue 显示 Repo、Commit/diff 摘要和等待时间
  And 点击记录进入真实 review/promote 上下文

Scenario: Dashboard 数据失败
  Given 聚合 API 返回失败
  When 页面加载
  Then 页面显示可重试错误且保留已成功的独立 section
  And 不把失败 section 显示为 0 或 empty
```

## Validation Evidence Required

- API contract/client 对齐测试（如新增 endpoint）。
- `bun run type-check`、`bun run build`、`bun run lint`。
- Playwright：有 active work、有 review、全空、部分失败、forbidden、移动布局。
- 负向断言：接口失败不渲染为 `0` 或 empty。

## Residual Work Destination

- 更复杂的跨 Repo analytics 或趋势图独立评估，不进入 MVP Dashboard。
