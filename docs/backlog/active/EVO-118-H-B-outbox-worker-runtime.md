# EVO-118-H-B Outbox Worker 运行生命周期

- **类型**：Technical / Reliability / Operations
- **状态**：Done / Complete
- **优先级**：P1
- **父 Epic**：[EVO-118-H](EVO-118-H-durable-outbox-events.md)
- **依赖**：EVO-118-H-A Done
- **当前迭代**：[Iteration 067](../../iterations/ITERATION-067.md)
- **影响范围**：backend / worker / config / tests / docs

## 工程目标

把 OutboxWorker 接入同仓库可运行生命周期，具备优雅停止、崩溃重启、超时、退避、批量、
积压恢复、可观察失败和受控 dead-letter replay，并证明 100+ 事件可恢复处理。

这是基础运行态 Story：当前业务 producer/subscriber 尚未接入，但交付物必须是实际可执行
的独立 Worker 进程和进程级数据库证据，不能只交付未接线 library helper。

## Governing Decisions And Constraints

- [Security Review SOP](../../sop/SECURITY-REVIEW.md)：Durable Event 必须证明崩溃/重启后不丢、重试有界和幂等。
- [Architecture](../../reference/ARCHITECTURE.md)：保持模块化单体；Worker 可同仓库独立运行，不引入 Kafka/微服务。
- [Production Readiness Baseline](../../reference/PRODUCTION-READINESS-BASELINE.md)：H-B 完成不单独关闭 EVENT-01。
- Hard：一次 batch 的最坏 delivery timeout 总和必须小于 claim lease；否则旧 lease 可在当前 batch 完成前被回收。
- Hard：未知 event type、数据库错误或配置错误 fail closed；不得标记 delivered 或静默退出 0。
- Hard：日志和 `last_error` 不记录 payload 或 handler 原始错误文本，只记录 event metadata 与稳定错误码。
- Soft：MVP 使用同仓库独立二进制；H-C 可在不改变运行契约的前提下注册真实 handler。
- Assumption：SQLite 仅用于单进程 Lite；PostgreSQL 是生产并发路径，需真实进程测试验证。

## 不做事项

- 不接入具体 Push/Commit/Promote producer；归 EVO-118-H-C 和业务 Story。
- 不实现 Webhook UI、Indexer Parser 或独立微服务。
- 不把未知业务事件当作成功；本轮仅内置无副作用 `system.outbox.probe` 诊断 handler。

## 技术验收

- [x] `outbox-worker run` / `run --once` 可执行并优雅处理 SIGINT；不使用不可追踪的 handler `spawn` 作为事件边界。
- [x] 配置 batch、poll、lease、delivery timeout、default max attempts/backoff，并验证 `batch × timeout < lease` 与正数/上限边界。
- [x] delivery timeout/handler failure 进入 retry/dead-letter，`last_error` 和日志仅含稳定错误码，不含 payload 或原始 secret。
- [x] 进程级崩溃遗留 claim 恢复；SQLite 与 PostgreSQL 各 101 条 backlog 在一次 `run --once` 中排空。
- [x] `replay <event-id> --confirm` 只允许 dead-letter → pending；缺少确认、非 dead-letter 或无效 ID 非零退出且不改状态。
- [x] 未知 event type fail closed；`system.outbox.probe` 是本轮唯一成功 handler。
- [x] 配置、CLI、SQLite Lite/PostgreSQL production 边界和 H-C 接入责任写入稳定文档。

## 最小验证

- focused worker/runtime/config/CLI tests、进程级 crash/restart + 101 backlog + replay harness。
- PostgreSQL 16 隔离容器运行相同进程路径；SQLite file database 运行 Lite 路径。
- `cargo test --workspace`、workspace check、strict Clippy。

## 残余工作归口

- 业务 producer/subscriber 接入归 EVO-118-H-C；最终生产进程 Smoke 归 EVO-118-E。
- H-C 必须注册真实 Push handler，并决定生产部署中 Worker 进程的 supervisor/replica 配置。

## 实际验证与审查

- Domain timeout/error-code/config 构造器 3/3、Infra config 3/3、SQLite repository/replay 10/10 通过。
- SQLite file child process 证明 101 probe backlog、expired claim、未知 handler fail-closed、secret 不出现在输出、confirmed replay 与 SIGINT 正常退出。
- PostgreSQL 16 隔离容器最终代码复验：并发 claim/upgrade/replay 1/1、101 backlog Worker 子进程 1/1 通过。
- `cargo fmt --all -- --check`、workspace check、strict Clippy、`cargo test --workspace` 全部通过；Markdown links、diff check 与治理校验通过（仅 Iteration 058 既有 warning）。
- Navigator 发现并修复两项：lease/timeout/backoff 需明确运维上限；lease-expiry `last_error` 需改为稳定 `CLAIM_LEASE_EXPIRED`。修复后无 blocking finding。
- Git Smart HTTP E2E 在一次 workspace 运行中发生长等待，独立复验及最终 workspace 均通过；测试稳定性缺陷已单独归 [EVO-125](EVO-125-git-smart-http-e2e-hang.md)，不改变 H-B 验收。

闭环状态：`Complete`。父 H / EVENT-01 仍由 H-C 的真实 Push producer/subscriber 接入关闭。
