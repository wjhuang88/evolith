# Evolith 生产就绪设计方案

> 基于 [架构审计报告](./architecture-audit.md) 的发现，本文档定义将 Evolith 从原型提升为生产级 SaaS 平台所需的架构调整和技术方案。

---

## 1. 设计原则

1. **接线优先 (Wire First)** — 优先利用已有代码（Repository 实现、密码哈希器、RBAC 中间件），而非重写
2. **最小变更面 (Minimal Surface)** — 每次修改只影响必要的层级，保持已验证代码不变
3. **渐进式迁移** — 先 SQLite 跑通全链路，再扩展 PostgreSQL
4. **安全不妥协** — 密码哈希、认证中间件、CORS 限制在第一天就必须到位

---

## 2. 后端架构调整

### 2.1 应用初始化重构 (`main.rs`)

**目标**: 将 `main.rs` 从"内存存储初始化"改为"数据库 + 服务层初始化"。

**当前流程**:
```
main() → 创建内存 HashMap → 注入 handlers → 启动
```

**目标流程**:
```
main() → 加载配置 → 创建数据库连接池 → 运行迁移 → 
         创建 Repository 实例 → 创建 Service 实例 → 
         挂载中间件链 → 注入 handlers → 优雅启动/关闭
```

**中间件链** (按执行顺序):
```rust
App::new()
    .app_data(web::Data::new(app_state.clone()))
    .wrap(middleware::Logger::default())
    .wrap(RequestIdMiddleware)        // 请求追踪
    .wrap(cors_configuration())       // 限制性 CORS
    .wrap(RateLimitMiddleware)        // 速率限制
    .wrap(AuthMiddleware)             // JWT 验证
    // 注意: RBAC 在路由级别通过 guard 或 extractor 实现，不在全局中间件
    .configure(configure_routes)
```

**AppState 结构** (替代分散的多个 State):
```rust
pub struct AppState {
    pub config: AppConfig,
    pub db: DatabasePool,
    pub jwt: JwtHandler,
    pub hasher: Argon2Hasher,
    // Repositories (trait objects for testability)
    pub user_repo: Box<dyn UserRepository>,
    pub tenant_repo: Box<dyn TenantRepository>,
    pub tool_repo: Box<dyn ToolRepository>,
    pub skill_repo: Box<dyn SkillRepository>,
    pub snippet_repo: Box<dyn SnippetRepository>,
    pub api_key_repo: Box<dyn ApiKeyRepository>,
    pub audit_repo: Box<dyn AuditRepository>,
    pub subscription_repo: Box<dyn SubscriptionRepository>,
    // 可选服务
    pub cache: Option<Box<dyn Cache>>,
    pub mailer: Option<Box<dyn Mailer>>,
}
```

### 2.2 认证中间件实现

**文件**: `backend/crates/api/src/middleware/auth.rs`

**职责**:
1. 从 `Authorization: Bearer <token>` 提取 JWT
2. 验证签名和过期时间
3. 将 `Claims { user_id, tenant_id, role, tenant_role }` 注入请求扩展
4. 白名单路径跳过认证（`/api/v1/auth/login`, `/api/v1/auth/register`, `/health`）

**RBAC 实现方式**: 使用 Actix-web 的 Extractor 模式而非全局中间件

```rust
// 在 handler 函数签名中声明权限要求
pub async fn create_tool(
    claims: AuthenticatedUser,           // 自动验证 JWT
    tenant: TenantContext,               // 自动提取租户上下文
    _guard: RequireRole<AdminOrOwner>,   // 编译时权限检查
    body: web::Json<CreateToolRequest>,
    state: web::Data<AppState>,
) -> Result<HttpResponse> { ... }
```

### 2.3 Handler 重构为 Service 调用

**当前** (handler 直接操作内存 HashMap):
```rust
pub async fn create_tool(body: web::Json<CreateToolRequest>, store: web::Data<ToolStore>) {
    let tool = Tool { id: Uuid::new_v4(), owner_id: "00000000...".to_string(), ... };
    store.tools.lock().unwrap().insert(tool.id.clone(), tool);
}
```

**目标** (handler 只做请求/响应转换，业务逻辑在 Service 层):
```rust
pub async fn create_tool(
    claims: AuthenticatedUser,
    body: web::Json<CreateToolRequest>,
    state: web::Data<AppState>,
) -> Result<HttpResponse> {
    body.validate()?;
    let tool = state.tool_repo.create(&CreateTool {
        tenant_id: claims.tenant_id,
        owner_id: claims.user_id,
        name: body.name.clone(),
        // ...
    }).await?;
    Ok(HttpResponse::Created().json(ApiResponse::success(tool)))
}
```

