# Iteration 017: 前端 CLI Interface 概念收口

> 文档状态：Closed
> 计划发布日期：2026-05-27
> 激活日期：2026-05-29
> 计划目标：完成 Vite SPA 中仍暴露给用户的 Snippet 概念迁移，使前端入口与
> CLI-friendly interface 产品方向一致。
>
> 基线保护：本文件一旦提交，以下"发布计划基线"内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

本轮候选承接 `EVO-017` 已确立的 CLI-friendly interface 产品方向，处理前端仍保留
`/snippets` 页面、API client 和可见文案的问题。该计划只建立执行基线，不在本次
规划中启动实现。

激活门禁：

- [Iteration 009](ITERATION-009.md) 与 [Iteration 010](ITERATION-010.md) 的 `Review`
  收口缺口已由 [Iteration 021](ITERATION-021.md) 处理完成；该阻塞已解除。
- 激活前重新核对 `EVO-026` 仍满足 DoR，且没有新的依赖或范围变化。 ✅ DoR 通过：验收标准明确，依赖 EVO-017/022-025 均 Done。
- 明确旧 `/snippets` 路由采用兼容跳转、别名保留或下线策略。 → 决策：前端 React Router 增加重定向规则，旧路径 `/snippets*` → `/interfaces*`。

**激活核查（2026-05-29）**：
- [x] 无 Active / In Progress / Review iteration
- [x] EVO-026 DoR 满足（验收标准、依赖、影响范围、验证方式均完整）
- [x] 范围确认：前端代码符号和 UI 概念迁移，不涉及后端 API 路由重命名
- [x] 后端保持 `/api/v1/snippets` 兼容路径（ADR-0002 + CLI-INTERFACE-FORMAT.md 明确）
- [x] 旧路由兼容策略：React Router 重定向 `/snippets*` → `/interfaces*`

### 执行计划

**范围**：16 个前端文件，~228 处 snippet 引用。前端代码符号重命名 + UI 概念迁移。
**不涉及**：后端 API 路由、数据库、build 脚本、部署配置。

| 步骤 | 内容 | 文件 |
|------|------|------|
| 1 | 类型文件重命名 `snippet.ts` → `cli-interface.ts`，接口 `Snippet` → `CliInterface` | `types/snippet.ts`, `types/index.ts`, `lib/api/types.ts` |
| 2 | API client 文件重命名 `snippets.ts` → `cli-interfaces.ts`，保持 `/api/v1/snippets` 后端路径 | `lib/api/snippets.ts`, `lib/api/index.ts` |
| 3 | 路由目录重命名 `app/snippets/` → `app/interfaces/`，更新 `main-spa.tsx` 路由注册 | 3 个页面文件 + `main-spa.tsx` |
| 4 | 页面组件内 snippet 引用迁移（imports、state、i18n keys、API calls） | 3 个页面文件 |
| 5 | 导航 Header 迁移 + Dashboard/Landing 页面迁移 | `Header.tsx`, `dashboard/page.tsx`, `page.tsx` |
| 6 | i18n 两个 locale 文件迁移：`snippets.*` → `interfaces.*` | `en.json`, `zh-CN.json` |
| 7 | Permissions hooks + Billing UsageDisplay 迁移 | `usePermission.ts`, `UsageDisplay.tsx` |
| 8 | 旧路由兼容：React Router 重定向 `/snippets*` → `/interfaces*` | `main-spa.tsx` |
| 9 | 验证：`bun run type-check` + `bun run build` + 搜索确认仅剩兼容/历史引用 | — |

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|---------|--------|---------------|
| EVO-026 | 前端 Snippets 入口迁移为 CLI 友好接口 | 无 | P1 | EVO-017 已 Done；当前 `Ready`，激活前重新核对 DoR |

## 3. 发布计划基线：不做事项

- 不实施 Skill ZIP / Git / SkillHub 导入或版本管理。
- 不删除后端 legacy snippet 兼容 API，除非另有契约和迁移计划。
- 不启动 embedded frontend 或 GitHub CI/CD。

