# 项目地图

本文件记录 Evolith 当前代码和运行边界，作为 agent 开始任务时的稳定参考。

## 顶层结构

| 路径 | 说明 |
|------|------|
| `backend/` | Rust workspace，包含 API、domain、infra 和各 service crate |
| `frontend/` | Next.js 14 前端，App Router + TypeScript + Tailwind |
| `backend/migrations/sqlite/` | SQLite 开发迁移 |
| `backend/migrations/postgres/` | PostgreSQL 生产迁移 |
| `backend/sandbox/` | Python / Node.js skill executor 镜像 |
| `deploy/` | Nginx 和 Kubernetes 部署配置 |
| `scripts/` | 本地开发、备份和部署辅助脚本 |
| `docs/` | 产品、架构、API、测试和工程流程文档 |

## 后端关键入口

| 文件 | 职责 |
|------|------|
| `backend/src/main.rs` | 服务启动、配置、数据库分支、middleware 和路由装配 |
| `backend/crates/api/src/state.rs` | `AppState` 共享依赖容器 |
| `backend/crates/api/src/routes/mod.rs` | API 路由总入口 |
| `backend/crates/api/src/middleware/rbac.rs` | JWT/cookie/API key 提取和 RBAC |
| `backend/crates/api/src/middleware/csrf.rs` | 双提交 cookie CSRF 防护 |
| `backend/crates/domain/src/repository.rs` | Repository trait 边界 |
| `backend/crates/infra/src/db/` | SQLite/PostgreSQL repository 实现 |
| `backend/crates/infra/src/config.rs` | 环境变量和默认配置 |
| `backend/crates/service-skill/src/executor.rs` | Skill 执行器和 Docker sandbox |

## 前端关键入口

| 文件 | 职责 |
|------|------|
| `frontend/src/lib/api/client.ts` | Axios client、CSRF、token refresh、错误解析 |
| `frontend/src/stores/authStore.ts` | 登录状态和用户信息 |
| `frontend/src/components/AuthInitializer.tsx` | 启动时恢复认证状态 |
| `frontend/src/components/AuthGuard.tsx` | 客户端路由保护 |
| `frontend/src/middleware.ts` | Cookie-based SSR route guard |
| `frontend/src/app/providers.tsx` | React Query、Theme、Toast、Auth 初始化 |
| `frontend/src/locales/` | `zh-CN` / `en` 国际化资源 |

## 配置边界

后端配置使用 `config` crate 的双下划线环境变量映射，例如：

- `DATABASE__DATABASE_TYPE`
- `DATABASE__URL`
- `JWT__SECRET`
- `SANDBOX__ENABLED`
- `RATE_LIMIT__UNAUTHENTICATED_RPM`

不要使用单下划线形式替代嵌套配置。Compose、K8s、CI 和 `.env` 都应保持同一命名方式。

前端 API 默认值在 `frontend/src/lib/api/client.ts`：

```text
NEXT_PUBLIC_API_URL || http://localhost:8080/api/v1
```

容器和生产环境也应包含 `/api/v1` 前缀，除非网关明确做了路径重写。

## 双数据库开发规则

涉及数据库结构或 repository 行为时，必须同时考虑：

1. `backend/migrations/sqlite/`
2. `backend/migrations/postgres/`
3. `Sqlite*Repository`
4. `Pg*Repository`
5. repository 集成测试

SQLite 适合 lite 开发，PostgreSQL 是生产主路径。不要只验证一侧后结束数据库相关任务。
