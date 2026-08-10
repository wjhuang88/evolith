# EVO-121 Product Experience Convergence

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- [Product Interaction Architecture](../../design/PRODUCT-INTERACTION-ARCHITECTURE.md)
- [ADR-0008 Repo-centric Interaction Architecture](../../decisions/ADR-0008-repo-centric-interaction-architecture.md)
- [EVO-120 First-run Repo Onboarding](EVO-120-first-run-repo-onboarding.md)
- [EVO-112-B Repo Detail](EVO-112-B-repo-detail-read-only.md)
- [EVO-112-C Commit Evidence](EVO-112-C-repo-commit-evidence-detail.md)
- [Testing SOP](../../sop/TESTING.md)

## Summary

| Field | Value |
| --- | --- |
| ID | EVO-121 |
| Type | epic / frontend + api |
| Status | In Progress（EVO-121-C/D Done；其余子 Story 按依赖推进） |
| Priority | P0 |
| Parent Epic | None |
| Source | 2026-08-08 product interaction completion audit |

## Problem Or Outcome

Repo onboarding、Repo Detail、Workspace 和 Discovery 已有独立 owner，但整体页面编排仍缺 App Shell、任务型 Dashboard、统一鉴权入口、Settings、Activity 和 Legacy UI 退场的执行归口。若不拆分，开发者会继续在现有路由上局部补丁，最终无法形成一条一致的产品主链。

本 Epic 将 [Product Interaction Architecture](../../design/PRODUCT-INTERACTION-ARCHITECTURE.md) 中尚未归口的页面收敛为可独立验收的 Story。

## Goal And Non-goals

### Goal

- 所有目标页面、入口判断和导航都有唯一 owner。
- 应用主导航只表达日常开发任务。
- 公开入口、鉴权、first-run、deep link 和 settings 路径使用统一编排。
- Activity 将 Session、Policy、Commit 与事件证据连接起来。
- Repo-derived Discovery 可用后直接删除旧 Registry UI。

### Non-goals

- 不在本 Epic 内实现 Repo Detail、Workspace 或 Discovery 本体。
- 不保留旧 UI 路由兼容、迁移向导或 Legacy 菜单。
- 不在本 Epic 删除后端 legacy API/DB；取消双写与后端退场归 ADR-0009 / EVO-122，Sandbox 归 EVO-111。

## Child Stories

| 子 Story | 独立结果 | 状态 | 依赖 | 所属迭代 |
| --- | --- | --- | --- | --- |
| [EVO-121-A](EVO-121-A-target-application-shell.md) | 目标 App Shell 与无死链主导航 | Proposed / paused | EVO-109, EVO-121-D, EVO-121-E | - |
| [EVO-121-B](EVO-121-B-task-first-dashboard.md) | 任务型 Dashboard | Proposed / paused | EVO-112-B/C, EVO-105, EVO-106, EVO-121-E | - |
| [EVO-121-C](EVO-121-C-public-entry-and-auth-routing.md) | Git-centric 公开入口与统一 entry resolver | Done / Complete | EVO-120, EVO-112-B/C（Done） | Iteration 064 Closed / Complete |
| [EVO-121-D](EVO-121-D-settings-information-architecture.md) | `/settings/*` 管理信息架构 | Done / Complete | EVO-121-C（Done）；EVO-118-E 仅为最终发布 Gate | Iteration 065 Closed / Complete |
| [EVO-121-E](EVO-121-E-auditable-activity-timeline.md) | 可追溯 Activity 时间线 | Proposed / paused | EVO-118-H, EVO-105, EVO-106 | - |
| [EVO-121-F](EVO-121-F-retire-legacy-registry-ui.md) | 删除旧 Registry 页面、路由与文案 | Proposed / paused | EVO-109, EVO-121-A | - |

## Epic Completion Criteria

- [ ] 六个必需子 Story 均为 Done，或经 ADR 明确缩减目标。
- [ ] EVO-120、EVO-112-B/C、EVO-104、EVO-109 与本 Epic 共同覆盖 Route Ownership Matrix 的全部 surface。
- [ ] 主链 `login -> onboarding/dashboard -> repo -> workspace -> commit result -> activity` 有桌面与移动 E2E 证据。
- [ ] `rg` 证明用户可见新导航和页面不再呈现旧 Tool/Skill/Interface CRUD 主线。
- [ ] 没有未注册目标路由、假成功、吞错为空状态或无 provenance 的 Discovery 结果。

## Closure Ledger

| 项目 | 本轮记录 |
| --- | --- |
| 请求结果 | 从第一性原理重建整体交互和页面编排，并推进到可开发闭环 |
| 产物 | Product Interaction Architecture、ADR-0008、EVO-120、EVO-121 与六个子 Story、相关 owner 同步 |
| 状态同步归口 | Product Backlog、EVO-100、Board、Implementation Roadmap、Production Readiness Plan、Docs Map |
| 验证证据 | 2026-08-08：governance validator `0 warning(s)`；全量 Markdown 相对链接 `all markdown links exist`；`git diff --check` 通过；Route Ownership Matrix 13 个 surface 均有 owner；EVO-112/121/122 父子项与启动顺序核对无循环依赖；EVO-110 仅保留 Dropped/历史引用 |
| 残余工作归口 | 具体产品实现归各子 Story；EVO-118 Gate 保持独立，不在本次文档任务中启动 |

## Residual Work Destination

- First-run：EVO-120。
- Repo read experience：EVO-112-B/C。
- Controlled Agent Workspace：EVO-104/105/106/107。
- Repo-derived Resources / Discover：EVO-108/109。
- Backend Repo-derived 承接与直接清理：EVO-122；Sandbox removal：EVO-111。

## Source Snapshot

- Source: 用户要求按最终目标重新审视整体交互，不为未上线产品投入旧版迁移成本。
- Confirmed current facts: `main-spa.tsx` 无 Repo Detail/Discover/Activity 路由；onboarding 创建 Tool；Header 有 Legacy disclosure；landing 仍以 Tools/Skills/Interfaces 为主。