### 2.4 缺失 Repository 实现

需要新增以下 Repository 的 SQLite 实现:

| Repository | 文件路径 | 预估行数 |
|------------|----------|----------|
| `ToolRepository` | `infra/src/db/tool_repo.rs` | ~200 |
| `SkillRepository` | `infra/src/db/skill_repo.rs` | ~250 |
| `SnippetRepository` | `infra/src/db/snippet_repo.rs` | ~250 |
| `SubscriptionRepository` | `infra/src/db/subscription_repo.rs` | ~200 |
| `PaymentRepository` | `infra/src/db/payment_repo.rs` | ~150 |
| `ApiKeyRepository` | `infra/src/db/api_key_repo.rs` | ~150 |

**实现模式**: 参考已有的 `user_repo.rs` 和 `tenant_repo.rs`，使用 `sqlx::query_as!` 宏。

### 2.5 密码安全

**修复方案**:

```rust
// 注册时
let password_hash = state.hasher.hash_password(&body.password)?;

// 登录时
let valid = state.hasher.verify_password(&body.password, &user.password_hash)?;

// 重置密码时
let new_hash = state.hasher.hash_password(&body.new_password)?;
```

所有密码操作统一通过 `service-auth/src/password.rs` 中已有的 `Argon2Hasher`。

### 2.6 CORS 安全配置

```rust
fn cors_configuration(config: &AppConfig) -> Cors {
    match config.environment.as_str() {
        "production" => Cors::default()
            .allowed_origin(&config.cors.allowed_origin)  // 如 "https://app.evolith.io"
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::CONTENT_TYPE,
                header::ACCEPT,
                HeaderName::from_static("x-request-id"),
                HeaderName::from_static("x-tenant-id"),
            ])
            .max_age(3600),
        _ => Cors::permissive(),  // 开发环境保持宽松
    }
}
```

### 2.7 优雅关闭

```rust
let server = HttpServer::new(move || { ... })
    .bind((host.as_str(), port))?
    .shutdown_timeout(30)  // 30 秒等待
    .run();

// 信号处理
tokio::select! {
    result = server => result?,
    _ = tokio::signal::ctrl_c() => {
        info!("Received SIGINT, shutting down gracefully");
    }
}
```

---

## 3. 数据库迁移策略

### 3.1 问题

现有 5 个迁移文件使用 SQLite 语法 (`datetime('now')`, `TEXT` for UUID)，无法在 PostgreSQL 上运行。

### 3.2 方案：双轨迁移

```
backend/migrations/
├── sqlite/
│   ├── 001_init.sql
│   ├── 002_tenants.sql
│   ├── 003_invitations.sql
│   ├── 004_user_permissions.sql
│   └── 005_billing.sql
└── postgres/
    ├── 001_init.sql
    ├── 002_tenants.sql
    ├── 003_invitations.sql
    ├── 004_user_permissions.sql
    └── 005_billing.sql
```

**启动时根据 `DATABASE__DATABASE_TYPE` 选择迁移目录**。

### 3.3 PostgreSQL 迁移关键差异

| SQLite | PostgreSQL |
|--------|-----------|
| `TEXT PRIMARY KEY` | `UUID PRIMARY KEY DEFAULT gen_random_uuid()` |
| `datetime('now')` | `NOW()` |
| `TEXT` (JSON) | `JSONB` |
| `TEXT[]` 不支持 | `TEXT[]` 支持 |
| `BOOLEAN` (0/1) | `BOOLEAN` (true/false) |
| `REAL` | `NUMERIC(12,2)` |

---

## 4. 前端安全与基础设施

### 4.1 Token 存储迁移 (localStorage → httpOnly Cookie)

**方案**: 后端登录成功后通过 `Set-Cookie` 设置 httpOnly cookie

```rust
// 后端设置 cookie
HttpResponse::Ok()
    .cookie(
        Cookie::build("evolith_session", &token)
            .http_only(true)
            .secure(true)       // 仅 HTTPS
            .same_site(SameSite::Strict)
            .path("/")
            .max_age(Duration::hours(24))
            .finish()
    )
    .json(ApiResponse::success(response))
```

**前端变更**:
- 移除 `localStorage.setItem(TOKEN_KEY, ...)` 
- 移除 `localStorage.getItem(TOKEN_KEY)`
- Axios 配置 `withCredentials: true`
- 移除前端手动设置 `Authorization` header
- Cookie 由浏览器自动发送

