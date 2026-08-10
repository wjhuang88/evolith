# Iteration 060: Durable Outbox Boundary

> 文档状态：Closed / Partial
> 计划目标：建立双数据库 Outbox schema/repository 的最小可靠事件边界。
> 本轮不执行最终生产构建/部署；EVO-118-E 仍是所有产品和 legacy cleanup 完成后的最后 Gate。

## 1. 发布计划基线

- 故事：[EVO-118-H](../backlog/active/EVO-118-H-durable-outbox-events.md)
- 依赖：EVO-118-G-A/B/C/D 已完成本地验收；G-A 远端 Branch Protection 仍有外部 403 residual。
- 不做：Kafka/微服务、最终部署、完整 Webhook/Indexer UI。

## 2. 验收与验证

- SQLite/PostgreSQL schema 与 repository 字段一致。
- enqueue 保持 pending；claim 增加 attempts 并进入 processing。
- 幂等键唯一；失败按 max_attempts 转 dead-letter。
- 验证 domain、SQLite integration、PG compile/upgrade、Worker 状态测试、workspace check/clippy。

## 3. 执行记录

| 日期 | 类型 | 记录 |
| --- | --- | --- |
| 2026-08-08 | activation | Iteration 059 Closed / Complete；G-D 完成后激活 H。 |
| 2026-08-08 | validation | Outbox repository 4/4 与 Worker success/retry 状态测试通过；PG repository 编译通过。 |
| 2026-08-08 | change request | 用户要求继续全部已规划产品开发；类型：scope-change；决策：本迭代以 Outbox schema/repository 最小边界 Partial 收口，未完成的运行态与业务接入继续归 EVO-118-H，切换到已产生代码的 EVO-112-B 并用新编号补齐治理与验收。 |
| 2026-08-08 | completion | Iteration 060 Closed / Partial；EVO-118-H 保持 Review / Partial，EVENT-01 不关闭。 |

## 4. 闭环台账

| 项目 | 记录 |
| --- | --- |
| 产物 | `011_outbox_events`、Outbox domain/repository、SQLite/PG 实现。 |
| 证据 | SQLite outbox/retry tests 4/4；workspace check/clippy 通过。 |
| 残余 | PG 并发实测、Worker 生命周期/崩溃恢复、业务事件接入归 H 后续 slice。 |
| 状态 | `Partial`。 |

## 5. Review

- 完成：双数据库 Outbox schema/repository、基础 Worker 状态机与 SQLite 集成测试。
- 未完成：PostgreSQL 并发 claim、Worker 进程生命周期与崩溃恢复、业务事务和 subscriber 接入。
- 闭环状态：`Partial`；残余均保留在 EVO-118-H，未关闭 EVENT-01。
- 切换处置：本 Iteration 不再保持 Active；Iteration 061 承接 EVO-112-B，避免两个 In Progress Iteration 并存。
