# Evolith - Enterprise AI Agent Harness Platform

> 本文件是 AI Agent 的启动文档。先读本文件建立约束和任务路由；复杂步骤不要堆在这里，按任务读取 `docs/sop/`、`docs/reference/` 或 `EVOLUTION.md`。

## Agent Engineering Rules

### Hard Constraints

- **流程操作先查 Task Router**：涉及需求进入、迭代变更、发布部署、数据库迁移等流程性操作时，必须先查下方 Task Router 找到必读 SOP，读完再动手——即使操作本身看起来很简单。
- **先看工作区状态**：修改前运行 `git status --short --branch`，识别用户已有改动；不要回滚无关变更。
- **Backlog first**：新功能、缺陷、技术债先进入 `docs/backlog/PRODUCT-BACKLOG.md`；紧急修复除外，但事后必须补记录。
- **迭代推进**：开始迭代按 `docs/sop/START-ITERATION.md` 固定步骤执行；进入开发前按 `docs/sop/REQUIREMENT-INTAKE.md` 检查 DoR，再按 `docs/sop/ITERATION-WORKFLOW.md` 推进；完成时检查 DoD 并更新 backlog/iteration 状态。
- **开始迭代先盘点既有计划**：用户要求开始迭代时，先检查 `docs/iterations/` 中
  `Active / In Progress / Review / Planned / Blocked` 的迭代并记录处置结论；未处理
  既有迭代前，不得直接从 backlog 选择新的 story。
- **Story 格式先匹配任务性质**：产品/API/权限/状态类行为工作必须写明角色、目标、
  价值和 Given/When/Then 验收；技术、治理和 Spike 不硬套用户故事，但必须写清
  工程目标、失败模式、命令级或一致性验证、状态同步和残余归口。
- **实施任务必须闭环**：凡是修改代码、配置或治理文档的实施任务，完成声明前必须按 `docs/sop/TASK-CLOSURE.md` 核对产物、状态同步、验证证据与残余归口；任一适用项缺失只能报告 `Partial` 或 `Blocked`，不得报告完成。
- **已发布迭代计划不可覆写**：已提交到仓库的 `Planned` iteration 是计划基线；启动同一范围时只能追加实际选入、执行、验证和复盘记录。若改为另一组 story 或另一目标，保留原计划并新建迭代编号，不得把旧计划文档改造成新工作的完成记录。
- **复杂任务分阶段结对**：跨多层、合约、数据库、权限、发布或高风险改动时，按 `docs/sop/PAIRING-WORKFLOW.md` 在 Driver 实现后切换 Navigator 审查；不要在同一段推理中并行扮演双角色。
- **中途变更先停手**：开发中收到需求变更时，先暂停扩大代码改动，按 `docs/sop/CHANGE-CONTROL.md` 做变更分类、backlog/ADR/iteration 记录，再继续。
- **文档分层**：需求池写 `docs/backlog/`，迭代记录写 `docs/iterations/`，决策写 `docs/decisions/`，操作流程写 `docs/sop/`，稳定事实写 `docs/reference/`，阶段计划写 `docs/roadmap/`，远期提案写 `docs/proposals/`，历史快照写 `docs/archive/`。
- **经验写回**：失败后找到根因、发现新陷阱、多次尝试后成功、用户指出遗漏时，按模板写入 `EVOLUTION.md`。
- **脚本行为变更必须写 Release Note**：修改 `scripts/*.sh`、部署脚本、构建脚本的参数、默认值、退出码、执行顺序或副作用时，更新 `docs/reference/SCRIPTS-RELEASE-NOTES.md`。
- **双数据库一致性**：数据库结构或 repository 行为变化必须同时考虑 SQLite 和 PostgreSQL migrations/repositories/tests。
- **前端 API 前缀**：`VITE_API_URL` 应包含 `/api/v1`，除非网关明确做路径重写；旧 `NEXT_PUBLIC_API_URL` 仅为兼容读取。
- **配置键格式**：后端嵌套配置使用双下划线环境变量，例如 `DATABASE__DATABASE_TYPE`，不要混用 `DATABASE_TYPE`。
- **重要取舍写 ADR**：技术栈、部署形态、认证/数据边界等重大决策必须写入 `docs/decisions/`。