## 4. 发布计划基线：计划验收标准

- [ ] 前端用户可见导航、页面和中英文文案不再将 Snippet 作为新主概念。
- [ ] API client 和路由迁移策略有明确兼容边界，并同步相关合约或 reference。
- [ ] 搜索代码确认遗留 `snippet` 引用只剩明确的兼容或历史用途。
- [ ] `bun run type-check` 与 `bun run build` 通过并记录真实结果。

## 5. 发布计划基线：计划验证

```bash
cd frontend
bun run type-check
bun run build
rg -n "Snippet|snippet|snippets|代码片段" src
```

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| 前端改名但仍请求 legacy API，造成概念和契约混乱 | 激活前确定兼容 client 与路由策略，保留迁移说明 |
| 路由迁移破坏已保存链接 | 保留兼容跳转或别名，使用浏览器回归验证关键入口 |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | Active；EVO-026 选入执行 |
| 产物 | 前端代码符号 Snippet→CliInterface 重命名 + UI 概念迁移 + 旧路由重定向 |
| 状态同步归口 | EVO-026 → In Progress → Done；Iteration 017 → Active → Closed |
| 验证证据 | `bun run type-check` 0 errors；`bun run build` 0 errors；snippet 搜索确认仅剩兼容/历史 |
| 残余工作归口 | 后端 `/api/v1/snippets` → `/api/v1/cli-interfaces` 路由重命名另建 story；`max_snippets` Plan 字段与后端 billing schema 同步 |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-05-27 | planning | 发布 future plan 基线；EVO-026 总表与详情块已同步为 `Ready`，但因 Iteration 009 / 010 仍为 `Review`，本迭代暂不可激活。 |
| 2026-05-28 | planning-disposition | Iteration 009 / 010 已由 Iteration 021 关闭；本迭代转为 `Planned / Ready for activation`，激活前仍需核对 EVO-026 DoR 与旧 `/snippets` 路由策略。 |
| 2026-05-29 | activation | 库存盘点：无 Active/IP/Review iteration。EVO-026 DoR 通过。后端无 `/api/v1/cli-interfaces` 路由，确认前端-only 迁移。激活执行计划 9 步。状态 → Active。 |
| 2026-05-29 | execution | 16 前端文件完成迁移：types、API client、路由目录、页面组件、导航、i18n、permissions、billing。旧 `/snippets` 路由保留重定向兼容。后端 API 路径不变。 |

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|

## 10. Review

- 完成：
  - [x] 导航、页面标题、空状态、按钮、详情页和新建页不再以 Snippet 作为用户可见主概念
  - [x] `snippetsApi` → `cliInterfacesApi`，后端 API 路径保持 `/api/v1/snippets` 兼容
  - [x] 中英文 i18n 文案全部迁移（`snippets.*` → `interfaces.*`）
  - [x] 旧 `/snippets` 路由有 React Router 重定向兼容策略
- 未完成：无
- 验证结果：
  - `bun run build` → 0 errors, 546KB JS bundle
  - `cargo check --workspace` → 0 errors
  - `snippet|Snippet` grep → 仅剩 API URL `/snippets` 和重定向路由，无用户可见残留
  - `zh-CN.json` 和 `en.json` → 0 snippet/代码片段 残留
- 闭环状态：`Complete`
- 残余归口：
  - 后端 `/api/v1/snippets` → `/api/v1/cli-interfaces` 路由重命名另建 story（超出本次前端迁移范围）
  - `max_snippets` Plan 字段与后端 billing schema 同步另建 story

## 11. Retrospective

- 做得好的：前端-only 迁移边界清晰，后端 API 路径不变减少了实现复杂度和风险。旧路由重定向确保了向后兼容。
- 需要调整的：两套重复类型定义（`types/cli-interface.ts` 和 `lib/api/types.ts` 中的 `CliInterface`）应后续统一。
- 写入 EVOLUTION：前端概念迁移应先确认后端 API 兼容边界再动手，避免过度改动。
