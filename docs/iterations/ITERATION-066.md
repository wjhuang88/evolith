# Iteration 066: Recoverable Outbox Claims

> 文档状态：Closed / Complete
> 计划发布日期：2026-08-09
> 计划目标：完成 EVO-118-H-A，以 claim lease/fencing 和 PostgreSQL 并发证据消除 Outbox 崩溃后永久卡死与重复领取风险。
>
> 基线保护：本文件一旦提交，以下“发布计划基线”内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

- Outbox claim 写入租约和 fencing token，崩溃后可恢复，旧 Worker 状态写入 fail closed。
- PostgreSQL 真实双 claimer 验证无重复、无遗漏；SQLite 保持明确单进程 Lite 边界。
- MVP deliverable：可运行数据库集成测试证明 claim → crash/lease expiry → reclaim → ACK，且旧 ACK 不覆盖新 claim。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
| --- | --- | --- | --- | --- |
| EVO-118-H-A | 可回收 Outbox Claim 与 PostgreSQL 并发语义 | EVO-118-H | P1 | Iteration 060 schema/repository boundary 已存在；H-A DoR 完整 |

## 3. 发布计划基线：不做事项

- 不启动常驻 Worker、配置运行模式、积压基准或 replay API；归 H-B。
- 不接入 Push/Commit/Promote/Webhook/Indexer/Agent Session；归 H-C 与各业务 Story。
- 不修改 Iteration 060 的 Closed / Partial 计划和执行基线。
- 不关闭 EVENT-01 或宣称 EVO-118-H Complete。

## 4. 发布计划基线：计划验收标准

- Story 形态：Technical / Reliability / Data Integrity；BDD 不适用，以双数据库失败恢复和并发集成证据验收。
- [x] paired `012` migrations 增加 claim token/lease expiry，`011` 保持不变。
- [x] claim/reclaim/attempt/dead-letter 状态转换与 fencing token 行为一致。
- [x] SQLite crash/reclaim、stale ACK/NACK 和 max-attempt tests 通过。
- [x] PostgreSQL 两个并发 claimer 的结果集合互斥且覆盖全部事件；stale recovery 通过。
- [x] focused、format、workspace check、strict Clippy 与治理文档验证通过。
- [x] Navigator 对数据竞争、租约耗尽、旧 Worker 写入和双数据库漂移无 blocking finding。

## 5. 发布计划基线：计划验证

```bash
cd backend
cargo test -p infra --test outbox_repo_tests
TEST_POSTGRES_URL=postgres://evolith_test:dev_password@127.0.0.1:5432/evolith_test \
  cargo test -p infra --test pg_outbox_claim_tests -- --nocapture
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings

cd ..
python3 scripts/tests/check-markdown-links.py
git diff --check
scripts/validate_project_governance.sh .
```

## 6. 发布计划基线：风险、威胁模型与回滚

| 项目 | 内容 |
| --- | --- |
| 受保护资产 | Outbox Event、幂等语义、派生任务状态和审计证据 |
| 攻击者/调用者 | 并发 Worker、崩溃/暂停的旧 Worker、数据库故障 |
| 入口 | repository claim、ACK/NACK、migration、Worker delivery boundary |
| 信任边界 | Worker → DB；旧 lease owner → 新 lease owner；SQLite Lite → PostgreSQL production |
| 失败模式 | processing 永久卡死、重复 claim、旧 ACK 覆盖新状态、attempt 耗尽仍不可观察、双库漂移 |
| 安全默认 | lease 到期可回收；token 不匹配返回 conflict；最终过期进入 dead-letter |
| 验证证据 | SQLite failure-path tests、PostgreSQL concurrent integration、Navigator review |

回滚点：新增 `012` migrations、模型/repository 签名与 focused tests 是独立切片；失败时
保持 H-A In Progress 并修复，不改写 `011` 或恢复无租约的完成声明。

## 7. 闭环台账

