# Iteration 062: First-run Repo Onboarding

> 文档状态：Closed / Complete
> 计划发布日期：2026-08-08
> 计划目标：完成 EVO-120 的首次 Repo 创建与登录分流闭环，用真实桌面/移动浏览器验证正常、失败和 deep-link 状态。
>
> 基线保护：本文件一旦提交，以下“发布计划基线”内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。
> 本轮不执行 EVO-118-E，不发布、不部署、不推送，也不修改后端或数据库。

## 1. 发布计划基线：目标

- `/onboarding` 只围绕真实 Repo 创建，不再创建 legacy Tool。
- 普通登录无 redirect 时按真实 Repo 查询结果进入 onboarding 或 `/dashboard`；安全站内 redirect 优先恢复。
- 首个 Repo 创建成功后直接进入 `/repos/:id/overview`，失败时保留输入并提供可重试错误。
- 交付一个桌面和移动端均可操作、可键盘导航、覆盖 loading/empty/error/forbidden 的真实 first-run 页面。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|-----------|--------|---------------|
| EVO-120 | First-run Repo Onboarding | 无 | P0 | EVO-112-A/B、EVO-103-A、EVO-118-F 与 G-B/C/D 已完成；G-A/H residual 不在本次 Repo 查询/创建读写路径 |

## 3. 发布计划基线：不做事项

- 不伪造 remote import；后端无 import contract 时不显示入口。
- 不实现 Workspace、Resources、任务型 Dashboard 内容或完整 root/join/verification entry resolver。
- 不删除 legacy route/backend/table，不执行 production build/deploy，不关闭 DEPLOY-01。
- 不修改 Repo Detail、后端 Repo lifecycle 或数据库 schema。

## 4. 发布计划基线：计划验收标准

- Story 形态：Product / User Story；EVO-120 的 Given/When/Then 场景是验收 owner。
- `/onboarding` 受 AuthGuard 保护；Repo 查询区分 loading、empty、error、forbidden，retry 只重试失败查询。
- 已有 Repo 访问 onboarding 自动 replace 到 `/dashboard`，无 Repo 显示真实 Repo 创建表单。
- 普通 login 无 redirect 时查询 Repo 并分流；站内相对 redirect 优先，外部/协议相对目标不得执行。
- Repo 创建仅提交现有 API contract 字段；pending 禁止重复提交，失败保留所有输入且不跳转。
- 成功使用响应 Repo id 进入 `/repos/:id/overview`；不显示 import 或 Start Workspace 假入口。
- en/zh-CN key parity、键盘焦点、错误关联、桌面与 390px 移动布局通过检查。

## 5. 发布计划基线：计划验证

```bash
cd frontend
bun run type-check
bun run build
bun run lint

cd ..
python3 scripts/tests/check-markdown-links.py
git diff --check
sh /Users/GHuang/.agents/skills/agent-project-governance/scripts/validate_project_governance.sh .
```

