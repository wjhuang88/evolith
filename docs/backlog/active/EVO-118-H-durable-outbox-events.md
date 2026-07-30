# EVO-118-H Durable Outbox 与可靠事件交付

- **类型**：Technical / Reliability / Eventing
- **状态**：Proposed
- **优先级**：P1
- **父 Epic**：[EVO-118](EVO-118-production-readiness-and-security-hardening.md)
- **依赖**：EVO-118-E、EVO-118-G
- **影响范围**：backend / db / worker / docs / tests

## 工程目标

建立 PostgreSQL Outbox + Worker 的最小可靠事件边界，使 Push、Commit、Promote、Webhook、Indexer 和 Agent Session Event 在进程崩溃或重启后仍可重试、幂等和审计。

## 已确认失败模式

- Push 后元数据同步使用进程内异步任务，失败只记录 warning。
- Webhook/Indexer 尚未实现，但现有路线图未明确持久事件前置依赖。
- 进程在 Git 操作成功后、异步任务完成前退出会永久丢失派生工作。
- 没有统一 delivery 状态、重试上限、幂等 Key、死信或 Reconcile 入口。

## 验收场景

### Scenario 1：事件不会因重启丢失

- **Given** Git Push/Commit/Promote 已成功并创建 Outbox Event
- **When** Worker 在发送前崩溃并重启
- **Then** Event 仍待处理并最终成功交付一次逻辑结果

### Scenario 2：重复投递幂等

- **Given** Worker 在 ACK 前崩溃导致同一 Event 重试
- **When** Webhook/Indexer 重复接收
- **Then** 通过 idempotency key 不产生重复索引或重复业务副作用

### Scenario 3：失败有界且可观察

- **Given** 外部目标持续失败
- **When** 达到最大重试
- **Then** Event 进入 failed/dead-letter 状态，保留错误摘要和手动重放入口

### Scenario 4：事件与元数据一致

- **Given** DB 元数据更新和事件生成属于同一业务动作
- **When** 事务提交或回滚
- **Then** 不出现“状态已改但没有事件”或“事件存在但状态未提交”

## 工程要求

- 增加双数据库兼容的 Outbox schema/repository；生产主路径使用 PostgreSQL 锁/claim 语义。
- 事件至少包含：ID、类型、Aggregate、Payload、Idempotency Key、状态、尝试次数、下次执行时间、错误摘要和时间戳。
- Worker 可作为同仓库独立运行模式，不要求拆微服务或引入 Kafka。
- Push/Commit/Promote 的 DB 元数据与 Outbox 写入尽可能同事务；Git 文件系统操作使用可重建/Reconcile 语义。
- Webhook、Indexer、Audit/Agent Event 订阅统一事件，不在 Handler 中散落 `spawn`。
- 配置退避、最大重试、超时、并发和死信重放。
- 记录 metrics/logs，避免 Payload 中持久化 Secret。

## 不做事项

- 不引入 Kafka、NATS 或复杂事件总线作为 MVP 前置。
- 不实现跨区域 exactly-once；目标是 at-least-once + 幂等。
- 不在本 Story 完成 Webhook UI 或 Indexer Parser。

## 最小验证

- Worker 崩溃/重启、重复 claim、超时和最大重试测试。
- 同一 Event 重放幂等测试。
- DB 事务回滚时无 Outbox Event 测试。
- PostgreSQL 并发 Worker claim 测试；SQLite Lite 行为有明确限制和测试。
- 100+ 事件吞吐与积压恢复基准。
- EVO-107/108 使用本边界的集成契约更新。

## 解锁内容

解除 EVENT-01 Gate；为 Webhook Out、Indexer、Agent Session Event 和审计派生工作提供可靠基础。
