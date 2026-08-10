# EVO-118-H-A 可回收 Outbox Claim 与 PostgreSQL 并发语义

- **类型**：Technical / Reliability / Data Integrity
- **状态**：Done / Complete
- **优先级**：P1
- **父 Epic**：[EVO-118-H](EVO-118-H-durable-outbox-events.md)
- **当前迭代**：[Iteration 066](../../iterations/ITERATION-066.md)
- **依赖**：Iteration 060 的双数据库 Outbox schema/repository 最小边界
- **影响范围**：backend / db / tests / docs

## 工程目标

让已 claim 的 Outbox Event 具备有界租约和 fencing token：Worker 在 ACK 前崩溃后，过期
事件可以被重新领取；旧 Worker 不能在事件被新 Worker 领取后覆盖最终状态。PostgreSQL
生产路径必须证明多个 Worker 并发 claim 不重复，SQLite 明确保持单进程 Lite 语义。

## Governing Decisions And Constraints

- [Security Review SOP](../../sop/SECURITY-REVIEW.md)：Durable Event 必须证明崩溃/重启后不丢、重试有界和幂等。
- [Database Migration SOP](../../sop/DATABASE-MIGRATION.md)：schema/repository 行为同时覆盖 SQLite/PostgreSQL。
- [Production Readiness Baseline](../../reference/PRODUCTION-READINESS-BASELINE.md)：EVENT-01 在完整 H 关闭前保持开放。
- Hard：PostgreSQL 是生产主路径；并发 claim 依赖行锁/`SKIP LOCKED`，同一事件一次租约内只能返回给一个 Worker。
- Hard：过期 claim 必须可恢复；旧 claim 的 ACK/NACK 必须 fail closed，不能覆盖新 claim。
- Soft：租约字段和 token 命名遵循现有 Outbox 模型，不引入外部消息系统。
- Assumption：SQLite 仅承担单进程开发/测试 Lite 路径；用明确测试和文档约束，不宣称多 Worker 并发能力。

## 不做事项

- 不启动常驻 Worker、不新增进程运行模式；归 EVO-118-H-B。
- 不接入 Push/Commit/Promote 或 Webhook/Indexer subscriber；归 EVO-118-H-C 及各业务 Story。
- 不实现手动 dead-letter replay API、metrics dashboard 或 100+ 事件基准；归 EVO-118-H-B。
- 不引入 Kafka、NATS 或跨区域 exactly-once。

## 技术验收

- [x] 新 migration 为 SQLite/PostgreSQL 同时增加 claim token 与 lease expiry，不改写已发布的 `011` migration。
- [x] claim 原子写入 `processing`、attempts、claim token、lease expiry；返回值包含本次 token。
- [x] lease 到期且 attempts 未耗尽时可被新 token 重新 claim；达到上限的过期 processing 事件进入 dead-letter。
- [x] `mark_delivered` / `mark_failed` 仅接受当前 claim token；旧 token 返回冲突且不改变状态。
- [x] SQLite integration 覆盖 crash/reclaim、stale ACK/NACK fencing 和 max-attempt dead-letter。
- [x] PostgreSQL integration 真实运行两个并发 claimer，证明同一批事件无重复、无遗漏，并覆盖 stale claim recovery。
- [x] domain/infra tests、format、workspace check 与 strict Clippy 通过。

## 最小验证

```bash
cd backend
cargo test -p infra --test outbox_repo_tests
TEST_POSTGRES_URL=postgres://evolith_test:dev_password@127.0.0.1:5432/evolith_test \
  cargo test -p infra --test pg_outbox_claim_tests -- --nocapture
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
```

## 失败模型与回滚

| 失败 | 安全默认 / 证据 |
| --- | --- |
| Worker claim 后崩溃 | lease 到期后重新 claim；事件仍在数据库 |
| 旧 Worker 延迟 ACK/NACK | token 不匹配返回 conflict；不覆盖新 owner 状态 |
| 多个 PostgreSQL Worker 同时 claim | `FOR UPDATE SKIP LOCKED` + 原子 update；结果集合互斥 |
| 最后一次 attempt 中崩溃 | lease 到期后转 dead-letter，不永久 processing |

回滚只撤回尚未发布的 `012` migration、repository/model 签名和对应测试；不改写 Iteration
060 已建立的 `011` 基线。

## 解锁内容

- EVO-118-H-B 可以依赖可恢复的 claim 协议接入常驻 Worker 生命周期。
- EVO-118-H-C 可以在不依赖进程内 `spawn` 的前提下接入现有 Push 事件。

## 残余工作归口

- Worker 生命周期、配置、积压恢复和 replay：EVO-118-H-B。
- Push 事务/补偿事件与通用 subscriber contract：EVO-118-H-C。
- Commit/Promote 与 Agent Session 产生对应 Outbox Event：EVO-105/EVO-106 的验收范围。

## 实际验证与闭环

- paired `012_outbox_claim_leases` migration 已增加 nullable token/lease 字段与索引；SQLite、
  PostgreSQL 均从存在事件的 `011` 升级到 `012` 并证明状态/数据保留。
- SQLite focused 8/8：claim、租约内不重复、crash/reclaim、stale ACK/NACK、最后 attempt
  dead-letter、非法 attempts/lease fail closed 与 upgrade preservation。
- PostgreSQL 16 focused 1/1：两个并发 claimer 各取 5/10，集合互斥且覆盖全部事件；
  stale ACK/NACK fencing、reclaim 和 `011 → 012` preservation 通过。
- `cargo test -p infra`、format、workspace all-targets check、strict Clippy 通过。
- Navigator 首轮发现 migration upgrade 未被证明、repository 可接受非法 attempt/lease；
  修复并复验后无 blocking finding。Residual risk：H-B 必须协调 lease 与 delivery timeout。

闭环状态：`Complete`。父 EVO-118-H 与 EVENT-01 仍未完成；H-B 已在 Iteration 067 完成，当前下一依赖切片是 H-C。
