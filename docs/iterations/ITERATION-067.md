# Iteration 067: Outbox Worker Runtime

> 文档状态：Closed / Complete
> 计划发布日期：2026-08-09
> 计划目标：完成 EVO-118-H-B，交付可执行、可停止、可恢复、可观测且支持受控 replay 的独立 Outbox Worker 运行态。
>
> 基线保护：本文件一旦提交，以下“发布计划基线”内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

- `outbox-worker` 独立进程提供 continuous、once、confirmed replay 三种受控操作。
- 配置和构造器共同强制 batch/delivery timeout/lease 不变量；未知事件和错误 fail closed。
- SQLite/PostgreSQL 进程测试证明 crash 遗留 claim 与 101 条积压恢复；SIGINT 优雅停止。
- MVP deliverable：真实二进制在文件 SQLite 与 PostgreSQL 16 上运行，不依赖未实现的 H-C producer。
- Foundation exception：本轮只内置无副作用 probe handler；业务可达性由 H-C 接入真实 Push handler 后闭合。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
| --- | --- | --- | --- | --- |
| EVO-118-H-B | Outbox Worker 运行生命周期 | EVO-118-H | P1 | H-A Done / Iteration 066 Closed / Complete |

## 3. 发布计划基线：不做事项

- 不接入 Push/Commit/Promote/Webhook/Indexer/Agent Session 业务事件；归 H-C 与业务 Story。
- 不在 HTTP Server handler 内启动 detached background task。
- 不新增 Web replay API、管理 UI、Kafka/NATS、微服务或最终 production Compose supervisor。
- 不关闭父 H 或 EVENT-01；最终发布进程 Smoke 归 EVO-118-E。

## 4. 发布计划基线：计划验收标准

- Story 形态：Technical / Reliability / Operations；BDD 不适用，以二进制、进程、失败恢复和双数据库证据验收。
- [x] `run`、`run --once`、`replay <id> --confirm` 命令与非零失败语义闭合。
- [x] config/default/validation 与构造器不变量一致；`batch × timeout < lease`。
- [x] timeout/handler failure 使用稳定错误码，不持久化或打印 payload/原始错误 secret。
- [x] SQLite 进程：101 probe backlog + expired processing 恢复并 delivered；SIGINT 正常退出。
- [x] PostgreSQL 16 进程：相同 101 backlog/recovery 路径通过。
- [x] replay 仅 dead-letter 可确认重置；缺确认/错误状态不变。
- [x] focused、workspace、format、strict Clippy、文档与治理门禁通过。
- [x] Navigator 对 lease/timeouts、shutdown、secret、replay 状态和未知 handler 无 blocking finding。

## 5. 发布计划基线：计划验证

