# Iteration 001: API 对齐与前端迁移准备

> 时间：2026-05-15 起
> 目标：先让现有前后端接口对齐，为后续 React + Vite + Bun 迁移清理风险。

## 1. 本轮目标

- 修正当前控制台中最容易导致功能不可用的 API method/字段不一致。
- 明确前端迁移前必须冻结的接口边界。
- 不在本轮实施 Vite 迁移，只做迁移准备和接口校准。

## 2. 选入故事

| ID | 标题 | 优先级 | 负责人 | 状态 |
|----|------|--------|--------|------|
| EVO-001 | 前后端 API 对齐 | P0 | Agent/Human | Done |

## 3. 不做事项

- 不迁移 Next.js 到 Vite。
- 不实现忘记密码/邀请加入完整流程。
- 不实现 MCP 工具真实执行。
- 不调整视觉设计。
- 不继续扩展 snippet 细节；snippet 已在迭代中被新产品方向替代为 CLI 友好接口。

## 4. 验收标准

- [x] 前端 API client 不再调用后端不存在的工具/技能 route。
- [x] tool update method 和后端一致。
- [x] auth change password 字段和后端一致。
- [x] snippet 相关未完成对齐项已记录为变更请求，并转入 EVO-017。
- [x] `npm run type-check` 通过。
- [x] `npm run build` 通过。
- [x] 后端相关测试至少运行 `cargo test -p api`。

## 5. 验证计划

```bash
cd frontend
npm run type-check
npm run build

cd ../backend
cargo test -p api
```

## 6. 风险与回滚

| 风险 | 处理 |
|------|------|
| 前端页面依赖旧字段 | 先在 API client 做适配层，避免大范围页面重写 |
| 后端 DTO 本身不稳定 | 将差异记录回 backlog，不在本轮扩大范围 |
| 构建暴露更多类型错误 | 只修与 EVO-001 相关的错误，其余进入 backlog |

## 7. 执行记录

| 日期 | 记录 |
|------|------|
| 2026-05-15 | 创建迭代计划，选入 EVO-001。 |
| 2026-05-15 | 开始按迭代流程执行 EVO-001，先做 API client 与后端 DTO/route 差异排查。 |
| 2026-05-15 | 迭代中收到产品方向变更：旧 snippet 概念改为 CLI 友好接口。停止扩大 snippet 对齐工作，新增 EVO-017 和 ADR-0002。 |
| 2026-05-15 | `npm run type-check` 首次验证失败：`frontend/src/lib/api/tools.ts` 中 list meta 可能为 undefined。已作为当前开发中暴露的问题修正。 |
| 2026-05-15 | 按防呆反馈补充 AGENTS、迭代 SOP、backlog 规则和迭代模板中的中途变更控制检查表。 |
| 2026-05-15 | 验证通过：`npm run type-check`、`npm run build`、`cargo test -p api`。 |

## 8. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|
| 2026-05-15 | product-pivot | 接受，新增 EVO-017 和 ADR-0002 | 旧 snippet 对齐不再作为 EVO-001 验收目标，转为 CLI 友好接口迁移输入 | 保留通用 API client 修复；snippet 适配代码仅作为临时兼容，后续由 EVO-017 清理 |

## 9. Review

- 完成：EVO-001 前端 API client 对齐；中途产品变更按 change request 处理；新增 ADR-0002 和 EVO-017；补充防呆流程。
- 未完成：旧 snippet 语义迁移未在本轮实现，已转入 EVO-017。
- 验证结果：`npm run type-check` 通过；`npm run build` 通过但有既有 React Hook dependency warnings 和 Next static generation localStorage warning；`cargo test -p api` 通过，含 22 unit tests、12 auth E2E、0 doctest。

## 10. Retrospective

- 做得好的：Backlog/Iteration/ADR 的组合能承接中途 product pivot，避免继续扩大旧 snippet 方向。
- 需要调整的：原迭代 SOP 缺少“迭代中需求变更”的明确入口；已补充 change request 判断、暂停规则、半成品处理和防呆检查表。
- 写入 EVOLUTION：已写入“迭代中需求变更需要明确变更控制入口”。