### Git Rules

- 提交信息使用语义前缀：`feat:`、`fix:`、`docs:`、`refactor:`、`test:`、`chore:`、`perf:`、`security:`。
- Agent 参与生成的提交，提交信息末尾必须注明模型：`[model: <name>]`。
- 提交前必须检查 staged diff：`git diff --cached`。
- 不要用 `git add .` 盲加；除非已经确认所有变更都属于本次任务。
- 一次提交只表达一个主题；脚本行为变更与 `docs/reference/SCRIPTS-RELEASE-NOTES.md` 同步提交。
- 详细规则见 [docs/sop/GIT-WORKFLOW.md](docs/sop/GIT-WORKFLOW.md)。

### Task Router

| 任务类型 | 必读文档 | 按需参考 |
|----------|----------|----------|
| 了解项目结构 | [docs/reference/PROJECT-MAP.md](docs/reference/PROJECT-MAP.md) | [docs/README.md](docs/README.md) |
| 需求进入/拆分/排期 | [docs/sop/REQUIREMENT-INTAKE.md](docs/sop/REQUIREMENT-INTAKE.md) | [docs/backlog/PRODUCT-BACKLOG.md](docs/backlog/PRODUCT-BACKLOG.md) |
| 迭代中需求变更 | [docs/sop/CHANGE-CONTROL.md](docs/sop/CHANGE-CONTROL.md) | [docs/decisions/README.md](docs/decisions/README.md) |
| 开始一次迭代 | [docs/sop/START-ITERATION.md](docs/sop/START-ITERATION.md) | [docs/iterations/README.md](docs/iterations/README.md) |
| 复杂任务结对审查 | [docs/sop/PAIRING-WORKFLOW.md](docs/sop/PAIRING-WORKFLOW.md) | [docs/sop/ITERATION-WORKFLOW.md](docs/sop/ITERATION-WORKFLOW.md) |
| 本地启动/调试 | [docs/sop/LOCAL-DEV.md](docs/sop/LOCAL-DEV.md) | [EVOLUTION.md](EVOLUTION.md) |
| 新增功能/API/页面 | [docs/sop/NEW-FEATURE.md](docs/sop/NEW-FEATURE.md) | [docs/reference/API-CONTRACT.md](docs/reference/API-CONTRACT.md) |
| API 合约变更 | [docs/sop/CONTRACT-FIRST.md](docs/sop/CONTRACT-FIRST.md) | [docs/reference/API-CONTRACT.md](docs/reference/API-CONTRACT.md) |
| 数据库迁移 | [docs/sop/DATABASE-MIGRATION.md](docs/sop/DATABASE-MIGRATION.md) | [docs/reference/CONFIG.md](docs/reference/CONFIG.md) |
| 测试与验证 | [docs/sop/TESTING.md](docs/sop/TESTING.md) | [docs/reference/TESTING.md](docs/reference/TESTING.md) |
| 任务收口/完成声明 | [docs/sop/TASK-CLOSURE.md](docs/sop/TASK-CLOSURE.md) | [docs/sop/ITERATION-WORKFLOW.md](docs/sop/ITERATION-WORKFLOW.md) |
| 配置排查 | [docs/reference/CONFIG.md](docs/reference/CONFIG.md) | [docs/sop/LOCAL-DEV.md](docs/sop/LOCAL-DEV.md) |
| 发布/部署/回滚 | [docs/sop/RELEASE.md](docs/sop/RELEASE.md) | [docs/reference/SCRIPTS-RELEASE-NOTES.md](docs/reference/SCRIPTS-RELEASE-NOTES.md) |
| Git 提交 | [docs/sop/GIT-WORKFLOW.md](docs/sop/GIT-WORKFLOW.md) | [EVOLUTION.md](EVOLUTION.md) |
| 排查问题 | [EVOLUTION.md](EVOLUTION.md) | [docs/reference/PROJECT-MAP.md](docs/reference/PROJECT-MAP.md) |
| 文档整理 | [docs/sop/DOC-CHECK.md](docs/sop/DOC-CHECK.md) | [docs/README.md](docs/README.md) |
| 技术决策 | [docs/decisions/README.md](docs/decisions/README.md) | [docs/roadmap/IMPLEMENTATION-ROADMAP.md](docs/roadmap/IMPLEMENTATION-ROADMAP.md) |

