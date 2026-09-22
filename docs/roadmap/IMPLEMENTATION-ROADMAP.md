# Evolith 实施路线图

> 制定日期：2026-05-15
> 最近更新：2026-09-10（WalGit-backed Git Data Plane replan）
> 目标：维护阶段级长期路线、Backlog / ADR / Release Gate 归口关系。
> **当前激活顺序不由本文定义**：请读 [Current Execution Plan 2026-09](CURRENT-EXECUTION-PLAN-2026-09.md)。

本文不是任务池。Agent 不应直接从本文开工：

- 当前整体事实：[Project Status Baseline 2026-09-10](../reference/PROJECT-STATUS-BASELINE-2026-09-10.md)。
- 可执行 Story：[Product Backlog](../backlog/PRODUCT-BACKLOG.md)。
- 当前执行排序：[Current Execution Plan 2026-09](CURRENT-EXECUTION-PLAN-2026-09.md)。
- 重大取舍：[ADR](../decisions/README.md)。
- 未成熟方向：[Proposals](../proposals/README.md)。
- 历史执行基线：[Production Readiness Plan 2026-07](PRODUCTION-READINESS-PLAN-2026-07.md) 与 [Two-Month Plan](TWO-MONTH-PLAN-2026-07.md)。

## 1. 产品方向

2026-06-23 的 Git-centric 产品方向继续有效：

> Evolith 是 Git 托管 + AI Agent/Vibe Coding + Pages 式 Skill/CLI/MCP 能力发现平台。

Git Repo 继续是代码和版本历史的事实源；Skill/CLI/MCP 是从 Repo 内容派生的能力。2026-09-10 的 [ADR-0011](../decisions/ADR-0011-walgit-backed-git-data-plane.md) 改变的是 **Git data plane 的实现方式**，不是产品方向：durable Git bytes 从应用 persistent filesystem 演进为 WalGit-backed object store/WAL，Evolith 保持 tenant/RBAC/Agent/Policy/Audit 产品控制面。

## 2. 架构路线

### 2.1 保持模块化单体

```text
React + Vite + Bun
        |
        v
Evolith Actix Application
- HTTP/API adapters
- Auth / Tenant / RBAC
- Agent / Policy / Product workflows
        |
        v
service-git v2
- Smart HTTP protocol
- Repo Context
- Git write boundary
- bundle-uri
        |
        v
WalGit primitives
- walgit-git
- walgit-wal
- walgit-store
- walgit-bundle
        |
        v
Object Store (durable Git truth)
Local disk = disposable cache
```

- 不因 WalGit 引入第二个强制 Axum 产品服务。
- 不以拆微服务/Kafka/Kubernetes 作为当前架构目标。
- Actix 仍是产品 HTTP adapter；WalGit server 中可复用的是 Git data-plane orchestration，而非 Router/Auth/UI。

### 2.2 事实源演进

| 数据 | EVO-126-H 前 current implementation | EVO-126-H 后 target |
|------|-------------------------------------|--------------------|
| Git objects/refs/history | persistent filesystem bare repo | object store via WalGit WAL/manifest |
| local packs/repo | durable repo | disposable materialized cache |
| 用户/租户/权限/Repo metadata | PostgreSQL / SQLite | PostgreSQL / SQLite |
| Agent/Policy/Audit/Outbox/Index | PostgreSQL / SQLite | PostgreSQL / SQLite；provenance 指向 repo/ref/commit/WAL seq |

Current 与 Target 不得混写。

### 2.3 目标内部边界

```text
Adapters
  -> Application Services
     -> RepoApplicationService / GitWriteService / PolicyEvaluator
     -> AgentSessionService / CapabilityIndexer / WebhookDeliveryService
        -> Domain + Repository / Outbox / EgressPolicy
        -> service-git public contract
           -> WalGit engine adapter
```

WalGit types 不向 `api`/核心 domain 扩散；`service-git` 是 anti-corruption layer。

## 3. 已完成阶段

| Phase | 状态 | 结果 |
|-------|------|------|
| Phase A API 对齐 | Done | 主要前后端合约漂移修复 |
| Phase B React + Vite + Bun | Done | 静态 SPA 与 Embedded Frontend 方向形成 |
| Phase C Auth lifecycle | Done / partial future hardening | 注册、登录、重置、验证、邀请具备；Agent Session 独立后续 |
| Phase D MCP Tool execution | Done / Hardened baseline | Typed Capability 与 Egress/SSRF Gate 已关闭 |
| Phase E'-1 Git Alpha | Done / Alpha | Repo CRUD、Smart HTTP、真实 clone/push/pull、Context API |
| Production Stabilization S1 | Done for defined gates | SEC-01/02、DATA-01（旧引擎单实例）Closed |
| Production Stabilization S2 | Mostly Done | DATA-02、EVENT-01 Closed；REL-01 仅 G-A remote evidence residual |
| Repo-centric Web foundation | Done / Alpha | EVO-112 A/B/C、EVO-120、EVO-121-C/D 完成 |

这些历史完成结果在 EVO-126 后继续作为“必须保持的行为/安全基线”，不因 data-plane 改造被撤销。

## 4. 当前架构阶段：Git Data Plane Evolution

归口：[EVO-126](../backlog/active/EVO-126-walgit-git-data-plane-refactor.md)，Gate：**GIT-DP-01**。

### G1 Dependency Foundation

