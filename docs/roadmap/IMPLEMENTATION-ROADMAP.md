# Evolith 实施路线图

> 制定日期：2026-05-15
> 最近更新：2026-05-16
> 目标：维护阶段优先级、实施顺序和 Backlog / Proposals 归口关系。

本文档不是任务池。Agent 不应直接从本文档开工：

- 可执行故事归口到 [Product Backlog](../backlog/PRODUCT-BACKLOG.md)。
- 未成熟方向归口到 [Proposals](../proposals/README.md)。
- 重大取舍归口到 [ADR](../decisions/README.md)。
- 本文档只维护阶段排序、依赖关系和路线图判断。

## 1. 当前判断

Evolith 的产品定位是企业级 AI Agent Harness 平台：为企业内部智能体提供可治理、可审计、可复用、可集成的工具、技能、CLI 友好接口和运行支撑能力。后续路线优先服务这个定位，不再按“代码片段仓库”或纯演示控制台扩展。

Evolith 的后端主体架构已经成型：数据库 repository、双数据库 migration、认证、RBAC、CSRF、基础 CRUD、Docker sandbox、部署栈和 CI/CD 都已具备。

下一阶段不应继续扩展大而散的新功能，而应先处理三类问题：

1. **需求闭环缺口**：需求和 API 合约中仍存在 501、stub、前端 mock 和服务 crate placeholder。
2. **前端工程简化**：当前前端是 Next.js，但企业级 harness 控制台不需要 SSR；迁移为 `React + Vite + Bun` 静态 SPA 是 P0 工程门禁，完成后再推进前端嵌入后端发布物。
3. **产品概念迁移**：旧 snippet 主线停止扩展，后续替换为面向大模型和 CLI 调用的 CLI 友好接口，见 [ADR-0002](../decisions/ADR-0002-cli-friendly-interface-replaces-snippet.md)。

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

## 3. 差距盘点

### 3.1 API 合约中明确 501 的接口

| 模块 | 接口 | 当前状态 | 归口 |
|------|------|----------|------|
| Auth | `POST /api/v1/auth/send-verify` | 501 | EVO-018 |
| Auth | `POST /api/v1/auth/verify-email` | 501 | EVO-018 |
| Auth | `POST /api/v1/auth/forgot-password` | 501 | EVO-003 |
| Auth | `POST /api/v1/auth/reset-password` | 501 | EVO-003 |
| Skills | `PUT /api/v1/skills/{id}` | 501 | EVO-006 |
| Snippets | `PUT /api/v1/snippets/{id}` | 501 | Deferred by EVO-017 |
| Members | `POST /api/v1/tenant/{tenant_id}/members/join` | 501 | EVO-004 |
| Audit | `GET /api/v1/tenant/{tenant_id}/audit-logs/{log_id}` | 501 | EVO-013 |

### 3.2 当前代码中的关键 placeholder

| 区域 | 文件/能力 | 影响 | 归口 |
|------|-----------|------|------|
| MCP 工具执行 | `service-tool/src/executor.rs`、`api/src/handlers/mcp_handlers.rs` | `tools/call` 目前返回 stub 文本，不是真执行 | EVO-005 |
| Skill registry | `service-skill/src/registry.rs` | service crate 未形成可复用注册能力 | EVO-019 |
| CLI interface parser/reference | `service-snippet/src/parser.rs`、`reference.rs` | 旧 snippet parser/reference 需要迁移为 CLI 友好接口格式 | EVO-017 / EVO-009 |
| Storage | `infra/src/storage.rs` | 对象存储未实现，影响技能包/附件 | EVO-020 |
| Auth session/RBAC service | `service-auth/src/session.rs`、`rbac.rs` | 当前主要由 API middleware 承担 | 后续拆分 |
| Frontend tenant pages | members/api-keys/settings | 仍有 mock/TODO | EVO-010 / EVO-011 / EVO-012 |

### 3.3 前后端接口不一致

| 问题 | 说明 | 状态 |
|------|------|------|
| 前端 `PATCH /tools/{id}`，后端是 `PUT /tools/{id}` | 更新工具会失败 | EVO-001 done |
| 前端存在 `/tools/{id}/execute`，后端无对应 REST route | 工具执行入口不一致 | EVO-001 client guard；真实执行归 EVO-005 |
| 前端 `PATCH /skills/{id}`，后端是 `PUT /skills/{id}` 且 501 | 更新技能不可用 | EVO-006 |
| 前端请求 skill/snippet categories/versions/languages，后端未提供 | 页面能力和 API 不一致；snippet 相关调用进入 EVO-017 迁移 | EVO-001 guard / EVO-017 |
| 前端 auth `changePassword` 发送 `current_password`，后端 DTO 是 `old_password` | 修改密码会失败 | EVO-001 done |
| 前端 snippet 使用 `title/category`，后端 DTO 更接近 `name/language/framework/content/code` | 旧 snippet 字段对齐不再作为独立目标，进入 CLI interface 迁移设计 | Deferred by EVO-017 |

## 4. 实施阶段

### Phase A — 现状校准与 API 对齐（Done）

目标：先让已有页面和后端接口对齐，减少“看起来有功能，实际不可用”的问题。

归口：EVO-001。

状态：Done。验证记录见 [Iteration 001](../iterations/ITERATION-001.md)。

### Phase B — 前端迁移到 React + Vite + Bun（P0 / 高优先级）

目标：去掉 Next.js SSR/runtime，保留 React 生态，改为静态 SPA。该阶段是企业级 harness 平台交付形态的高优先级基础工作。

归口：EVO-002。

