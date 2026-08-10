# Iteration 065: Settings Information Architecture

> 文档状态：Closed / Complete
> 计划发布日期：2026-08-09
> 计划目标：完成 EVO-121-D，将个人与 Workspace 管理入口收敛到按角色授权的 `/settings/*` 二级结构。
>
> 基线保护：本文件一旦提交，以下“发布计划基线”内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

- 从 user menu 进入共享 Settings layout，Settings 不再占开发主导航。
- 目标路由为 `/settings/profile|workspace|members|api-keys|billing`，刷新和 active state 稳定。
- owner/admin/member 只看到允许入口；无权 deep link 显示 forbidden，隐藏导航不替代 API 授权。
- MVP deliverable：认证用户可在桌面与移动端运行、导航并刷新权限正确的 Settings 页面。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
| --- | --- | --- | --- | --- |
| EVO-121-D | Settings Information Architecture | EVO-121 | P1 | EVO-121-C Done；ADR-0008、权限契约已确认；EVO-118-E 不是开发前置 |

## 3. 发布计划基线：不做事项

- 不改变后端 RBAC、API Key capability 或租户权限契约。
- 不实现 EVO-012 的租户设置保存，不实现 EVO-014/Iteration 026 的 Stripe/计费写能力。
- 不为 `/profile` 或 `/tenant/*` 建设兼容体验；内部链接直接切换到目标路由。
- 不提前实现 EVO-121-A 的最终 App Shell 或 EVO-121-F 的 Legacy UI 删除。

## 4. 发布计划基线：计划验收标准

- Story 形态：Product / Permission Story；使用 Given/When/Then 与角色负向矩阵验收。
- [x] user menu 只有统一 Settings 入口，开发主导航不再包含 Settings。
- [x] 五个 `/settings/*` 路由共享响应式二级导航，刷新后 active state 正确。
- [x] member 可访问 profile/members；admin 可访问 profile/workspace/members/api-keys；owner 可访问全部五页。
- [x] 无权 deep link 显示 403/forbidden 且不挂载业务页。
- [x] 内部 UI 链接不再指向 `/profile` 或 `/tenant/*`；旧路径不提供兼容页面。
- [x] loading/error/retry 由已有业务页真实表达；API Keys 500 不再与 Empty 同时出现，Retry 可恢复。
- [x] 中英文文案、键盘焦点、桌面与 390px 移动布局通过验证。
- [x] 用户可见文档：Product Interaction Architecture 已是目标基线，无额外使用文档受影响。

## 5. 发布计划基线：计划验证

```bash
cd frontend
bun test tests/settings-policy.test.ts
bun run type-check
bun run build
bun run lint

rg -n 'to="/(tenant|profile)|href: "/(tenant|profile)' frontend/src
python3 scripts/tests/check-markdown-links.py
git diff --check
scripts/validate_project_governance.sh .
```

手工/浏览器：owner/admin/member role matrix；desktop + 390px；direct deep link、refresh、active state、forbidden、loading/error/retry、键盘焦点。

## 6. 发布计划基线：风险、威胁模型与回滚

| 项目 | 内容 |
| --- | --- |
| 受保护资产 | Tenant 成员、API Key、计费与 Workspace 管理入口 |
| 攻击者/调用者 | 已认证 member、admin、owner |
| 入口 | user menu、Settings 二级导航、直接 deep link |
| 信任边界 | Browser route visibility → backend JWT/RBAC API |
| 失败模式 | 低权限入口暴露、直接 URL 绕过、无权页面先挂载并发请求、旧链接继续分裂 IA |
| 安全默认 | 未满足角色时显示 forbidden；不挂载受限页面；服务端授权保持最终边界 |
| 验证证据 | 纯策略测试、三角色导航/deep-link 矩阵、真实浏览器验证 |

回滚点：路由、Settings layout、Header/Dashboard 内部链接与 i18n 为独立前端切片；若门禁失败，保持 Story In Progress 并修复，不恢复旧 `/tenant/*` 作为兼容产品路径。