- EVO-126-A：Rust 1.90、exact WalGit SHA pin、MIT attribution、supply-chain boundary。

### G2 Engine Boundary

- EVO-126-B：`service-git v2` + Evolith-owned contracts + WalGit Store/Registry lifecycle。

### G3 Protocol / Context / Write Parity

- EVO-126-C：Smart HTTP read v0/v2；
- EVO-126-E：Repo Context on RepoHandle/ObjectAccess；
- EVO-126-D：receive-pack -> WAL/manifest CAS + Durable Push Event。

### G4 Operations / Migration

- EVO-126-F：object-store config/readiness、DB/object-store lifecycle reconcile、filesystem repo import、recovery re-baseline。

### G5 Clone Acceleration

- EVO-126-G：secure bundle-uri for Agent/CI clone/fetch workload。

### G6 Cutover

- EVO-126-H：default WalGit-backed engine、legacy filesystem Git engine consumer removal、final migration/recovery/protocol/context/event/bundle evidence，关闭 GIT-DP-01。

详细边界见 [WalGit Refactor Design](../design/WALGIT-GIT-DATA-PLANE-REFACTOR.md)。

## 5. GIT-DP-01 后恢复的产品阶段

### Agent Write Loop + Vibe Coding

归口：EVO-105、106、107、104。

进入条件新增：**GIT-DP-01 Closed**。

继续保留的硬约束：

- Agent Token 不得直接获得不受 Policy 控制的 generic `git-receive-pack`。
- `commit:<scope>` 落实为 Branch/Path capability。
- Commit/Promote 失败时目标 Ref 不移动。
- Policy `auto_merge / require_review / block` 由行为测试证明。
- Push/Commit/Promote/Session/Webhook 事件进入 Durable Outbox。
- Webhook/其他租户可控 HTTP 出站复用统一 Egress Policy。

### Capability Index and Discovery

归口：EVO-108、109。进入条件：新 data plane 的 Push/Commit provenance 稳定 + Durable Event contract 保持。

### Product Experience Convergence

归口：EVO-121。C/D 已完成；E/B Activity/Dashboard、A/F final Shell/legacy UI removal 依赖真实产品路径逐步恢复。

### Legacy Cleanup

归口：EVO-111、EVO-121-F、EVO-122。Repo-derived read/execute 承接后直接删除 legacy Sandbox/Registry runtime/table，不建设未上线兼容双写。

### Final Production Convergence

归口：EVO-118-E。只有产品目标、legacy cleanup、REL-01 residual 与 **GIT-DP-01** 全部闭合后，才执行最终 clean production build、Embedded Frontend 单一交付、Compose/Gateway 和全协议 smoke，关闭 DEPLOY-01。

## 6. 长期顺序概览

```text
Completed foundation
  SEC-01/02 + DATA-01/02 + EVENT-01 + Repo Web base
        |
        v
EVO-126 A-H / GIT-DP-01
        |
        v
EVO-105 / 106 / 107 / 104
        |
        v
EVO-121-E/B + EVO-108/109
        |
        v
EVO-121-A/F + EVO-111/122
        |
        v
EVO-118-E / DEPLOY-01
```

具体可激活 Story 和依赖顺序以 Current Execution Plan + Product Backlog 为准。

## 7. Release Gates

| Gate | 状态 | Owner |
|------|------|-------|
| SEC-01 | Closed | EVO-118-B |
| SEC-02 | Closed | EVO-118-C |
| DATA-01 | Closed historical filesystem-engine evidence | EVO-118-D |
| DATA-02 | Closed historical lifecycle evidence | EVO-118-F |
| EVENT-01 | Closed | EVO-118-H |
| REL-01 | Partial | EVO-118-G-A residual |
| GIT-DP-01 | **Open** | EVO-126 A-H |
| DEPLOY-01 | Open / final | EVO-118-E |

DATA-01/DATA-02 的历史完成不等于 object-store migration 已完成；GIT-DP-01 是因架构改线新增 Gate。

## 8. 暂缓 / 独立事项

| 事项 | 处理 |
|------|------|
| 独立 WalGit/Axum service | 不采用为目标；保持单一 Evolith control plane |
| LFS 产品化 | WalGit 有机制但不自动纳入 initial cutover；独立需求 |
| Kafka/NATS | Outbox + Worker 继续满足 MVP 事件需求 |
| Live terminal/Yjs | 超出 Vibe Coding MVP |
| Wasmer/WASI | Sandbox 删除后仍有明确 runtime 需求再评估 |
| 独立 P1/P2 债务 | 保留 Backlog，不静默抢占 EVO-126 P0 |

## 9. 计划维护规则

1. 实施只从满足 DoR 的 Backlog Story 启动。
2. 当前激活顺序只由 [CURRENT-EXECUTION-PLAN-2026-09](CURRENT-EXECUTION-PLAN-2026-09.md) 定义。
3. 已发布历史 plan/Iteration 不因改线被覆盖；新工作使用新编号。
4. Current implementation 与 Accepted target 必须分开描述；EVO-126-H 前 filesystem/subprocess 是运行事实。
5. 新 Git publication、数据损坏、跨租户、migration/recovery 或生产构建问题必须进入 P0/P1 Backlog。
6. Gate 只由实际测试、真实 Git client、故障/恢复和 Review 证据关闭，不能由 ADR/代码合并本身关闭。
