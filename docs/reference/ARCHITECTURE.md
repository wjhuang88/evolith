# 架构设计

> 本文档描述 Evolith 当前代码、运行边界和已接受的演进方向。当前完成度与发布 Gate 见 [生产就绪与项目完成度基线](PRODUCTION-READINESS-BASELINE.md)；页面编排见 [Product Interaction Architecture](../design/PRODUCT-INTERACTION-ARCHITECTURE.md)；legacy backend 退场见 [ADR-0009](../decisions/ADR-0009-no-prelaunch-registry-compatibility.md)；执行顺序见 [Production Readiness Plan](../roadmap/PRODUCTION-READINESS-PLAN-2026-07.md)。

## 1. 架构定位

Evolith 当前采用**前后端分离开发、单进程交付的模块化单体架构**：

- 后端是 Rust Workspace，通过多个 crate 划分 API、领域、基础设施和业务模块。
- HTTP 发布物是 `evolith` Actix-web 进程；Durable Event 另有同仓库 `outbox-worker` 独立运行入口，最终生产 supervisor/replica 编排仍归后续交付 Gate。
- React + Vite 静态 SPA 通过 `rust-embed-for-web` 嵌入后端发布物。
- Nginx 或平台负载均衡只作为可选 Gateway、TLS 终止和反向代理层。
- SQLite 用于 Lite 开发；PostgreSQL 是生产主数据库。
- Git 仓库位于服务端文件系统；生产必须使用持久存储。

内部多个 `service-*` crate 不代表独立微服务。当前主要问题是权限、数据一致性、事件可靠性和产品闭环，不是服务数量；不得以“架构升级”为名提前拆微服务。

## 2. 当前整体架构

```text
┌──────────────────────────────────────────────────────────────┐
│ Clients                                                      │
│ Web SPA | Git Client | MCP Client | External Agent (future) │
└──────────────────────────────┬───────────────────────────────┘
                               │
┌──────────────────────────────▼───────────────────────────────┐
│ Optional Gateway                                             │
│ TLS termination | reverse proxy | load balancing            │
└──────────────────────────────┬───────────────────────────────┘
                               │
┌──────────────────────────────▼───────────────────────────────┐
│ Evolith Modular Monolith                                     │
│                                                              │
│ API / Middleware                                             │
│ - Cookie JWT / API Key / Git Basic challenge                │
│ - tenant boundary / CSRF / request ID / current rate limit  │
│                                                              │
│ Git Platform                                                 │
│ - Repo CRUD                                                   │
│ - Git Smart HTTP via git subprocess                          │
│ - Repo Context via gix                                       │
│ - policy.yaml parser                                         │
│                                                              │
│ Pre-launch Legacy Modules (pending removal)                  │
│ - Tool / Skill / CLI legacy APIs                             │
│ - HTTP Tool executor                                         │
│ - Docker Sandbox disabled by default, pending removal        │
│                                                              │
│ Embedded Frontend                                             │
│ - React + Vite static assets                                 │
│ - runtime config + SPA fallback                              │
└───────────────┬────────────────────┬─────────────────────────┘
                │                    │
       ┌────────▼────────┐   ┌───────▼─────────────────────┐
       │ Metadata Stores │   │ Git Repository Storage      │
       │ SQLite / PG     │   │ persistent filesystem      │
       │ Redis optional  │   │ current code: local path   │
       └─────────────────┘   └─────────────────────────────┘
```

### 当前成熟度说明

- Repo CRUD、Smart HTTP 和 Context API 是可用的 Git 后端 Alpha 基础。
- Repo UI Shell 已实现；Repo Detail/Commit Evidence、Commit/Promote、Agent Session、Webhook、Vibe Coding、Activity 和 Indexer 仍在 Backlog。
- API Key/MCP 授权、HTTP Tool SSRF 与 Git 数据持久化 Gate 已关闭；生产构建 DEPLOY-01 仍开放。
- 因此“基础能力已实现”不能等价为“外部 Alpha/生产可用”。

## 3. 事实源与数据边界

### 3.1 Git Repository

Git 是以下数据的事实源：

