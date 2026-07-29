# Evolith — AI-Native Git Platform

> AI-native git hosting + vibe coding + agent capability discovery.

> 本文件是 AI Agent 的启动文档。先读本文件建立硬约束、当前事实和任务路由；具体操作步骤按任务读取 `docs/sop/`，稳定结构读取 `docs/reference/`，历史经验读取 `EVOLUTION.md`。

## Agent Engineering Rules

### Hard Constraints

- **流程操作先查 Task Router**：需求进入、迭代变更、发布部署、数据库迁移等流程性操作必须先读取对应 SOP。
- **先看工作区状态**：修改前运行 `git status --short --branch`，识别用户已有改动；不要回滚无关变更。
- **Backlog first**：新功能、缺陷、技术债先进入 `docs/backlog/PRODUCT-BACKLOG.md`；可执行上下文写入 `docs/backlog/active/` item file。紧急修复允许先止血，但必须在同一工作批次补记录。
- **迭代推进**：开始迭代按 `START-ITERATION.md`；进入开发前检查 DoR；执行按 `ITERATION-WORKFLOW.md`；完成时检查 DoD 并同步状态。
- **开始迭代先盘点既有计划**：先检查 `Active / In Progress / Review / Planned / Blocked` 的 iteration，并记录继续、收口、激活、阻塞、延期或改线结论。
- **Story 格式匹配任务性质**：Product / API / Permission / State 行为工作需要角色、目标、价值和 Given/When/Then；Technical / Governance / Spike 使用等价技术验收，不机械套 BDD。
- **实施任务必须闭环**：修改代码、配置、脚本、测试或治理文档后，按 `TASK-CLOSURE.md` 核对产物、状态、验证和残余。缺任一适用项只能报告 `Partial` 或 `Blocked`。
- **已发布计划不可覆写**：已提交的 `Planned` iteration 是计划基线。目标变化时保留原计划并新建编号，不得把旧计划改写成另一项工作的完成记录。
- **复杂任务分阶段结对**：跨层、合约、数据库、权限、发布或高风险改动按 `PAIRING-WORKFLOW.md` 先 Driver 实现，再 Navigator 审查。
- **中途变更先停手**：开发中收到范围变化时先暂停扩大改动，按 `CHANGE-CONTROL.md` 分类并同步 backlog / ADR / iteration。
- **文档分层**：需求在 `docs/backlog/`，迭代在 `docs/iterations/`，决策在 `docs/decisions/`，步骤在 `docs/sop/`，稳定事实在 `docs/reference/`，阶段计划在 `docs/roadmap/`，远期提案在 `docs/proposals/`，历史在 `docs/archive/`。
- **经验写回**：失败后找到根因、发现新陷阱、多次尝试后成功或用户指出遗漏时，先按 `EVOLUTION-FEEDBACK.md` 判断是否写入 `EVOLUTION.md` 或升级规则。
- **脚本行为变更写 Release Note**：修改 `scripts/*.sh` 或部署/构建脚本的参数、默认值、退出码、顺序或副作用时，同步 `docs/reference/SCRIPTS-RELEASE-NOTES.md`。
- **双数据库一致性**：数据库结构或 Repository 行为变化必须同时考虑 SQLite 和 PostgreSQL migration、实现与测试。
- **前端 API 前缀**：`VITE_API_URL` 应包含 `/api/v1`，除非网关明确重写；旧 `NEXT_PUBLIC_API_URL` 只能作为兼容读取。
- **配置键格式**：后端嵌套配置使用双下划线，例如 `DATABASE__DATABASE_TYPE`，不要混用 `DATABASE_TYPE`。
- **重要取舍写 ADR**：技术栈、部署形态、认证、存储和数据边界等重大决策必须写入 `docs/decisions/`。

### Coding Behavior

#### 约束分类

影响多个文件的决策前，先分类约束：

| 类型 | 含义 | 处理方式 |
|------|------|----------|
| Hard | 平台限制、外部合约、不可逆边界 | 推导必要门禁 |
| Soft | 风格、范式、工具偏好 | 仅在影响选择时记录 |
| Assumption | 未验证的负载、兼容性或行为假设 | 标记并验证；阻塞时创建 Spike |

不要以“行业通常如此”替代本项目事实。每个门禁都应追溯到代码、配置、合约、ADR 或明确的用户要求。

