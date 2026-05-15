# Evolith 开发计划

> 制定日期：2026-05-15
> 目标：基于现有需求文档和当前实现状态，明确下一阶段开发顺序。

## 1. 当前判断

Evolith 的后端主体架构已经成型：数据库 repository、双数据库 migration、认证、RBAC、CSRF、基础 CRUD、Docker sandbox、部署栈和 CI/CD 都已具备。

下一阶段不应继续扩展大而散的新功能，而应先处理两类问题：

1. **需求闭环缺口**：需求和 API 合约中仍存在 501、stub、前端 mock 和服务 crate placeholder。
2. **前端工程简化**：当前前端是 Next.js，但项目定位是 SaaS 控制台，不需要 SSR；建议迁移为 `React + Vite + Bun` 的静态 SPA。

具体需求池维护在 [Product Backlog](../backlog/PRODUCT-BACKLOG.md)。本文档只保留阶段方向和优先级判断。

## 2. 技术路线

### 2.1 前端路线

采用：

```text
React + Vite + TypeScript + Tailwind + React Router + Zustand + TanStack Query + Bun
```

保留：

- React 组件模型
- TypeScript
- Tailwind
- Zustand
- TanStack Query
- Axios API client
- i18n 资源

移除：

- Next.js App Router
- Next middleware
- Next standalone runtime
- `next.config.js`
- `.next` 构建产物依赖

暂不采用 SolidJS。SolidJS 可以作为后续独立实验方向，但不和本次 Next.js 去除绑定。

### 2.2 部署路线

前端构建为纯静态文件，由 Nginx 托管：

```text
bun install
bun run build
nginx serve dist/
```

生产建议支持运行时配置：

```text
/config.js -> window.__EVOLITH_CONFIG__.apiBaseUrl
```

避免每个环境都重新 build 前端镜像。

## 3. 需求差距盘点

### 3.1 API 合约中明确 501 的接口

| 模块 | 接口 | 当前状态 | 建议优先级 |
|------|------|----------|------------|
| Auth | `POST /api/v1/auth/send-verify` | 501 | P1 |
| Auth | `POST /api/v1/auth/verify-email` | 501 | P1 |
| Auth | `POST /api/v1/auth/forgot-password` | 501 | P0 |
| Auth | `POST /api/v1/auth/reset-password` | 501 | P0 |
| Skills | `PUT /api/v1/skills/{id}` | 501 | P1 |
| Snippets | `PUT /api/v1/snippets/{id}` | 501 | P1 |
| Members | `POST /api/v1/tenant/{tenant_id}/members/join` | 501 | P0 |
| Audit | `GET /api/v1/tenant/{tenant_id}/audit-logs/{log_id}` | 501 | P2 |

### 3.2 当前代码中的关键 placeholder

| 区域 | 文件/能力 | 影响 |
|------|-----------|------|
| MCP 工具执行 | `service-tool/src/executor.rs`、`api/src/handlers/mcp_handlers.rs` | `tools/call` 目前返回 stub 文本，不是真执行 |
| Skill registry | `service-skill/src/registry.rs` | service crate 未形成可复用注册能力 |
| Snippet parser/reference | `service-snippet/src/parser.rs`、`reference.rs` | 片段格式解析和 LLM 引用仍偏弱 |
| Storage | `infra/src/storage.rs` | 对象存储未实现，影响技能包/附件 |
| Auth session/RBAC service | `service-auth/src/session.rs`、`rbac.rs` | 当前主要由 API middleware 承担 |
| Frontend tenant pages | members/api-keys/settings | 仍有 mock/TODO |

### 3.3 前后端接口不一致

| 问题 | 说明 | 优先级 |
|------|------|--------|
| 前端 `PATCH /tools/{id}`，后端是 `PUT /tools/{id}` | 更新工具会失败 | P0 |
| 前端存在 `/tools/{id}/execute`，后端无对应 REST route | 工具执行入口不一致 | P0 |
| 前端 `PATCH /skills/{id}`，后端是 `PUT /skills/{id}` 且 501 | 更新技能不可用 | P1 |
| 前端请求 skill/snippet categories/versions/languages，后端未提供 | 页面能力和 API 不一致 | P2 |
| 前端 auth `changePassword` 发送 `current_password`，后端 DTO 是 `old_password` | 修改密码会失败 | P0 |
| 前端 snippet 使用 `title/category`，后端 DTO 更接近 `name/language/framework/content/code` | 创建片段字段不一致 | P0 |

## 4. 分阶段计划

### Phase A — 现状校准与 API 对齐（P0）

目标：先让已有页面和后端接口对齐，减少“看起来有功能，实际不可用”的问题。

任务：

1. 修正前端 API client 的 HTTP method 和字段名。
2. 对齐 auth change password 请求字段。
3. 对齐 tool create/update DTO，明确 handler 配置字段。
4. 对齐 snippet create/update DTO。
5. 移除或隐藏前端调用但后端不存在的 categories/versions/languages/execute route。
6. 为这些接口增加最小回归测试或类型检查。

验收：

- `npm run type-check`
- `npm run build`
- `cargo test -p api`
- 手工验证登录、修改密码、工具 CRUD、片段 CRUD。

### Phase B — 前端迁移到 React + Vite + Bun（P0）