- 代码和文件内容；
- Commit、Branch、Tag、Ref 和历史；
- Repo 内 `.evolith/*`、`SKILL.md`、`interface.yaml`、`tool.yaml` 等能力描述。

当前已实现：

- Repo 创建、查询、更新和删除；
- Smart HTTP `info/refs`、`git-upload-pack`、`git-receive-pack`；
- 真实 Git 客户端 `clone` / `pull` / `push`；
- Repo Context：Tree、Blob、Commit、Diff；
- Push 后默认分支元数据同步。

Smart HTTP 使用 Git subprocess；Repo Context 使用 `gix`。两条路径职责不同，不应为“统一库”而强行合并。

### 3.2 PostgreSQL / SQLite

数据库存储：

- 用户、租户、角色、凭证；
- Repo 元数据和生命周期状态；
- 审计、Agent Session、Outbox/Event（目标）；
- Capability Index；
- 当前仍存在、但由 ADR-0009 / EVO-122 计划删除的 legacy Tool / Skill / CLI 数据。

PostgreSQL 是生产主路径；SQLite 是 Lite 开发和本地测试。Schema、Repository 行为和测试必须双轨考虑，但允许对生产并发语义明确记录 SQLite 限制。

### 3.3 DB 与 Git 文件系统一致性

当前 Repo 创建/删除仍由 Handler 顺序编排 DB 与文件系统，存在分裂风险。目标不是跨介质 ACID，而是：

```text
State + Compensation + Reconciler
```

推荐生命周期：

```text
CREATING -> ACTIVE
        \-> ERROR -> retry/cleanup
ACTIVE -> DELETING -> trash -> removed
                  \-> ERROR -> reconcile
```

- 创建失败不能返回可用 Repo。
- 删除失败不能形成数据库不可追踪的磁盘孤儿。
- Seed Template 必须形成真实 Blob/Tree/Initial Commit，而不是写入 bare repo 根目录。
- 对账工具必须识别 DB-only、disk-only 和正常 Repo。

归口：EVO-118-F。

### 3.4 Git Storage 部署边界

- 单实例可以使用本地持久卷。
- 容器可写层不能作为 Git 事实源。
- 多实例必须采用共享/一致存储、Repo affinity 或明确复制策略。
- 仅共享 PostgreSQL 不能共享 Git 对象。
- 备份必须同时覆盖 PostgreSQL 和 Git Repo/Ref/Object。
- Readiness 应检查 Git Storage 可用性。

当前生产 Compose 和备份尚未满足这些条件，归口 EVO-118-D。

### 3.5 Redis 与对象存储

- Redis 不是核心事实源。
- 普通缓存可明确降级；Session、限流、分布式锁等安全状态不得静默进程内降级。
- MinIO 不是当前 Git 主存储；LFS/制品如需对象存储应独立设计。

## 4. 后端模块与依赖方向

```text
backend/
├── src/main.rs
├── crates/
│   ├── api/             # routes / handlers / DTO / middleware / AppState
│   ├── domain/          # domain models / repository traits
│   ├── infra/           # config / DB / cache / mail / storage adapters
│   ├── common/          # errors / logs / sanitize / shared execution
│   ├── service-git/     # gix context and git service boundary
│   ├── service-auth/    # JWT / password / partial auth services
│   ├── service-audit/
│   ├── service-tool/    # MCP / HTTP Tool compatibility
│   ├── service-skill/   # parser + legacy Sandbox facade
│   ├── service-snippet/ # legacy CLI compatibility
│   └── service-payment/
├── migrations/sqlite/
├── migrations/postgres/
└── sandbox/             # legacy, pending EVO-111
```

当前依赖基本方向：

```text
api -> domain/service boundaries -> infra implementations
```

### AppState 边界

`AppState` 当前直接持有多个 Repository、Cache、Mailer、Executor 和 Git Storage Path。该模式在早期可接受，但跨层工作流不应继续堆入 Handler。

目标 Application Service：

- `RepoApplicationService`
- `GitWriteService`
- `CredentialService`
- `PolicyEvaluator`
- `AgentSessionService`
- `CapabilityIndexer`
- `WebhookDeliveryService`