#### 简单优先

- 只实现用户要求的结果，不添加推测性特性。
- 单次使用的代码不提前抽象。
- 不为不可能发生的场景堆叠错误处理。
- 能用更小范围解决时，不扩大重构。

#### 手术刀式变更

- 不顺手格式化或重构无关文件。
- 匹配现有风格。
- 只清理本次变更产生的孤儿代码。
- 每个变更行都应能追溯到当前请求、Backlog 或必要验证。

#### 目标驱动

把任务转换为可验证目标：

```text
修 bug -> 先能复现，再修复并证明回归通过
加校验 -> 为无效输入建立可观察失败，再让测试通过
重构 -> 重构前后行为和门禁一致
文档校准 -> 对照 Manifest / Code / ADR，完成链接与旧术语检查
```

多步任务先写“步骤 -> 验证”。

### Git Rules

- 提交信息使用 `feat:`、`fix:`、`docs:`、`refactor:`、`test:`、`chore:`、`perf:` 或 `security:`。
- 推荐格式：`type(scope): description (#story-id) [model: <model-name>]`。
- Agent 参与生成的提交必须以 `[model: <model-name>]` 结尾。
- 提交前检查 `git diff --cached`。
- 不用 `git add .` 盲加；显式选择本次文件。
- 一次提交只表达一个清晰主题。
- 不使用破坏性命令覆盖用户改动。
- 详细规则见 `docs/sop/GIT-WORKFLOW.md`。

## Task Router

| 任务类型 | 必读文档 | 按需参考 |
|----------|----------|----------|
| 了解项目结构 | [项目地图](docs/reference/PROJECT-MAP.md) | [文档地图](docs/README.md)、[架构](docs/reference/ARCHITECTURE.md) |
| 需求进入/拆分/排期 | [需求进入](docs/sop/REQUIREMENT-INTAKE.md) | [Product Backlog](docs/backlog/PRODUCT-BACKLOG.md) |
| 开始一次迭代 | [开始迭代](docs/sop/START-ITERATION.md) | [迭代目录](docs/iterations/README.md) |
| 迭代执行 | [迭代工作流](docs/sop/ITERATION-WORKFLOW.md) | [测试](docs/sop/TESTING.md) |
| 迭代中需求变更 | [变更控制](docs/sop/CHANGE-CONTROL.md) | [ADR 目录](docs/decisions/README.md) |
| 复杂任务结对审查 | [结对工作流](docs/sop/PAIRING-WORKFLOW.md) | [迭代工作流](docs/sop/ITERATION-WORKFLOW.md) |
| 本地启动/调试 | [本地开发](docs/sop/LOCAL-DEV.md) | [配置](docs/reference/CONFIG.md)、[经验](EVOLUTION.md) |
| 新增功能/API/页面 | [新增功能](docs/sop/NEW-FEATURE.md) | [API 合约](docs/reference/API-CONTRACT.md) |
| API 合约变更 | [Contract First](docs/sop/CONTRACT-FIRST.md) | [API 合约](docs/reference/API-CONTRACT.md) |
| 数据库迁移 | [数据库迁移](docs/sop/DATABASE-MIGRATION.md) | [配置](docs/reference/CONFIG.md) |
| 测试与验证 | [测试 SOP](docs/sop/TESTING.md) | [测试参考](docs/reference/TESTING.md) |
| 任务收口 | [任务闭环](docs/sop/TASK-CLOSURE.md) | [迭代工作流](docs/sop/ITERATION-WORKFLOW.md) |
| 发布/部署/回滚 | [发布](docs/sop/RELEASE.md) | [脚本 Release Notes](docs/reference/SCRIPTS-RELEASE-NOTES.md) |
| Git 提交 | [Git 工作流](docs/sop/GIT-WORKFLOW.md) | [经验](EVOLUTION.md) |
| 问题排查/经验写回 | [经验反馈](docs/sop/EVOLUTION-FEEDBACK.md) | [经验](EVOLUTION.md) |
| 文档整理 | [文档检查](docs/sop/DOC-CHECK.md) | [文档地图](docs/README.md) |
| 技术决策 | [ADR 目录](docs/decisions/README.md) | [实施路线图](docs/roadmap/IMPLEMENTATION-ROADMAP.md) |

## Current Known Traps

