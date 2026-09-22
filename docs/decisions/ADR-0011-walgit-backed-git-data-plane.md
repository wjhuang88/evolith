# ADR-0011: WalGit-backed Git Data Plane

## 状态

Accepted（2026-09-10）

## 背景

Evolith 已经完成 filesystem-backed Git Alpha：Repo CRUD、Smart HTTP、真实 clone/pull/push、Repo Context、单实例 Git durability/recovery、Repo lifecycle 与 durable Push Event 均有实现和证据。

当前实现的核心边界是：

```text
Actix API
  -> service-git
     -> git subprocess (Smart HTTP)
     -> gix (Repo Context)
     -> persistent filesystem bare repositories
```

该边界适合单实例 Alpha，但长期存在以下结构性问题：

1. durable Git truth 绑定应用 filesystem，容器/节点与 Git 数据平面耦合；
2. 多实例需要 shared filesystem、repo affinity 或复制策略，PostgreSQL 本身无法共享 Git object；
3. Agent/Vibe Coding 预计产生更高的 clone/fetch 频率，传统 upload-pack 路径成本较高；
4. Evolith 已经需要 Push/Commit/Promote/Audit provenance，WAL sequence 能提供更清晰的 Git-side publication evidence；
5. 当前 `service-git/src/lib.rs` 同时承担 storage helper、subprocess、context read，边界不利于后续演进。

WalGit 提供 Rust crates 级的 Git engine primitives：object-store abstraction、Registry/RepoHandle、WAL + manifest CAS、Git protocol primitives、bundle-uri 等。其 `walgit-server` 又提供成熟的 Smart HTTP orchestration，可在许可证允许范围内吸收和改写。

本决策由用户明确接受为架构改造方向，不作为未成熟 Proposal 继续搁置。

## 选项

### A. 继续维护当前 filesystem + git subprocess

优点：改动最小、现有 Alpha 已验证。

缺点：长期多实例、对象存储、Agent clone 负载、WAL provenance 均需自行设计；会继续扩大已有 storage/subprocess 代码的维护面。

### B. 将完整 `walgit-server` 作为独立服务部署

优点：复用最多，上手快。

缺点：引入第二个 Axum Web 服务、第二套 AppState/Auth/Policy/UI 语义，与 Evolith 当前“模块化单体、单一产品控制面”冲突；授权、租户、Agent、Audit 边界容易分裂。

### C. 直接依赖 WalGit engine crates，并吸收/改写 `walgit-server` 的 Git data-plane orchestration 到 Evolith `service-git`

优点：复用困难的 Git/WAL/storage/protocol 机制，同时保持 Evolith 对产品语义、Actix adapter 和模块化单体的所有权；`service-git` 可成为稳定 anti-corruption layer。

缺点：需要 Rust 1.90、pre-1.0 upstream pinning、较大迁移和新的 object-store 运维边界；需要对衍生代码做许可证治理。

## 决策

采用 **C**。

### 1. Evolith 保持唯一控制面

以下语义继续由 Evolith 拥有：

- Tenant / User / RBAC / API Key / Agent identity；
- Repository metadata 与 lifecycle product semantics；
- `.evolith/policy.yaml` / PolicyEvaluator；
- Agent Session、Commit/Promote、Review/Promotion；
- Audit、Durable Outbox、Webhook、Capability Index；
- Actix HTTP API 与 React 产品 UI。

不得将 WalGit 自带 Authenticator、UI、login/session 或 product policy 当作 Evolith 业务事实源。

### 2. WalGit 提供 Git machinery

计划直接依赖：

- `walgit-store`
- `walgit-config`
- `walgit-git`
- `walgit-wal`
- `walgit-bundle`（在 bundle-uri Story 接入）

不把 `walgit-server` Router/AppState 作为运行时依赖边界。`walgit-server` 中与 Git data plane 相关的 Smart HTTP、pkt-line、streaming、receive/upload-pack orchestration、cache/bundle 逻辑允许按 MIT 许可证吸收、改写到 `service-git`。

### 3. `service-git` 是 Evolith-owned anti-corruption layer

目标依赖方向：

```text
api / application services
       |
       v
Evolith service-git public contracts
       |
       +--> protocol/*
       +--> context/*
       +--> engine/*
       |
       v
walgit-git / walgit-wal / walgit-store / walgit-bundle
```

`api`、核心 domain 和产品 application service 不应直接暴露 `walgit_*` 类型。上游类型只允许存在于 `service-git` 内部 adapter/engine 实现。

### 4. Object store 成为目标 Git durable source of truth

EVO-126-H cutover 后：

- Git objects / refs / WAL / manifest / bundles 的 durable truth 在 object store；
- local bare repo、materialized pack、memory cache 都是 disposable cache；
- PostgreSQL 继续保存 identity、tenant、permissions、repo metadata、Agent/Audit/Outbox/Index；
- 不引入“PostgreSQL 保存 Git object”或新的 DB-centric Git 模型。

这扩展 ADR-0004 的 Git-centric 决策，但改变其“Git repo durable bytes 位于应用文件系统”的实现后果。

### 5. Smart HTTP 决策被替代

