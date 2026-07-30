# 项目地图

本文件记录 Evolith 当前代码和运行边界，作为 Agent 开始任务时的稳定参考。

## 顶层结构

| 路径 | 说明 |
|------|------|
| `backend/` | Rust Workspace；内部模块默认构建为单个 `evolith` 进程 |
| `frontend/` | React + Vite + Bun 静态 SPA，TypeScript + Tailwind |
| `backend/migrations/sqlite/` | SQLite 开发迁移 |
| `backend/migrations/postgres/` | PostgreSQL 生产迁移 |
| `backend/sandbox/` | legacy Python / Node.js Sandbox 镜像，待 EVO-111 删除 |
| `deploy/` | 可选 Nginx 和 Kubernetes 部署配置 |
| `scripts/` | 本地开发、备份和部署辅助脚本 |
| `docs/` | 产品、架构、API、测试和工程流程文档 |

## 后端关键入口

| 文件 | 职责 |
|------|------|
| `backend/src/main.rs` | 服务启动、配置、数据库分支、middleware 和路由装配 |
| `backend/crates/api/src/state.rs` | `AppState` 共享依赖容器 |
| `backend/crates/api/src/routes/mod.rs` | API 路由总入口 |
| `backend/crates/api/src/middleware/rbac.rs` | JWT / Cookie / API Key 提取和 RBAC |
| `backend/crates/api/src/middleware/csrf.rs` | double-submit Cookie CSRF 防护 |
| `backend/crates/domain/src/repository.rs` | Repository trait 边界 |
| `backend/crates/infra/src/db/` | SQLite / PostgreSQL Repository 实现 |
| `backend/crates/infra/src/config.rs` | 环境变量和默认配置 |
| `backend/crates/service-git/src/lib.rs` | Git 仓库路径、初始化、Smart HTTP subprocess、Repo Context 与资源上限 |
| `backend/crates/service-skill/src/executor.rs` | legacy Skill 执行器和 Docker Sandbox，ADR-0005 / EVO-111 待删除 |

## Git 平台边界

当前 Git 能力分为两条实现路径：

1. Git Smart HTTP 使用系统 `git` subprocess 处理标准协议流。
2. Repo Context 使用 `gix` 读取文件树、Blob、Commit 和 Diff。

Git 仓库位于服务端文件系统。业务元数据位于 SQLite / PostgreSQL。多实例部署需要显式解决仓库存储共享、一致性或请求路由，不能只共享数据库。

## 前端关键入口

| 文件 | 职责 |
|------|------|
| `frontend/src/lib/api/client.ts` | Axios Client、CSRF、Token Refresh、错误解析 |
| `frontend/src/stores/authStore.ts` | 登录状态和用户信息 |
| `frontend/src/components/AuthInitializer.tsx` | 启动时恢复认证状态 |
| `frontend/src/components/AuthGuard.tsx` | 客户端路由保护 |
| `frontend/src/main-spa.tsx` | Vite SPA 入口和 React Router 路由树 |
| `frontend/src/app/providers.tsx` | React Query、Theme、Toast、Auth 初始化 |
| `frontend/src/locales/` | `zh-CN` / `en` 国际化资源 |

## 配置边界

后端配置使用 `config` crate 的双下划线环境变量映射，例如：

- `DATABASE__DATABASE_TYPE`
- `DATABASE__URL`
- `JWT__SECRET`
- `SANDBOX__ENABLED`
- `RATE_LIMIT__UNAUTHENTICATED_RPM`

不要使用单下划线形式替代嵌套配置。Compose、Kubernetes、CI 和 `.env` 应保持同一命名方式。

前端 API 默认值在 `frontend/src/lib/config.ts`：

```text
VITE_API_URL || /api/v1
```

容器和生产环境也应包含 `/api/v1` 前缀，除非网关明确做了路径重写。

## 双数据库开发规则

涉及数据库结构或 Repository 行为时，必须同时考虑：

1. `backend/migrations/sqlite/`
2. `backend/migrations/postgres/`
3. `Sqlite*Repository`
4. `Pg*Repository`
5. Repository 集成测试

SQLite 适合 Lite 开发，PostgreSQL 是生产主路径。MySQL Repository 未实现，不是当前支持的运行路径。

## 版本事实源

- Rust MSRV：`backend/Cargo.toml`，当前为 1.88。
- Rust 解析依赖：`backend/Cargo.lock`。
- 前端直接依赖：`frontend/package.json`。
- 前端解析依赖：`frontend/bun.lock`。
- 不从历史 Iteration、Phase 清单或旧 Docker 文档推断当前版本。
