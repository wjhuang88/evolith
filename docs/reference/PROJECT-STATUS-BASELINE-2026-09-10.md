# Evolith 项目整体状态基线（2026-09-10）

> 状态：Current project baseline
> 基线日期：2026-09-10
> 代码基线：`main` @ `4ffe0633dc2e160fab580bd1ed5b1cdd12245dfe`
> 当前执行计划：[CURRENT-EXECUTION-PLAN-2026-09](../roadmap/CURRENT-EXECUTION-PLAN-2026-09.md)
> Git Data Plane 决策：[ADR-0011](../decisions/ADR-0011-walgit-backed-git-data-plane.md)
> 重构 Epic：[EVO-126](../backlog/active/EVO-126-walgit-git-data-plane-refactor.md) / [GitHub #12](https://github.com/wjhuang88/evolith/issues/12)

本文档重新建立“当前实现、已接受目标、未完成计划、发布 Gate”四类事实的统一基线。它不覆盖历史 Iteration/ADR/Release evidence；历史完成结果继续有效，但不得被误读为未来架构已经完成。

## 1. 一句话状态

Evolith 已具备可工作的 Git hosting Alpha、Repo-centric Web 基础、安全与单实例耐久性基线、Durable Outbox/Push Event；Agent Commit/Promote/Session、Activity/Dashboard 最终收敛、Indexer/Discovery、legacy cleanup 与最终生产发布仍未完成。2026-09-10 已接受新的 Git Data Plane 方向：**Evolith 保持模块化单体控制面，`service-git` 重构为基于 WalGit primitives 的对象存储 + WAL 引擎；当前 filesystem-backed Git 实现继续作为迁移前事实，直到 EVO-126 完成 cutover。**

## 2. 当前实现事实

| 领域 | 当前实现 | 成熟度/可依赖范围 | 主要残余 |
|------|----------|------------------|----------|
| 架构形态 | Rust Workspace 模块化单体；Actix Web 主进程 + 独立 Outbox Worker | Stable direction | 不拆第二套 WalGit/Axum 产品服务 |
| 前端 | React + Vite + TypeScript + Tailwind，静态资源嵌入 Rust | Alpha product | 最终 Shell/Dashboard/Activity/legacy UI 退场未闭合 |
| Auth / Tenant / RBAC | Cookie/JWT、API Key、Typed Capability、tenant hiding | Hardened baseline | Agent Scoped Token 尚未实现 |
| Egress | HTTP Tool 统一 Egress/SSRF Policy | Hardened baseline | Webhook 等后续出站必须复用 |
| Repo CRUD | `git_repos` + lifecycle state + Initial Commit + Reconciler | Alpha / single-instance | 当前 durable Git truth 仍在 filesystem；将由 EVO-126 改造 |
| Git Smart HTTP | `git` subprocess `upload-pack/receive-pack` + Actix adapter | Alpha，真实 clone/push/pull 已验证 | target 将由 WalGit Git/WAL primitives 接管 |
| Repo Context | `gix` 直接读取 local bare repo，Tree/Blob/Commit/Diff 有界 | Alpha/usable | target 改为 `RepoHandle/ObjectAccess`，Local 路径仍可继续用 gix |
| Git durability | PostgreSQL + persistent Git volume 联合 backup/restore | 已关闭 DATA-01 的历史单实例 Gate | 不再作为长期多实例/对象存储架构；迁移后需新 Gate 证明 |
| Repo lifecycle | DB/FS state + compensation + reconciler + quarantine | DATA-02 Closed | 迁移后改为 DB/Object-Store lifecycle/reconcile |
| Durable events | Outbox claim/worker/push producer/subscriber/reconcile | EVENT-01 Closed | Commit/Promote/Webhook/Indexer 各自接入仍未实现 |
| Repo Web | Repo list/create/detail/files/commits/settings、Commit Evidence、Onboarding、public/auth entry、Settings IA | Alpha/usable | Dashboard、Activity、final App Shell/legacy UI removal 未完成 |
| Agent write loop | 尚未实现 | 不可依赖 | EVO-105/106/107/104 |
| Capability discovery | Parser/legacy 基础存在；目标 Indexer/Discovery 未实现 | 不可依赖新产品链 | EVO-108/109 |
| Production delivery | Embedded Frontend 方向已接受 | 未达到 External Alpha | DEPLOY-01 仍开放 |

## 3. 已完成并继续有效的结果

以下结果不因 WalGit 重构被“撤销”；它们是新架构必须保持或重新证明的既有能力：

- EVO-118-B / SEC-01：API Key、Typed Capability、MCP execute 与 tenant authorization 基线已关闭。
- EVO-118-C / SEC-02：HTTP Egress/SSRF 安全边界已关闭。
- EVO-118-D / DATA-01：**旧 filesystem 引擎**的单实例 PostgreSQL + Git 联合持久化、备份和恢复已证明。
- EVO-118-F / DATA-02：Repo lifecycle、补偿、真实 Initial Commit、Reconciler 已证明。
- EVO-118-G-B/C/D：Runtime readiness、caller-aware rate limit、关键依赖 fail-closed 已完成。
- EVO-118-H-A/B/C / EVENT-01：Durable Outbox、Worker、Push producer/subscriber 与完整 Git E2E 已完成。
- EVO-112-A/B/C、EVO-120、EVO-121-C/D：Repo Web 基础、Commit Evidence、Onboarding、Entry/Auth、Settings IA 已完成。
- EVO-125：真实 Git 客户端 E2E 已改为 bounded async runner，连续 clone/push/pull 稳定验证已完成。

## 4. 当前未完成计划

| Workstream | 状态 | 说明 |
|------------|------|------|
| EVO-118-G-A | Review / Partial | PR/Main 本地门禁已完成，Branch Protection 远端证据 residual；发布前必须闭合 |
| EVO-126 WalGit Git Data Plane | **Accepted / Next** | 本轮新增 P0 架构重构；先于 Agent Write Loop 激活 |
| EVO-105 / EVO-106 | Proposed | Commit/Promote + Agent Session/Scoped Token；等待 EVO-126 Git write boundary 稳定后激活 |
| EVO-107 / EVO-104 | Proposed | Webhook + Vibe Coding；依赖 Agent write loop 与新 data plane |
| EVO-121-E/B/A/F | Proposed / paused | Activity、Dashboard、final Shell、legacy UI removal |
| EVO-108/109 | Proposed | Indexer + Discovery；依赖稳定 durable Git/Commit events |
| EVO-111 / EVO-122 | Proposed / paused | Sandbox 与 legacy Registry backend/table 清理 |
| EVO-118-E | Proposed / final release gate | 最终 clean production build、single delivery、full smoke，关闭 DEPLOY-01 |
| EVO-123/124 等独立债务 | Proposed | 不抢占当前 Git Data Plane P0，按依赖单独排期 |

## 5. 2026-09-10 接受的 Git Data Plane 目标

### 5.1 不变的产品/控制面边界

Evolith 继续拥有：

- User / Tenant / RBAC / API Key / Agent identity；
- Repository metadata 与 lifecycle product semantics；
- `.evolith/policy.yaml` 与 PolicyEvaluator；
- Agent Session、Commit/Promote、Review/Promotion；
- Audit、Durable Outbox、Webhook、Capability Index；
- React 产品 UI 与 Actix HTTP adapters。

### 5.2 改变的 Git 数据面边界

目标由：

```text
Actix -> service-git -> git subprocess/gix -> durable local bare repo filesystem
```

改为：

```text
Actix adapters
    -> Evolith service-git v2
        -> walgit-git / walgit-wal / walgit-store / walgit-bundle
            -> object store (durable source of truth)
            -> local disk (disposable cache only)
```

`walgit-server` 的 Smart HTTP/protocol orchestration 可按 MIT 许可证吸收、改写到 `service-git`，但不引入其 Axum Router、Authenticator、UI 或产品 Policy 作为 Evolith 控制面。

### 5.3 依赖策略

- 第一阶段升级 Evolith Rust MSRV 1.88 -> 1.90。
- WalGit engine crates 必须 pin exact upstream commit SHA，不允许跟随 `main`。
- WalGit types 不泄漏到 `api` / core domain contract；`service-git` 是 anti-corruption layer。
- substantial copied/derived code 保留 MIT attribution，并维护 `THIRD_PARTY_NOTICES.md`。
- upstream pre-1.0 API 变化通过独立 upgrade Story 处理，不在产品开发中静默漂移。

## 6. 新旧事实源划分

### 当前（EVO-126-H cutover 前）

| 数据 | 当前事实源 |
|------|------------|
| Git objects/refs/commits | persistent filesystem bare repo |
| 用户/租户/权限/Repo metadata | PostgreSQL（Lite 为 SQLite） |
| durable event state | PostgreSQL/SQLite Outbox |

### 目标（EVO-126-H cutover 后）

| 数据 | 目标事实源 |
|------|------------|
| Git objects/refs/WAL/manifest/bundles | object store via WalGit |
| local bare repo/materialized packs | cache，不可作为 durable truth |
| 用户/租户/权限/Repo metadata | PostgreSQL（Lite 为 SQLite） |
| Agent/Policy/Audit/Outbox/Index state | PostgreSQL/SQLite，Git provenance 指向 repo/ref/commit/WAL seq |

## 7. Gate 基线

| Gate | 当前状态 | 说明 |
|------|----------|------|
| SEC-01 | Closed | 继续保持，不因 data plane 更换降低权限边界 |
| SEC-02 | Closed | 继续保持 |
| DATA-01 | Closed (historical filesystem engine) | 证明旧单实例引擎；不等价于 WalGit/object-store migration 完成 |
| DATA-02 | Closed (historical filesystem lifecycle) | lifecycle 语义保留，storage reconciler 实现需迁移 |
| EVENT-01 | Closed | 新 receive-pack 必须继续满足 durable producer/subscriber 合约 |
| REL-01 | Partial | G-A remote branch-protection evidence residual |
| **GIT-DP-01** | **Open** | EVO-126：object-store/WAL data plane、migration、Git E2E、Context、bundle、recovery/cutover 完成后关闭 |
| DEPLOY-01 | Open / final release gate | 最终产品/cleanup + GIT-DP-01 后执行 |

GIT-DP-01 不是对旧 DATA-01/DATA-02 的历史否定，而是因架构改线新增的迁移 Gate。

## 8. 当前严格执行顺序

当前优先顺序由 [CURRENT-EXECUTION-PLAN-2026-09](../roadmap/CURRENT-EXECUTION-PLAN-2026-09.md) 负责：

```text
Governance baseline / ADR-0011
→ EVO-126-A dependency/toolchain boundary
→ EVO-126-B service-git v2 + Registry/Store
→ EVO-126-C Smart HTTP read path
→ EVO-126-D WAL-backed receive-pack
→ EVO-126-E Repo Context migration
→ EVO-126-F object-store lifecycle/durability/migration
→ EVO-126-G bundle-uri
→ EVO-126-H cutover + legacy Git engine removal
→ resume EVO-105 / 106 / 107 / 104
→ Experience / Discovery / legacy cleanup
→ EVO-118-E final production convergence
```

Story 仍按 WIP 小批次执行；依赖允许 refinement 后调整 C/D/E/F 的先后，但不得跳过 A/B 或在 H 之前关闭 GIT-DP-01。

## 9. 发布口径

- **当前不能声明生产就绪。** DEPLOY-01、REL-01 residual 与 GIT-DP-01 均未关闭。
- 在 EVO-126-H 之前，运行中的事实仍是 filesystem-backed Git；任何文档不得提前描述为 object-store Git 已落地。
- Agent Beta 的进入条件新增 GIT-DP-01：不得在旧写路径上完成并宣称最终 Agent write architecture。
- External Alpha/Production 的恢复、readiness、capacity 与 smoke 必须在最终 WalGit-backed data plane 上重新证明。

## 10. 文档权威顺序

发生冲突时按以下顺序判断：

1. 代码/配置/实际测试证据：说明“当前实现事实”。
2. 本 Project Status Baseline：说明“当前整体状态和已接受迁移方向”。
3. ADR-0011：说明“Git Data Plane 长期决策”。
4. CURRENT-EXECUTION-PLAN-2026-09：说明“当前激活顺序”。
5. EVO-126 parent/child item：说明“可执行验收”。
6. BOARD：只派生当前运营视图。
7. 2026-07 Production Readiness Plan 与 Two-Month Plan：历史计划基线，不再拥有当前激活顺序。

## 11. 下一动作

下一个可进入 Iteration refinement 的 Story 是 **EVO-126-A**。在启动前仍需按 `START-ITERATION.md` 盘点非终态 Iteration；EVO-118-G-A / Iteration 056 的 Review/Partial residual 必须保留并在 release closure 前处置，但不要求把 WalGit 重构混入旧 Iteration。
