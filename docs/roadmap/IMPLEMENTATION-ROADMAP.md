# Evolith 实施路线图

> 制定日期：2026-05-15
> 最近更新：2026-06-23（方向调整：从 Skill/CLI/MCP Registry 转向 Git 托管 + Vibe Coding 平台）
> 目标：维护阶段优先级、实施顺序和 Backlog / Proposals 归口关系。

本文档不是任务池。Agent 不应直接从本文档开工：

- 可执行故事归口到 [Product Backlog](../backlog/PRODUCT-BACKLOG.md)。
- 未成熟方向归口到 [Proposals](../proposals/README.md)。
- 重大取舍归口到 [ADR](../decisions/README.md)。
- 本文档只维护阶段排序、依赖关系和路线图判断。

## 1. 当前判断

**2026-06-23 方向调整**：Evolith 的产品定位从"企业级 AI Agent Harness 平台（DB-centric skill/CLI/MCP registry）"转变为"**Git 托管 + Vibe Coding 平台 + Pages 式 skill/CLI/MCP 索引发现**"。git repo 作为 substrate，skill/CLI/MCP 作为 Pages 式衍生能力（类比 GitHub Pages ↔ GitHub Repo）。详见 [Git-Centric Platform Proposal](../proposals/GIT-CENTRIC-PLATFORM.md) + [ADR-0004](../decisions/ADR-0004-git-centric-storage.md) + [ADR-0005](../decisions/ADR-0005-deprecate-sandbox-runtime.md)。

Evolith 的后端主体架构已经成型：数据库 repository、双数据库 migration、认证、RBAC、CSRF、基础 CRUD、Docker sandbox 和部署栈都已具备；GitHub CI/CD（EVO-030）已在 Iteration 029 落地。

下一阶段的主线不再是"扩展 skill/CLI/MCP 注册能力"，而是"构建 git 托管服务 + 接入外部 agent engine + 让 vibe coding 可用"。旧 Phase E（Skill 生命周期 / CLI 友好接口完整性）整体 Superseded/Dropped，被新的 Phase E'（Git 托管 + Vibe Coding）替代。

原路线图关注的三类问题（需求闭环、前端工程、产品概念迁移）已部分解决：

1. **需求闭环缺口**：EVO-006（Skill update）、EVO-009（parser）等关键缺口已 Done；剩余 stub 由 Phase F（EVO-012~014）单独处理。
2. **前端工程简化**：EVO-002 + EVO-016-B + EVO-026 全部 Done；React + Vite + Bun + rust-embed-for-web 落地。
3. **产品概念迁移**：EVO-017 + EVO-009 完成 snippet → CLI interface 概念迁移；下一步迁移到 git 存储（Phase E'）。

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

当前前端构建为纯静态文件，并在后端发布物中通过 `rust-embed-for-web` 提供：

```text
bun install
bun run build
cargo build --release
```

Nginx 只保留为可选网关/SSL/反代组件，不再是前端资源托管的必需组件。

生产建议支持运行时配置：

```text
/config.js -> window.__EVOLITH_CONFIG__.apiBaseUrl
```

避免每个环境都重新 build 前端镜像。

## 3. 差距盘点

### 3.1 API 合约中明确 501 的接口

| 模块 | 接口 | 当前状态 | 归口 |
|------|------|----------|------|
| Auth | `POST /api/v1/auth/send-verify` | ✅ Implemented | EVO-018 / Iteration 009 |
| Auth | `POST /api/v1/auth/verify-email` | ✅ Implemented | EVO-018 / Iteration 009 |
| Skills | `PUT /api/v1/skills/{id}` | ✅ Implemented | EVO-006 / Iteration 010 |
| Snippets | `PUT /api/v1/snippets/{id}` | 501 | Deferred by EVO-017 |
| Audit | `GET /api/v1/tenant/{tenant_id}/audit-logs/{log_id}` | 501 | EVO-013 |

已在 Iteration 005 / EVO-031 完成：forgot/reset password 与 invitation join 公开入口。

### 3.2 当前代码中的关键 placeholder