```bash
cd backend
cargo test -p domain outbox
cargo test -p infra config
cargo test --test outbox_worker_process_tests -- --nocapture
TEST_POSTGRES_URL=postgres://evolith_test:dev_password@127.0.0.1:55432/evolith_test \
  cargo test --test outbox_worker_process_tests postgres -- --nocapture
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
| 受保护资产 | Outbox payload、delivery state、claim lease、dead-letter 与审计证据 |
| 攻击者/调用者 | 运维 CLI 调用者、崩溃 Worker、未知/失败 handler、错误配置 |
| 入口 | `outbox-worker` CLI、环境配置、DB repository、SIGINT |
| 信任边界 | Operator → CLI → DB；Worker → handler；旧 claim → 新 claim |
| 失败模式 | lease 早于 batch 超时、未知事件假成功、secret 进入日志/last_error、误 replay、停止时遗留不可恢复状态 |
| 安全默认 | 配置拒绝启动；未知事件失败重试；replay 要求 confirm + dead-letter；停止等待当前有界 batch |
| 验证证据 | config/worker unit、SQLite/PG child process、secret negative、replay negative、Navigator |

回滚点：独立 binary、runtime/config、repository replay 方法和测试为一个切片；失败时保持
H-B In Progress，不回退 H-A lease/fencing，也不把 Worker 塞回 HTTP handler。

## 7. 闭环台账

| 项目 | 本轮记录 |
| --- | --- |
| 请求结果 | 继续完成所有已规划迭代；本轮交付 EVO-118-H-B 可运行 Worker 进程 |
| 产物 | runtime/config、CLI binary、safe probe handler、replay repository、unit/process tests、CONFIG/Architecture docs |
| 状态同步归口 | H-B、父 H、EVO-118、Product Backlog、Iteration 067、Iteration Index、Baseline、Roadmap、Board |
| Story/BDD 归口 | Technical / Reliability；用进程、失败恢复、双数据库和状态证据替代行为 BDD |
| 验证证据 | focused + SQLite/PG child process + workspace gates + docs/governance + Navigator |
| 残余工作归口 | 真实 Push handler/producer 归 H-C；部署 supervisor/最终 Smoke 归 EVO-118-E |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
| --- | --- | --- |
| 2026-08-09 | inventory | Iterations 018/019/020/027 继续 Superseded/Blocked；025/026 继续 Blocked 待 refinement；056/060 Closed / Partial 且 residual 有 owner；057~059、061~066 Closed / Complete；无 Active/Review iteration 阻塞本轮。 |
| 2026-08-09 | refinement | H-B 保持单一“可运维 Worker 进程”结果；runtime、timeout、replay 与恢复共同构成该运行契约。Foundation exception 明确记录，真实业务 handler 归 H-C。 |
| 2026-08-09 | activation | H-A 已 Done；H-B 的 CLI、配置不变量、失败模型、双数据库进程证据、不做事项和 residual owner 完整，激活 Iteration 067。 |
| 2026-08-10 | defect routing | Workspace Gate 的既有 `test_git_clone_push_pull_e2e` 出现 224.86s 停滞后中止；独立复验最终通过但报告 30425.94s。该缺陷不改变 H-B 范围，已按 Backlog-first 归 EVO-125。 |
| 2026-08-10 | implementation | 交付独立 `outbox-worker` continuous/once/replay CLI、`OutboxRuntime`、timeout/backoff/lease 配置、稳定错误码、SQLite/PG 原子 dead-letter replay 与稳定文档。 |
| 2026-08-10 | verification | Domain 3/3、config 3/3、SQLite repository 10/10、SQLite process 3/3；PostgreSQL 16 claim/upgrade/replay 1/1 与 Worker 101 backlog process 1/1；workspace fmt/check/strict Clippy/test 全通过。 |
| 2026-08-10 | navigator | 发现并修复时间参数缺少实际运维上限、lease-expiry 使用自由文本错误两项；最终无 blocking finding。 |
| 2026-08-10 | closure | H-B Done / Complete，Iteration 067 Closed / Complete；父 H 与 EVENT-01 保持开放，H-C 承接真实 Push producer/subscriber。 |

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
| --- | --- | --- | --- | --- |

## 10. Review

- 完成：独立 Worker 进程、continuous/once/confirmed replay、bounded delivery、优雅停止、双数据库恢复、错误脱敏和稳定配置文档全部闭合。
- 未完成：本 Story 无未完成验收；真实业务 handler/producer 不属于 H-B。
- 验证结果：focused、SQLite file process、PostgreSQL 16 process、workspace fmt/check/strict Clippy/test、Markdown/diff/governance 全部通过；治理仅报告 Iteration 058 既有 warning。
- Navigator：两项发现已修复；最终无 blocking finding。
- 闭环状态：`Complete`
- 残余归口：H-C 注册真实 Push producer/subscriber 并决定生产 supervisor/replica；EVO-118-E 做最终生产 Smoke；EVO-125 独立修复既有 Git E2E 不稳定。EVENT-01 保持开放。

## 11. Retrospective

- 做得好的：先把 batch timeout 与 lease 写成构造器不变量，再用真实子进程和双数据库证明恢复；replay 同时由状态条件和显式确认保护。
- 需要调整的：进程测试不应靠固定 sleep 判断信号处理器已就绪；改为等待真实 probe delivered 后再发 SIGINT，证据更稳定。
- 用户可见文档：CLI/config 变化已同步 CONFIG、Architecture 与 `.env.example`；无 UI/API 合约变化。
- 写入 EVOLUTION：本轮经验已由测试就绪探针和 EVO-125 缺陷归口承接，无额外跨 Story 稳定经验需要重复写入。