### Current Known Traps

1. `docker-compose.yml` 里的后端环境变量必须使用 `DATABASE__DATABASE_TYPE` / `DATABASE__URL` 形式，否则 `AppConfig` 不会按预期读取嵌套配置。
2. 前端容器或本地环境的 `VITE_API_URL` 如果只有 `http://localhost:8080`，请求会打到 `/auth/...` 而不是 `/api/v1/auth/...`。
3. `CsrfMiddleware` 保护所有非豁免状态变更请求，手写 fetch 时要带 `X-CSRF-Token`。
4. Docker sandbox 初始化失败会降级到 `DefaultSkillExecutor`，不要只看接口返回成功就假定沙箱已启用。
5. SQLite 和 PostgreSQL SQL 类型、时间、JSON、UUID 行为不同，migration 不能简单复制后不验证。
6. 邮件链接必须使用 `APP__PUBLIC_URL` 指向前端公开地址；不要用后端监听地址拼 reset/invite 链接。
7. 当前 Nginx 托管 Vite 静态资源是 EVO-016 前的过渡策略；修改反代时必须验证 `/assets/` 不被 rewrite 成 `index.html`。
8. MCP `tools/call` 会触发真实出站请求，必须要求有效 API Key；HTTP executor 不得隐式探测系统代理，相关测试必须使用本地可控服务。
9. 已发布的 future iteration 可能承载后续依赖；不得用更高优先级工作就地改写其目标，否则会丢失原计划和依赖链。改线时创建新 iteration，并显式标注原计划保留或阻塞。
10. 文件已生成或代码已修改不等于任务完成；若验证、backlog/iteration 同步或已知残余归口缺失，必须按 `Partial` 报告并继续收口。
11. backlog 中没有 `In Progress` story 不表示可以直接开新迭代；iteration 文档可能
    仍处于待收口、待激活或阻塞状态，必须先盘点并处置。
12. 不要把传统 Scrum Sprint 或 BDD 机械套进所有工作；Evolith 的 iteration 是可审计
    工作批次，BDD 主要约束行为验收，技术/治理工作使用等价验证。

### Session End Checklist

- [ ] 是否留下未说明的代码或文档变更？
- [ ] 新功能/缺陷/技术债是否已进入 backlog，或说明了为什么不需要？
- [ ] 如果推进了迭代故事，是否更新了 backlog/iteration 状态？
- [ ] 如果开始了新迭代，是否先盘点并处置 `Active / In Progress / Review / Planned / Blocked` 的既有 iteration？
- [ ] 新增或选入 Story 时，格式是否匹配 Product / API / Technical / Governance / Spike，行为类是否已有 BDD 场景？
- [ ] 如果启动或改线了已发布的 planned iteration，是否保留原计划基线并避免就地替换目标？
- [ ] 如果发生中途需求变更，是否按变更分类表更新了 backlog、iteration、ADR 和半成品处理记录？
- [ ] 是否运行了与风险匹配的验证？未运行是否说明原因？
- [ ] 是否按 `TASK-CLOSURE.md` 给出 `Complete / Partial / Blocked` 结论，且适用的状态同步和残余归口均已落文档？
- [ ] 是否触发了 `EVOLUTION.md` 写回条件？
- [ ] 是否做了重大技术取舍但忘记写 ADR？
- [ ] 是否修改了脚本行为但忘记更新 `docs/reference/SCRIPTS-RELEASE-NOTES.md`？
- [ ] 如用户要求提交，commit message 是否包含语义前缀和 `[model: <name>]`？

## Project Overview