| 区域 | 文件/能力 | 影响 | 归口 |
|------|-----------|------|------|
| MCP 工具执行 | `service-tool/src/executor.rs`、`api/src/handlers/mcp_handlers.rs` | HTTP tool 已真实执行；Function executor 仍未支持 | EVO-005 / EVO-032 |
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
| 前端 `PATCH /skills/{id}`，后端是 `PUT /skills/{id}` 且已实现 | 更新技能可用 | EVO-006 Done |
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

实施前要求：EVO-002 需要拆成多个 0.5-2 天故事，例如构建工具迁移、路由迁移、运行时配置、Docker/Nginx 静态托管调整、Next.js 遗留入口移除。GitHub CI/CD 不混入前端迁移链，单独归口到项目后段的 EVO-030。EVO-017 完成概念迁移后，应优先进入 EVO-002，不再被普通 P0 业务功能后置。

### Phase C — 认证闭环（P0/P1）

目标：完成用户生命周期关键流程。

归口：EVO-003、EVO-004、EVO-018。

阶段完成标准：注册、登录、修改密码、忘记密码、重置密码、邮箱验证、邀请加入全流程可跑通。

### Phase D — MCP 工具执行闭环（P0/P1）

目标：让 MCP `tools/call` 真正执行工具，而不是返回 stub。

归口：EVO-005。

阶段完成标准：MCP `initialize`、`tools/list`、`tools/call` 可由 API key 调用；私有工具不能被越权调用；HTTP tool 返回值能被 MCP content 包装。

### Phase E' — Git 托管 + Vibe Coding（P0 / 高优先级 / 2026-06-23 方向调整后主线）

**目标**：将 Evolith 从 DB-centric skill/CLI/MCP registry 重构为通用 git 托管 + vibe coding 平台 + Pages 式 skill/CLI/MCP 索引发现能力。git repo 作为 substrate，skill/CLI/MCP 作为 Pages 式衍生能力（类比 GitHub Pages ↔ GitHub Repo）。

**归口**：[EVO-100](../backlog/active/EVO-100-git-centric-platform-foundation.md)（Epic）+ EVO-101~103、105~111（10 个子 Story）+ [EVO-104](../backlog/active/EVO-104-vibe-coding-web-ui.md)（Vibe Coding Web UI）。

**子阶段**：

- **Phase E'-1 Git Service 基础**：[EVO-101](../backlog/active/EVO-101-git-repos-schema.md)（Done：git_repos 表 + 双轨 migration）+ [EVO-102](../backlog/active/EVO-102-evolith-policy-yaml.md)（Done：policy.yaml 规范）+ [EVO-103](../backlog/active/EVO-103-repo-context-and-smart-http.md)（Next：Repo CRUD + Smart HTTP + Repo Context API）。
- **Phase E'-1.5 仓库管理 UI**：[EVO-112](../backlog/active/EVO-112-repo-management-ui.md)（仓库列表 / 创建 / 详情页 + 导航重构 + Dashboard repo-centric 改版）。依赖 EVO-103 后端 API 就绪。
- **Phase E'-2 Agent 集成 + Vibe Coding 形态**：[EVO-104](../backlog/active/EVO-104-vibe-coding-web-ui.md)（Vibe Coding Web UI）+ [EVO-105](../backlog/active/EVO-105-commit-and-promote-api.md)（Commit API + 直推直合 + Promote API）+ [EVO-106](../backlog/active/EVO-106-agent-session-and-scoped-token.md)（Agent Session API + Scoped Token）+ [EVO-107](../backlog/active/EVO-107-webhook-out.md)（Webhook Out）。
- **Phase E'-3 Indexer + Discovery + 旧表双写**：[EVO-108](../backlog/active/EVO-108-skill-cli-mcp-indexer.md)（Skill/CLI/MCP Indexer）+ [EVO-109](../backlog/active/EVO-109-discovery-api-and-pages-ui.md)（Discovery API + Pages 式发现 UI）+ [EVO-110](../backlog/active/EVO-110-old-table-dual-write.md)（旧表双写适配）。
- **Phase E'-4 Sandbox 废弃收尾**：[EVO-111](../backlog/active/EVO-111-deprecate-sandbox-runtime.md)（按 ADR-0005 删除 service-skill 执行层 + bollard + sandbox 镜像）。

**阶段完成标准**：

