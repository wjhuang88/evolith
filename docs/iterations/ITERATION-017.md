# Iteration 017: 前端 CLI Interface 概念收口

> 文档状态：Planned / Blocked for activation
> 计划发布日期：2026-05-27
> 计划目标：完成 Vite SPA 中仍暴露给用户的 Snippet 概念迁移，使前端入口与
> CLI-friendly interface 产品方向一致。
>
> 基线保护：本文件一旦提交，以下“发布计划基线”内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

本轮候选承接 `EVO-017` 已确立的 CLI-friendly interface 产品方向，处理前端仍保留
`/snippets` 页面、API client 和可见文案的问题。该计划只建立执行基线，不在本次
规划中启动实现。

激活门禁：

- 先处置 [Iteration 009](ITERATION-009.md) 与 [Iteration 010](ITERATION-010.md) 的
  `Review` 收口缺口。
- 激活前重新核对 `EVO-026` 仍满足 DoR，且没有新的依赖或范围变化。
- 明确旧 `/snippets` 路由采用兼容跳转、别名保留或下线策略。

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
| 请求结果 | future plan only；未启动实现 |
| 产物 | 激活后应交付前端页面、路由/API 边界和 i18n 概念收口 |
| 状态同步归口 | EVO-026、Iteration 017、API contract/reference（如适用） |
| 验证证据 | 前端 type-check/build、代码搜索与关键路由手工回归 |
| 残余工作归口 | 后端 legacy snippet 移除或完整 CLI interface API 如超出范围则另建 story |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-05-27 | planning | 发布 future plan 基线；EVO-026 总表与详情块已同步为 `Ready`，但因 Iteration 009 / 010 仍为 `Review`，本迭代暂不可激活。 |

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|

## 10. Review

- 完成：
- 未完成：
- 验证结果：
- 闭环状态：`Blocked`（仅计划发布，尚未激活）
- 残余归口：激活门禁见第 1 节。

## 11. Retrospective

- 做得好的：
- 需要调整的：
- 写入 EVOLUTION：