引入 Application Service 仍属于模块化单体演进，不代表拆微服务。

## 5. 前端架构与产品状态

技术路线：

```text
React + Vite + TypeScript + Tailwind
React Router + Zustand + TanStack Query + Axios + i18next
Bun package manager and script runtime
```

当前用户路由包括：

- Dashboard；
- Repositories list/create；
- Tools / Skills / Interfaces；
- Tenant settings / members / billing / API keys。

Repo Detail、Commit Evidence、Vibe Coding Workspace、Discover 和 Activity 尚未实现。因此当前前端应描述为 Repo UI Shell + pre-launch legacy UI，而不是完整 AI-native Git 产品；目标页面编排见 Product Interaction Architecture，旧 UI 由 EVO-121-F 直接删除。

目标构建顺序：

```text
frontend build -> frontend/dist -> Rust embed -> single runtime image
```

生产 Docker/Compose 仍需收敛为单一事实交付路径，归口 EVO-118-E。

## 6. 关键请求流程

### 6.1 Web API

```text
Browser
  -> Gateway
  -> Actix middleware
  -> Authentication
  -> Authorization / Tenant / CSRF / Rate Limit
  -> Application Service or Handler
  -> Repository / GitStorage / Outbox
```

目标是将授权、资源和副作用编排从 Handler 收敛到单一服务边界。

### 6.2 Git Smart HTTP

```text
Git Client
  -> /repos/{repo}/info/refs or service endpoint
  -> Basic challenge / credential resolution
  -> tenant + repo + operation authorization
  -> bounded git subprocess streaming
  -> persistent Git Storage
  -> metadata/outbox/reconcile
```

已实现 subprocess timeout、Upload Pack 流式响应、Receive Pack 有界结果、Repo tenant 检查和
Push durable producer/reconcile。Receive Pack 成功响应前必须完成 typed Outbox enqueue；Worker
按当前 Git ref 幂等刷新 metadata，旧/乱序事件只做 no-op。

待硬化：

- Ref/Branch/Path scope；
- protected branch / force push / delete ref；
- Push/body/repository quotas；
- Agent Token 与人类 Token 分离；
- Durable post-push event。

建议：人类凭证可在明确策略下使用 Smart HTTP；Agent Scoped Token 默认走 Commit/Promote API，不直接获得通用 `receive-pack`。

### 6.3 Repo Context

```text
Authorized Client
  -> Repo metadata + resource scope
  -> web::block / timeout
  -> gix object access
  -> bounded response
```

Tree、Blob、Commit、Diff 上限必须继续保留，且在读取/遍历过程中生效。

### 6.4 MCP HTTP Tool

当前 MCP `tools/call` 可触发真实出站 HTTP。EVO-118-B/C 已关闭 Typed Capability、tenant hiding 与 Egress/SSRF Gate；ADR-0009 / EVO-122-A 将进一步把执行身份从旧 Tool 行收敛为 Repo/Commit/Path provenance，且不得回退现有安全边界。

目标流程：

```text
MCP Client
  -> valid credential
  -> explicit ToolExecute capability
  -> Tool tenant/resource authorization
  -> EgressPolicy
       - scheme
       - DNS/final IP
       - private/loopback/metadata block
       - redirect revalidation
       - timeout/size/concurrency
  -> external target
  -> audit
```

归口：EVO-118-B/C。

### 6.5 Commit / Promote / Agent Session（目标）

```text
External Agent
  -> create Agent Session
  -> receive scoped/revocable token
  -> read Repo Context
  -> Commit API on agent branch
  -> PolicyEvaluator
       -> block
       -> require_review
       -> auto_merge
  -> Promote
  -> durable audit/outbox
```

`.evolith/policy.yaml` 目前已有 Parser，不代表策略执行已经完成。三态行为需由 EVO-105 的测试证明。

### 6.6 Durable Event（目标）

Webhook、Indexer、Audit 派生和 Agent Event 不应依赖进程内 `spawn`：

```text
DB transaction
  -> update metadata
  -> insert outbox event
Worker
  -> claim
  -> deliver with idempotency
  -> retry/dead-letter/replay
```