1. 当前 Rust MSRV 是 **1.88**，与 `backend/Cargo.toml` 和 `backend/Dockerfile` 一致；不要沿用历史 1.75 / 1.82 说明。
2. Evolith 当前是**模块化单体**：多个 crate 默认构建成一个 Actix 进程，不要把 crate 边界描述成独立微服务。
3. `docker-compose.yml` 必须使用 `DATABASE__DATABASE_TYPE` / `DATABASE__URL`，否则嵌套配置不会按预期读取。
4. `VITE_API_URL` 只有 `http://localhost:8080` 时会请求 `/auth/...`，应包含 `/api/v1`，除非网关重写。
5. `CsrfMiddleware` 保护非豁免状态变更请求，浏览器手写 fetch 需要 `X-CSRF-Token`。
6. `SANDBOX__ENABLED` 默认关闭。ADR-0005 已决定废弃 Sandbox；EVO-111 前仅作 legacy 兼容，新功能不得依赖它。
7. SQLite 与 PostgreSQL 的类型、时间、JSON、UUID 和事务行为不同，migration 不能复制后不验证。
8. 邮件链接必须使用 `APP__PUBLIC_URL`，不要用后端监听地址拼接 reset / invite URL。
9. 前端资源已嵌入后端；修改路由或网关时验证 `/assets/`、`/api/v1`、`/repos/`、`/health`、`/mcp` 和 SPA fallback 不互相截获。
10. MCP `tools/call` 会产生真实出站请求，必须要求有效 API Key；测试使用本地可控服务，不隐式依赖系统代理。
11. Git Smart HTTP 使用 Git subprocess；Repo Context 使用 `gix`。不要混淆两条实现路径。
12. Git 仓库位于文件系统。多实例部署必须解决共享/一致存储或路由问题，只共享 PostgreSQL 不够。
13. MinIO 不是当前 Git 仓库主存储路径；新的 Git-centric 能力不要默认依赖对象存储。
14. 文件存在或代码已改不等于完成；验证、状态同步或残余归口缺失时必须报告 `Partial`。
15. Backlog 没有 `In Progress` 不代表可以直接开新迭代；先盘点非终态 iteration。
16. 已发布的 future iteration 不得因更高优先级工作被就地改写。
17. Release 构建不得包含 sourcemap、硬编码 Secret 或调试环境变量注入。
18. 依赖版本以 Manifest + Lockfile 为准，不从历史 Phase 或 Iteration 清单推断。

## Session End Checklist

- [ ] 是否留下未说明的代码或文档变更？
- [ ] 新功能、缺陷或技术债是否进入 Backlog，或说明无需新增记录？
- [ ] 是否先更新 owner docs，再同步 Board 等派生视图？
- [ ] 是否处理了非终态 iteration 库存？
- [ ] Story 类型和验收格式是否匹配任务性质？
- [ ] 是否保留已发布计划基线？
- [ ] 中途范围变化是否走变更控制？
- [ ] 是否执行风险匹配的验证，并记录真实结果？
- [ ] 是否按 `TASK-CLOSURE.md` 给出 `Complete / Partial / Blocked`？
- [ ] 是否有需写回 `EVOLUTION.md` 的新陷阱？
- [ ] 是否做了需要 ADR 的重大取舍？
- [ ] 是否修改脚本但遗漏 Release Note？
- [ ] Agent 提交信息是否包含模型标识？

## Current Project Baseline

### Product Direction

Evolith is a Git-centric AI development platform:

- Git repository hosting with agent-ready collaboration.
- Repo Context APIs for tree, blob, commit and diff.
- Vibe Coding with controlled Agent commits and promotion.
- Skill / MCP / CLI discovery derived from repository content.
- Enterprise governance through tenant isolation, RBAC, audit and scoped credentials.

2026-06-23 后，旧 DB-centric Skill / CLI / MCP Registry 不再是产品主线。Git Repository 是基础载体，能力索引是衍生视图。

### Architecture

