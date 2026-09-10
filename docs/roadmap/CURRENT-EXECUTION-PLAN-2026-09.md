# Evolith 当前执行计划（2026-09）

> 状态：Current execution ordering
> 生效：2026-09-10
> 触发：WalGit-backed Git Data Plane architecture pivot
> 整体基线：[PROJECT-STATUS-BASELINE-2026-09-10](../reference/PROJECT-STATUS-BASELINE-2026-09-10.md)
> 决策：[ADR-0011](../decisions/ADR-0011-walgit-backed-git-data-plane.md)
> 当前 P0 Epic：[EVO-126](../backlog/active/EVO-126-walgit-git-data-plane-refactor.md)

本文自 2026-09-10 起作为**唯一当前激活顺序 owner**。`PRODUCTION-READINESS-PLAN-2026-07.md` 与 `TWO-MONTH-PLAN-2026-07.md` 保留为历史计划基线，不覆写其当时目标、状态和证据。

## 1. 改线原因

2026-08-10 前，项目已完成 Git Alpha、Repo Web 基础、安全/耐久/Outbox 等关键底座。原计划下一阶段准备进入 EVO-105/106 Agent Write Loop。

2026-09-10 决策改变了“Git engine 长期实现方式”：不再把应用节点 persistent filesystem + direct subprocess 作为最终 Git data plane，而是采用 WalGit engine primitives、object store、WAL/manifest CAS，并将 `walgit-server` 的协议 data-plane 代码吸收到 Evolith `service-git v2`。

如果先在旧写路径上实现 Commit/Promote/Agent Session，再迁移 Git engine，会导致 Policy、write transaction、event provenance 和 Context 适配被重复重构。因此本轮明确把 EVO-126 插入 Agent Write Loop 之前。

## 2. 非终态库存处置

| 库存 | 当前处置 | 说明 |
|------|----------|------|
| EVO-118-G-A / Iteration 056 | 保持 Review / Partial residual | Branch Protection 远端证据在 release 前闭合；不把 WalGit 工作塞入旧 Iteration |
| EVO-121 | 保持 In Progress Epic | C/D 已完成；A/B/E/F 按后续产品依赖激活 |
| EVO-118 | 保持 In Progress Epic | E 最终 release gate；G-A residual；其他已完成结果保持 |
| EVO-100 | 保持 In Progress Epic | 产品方向不变；其 Agent/Discovery 子项等待新 Git data plane |
| EVO-105/106/107/104 | Proposed / paused for activation | 改为依赖 GIT-DP-01/EVO-126-H |
| EVO-108/109 | Proposed | 等稳定新 Push/Commit provenance 后激活 |
| EVO-118-E | final release gate | 移到所有产品/cleanup/data-plane cutover 后 |

本轮不是继续修改任何已发布 Planned Iteration；实现 EVO-126-A 时必须按 START-ITERATION 新建 Iteration 编号。

## 3. Phase G0 — Governance Baseline

本批次只建立方向与执行事实，不声称 runtime 已迁移：

- 新建 Project Status Baseline；
- 接受 ADR-0011；
- 新建 WalGit Git Data Plane design；
- 建立 EVO-126 Epic + A~H Stories；
- 建立 GitHub Issue #12~#20 tracking；
- 更新 Backlog/Board/Docs/ADR index 与旧计划 owner 状态。

Exit：Current implementation 与 Accepted target 不混淆；下一 Story 唯一为 EVO-126-A。

## 4. Phase G1 — Dependency Foundation

### EVO-126-A

目标：Rust 1.90、exact WalGit dependency pin、license/supply-chain boundary。

进入：无硬实现依赖。

退出：workspace 在 Rust 1.90 全量门禁通过；当前 filesystem Git 行为不变。

## 5. Phase G2 — Engine Boundary

### EVO-126-B

依赖：EVO-126-A。

目标：`service-git v2` public contract + WalGit Store/Registry + stable RepoId mapping；建立新引擎但暂不切流量。

退出：create/open/delete/list engine contract 与 memory/object-store 集成证据完成；WalGit types 不泄漏到 API/domain。

## 6. Phase G3 — Protocol and Context Parity

以下 Story 都依赖 B；按单 Story WIP 执行。建议顺序 C -> E -> D，以先稳定 read/context 再切 write，但 refinement 可以在不破坏依赖的前提下调整。

