# 技术栈说明

> 本文档记录 Evolith 当前可验证的技术与版本基线。版本事实以 Manifest 和 Lockfile 为准，历史迭代记录只用于追溯，不作为当前依赖来源。

## 1. 权威来源

| 范围 | 权威文件 |
|------|----------|
| Rust 工具链最低版本 | `backend/Cargo.toml` 的 `workspace.package.rust-version` |
| Rust 依赖解析结果 | `backend/Cargo.lock` |
| Rust 直接依赖声明 | `backend/Cargo.toml` 与各 crate `Cargo.toml` |
| 前端直接依赖声明 | `frontend/package.json` |
| 前端解析结果 | `frontend/bun.lock` |
| 容器 Builder 版本 | `backend/Dockerfile` |
| CI 实际命令 | `.github/workflows/ci.yml` |

版本升级后必须同步本文件、README 和受影响的开发/发布文档。

## 2. 后端基线

### 2.1 工具链与交付

| 技术 | 当前基线 | 用途 |
|------|----------|------|
| Rust | 1.90 | Workspace MSRV；与生产 Builder 镜像一致 |
| Edition | 2021 | Workspace edition |
| Cargo resolver | 2 | Workspace 依赖解析 |
| Debian | bookworm-slim | 生产运行镜像 |

Release Profile：

```toml
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

后端默认产出单个 `evolith` 可执行文件。前端静态资源在构建时嵌入该发布物。

### 2.2 核心运行时

| Crate | 当前声明 | 用途 |
|-------|----------|------|
| `actix-web` | 4.13.0 | HTTP 服务 |
| `tokio` | 1.49.0 | 异步运行时 |
| `futures-util` | 0.3 | 异步流工具 |
| `serde` / `serde_json` | 1.x | 序列化 |
| `thiserror` | 2.0.18 | 类型化错误 |
| `anyhow` | 1.x | 应用级错误上下文 |
| `tracing` | 0.1 | 结构化日志 |
| `tracing-subscriber` | 0.3 | 日志订阅和 JSON 输出 |

### 2.3 Git

| 技术 | 当前声明 | 用途 |
|------|----------|------|
| Git CLI | 系统包 | Smart HTTP 服务 subprocess |
| `gix` | 0.78.0 | Repo Context：revision、tree、blob 和 diff |

实现边界：

- Smart HTTP 使用 Git subprocess 处理标准协议流。
- Repository Context 使用 `gix` 读取对象和历史。
- Git 仓库当前存储在服务端文件系统，不在 PostgreSQL 或 MinIO 中。

### 2.4 数据与基础设施

| 技术 | 当前声明或镜像 | 用途 | 当前状态 |
|------|----------------|------|----------|
| SQLx | 0.9.0 | 异步数据库访问与 migration | SQLite / PostgreSQL 主路径 |
| SQLite | SQLx feature | Lite 开发和测试 | 支持 |
| PostgreSQL | 16-alpine | 生产数据库 | 支持 |
| MySQL | SQLx feature 保留 | 非主路径 | Repository 未实现，启动时拒绝 |
| Redis | crate 1.2.2 / image 7-alpine | 缓存 | 可选 |
| MinIO | Compose image | 早期对象存储基础设施 | 非 Git 主存储路径 |

SQLx 当前启用了 `sqlite`、`postgres` 和 `mysql` feature，但 feature 存在不等于业务支持。生产主服务只支持 SQLite 和 PostgreSQL 分支。

数据库变更必须同时考虑：

1. `backend/migrations/sqlite/`
2. `backend/migrations/postgres/`
3. `Sqlite*Repository`
4. `Pg*Repository`
5. 双路径测试

### 2.5 认证、安全与治理

| Crate | 当前声明 | 用途 |
|-------|----------|------|
| `jsonwebtoken` | 10.4.0 | JWT |
| `argon2` | 0.5 | Argon2id 密码哈希 |
| `validator` | 0.20.0 | DTO / Domain 校验 |
| `jsonschema` | 0.46.5 | Tool Schema 校验 |
| `actix-governor` | 0.10.0 | 请求限流 |
| `hmac` | 0.13.0 | Webhook 签名 |
| `sha2` | 0.11.0 | SHA-2 |
| `regex` | 1.12.3 | 输入与格式校验 |

当前认证面包括：

- 浏览器：httpOnly Cookie JWT + double-submit Cookie CSRF。
- API Client：API Key + scope / RBAC。
- Git Client：Basic challenge 后映射到凭证和仓库授权。

### 2.6 网络、邮件、计费和可观测性

| Crate | 当前声明 | 用途 |
|-------|----------|------|
| `reqwest` | 0.13.4 | HTTP Tool、Webhook 和外部请求 |
| `lettre` | 0.11.22 | SMTP 邮件 |
| `async-stripe` | 1.0.0-rc.6 | Stripe 集成 |
| `actix-web-prom` | 0.10.0 | Actix 指标 |
| `prometheus` | 0.14.0 | Prometheus 指标 |
| `redis` | 1.2.2 | 异步缓存连接 |

### 2.7 Embedded Frontend

| Crate | 当前声明 | 用途 |
|-------|----------|------|
| `rust-embed-for-web` | 11.3 | 嵌入 Vite 静态产物 |
| `actix-web-rust-embed-responder` | 2.4.0 | Actix 静态资源响应与协商 |

构建顺序：

```text
bun install --frozen-lockfile
bun run build
cargo build --release
```

后端编译依赖 `frontend/dist/` 已存在。

### 2.8 Legacy Sandbox

| 技术 | 当前声明 | 状态 |
|------|----------|------|
| `bollard` | 0.21.0 | legacy Docker Sandbox 客户端 |
| Python image | 3.11 | legacy |
| Node image | 20 | legacy |

`SANDBOX__ENABLED` 默认 `false`。ADR-0005 已决定废弃服务端 Sandbox，EVO-111 负责删除执行层、`bollard` 和镜像。新 Git-centric 功能不得新增对 Sandbox 的依赖。

## 3. 前端基线

### 3.1 工具链

| 技术 | 当前声明 | 用途 |
|------|----------|------|
| Bun | 1.3.14 | 包管理、锁文件和脚本运行 |
| Vite | 8.0.16 | 开发服务器和静态 SPA 构建 |
| TypeScript | 6.0.3 | 类型检查 |
| ESLint | 10.4.1 | 静态检查 |
| Prettier | 3.8.3 | 格式化 |
| `@vitejs/plugin-react` | 6.0.2 | React 构建插件 |

使用 Bun 时，Node.js 不是 Lite 模式的独立必需依赖。不要同时维护 npm/yarn/pnpm 锁文件。

### 3.2 UI 与应用框架

| Package | 当前声明 | 用途 |
|---------|----------|------|
| React | 19.2.7 | UI |
| React DOM | 19.2.7 | 浏览器渲染 |
| React Router DOM | 7.17.0 | SPA 路由 |
| Tailwind CSS | 4.3.0 | 样式系统 |
| Radix UI | 多包 | 无样式可访问组件 |
| Lucide React | 1.17.0 | 图标 |
| `class-variance-authority` | 0.7.1 | 组件 Variant |
| `tailwind-merge` | 3.6.0 | Tailwind 类合并 |

旧文档中的 React 18、TypeScript 5、Tailwind 3 和 ESLint 8 已不是当前版本。

### 3.3 状态、数据和国际化

| Package | 当前声明 | 用途 |
|---------|----------|------|
| TanStack Query | 5.101.0 | 服务端状态和缓存 |
| Zustand | 5.0.14 | 客户端状态 |
| Axios | 1.17.0 | API Client |
| i18next | 26.3.1 | 国际化核心 |
| react-i18next | 17.0.8 | React 集成 |
| i18next-browser-languagedetector | 8.2.1 | 语言检测 |

### 3.4 当前脚本

```json
{
  "dev": "vite",
  "build": "vite build",
  "preview": "vite preview",
  "lint": "eslint src/",
  "format": "prettier --write .",
  "type-check": "tsc --noEmit"
}
```

`frontend/package.json` 当前没有 Vitest 或 Playwright 直接依赖，因此不能把它们描述为已建立的前端测试基线。需要引入时应先进入 Backlog 并更新 Manifest。

## 4. 部署基线

### 4.1 Lite

- Rust 1.90。
- Bun 1.3.14。
- SQLite 内存数据库。
- legacy Sandbox 关闭。
- 不要求 Docker。

### 4.2 Full / Production

- Rust 后端单进程。
- 嵌入式 React SPA。
- PostgreSQL 16。
- Redis 可选。
- Git 仓库持久卷。
- Nginx / Platform LB 可选。

MinIO 仍可由开发 Compose 启动，但当前 Git-centric 主线使用 Git 文件系统，不应把 MinIO 视为仓库数据层。

### 4.3 容器

`backend/Dockerfile` 当前：

- Builder：`rust:1.90-slim-bookworm`
- Runtime：`debian:bookworm-slim`
- Runtime 包含 `git`、`curl`、CA 和 OpenSSL 运行库
- 非 root 用户 `evolith`
- Health Check：`/health`

## 5. CI 基线

`.github/workflows/ci.yml` 当前只在 `v*.*.*` Tag push 时运行。

实际门禁：

```text
Frontend:
  bun install --frozen-lockfile
  bun run type-check
  bun run build
  bun run lint