- 本地高位端口运行真实 backend/frontend，注册至少两个临时 tenant：无 Repo 与已有 Repo。
- 浏览器验证 login -> onboarding -> create -> overview、已有 Repo -> dashboard、安全 redirect、失败保留输入、未认证 onboarding、桌面/移动无溢出。
- 归档不少于 5 张截图到 `docs/iterations/screenshots/iter-062/`。
- 本 Story 不改后端；后端全量测试明确记录为未重跑，不把工作区中前序后端改动归入本轮证据。

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| Repo list 错误被吞成空状态 | onboarding/login 直接捕获请求错误并区分 forbidden/error，不把失败当作 0 Repo |
| 登录 redirect 可形成 open redirect | 只接受以单 `/` 开头且不以 `//` 开头的站内路径 |
| StrictMode 或快速路由造成陈旧请求写状态 | effect 使用取消标志；redirect 后不继续更新已卸载页面 |
| 创建重复提交或失败丢输入 | pending 禁用提交；输入由受控 state 保存，失败只更新 error |
| 现有脏工作区包含前序后端/治理改动 | 仅手术刀式修改本 Story 文件，不回滚、不混淆验证归属 |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 持续完成全部规划迭代；本轮交付真实 First-run Repo Onboarding |
| 产物 | onboarding 页面、登录分流/安全 redirect、受保护路由、i18n、浏览器证据 |
| 状态同步归口 | EVO-120、Product Backlog、Iteration 062/index、Board、AGENTS、Readiness current execution |
| Story/BDD 归口 | EVO-120 Acceptance Criteria |
| 验证证据 | 前端三门禁、真实浏览器正常/失败/权限/移动流程、Markdown/diff/governance checks |
| 残余工作归口 | Dashboard 内容 -> EVO-121-B；完整 entry resolver -> EVO-121-C；Workspace -> EVO-104；Resources -> EVO-109；legacy 删除 -> EVO-121-F |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-08-08 | inventory | 现有 Active/Review 为零；018-020/027 继续因旧 Registry 方向 Superseded 而阻塞，025/026 继续等待独立 refinement；056/060 已 Closed / Partial 且 residual 分别归 G-A/H，不阻塞本产品 Story。 |
| 2026-08-08 | selection | EVO-112-C 仍包含未完成的 Agent-result deep link 依赖，不满足 DoR；选择依赖闭合且可前端独立验收的 EVO-120。 |
| 2026-08-08 | clarification | 已有 Repo 用户的本 Story 结果限定为进入现有 `/dashboard`；任务型 Dashboard 内容归 EVO-121-B，完整多入口 resolver 归 EVO-121-C。 |
| 2026-08-08 | activation | EVO-120 满足 DoR 并进入 In Progress；Iteration 062 成为唯一 Active Iteration。 |
| 2026-08-09 | deviation | 默认 `:memory:` SQLite 多连接运行态出现 `no such table: users`；独立登记 EVO-124，本 Iteration 不扩 scope，重启 backend 并改用隔离临时 SQLite 文件继续浏览器验收。 |

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|
| 2026-08-08 | clarification | 接受 | 消除 EVO-120 与 EVO-121-B/C 的 owner 重叠，不改变首次 Repo 创建闭环 | 现有 Dashboard 保留；entry helper 只服务 login/onboarding，后续由 EVO-121-C 统一复用 |

## 10. Review

- 完成：真实 Repo onboarding、普通登录分流、安全站内 redirect、AuthGuard、seeded create -> Overview、查询与创建失败态、双语文案、桌面/移动及键盘路径全部闭合。
- 未完成：无本 Story 验收残余；完整 entry resolver、任务型 Dashboard、Workspace、Resources 与 legacy 删除保留在既有 owner Story。
- 验证结果：`bun run type-check`、`bun run build`、`bun run lint` 通过；真实 backend/frontend + Headless Chrome 覆盖未认证、两个 tenant、500、403、Retry、409、创建成功、existing Repo、redirect 与 390px；截图 6 张归档于 `screenshots/iter-062/`。Markdown、diff、governance 最终门禁见关闭记录。
- Navigator：2026-08-09 复核登录分流、错误不吞为 empty、open redirect 防护、异步卸载保护、pending/输入保留、i18n 与路由闭环；无未关闭 finding。
- 闭环状态：`Complete`
- 残余归口：见闭环台账；另发现默认 `:memory:` SQLite 多连接隔离缺陷并独立登记 EVO-124，不属于本 Story。

## 11. Retrospective

- 做得好的：先以真实浏览器发现“创建成功但空仓无默认 ref”的假闭环，再用既有 `seed_template` contract 收敛为真实 Initial Commit；负向与恢复路径均保留可核验证据。
- 需要调整的：本地默认 `:memory:` 不能作为多连接 runtime smoke 的可靠数据库，后续由 EVO-124 修复；在此之前按 LOCAL-DEV 使用临时 SQLite 文件。
- 写入 EVOLUTION：EVO-124 与 LOCAL-DEV 已承接该可复用陷阱，本轮不重复写入历史经验。

## 12. 关闭记录

| 日期 | 记录 |
|------|------|
| 2026-08-09 | EVO-120 全部 Technical Acceptance 完成；Driver 实现与 Navigator 复核闭合，Iteration 062 Closed / Complete。 |
| 2026-08-09 | 最终门禁：frontend type-check/build/lint 通过；en/zh-CN JSON 与 key parity 通过；Markdown link validation 通过 290 个仓库文件；`git diff --check` 通过；governance validation 通过并仅保留既有 Iteration 058 证据格式 warning。 |
| 2026-08-09 | 本轮未修改 backend/schema/deploy，未重跑工作区前序后端全量测试；不关闭 DEPLOY-01，不声明生产就绪。 |
