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
| EVO-001 | 前后端 API 对齐 | P0 | Agent/Human | Planned |

## 3. 不做事项

- 不迁移 Next.js 到 Vite。
- 不实现忘记密码/邀请加入完整流程。
- 不实现 MCP 工具真实执行。
- 不调整视觉设计。

## 4. 验收标准

- [ ] 前端 API client 不再调用后端不存在的工具/技能/片段 route。
- [ ] tool update method 和后端一致。
- [ ] auth change password 字段和后端一致。
- [ ] snippet create/update 字段和后端 DTO 对齐。
- [ ] `npm run type-check` 通过。
- [ ] `npm run build` 通过。
- [ ] 后端相关测试至少运行 `cargo test -p api`。

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

## 8. Review

- 完成：
- 未完成：
- 验证结果：

## 9. Retrospective

- 做得好的：
- 需要调整的：
- 写入 EVOLUTION：