目标：去掉 Next.js SSR/runtime，保留 React 生态，改为静态 SPA。

任务：

1. 新建 Vite 配置：`vite.config.ts`、`index.html`、`src/main.tsx`、`src/App.tsx`。
2. 引入 `react-router-dom`，把 `src/app/**/page.tsx` 迁移为 route config。
3. 删除 `frontend/src/middleware.ts`，由 `AuthGuard` 做 SPA route guard。
4. 将 `next/navigation` 替换为 React Router。
5. 将 `NEXT_PUBLIC_API_URL` 改为 `VITE_API_URL`，并预留运行时配置。
6. 替换 npm scripts：`dev`、`build`、`preview`、`type-check` 使用 Vite/Bun。
7. 更新 Dockerfile 为 Bun build + Nginx static。
8. 更新 Nginx SPA fallback：`try_files $uri /index.html;`。
9. 更新 CI/CD 和部署文档。

验收：

- `bun install`
- `bun run type-check`
- `bun run build`
- 本地 Nginx 或 `vite preview` 验证刷新深层路由不 404。
- 验证登录、退出、受保护路由跳转、CSRF 状态变更请求。

### Phase C — 认证闭环（P0/P1）

目标：完成用户生命周期关键流程。

任务：

1. 实现 forgot password。
2. 实现 reset password。
3. 实现 invite accept/join。
4. 接入 mailer 发送重置/邀请邮件；开发环境使用 ConsoleMailer。
5. 实现前端 forgot/reset/join 页面真实 API 调用。
6. 补充 auth E2E 测试。

验收：

- 注册、登录、修改密码、忘记密码、重置密码、邀请加入全流程可跑通。
- 邮件发送在 SMTP disabled 时可在日志中看到 ConsoleMailer 输出。

### Phase D — MCP 工具执行闭环（P0/P1）

目标：让 MCP `tools/call` 真正执行工具，而不是返回 stub。

任务：

1. 定义 Tool handler 执行模型：HTTP handler 优先，Function handler 后置。
2. 实现 JSON Schema 参数校验后的 HTTP 调用。
3. 实现超时、错误映射、输出 schema 校验。
4. 记录工具调用审计日志。
5. 增加 API key 租户隔离测试。

验收：

- MCP `initialize`、`tools/list`、`tools/call` 可由 API key 调用。
- 私有工具不能被无 API key 调用。
- HTTP tool 返回值能被 MCP content 包装。

### Phase E — Skill / Snippet 完整性（P1）

目标：补齐创建后的编辑、引用和格式解析能力。

任务：

1. 实现 `PUT /skills/{id}`。
2. 实现 `PUT /snippets/{id}`。
3. 实现 snippet reference format：direct / inline / with_deps。
4. 实现 SKILL.md 和 snippet frontmatter parser。
5. 明确技能包上传和 storage 的范围，必要时拆到下一阶段。

验收：

- Skill/Snippet CRUD 完整。
- 引用输出可直接给 LLM 使用。
- 格式 parser 对示例文档有测试覆盖。

### Phase F — 租户管理、计费和审计增强（P1/P2）

目标：补齐管理后台实用能力。

任务：

1. 租户设置保存。
2. API key 页面接真实接口，支持 revoke。
3. Members 页面接真实列表、邀请、移除。
4. Audit log detail 接口。
5. Stripe webhook 路由恢复和测试。
6. 用量统计和配额限制从展示走向 enforcement。

验收：

- tenant settings / members / api keys 页面不再使用 mock 数据。
- 计费和配额至少对创建工具、技能、片段有基础限制。

## 5. 推荐执行顺序

近期建议按以下顺序执行：

1. **Phase A：API 对齐**，先修当前不可用的前后端连接。
2. **Phase B：React + Vite + Bun 迁移**，在接口对齐后做前端工程切换。
3. **Phase C：认证闭环**，补齐 SaaS 必需流程。
4. **Phase D：MCP 工具执行闭环**，让核心价值真正可用。
5. **Phase E/F**，补齐编辑、引用、管理和计费增强。

## 6. 暂缓事项

| 事项 | 暂缓原因 |
|------|----------|
| SolidJS 重写 | 成本高，当前主要问题是 Next.js 过重，不是 React 不适合 |
| 制品仓库 | 需求文档标为远期规划，当前主线未闭环 |
| MySQL repository | 当前生产目标是 PostgreSQL，MySQL 仅配置层预留 |
| 完整语义搜索 | snippet 关键词搜索先满足 P0，语义搜索可后置 |
| 租户子域名识别 | 当前 cookie/API key + tenant path 已能支撑控制台，子域名可后置 |

## 7. 远期目标

| 目标 | 文档 | 进入条件 |
|------|------|----------|
| Rust CLI | [Evolith Rust CLI](../planned/RUST-CLI.md) | API 合约稳定，Skill/Snippet parser 完成 |
| 前端嵌入后端发布物 | [前端静态产物嵌入后端](../planned/EMBEDDED-FRONTEND.md) | 完成 React + Vite + Bun 静态 SPA 迁移 |

## 8. 计划维护规则

- 每完成一个 Phase，在本文档中补充完成日期和验证结果。
- 发现新的现状差距，先补到 “需求差距盘点”，再决定 Phase。
- 如果实现过程中踩坑，写入 `EVOLUTION.md`。