| 项目 | 本轮记录 |
| --- | --- |
| 请求结果 | 继续完成所有已规划迭代；本轮交付 EVO-118-H-A 的可恢复 claim 数据边界 |
| 产物 | H-A Story、H 父项拆分、paired migration、domain/repository 实现、SQLite/PG integration tests |
| 状态同步归口 | H-A、父 H、EVO-118、Product Backlog、Iteration 066、Iteration Index、Baseline、Roadmap、Board |
| Story/BDD 归口 | Technical / Reliability；用失败恢复、fencing 和并发数据库测试替代行为 BDD |
| 验证证据 | focused SQLite/PG、format/check/clippy、Markdown/diff/governance、Navigator |
| 残余工作归口 | Worker runtime/replay 归 H-B；Push 接入归 H-C；未来 producer 归 EVO-105/106 |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
| --- | --- | --- |
| 2026-08-09 | inventory | Iterations 018/019/020/027 继续 Superseded/Blocked；025/026 继续 Blocked 待 refinement；056/060 已 Closed / Partial 且 residual 有 owner；061~065 Closed / Complete；无 Active/Review iteration 阻塞本轮。 |
| 2026-08-09 | refinement | EVO-118-H 包含 claim 正确性、Worker runtime、业务接入三个独立结果，保留为父 Epic 并拆 H-A/B/C；Iteration 060 基线保持 Closed / Partial。 |
| 2026-08-09 | activation | H-A 具备单一技术结果、Hard/Soft/Assumption、双数据库验收、失败模型、不做事项和残余 owner；激活 Iteration 066。 |
| 2026-08-09 | driver | 新增 paired 012 migration、claim token/lease、stale recovery、max-attempt dead-letter 与 ACK/NACK fencing；Worker 单次处理携带当前 token。 |
| 2026-08-09 | navigator | 首轮发现仅验证 fresh migration、repository 接受 `max_attempts=0` 与非正 lease；补 SQLite/PG `011 → 012` preservation 和 fail-closed validation 后复核无 blocking finding。Residual risk：H-B 需协调 lease 与 delivery timeout。 |
| 2026-08-09 | validation | SQLite focused 8/8；PostgreSQL 16 focused 1/1（两个 claimer 各 5/10、互斥/全覆盖、stale ACK/NACK、upgrade preservation）；`cargo test -p infra`、fmt、workspace all-targets check、strict Clippy、Markdown 297、diff check 通过。Governance validator exit 0，仅报告预存 Iteration 058 evidence warning。首次 sandbox 内 loopback 连接被拒，授权本机隔离容器后相同测试通过。 |
| 2026-08-09 | completion | EVO-118-H-A Done / Complete；Iteration 066 Closed / Complete。父 H 保持 In Progress，EVENT-01 开放；H-B/H-C 是已登记 residual。 |

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
| --- | --- | --- | --- | --- |

## 10. Review

- 完成：paired 012 upgrade、recoverable lease、fencing token、并发 claim、stale recovery、最终 attempt dead-letter 与非法边界 fail closed。
- 未完成：常驻 Worker/replay 与 Push 业务接入不在 H-A 范围，分别归 H-B/H-C。
- 验证结果：SQLite 8/8；真实 PostgreSQL 16 1/1；infra tests、fmt/check/clippy、Markdown 297、diff 通过；governance validator exit 0，保留预存 Iteration 058 evidence warning。
- Navigator：修正 upgrade preservation 和非法 attempt/lease 两项 finding 后无 blocking finding；H-B 需保证 delivery timeout 小于 lease 或支持续租。
- 闭环状态：`Complete`
- 残余归口：H-B/H-C；EVO-105/106 负责未来 producer；EVENT-01 保持开放。

## 11. Retrospective

- 做得好的：先把 crash/fencing 变成 repository contract，再用真实 PostgreSQL 并发集合证明生产语义。
- 需要调整的：migration 测试必须从前一版本带已有行升级，fresh install 不能单独证明升级安全。
- 写入 EVOLUTION：新增“数据库 migration 需要 existing-row upgrade evidence；fresh schema 通过不足以关闭 migration Gate”。
