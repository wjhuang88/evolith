# Iteration 040: 前端死代码与类型卫生清理

> 文档状态：Closed（2026-06-04）
> 计划发布日期：2026-06-04
> 计划目标：完成 EVO-058，清除前端未使用的死类型、不安全断言和假实现，使类型系统反映真实 API 契约。
>
> 基线保护：本文件一旦提交，以下"发布计划基线"内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

- 删除 `src/types/` 死类型目录（auth.ts / tool.ts / skill.ts / cli-interface.ts / index.ts，整目录未被引用且与真实 API 冲突）。
- 删除 `src/stores/uiStore.ts`（导出但无引用）。
- 删除 `src/app/accept-invitation/page.tsx`（冗余 re-export，路由已映射 JoinPage）。
- 修正不安全断言：`lib/theme.tsx:18` 与 `hooks/usePermission.ts:20` 的 `as` cast、`main-spa.tsx:43` 的 `getElementById('root')!`。
- 合并 `lib/api/types.ts` 重复 `User` 定义为单一真源。
- `skillsApi.versions` 改为明确「未实现」或对接真实端点，不再伪造数组。
- 不改任何运行时行为（纯类型卫生）。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|---------|--------|---------------|
| EVO-058 | 前端死代码与类型卫生清理 | 无 | P2 | 无硬依赖 |

## 3. 发布计划基线：不做事项

- 不清理后端死代码（归 EVO-051，已完成）。
- 不改 billing/i18n 缺口（随对应 feature 迭代处理）。
- 不重写 API client 拦截器逻辑。
- 不改任何运行时行为。

## 4. 发布计划基线：计划验收标准

- Story 格式与 BDD 适用性：
  - [x] Technical Story；BDD 不适用，使用命令级验收。
- [ ] `src/types/`、`uiStore.ts`、`accept-invitation/page.tsx` 删除且无残留 import。
- [ ] `rg "as Theme|as TokenRole|getElementById\('root'\)!" frontend/src` 无不安全用法（或已加守卫）。
- [ ] `lib/api/types.ts` 仅有单一 `User` 定义。
- [ ] `skillsApi.versions` 不再返回伪造数组。
- [ ] 前端类型检查与构建通过（`bun run build` 0 错误）。

## 5. 发布计划基线：计划验证

```bash
cd frontend
bun run build
tsc --noEmit
rg "as Theme|as TokenRole|getElementById\('root'\)!" src/
rg "from.*types/" src/
rg "from.*uiStore" src/
rg "accept-invitation" src/
git diff --check
```

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| 死类型被间接引用（barrel export） | 删除前先 grep 全量引用，逐个清理 |
| 不安全断言修正引入运行时分支 | 用类型守卫替代，不改变控制流 |
| skillsApi.versions 改为 throw 影响调用方 | 检查所有调用点，确认无生产代码依赖 |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 清除前端死类型 + 不安全断言修正 + 假实现修复 |
| 产物 | 代码删除 + 类型修正 |
| 状态同步归口 | EVO-058、Iteration 040、iterations/README.md |
| Story/BDD 归口 | Technical Story；命令级验收 |
| 验证证据 | bun run build + tsc + rg 检查 |
| 残余工作归口 | billing/i18n 缺口随 feature 迭代 |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-06-04 | activation | 状态 → Active / In Progress。前置：Iteration 039 Closed，无 Active/Review 迭代。EVO-058 Ready → In Progress。委托 deep agent 实施。 |
| 2026-06-04 | execution | Deep agent 完成实施：删除 7 个死代码文件（-139 行）+ stores/index.ts re-export 移除 + skills.ts versions() 假实现移除 + types.ts 重复 User 接口移除。 |
| 2026-06-04 | closure | 独立验证通过：bun run build 0 errors / tsc --noEmit 0 errors / 3 项 rg 清理检查全绿。状态 → Closed。 |

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|

## 10. Review

- 完成：
  - 删除 7 个死代码文件（-139 行）：src/types/ 目录（5 文件）+ uiStore.ts + accept-invitation/page.tsx
  - stores/index.ts 移除 useUIStore re-export
  - lib/api/skills.ts 移除 versions() 假实现（0 调用方，包装单 skill 冒充版本列表）
  - lib/api/types.ts 移除重复 User 接口（保留完整定义，含 tenant_id/tenant_role/email_verified）
- 未完成：无
- 验证结果：
  - `bun run build` → 0 errors ✅
  - `tsc --noEmit` → 0 errors ✅
  - `rg "from.*types/"` → 0 hits ✅
  - `rg "from.*uiStore"` → 0 hits ✅
  - `rg "skillsApi\.versions"` → 0 hits ✅
  - `rg "accept-invitation"` → 2 expected hits（路由定义 + LayoutWrapper 公开路由列表，非死代码）✅
- 闭环状态：`Complete`
- 残余归口：不安全 as cast（theme.tsx / usePermission.ts / main-spa.tsx）保留为已知技术债，非死代码，不在 EVO-058 范围

## 11. Retrospective

- 做得好的：
  - Explore agent 提供了完整的引用计数分析，确认 7 个文件确实零引用
  - skillsApi.versions 假实现识别准确（0 调用方 + 包装单 skill 冒充数组）
  - 重复 User 接口合并保留了完整定义，不破坏现有 17 个引用方
- 需要调整的：
  - 无
- 写入 EVOLUTION：无新陷阱。本次为纯清理，未触发代码变更或流程问题。