Backend:
  cargo fmt --all -- --check
  cargo check --workspace --all-targets
  cargo clippy --workspace --all-targets -- -D warnings
  cargo test --workspace                  # SQLite
  cargo test --workspace                  # PostgreSQL env
```

CI 启动 PostgreSQL 16 Service。Tag-only 是当前发布驱动策略，不要在文档中描述为每次 Push 或 PR 自动执行。

## 6. 版本升级规则

1. 先修改 Manifest 和 Lockfile，并完成 breaking migration。
2. Rust 依赖升级后检查实际 MSRV，统一更新 Workspace、Dockerfile 和 README。
3. 前端升级后以 `package.json` + `bun.lock` 为事实源。
4. 不把“latest”当作固定版本号写入稳定 Reference。
5. 依赖 feature 存在不等于产品能力完整支持。
6. 完成后执行与风险匹配的 fmt、check、clippy、test、type-check、build 和 lint。

## 7. 相关文档

- [架构设计](./ARCHITECTURE.md)
- [项目地图](./PROJECT-MAP.md)
- [配置参考](./CONFIG.md)
- [测试](./TESTING.md)
- [本地开发](../sop/LOCAL-DEV.md)
- [发布](../sop/RELEASE.md)
- [全量依赖 latest 迁移记录](../backlog/archive/2026-Q2/EVO-086-全量依赖-latest-迁移.md)
- [ADR-0005 废弃 Sandbox Runtime](../decisions/ADR-0005-deprecate-sandbox-runtime.md)