实施前要求：EVO-002 需要拆成多个 0.5-2 天故事，例如构建工具迁移、路由迁移、运行时配置、Docker/Nginx 调整、CI 调整。EVO-017 完成概念迁移后，应优先进入 EVO-002，不再被普通 P0 业务功能后置。

### Phase C — 认证闭环（P0/P1）

目标：完成用户生命周期关键流程。

归口：EVO-003、EVO-004、EVO-018。

阶段完成标准：注册、登录、修改密码、忘记密码、重置密码、邮箱验证、邀请加入全流程可跑通。

### Phase D — MCP 工具执行闭环（P0/P1）

目标：让 MCP `tools/call` 真正执行工具，而不是返回 stub。

归口：EVO-005。

阶段完成标准：MCP `initialize`、`tools/list`、`tools/call` 可由 API key 调用；私有工具不能被越权调用；HTTP tool 返回值能被 MCP content 包装。

### Phase E — Skill / CLI 友好接口完整性（P1）

目标：补齐 Skill 编辑能力，并将旧 snippet 主线迁移为 CLI 友好接口。

归口：EVO-006、EVO-009、EVO-017、EVO-019、EVO-020。

阶段完成标准：Skill CRUD 完整；CLI interface 描述可直接给 LLM 使用，也可被 Rust CLI 复用；格式 parser 对示例文档有测试覆盖。

### Phase F — 租户管理、计费和审计增强（P1/P2）

目标：补齐管理后台实用能力。

归口：EVO-010 至 EVO-014。

阶段完成标准：tenant settings / members / api keys 页面不再使用 mock 数据；计费和配额至少对创建工具、技能、CLI interface 有基础限制。

## 5. 推荐执行顺序

近期建议按以下顺序执行：

1. **EVO-002：React + Vite + Bun 迁移拆分并实施**，去掉 Next.js runtime；这是当前最高优先级工程门禁。
2. **EVO-003 / EVO-004 / EVO-018：认证与邀请闭环**，补齐 SaaS 用户生命周期。
3. **EVO-005：MCP 工具执行闭环**，让核心价值真正可用。
4. **EVO-006 / EVO-009 / EVO-019 / EVO-020：Skill 与 CLI interface 完整性**。
5. **EVO-010 至 EVO-014：租户管理、计费和审计增强**。

## 6. 暂缓事项

| 事项 | 暂缓原因 |
|------|----------|
| SolidJS 重写 | 成本高，当前主要问题是 Next.js 过重，不是 React 不适合 |
| 制品仓库 | 需求文档标为远期规划，当前主线未闭环 |
| MySQL repository | 当前生产目标是 PostgreSQL，MySQL 仅配置层预留 |
| 完整语义搜索 | CLI interface 基础格式先满足 P0，语义搜索可后置 |
| 租户子域名识别 | 当前 cookie/API key + tenant path 已能支撑控制台，子域名可后置 |

## 7. 远期目标

| 目标 | 文档 | 进入条件 |
|------|------|----------|
| Rust CLI | [Evolith Rust CLI](../proposals/RUST-CLI.md) | API 合约稳定，Skill/CLI interface parser 完成 |
| 前端嵌入后端发布物 | [前端静态产物嵌入后端](../proposals/EMBEDDED-FRONTEND.md) | 完成 React + Vite + Bun 静态 SPA 迁移 |

## 8. 计划维护规则

- 每完成一个 Phase，在本文档中补充完成日期和验证结果。
- 发现新的现状差距，先补到 “需求差距盘点”，再决定 Phase。
- 如果实现过程中踩坑，写入 `EVOLUTION.md`。

## 9. 与 Backlog / Proposals 的分工

本文档只保留阶段方向、差距盘点和执行顺序。具体工作项按以下规则归口：

| 内容类型 | 归口 | 说明 |
|----------|------|------|
| 能在 0.5-2 天内完成且有验收标准 | [Product Backlog](../backlog/PRODUCT-BACKLOG.md) | Agent 可以直接排期实现 |
| 方向明确但范围过大或依赖未成熟 | [Proposals](../proposals/README.md) | 不能直接开工，先沉淀方案 |
| 多个 backlog/proposal 的阶段顺序 | 本文档 | 只描述排序和取舍，不替代 backlog |
| 重大产品/技术取舍 | [ADR](../decisions/README.md) | 例如 ADR-0001、ADR-0002 |

当前迁移状态：

| 路线图内容 | 当前归口 | 状态 |
|----------------------|----------|------|
| Phase A API 对齐 | EVO-001 | Done |
| Phase B React + Vite + Bun | EVO-002 | Ready；P0 高优先级；实施前应按 DoR 拆成更小故事 |
| Phase C forgot/reset password | EVO-003 | Ready |
| Phase C invitation join | EVO-004 | Ready |
| Phase C send/verify email | EVO-018 | Proposed |
| Phase D MCP 工具执行闭环 | EVO-005 | Ready |
| Phase E Skill update | EVO-006 | Proposed |
| Phase E CLI interface 迁移 | EVO-017 / EVO-009 | EVO-017 Done；EVO-009 Proposed |
| Phase E Skill registry | EVO-019 | Proposed |
| Phase E Storage | EVO-020 | Proposed |
| Phase F tenant members/api keys/settings/audit/billing | EVO-010 至 EVO-014 | Proposed |
| Rust CLI | Proposal: [RUST-CLI](../proposals/RUST-CLI.md) | Deferred |
| 前端嵌入后端发布物 | Proposal: [EMBEDDED-FRONTEND](../proposals/EMBEDDED-FRONTEND.md) | Deferred |
| AI Gateway / Agent Runtime | Proposals | 远期想法，不进当前实施路线 |
