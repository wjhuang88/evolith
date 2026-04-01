# Evolith - AI Agent Development Platform

## Project Overview

Evolith is an AI agent development service platform providing:
- MCP Server tool encapsulation (remote MCP tools)
- Hybrid skill system (Claude Skills compatible + server-side code execution)
- Code snippet repository (LLM-optimized documentation)

**Tech Stack**: Rust + Actix-web (backend), Next.js 14 (frontend), SQLite (dev) / PostgreSQL (prod)

## Architecture

```
evolith/
├── backend/                 # Rust workspace
│   ├── crates/
│   │   ├── api/           # Routes, handlers, middleware, DTOs
│   │   ├── service-tool/  # MCP tools service
│   │   ├── service-skill/ # Skill execution service
│   │   ├── service-snippet/# Code snippets service
│   │   ├── service-auth/  # JWT, password, RBAC
│   │   ├── service-audit/ # Audit logging
│   │   ├── domain/        # Models, Repository traits
│   │   ├── infra/         # Config, DB, cache, storage
│   │   └── common/        # Errors, utils, logging
│   └── migrations/        # SQL migrations
├── frontend/              # Next.js 14 + TypeScript + Tailwind
│   ├── src/app/          # Pages (tools, skills, snippets)
│   ├── src/components/   # UI components
│   ├── src/lib/          # API client
│   ├── src/stores/       # State management
│   └── src/types/        # TypeScript interfaces
└── docs/                 # Documentation
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

- JWT tokens with HS256
- Password hashing with Argon2id
- Role-based access control (RBAC)

## Development Guidelines

### Adding New Features

1. **Create specification** in `docs/` first
2. **Define domain models** in `crates/domain/`
3. **Implement repository trait** in domain layer
4. **Implement infrastructure** in appropriate service crate
5. **Add API routes** in `crates/api/`
6. **Add frontend** in `frontend/src/app/`
7. **Write tests** - unit tests in same file, integration tests in `tests/`

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

```bash
# Create new migration
cargo sqlx migrate add <migration_name>

# Run migrations
cargo sqlx migrate run
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

### Frontend Key Dependencies

| Package | Purpose |
|---------|---------|
| Next.js 14 | React framework |
| TypeScript | Type safety |
| Tailwind CSS | Styling |
| Zustand | State management |
| Axios | HTTP client |
| React Hook Form | Form handling |

## Status

> **Architecture Audit** (2026-03-13): Found Phase 1-7 implementations were UI + in-memory HashMap prototypes only. See [Architecture Audit](./docs/architecture-audit.md).

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
- [x] API contract document (`docs/api-contract.md`) fully updated

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

**Build/test command**: `cargo test --workspace` (no exclusions needed — all crates compile clean)

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

### Current Phase: Phase 7 (Sandbox Executor)

See [Production Plan](./docs/production-plan.md) for Phase 1-8 roadmap.

## Links

- [Requirements](./docs/requirements.md)
- [Architecture](./docs/architecture.md)
- [Architecture Audit](./docs/architecture-audit.md) — Current state assessment
- [Production Design](./docs/production-design.md) — Target architecture
- [Production Plan](./docs/production-plan.md) — **Active execution plan (Phase 0-8)**
- [Multi-Tenant Design](./docs/multi-tenant.md)
- [API Design](./docs/api-design.md)
- [API Contract](./docs/api-contract.md)
- [Skill Format](./docs/skill-format.md)
- [Snippet Format](./docs/snippet-format.md)
- [Tech Stack](./docs/tech-stack.md)
- [Testing](./docs/testing.md)
- [Implementation Plan](./docs/implementation-plan.md) — ⚠️ Superseded by Production Plan
- [Billing](./docs/billing.md)
- [Permissions](./docs/permissions.md)
- [i18n](./docs/i18n.md)