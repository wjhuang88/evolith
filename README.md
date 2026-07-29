# Evolith — AI-Native Git Platform

> AI 原生 Git 平台 | Where AI Ships Code

Evolith 是面向 AI Agent 协作的软件开发平台，以 Git 仓库作为代码、策略和能力描述的事实源，将 Git 托管、Agent 编程协作以及 Skill / MCP / CLI 能力发现整合在一起。

项目当前处于开发中 Alpha：Git 服务底座已经支持真实 `clone` / `pull` / `push` 和仓库上下文读取；仓库管理 UI、Agent Session、Commit / Promote 工作流与能力索引仍按 Phase E' 路线推进。

## 核心方向

- **Git 作为基础载体**：标准 Git 协议接入，仓库内容是代码和 Agent 能力的事实源。
- **Agent-ready 协作**：围绕受限凭证、仓库策略、提交、审查、提升和审计构建协作链路。
- **Vibe Coding**：Agent 基于仓库上下文直接编码和提交。
- **能力自动发现**：计划从仓库索引 Skill、MCP Tool 和 CLI Interface。
- **企业治理**：RBAC、多租户、审计、CSRF、资源边界和 API Key Scope。

## 当前能力边界

### 已落地

- Repository CRUD、Git Smart HTTP 和真实 Git 客户端 `clone` / `pull` / `push`。
- Repo Context API：文件树、Blob、Commit 和 Diff。
- `.evolith/policy.yaml` 格式与安全默认语义。
- Cookie / JWT、API Key、RBAC、CSRF、审计和限流。
- SQLite 开发模式与 PostgreSQL 生产路径。
- React + Vite 静态 SPA 嵌入 Rust 后端发布物。

### 正在推进

- 仓库管理 UI。
- Commit / Promote API、Agent Session 与 Scoped Token。
- Webhook、Vibe Coding 工作区以及 Skill / MCP / CLI Indexer。

### Legacy 兼容

早期 DB-centric Registry 和 Docker Sandbox 代码仍部分保留，但不再是新功能主线。`SANDBOX__ENABLED` 默认关闭，相关执行层计划由 EVO-111 删除。

## 运行架构

Evolith 当前是一个**模块化单体**，不是由多个独立部署服务组成的微服务系统：

```text
Browser / Git Client / MCP Client / Future Agent SDK
                         |
                 Optional Gateway
                         |
          Evolith Rust Application (Actix-web)
          ├── HTTP API / Auth / RBAC / Audit
          ├── Git Smart HTTP / Repo Context
          ├── Compatibility modules
          └── Embedded React SPA
                         |
       SQLite (dev) / PostgreSQL (prod) / Redis
                         |
              Git repositories on filesystem
```

后端通过 Rust Workspace 拆分内部 crate，但默认构建和部署为一个 `evolith` 进程。Nginx 或平台负载均衡只作为可选网关、SSL 终止和反向代理层。

## 技术栈

| 层级 | 当前选型 |
|------|----------|
| 后端 | Rust 1.88 + Actix-web + Tokio |
| Git | Git Smart HTTP subprocess + `gix` 上下文读取 |
| 数据库 | SQLite（开发）/ PostgreSQL 16（生产） |
| 缓存 | Redis 7，可按配置启用 |
| 前端 | React 19 + Vite 8 + TypeScript 6 + Tailwind CSS 4 |
| 工具链 | Bun 1.3.14 |
| 发布 | 前端嵌入 Rust 后端；Docker / Nginx 可选 |

依赖版本的权威来源是 `backend/Cargo.toml`、`backend/Cargo.lock`、`frontend/package.json` 和 `frontend/bun.lock`。

## 文档入口

- [文档地图](./docs/README.md)
- [项目地图](./docs/reference/PROJECT-MAP.md)
- [架构设计](./docs/reference/ARCHITECTURE.md)
- [技术栈说明](./docs/reference/TECH-STACK.md)
- [本地开发 SOP](./docs/sop/LOCAL-DEV.md)
- [API 合约](./docs/reference/API-CONTRACT.md)
- [实施路线图](./docs/roadmap/IMPLEMENTATION-ROADMAP.md)
- [经验与已知陷阱](./EVOLUTION.md)

## 环境要求

### Lite 模式

| 依赖 | 最低要求 | 说明 |
|------|----------|------|
| Rust | 1.88 | 与 Workspace `rust-version` 和 Builder 镜像一致 |
| Bun | 1.3.14 | 前端安装、构建和开发脚本 |
| Git | 2.x | Git 服务验证 |
| curl | 稳定版本 | 健康检查 |

Lite 模式不要求 Docker、PostgreSQL 或 Redis，默认使用 SQLite 内存数据库并关闭 legacy Sandbox。

### Full / Production 模式

- Docker 24+ 与 Docker Compose 2.20+ 推荐。
- PostgreSQL 16 是生产主数据库。
- Redis 7 用于缓存相关能力。
- Nginx / Platform LB 为可选网关。
- MinIO 仍在 Compose 中保留，但不是 Git 仓库主存储路径。

## 快速开始

```bash
git clone https://github.com/wjhuang88/evolith.git
cd evolith

# 无需 Docker
./scripts/dev.sh lite

# 启动完整基础设施
./scripts/dev.sh start

./scripts/dev.sh status
./scripts/dev.sh stop
```

常用验证：

```bash
cd backend
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace

cd ../frontend
bun install --frozen-lockfile
bun run type-check
bun run build
```

## 许可证

MIT License