## 7. 闭环台账

| 项目 | 本轮记录 |
| --- | --- |
| 请求结果 | 继续完成所有已规划迭代；本轮交付 EVO-121-D 的可运行 Settings IA |
| 产物 | Settings 路由策略/测试、共享 layout、role guard、Header/Dashboard links、i18n、浏览器证据 |
| 状态同步归口 | EVO-121-D、父 EVO-121、Product Backlog、Iteration 065、Iteration Index、Board |
| Story/BDD 归口 | Product / Permission Story；角色可见性、deep-link forbidden 与 active state 使用 BDD |
| 验证证据 | focused test、frontend 三门禁、角色矩阵、desktop/mobile/keyboard、links/diff/governance |
| 残余工作归口 | 业务页缺失能力保留 EVO-012、EVO-014/Iteration 026；App Shell/Legacy 退场归 EVO-121-A/F |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
| --- | --- | --- |
| 2026-08-09 | inventory | Iterations 018/019/020/027 继续 Superseded/Blocked；025/026 继续 Blocked 待 refinement；056/060 已 Closed / Partial 且 residual 有 owner；061~064 Closed / Complete；无其他 Active/Review iteration 阻塞本轮。 |
| 2026-08-09 | clarification | 修正父项表中 EVO-118-E 的伪硬依赖：ADR-0010 与 Story 正文均确认其只为最终发布 Gate；直接依赖 EVO-121-C 已 Done。 |
| 2026-08-09 | activation | EVO-121-D 通过 DoR：单一 IA 结果、BDD、角色矩阵、范围/不做、依赖、验证和残余 owner 完整；激活 Iteration 065。 |
| 2026-08-09 | driver | 新增共享 Settings layout、五个目标路由、统一角色策略/测试、deep-link forbidden、user menu 与 Dashboard 内部链接收敛；旧路径不提供兼容页面。 |
| 2026-08-09 | browser | owner/admin/member 桌面矩阵、390px、active/refresh/keyboard、API Keys 500→Retry、Workspace 只读与旧路由 404 通过；隔离运行态一次命中本地 rate limit，重启实例后补跑通过。 |
| 2026-08-09 | navigator | 发现并修复 Error 与 Empty 同显、Workspace 假交互、Dashboard 未授权管理入口；复核 member 无管理入口、admin Workspace `enabledControls=0`，无 blocking finding。 |
| 2026-08-09 | validation | focused 4/4、type-check/build/lint、locale parity 730、旧链接扫描、Markdown 293、diff check 通过；governance validator 仅报告预存 Iteration 058 evidence warning。 |
| 2026-08-09 | completion | EVO-121-D Done / Complete；Iteration 065 Closed / Complete；当前无 Active Iteration，下一 Story 需先复核 EVO-118-H 与 EVO-105/106 的硬依赖。 |

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
| --- | --- | --- | --- | --- |

## 10. Review

- 完成：`/settings/*`、共享二级导航、user menu、角色矩阵、forbidden、error/retry、responsive 与 truthful read-only 状态。
- 未完成：EVO-012 租户设置保存与 EVO-014/Iteration 026 计费写路径不在本 Story 范围。
- 验证结果：focused 4/4；frontend type-check/build/lint；locale 730；browser owner/admin/member + desktop/mobile；Markdown/diff/old-link checks 通过。
- Navigator：no blocking findings after one correction cycle；backend RBAC 保持最终授权边界。
- 闭环状态：`Complete`
- 残余归口：EVO-012、EVO-014/Iteration 026、EVO-121-A/F；validator 的预存 Iteration 058 warning 由该历史 iteration owner 复核。

## 11. Retrospective

- 做得好的：导航可见性与 direct-route guard 共用纯策略矩阵，负向行为可独立测试。
- 需要调整的：浏览器错误注入应优先验证稳定错误分类，不依赖后端内部 message；重复全页刷新需考虑运行态限流窗口。
- 写入 EVOLUTION：无新增稳定陷阱；错误不得伪装为空状态已由 Product Interaction Architecture 覆盖。