ADR-0006 在其历史上下文中继续成立：它解释了为什么 2026-06 的 Alpha 使用 `git` CLI subprocess。

**本 ADR supersedes ADR-0006 作为长期 Smart HTTP 目标。**

在 EVO-126-H cutover 前，subprocess 仍是 current implementation；不得因 ADR 接受就提前描述为已删除。

### 6. Receive-pack 与 Policy/WAL 分层

目标写路径：

```text
git receive-pack request
 -> Evolith auth / tenant / capability
 -> service-git receive protocol parsing
 -> Evolith write-policy hook
 -> pack/connectivity/ref validation
 -> RepoHandle::publish_push
 -> WAL + manifest CAS publication
 -> durable Outbox event / audit provenance
```

WalGit WAL 负责 durability/publication，不接管 Evolith Agent Policy。Agent Scoped Token 继续默认禁止通用 `receive-pack`，Commit/Promote 产品语义仍归 EVO-105/106。

### 7. Repo Context 保持 Evolith 合约

Tree/Blob/Commit/Diff API 的资源上限、tenant hiding、ref reachability 与 Commit Evidence 语义保持不变。底层从“固定 local path + gix”迁移为：

```text
Registry/open -> RepoHandle freshness/sync -> ObjectAccess
    Local  -> gix
    Remote -> WalGit supported remote object path / bounded fallback
```

### 8. Dependency / license governance

- Evolith MSRV 先升级到 Rust 1.90。
- WalGit Git dependencies 必须 pin exact commit SHA；禁止依赖移动 `main`。
- 上游升级单独评审，不在无关 Story 中自动漂移。
- substantial copied/derived code 必须保留 WalGit MIT copyright/permission notice；仓库维护 `THIRD_PARTY_NOTICES.md`。
- WalGit pre-1.0 API 变化由 `service-git` adapter 吸收，不传播到产品层。

## 后果

### 正面

- Git durable truth 从应用节点解耦，为多实例和弹性服务提供更清晰的数据面。
- WAL + manifest CAS 提供明确的 publication point，可与 Evolith durable event/audit 形成更强 provenance。
- 复用成熟 pack/ref/protocol/storage 机制，减少自研 Git server 核心复杂度。
- bundle-uri 与 object store/CDN 更适合 Agent 高频 clone/fetch。
- Evolith 可以把差异化工程集中在 Agent identity、Policy、Commit/Promote、Audit/Discovery。

### 负面与风险

- 架构迁移大，必须保留旧 Alpha 行为直到 cutover 证据完成。
- Rust 版本与依赖树升级会增加一次供应链/兼容性审查。
- WalGit 当前 pre-1.0，内部接口可能变化；exact pin 只能控制漂移，不能消除未来升级成本。
- 对象存储引入新的 availability、latency、credential、cost、versioning/retention 运维问题。
- filesystem repo -> WalGit import 必须避免数据丢失、半迁移和重复导入。

## 迁移 Gate

新增 **GIT-DP-01**：

只有以下全部成立才可关闭：

1. WalGit dependency/toolchain boundary 完成；
2. Registry/Store lifecycle、Smart HTTP read/write、Repo Context 均迁移；
3. 旧 repo migration rehearsal 成功且可重复/恢复；
4. object-store failure readiness/fail-closed 有证据；
5. Push Outbox/Audit provenance 在 WAL 写路径上保持稳定；
6. bundle-uri 与真实 Git client E2E 通过；
7. production default cutover 后不依赖 durable local Git filesystem；
8. legacy Git engine consumer inventory 为零并完成删除/隔离；
9. Security/Navigator/Data review 闭合。

## 与既有 ADR 的关系

- ADR-0004：**保留核心决策，部分实现后果被本 ADR 更新**。Git 继续是代码/历史事实源，但 durable storage 从应用 filesystem 演进为 object-store-backed WalGit repository。
- ADR-0006：**Superseded as target**。历史 Alpha subprocess 决策保留，目标协议实现改由 WalGit primitives + Evolith service-git v2。
- ADR-0007：保持。Agent 写入必须由 Evolith Policy/Scoped Token 控制。
- ADR-0008：保持。Repo-centric 产品交互不因 data plane 改造改变。
- ADR-0009：保持。不为未上线 legacy Registry 建兼容双写。
- ADR-0010：保持。最终 DEPLOY-01 仍在产品/清理/本次 data-plane cutover 后执行。

## 相关链接

- [Project Status Baseline 2026-09-10](../reference/PROJECT-STATUS-BASELINE-2026-09-10.md)
- [WalGit Git Data Plane Refactor Design](../design/WALGIT-GIT-DATA-PLANE-REFACTOR.md)
- [EVO-126 Epic](../backlog/active/EVO-126-walgit-git-data-plane-refactor.md)
- [GitHub Issue #12](https://github.com/wjhuang88/evolith/issues/12)
- [ADR-0004 Git-centric Storage](ADR-0004-git-centric-storage.md)
- [ADR-0006 Smart HTTP via Git subprocess](ADR-0006-smart-http-via-git-subprocess.md)
- [ADR-0007 Agent write boundaries](ADR-0007-agent-write-and-production-delivery-boundaries.md)
- [WalGit](https://github.com/tobi/walgit)