**过渡方案**: 如果短期内无法改 cookie 方案，至少确保：
- 实施 CSP (Content Security Policy) 防止 XSS
- Token 设置短过期时间 (如 1h) + refresh token 机制

### 4.2 路由守卫 (`middleware.ts`)

```typescript
// frontend/src/middleware.ts
import { NextResponse } from 'next/server';
import type { NextRequest } from 'next/server';

const PUBLIC_PATHS = ['/login', '/register', '/forgot-password', '/reset-password', '/verify-email'];

export function middleware(request: NextRequest) {
  const { pathname } = request.nextUrl;
  
  // 公开路径不需要认证
  if (PUBLIC_PATHS.some(p => pathname.startsWith(p))) {
    return NextResponse.next();
  }
  
  // 检查认证 cookie/token
  const token = request.cookies.get('evolith_session');
  if (!token) {
    return NextResponse.redirect(new URL('/login', request.url));
  }
  
  return NextResponse.next();
}

export const config = {
  matcher: ['/((?!_next/static|_next/image|favicon.ico|api).*)'],
};
```

### 4.3 ErrorBoundary

```tsx
// frontend/src/components/ErrorBoundary.tsx
'use client';

import { Component, ReactNode } from 'react';

interface Props {
  children: ReactNode;
  fallback?: ReactNode;
}

interface State {
  hasError: boolean;
  error?: Error;
}

export class ErrorBoundary extends Component<Props, State> {
  state: State = { hasError: false };

  static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, info: React.ErrorInfo) {
    console.error('ErrorBoundary caught:', error, info);
    // TODO: 发送到错误监控服务 (如 Sentry)
  }

  render() {
    if (this.state.hasError) {
      return this.props.fallback || (
        <div className="flex items-center justify-center min-h-screen">
          <div className="text-center">
            <h1 className="text-2xl font-bold text-gray-900">出现了一些问题</h1>
            <p className="mt-2 text-gray-600">请刷新页面或联系技术支持</p>
            <button
              onClick={() => this.setState({ hasError: false })}
              className="mt-4 px-4 py-2 bg-blue-600 text-white rounded-md"
            >
              重试
            </button>
          </div>
        </div>
      );
    }
    return this.props.children;
  }
}
```

### 4.4 i18n 实际接入

**当前**: `i18next` 和 `react-i18next` 依赖已安装，但无翻译文件、无 provider、无 `useTranslation` 调用。

**需要**:
1. 创建 `frontend/src/locales/zh-CN.json` 和 `frontend/src/locales/en.json`
2. 创建 `frontend/src/lib/i18n.ts` 初始化配置
3. 在 `layout.tsx` 中包裹 `I18nextProvider`
4. 逐步替换硬编码文本为 `t('key')` 调用

---

## 5. 缺失服务实现

### 5.1 速率限制

**方案**: 基于 IP + 用户 ID 的双维度限速

- 未认证请求: 30 req/min (IP 维度)
- 认证请求: 300 req/min (用户维度)  
- API Key 请求: 基于 Plan 配置 (1000-100000/月)

**实现**: 使用 `governor` crate (Rust 生态中成熟的限速库)，数据存储使用内存 (单实例) 或 Redis (多实例)。

### 5.2 邮件发送

密码重置、邮箱验证、成员邀请都需要发邮件。

**方案**: 
- 开发环境: `lettre` crate 连接本地 SMTP 或打印到日志
- 生产环境: `lettre` crate 连接 SMTP 服务 (如 AWS SES, SendGrid)
- 接口: 定义 `Mailer` trait，实现可替换

### 5.3 健康检查

```rust
// GET /health
pub async fn health_check(state: web::Data<AppState>) -> impl Responder {
    let db_ok = state.db.acquire().await.is_ok();
    let status = if db_ok { "healthy" } else { "degraded" };
    
    HttpResponse::Ok().json(serde_json::json!({
        "status": status,
        "version": env!("CARGO_PKG_VERSION"),
        "database": db_ok,
        "uptime_seconds": state.start_time.elapsed().as_secs(),
    }))
}
```

### 5.4 沙箱执行器 (Phase 2)

**建议方案**: 先实现 Docker 容器沙箱，后续考虑 WASM

```
用户提交代码 → 创建临时容器 → 挂载代码 → 限制 CPU/内存/网络 → 
执行 → 收集 stdout/stderr → 销毁容器 → 返回结果
```

资源限制:
- CPU: 0.5 核
- 内存: 256MB
- 执行时间: 30s (可配置)
- 网络: 默认禁用
- 文件系统: 只读 + /tmp

---

## 6. service-payment 修复