Evolith is an enterprise-grade AI Agent Harness platform providing:
- MCP Server tool encapsulation (remote MCP tools)
- Hybrid skill system (Claude Skills compatible + server-side code execution)
- CLI-friendly interface repository (replacing the legacy snippet concept)
- Governance, audit, permission, and deployment foundations for enterprise agent adoption

**Tech Stack**: Rust + Actix-web (backend), React + Vite + Bun static SPA frontend, SQLite (dev) / PostgreSQL (prod)

## Architecture

```
evolith/
├── backend/                 # Rust workspace
│   ├── crates/
│   │   ├── api/           # Routes, handlers, middleware, DTOs
│   │   ├── service-tool/  # MCP tools service
│   │   ├── service-skill/ # Skill execution service (Docker sandbox)
│   │   ├── service-snippet/# Code snippets service
│   │   ├── service-auth/  # JWT, password, RBAC
│   │   ├── service-audit/ # Audit logging
│   │   ├── service-payment/# Stripe billing integration
│   │   ├── domain/        # Models, Repository traits
│   │   ├── infra/         # Config, DB, cache, mailer, storage
│   │   └── common/        # Errors, utils, logging, sanitize
│   ├── migrations/
│   │   ├── sqlite/        # SQLite migrations (dev)
│   │   └── postgres/      # PostgreSQL migrations (prod)
│   └── sandbox/           # Sandbox Dockerfiles (Python, Node.js)
├── frontend/              # React + Vite + Bun + TypeScript + Tailwind
│   ├── src/app/          # Pages (22 routes)
│   ├── src/components/   # UI components
│   ├── src/lib/          # API client, i18n
│   ├── src/locales/      # zh-CN.json, en.json
│   ├── src/stores/       # State management (Zustand)
│   └── src/types/        # TypeScript interfaces
├── deploy/                # Nginx configs
├── scripts/               # dev.sh, backup.sh, deploy.sh
├── .github/workflows/     # CI/CD pipelines (EVO-030 rebuild)
└── docs/                  # Documentation
```

## Key Design Patterns

### Database Abstraction (Repository Trait)

All database operations use the Repository trait pattern for seamless SQLite↔PostgreSQL switching:

```rust
// Domain layer defines trait
pub trait UserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>>;
    async fn create(&self, user: &CreateUser) -> Result<User>;
}

// Infrastructure implements for specific DB
impl UserRepository for SqliteUserRepository { ... }
impl UserRepository for PostgresUserRepository { ... }
```

### Hybrid Skill System

Skills follow Claude SKILL.md format with server-side execution:
- Format: YAML frontmatter + Markdown content (agentskills.io spec)
- Execution: Sandboxed code runner on backend
- Storage: PostgreSQL with version history

### Error Handling

All errors use `common::error::AppError`:
```rust
pub enum AppError {
    DatabaseError(String),
    NotFoundError(String),
    ValidationError(String),
    AuthenticationError(String),
    AuthorizationError(String),
    ExternalServiceError { service: String, message: String },
    // ... more variants
}
```

### Logging & Request Tracing

- Request ID generated for each request (`X-Request-ID` header)
- Structured logging with tracing
- Slow request detection (>1000ms by default)

### Authentication

- JWT tokens with HS256, delivered via httpOnly cookie (`evolith_token`)
- CSRF double-submit cookie (`csrf_token`) for state-changing requests
- Password hashing with Argon2id
- Role-based access control (RBAC)
- Security headers middleware (CSP, X-Frame-Options, X-Content-Type-Options, etc.)

## Development Guidelines

### Adding New Features

1. **Create or update backlog item** in `docs/backlog/PRODUCT-BACKLOG.md`
2. **Check iteration readiness** using `docs/sop/REQUIREMENT-INTAKE.md`, then execute with `docs/sop/ITERATION-WORKFLOW.md`
3. **Update API contract** in `docs/reference/API-CONTRACT.md` when interfaces change
4. **Define domain models** in `crates/domain/`
5. **Implement repository trait** in domain layer
6. **Implement infrastructure** in appropriate service crate
7. **Add API routes** in `crates/api/`
8. **Add frontend** in `frontend/src/app/`
9. **Write tests** - unit tests in same file, integration tests in `tests/`
10. **Update backlog/iteration status** before finishing

