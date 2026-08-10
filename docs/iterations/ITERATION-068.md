# Iteration 068: Durable Push Event Integration

> 文档状态：Closed / Complete
> 计划发布日期：2026-08-10
> 计划目标：完成 EVO-118-H-C，使真实 Git Push 通过 Durable Outbox 驱动幂等 Repo metadata 派生，不再依赖可丢失的 post-push task。
>
> 基线保护：本文件一旦提交，以下“发布计划基线”内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

- receive-pack 成功后、HTTP 成功响应前持久化类型化 `repo.push.completed.v1` 事件。
- 独立 Worker 消费事件并幂等刷新 Repo default branch metadata，支持 crash/restart 与重复投递。
- 明确 Git/DB 跨介质失败和 reconcile 语义；失败不再只 warning 或返回假成功。
- MVP deliverable：真实 Git Push -> durable event -> Worker -> Repo metadata 的 SQLite/PostgreSQL 可运行闭环。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
| --- | --- | --- | --- | --- |
| EVO-118-H-C | Durable Push Event 业务接入 | EVO-118-H | P1 | H-B Done / Iteration 067 Closed / Complete |

库存处置：Iterations 018/019/020/025/026/027 保持既有 Blocked/Superseded disposition；
Iterations 056/060 保持 Closed / Partial 且 residual 已归口；Iteration 067 Closed / Complete；
无 Active/Review iteration 阻塞本轮，Iteration 068 是唯一 Active iteration。

## 3. 发布计划基线：不做事项

- 不实现 Commit/Promote/Agent Session producer；归 EVO-105/EVO-106。
- 不实现 Webhook 外发或 Indexer Parser；归 EVO-107/EVO-108。
- 不新增消息总线、微服务、管理 UI 或 production supervisor；最终部署归 EVO-118-E。
- 不改变 Smart HTTP 授权、Force Push、protected branch/path 或 Agent token 语义。

## 4. 发布计划基线：计划验收标准

- Story 形态：Technical / Reliability / Integration；跨进程可观察行为使用 H-C item 中的四个 BDD 场景。
- [ ] Push 成功响应前 durable event 已持久化；enqueue 失败不返回假成功，分裂有 reconcile 路径。
- [ ] metadata subscriber 不依赖 handler detached post-push task，crash/restart 后可恢复。
- [ ] stable idempotency key 使重复 enqueue/delivery 得到同一逻辑结果。
- [ ] payload/last_error/log secret 负向测试通过。
- [ ] SQLite/PostgreSQL producer/subscriber、事务失败与真实 Push E2E 通过。
- [ ] EVO-105/106/107/108 事件契约依赖同步。
- [ ] workspace 全量门禁、文档治理和 Navigator 审查通过。

## 5. 发布计划基线：计划验证

```bash
cd backend
cargo test -p domain outbox
cargo test -p infra outbox
cargo test -p api git_smart_http
cargo test --test git_smart_http_e2e_tests -- --nocapture
TEST_POSTGRES_URL=postgres://evolith_test:dev_password@127.0.0.1:55432/evolith_test \
  cargo test --test durable_push_event_tests postgres -- --nocapture
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace

cd ..
python3 scripts/tests/check-markdown-links.py
git diff --check
sh /Users/GHuang/.agents/skills/agent-project-governance/scripts/validate_project_governance.sh .
```

## 6. 发布计划基线：风险、威胁模型与回滚

| 项目 | 内容 |
| --- | --- |
| 受保护资产 | Git Ref、Repo metadata、Outbox payload/delivery state、租户身份与 credential |
| 攻击者/调用者 | Git credential 调用方、跨租户调用者、崩溃 Worker、重复/伪造事件 |
| 入口 | Git Smart HTTP receive-pack、Outbox repository、Worker handler、Git storage |
| 信任边界 | Caller -> API -> git subprocess；Git filesystem -> producer -> DB；DB -> Worker -> Repo metadata |
| 失败模式 | Git 已写而事件未入库、成功响应早于持久化、重复副作用、跨租户 metadata 更新、secret 持久化/日志泄露 |
| 安全默认 | 授权保持 fail closed；enqueue 失败返回失败；事件 schema 校验；Worker 失败重试/dead-letter；不记录敏感 payload |
| 验证证据 | auth/tenant 既有 E2E + producer/subscriber 负向测试 + SQLite/PG + crash/restart + Navigator |

回滚点：typed event、producer wiring、subscriber 和测试作为一个切片回滚；不回退 H-A/H-B
lease/worker 基线。若 Git 已写但 DB 失败，保留 Git 事实并通过相同 Push/reconcile 重建事件。

