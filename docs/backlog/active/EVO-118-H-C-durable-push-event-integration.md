# EVO-118-H-C Durable Push Event 业务接入

- **身份**：平台维护者、Git Push 调用方与后续事件消费者
- **类型**：Technical / Reliability / Integration
- **状态**：Done / Complete（Iteration 068）
- **优先级**：P1
- **父 Epic**：[EVO-118-H](EVO-118-H-durable-outbox-events.md)
- **依赖**：EVO-118-H-B Done；现有 Git Smart HTTP Push 路径
- **影响范围**：backend / git / db / worker / tests / docs

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- [父 Epic EVO-118-H](EVO-118-H-durable-outbox-events.md)
- [Iteration 068](../../iterations/ITERATION-068.md)
- [Security Review](../../sop/SECURITY-REVIEW.md)
- [Production Readiness Baseline](../../reference/PRODUCTION-READINESS-BASELINE.md)

## 工程目标

把现有 Push 后派生工作从可丢失的进程内异步边界迁移到 Durable Outbox，并建立类型化
subscriber/idempotency contract，使未来 Webhook/Indexer 可以复用同一可靠边界。

本 Story 的可运行结果是：真实 Git Smart HTTP Push 成功后产生 `repo.push.completed.v1`
事件，独立 Worker 在崩溃恢复和重复投递下幂等刷新 Repo metadata。Git 已写入而事件
持久化失败时不得向调用方返回成功；后续相同 Push 或明确 reconcile 可重建事件。

## 不做事项

- 不提前实现尚不存在的 Commit/Promote/Agent Session producer；这些由 EVO-105/EVO-106
  在各自业务事务中 enqueue。
- 不实现 Webhook 外发或 Indexer Parser；归 EVO-107/EVO-108。
- 不新增消息总线、微服务、Web replay API 或生产 supervisor；最终部署归 EVO-118-E。
- 不改变 Agent Scoped Token、Branch/Path Policy 或 Force Push 授权语义。

## 事件契约与失败模型

- 事件类型：`repo.push.completed.v1`；aggregate 为 Repo UUID。
- Payload 仅含 tenant/repo/default branch/commit SHA/commit timestamp 等非敏感事实；禁止
  Git credential、Authorization、原始 pack、URL credential 或任意请求 Header。
- 幂等键由 Repo、default branch 与 commit SHA 稳定派生；相同事实重复 enqueue/delivery
  不产生重复业务副作用。
- Git 文件系统与数据库不能跨介质 ACID：receive-pack 成功而 enqueue 失败时返回明确失败并
  记录可 reconcile 事实，不以 warning 伪装成功；事件一旦入库，metadata 由 Worker 重试。

## 验收场景

### Scenario 1：Push 成功进入持久边界

- **Given** 已授权调用方对 Tenant 内 Repo 执行 Git Push
- **When** receive-pack 成功更新 default branch
- **Then** 响应成功前已持久化结构化 `repo.push.completed.v1` 事件
- **And** Worker 重启后仍能刷新对应 Repo metadata

### Scenario 2：重复投递保持幂等

- **Given** 相同 Repo/default branch/commit 的事件被重复 enqueue 或重复 delivery
- **When** Worker 处理这些尝试
- **Then** 只形成同一个逻辑 metadata 结果
- **And** 不因唯一键冲突把已成功持久化的 Push 误报为失败

### Scenario 3：跨介质分裂不返回假成功

- **Given** receive-pack 已移动 Git Ref，但数据库 enqueue 失败
- **When** HTTP handler 完成 Push 后处理
- **Then** 调用方收到失败而不是成功
- **And** 相同 Push/reconcile 可根据 Git default branch 当前事实重建 durable event

### Scenario 4：Payload 与错误日志不泄密

- **Given** 请求携带 Git credential 或其他敏感输入
- **When** producer、subscriber 或 delivery 失败
- **Then** Outbox payload、last_error 和应用日志不包含 credential、原始 pack 或 Header

## 技术验收

- [x] Push 成功后持久化结构化事件；失败路径不返回假成功，Git/DB 分裂有明确 reconcile 归口。
- [x] 现有 push 后 metadata 派生工作消费 Durable Event，不再依赖可丢失的 fire-and-forget task。
- [x] subscriber 使用 idempotency key，重复投递不产生重复业务副作用。
- [x] crash/restart、重复投递、事务/补偿失败和 payload secret 负向测试通过。
- [x] EVO-105/EVO-106/EVO-107/EVO-108 的事件生产/消费契约和依赖同步。
- [x] Driver 实现后完成 Navigator 审查，无未归口 blocking finding。

## 最小验证

- 真实 Git Push E2E + crash/restart + duplicate delivery。
- SQLite/PostgreSQL repository/integration、workspace tests、strict Clippy。
- Markdown links、diff check 与治理校验。

## 残余工作归口

- Commit/Promote producer 归 EVO-105；Agent Session Event 归 EVO-106；Webhook/Indexer consumer
  归 EVO-107/EVO-108。
- 最终 Worker supervisor 与 production smoke 归 EVO-118-E；后续业务 producer/consumer 继续
  由 EVO-105/EVO-106/EVO-107/EVO-108 负责。

## ADR 结论

不新增 ADR。本 Story 实现既有 Production Readiness Baseline 与父 H 已确定的
`Durable Outbox + at-least-once + idempotent subscriber + reconcile` 边界，不改变技术栈、
部署、认证、存储或数据事实源。

## 实际验证（2026-08-10）

- `cargo test -p domain repo_push_event`：2/2 通过；固定事件 schema、idempotency identity、
  secret/未知字段负向检查通过。
- `cargo test -p infra --test outbox_repo_tests`：11/11 通过；SQLite 幂等复用、key collision、
  lease/fencing/replay 通过。
- `cargo test -p api --test git_smart_http_e2e_tests receive_pack_does_not_report_success_when_post_push_enqueue_fails`：1/1 通过；Git Ref 已移动但 enqueue 故障返回失败，metadata 未假更新。
- `cargo test -p api --test durable_push_event_postgres_tests` with dedicated PostgreSQL 16：1/1 通过；真实 PostgreSQL producer/reconcile、Worker subscriber、metadata update 与重复投递幂等闭合。
- `cargo test --test outbox_worker_process_tests`：SQLite 2/2 通过；PostgreSQL 进程 1/1 通过（显式 `TEST_POSTGRES_URL`）。
- `cargo clippy --workspace --all-targets -- -D warnings` 与 `cargo check --workspace --all-targets`：通过。
- `test_git_clone_push_pull_e2e`：连续 3/3 通过；同步 Git 子进程导致的 runtime starvation 已由
  [EVO-125](EVO-125-git-smart-http-e2e-hang.md) 修复为 bounded async runner，完整 Smart HTTP
  clone/push/pull 语义保持真实执行。

闭环状态：`Complete`。
