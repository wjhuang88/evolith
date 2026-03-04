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

**Phase 1: Core Framework** - COMPLETED ✅

Completed:
- [x] P1-001: Config management with validation
- [x] P1-002: Comprehensive error handling
- [x] P1-003: Logging system with request tracing
- [x] P1-004: Auth middleware (JWT + password hashing)
- [x] P1-005: API skeleton
- [x] P1-006: Unified response format
- [x] P1-007: Request validation
- [x] P1-008: Frontend layout components
- [x] P1-009: UI component library
- [x] P1-010: Theme system
- [x] P1-011: API client
- [x] P1-012: Auth state

**Phase 2: Business Logic** - COMPLETED ✅

Completed:
- [x] P2-001: Multi-tenant support (see docs/multi-tenant.md)
- [x] P2-002: User authentication (register, login, logout)
- [x] P2-003: Tool CRUD operations
- [x] P2-004: Skill CRUD operations
- [x] P2-005: Snippet CRUD operations
- [x] P2-006: MCP protocol implementation

**Phase 3: Core Features** - COMPLETED ✅

Completed:
- [x] P3-001: Tool parameter validation (JSON Schema)
- [x] P3-002: Skill remote loading
- [x] P3-003: Skill code execution (simulated sandbox)
- [x] P3-004: Snippet reference generation

**Phase 4: Frontend Integration** - COMPLETED ✅

Completed:
- [x] P4-001: Skill execution UI (modal with arguments input, execution result display)
- [x] P4-002: MCP tool invocation UI (direct API + JSON-RPC 2.0 protocol)
- [x] P4-003: Skill create/edit forms
- [x] P4-004: Snippet reference copy UI (3 formats: direct, inline, with_deps)
- [x] P4-005: Snippet create/edit forms
- [x] P4-006: Tool create/edit forms

**Phase 5: Multi-Tenant & i18n** - COMPLETED ✅

Completed:
- [x] P5-001: Multi-tenant data model (tenant_id on all tables)
- [x] P5-002: Tenant middleware (tenant identification)
- [x] P5-003: Tenant quota management (see docs/multi-tenant.md)
- [x] P5-004: i18n framework (react-i18next)
- [x] P5-005: Language switching component
- [x] P5-006: zh-CN and en translations

**Phase 6: User & Permissions** - COMPLETED ✅

Completed:
- [x] P6-000: MCP endpoint with tenant-aware API Key authentication
- [x] P6-001: User registration with tenant creation
- [x] P6-002: Role-based access control (RBAC)
- [x] P6-003: API Key management
- [x] P6-004: Member invitation system
- [x] P6-005: Audit logging

**Phase 7: Billing & Plans** - COMPLETED ✅

Completed:
- [x] P7-001: Plan definitions (Free/Starter/Pro/Enterprise)
- [x] P7-002: Subscription management
- [x] P7-004: Invoice management
- [x] P7-005: Usage tracking and quotas

Pending:
- [ ] P7-003: Payment integration (Stripe)
- [ ] P7-006: Overage billing

## Links

- [Requirements](./docs/requirements.md)
- [Architecture](./docs/architecture.md)
- [Multi-Tenant Design](./docs/multi-tenant.md)
- [API Design](./docs/api-design.md)
- [Skill Format](./docs/skill-format.md)
- [Snippet Format](./docs/snippet-format.md)
- [Tech Stack](./docs/tech-stack.md)
- [Testing](./docs/testing.md)
- [Implementation Plan](./docs/implementation-plan.md)
- [Session Handoff](./docs/session-handoff.md)