### Testing

```bash
# Backend tests
cargo test -p <crate-name>

# Full workspace test
cargo test --workspace

# With coverage
cargo tarpaulin --workspace
```

### Code Style

- Use `cargo fmt` before commits
- Run `cargo clippy` to catch common mistakes
- Maximum line width: 100 characters
- Document public APIs with doc comments

### Database Migrations

Dual-track migrations: `backend/migrations/sqlite/` and `backend/migrations/postgres/` with DB-specific SQL syntax.

```bash
# Migrations run automatically on startup based on DATABASE__DATABASE_TYPE
# Manual migration (SQLite)
cargo sqlx migrate run --source backend/migrations/sqlite

# Manual migration (PostgreSQL)
cargo sqlx migrate run --source backend/migrations/postgres
```

## Configuration

Environment variables (see `crates/infra/src/config.rs`):

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE__URL` | Database connection string | `:memory:` (dev) |
| `DATABASE__DATABASE_TYPE` | sqlite/postgres/mysql | `sqlite` |
| `JWT__SECRET` | JWT signing key | (dev only) |
| `JWT__EXPIRATION` | Token lifetime | `24h` |
| `LOG__LEVEL` | trace/debug/info/warn/error | `info` |
| `ENVIRONMENT` | development/production | `development` |
| `APP__PUBLIC_URL` | Public frontend URL for email links | `http://localhost:3001` |
| `CORS__ALLOWED_ORIGIN` | Allowed frontend origin | `http://localhost:3001` |
| `CSRF__ENABLED` | Enable CSRF protection | `true` |
| `SANDBOX__ENABLED` | Enable sandbox executor | `true` |
| `SANDBOX__TIMEOUT_SECONDS` | Sandbox execution timeout | `30` |
| `SANDBOX__MEMORY_MB` | Sandbox memory limit | `256` |
| `RATE_LIMIT__UNAUTHENTICATED_RPM` | Unauthenticated request limit | `30` |
| `RATE_LIMIT__AUTHENTICATED_RPM` | Authenticated request limit | `300` |
| `RATE_LIMIT__API_KEY_RPM` | API key request limit | `1000` |
| `SMTP__HOST` | SMTP server host | (none) |
| `SMTP__PORT` | SMTP server port | `587` |
| `SMTP__FROM` | Sender email address | (none) |

MySQL currently has config/pool entry only; main service rejects it because MySQL repositories are not implemented. Use PostgreSQL for production.

## Common Tasks

### Add New API Endpoint

1. Define DTO in `crates/api/src/dto/`
2. Add handler in `crates/api/src/handlers/`
3. Register route in `crates/api/src/routes/`
4. Add validation using `validator` crate
5. Test with `cargo test -p api`

### Add New Service

1. Create crate in `backend/crates/service-<name>/`
2. Add to workspace `Cargo.toml`
3. Implement domain traits in `domain/`
4. Add API handlers in `api/`

### Database Switch (Dev → Prod)

1. Change `DATABASE__DATABASE_TYPE` to `postgres`
2. Update `DATABASE__URL` to production connection string
3. Run migrations: `cargo sqlx migrate run`
4. No code changes required (Repository pattern)

## Dependencies

### Backend Key Dependencies

| Crate | Purpose |
|-------|---------|
| actix-web | HTTP server |
| sqlx | Database ORM |
| serde | Serialization |
| tokio | Async runtime |
| jsonwebtoken | JWT handling |
| argon2 | Password hashing |
| tracing | Structured logging |
| validator | Input validation |
| bollard | Docker API (sandbox) |
| actix-governor | Rate limiting |
| lettre | SMTP email |
| cookie | httpOnly cookie support |

### Frontend Key Dependencies

