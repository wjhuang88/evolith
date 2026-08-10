# EVO-118-H-C Durable Push Event 业务接入

- **类型**：Technical / Reliability / Integration
- **状态**：Proposed
- **优先级**：P1
- **父 Epic**：[EVO-118-H](EVO-118-H-durable-outbox-events.md)
- **依赖**：EVO-118-H-B Done；现有 Git Smart HTTP Push 路径
- **影响范围**：backend / git / db / worker / tests / docs

## 工程目标

把现有 Push 后派生工作从可丢失的进程内异步边界迁移到 Durable Outbox，并建立类型化
subscriber/idempotency contract，使未来 Webhook/Indexer 可以复用同一可靠边界。

## 不做事项

- 不提前实现尚不存在的 Commit/Promote/Agent Session producer；这些由 EVO-105/EVO-106
  在各自业务事务中 enqueue。
- 不实现 Webhook 外发或 Indexer Parser；归 EVO-107/EVO-108。

## 技术验收

- [ ] Push 成功后持久化结构化事件；失败路径不返回假成功，Git/DB 分裂有明确 reconcile 归口。
- [ ] 现有 push 后 metadata 派生工作消费 Durable Event，不再依赖可丢失的 fire-and-forget task。
- [ ] subscriber 使用 idempotency key，重复投递不产生重复业务副作用。
- [ ] crash/restart、重复投递、事务/补偿失败和 payload secret 负向测试通过。
- [ ] EVO-105/EVO-106/EVO-107/EVO-108 的事件生产/消费契约和依赖同步。

## 最小验证

- 真实 Git Push E2E + crash/restart + duplicate delivery。
- SQLite/PostgreSQL repository/integration、workspace tests、strict Clippy。

## 残余工作归口

- Commit/Promote producer 归 EVO-105；Agent Session Event 归 EVO-106；Webhook/Indexer consumer
  归 EVO-107/EVO-108。
