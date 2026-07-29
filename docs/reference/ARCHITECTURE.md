# 架构设计

> 本文档描述 Evolith 当前代码与运行边界。历史方案、待实施能力和重大取舍分别归口到 `docs/archive/`、`docs/roadmap/`、`docs/proposals/` 和 `docs/decisions/`。

## 1. 架构定位

Evolith 当前采用**前后端分离开发、单进程交付的模块化单体架构**：

- 后端是 Rust Workspace，通过多个 crate 划分 API、领域、基础设施和业务模块。
- 默认发布物是一个 `evolith` Actix-web 进程。
- React + Vite 静态 SPA 通过 `rust-embed-for-web` 嵌入后端发布物。
- Nginx 或平台负载均衡仅作为可选网关、SSL 终止和反向代理层。
- SQLite 用于 Lite 开发；PostgreSQL 是生产主数据库。
- Git 仓库位于服务端文件系统，业务元数据位于数据库。

内部存在多个 `service-*` crate 不代表它们是独立部署的微服务。只有在未来明确拆分进程、网络协议、部署生命周期和故障边界后，才应使用“微服务”描述。

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
│ Evolith Rust Application                                     │
│                                                              │
│ API / Middleware                                             │
│ - Cookie JWT / API Key / Git Basic challenge                │
│ - RBAC / tenant boundary / CSRF / rate limit / audit         │
│                                                              │
│ Git Platform                                                 │
│ - Repo CRUD and filesystem lifecycle                         │
│ - Git Smart HTTP via git subprocess                          │
│ - Repo Context via gix: tree / blob / commit / diff          │
│ - .evolith/policy.yaml parsing                               │
│                                                              │
│ Compatibility Modules                                        │
│ - Tool / Skill / CLI legacy APIs                             │
│ - Docker Sandbox disabled by default, pending EVO-111        │
│                                                              │
│ Embedded Frontend                                             │
│ - React + Vite static assets                                 │
│ - SPA fallback                                                │
└───────────────┬────────────────────┬─────────────────────────┘
                │                    │
       ┌────────▼────────┐   ┌───────▼─────────────────────┐
       │ Metadata Stores │   │ Repository Storage          │
       │ SQLite / PG     │   │ Git repositories on disk   │
       │ Redis optional  │   │                             │
       └─────────────────┘   └─────────────────────────────┘
```

## 3. 事实源与数据边界

### 3.1 Git 仓库

Git 仓库是代码、版本历史和仓库内 Agent 能力描述的事实源。

当前 Git 路径包括：

- Repository 创建、查询、更新和删除。
- Smart HTTP `info/refs`、`git-upload-pack`、`git-receive-pack`。
- 真实 Git 客户端认证挑战与 `clone` / `pull` / `push`。
- Repo Context：文件树、Blob、Commit 和 Diff。
- Push 后元数据同步。

Smart HTTP 使用 Git subprocess 处理协议流；Repo Context 使用 `gix` 读取对象和历史。两者承担不同职责，不应混写为同一实现。

### 3.2 数据库

数据库存储用户、租户、权限、API Key、审计、仓库元数据以及仍处于兼容期的 Tool / Skill / CLI 数据。

- SQLite：Lite 开发和多数本地测试。
- PostgreSQL：生产主路径。
- 数据库结构或 Repository 行为变化必须同时覆盖两套 migration、实现和测试。
- MySQL 只有部分依赖入口，没有完整 Repository 实现，主服务不支持以 MySQL 启动。

### 3.3 文件系统

当前 Git 仓库存储在服务端文件系统。由此产生以下部署边界：

- 单实例部署可直接使用本地持久卷。
- 多实例部署必须提供共享或一致的仓库存储，或采用明确的请求路由与副本策略。
- 仅共享 PostgreSQL 不能让多个实例自动共享 Git 对象。

当前代码不应被描述为已经具备无状态水平扩展能力。

### 3.4 Redis 与 MinIO

- Redis 是可配置的缓存基础设施，不是核心事实源。
- MinIO 仍出现在部分 Compose 和早期对象存储设计中，但不是当前 Git 仓库主存储路径。
- 新的 Git-centric 能力不要默认依赖 MinIO；确需对象制品或 LFS 时应通过独立设计和 Backlog 进入。

## 4. 后端模块

```text
backend/
├── src/main.rs                 # 启动、配置、数据库分支、中间件和路由装配
├── crates/
│   ├── api/                    # HTTP 路由、handler、DTO、中间件、AppState
│   ├── domain/                 # 领域模型和 Repository trait
│   ├── infra/                  # 配置、SQLite/PG、缓存、邮件、存储适配
│   ├── common/                 # 错误、日志、清洗和通用工具
│   ├── service-git/            # gix 仓库上下文与 Git 领域能力
│   ├── service-auth/           # 认证相关服务
│   ├── service-audit/          # 审计服务
│   ├── service-tool/           # MCP Tool 兼容与执行能力
│   ├── service-skill/          # Skill parser + legacy Sandbox 执行层
│   ├── service-snippet/        # legacy Snippet / CLI Interface 兼容模块
│   └── service-payment/        # Stripe 计费集成
├── migrations/
│   ├── sqlite/
│   └── postgres/
└── sandbox/                    # legacy 镜像，待 EVO-111 删除
```

依赖方向遵循：

```text
api -> service/domain traits -> infra implementations
```

API 层通过 `AppState` 持有共享依赖。领域层定义 Repository trait，基础设施层分别提供 SQLite 和 PostgreSQL 实现。

## 5. 前端架构

前端当前技术路线：

```text
React + Vite + TypeScript + Tailwind
React Router + Zustand + TanStack Query + Axios + i18next
Bun package manager and script runtime
```

关键入口：

| 路径 | 职责 |
|------|------|
| `frontend/src/main-spa.tsx` | SPA 入口与路由树 |
| `frontend/src/app/` | 页面 |
| `frontend/src/components/` | UI 和功能组件 |
| `frontend/src/lib/api/client.ts` | Axios、CSRF、刷新和错误处理 |
| `frontend/src/stores/authStore.ts` | 认证状态 |
| `frontend/src/locales/` | `zh-CN` / `en` 资源 |

生产构建先生成 `frontend/dist/`，再由 Rust 构建过程嵌入。后端同时提供 API、健康检查和静态资源，需要保持以下路径互不截获：

- `/api/v1`
- `/repos/` Git Smart HTTP
- `/mcp`
- `/health`
- `/assets/`
- SPA 深层路由 fallback

## 6. 关键请求流程

### 6.1 Web API

```text
Browser
  -> optional gateway
  -> Actix middleware
  -> auth / tenant / RBAC / CSRF / rate limit
  -> handler
  -> Repository trait
  -> SQLite or PostgreSQL implementation