- 用户可创建 git repo，`git clone http://.../repos/{id}` 完整可用。
- `.evolith/policy.yaml` 解析生效，三态（auto_merge / require_review / block）行为正确。
- 仓库管理 UI（列表 / 创建 / 详情 / 导航重构）可用，平台以 Git 仓库为中心。
- Vibe Coding conversation-first UI MVP 可用：agent session stream 为主，file tree 抽屉化，文件内容和 diff 作为 contextual artifact 呈现。
- Agent Session 创建 / 审计 / 撤销全链路贯通。
- Push 一个含 `SKILL.md` / `interface.yaml` / `tool.yaml` 的 commit → `*_index` 表在 5s 内更新。
- `GET /skills?q=` 等跨仓搜索 API 可用；旧 `GET /skills/{id}` 等 API 行为不变。
- `cargo test --workspace` 与 `cargo clippy --workspace --all-targets -- -D warnings` 全绿。
- Docker sandbox 整层删除，启动时间显著缩短。

**UX 调研前置门禁**：已解除。[EVO-104](../backlog/active/EVO-104-vibe-coding-web-ui.md) U-01 ~ U-05（编辑器选型、主布局、Chat 流式架构、Agent Branch 心智模型、直推直合三态视觉反馈）已写入 [Vibe Coding UI Design Decisions](../design/vibe-coding-ui-decisions.md)。EVO-104 实现仍需等待 EVO-103 / EVO-105 / EVO-106 / EVO-112 依赖。

**不做**：

- 不实现 PR/MR UI（MVP 仅仓库浏览 + 直推直合）。
- 不实现 SSH / LFS（Phase 5+ 扩展）。
- 不构建 LLM loop（外部 agent engine 提供）。
- 不实现 live preview pane / web 终端 / Yjs 多人协作（明确 out of MVP）。

**关联文档**：

- [Git-Centric Platform Proposal](../proposals/GIT-CENTRIC-PLATFORM.md)
- [ADR-0004 Git-Centric Storage](../decisions/ADR-0004-git-centric-storage.md)
- [ADR-0005 Deprecate Sandbox Runtime](../decisions/ADR-0005-deprecate-sandbox-runtime.md)

### Phase F — 租户管理、计费和审计增强（P1/P2）

目标：补齐管理后台实用能力。

归口：EVO-010 至 EVO-014。

阶段完成标准：tenant settings / members / api keys 页面不再使用 mock 数据；计费和配额至少对创建工具、技能、CLI interface 有基础限制。

### Phase G — GitHub CI/CD 重建（P2 / 项目后段）

目标：在前端迁移、部署形态和核心项目结构稳定后，基于最终命令重建 GitHub Actions，避免在迁移期间维护过时的 Next.js workflow。

归口：EVO-030。

阶段完成标准：CI 覆盖后端 fmt/clippy/test、前端 type-check/build、必要安全扫描；如恢复部署自动化，deploy workflow 使用静态 SPA 生产镜像和最终部署脚本。

## 5. 推荐执行顺序

近期建议按以下顺序执行（2026-06-23 方向调整后重排）：