```text
evolith/
├── backend/                       # Rust Workspace -> one evolith process
│   ├── src/main.rs               # composition root
│   ├── crates/
│   │   ├── api/                  # routes, handlers, middleware, DTO, AppState
│   │   ├── domain/               # models and Repository traits
│   │   ├── infra/                # config, SQLite/PG, cache, mailer
│   │   ├── common/               # errors, logging, sanitize, utilities
│   │   ├── service-git/          # gix repository context
│   │   ├── service-auth/         # auth services
│   │   ├── service-audit/        # audit
│   │   ├── service-tool/         # MCP Tool compatibility
│   │   ├── service-skill/        # parser + legacy Sandbox pending EVO-111
│   │   ├── service-snippet/      # legacy Snippet / CLI compatibility
│   │   └── service-payment/      # Stripe
│   ├── migrations/sqlite/
│   ├── migrations/postgres/
│   └── sandbox/                  # legacy, pending deletion
├── frontend/                      # React 19 + Vite 8 + Bun + TypeScript 6
├── deploy/                        # optional Nginx / Kubernetes
├── scripts/                       # local, backup and deploy helpers
├── docs/                          # reference, SOP, backlog, iteration, ADR
└── .github/workflows/             # tag-driven CI
```

### Runtime and Storage

- Rust 1.88, Edition 2021.
- Actix-web 4.13 and Tokio 1.49.
- SQLite for Lite development, PostgreSQL 16 for production.
- Git repositories on server filesystem.
- Redis optional cache.
- React static assets embedded via `rust-embed-for-web`.
- Nginx optional; not required to serve the SPA.

### Key Patterns

#### Git-Centric Source of Truth

```text
Git repository
├── code and history
├── .evolith/policy.yaml
├── SKILL.md
├── interface.yaml
└── tool.yaml
```

Skill / CLI / MCP discovery should be derived from repository content. Legacy database rows remain compatibility data until the Indexer and migration work are complete.

#### Database Repository Pattern

Domain defines traits; Infrastructure implements SQLite and PostgreSQL variants; API receives trait objects through `AppState`.

#### Authentication

- Browser: httpOnly Cookie JWT + CSRF.
- API Client: API Key + scope / RBAC.
- Git Client: Basic challenge mapped to credentials and repository authorization.
- Password: Argon2id.

#### Legacy Sandbox

Docker-based Skill execution exists for compatibility only, is disabled by default, and is scheduled for deletion by EVO-111. Do not expand it or use it as a dependency for new Git-centric work.

## Development Baseline

### Backend

```bash
cd backend
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

### Frontend

```bash
cd frontend
bun install --frozen-lockfile
bun run type-check
bun run build
bun run lint
```

### Database Migrations

```bash
cargo sqlx migrate run --source backend/migrations/sqlite
cargo sqlx migrate run --source backend/migrations/postgres
```

Migrations normally run on startup according to `DATABASE__DATABASE_TYPE`.

### Configuration

| Variable | Meaning | Default / Boundary |
|----------|---------|--------------------|
| `DATABASE__DATABASE_TYPE` | `sqlite` / `postgres` | `sqlite` |
| `DATABASE__URL` | connection string | `:memory:` in dev |
| `JWT__SECRET` | JWT signing secret | development value must be replaced in production |
| `APP__PUBLIC_URL` | public frontend URL | `http://localhost:3001` |
| `CORS__ALLOWED_ORIGIN` | browser origin | `http://localhost:3001` |
| `CSRF__ENABLED` | CSRF protection | `true` |
| `SANDBOX__ENABLED` | legacy Docker execution | `false` |
| `RATE_LIMIT__UNAUTHENTICATED_RPM` | unauthenticated limit | `30` |
| `RATE_LIMIT__AUTHENTICATED_RPM` | authenticated limit | `300` |
| `RATE_LIMIT__API_KEY_RPM` | API Key limit | `1000` |

MySQL is not supported by the main service even though SQLx has a MySQL feature enabled.

## Links

- [Documentation Map](./docs/README.md)
- [Project Map](./docs/reference/PROJECT-MAP.md)
- [Architecture](./docs/reference/ARCHITECTURE.md)
- [Tech Stack](./docs/reference/TECH-STACK.md)
- [API Contract](./docs/reference/API-CONTRACT.md)
- [Testing](./docs/reference/TESTING.md)
- [Permissions](./docs/reference/PERMISSIONS.md)
- [Multi-Tenant Design](./docs/reference/MULTI-TENANT.md)
- [Implementation Roadmap](./docs/roadmap/IMPLEMENTATION-ROADMAP.md)
- [Evolution Notes](./EVOLUTION.md)