### EVO-126-C — Smart HTTP read

- info/refs；
- protocol v0/v2；
- ls-refs / upload-pack；
- framework-neutral streaming；
- clone/fetch/pull E2E。

### EVO-126-E — Repo Context

- RepoHandle/ObjectAccess；
- gix local read 保留；
- remote/bounded materialization；
- Files/Commits/Evidence parity。

### EVO-126-D — WAL write

- receive-pack parse/ingest；
- Evolith authorization/policy hook；
- WAL + manifest CAS publication；
- durable Push Outbox / WAL seq provenance；
- push conflict/failure/restart E2E。

## 7. Phase G4 — Operations and Migration

### EVO-126-F

依赖：EVO-126-B，实施验收需联合 C/D/E 的稳定读写边界。

目标：

- object-store production config；
- readiness/fail-closed；
- DB/Object-Store lifecycle reconcile；
- old filesystem repo import；
- durability/recovery re-baseline；
- migration dry-run/idempotency/failure recovery。

退出：旧 repo 可以安全迁入，新实例/空 cache 可 clone/read/push，store outage readiness 503。

## 8. Phase G5 — Agent Clone Acceleration

### EVO-126-G

依赖：C + F。

目标：bundle-uri capability/list/serve、secure access/fallback/metrics，并以标准 Git client 验证。

该 Story 是 EVO-126 必需项，因为 Agent/Vibe Coding 高频 clone/fetch 是选择 WalGit 架构的重要收益之一。

## 9. Phase G6 — Cutover

### EVO-126-H

依赖：C/D/E/F/G 全部完成。

目标：

- default engine = WalGit-backed；
- production 不再依赖 durable local Git filesystem；
- legacy filesystem Smart HTTP/storage consumer inventory = 0；
- 完整 migration/restart/clone/push/pull/context/outbox/bundle smoke；
- 更新 Architecture/Project Map/Config/Testing/Release docs；
- Security/Navigator/Data review；
- 关闭 GIT-DP-01 和 EVO-126。

## 10. Resume Product Plan

GIT-DP-01 关闭后恢复产品主线：

```text
EVO-105 Commit API + PolicyEvaluator / Promote
→ EVO-106 Agent Session + Scoped Token
→ EVO-107 Webhook
→ EVO-104 Vibe Coding Workspace
→ EVO-121-E/B Activity + Dashboard
→ EVO-108/109 Indexer + Discovery
→ EVO-121-A/F final Shell + legacy UI removal
→ EVO-111 / EVO-122 legacy backend cleanup
→ EVO-118-E final production convergence
```

实际仍按 Backlog DoR 与 WIP 选择单 Story；这里表达依赖顺序，不代表把整条链一次激活。

## 11. Gate 变化

| Gate | 2026-09 处理 |
|------|--------------|
| SEC-01/SEC-02 | 保持 Closed，不允许新引擎降低权限/egress 边界 |
| DATA-01/DATA-02 | 保留旧 filesystem engine 的完成证据；不删除、不改写 |
| EVENT-01 | 保持 Closed；新 WAL receive path 必须继续满足 |
| REL-01 | G-A residual，release 前关闭 |
| GIT-DP-01 | 新增 Open；EVO-126-H 后关闭 |
| DEPLOY-01 | 继续最后执行；最终 smoke 必须跑在 WalGit-backed engine 上 |

## 12. 变更控制

如果 WalGit upstream 在实施期间出现 breaking change：

- 不允许直接把 dependency 从 pinned SHA 漂到新 main；
- 先判断是否影响当前 Story 验收；
- 影响当前 Story 则按 CHANGE-CONTROL 记录；
- 不影响则新建 dependency upgrade Story；
- 如发现 WalGit engine 无法满足 hard requirement，应暂停 EVO-126 后续切片，形成 ADR amendment/superseding ADR，而不是在 service-git 中堆无归口 workaround。

## 13. 完成声明

- G0 文档批次完成只表示“架构与计划已治理”，不表示 EVO-126 implementation complete。
- A~H 每项必须独立记录实际验证和 residual。
- 只有 H 的 cutover/review/recovery 证据闭合后，才可声明 GIT-DP-01 / EVO-126 Complete。