## 7. 闭环台账

| 项目 | 本轮记录 |
| --- | --- |
| 请求结果 | 继续完成规划迭代；本轮交付 H-C 的真实 Push durable event 闭环 |
| 产物 | typed event/payload、Push producer、Worker subscriber、双数据库与 E2E 测试、Architecture/依赖文档 |
| 状态同步归口 | H-C、父 H、EVO-118、Product Backlog、Iteration 068/index、Baseline、Roadmap、Board、AGENTS |
| Story/BDD 归口 | Technical / Reliability / Integration；四个 BDD 场景覆盖持久化、重复、分裂和 secret |
| 验证证据 | focused + real Push + SQLite/PostgreSQL + crash/restart + workspace + docs/governance + Navigator |
| 残余工作归口 | Commit/Promote/Session producer 归 EVO-105/106；Webhook/Indexer 归 EVO-107/108；supervisor 归 EVO-118-E；E2E hang 归 EVO-125 |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
| --- | --- | --- |
| 2026-08-10 | inventory | 无 Active/Review iteration；历史 Blocked/Superseded 与 Closed/Partial 均保持既有 disposition 和 residual owner。 |
| 2026-08-10 | refinement | 补齐 H-C Required Reads、四个 BDD 场景、事件 schema/idempotency、威胁模型、Git/DB 分裂与 residual owner；不需要新 ADR。 |
| 2026-08-10 | activation | H-B 已 Done；H-C 满足 DoR 并进入 In Progress，Iteration 068 成为唯一 Active iteration。 |
| 2026-08-10 | implementation | typed `repo.push.completed.v1` producer、幂等 SQLite/PostgreSQL enqueue、Receive Pack 503/reconcile、Worker subscriber、secret/tenant/旧事件负向测试已完成。 |
| 2026-08-10 | validation | Domain 2/2、SQLite outbox 11/11、SQLite failure E2E 1/1、PostgreSQL H-C integration 1/1、Worker SQLite/PG process、check/clippy 通过；完整 Git Smart HTTP 4/4 通过。 |
| 2026-08-10 | fixture correction | Repo Context SQLite fixtures 补齐 migrations 011/012；metadata Push 测试改为断言 pending durable event、执行真实 Worker 后验证元数据，不再等待已删除的进程内后台任务。定向测试 1/1、workspace 跳过已登记 EVO-125 单项后全绿，PostgreSQL H-C 与 claim/idempotency 顺序复验各 1/1。 |
| 2026-08-10 | EVO-125 closure | 复现并确认同步 Git 子进程导致 runtime starvation；改用 `tokio::process::Command` + 20 秒阶段超时 + `kill_on_drop`。完整 clone/push/pull 聚焦 E2E 连续 3/3、Smart HTTP suite 4/4、未跳过的 workspace 全量测试、format/check/strict Clippy、文档与治理校验均通过；`lsof -c git` 无遗留 Git 子进程。EVO-125、H-C 与本 Iteration 均达到 Complete。 |
| 2026-08-10 | navigator | 发现并修复 trait 幂等默认退化、SQLite 冲突吞错、超时 stdin 残留和 former-default 旧事件处理；无未归口 H-C blocking finding；当时的 EVO-125 独立残余已在最终 closure 记录中关闭。 |

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
| --- | --- | --- | --- | --- |

## 10. Review

- 完成：typed event contract、Push producer、reconcile handshake、幂等 subscriber、SQLite/PostgreSQL、crash/restart/secret/tenant 负向证据。
- 完成：完整 Git Smart HTTP clone-push-pull 连续 3/3 通过；Git 子进程具备有界 timeout 与阶段诊断。
- 验证结果：见 H-C item 实际验证；workspace 全量门禁已通过。
- 闭环状态：`Complete`
- 残余归口：后续 Commit/Promote/Agent Session/Webhook/Indexer producer/consumer 仍由各自 Story 负责；最终 Worker supervisor 与 production smoke 归 EVO-118-E。

## 11. Retrospective

- 做得好的：把 reconcile 放到 receive-pack advertisement，覆盖 up-to-date 重试无法触发 producer 的分裂场景。
- 需要调整的：Smart HTTP E2E 必须使用异步、有界 Git 子进程；本轮已由 EVO-125 收口。
- 用户可见文档：API Contract、Architecture、CONFIG 已同步 durable event 与 503/reconcile 语义。
- 事件基础设施完成，但未来业务事件仍按 owner Story 接入，不将 H-C 扩大为 Commit/Promote/Webhook/Indexer 实现。
