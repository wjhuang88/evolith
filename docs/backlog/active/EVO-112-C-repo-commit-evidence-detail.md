# EVO-112-C Repo Commit Evidence Detail

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- [Parent Story](EVO-112-repo-management-ui.md)
- [Product Interaction Architecture](../../design/PRODUCT-INTERACTION-ARCHITECTURE.md)
- [ADR-0008](../../decisions/ADR-0008-repo-centric-interaction-architecture.md)
- [EVO-103-C Repo Context API](EVO-103-C-repo-context-api.md)
- [EVO-105 Commit And Promote](EVO-105-commit-and-promote-api.md)
- [API Contract](../../reference/API-CONTRACT.md)
- [Security Review](../../sop/SECURITY-REVIEW.md)

## Summary

- 类型：feature / api / frontend
- 优先级：P0
- 状态：Done / Complete（Iteration 063 Closed / Complete）
- 父 Epic：`EVO-112`
- 影响范围：backend、frontend、API contract、tests

## Problem Or Outcome

Evolith 将 Commit 定义为可信结果边界，但现有计划只提供 commit 列表。Workspace、Dashboard 和 Activity 如果只能链接回列表，用户无法证明某次 Agent 操作最终生成了哪个 Git 对象、改了什么以及位于哪个 Ref。

作为开发者或审查者，我希望通过稳定 commit deep link 查看完整 Git 证据，以便从任何产品流程回到不可歧义的结果。

## Goal And Non-goals

- Goal：`/repos/:id/commits/:sha` 展示 full SHA、message、author/committer、time、parents、经校验的查询 Ref 与 structured changed-file diff；为未来 Workspace/Activity 提供稳定 deep link。
- Non-goals：不实现文本 patch/diff viewer、任意历史 rewrite、cherry-pick、revert 或通用 code review discussion；文本 diff viewer 仍归 EVO-104。

## Dependencies And Blockers

- 依赖 EVO-112-B 的 Repo layout 与 Commits 列表。
- 复用 EVO-103-C；若当前 API 无 commit detail/diff，则先按 CONTRACT-FIRST 增加最小 tenant-scoped read endpoint。
- Agent result 关联由 EVO-105/EVO-104 提供，但不是本 Story 的硬依赖；本 Story 交付可由普通 Git commit list 访问的稳定目标页，未来入口只复用该 URL。

## Acceptance Criteria

```gherkin
Scenario: 用户从 Commit 列表查看证据
  Given Repo 中存在可访问 commit
  When 用户点击 commit SHA
  Then 页面进入 /repos/{id}/commits/{sha}
  And 显示 full SHA、message、author/committer time、parents 和 structured changed-file diff
  And 明确显示已由 API 验证可到达该 commit 的查询 Ref

Scenario: Commit 不属于当前 tenant Repo
  Given SHA 只存在于无权访问的 Repo
  When 用户请求 commit detail
  Then API 不泄露对象存在性或元数据
```

## Validation Evidence Required

- API Contract、repository/handler 集成测试、tenant/RBAC 负向测试。
- SHA/Ref 校验、root commit、diff entry 上限与错误映射测试。
- Backend baseline 与 frontend type-check/build/lint。
- 浏览器自动化：commit list -> detail、not-found/error、移动 changed-file diff。

## Closure Record

- Contract/API：新增 `GET /tenant/{tenant_id}/repos/{repo_id}/commits/{sha}?ref={ref}`；精确 SHA、tenant/API Key、Ref reachability、root commit、structured diff 上限与 timeout/error mapping 已实现并测试。
- Product：Commit list SHA 可键盘访问并 deep-link；详情页展示 full SHA、message、author/committer、parents、verified Ref 与 changed files，未知 SHA 显示明确 not-found。
- Browser：真实 seeded Repo + 第二个 commit 完成桌面和 390px 验收；页面无整体横向溢出，changed-file table 独立横向滚动，无 page error；证据见 `docs/iterations/screenshots/iter-063/`。
- Gates：Backend fmt/check/strict Clippy/workspace tests、Frontend type-check/build/lint、locale parity、Markdown links、diff/governance checks 全部通过。
- Navigator：无 blocking finding；闭环结论 `Complete`。

## Residual Work Destination

- Agent result / Workspace / Activity 入口、文本 patch viewer、Revert/cherry-pick/讨论线程仍归 EVO-105/EVO-104/EVO-121-E。

## DoR And Activation Record

- 2026-08-09 refinement：Agent result 入口是未来消费方，不是 Commit evidence page 的创建前置；从本 Story 验收移回 EVO-105/EVO-104 owner，未缩减稳定 deep-link 目标。
- `diff` 在本 Story 明确为受资源上限约束的 structured changed-file diff；文本 patch viewer 沿用 EVO-112 父项既有 Non-goal，归 EVO-104。
- 类型：API / Product Story；硬依赖 EVO-112-B、EVO-103-C 已 Done，范围预计 0.5-2 天，BDD、API contract、负向测试、前后端与浏览器验证路径齐备，满足 DoR。

## Source Snapshot

- Source: 2026-08-08 interaction completion audit。
- Failure mode: Commit 被称为结果边界，但没有可从 Workspace/Activity 稳定引用的证据页面。
