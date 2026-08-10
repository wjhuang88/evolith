# EVO-121-E Auditable Activity Timeline

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- [Parent Epic](EVO-121-product-experience-convergence.md)
- [Product Interaction Architecture](../../design/PRODUCT-INTERACTION-ARCHITECTURE.md)
- [EVO-118-H Durable Outbox](EVO-118-H-durable-outbox-events.md)
- [EVO-105 Commit And Promote](EVO-105-commit-and-promote-api.md)
- [EVO-106 Agent Session](EVO-106-agent-session-and-scoped-token.md)
- [EVO-112-C Commit Evidence](EVO-112-C-repo-commit-evidence-detail.md)
- [API Contract](../../reference/API-CONTRACT.md)

## Summary

- 类型：feature / api / frontend
- 优先级：P0
- 状态：Proposed / paused
- 父 Epic：`EVO-121`
- 影响范围：backend、frontend、API contract、tests

## Problem Or Outcome

目标导航包含 Activity，但当前没有稳定事件 read model 或页面。进程内事件也不能证明 Agent 操作、Policy 结果和 Commit 之间的关系。

作为开发者或审查者，我希望按时间查看可追溯事件并回到 Repo、Session 和 Commit，以便确认系统做了什么、结果在哪里以及为什么需要 review/block。

## Goal And Non-goals

- Goal：tenant-scoped、分页的 Activity read API；事件类型至少覆盖 Session、Commit、Promote、Policy result；页面支持 Repo/status/type 过滤和实体 deep link。
- Non-goals：不建设通用日志搜索或 SIEM；不暴露 secret、token、prompt 私密内容或跨 tenant existence。

## Dependencies And Blockers

- 硬依赖 EVO-118-H durable event boundary。
- Commit/Policy/Session 字段依赖 EVO-105/106 的稳定契约。
- API 变更必须先更新 API-CONTRACT；权限与 tenant hiding 需要负向测试。

## Acceptance Criteria

```gherkin
Scenario: 用户追溯 require_review 结果
  Given Agent session 产生 require_review Commit
  When 用户在 Activity 过滤对应 Repo
  Then 时间线关联显示 Session、Policy result 和 Commit
  And 用户可进入对应 Repo/Session/Commit

Scenario: 用户无权查看其他 tenant 事件
  Given 事件属于另一 tenant
  When 用户按事件 ID 或过滤条件查询
  Then API 不泄露事件存在性或元数据

Scenario: 事件读取失败
  Given Activity API 暂时不可用
  When 页面加载
  Then 页面显示 retry error
  And 不显示假 empty timeline
```

## Validation Evidence Required

- Backend handler/repository 集成测试、tenant/RBAC 负向矩阵、分页稳定性测试。
- API Contract 与 frontend client 对齐。
- Backend baseline 与 frontend type-check/build/lint。
- Playwright：时间线、过滤、实体 deep link、empty/error/forbidden、移动布局。

## Residual Work Destination

- 高级全文检索、导出和长期归档独立评估；Webhook delivery 管理仍归 EVO-107/118-H。