| Package | Purpose |
|---------|---------|
| Vite | Static SPA build tool |
| Bun | Package manager and script runtime |
| React Router | SPA routing |
| TypeScript | Type safety |
| Tailwind CSS | Styling |
| Zustand | State management |
| Axios | HTTP client |
| React Hook Form | Form handling |
| react-i18next | Internationalization |

## Status

> **Architecture Audit** (2026-03-13): Found Phase 1-7 implementations were UI + in-memory HashMap prototypes only. All issues resolved in Phase 0-8.

### Prototype Phases (P1-P7) — UI/Handler Layer ✅, Infrastructure ❌

These phases created working UI and API handler code, but all backed by in-memory HashMap stores. All handler code was rewritten in Phase 0.

- P1: Core Framework (config, error handling, logging, API skeleton, frontend layout) ✅
- P2: Business Logic (auth handlers, CRUD handlers, MCP handler) ✅
- P3: Core Features (tool validation, skill loading, snippet reference) ✅
- P4: Frontend Integration (all UI pages and forms) ✅
- P5: Multi-Tenant & i18n (data model, tenant middleware, i18n deps installed but no translations) ⚠️ partial
- P6: User & Permissions (RBAC code, API key handlers, member handlers) ✅
- P7: Billing & Plans (billing handlers, Stripe integration) ✅

### Production Readiness Plan — Phase 0 ✅ COMPLETE (2026-03-14)

**Goal**: Replace all in-memory HashMap stores with database-backed repositories, implement real auth security, harden CORS.

**All Phase 0 tasks completed:**
- [x] 8 SQLite Repository implementations (user, tenant, invitation, audit, tool, skill, snippet, api_key) — `infra/src/db/`
- [x] 8 Domain Repository traits — `domain/src/repository.rs`
- [x] AppState struct with `Arc<dyn Repo>` fields — `api/src/state.rs`
- [x] AuthenticatedUser extractor with Deref to CurrentUser — `api/src/middleware/auth.rs`
- [x] Full RBAC middleware (425 lines) — `api/src/middleware/rbac.rs`
- [x] Fixed 3 infra compile errors (tool/skill/snippet repos)
- [x] Decoupled service-payment from api crate
- [x] Rewrote `main.rs` with AppState + DB pool + migrations + CORS
- [x] Rewrote all 9 handler files to use repositories (auth, tool, skill, snippet, api_key, mcp, member, audit, billing)
- [x] Updated all 9 route files
- [x] `cargo check --workspace --exclude service-payment` → 0 errors
- [x] `cargo test --workspace --exclude service-payment` → 70 passed, 0 failed
- [x] API contract document (`docs/reference/API-CONTRACT.md`) fully updated

**Build/test command**: `cargo test --workspace` (no exclusions needed — all crates compile clean)

### Phase 1 — Frontend Security ✅ COMPLETE (2026-03-14)

**Goal**: Route guards, error handling, token refresh, auth state restoration.

**All Phase 1 tasks completed:**
- [x] Next.js middleware route guard (cookie-based SSR check) — `frontend/src/middleware.ts`
- [x] AuthGuard client component (localStorage-based client check) — `frontend/src/components/AuthGuard.tsx`
- [x] ErrorBoundary / `error.tsx` — `frontend/src/app/error.tsx`
- [x] Global error handler — `frontend/src/app/global-error.tsx`
- [x] 404 page — `frontend/src/app/not-found.tsx`
- [x] Token refresh interceptor with mutex queue — `frontend/src/lib/api/client.ts`
- [x] AuthInitializer component (restores auth state, listens for `auth:unauthorized`) — `frontend/src/components/AuthInitializer.tsx`
- [x] Auth store `initialize()` method — `frontend/src/stores/authStore.ts`
- [x] AuthInitializer wired into `providers.tsx`
- [x] AuthGuard wired into `LayoutWrapper.tsx` (protects non-public routes)
- [x] Cookie/localStorage sync in `setToken()`/`clearToken()` for SSR middleware

### Phase 2 — Code Quality ✅ COMPLETE (2026-03-15)

**Goal**: Eliminate panic risks, fix compile errors, establish test baseline.