目标是 at-least-once + 幂等，不追求跨系统 exactly-once。归口 EVO-118-H。

当前 H-A/H-B/H-C 基础已具备：SQLite/PostgreSQL Outbox claim 使用有界 lease 与 fencing token；
过期 claim 可恢复，旧 Worker ACK/NACK 不能覆盖新 owner，最后一次 attempt 过期进入
dead-letter。独立 `outbox-worker` 支持 continuous、one-batch 与 confirmed replay，强制
`batch × delivery timeout < lease`，优雅停止等待当前有界 batch，日志和 `last_error` 只暴露
稳定错误码。H-C 已注册 `repo.push.completed.v1` producer/subscriber：payload 只含非敏感
Repo/tenant/ref/commit facts，idempotency key 由 Repo/default branch/commit 派生，Worker
以当前 Git ref 防止旧事件回写；生产 supervisor 仍归 EVO-118-E，EVENT-01 在 Commit/Promote、
Webhook、Indexer 等后续 producer/consumer 约束同步前保持开放。

## 7. 安全边界

### 已具备的基础

- Argon2id 密码哈希；
- httpOnly Cookie JWT；
- double-submit Cookie CSRF；
- API Key 哈希存储和过期/撤销字段；
- Git Basic challenge、tenant/repo ownership check；
- Request ID、结构化日志和部分敏感字段清洗；
- Repo Context 资源上限；
- HTTP Tool 响应体上限；
- 安全响应头。

### 尚未解除的 Gate

- API Key 管理角色和任意权限字符串；
- MCP `execute` capability；
- HTTP Tool SSRF/DNS/Redirect/Egress；
- Agent branch/path scope 与 Policy enforcement；
- JWT Session/revocation/role refresh；
- 分级限流接线；
- SMTP/Redis 生产降级语义。

所有相关变更必须遵守 [Security Review SOP](../sop/SECURITY-REVIEW.md)。

## 8. 可靠性与运行边界

### Health

目标：

- `/health/live`：只表示进程存活，200。
- `/health/ready`：检查 DB、Git Storage 和适用关键依赖；失败返回 503。

当前 readiness 语义仍需 EVO-118-G 修复。

### Events

- 可丢失的非关键辅助任务可使用进程内异步任务。
- 影响 Webhook、Indexer、审计或用户可见状态的事件必须持久化。

### Mail

- Development 可以 ConsoleMailer，但不得记录完整 Token/链接。
- Production SMTP 声明启用但初始化失败时 fail closed。
- 业务交付建议使用邮件 Outbox/重试，避免假成功。

## 9. Legacy 与迁移边界

| 能力 | 当前判断 |
|------|----------|
| Git Repo / Smart HTTP / Context | 当前主线，后端 Alpha 基础已具备 |
| Repo UI / Commit / Session / Vibe Coding | 产品主线，按 EVO-118-F/G/H 直接边界推进；最终发布归 EVO-118-E |
| Skill / MCP / CLI Indexer | 计划由 Git Repo 派生 |
| DB-centric Registry | Pre-launch legacy，ADR-0009 决定不建设兼容层；由 EVO-121-F/EVO-122 删除 |
| Docker Skill Sandbox | 默认关闭，ADR-0005 已决定删除 |
| Snippet 模型 | historical/compatibility，不作为新产品概念 |

新功能不得以 legacy Sandbox 或旧 Registry 为硬依赖，也不得新增双写、fallback 或旧 UUID 身份。

## 10. 发布判断

当前阶段：

> **Git backend Alpha foundation / Production release blocked**

环境准入、Gate 和发布验证以 [Production Readiness Baseline](PRODUCTION-READINESS-BASELINE.md) 与 [Release SOP](../sop/RELEASE.md) 为准。

以下说法在 Gate 关闭前不成立：

- “已经是可生产部署的 Git 平台”；
- “API Key 权限体系已完整”；
- “HTTP Tool 已安全支持任意 URL”；
- “备份脚本可恢复 Evolith 全量数据”；
- “Policy Parser 完成等于 Agent 策略已经生效”；
- “文件嵌入代码存在等于生产镜像构建链路已经闭环”。
