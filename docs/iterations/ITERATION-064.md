# Iteration 064: Git-centric Public Entry And Auth Routing

> 文档状态：Closed / Complete
> 计划发布日期：2026-08-09
> 计划目标：完成 EVO-121-C，以单一 entry policy 统一 root、login、join、verification 与 protected deep link，并将公开首页从旧 Registry 定位切换为真实 Git-centric 产品入口。
>
> 基线保护：本文件一旦提交，以下“发布计划基线”内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。
> 本轮不实现 Settings IA、Task-first Dashboard、Workspace、Discovery、Legacy route 删除或生产发布。

## 1. 发布计划基线：目标

- 将完整站内目标 `pathname + search + hash` 安全保存到 login，并在认证后优先恢复；外部、scheme-relative、login-loop 目标 fail closed。
- 普通认证入口统一按真实 Repo 查询分流：无 Repo -> `/onboarding`，有 Repo -> `/dashboard`；查询失败保留明确错误与 retry，不伪装成无 Repo。
- 已认证访问 `/` 时复用同一 resolver；join 成功、verification -> login 与 register -> login 保留安全 redirect。
- Repo deep link 返回 403 时显示明确 forbidden，不回退 Dashboard；公开首页首屏只陈述真实 Git hosting/Commit evidence 状态，不显示旧 Tools/Skills/Interfaces 主线或未注册链接。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|-----------|--------|---------------|
| EVO-121-C | Git-centric Public Entry And Auth Routing | EVO-121 | P0 | EVO-120、EVO-112-B/C Done；ADR-0008 与 Product Interaction Architecture 已 Accepted |

## 3. 发布计划基线：不做事项

- 不修改 backend、数据库、Cookie/JWT 合约、RBAC 规则或 CSRF exemption。
- 不实现 `/settings/*`、Activity、Workspace、Discover、Resources 或最终 App Shell。
- 不为旧 Registry 页面建立兼容跳转或营销 CMS；Legacy route 删除仍归 EVO-121-F。
- 不把尚未实现的 Agent Workspace/Repo-derived Discovery 描述成已可用能力，也不创建假 demo 按钮。

## 4. 发布计划基线：计划验收标准

- Story 形态：Product / Authentication Routing Story；EVO-121-C Given/When/Then 是验收 owner。
- 未登录访问 `/repos/:id/files?ref=main#blob`，login redirect 完整保存并编码原路径；成功登录后恢复同一 URL。
- 无 redirect 登录与 join 成功使用同一 Repo 判定；无 Repo 到 onboarding，有 Repo 到 dashboard；查询 403/5xx 显示明确错误并可 retry。
- 已认证访问 `/` 使用同一 entry policy，不渲染 public landing 后再静默停留。
- register/verification 在返回 login 时保留经校验的站内 redirect；外部或 `//host` redirect 被拒绝。
- 原目标 API 返回 403 时页面显示 forbidden，且 URL/页面不跳 Dashboard。
- Landing 第一视口表达 Git Repository 与可审计 Commit 证据；未来 Agent/Discovery 状态真实标注且不可点击；无 `/docs`、`/privacy`、`/terms` 等未注册死链。
- loading/error/forbidden/retry、桌面和 390px、键盘焦点与无页面横向溢出通过浏览器验收；en/zh-CN key parity 保持一致。

## 5. 发布计划基线：计划验证

```bash
cd frontend
bun test tests/entry-policy.test.ts
bun run type-check
bun run build
bun run lint

cd ..
python3 scripts/tests/check-markdown-links.py
git diff --check
sh /Users/GHuang/.agents/skills/agent-project-governance/scripts/validate_project_governance.sh .
```

- 真实 backend/frontend + 浏览器验证匿名 deep link、无/有 Repo 登录、root resolver、403、resolver failure/retry、landing 桌面/390px；截图归档到 `docs/iterations/screenshots/iter-064/`。

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| crafted redirect 形成 open redirect 或 login loop | pure policy 只接受同 origin 单斜杠绝对路径，拒绝 `//`、外部 origin 与 `/login`；单元和浏览器负向验证 |
| AuthGuard 只保存 pathname 丢失任务上下文 | 统一构造 pathname + search + hash，并在 login URL 中安全编码 |
| Repo 查询失败被误判为 first-run | resolver error 保持显式失败与 retry；不得 fallback onboarding/dashboard |
| forbidden deep link 被 Dashboard 掩盖 | 目标页保留原 URL 并渲染 403；entry policy 不探测后改写受保护目标 |
| public landing 夸大未实现能力或暴露死链 | 只将 Git hosting/Commit evidence 标为 available；未来能力明确非可用状态且无 action；移除未注册 route |
| auth/root 跨层改动产生分叉 | 单一 pure entry policy + shared async resolver；Driver 后执行 Navigator 路由/安全矩阵复核 |

## 7. 安全审查最小记录

| 项目 | 内容 |
|------|------|
| 受保护资产 | auth token、原始任务 URL、tenant Repo 存在性、Repo 页面授权结果 |
| 攻击者/调用者 | crafted redirect 的匿名访问者、无目标 Repo 权限的已认证用户、过期 session |
| 入口 | `/`、`/login`、`/register`、`/join`、`/verify-email`、所有 AuthGuard route |
| 信任边界 | Browser URL -> entry policy -> auth store/API -> protected page API |
| 失败模式 | open redirect、query/hash 丢失、循环跳转、错误 first-run、403 被 fallback、公开页虚假能力/死链 |
| 安全默认 | 站外 redirect 拒绝；Repo 查询失败显式失败；目标授权由 API 决定；403 留在目标 URL |
| 验证证据 | pure redirect matrix、真实登录/Repo 查询、403/5xx 拦截、桌面/移动浏览器与 Navigator |

