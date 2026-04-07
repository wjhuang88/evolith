# Evolith 架构审计报告

> 审计日期: 2026-03-13
> 审计范围: 全代码库 (Rust 后端 + Next.js 前端)
> 审计目标: 评估生产就绪度，识别阻断性问题

---

## 执行摘要

Evolith 的**架构设计文档（设计意图）**质量优秀：分层清晰、Repository trait 模式正确、领域模型合理。但**实际实现与设计严重脱节**——几乎所有关键基础设施（数据库连接、认证中间件、RBAC、密码哈希）都只停留在 "代码存在但未接入" 的状态。

**核心判断：当前系统是一个 UI + 内存存储的原型，不是可上线的 SaaS 平台。**

---

## 1. 严重度分级

| 级别 | 定义 | 数量 |
|------|------|------|
| **CRITICAL** | 安全漏洞或数据丢失风险，阻止上线 | 9 |
| **HIGH** | 功能缺失或严重缺陷，影响核心体验 | 9 |
| **MEDIUM** | 质量/性能问题，不影响基本使用 | 7 |

---

## 2. CRITICAL 级发现

### C1: 数据库未接入 — 所有数据存储在内存 HashMap

**位置**: `backend/src/main.rs` 行 34-53

**现状**:
```rust
// main.rs - 实际代码
let auth_state = AuthState::new(&config.jwt);           // 内存 HashMap
let tool_store = std::sync::Arc::new(ToolStore::default()); // 内存 HashMap
let api_key_store = ApiKeyState::new();                  // 内存 HashMap
let member_state = MemberState::new();                   // 内存 HashMap
let audit_state = AuditState::new();                     // 内存 HashMap
let billing_state = BillingState::new();                 // 内存 HashMap
```

**影响**: 服务重启 = 所有数据丢失。用户、工具、技能、订阅、审计日志全部清空。

**现有资产**:
- `infra/src/db/pool.rs` — 数据库连接池创建代码 ✅ 已实现
- `infra/src/db/user_repo.rs` — UserRepository SQLite 实现 ✅ 已实现
- `infra/src/db/tenant_repo.rs` — TenantRepository SQLite 实现 ✅ 已实现
- `infra/src/db/invitation_repo.rs` — InvitationRepository SQLite 实现 ✅ 已实现
- `infra/src/db/audit_repo.rs` — AuditRepository SQLite 实现 ✅ 已实现
- `domain/src/repository.rs` — Repository trait 定义 ✅ 设计良好

**缺失**:
- `ToolRepository` 实现 — 不存在
- `SkillRepository` 实现 — 不存在
- `SnippetRepository` 实现 — 不存在
- `SubscriptionRepository` / `PaymentRepository` / `InvoiceRepository` — 不存在
- `main.rs` 中 `create_pool()` 从未被调用

---

### C2: 密码明文存储

**位置**: `backend/crates/api/src/handlers/auth_handlers.rs` 行 337

```rust
// 注册时直接存储明文密码
let password_hash = body.password.clone();
```

**位置**: `auth_handlers.rs` 行 167-174

```rust
fn verify_password(password: &str, hash: &str) -> bool {
    if hash == "demo123!" {
        return password == "demo123!";
    }
    password == hash  // 明文比较
}
```

**讽刺的是**: `service-auth/src/password.rs` 中有完整的 Argon2id 实现（带测试），但从未被调用。

---

### C3: 认证中间件为空

**位置**: `backend/crates/api/src/middleware/auth.rs`

```rust
//! Auth middleware (placeholder)
// TODO: Implement auth middleware
```

**影响**: 所有 API 端点可被任意访问，无需 token。JWT 虽然能生成，但没有中间件验证。

---

### C4: RBAC 中间件已实现但未挂载

**位置**: `backend/crates/api/src/middleware/rbac.rs` — 完整的 RBAC 逻辑
**位置**: `backend/src/main.rs` 行 67-68 — 只挂载了 Logger 和 Cors

```rust
App::new()
    // ... app_data ...
    .wrap(middleware::Logger::default())  // 仅日志
    .wrap(cors_configuration())           // 仅 CORS
    .configure(configure_routes)
    // RBAC? Auth? Rate Limit? — 全部缺失
```

---

### C5: CORS 完全开放

**位置**: `backend/src/main.rs` 行 88-89

