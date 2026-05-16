# Evolith - AI Agent Development Platform

> 本文件是 AI Agent 的启动文档。先读本文件建立约束和任务路由；复杂步骤不要堆在这里，按任务读取 `docs/sop/`、`docs/reference/` 或 `EVOLUTION.md`。

## Agent Engineering Rules

### Hard Constraints

- **流程操作先查 Task Router**：涉及需求进入、迭代变更、发布部署、数据库迁移等流程性操作时，必须先查下方 Task Router 找到必读 SOP，读完再动手——即使操作本身看起来很简单。
- **先看工作区状态**：修改前运行 `git status --short --branch`，识别用户已有改动；不要回滚无关变更。
- **Backlog first**：新功能、缺陷、技术债先进入 `docs/backlog/PRODUCT-BACKLOG.md`；紧急修复除外，但事后必须补记录。
- **迭代推进**：进入开发前按 `docs/sop/ITERATION-WORKFLOW.md` 检查 DoR；完成时检查 DoD 并更新 backlog/iteration 状态。
- **中途变更先停手**：开发中收到需求变更时，先暂停扩大代码改动，按 `docs/sop/ITERATION-WORKFLOW.md#5-迭代中需求变更` 做变更分类、backlog/ADR/iteration 记录，再继续。
- **文档分层**：需求池写 `docs/backlog/`，迭代记录写 `docs/iterations/`，决策写 `docs/decisions/`，操作流程写 `docs/sop/`，稳定事实写 `docs/reference/`，阶段计划写 `docs/roadmap/`，远期提案写 `docs/proposals/`，历史快照写 `docs/archive/`。
- **经验写回**：失败后找到根因、发现新陷阱、多次尝试后成功、用户指出遗漏时，按模板写入 `EVOLUTION.md`。
- **脚本行为变更必须写 Release Note**：修改 `scripts/*.sh`、部署脚本、构建脚本的参数、默认值、退出码、执行顺序或副作用时，更新 `docs/reference/SCRIPTS-RELEASE-NOTES.md`。
- **双数据库一致性**：数据库结构或 repository 行为变化必须同时考虑 SQLite 和 PostgreSQL migrations/repositories/tests。
- **前端 API 前缀**：`NEXT_PUBLIC_API_URL` 应包含 `/api/v1`，除非网关明确做路径重写。
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
| 需求进入/拆分/排期 | [docs/sop/ITERATION-WORKFLOW.md](docs/sop/ITERATION-WORKFLOW.md) | [docs/backlog/PRODUCT-BACKLOG.md](docs/backlog/PRODUCT-BACKLOG.md) |
| 迭代中需求变更 | [docs/sop/ITERATION-WORKFLOW.md#5-迭代中需求变更](docs/sop/ITERATION-WORKFLOW.md#5-迭代中需求变更) | [docs/decisions/README.md](docs/decisions/README.md) |
| 开始一次迭代 | [docs/iterations/README.md](docs/iterations/README.md) | [docs/iterations/ITERATION-TEMPLATE.md](docs/iterations/ITERATION-TEMPLATE.md) |
| 本地启动/调试 | [docs/sop/LOCAL-DEV.md](docs/sop/LOCAL-DEV.md) | [EVOLUTION.md](EVOLUTION.md) |
| 新增功能/API/页面 | [docs/sop/NEW-FEATURE.md](docs/sop/NEW-FEATURE.md) | [docs/reference/API-CONTRACT.md](docs/reference/API-CONTRACT.md) |
| API 合约变更 | [docs/sop/CONTRACT-FIRST.md](docs/sop/CONTRACT-FIRST.md) | [docs/reference/API-CONTRACT.md](docs/reference/API-CONTRACT.md) |
| 数据库迁移 | [docs/sop/DATABASE-MIGRATION.md](docs/sop/DATABASE-MIGRATION.md) | [docs/reference/CONFIG.md](docs/reference/CONFIG.md) |
| 测试与验证 | [docs/sop/TESTING.md](docs/sop/TESTING.md) | [docs/reference/TESTING.md](docs/reference/TESTING.md) |
| 配置排查 | [docs/reference/CONFIG.md](docs/reference/CONFIG.md) | [docs/sop/LOCAL-DEV.md](docs/sop/LOCAL-DEV.md) |
| 发布/部署/回滚 | [docs/sop/RELEASE.md](docs/sop/RELEASE.md) | [docs/reference/SCRIPTS-RELEASE-NOTES.md](docs/reference/SCRIPTS-RELEASE-NOTES.md) |
| Git 提交 | [docs/sop/GIT-WORKFLOW.md](docs/sop/GIT-WORKFLOW.md) | [EVOLUTION.md](EVOLUTION.md) |
| 排查问题 | [EVOLUTION.md](EVOLUTION.md) | [docs/reference/PROJECT-MAP.md](docs/reference/PROJECT-MAP.md) |
| 文档整理 | [docs/sop/DOC-CHECK.md](docs/sop/DOC-CHECK.md) | [docs/README.md](docs/README.md) |
| 技术决策 | [docs/decisions/README.md](docs/decisions/README.md) | [docs/roadmap/IMPLEMENTATION-ROADMAP.md](docs/roadmap/IMPLEMENTATION-ROADMAP.md) |

### Current Known Traps

1. `docker-compose.yml` 里的后端环境变量必须使用 `DATABASE__DATABASE_TYPE` / `DATABASE__URL` 形式，否则 `AppConfig` 不会按预期读取嵌套配置。
2. 前端容器的 `NEXT_PUBLIC_API_URL` 如果只有 `http://localhost:8080`，请求会打到 `/auth/...` 而不是 `/api/v1/auth/...`。
3. `CsrfMiddleware` 保护所有非豁免状态变更请求，手写 fetch 时要带 `X-CSRF-Token`。
4. Docker sandbox 初始化失败会降级到 `DefaultSkillExecutor`，不要只看接口返回成功就假定沙箱已启用。
5. SQLite 和 PostgreSQL SQL 类型、时间、JSON、UUID 行为不同，migration 不能简单复制后不验证。

### Session End Checklist

- [ ] 是否留下未说明的代码或文档变更？
- [ ] 新功能/缺陷/技术债是否已进入 backlog，或说明了为什么不需要？
- [ ] 如果推进了迭代故事，是否更新了 backlog/iteration 状态？
- [ ] 如果发生中途需求变更，是否按变更分类表更新了 backlog、iteration、ADR 和半成品处理记录？
- [ ] 是否运行了与风险匹配的验证？未运行是否说明原因？
- [ ] 是否触发了 `EVOLUTION.md` 写回条件？
- [ ] 是否做了重大技术取舍但忘记写 ADR？
- [ ] 是否修改了脚本行为但忘记更新 `docs/reference/SCRIPTS-RELEASE-NOTES.md`？
- [ ] 如用户要求提交，commit message 是否包含语义前缀和 `[model: <name>]`？

## Project Overview

Evolith is an AI agent development service platform providing:
- MCP Server tool encapsulation (remote MCP tools)
- Hybrid skill system (Claude Skills compatible + server-side code execution)
- CLI-friendly interface repository (replacing the legacy snippet concept)

**Tech Stack**: Rust + Actix-web (backend), Next.js 14 (frontend), SQLite (dev) / PostgreSQL (prod)

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
├── frontend/              # Next.js 14 + TypeScript + Tailwind
│   ├── src/app/          # Pages (22 routes)
│   ├── src/components/   # UI components
│   ├── src/lib/          # API client, i18n
│   ├── src/locales/      # zh-CN.json, en.json
│   ├── src/stores/       # State management (Zustand)
│   └── src/types/        # TypeScript interfaces
├── deploy/                # Nginx configs
├── scripts/               # dev.sh, backup.sh, deploy.sh
├── .github/workflows/     # CI/CD pipelines
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
2. **Check iteration readiness** using `docs/sop/ITERATION-WORKFLOW.md`
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
| `CORS__ALLOWED_ORIGIN` | Allowed frontend origin | `http://localhost:3000` |
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
| Next.js 14 | React framework |
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
- [x] i18n configuration (`src/lib/i18n.ts`)
- [x] Both locale files (`zh-CN.json` + `en.json`, ~647 lines each)
- [x] All 23+ components converted to `t()` calls
- [x] LanguageSwitcher component wired into Header
- [x] `npm run build` → 0 errors

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
- [x] `npm run build` → 0 errors, 22 routes

### Phase 7 — Sandbox Executor ✅ COMPLETE (2026-03-15)

**Goal**: Docker-based sandboxed code execution for skills.

**All Phase 7 tasks completed:**
- [x] `SkillExecutor` trait + `DockerSkillExecutor` implementation — `service-skill/src/executor.rs`
- [x] Docker sandbox with `bollard` crate (Python 3.11 + Node.js 20 runtimes)
- [x] Resource limits: CPU (0.5 cores), memory (256MB), PIDs (64), network disabled, timeout (30s)
- [x] Sandbox Dockerfiles — `backend/sandbox/python/Dockerfile`, `backend/sandbox/node/Dockerfile`
- [x] AppState updated with `skill_executor: Arc<dyn SkillExecutor>`
- [x] `cargo test --workspace` → 236 passed, 0 failed

### Phase 8 — Production Deployment ✅ COMPLETE (2026-04-08)

**Goal**: Docker production stack, Nginx, CI/CD, security hardening.

**All Phase 8 tasks completed:**
- [x] Docker multi-stage build — `backend/Dockerfile` (Rust 1.82)
- [x] Frontend standalone build — `frontend/Dockerfile`
- [x] Production Docker Compose (5 services: postgres, redis, backend, frontend, nginx) — `docker-compose.prod.yml`
- [x] Nginx reverse proxy with SSL termination — `deploy/nginx/`
- [x] CI pipeline (fmt + clippy -D warnings + tests + cargo audit + npm audit + Docker build) — `.github/workflows/ci.yml`
- [x] Deploy pipeline (manual trigger, GHCR push, SSH deploy, health check, rollback) — `.github/workflows/deploy.yml`
- [x] `SecurityHeadersMiddleware` (CSP, X-Frame-Options, X-Content-Type-Options, X-XSS-Protection, Referrer-Policy) — `api/src/middleware/security_headers.rs`
- [x] Log sanitization (redacts password, token, secret, api_key) — `common/src/sanitize.rs`
- [x] Dev script with lite/full modes — `scripts/dev.sh`
- [x] Backup script — `scripts/backup.sh`
- [x] Deploy helper script — `scripts/deploy.sh`
- [x] Clippy `-D warnings` cleanup across workspace
- [x] `cargo test --workspace` → 237 passed, 0 failed
- [x] `npm run build` → 0 errors, 22 routes

### All Phases Complete (Phase 0-8) ✅

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