**发现**: `service-payment` crate 存在编译错误

- `async_stripe` crate 未解析 (可能未正确添加到 Cargo.toml 或版本不匹配)
- `ResourceType` enum 缺少 `Hash` derive

**修复**:
1. 确认 `async-stripe` 在 `service-payment/Cargo.toml` 中正确声明
2. 为 `ResourceType` 添加 `#[derive(Hash)]`

---

## 7. 可观测性

### 7.1 结构化日志

当前: `tracing_subscriber::fmt::layer()` — 纯文本日志

目标:
- 开发环境: 人类可读格式 (当前)
- 生产环境: JSON 格式，便于日志聚合

```rust
match config.environment.as_str() {
    "production" => registry.with(fmt::layer().json()),
    _ => registry.with(fmt::layer().pretty()),
}
```

### 7.2 指标收集

使用 `actix-web-prom` 或 `metrics` crate 暴露 Prometheus 指标:
- 请求计数 (by method, path, status)
- 请求延迟直方图
- 活跃连接数
- 数据库查询延迟

### 7.3 请求追踪

当前已有 `X-Request-ID` header 生成 (前端 `client.ts`)。后端需要:
1. 接收前端的 `X-Request-ID`，如果没有则自动生成
2. 在所有日志中包含 request_id
3. 在响应中回传 `X-Request-ID`

---

## 8. 部署架构

### 8.1 开发环境

```
docker-compose up → PostgreSQL + Redis + MinIO
cargo run          → 后端 (连接 Docker 服务)
npm run dev        → 前端 (代理到后端)
```

### 8.2 生产环境

```
                    ┌─────────────────────┐
                    │   CDN (CloudFlare)  │
                    └──────────┬──────────┘
                               │
                    ┌──────────┴──────────┐
                    │   Load Balancer     │
                    │   (Nginx / ALB)     │
                    └──────────┬──────────┘
                               │
              ┌────────────────┼────────────────┐
              │                │                │
     ┌────────┴────────┐ ┌────┴────┐  ┌────────┴────────┐
     │ Backend (Rust)  │ │ Backend │  │ Frontend (Node) │
     │ :8080           │ │ :8080   │  │ :3000           │
     └────────┬────────┘ └────┬────┘  └────────┬────────┘
              │               │                │
              └───────────────┼────────────────┘
                              │
         ┌────────────────────┼────────────────────┐
         │                    │                    │
    ┌────┴─────┐     ┌───────┴───────┐     ┌─────┴──────┐
    │PostgreSQL│     │    Redis      │     │   MinIO    │
    │ (主从)   │     │   (Sentinel)  │     │  (集群)    │
    └──────────┘     └───────────────┘     └────────────┘
```

### 8.3 环境变量补充

| 变量 | 说明 | 示例 |
|------|------|------|
| `CORS__ALLOWED_ORIGIN` | 允许的前端域名 | `https://app.evolith.io` |
| `SMTP__HOST` | 邮件服务器 | `smtp.sendgrid.net` |
| `SMTP__PORT` | 邮件端口 | `587` |
| `SMTP__USERNAME` | 邮件用户名 | `apikey` |
| `SMTP__PASSWORD` | 邮件密码 | `SG.xxxxx` |
| `SMTP__FROM` | 发件人地址 | `noreply@evolith.io` |
| `REDIS__URL` | Redis 连接串 | `redis://localhost:6379` |
| `STORAGE__ENDPOINT` | MinIO/S3 端点 | `http://localhost:9000` |
| `STORAGE__ACCESS_KEY` | 存储访问密钥 | `minio_access_key` |
| `STORAGE__SECRET_KEY` | 存储密钥 | `minio_secret_key` |
| `STRIPE__SECRET_KEY` | Stripe 密钥 | `sk_live_xxxxx` |
| `STRIPE__WEBHOOK_SECRET` | Stripe Webhook 密钥 | `whsec_xxxxx` |
| `RATE_LIMIT__REQUESTS_PER_MINUTE` | 速率限制 | `300` |

---

## 9. 与现有架构文档的关系

本文档**替代** `docs/architecture.md` 中的以下部分:
- 第 5 节 (数据模型) — 消除重复定义
- 第 6 节 (安全设计) — 补充实际实现方案
- 第 10 节 (环境配置) — 补充缺失变量

`docs/architecture.md` 应更新为:
1. 删除重复的第 5.1 节 (行 394-453 的非多租户版本)
2. 删除重复的 SQL schema (行 567-633 的非多租户版本)
3. 在第 6 节引用本文档的安全方案
4. 更新第 10 节的环境变量表