```rust
fn cors_configuration() -> Cors {
    Cors::permissive()  // 接受任何来源、任何方法
}
```

**影响**: 任意网站可以向 Evolith API 发起请求，结合 C3（无认证），攻击面完全暴露。

---

### C6: Token Refresh 未实现

**位置**: `auth_handlers.rs` 行 423-433

```rust
pub async fn refresh_token(...) -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::<()>::error(
        "NOT_IMPLEMENTED",
        "Token refresh not implemented yet",
    ))
}
```

**影响**: Token 过期后用户必须重新登录，无法自动续期。

---

### C7: `get_current_user` 未实现

**位置**: `auth_handlers.rs` 行 436-441

```rust
pub async fn get_current_user(_state: web::Data<AuthState>) -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::<()>::error(
        "NOT_IMPLEMENTED",
        "Get current user not implemented yet",
    ))
}
```

**影响**: 前端无法获取当前用户信息，页面刷新后无法恢复登录状态。

---

### C8: `ApiKeyState::new()` 重复调用

**位置**: `backend/src/main.rs` 行 40-42

```rust
let api_key_store = ApiKeyState::new();     // 第一次
let api_key_store = ApiKeyState::new();     // 第二次，覆盖第一次
```

**影响**: 变量遮蔽（shadowing），虽然功能不受影响但说明代码审查流程缺失。

---

### C9: owner_id 和 tenant_id 硬编码

**位置**: 多个 handler 文件

```rust
// tool_handlers.rs, skill_handlers.rs, snippet_handlers.rs
owner_id: "00000000-0000-0000-0000-000000000001".to_string(),
tenant_id: "00000000-0000-0000-0000-000000000001".to_string(),
```

**影响**: 所有资源都归属同一个硬编码用户和租户，多租户数据隔离完全无效。

---

## 3. HIGH 级发现

### H1: JWT Token 存储在 localStorage

**位置**: `frontend/src/lib/api/client.ts` 行 10-22

```typescript
const TOKEN_KEY = 'evolith_token';
localStorage.setItem(TOKEN_KEY, token);
```

**风险**: XSS 攻击可窃取 token。应使用 httpOnly cookie。

---

### H2: 前端无路由守卫

**位置**: `frontend/src/` — 无 `middleware.ts` 文件

**影响**: 所有页面（包括 `/tools`, `/skills`, `/tenant/billing` 等需要登录的页面）可直接通过 URL 访问。

---

### H3: 无 ErrorBoundary

**位置**: `frontend/src/` — 全项目无 ErrorBoundary 组件

**影响**: 任何组件渲染错误导致整个应用白屏。

---

### H4: 81+ `.unwrap()` 调用

**位置**: 分布在非测试代码中

**影响**: 任何 unwrap 失败 = 线程 panic = 请求 500。在生产环境中不可接受。

---

### H5: SandboxedExecutor 未实现

**位置**: `service-skill/src/executor.rs`

```rust
pub fn execute(...) {
    todo!()  // panic!
}
```

**影响**: 技能代码执行功能完全不可用。

---

### H6: Cache 模块是占位符

**位置**: `infra/src/cache.rs` — 接口定义但无实现

**影响**: 无缓存层，所有请求直接访问数据库（一旦数据库接入后）。

---

### H7: 迁移文件仅兼容 SQLite

**位置**: `backend/migrations/` — 5 个迁移文件

```sql
-- 使用 SQLite 特有语法
created_at TEXT NOT NULL DEFAULT (datetime('now'))
-- UUID 用 TEXT 类型
id TEXT PRIMARY KEY
```

**影响**: 声称支持 PostgreSQL 的多数据库架构，但迁移文件无法在 PostgreSQL 上运行。需要编写 PostgreSQL 版本的迁移。

---

### H8: i18n 只有依赖没有实现

**位置**: `frontend/package.json` — 有 `i18next`, `react-i18next` 依赖

**影响**: AGENTS.md 标记 P5-004/005/006 为"完成"，但实际无翻译文件，无 `useTranslation` 调用。

---

### H9: forgot_password 提前 return，逻辑被跳过

**位置**: `auth_handlers.rs` 行 604-605

```rust
let _ = user;
return HttpResponse::Ok().json(...);  // 直接返回，下面的逻辑全部注释掉
```

**影响**: 密码重置功能不可用（即使有 reset_password handler）。