**All Phase 2 tasks completed:**
- [x] Eliminated `unwrap()` in billing_handlers.rs (only production unwrap found)
- [x] Added `#[deny(clippy::unwrap_used)]` to workspace Clippy config (all 11 Cargo.toml files)
- [x] Fixed service-payment compile errors (async-stripe API + Hash derive)
- [x] Split `auth_handlers.rs` (741 lines) → `handlers/auth/` (6 files: mod, login, register, password, verify, profile)
- [x] Split `member_handlers.rs` → `handlers/members/` (4 files: mod, list, invite, manage)
- [x] Updated handlers/mod.rs and all route imports
- [x] 111 repository integration tests (8 repo test files in `infra/tests/`)
- [x] 12 auth end-to-end tests (`api/tests/auth_e2e_tests.rs`)
- [x] `cargo test --workspace` → 206 passed, 0 failed
- [x] `cargo check --workspace` → 0 errors
- [x] `cargo clippy --workspace` → 0 errors

**Build/test command**: `cargo test --workspace`

### Phase 3 — Infrastructure Services ✅ COMPLETE (2026-03-15)

**Goal**: Cache, rate limiting, mailer, observability (request ID, health checks, JSON logging, graceful shutdown).

**All Phase 3 tasks completed:**
- [x] `Cache` trait + `InMemoryCache` + `RedisCache` — `infra/src/cache.rs` (457 lines)
- [x] `actix-governor` rate limiting middleware — `api/src/middleware/rate_limit.rs` (113 lines)
- [x] `Mailer` trait + `SmtpMailer` + `ConsoleMailer` — `infra/src/mailer.rs` (438 lines)
- [x] `RequestIdMiddleware` (X-Request-ID propagation) — `api/src/middleware/request_id.rs` (120 lines)
- [x] Health endpoints (`/health`, `/health/live`, `/health/ready`) — `api/src/handlers/health.rs`
- [x] Production JSON logging — `backend/src/main.rs`
- [x] Graceful shutdown (`shutdown_timeout(30)`) — `backend/src/main.rs`
- [x] AppState updated with `cache: Arc<dyn Cache>`, `mailer: Arc<dyn Mailer>`
- [x] E2E tests updated with new config/state fields
- [x] `cargo test --workspace` → 220 passed, 0 failed
- [x] `cargo check --workspace` → 0 errors
- [x] `cargo clippy --workspace` → 0 errors

**Build/test command**: `cargo test --workspace` (no exclusions needed — all crates compile clean)

### Phase 4 — PostgreSQL Dual-DB ✅ COMPLETE (2026-03-15)

**Goal**: Full PostgreSQL support alongside SQLite for production use.

**All Phase 4 tasks completed:**
- [x] Migration reorg: `migrations/sqlite/` and `migrations/postgres/` directories
- [x] PostgreSQL-native migrations (UUID, TIMESTAMPTZ, BOOLEAN, JSONB, NUMERIC)
- [x] All 8 PostgreSQL repository implementations (`Pg*Repository`)
- [x] `main.rs` dual-DB branching based on `DATABASE__DATABASE_TYPE`
- [x] `cargo test --workspace` → 232 passed, 0 failed
- [x] `cargo check --workspace` → 0 errors

### Phase 5 — i18n ✅ COMPLETE (2026-03-15)

**Goal**: Full Chinese/English internationalization for all frontend components.

**All Phase 5 tasks completed:**
- [x] i18n configuration (`frontend/src/lib/i18n.ts`)
- [x] Both locale files (`zh-CN.json` + `en.json`, ~647 lines each)
- [x] All 23+ components converted to `t()` calls
- [x] LanguageSwitcher component wired into Header
- [x] Legacy frontend build validation → 0 errors (pre-Vite migration)

### Phase 6 — Frontend Polish ✅ COMPLETE (2026-03-15)

**Goal**: Complete missing pages, upgrade token security, improve UX.

