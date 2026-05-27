# Iteration 011: Members 与 API Keys 接真实 API（改线执行）

> 状态：Done ✅
> 实际执行目标（偏差）：EVO-010 (Members) + EVO-011 (API Keys) 前端页面接真实 API。
> 完成日期：2026-05-27
>
> 流程偏差：本编号于 2026-05-26 已发布为 Embedded Frontend refinement 计划，
> 后于 2026-05-27 被直接改线承载 EVO-010/EVO-011。本页保留实际执行事实并补录
> 原基线；原 EVO-016 目标未完成。

## 0. 原发布计划基线（补录，不代表已执行）

| 项目 | 2026-05-26 已发布内容 |
|------|-----------------------|
| 原计划标题 | Embedded Frontend 交付边界确认 |
| 原计划目标 | 把 EVO-016 从远期方向细化为可安全实施的 embedded frontend 交付切片，确定生产运行时配置与后端静态服务边界。 |
| 原候选 Story | `EVO-016-A` Embedded Frontend 交付边界与运行时配置确认（待建项） |
| 原进入条件 | EVO-022 至 EVO-025 已 Done；需先将 EVO-016 按 Epic/Story 规则细化并通过 DoR。 |
| 原不做事项 | 不直接实现 Rust 文件服务、单二进制嵌入或 CI/CD。 |
| 处理结论 | 原计划未执行；本轮 Done 不表示 EVO-016 获得任何实施进度。 |

## 1. 改线执行边界

EVO-010 和 EVO-011 合并本轮实施。EVO-010 后端缺 `UserRepository::find_by_tenant` 和 `remove_member` 实现，需先修后端再接前端。EVO-011 后端完整，直接接前端。

## 2. 改线选入故事

| ID | 标题 | 优先级 | 当前状态 |
|----|------|--------|----------|
| EVO-010 | 租户 Members 页面接真实 API | P1 | In Progress |
| EVO-011 | API Key 页面接真实 API | P1 | In Progress |

## 3. 实际范围

- EVO-011：API Keys 页面接真实 API（后端已完整，纯前端接线）。
- EVO-010：Members 页面接真实 API（后端需补 `find_by_tenant` + `remove_member`，再接前端）。

## 4. 实际不做事项

- 不做 EVO-016 embedded frontend（仍 Deferred）。
- 不做 EVO-012（依赖 EVO-016）。
- 不做 EVO-030 CI/CD。

## 5. 实际验收结果

- [x] `GET /api/v1/tenant/{id}/members` 返回真实成员列表。
- [x] `DELETE /api/v1/tenant/{id}/members/{mid}` 实际移除成员。
- [x] API Keys 页面展示真实数据，创建/撤销均可用。
- [x] Members 页面展示真实数据，邀请/移除均可用。
- [x] `cargo test --workspace` 通过。
- [x] `bun run build` 通过。

## 6. 实际验证

```bash
cd backend && cargo test --workspace
cd frontend && bun run build
```

## 7. 执行风险与进入条件

| 风险 | 计划控制 |
|------|----------|
| `tenant_id` 不可为 NULL，remove_member 无法直接清空 | 读 migration 确认后选择方案 |
| 前端 API 层类型与后端不匹配 | 严格对照 backend DTO |

## 8. 执行记录与偏差

| 日期 | 记录 |
|------|------|
| 2026-05-26 | Future iteration planned for EVO-016 refinement. |
| 2026-05-27 | Repurposed for EVO-010 + EVO-011 (higher priority P1 stories). Backend fix and frontend wiring in parallel. |
| 2026-05-27 | Process deviation recorded: original EVO-016 baseline restored for traceability; dependent Iteration 012 is blocked until EVO-016 refinement is replanned under a new iteration number. |
| 2026-05-27 | EVO-010 后端完成：UserRepository 加 find_by_tenant + remove_from_tenant，list_members + remove_member handler 实例化。254 tests passed。 |
| 2026-05-27 | EVO-011 前端完成：API Keys 页面接真实 API，api-keys.ts module + types + i18n。 |
| 2026-05-27 | EVO-010 前端完成：Members 页面接真实 API，members.ts module + types + i18n + tabbed UI。 |
| 2026-05-27 | Iteration 011 Done。`cargo test --workspace` → 254 passed, `bun run build` → 0 errors。 |