---

## 4. MEDIUM 级发现

### M1: 无速率限制

`RateLimitError` 类型存在于 `common/src/error.rs`，但无中间件实现。

### M2: 无请求大小限制

Actix-web 默认限制 256KB，但未显式配置。上传大文件/恶意请求无限制。

### M3: 无优雅关闭

`main.rs` 无 signal handler，SIGTERM 时直接杀进程，可能丢失进行中的请求。

### M4: Auth Store 泄露到 localStorage

`frontend/src/stores/authStore.ts` 使用 zustand `persist` 中间件将用户信息持久化到 localStorage。多租户场景下如果用户切换租户，旧数据可能残留。

### M5: 无 OpenAPI/Swagger 文档

API 端点无自动文档生成。前端和第三方接入者只能靠手写文档。

### M6: 缺失关键前端页面

- 无 onboarding 流程（新用户引导）
- 无用户 profile 页面
- 无 admin/超级管理员面板
- 无 404/500 错误页面

### M7: 无 Redis 集成

`docker-compose.yml` 配置了 Redis 7，config.rs 有 Redis 配置项，但代码中无任何 Redis 客户端使用。

---

## 5. 设计文档问题

### architecture.md
- **5.1 节重复出现**（行 306 和 394）：一个包含 tenant_id，一个不包含
- **SQL schema 重复出现**（行 458-565 和 567-633）：多租户版本和单租户版本共存
- **文档与实现不一致**：文档描述了完整的认证流程，但实现是空文件

### implementation-plan.md
- **状态不准确**：P6 标记"进行中"、P7 标记"待开始"，但 AGENTS.md 标记两者都"完成"
- **遗漏关键问题**：审查部分（R1-R10）发现了代码质量问题，但完全遗漏了"数据库未接入"、"密码明文"、"认证中间件为空"等 CRITICAL 级问题
- **工作量估计过于乐观**："18-24 天完成所有重构"但仅覆盖 P2 级问题

---

## 6. 正面发现（值得保留的设计）

| 组件 | 评价 |
|------|------|
| Repository trait 模式 (`domain/src/repository.rs`) | 设计优秀，正确抽象了数据访问层 |
| AppError 错误体系 (`common/src/error.rs`) | 完整、类型安全、覆盖了所有场景 |
| ApiResponse 统一响应格式 | 前后端契约清晰 |
| Argon2id 密码哈希 (`service-auth/src/password.rs`) | 实现正确，有单元测试 |
| JwtHandler (`service-auth/src/jwt.rs`) | 实现正确，支持 tenant_id |
| RBAC 中间件 (`api/src/middleware/rbac.rs`) | 逻辑完整，只需挂载 |
| AppConfig 配置系统 (`infra/src/config.rs`) | 环境变量 + 验证，设计合理 |
| CI Pipeline (`.github/workflows/ci.yml`) | fmt + clippy + build + test 全覆盖 |
| Docker Compose 基础设施 | PostgreSQL + Redis + MinIO 已定义 |
| 前端 API Client (`client.ts`) | 拦截器、错误处理、类型化 — 设计良好 |

---

## 7. 结论

### 可以上线吗？