**All Phase 6 tasks completed:**
- [x] Onboarding wizard (3-step flow) — `frontend/src/app/onboarding/page.tsx`
- [x] User Profile page — `frontend/src/app/profile/page.tsx`
- [x] httpOnly cookie auth (`evolith_token`) + CSRF middleware (`csrf_token`) — `api/src/middleware/csrf.rs`
- [x] Frontend token migration (`withCredentials: true`, CSRF header) — `frontend/src/lib/api/client.ts`
- [x] Toast, Skeleton, Pagination UI components — `frontend/src/components/ui/`
- [x] Responsive mobile layout fixes
- [x] CORS fix for cookie credentials
- [x] `cargo test --workspace` → 236 passed, 0 failed
- [x] Legacy frontend build validation → 0 errors, 22 routes (pre-Vite migration)

### Phase 7 — Sandbox Executor ✅ COMPLETE (2026-03-15)

**Goal**: Docker-based sandboxed code execution for skills.

**All Phase 7 tasks completed:**
- [x] `SkillExecutor` trait + `DockerSkillExecutor` implementation — `service-skill/src/executor.rs`
- [x] Docker sandbox with `bollard` crate (Python 3.11 + Node.js 20 runtimes)
- [x] Resource limits: CPU (0.5 cores), memory (256MB), PIDs (64), network disabled, timeout (30s)
- [x] Sandbox Dockerfiles — `backend/sandbox/python/Dockerfile`, `backend/sandbox/node/Dockerfile`
- [x] AppState updated with `skill_executor: Arc<dyn SkillExecutor>`
- [x] `cargo test --workspace` → 236 passed, 0 failed

### Phase 8 — Production Deployment Baseline ⚠️ PARTIAL (2026-04-08)

**Goal**: Docker production stack, Nginx, CI/CD, security hardening.

**Implemented baseline tasks and deferred automation:**
- [x] Docker multi-stage build — `backend/Dockerfile` (Rust 1.82)
- [x] Frontend standalone build — `frontend/Dockerfile`
- [x] Production Docker Compose (5 services: postgres, redis, backend, frontend, nginx) — `docker-compose.prod.yml`
- [x] Nginx reverse proxy with SSL termination — `deploy/nginx/`
- [ ] CI pipeline — deferred to EVO-030 after final Bun/Vite commands and deployment shape stabilize
- [ ] Deploy pipeline — deferred to EVO-030 if deployment automation is still needed
- [x] `SecurityHeadersMiddleware` (CSP, X-Frame-Options, X-Content-Type-Options, X-XSS-Protection, Referrer-Policy) — `api/src/middleware/security_headers.rs`
- [x] Log sanitization (redacts password, token, secret, api_key) — `common/src/sanitize.rs`
- [x] Dev script with lite/full modes — `scripts/dev.sh`
- [x] Backup script — `scripts/backup.sh`
- [x] Deploy helper script — `scripts/deploy.sh`
- [x] Clippy `-D warnings` cleanup across workspace
- [x] `cargo test --workspace` → 237 passed, 0 failed
- [x] `bun run build` → 0 errors, Vite static SPA

### Current Delivery Status

Phases 0-7 and the Phase 8 deployment baseline are implemented. GitHub CI/CD automation remains deferred to EVO-030, and embedded frontend packaging remains deferred to EVO-016.

## Links

- [Requirements](./docs/reference/product/REQUIREMENTS.md)
- [Architecture](./docs/reference/ARCHITECTURE.md)
- [API Contract](./docs/reference/API-CONTRACT.md)
- [Multi-Tenant Design](./docs/reference/MULTI-TENANT.md)
- [Skill Format](./docs/reference/formats/SKILL-FORMAT.md)
- [Snippet Format](./docs/reference/formats/SNIPPET-FORMAT.md) — legacy migration reference
- [Tech Stack](./docs/reference/TECH-STACK.md)
- [Testing](./docs/reference/TESTING.md)
- [Billing](./docs/reference/BILLING.md)
- [Permissions](./docs/reference/PERMISSIONS.md)
- [i18n](./docs/reference/I18N.md)
- [Ideas](./docs/proposals/IDEAS.md)