## 8. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 持续完成全部规划迭代；本轮交付真实 Git-centric landing 与一致、安全、可恢复的认证入口编排 |
| 产物 | entry policy/resolver、AuthGuard、root/login/join/register/verification、Repo forbidden state、landing/i18n、tests/browser evidence |
| 状态同步归口 | EVO-121-C、EVO-121 parent、Product Backlog、Iteration 064/index、Board、AGENTS、Readiness/docs map/roadmap |
| Story/BDD 归口 | EVO-121-C Acceptance Criteria 与本页 auth/public route matrix |
| 验证证据 | pure policy tests、frontend gates、真实浏览器正常/负向/移动、docs/diff/governance checks |
| 残余工作归口 | Settings -> EVO-121-D；Dashboard -> EVO-121-B；App Shell/Legacy -> EVO-121-A/F；Workspace/Discovery -> EVO-104/109 |

## 9. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-08-09 | inventory | 无 Active/Review；018-020/027 继续因 Superseded direction 阻塞，025/026 继续等待独立 refinement；056/060 Closed / Partial residual 已有 owner，不阻塞本 Story。 |
| 2026-08-09 | refinement | 明确完整 URL 保存、resolver failure/retry、register/verification redirect continuity、Repo 403 与 truthful landing 边界；不扩大 backend/auth contract。 |
| 2026-08-09 | activation | EVO-120、EVO-112-B/C 已 Done，ADR/BDD/威胁模型/验证路径齐备；EVO-121-C 满足 DoR，Iteration 064 成为唯一 Active Iteration。 |
| 2026-08-09 | driver | 新增 pure entry policy、共享 Repo resolver 与完整 URL AuthGuard；root/login/join/register/verification 统一 return-to-task，Landing 改为 truthful Git-centric 状态表达，Repo 403 独立呈现。 |
| 2026-08-09 | browser | 内置浏览器连接因缺少会话 sandbox policy metadata 不可用，按 browser skill fallback 到隔离 Chrome/CDP；真实 SQLite file + `/tmp` Git Storage 覆盖完整 route/security matrix。初始夹具遗漏 `seed_template` 仅造成空 Repo file-tree 404，随后用 seeded Repo 重验并覆盖截图，不计为产品失败。 |
| 2026-08-09 | navigator | 发现精确 `/login` 判定未覆盖大小写、尾斜杠与 percent-encoded route；Driver 规范化 decoded pathname 并补 3 个负向断言，重验后无 blocking finding。 |
| 2026-08-09 | closure | EVO-121-C Done / Complete；Iteration 064 Closed / Complete；当前无 Active Iteration，下一候选 EVO-121-D 必须重新执行 DoR。 |

## 10. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|
| 2026-08-09 | clarification | 接受 | 补足原 Story 的失败语义和安全可验证性，不改变统一 public/auth entry 目标 | 无半成品；按本基线实施 |

## 11. Review

- 完成：entry policy/resolver、完整 deep link、root/login/join/register/verification 编排、0/已有 Repo 分流、403/5xx + Retry、truthful landing、双语与桌面/390px 验收全部闭合。
- 未完成：本 Story 无未完成 acceptance；Settings、Dashboard、App Shell、Activity、Discovery 与 Legacy cleanup 继续由既有 Story 承接。
- 验证结果：
  - `bun test tests/entry-policy.test.ts`：4 pass / 0 fail / 13 assertions；覆盖完整 URL、外部/scheme-relative、安全 auth-page 传播、大小写/尾斜杠/编码 login-loop 与 Repo count 分流。
  - `bun run type-check`、`bun run build`、`bun run lint`：通过；build 仅保留既有 Browserslist stale 与 chunk-size warning。
  - locale parity：en/zh-CN 各 719 scalar keys，双向缺失 0。
  - 真实浏览器：无 Repo login -> onboarding；seeded Repo 匿名 deep link -> encoded login -> 原 `path + query + hash`；已有 Repo root -> dashboard；join -> 原 deep link；external redirect -> dashboard；register/verification login href 保留安全 redirect；Repo 403 保留 URL；Repo list 500 显式 error + Retry -> dashboard；page error 0、桌面/390px overflow false。
  - 截图：[Landing desktop](screenshots/iter-064/00-landing-desktop.png)、[Landing 390px](screenshots/iter-064/01-landing-mobile.png)、[Seeded deep link](screenshots/iter-064/02-deep-link-restored.png)、[Repo forbidden](screenshots/iter-064/03-repo-forbidden.png)、[Resolver error](screenshots/iter-064/04-entry-resolution-error.png)。
  - `python3 scripts/tests/check-markdown-links.py`、`git diff --check`：通过；governance validator 通过，仅保留既有 Iteration 058 validation-evidence warning。
- Navigator：安全路由、错误语义、truthful claims、键盘焦点、移动布局与状态同步复核完成；发现的 login-loop 规范化问题已修复，无 blocking finding。
- 闭环状态：`Complete`
- 残余归口：见闭环台账；不新增无 owner residual。

## 12. Retrospective

- 做得好的：先把 redirect、Repo inventory 与目标授权拆成 pure policy / async resolver / protected page 三个边界，使 open redirect、first-run 假成功与 403 fallback 可以独立验证。
- 需要调整的：浏览器夹具创建 Repo 时必须显式 `seed_template: true`，并为本地 file SQLite、Git Storage 与 rate limit 提供隔离配置；本轮已在最终证据中纠正，不改变产品代码。
- 写入 EVOLUTION：未发现新的项目级陷阱；SQLite file 与真实 Seed 要求已由 LOCAL-DEV/Current Known Traps 覆盖，不重复写回。