```

浏览器认证主要使用 httpOnly Cookie JWT；状态变更请求受 double-submit Cookie CSRF 保护。

### 6.2 Git Smart HTTP

```text
Git Client
  -> /repos/{repo}/info/refs or service endpoint
  -> Basic challenge / credential extraction
  -> tenant and repository authorization
  -> bounded git subprocess streaming
  -> repository filesystem
  -> push metadata synchronization and audit
```

Git 客户端路径不使用浏览器 CSRF 模式，但必须执行独立认证和仓库授权。

### 6.3 Repo Context

```text
Authorized Client
  -> Repo Context API
  -> repository metadata and scope check
  -> gix object access
  -> tree/blob/commit/diff response
```

资源上限必须在读取前或迭代过程中生效，不能先完整读取或完整收集后才判断超限。

### 6.4 MCP Tool

MCP `tools/call` 当前可以触发真实 HTTP 出站请求，必须要求有效 API Key，并遵守租户和 Tool 可见性边界。Function executor 尚不是完整主路径。

## 7. 安全边界

当前安全基线包括：

- Argon2id 密码哈希。
- httpOnly Cookie JWT。
- double-submit Cookie CSRF。
- API Key 与 scope 检查。
- Git Basic 认证挑战和仓库级授权。
- RBAC、资源所有权与多租户隔离。
- 审计日志、请求 ID、结构化日志和敏感字段清洗。
- 认证与未认证请求分级限流。
- Repo Context 的 Blob、Tree 和 Diff 资源边界。
- 安全响应头。

生产环境必须显式设置强 `JWT__SECRET`、公开 URL、CORS Origin、持久化数据库和仓库存储。

## 8. Legacy 与迁移边界

2026-06-23 后，产品主线从 DB-centric Skill / CLI / MCP Registry 转向 Git-centric Platform。

| 能力 | 当前判断 |
|------|----------|
| Git Repo / Smart HTTP / Context | 当前主线，已具备基础能力 |
| Repo UI / Commit / Agent Session / Vibe Coding | Phase E' 在建 |
| Skill / MCP / CLI Indexer | 计划从 Git 仓库派生，EVO-108/109 |
| DB-centric Registry | 兼容期数据和 API，不是新功能主线 |
| Docker Skill Sandbox | 默认关闭，ADR-0005 已决定废弃，EVO-111 删除 |
| Snippet 模型 | legacy 迁移参考，不应作为新产品概念 |

新功能不得以 legacy Sandbox 或旧 Registry 为硬依赖。历史实现可以保留在文档中，但必须标注 `legacy`、`compatibility` 或 `historical`。

## 9. 配置边界

嵌套配置统一使用双下划线：

```bash
DATABASE__DATABASE_TYPE=sqlite
DATABASE__URL=:memory:
JWT__SECRET=replace-in-production
SANDBOX__ENABLED=false
RATE_LIMIT__UNAUTHENTICATED_RPM=30
```

不要使用 `DATABASE_TYPE` / `DATABASE_URL` 代替嵌套键。

前端 API 默认前缀：

```text
VITE_API_URL || /api/v1
```

除非网关明确重写路径，配置中应保留 `/api/v1`。

## 10. 交付形态

### Lite

- Rust 1.88 + Bun 1.3.14。
- SQLite 内存数据库。
- legacy Sandbox 默认关闭。
- 不依赖 Docker。

### Full / Production

- PostgreSQL 16。
- 可选 Redis。
- 持久化 Git 仓库存储。
- 可选 Nginx / Platform LB。
- React 静态资源嵌入后端发布物。

生产部署的水平扩展必须首先解决 Git 仓库文件系统一致性，不能只扩展无共享存储的后端副本。

## 11. 相关文档

- [项目地图](./PROJECT-MAP.md)
- [技术栈说明](./TECH-STACK.md)
- [API 合约](./API-CONTRACT.md)
- [配置参考](./CONFIG.md)
- [权限](./PERMISSIONS.md)
- [多租户设计](./MULTI-TENANT.md)
- [测试](./TESTING.md)
- [ADR-0004 Git-centric storage](../decisions/ADR-0004-git-centric-storage.md)
- [ADR-0005 废弃 Sandbox Runtime](../decisions/ADR-0005-deprecate-sandbox-runtime.md)
- [ADR-0006 Smart HTTP via Git subprocess](../decisions/ADR-0006-smart-http-via-git-subprocess.md)
- [实施路线图](../roadmap/IMPLEMENTATION-ROADMAP.md)