> **更新 (2026-04-08)**: Phase 0-8 已全部完成。下方保留原始审计结论作为历史记录，实际解决状态见 [第 8 节](#8-解决状态)。

~~**不能。**~~ 原审计发现 9 个 CRITICAL 级和 9 个 HIGH 级问题。最核心的三个阻断：

1. ~~**无持久化存储** — 重启 = 全部数据丢失~~ → Phase 0 已解决
2. ~~**无认证/授权** — 所有 API 完全开放~~ → Phase 0 已解决
3. ~~**密码明文存储** — 直接违反安全法规~~ → Phase 0 已解决

**所有 9 个 CRITICAL 和 9 个 HIGH 级问题均已在 Phase 0-7 中解决。系统现已达到生产就绪状态。**

### 生产就绪文档

见 [生产就绪设计方案](./production-design.md) 和 [生产就绪实施计划](./production-plan.md)。

---

## 8. 解决状态

> 审计完成于 2026-03-13，所有问题在 2026-03-14 至 2026-03-15 期间（Phase 0-7）全部解决。

### 8.1 CRITICAL 级 — 全部解决 ✅

| 编号 | 问题 | 解决阶段 | 解决方案 |
|------|------|----------|----------|
| C1 | 数据库未接入（内存 HashMap） | Phase 0 | 8 个 SQLite Repository 实现 + AppState 使用 `Arc<dyn Repo>`，Phase 4 新增 PostgreSQL 双数据库支持 |
| C2 | 密码明文存储 | Phase 0 | 接入 `Argon2Hasher::hash_password()`，注册时哈希、登录时验证 |
| C3 | 认证中间件为空 | Phase 0 | 实现 `AuthenticatedUser` extractor，从 httpOnly cookie 提取并验证 JWT |
| C4 | RBAC 中间件未挂载 | Phase 0 | `RbacMiddleware` 挂载到 main.rs 中间件链 |
| C5 | CORS 完全开放 | Phase 0 | 配置为仅允许 `CORS__ALLOWED_ORIGIN` 指定的域，支持 credentials |
| C6 | Token Refresh 未实现 | Phase 0 | 实现 `/api/v1/auth/refresh` 端点，前端 Axios 拦截器自动续期 |
| C7 | `get_current_user` 未实现 | Phase 0 | 实现 `/api/v1/auth/me` 端点，从 JWT claims 查库返回用户信息 |
| C8 | `ApiKeyState::new()` 重复调用 | Phase 0 | 移除重复调用，统一为 Repository trait |
| C9 | owner_id / tenant_id 硬编码 | Phase 0 | 从 `AuthenticatedUser` extractor 动态获取，handler 重写 |

### 8.2 HIGH 级 — 全部解决 ✅

| 编号 | 问题 | 解决阶段 | 解决方案 |
|------|------|----------|----------|
| H1 | JWT Token 存储在 localStorage | Phase 6 | 迁移至 httpOnly cookie (`evolith_token`) + CSRF 双提交 cookie (`csrf_token`) |
| H2 | 前端无路由守卫 | Phase 1 | Next.js `middleware.ts` (SSR cookie 检查) + `AuthGuard` 客户端组件 |
| H3 | 无 ErrorBoundary | Phase 1 | `error.tsx` + `global-error.tsx` + `not-found.tsx` 全套错误处理 |
| H4 | 81+ `.unwrap()` 调用 | Phase 2 | 消除生产代码中的 unwrap，workspace 级 `clippy.unwrap_used = "deny"` |
| H5 | SandboxedExecutor 未实现 | Phase 7 | Docker 沙箱执行器 (`bollard` crate)，支持 Python 3.11 + Node.js 20，资源限制完整 |
| H6 | Cache 模块是占位符 | Phase 3 | `Cache` trait + `InMemoryCache` + `RedisCache` 双实现 |
| H7 | 迁移文件仅兼容 SQLite | Phase 4 | 双轨迁移：`migrations/sqlite/` + `migrations/postgres/`，PostgreSQL 原生语法 |
| H8 | i18n 只有依赖没有实现 | Phase 5 | 完整中英文翻译 (647 行/文件)，23+ 组件转 `t()` 调用，LanguageSwitcher 组件 |
| H9 | forgot_password 提前 return | Phase 0 | 重写密码重置流程，handler 正确实现 |

### 8.3 MEDIUM 级 — 解决情况

| 编号 | 问题 | 状态 | 解决方案 |
|------|------|------|----------|
| M1 | 无速率限制 | ✅ Phase 3 | `actix-governor` 中间件，可配置 `RATE_LIMIT__REQUESTS_PER_MINUTE` |
| M2 | 无请求大小限制 | ⚠️ 使用默认值 | Actix-web 默认 256KB，已满足当前需求 |
| M3 | 无优雅关闭 | ✅ Phase 3 | `shutdown_timeout(30)` 配置于 main.rs |
| M4 | Auth Store 泄露到 localStorage | ✅ Phase 6 | 迁移至 httpOnly cookie，localStorage 仅存非敏感用户信息 |
| M5 | 无 OpenAPI/Swagger 文档 | ⚠️ 使用手写文档 | `docs/api-contract.md` 作为前后端契约文档 |
| M6 | 缺失关键前端页面 | ✅ Phase 6 | Onboarding 向导、Profile 页面、404/500 错误页面 |
| M7 | 无 Redis 集成 | ✅ Phase 3 | `RedisCache` 实现，生产 Docker Compose 包含 Redis 7 服务 |
