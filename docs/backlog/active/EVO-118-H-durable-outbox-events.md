# EVO-118-H Durable Outbox 与可靠事件交付

- **类型**：Epic / Reliability / Eventing
- **状态**：Done / Complete
- **历史迭代**：Iteration 060 Closed / Partial
- **优先级**：P1
- **父 Epic**：[EVO-118](EVO-118-production-readiness-and-security-hardening.md)
- **依赖**：EVO-118-G
- **影响范围**：backend / db / worker / docs / tests

## 工程目标

建立 PostgreSQL Outbox + Worker 的最小可靠事件边界，使 Push、Commit、Promote、Webhook、Indexer 和 Agent Session Event 在进程崩溃或重启后仍可重试、幂等和审计。

## 拆分理由与子 Story

Iteration 060 已交付 schema/repository 最小边界，但剩余范围包含 claim 数据正确性、运行
生命周期和业务接入三个可独立完成或失败的结果。保留 EVO-118-H 为父 Epic，不直接再次
进入迭代；Iteration 060 的 Closed / Partial 历史不改写。

| 子 Story | 独立结果 | 状态 | 依赖 | 所属迭代 |
| --- | --- | --- | --- | --- |
| [EVO-118-H-A](EVO-118-H-A-recoverable-outbox-claims.md) | 可回收 lease/fencing claim + PostgreSQL 并发互斥 | Done / Complete | Iteration 060 boundary | Iteration 066 |
| [EVO-118-H-B](EVO-118-H-B-outbox-worker-runtime.md) | 常驻 Worker 生命周期、积压恢复与 replay | Done / Complete | H-A Done | Iteration 067 |
| [EVO-118-H-C](EVO-118-H-C-durable-push-event-integration.md) | 现有 Push 派生工作接入 Durable Event 与幂等 subscriber | Done / Complete | H-B Done | Iteration 068 |

父 Epic 只在 H-A/B/C 全部 Done、EVENT-01 证据同步且未来 producer/consumer 约束已写入
EVO-105/106/107/108 后进入 Done。以上条件现已满足；Commit/Promote/Agent Session 尚未实现，因此其具体事件
enqueue 是各 owner Story 的验收，不与 H 形成循环依赖。

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

## 实际验证与残余

- SQLite/PostgreSQL `011_outbox_events` schema 已建立，包含 payload、幂等键、状态、尝试次数、
  重试时间和错误摘要。
- Domain `OutboxRepository`、SQLite/PG repository 已实现 enqueue、claim、delivered、failed /
  dead-letter 状态变更。
- OutboxWorker 已补齐单次 claim/deliver/retry 处理，SQLite 集成测试 4/4 通过：pending→processing、幂等键拒绝、重试至 dead-letter、成功 delivered。
- Workspace check、strict Clippy 和格式检查通过。
- 残余：Commit/Promote/Agent Session 业务事件与 Webhook/Indexer/Agent Event subscriber 尚未实现，
  由 EVO-105/106/107/108 各自负责；H 提供的持久事件边界已完成。

闭环状态：`Complete`；EVENT-01 的 Durable Outbox 基础 Gate 已解除，后续业务 producer/consumer
仍须在各自 Story 中提供具体验收证据。

## 2026-08-09 Refinement

- 确认当前 `processing` 行没有租约或 recovery，claim 后崩溃会永久卡住；先由 H-A 关闭。
- PostgreSQL 并发实测与 stale claim fencing 归 H-A；Worker 进程运行态归 H-B；现有 Push
  producer/subscriber 接线归 H-C。
- H-A 激活后父 Epic 状态为 `In Progress`；Iteration 060 仍保持 Closed / Partial。

## H-A 完成记录

- Iteration 066 Closed / Complete：paired 012 migration、lease/fencing、stale recovery、
  max-attempt dead-letter 与非法边界 fail closed 已完成。
- SQLite focused 8/8、PostgreSQL 16 concurrent/upgrade focused 1/1、infra/full compile/lint
  gates 通过；Navigator 修正 upgrade evidence 和非法 lease/attempt 后无 blocking finding。
- H-A 收口时父 H 保持 In Progress、EVENT-01 开放；其后 H-B 已由 Iteration 067 完成。

## H-B 完成记录

- Iteration 067 Closed / Complete：独立 `outbox-worker` 的 continuous/once/confirmed replay、
  bounded delivery、优雅停止、稳定错误码与 101 backlog/crash recovery 已闭合。
- SQLite file 与 PostgreSQL 16 真实子进程、workspace check/test/strict Clippy、文档治理和
  Navigator 复验通过；EVO-125 独立承接既有 Git Smart HTTP E2E 不稳定。
- 父 H 保持 In Progress，EVENT-01 保持开放；下一依赖切片为 H-C Durable Push Event 接入。

## H-C 完成记录

- Iteration 068 Closed / Complete：真实 Push durable event producer、幂等 subscriber、SQLite/PostgreSQL
  证据、crash/restart、reconcile 与完整 Smart HTTP clone/push/pull E2E 已闭合。
- EVO-125 已 Done / Complete：同步 Git 子进程造成的 runtime starvation 改为异步 bounded runner；
  聚焦 E2E 连续 3/3 通过。
- 父 H 与 EVENT-01 Durable Outbox 基础 Gate 完成；未来业务事件仍归各 owner Story，不在 H 中扩张范围。