1. ~~**EVO-002：React + Vite + Bun 迁移拆分并实施**~~ — ✅ Done（Iteration 003-004）
2. ~~**EVO-003 / EVO-004 / EVO-018：认证与邀请闭环**~~ — ✅ Done（Iteration 005 + EVO-031 补漏）
3. ~~**EVO-005：MCP 工具执行闭环**~~ — ✅ Done（Iteration 006 + EVO-032 补漏）
4. ~~**EVO-030：GitHub CI/CD 重建**~~ — ✅ Done（Iteration 029）
5. ~~**EVO-016-B：前端嵌入后端发布物（rust-embed-for-web）**~~ — ✅ Done（Iteration 031）
6. ~~**EVO-086：全量依赖 latest 迁移**~~ — ✅ Done（2026-06-06）
7. ~~**EVO-100 Phase E'-1a：Git Service schema + policy**（EVO-101 + EVO-102）~~ — ✅ Done（Iteration 042）
8. ~~**EVO-113：方向变更评审缺口修复**~~ — ✅ Done（2026-06-25：默认策略、状态同步、断链和 reference/manifest 漂移修复）
9. **EVO-100 Phase E'-1b：Repo CRUD + Smart HTTP**（EVO-103）
10. **EVO-100 Phase E'-1.5：仓库管理 UI**（EVO-112；依赖 EVO-103 后端 API）
11. **EVO-100 Phase E'-2：Agent 集成 + Vibe Coding 形态**（EVO-104 + EVO-105 + EVO-106 + EVO-107；UX gate 已解除）
12. **EVO-100 Phase E'-3：Indexer + Discovery + 旧表双写**（EVO-108 + EVO-109 + EVO-110）
13. **EVO-100 Phase E'-4：Sandbox 废弃收尾**（EVO-111）
14. **EVO-012 / EVO-013 / EVO-014：租户管理、计费和审计增强**（独立进行，不阻塞主线）
15. **EVO-080：Wasmer/WASI 替代 Docker sandbox Spike**（如 EVO-111 收尾后无 runtime 候选则启动）

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
| 前端嵌入后端发布物 | [前端静态产物嵌入后端](../proposals/EMBEDDED-FRONTEND.md) | Done：Iteration 031 完成 rust-embed-for-web 迁移 |

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
| Phase B React + Vite + Bun | EVO-002 / EVO-021 至 EVO-025 | Done |
| Phase C forgot/reset password | EVO-003 | Done |
| Phase C invitation join | EVO-004 | Done |
| Phase C send/verify email | EVO-018 | Done |
| Phase D MCP 工具执行闭环 | EVO-005 / EVO-032 | Done |
| Phase E Skill update | EVO-006 | Done |
| Phase E CLI interface 迁移 | EVO-017 / EVO-009 | Done |
| Phase E Frontend snippet residue | EVO-026 | Done（Iteration 017） |
| Phase E' Git 托管 + Vibe Coding（2026-06-23 新主线） | EVO-100 + EVO-101~111 + EVO-104 + EVO-112 + EVO-113 | In Progress；EVO-101/102 Done，EVO-113 Done，EVO-104 UX gate resolved，EVO-103 为下一后端主线 |
| Phase E (old) Skill registry / Storage / 多来源 / 版本 / 描述 / CLI / MCP Serverless / 评分 / 生态兼容 | EVO-019 / EVO-020 / EVO-027 / EVO-028 / EVO-029 / EVO-045 / EVO-046 / EVO-047 / EVO-049 / EVO-049-B / EVO-050 | Superseded / Dropped by Phase E'（详见 [Product Backlog Archived Index](../backlog/PRODUCT-BACKLOG.md#archived-index)） |
| Phase F tenant members / api keys / settings / audit / billing | EVO-010 至 EVO-014 | EVO-010 / EVO-011 Done；EVO-012~014 Proposed |
| Phase G GitHub CI/CD 重建 | EVO-030 | Done（Iteration 029） |
| Rust CLI | Proposal: [RUST-CLI](../proposals/RUST-CLI.md) | Deferred |
| 前端嵌入后端发布物 | Proposal: [EMBEDDED-FRONTEND](../proposals/EMBEDDED-FRONTEND.md) | Done（Iteration 031） |
| Git-Centric Platform | Proposal: [GIT-CENTRIC-PLATFORM](../proposals/GIT-CENTRIC-PLATFORM.md) + [ADR-0004](../decisions/ADR-0004-git-centric-storage.md) + [ADR-0005](../decisions/ADR-0005-deprecate-sandbox-runtime.md) | 设计稿；2026-06-23 |
| AI Gateway / Agent Runtime | Proposals: [AI-GATEWAY](../proposals/AI-GATEWAY.md) / [AGENT-RUNTIME](../proposals/AGENT-RUNTIME.md) | 整合进 GIT-CENTRIC-PLATFORM |
| Serverless Runtime | Proposal: [SERVERLESS-RUNTIME](../proposals/SERVERLESS-RUNTIME.md) | 历史参考；ExecutionProvider 大幅简化（仅 HttpProxy） |
| Wasmer/WASI 替代 Docker sandbox Spike | [EVO-080](../backlog/active/EVO-080-spike-验证-wasmer-wasi-替代-docker-sandbox-可行性.md) | Ready；待 Phase E'-4 后视需要启动 |
