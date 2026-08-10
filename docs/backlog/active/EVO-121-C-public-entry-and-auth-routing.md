# EVO-121-C Git-centric Public Entry And Auth Routing

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- [Parent Epic](EVO-121-product-experience-convergence.md)
- [Product Interaction Architecture](../../design/PRODUCT-INTERACTION-ARCHITECTURE.md)
- [ADR-0008](../../decisions/ADR-0008-repo-centric-interaction-architecture.md)
- [EVO-120 First-run Repo Onboarding](EVO-120-first-run-repo-onboarding.md)

## Summary

- 类型：feature / frontend
- 优先级：P0
- 状态：Done / Complete（Iteration 064 Closed / Complete）
- 父 Epic：`EVO-121`
- 影响范围：frontend（landing、auth routes、route guard、i18n、tests）

## Problem Or Outcome

Landing 仍把 Tools/Skills/Interfaces 作为三项核心能力，登录和 join 成功固定跳 `/dashboard`，受保护 deep link 与 first-run 分流彼此独立。

作为首次访问者或返回用户，我希望入口准确表达 Git hosting + Agent Workspace + Repo-derived Discovery，并在认证后回到正确任务，以便不丢失上下文。

## Goal And Non-goals

- Goal：公开首页展示真实产品与真实截图/状态；统一 entry resolver 处理 root、login、join、verification 和 protected deep link；无 Repo 用户进入 onboarding。
- Non-goals：不建设内容营销 CMS；不展示未实现能力的可点击 demo；不保留旧产品 feature cards。

## Dependencies And Blockers

- EVO-120、EVO-112-B/C 已 Done；复用 first-run 判定、真实 Repo route 与 Commit evidence，不依赖最终 DEPLOY-01。
- `/docs`、`/privacy`、`/terms` 若未注册，不得继续作为可点击链接；应实现真实页面或移除入口。

## Governing Decisions And Security Boundary

- [ADR-0008](../../decisions/ADR-0008-repo-centric-interaction-architecture.md)：合法 protected deep link 优先于 onboarding/dashboard；所有普通入口使用同一 Repo 分流；不建立旧 UI 兼容层。
- [Product Interaction Architecture](../../design/PRODUCT-INTERACTION-ARCHITECTURE.md)：每个 loading/error/forbidden/retry/deep-link/mobile 状态均是验收范围。
- entry policy 只接受 same-origin 单斜杠绝对路径，拒绝外部 origin、scheme-relative URL 与 login loop；AuthGuard 保存 pathname + search + hash。
- Repo 查询失败不得 fallback 为 onboarding；目标页 API 返回 403 时保留原 URL 并显示 forbidden。

## Acceptance Criteria

```gherkin
Scenario: 未登录用户访问受保护 Repo deep link
  Given 用户没有有效 session
  When 用户访问 /repos/{id}/files
  Then 登录页保存完整原路径
  And 登录成功且有权限后返回该 deep link

Scenario: 新用户普通登录
  Given 用户登录成功且 tenant 没有 Repo
  When entry resolver 完成 Repo 判定
  Then 用户进入 /onboarding

Scenario: 已有 Repo 用户访问根路径
  Given 用户已登录且 tenant 有 Repo
  When 用户访问 /
  Then 用户进入 /dashboard

Scenario: 用户没有 deep link 权限
  Given 用户登录成功但无权访问原目标
  When resolver 恢复目标
  Then 页面显示 forbidden
  And 不静默回退 Dashboard

Scenario: 入口判定失败
  Given 用户已完成认证且没有待恢复 deep link
  When Repo 查询返回 forbidden 或暂时失败
  Then 页面显示明确错误和 retry
  And 不把失败解释为无 Repo

Scenario: 外部 redirect 被拒绝
  Given login、register 或 verification URL 携带外部或 scheme-relative redirect
  When entry policy 解析目标
  Then 外部目标不被恢复
  And 普通 Repo entry 判定继续执行
```

## Validation Evidence Required

- route resolver 单元/集成测试覆盖 root、login、join、verification、redirect 编码和 forbidden。
- `bun run type-check`、`bun run build`、`bun run lint`。
- Playwright 桌面/移动：首页、无 Repo 登录、已有 Repo 登录、deep link 恢复。
- 截图证明 landing 第一视口展示真实产品信号且无旧 Tools/Skills/Interfaces 主线。

## DoR And Activation Record

- Story 形态：Product / Authentication Routing Story；身份、价值、BDD、负向安全场景和残余归口齐备。
- 单一结果：所有 public/auth 入口消费同一 entry policy，landing 同步为该入口的真实产品表达；影响仅 frontend/i18n/docs，预计 0.5-2 天。
- 硬依赖：EVO-120、EVO-112-B/C Done；ADR-0008 Accepted；不存在循环依赖。
- Driver/Navigator：公开入口和认证路由属于安全敏感跨页面变更，Iteration 064 已记录威胁模型、失败矩阵和独立 Navigator Gate。
- 状态归口：EVO-121-C/parent、Product Backlog、Iteration 064/index、Board、AGENTS、Readiness/docs map/roadmap。

## Residual Work Destination

- SEO、内容实验与高级营销分析不属于 MVP，后续独立 Story。

## Completion Record

- 2026-08-09：共享 pure entry policy 与 async resolver 已统一 root、login、join、register/verification return-to-login 和 AuthGuard；完整 `pathname + search + hash` 经编码恢复。
- 普通入口按真实 Repo inventory 分流，0 Repo -> onboarding、已有 Repo -> dashboard；403/5xx 不再伪装成 first-run，Retry 保留 session。
- Repo 目标 403 保留原 URL 并显示 target-specific forbidden；外部、scheme-relative、大小写/尾斜杠/编码 login-loop redirect fail closed。
- Landing 已切换为 Git-centric 真实产品表达：Git Repo/Commit evidence 标记 Available，Agent work 标记 In development，Repo-derived discovery 标记 Planned；未来能力无 action，未注册死链和旧三卡主线已移除。
- Browser evidence：真实 SQLite + 隔离 Git Storage 覆盖无 Repo login、seeded Repo deep-link login、已有 Repo root、join deep link、register/verification continuity、外部 redirect 拒绝、403、500 + Retry、1440/390px 和无横向溢出；截图见 [Iteration 064](../../iterations/ITERATION-064.md)。
- Navigator：发现并修复大小写/尾斜杠/percent-encoded `/login` loop；复核后无 blocking finding。闭环结论：`Complete`。
