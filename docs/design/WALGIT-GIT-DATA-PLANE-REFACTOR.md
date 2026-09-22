# WalGit-backed Git Data Plane Refactor Design

> 状态：Accepted target design / not yet implemented
> 决策：[ADR-0011](../decisions/ADR-0011-walgit-backed-git-data-plane.md)
> 执行：[EVO-126](../backlog/active/EVO-126-walgit-git-data-plane-refactor.md) / [GitHub #12](https://github.com/wjhuang88/evolith/issues/12)
> 当前事实：[Project Status Baseline 2026-09-10](../reference/PROJECT-STATUS-BASELINE-2026-09-10.md)

本文描述目标实现结构。除明确标注 Current 的部分外，本文件不得作为“代码已完成”的证据。

## 1. 设计目标

1. Object store 成为 Git durable source of truth，本地 repo/pack 只作为可丢弃 cache。
2. `service-git` 成为 Evolith-owned、framework-neutral 的 Git platform boundary。
3. 复用 WalGit 的 store/git/wal/bundle primitives，吸收 `walgit-server` 中成熟的 Git protocol orchestration。
4. 不吸收 WalGit 产品控制面：Axum router、login/auth、UI、product policy 不进入 Evolith domain。
5. 保持 Evolith tenant/RBAC/Agent/Policy/Commit/Promote/Audit/Outbox 的语义所有权。
6. 在不降低现有真实 Git client、Repo Context、Commit Evidence、安全和 durable event 门禁的前提下完成迁移。

## 2. Current Architecture

```text
Git Client / Browser / Future Agent
             |
             v
      Evolith Actix API
             |
      auth / tenant / RBAC
             |
             v
        service-git
       /           \
 git subprocess    gix
 upload/receive    Context
       \           /
        local bare repo
             |
     persistent filesystem
```

当前 `service-git` 同时管理路径、bare repo 初始化、seed、删除、git subprocess 与 gix read。该实现已经证明 Alpha 能力，但 storage/protocol/context 责任聚合过重。

## 3. Target Architecture

```text
Clients
  |  Web API / Git Smart HTTP / Agent APIs
  v
Evolith Actix Adapters
  |  authentication + tenant + capability + request limits
  v
Application Services
  |  RepoApplicationService / GitWriteService / PolicyEvaluator
  v
service-git v2  -----------------------------+
  |                                             |
  | Evolith-owned contracts                     | Durable event facts
  |                                             v
  +-- protocol/                         Outbox / Audit
  |    +-- info_refs
  |    +-- upload_pack
  |    +-- receive_pack
  |    +-- pktline / stream
  |
  +-- context/
  |    +-- tree / blob / commit / diff
  |
  +-- engine/
  |    +-- repo identity mapping
  |    +-- registry/store bootstrap
  |    +-- WAL publish adapter
  |
  +-- bundle/
       +-- bundle-uri advertisement/list/serve
             |
             v
 walgit-git / walgit-wal / walgit-store / walgit-bundle
             |
       Object Storage
       Git durable truth
             |
        local cache
      disposable only
```

## 4. Crate Ownership

### 4.1 Evolith owns

`service-git` public API 必须使用 Evolith 类型。例如：

```rust
pub struct GitRepositoryIdentity {
    pub tenant_id: Uuid,
    pub repository_id: Uuid,
}

pub struct AuthorizedGitContext {
    pub tenant_id: Uuid,
    pub repository_id: Uuid,
    pub principal: GitPrincipal,
    pub capabilities: GitCapabilities,
}
```

具体字段可在 Story refinement 中调整，但核心规则不变：调用方不需要认识 `walgit_git::RepoId` 或 `walgit_wal::RepoHandle`。

### 4.2 WalGit owns machinery

优先直接依赖：

| Crate | 用途 |
|------|------|
| `walgit-store` | `ObjectStore` / CAS / S3/GCS-compatible backend |
| `walgit-config` | WalGit engine config object；由 Evolith config adapter 构造 |
| `walgit-git` | RepoId、LocalRepo、pack/ref/protocol primitives |
| `walgit-wal` | Registry、RepoHandle、sync、publish、manifest/WAL |
| `walgit-bundle` | bundle generation/list/serve |

禁止以 `walgit-server::AppState` 作为 Evolith service state。

## 5. `walgit-server` Adoption Matrix

| WalGit server concern | Evolith 处理 | 原因 |
|-----------------------|--------------|------|
| Smart HTTP protocol v0/v2 orchestration | **Absorb/adapt** 到 `service-git/protocol` | Git machinery，可复用成熟行为 |
| pkt-line helpers | **Absorb/adapt** 或直接复用稳定 lower-level API | 与 Web framework 无关 |
| request/response streaming helpers | **Absorb/adapt** 为 framework-neutral IO | 保留 backpressure/streaming，不绑定 Axum |
| info/refs advertisement cache | **Absorb design** | manifest version 可作为 cache key |
| receive-pack parse/report-status | 优先 **direct walgit-git dependency** | 已是 lower-level crate contract |
| RepoHandle sync/publish | **direct walgit-wal dependency** | 核心 WAL/data plane contract |
| Bundle URI | direct `walgit-bundle` + adapt server orchestration | Agent clone 优化 |
| Axum Router / Response / HeaderMap | **Do not adopt** | Evolith 使用 Actix；adapter 留在 `api` |
| WalGit Authenticator/OIDC/session | **Do not adopt** | Evolith 已有 auth/RBAC/Agent identity |
| WalGit product policy | **Do not adopt as authority** | Evolith `.evolith/policy.yaml` / PolicyEvaluator 是 owner |
| WalGit web UI/login/admin pages | **Do not adopt** | 与 Evolith 产品层冲突 |
| WalGit webhook/product events | selective mechanism reference only | Evolith Durable Outbox/Event 是稳定 owner |

## 6. Proposed `service-git` Layout

```text
backend/crates/service-git/src/
├── lib.rs
├── error.rs
├── types.rs
├── engine/
│   ├── mod.rs
│   ├── identity.rs
│   ├── registry.rs
│   ├── store.rs
│   └── publish.rs
├── protocol/
│   ├── mod.rs
│   ├── smart_http.rs
│   ├── pktline.rs
│   ├── upload_pack.rs
│   ├── receive_pack.rs
│   └── stream.rs
├── context/
│   ├── mod.rs
│   ├── tree.rs
│   ├── blob.rs
│   ├── commit.rs
│   └── diff.rs
├── policy/
│   └── push.rs
└── bundle/
    ├── mod.rs
    └── uri.rs
```

不要为了匹配目录机械拆文件；最终实现以清晰 owner 为准。原则是 `protocol/context/engine` 分层，不再把所有逻辑堆入单个 `lib.rs`。

## 7. Repository Identity Mapping

Evolith UUID 保持稳定外部/数据库 identity。WalGit physical RepoId 不成为用户命名事实源。

推荐映射：

```text
Evolith tenant UUID / repository UUID
           |
           v
WalGit RepoId("<tenant-uuid>/<repository-uuid>")
```

优点：

- 用户重命名 repo slug 不移动 durable object namespace；
- tenant/repo ownership 可从 DB 先验证，再生成 physical ID；
- URL 中的 repo name 永远不能直接成为 object-store prefix；
- 避免跨租户同名冲突。

若 WalGit RepoId validation/length 对 UUID mapping 有变化，在 EVO-126-B 以测试固定 stable encoding，不在业务层散落拼接逻辑。

## 8. Repository Lifecycle

### 8.1 Create

```text
DB transaction: repository = CREATING
       |
       v
service-git engine.create
       |
       v
WalGit Registry::create
       |
manifest CAS create in object store
       |
seed real initial commit through Git engine
       |
       v
DB: ACTIVE + initial commit metadata/outbox
```

失败处理保持 `State + Compensation + Reconciler`，但 reconcile 对象从 `disk directory` 改成 `WalGit manifest/repo namespace`。

### 8.2 Delete

```text
DB: DELETING
   -> make repository unavailable at product boundary
   -> delete/retire WalGit manifest namespace
   -> cleanup unreferenced objects according to engine/store semantics
   -> DB removed/tombstoned
```

删除必须先定义 linearization point；不能因 object store list/delete 部分失败而返回假成功。

## 9. Smart HTTP Read Flow

```text
GET info/refs / POST upload-pack
 -> Actix authentication
 -> tenant/repo lookup
 -> AuthorizedGitContext
 -> service-git protocol
 -> Registry::open
 -> RepoHandle::sync_refs / sync_objects
 -> walgit-git protocol primitives
 -> bounded streaming Git response
```

关键不变量：

- advertisement 只需要 refs 时不能先强制 materialize 全 repo；
- streaming response 生命周期内必须持有必要 read guard，避免 pack 被回收；
- cache key 必须与 manifest/version/freshness 绑定；
- Git protocol v0/v2 都有真实 client E2E。

## 10. Receive-pack / WAL Write Flow

```text
POST git-receive-pack
 -> Evolith auth/tenant/capability
 -> parse receive commands + pack
 -> Evolith push policy hook
 -> ingest/connectivity/ref validation
 -> RepoHandle::publish_push
 -> immutable WAL objects
 -> manifest CAS publication point
 -> PublishResult(seq, per-ref result)
 -> durable repo.push.completed.v1 Outbox
 -> report-status response
```

### 10.1 成功定义

HTTP/Git 成功不能早于：

1. required Git publication 已 durable；
2. Ref transaction 已得到最终 per-ref result；
3. Evolith 当前稳定契约要求的 Push durable producer 已 enqueue/commit。

如果 Outbox 与 object-store publication 无法做跨系统 ACID，不伪造 exactly-once；必须明确 state/reconcile/idempotency 补偿策略，并由 EVO-126-D 负向测试证明。

### 10.2 Agent write boundary

Agent Scoped Token 默认不允许通用 Smart HTTP `receive-pack`。WalGit capability 不能绕过 Evolith PolicyEvaluator。EVO-105/106 在 GIT-DP-01 后基于 `GitWriteService/service-git` 接入 Commit/Promote。

## 11. Repo Context Flow

```text
Repo Context request
 -> Evolith auth/tenant/reachability
 -> Registry/open
 -> RepoHandle sync_objects
 -> ObjectAccess
      Local  -> gix repository read
      Remote -> WalGit remote-object capability or explicit bounded materialization
 -> Evolith DTO
```

现有 `BLOB_MAX_BYTES`、tree/diff entry limit、blocking/wall-clock timeout、commit reachability 等安全门禁必须保留或加强。

## 12. Bundle URI

Agent/CI 高频 clone 是本重构的重要 workload。目标：

```text
Git clone
  +-- control/negotiation -> Evolith service-git
  +-- bundle bytes -------> authorized object-store/CDN path
  +-- remainder ----------> upload-pack
```

必须解决：

- repository read authorization；
- signed/short-lived URL 或 equivalent access model；
- bundle schedule/retention；
- blobless/filter compatibility；
- fallback rate/cost/observability；
- bundle 不得成为长期公开绕过 auth 的静态 URL。

## 13. Configuration Boundary

Evolith config 是外部 owner；不要要求部署者同时理解两套互相冲突的配置事实源。

目标示意：

```text
GIT_ENGINE__TYPE=walgit
GIT_ENGINE__CACHE_DIR=...
GIT_ENGINE__STORE__BACKEND=s3|gcs|...
GIT_ENGINE__STORE__BUCKET=...
GIT_ENGINE__STORE__PREFIX=...
```

最终命名由 EVO-126-F 按现有 nested config 风格确定。`walgit_config::Config` 应由 infra/service adapter 从 Evolith config 构造，不直接把 `walgit.toml` 变成第二个生产配置 owner。

Secrets/credentials 不得进入 Repo metadata、WAL metadata、audit payload 或 Git error body。

## 14. Data Migration

现有 repo 不允许“新建空 WalGit repo 然后切 URL”。迁移必须是一等流程：

1. inventory DB repo + filesystem bare repo；
2. 验证 refs/object connectivity；
3. import 到 WalGit object store；
4. 比较 refs/tags/default branch/object format；
5. clone/fetch/context smoke；
6. 标记 migration state；
7. cutover routing；
8. 保留旧 repo 只读安全窗口，随后按 H Story 的清理策略删除。

Migration tool 必须支持 dry-run、重复执行检测、失败恢复和明确 exit code。

## 15. Readiness / Durability / Recovery

旧模型：检查 Git storage path/volume + DB，联合备份 PostgreSQL 与 repo filesystem。

目标模型：

- readiness 验证 object store 必需 read/write/CAS capability；
- cache path 丢失不应使 durable repo 丢失；
- DB restore 与 Git object store 的一致性点必须记录 publication/revision 边界；
- object-store versioning/retention/replication 与 Evolith backup responsibility 明确；
- destructive restore 默认禁止覆盖非空目标，沿用 DATA-01 fail-safe 原则；
- migration 和 recovery 都要以真实 Git clone + Context read 验证，而不是只检查 object keys。

## 16. Event / Provenance Model

目标 Push event 建议扩展稳定 metadata：

```text
repository_id
tenant_id
ref
before_sha
after_sha
wal_seq
manifest_revision (if stable/publicly available)
principal_type
request_id
```

不要把敏感 token、raw auth header、object-store credential 写入 WAL metadata/outbox/audit。

`wal_seq` 用于 Git-side provenance，不替代 Outbox event id/fencing/idempotency key。

## 17. Security Review Checklist

EVO-126-C/D/F/G/H 必须按 `SECURITY-REVIEW.md` 复核：

- URL/path -> RepoId 不可注入任意 object prefix；
- cross-tenant hiding 保持；
- Agent generic write fail closed；
- force/delete/protected ref 策略归口明确；
- pack/object/body/timeout/concurrency 上限；
- object-store credential least privilege；
- signed bundle URL 不泄漏长期访问；
- CAS conflict/retry 不产生重复成功；
- cache poisoning/old manifest 不导致 stale write；
- logs/WAL metadata 无 secret。

## 18. Test Matrix

### Engine contract

- create/open/delete/not-found/already-exists；
- cache empty restart；
- store unavailable/retry/CAS conflict；
- object format mapping。

### Git protocol

- clone/fetch/pull v0/v2；
- push success/reject/conflict/delete/force rules；
- large body/timeout/client disconnect；
- bundle hit/fallback。

### Context

- tree/blob/commit/diff；
- reachability/404；
- resource limits；
- cache missing/remote object path。

### Lifecycle / recovery

- old filesystem repo import；
- interrupted import + retry；
- DB-only / object-store-only inventory；
- app/cache deletion + restart；
- object-store outage -> readiness 503；
- recovery -> clone/context/push smoke。

### Event

- push publication -> Outbox durable event；
- worker crash/replay；
- stale/duplicate event idempotency；
- WAL seq provenance preserved。

## 19. Rollout

| Phase | Story | Exit |
|------|-------|------|
| 0 | Governance baseline + ADR-0011 | Current/target/plan/issue owners consistent |
| 1 | EVO-126-A | Rust 1.90 + exact WalGit pin + license boundary |
| 2 | EVO-126-B | service-git v2 + Store/Registry lifecycle |
| 3 | EVO-126-C | Smart HTTP read E2E |
| 4 | EVO-126-D | WAL receive-pack + durable Push event |
| 5 | EVO-126-E | Repo Context parity |
| 6 | EVO-126-F | object-store config/readiness/migration/recovery |
| 7 | EVO-126-G | bundle-uri secure clone path |
| 8 | EVO-126-H | default cutover + legacy Git engine removal + GIT-DP-01 close |

实施时仍按 Story WIP，不把 3~7 合并成一个不可审查 PR。

## 20. Explicit Non-goals

- 不拆 Evolith 为 control-plane/data-plane 两个强制独立部署服务。
- 不使用 WalGit UI 取代 Evolith Repo UI。
- 不使用 WalGit Auth 取代 Evolith Tenant/RBAC/Agent identity。
- 不在 EVO-126 实现完整 Agent Session、Commit/Promote 或 Vibe Coding 产品功能。
- 不因 WalGit 已有 LFS 就自动把 LFS 纳入 initial cutover；需要独立需求/验收。
- 不声明 object-store 架构天然“生产就绪”；GIT-DP-01 与最终 DEPLOY-01 必须以证据关闭。
