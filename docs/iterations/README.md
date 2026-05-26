# Iterations 目录

本目录记录每轮迭代计划、执行和复盘。

## 文件

- [迭代模板](ITERATION-TEMPLATE.md)
- [Iteration 001](ITERATION-001.md) — 工程化文档体系与流程改造。
- [Iteration 002](ITERATION-002.md) — CLI 友好接口概念迁移。
- [Iteration 003](ITERATION-003.md) — 前端路由适配层。
- [Iteration 004](ITERATION-004.md) — 前端技术栈迁移 React + Vite + Bun（EVO-022~025）。
- [Iteration 005](ITERATION-005.md) — 认证闭环：忘记密码与邀请接受（EVO-003/004）。
- [Iteration 006](ITERATION-006.md) — MCP 工具真实执行（EVO-005）。
- [Iteration 007](ITERATION-007.md) — MCP 工具执行质量修复与流程防呆（EVO-032）。
- [Iteration 008](ITERATION-008.md) — Epic 与子需求拆分治理规则（EVO-034）。

## 未来计划

以下文档仅为排期草案，未启动实施，也不代表候选事项已进入 `In Progress`：

- [Iteration 009](ITERATION-009.md) — 邮箱验证闭环（候选 EVO-018）。
- [Iteration 010](ITERATION-010.md) — Skill 更新与 CLI Interface 格式基线（候选 EVO-006 / EVO-009）。
- [Iteration 011](ITERATION-011.md) — Embedded Frontend 交付边界确认（EVO-016 待拆分）。
- [Iteration 012](ITERATION-012.md) — Embedded Frontend 文件服务与单容器交付（依赖 EVO-016 refinement）。

## 命名

```text
ITERATION-001.md
ITERATION-002.md
...
```

## 推荐节奏

- 常规迭代：1 周。
- Agent 微迭代：一次会话只完成一个 backlog story 或一个明确切片。

## 状态同步

迭代开始：

1. 先按 [开始一次迭代 SOP](../sop/START-ITERATION.md) 执行固定检查。
2. 从 [Product Backlog](../backlog/PRODUCT-BACKLOG.md) 选择 Ready 项。
3. 创建 `ITERATION-<N>.md`。
4. 明确本轮不做什么。

迭代结束：

1. 更新完成情况和验证结果。
2. 更新 backlog item 状态。
3. 新经验写入 `EVOLUTION.md`。
