# P6-001: 用户注册与自动租户创建实施计划

> **Goal:** 实现完整的用户注册流程，包括自动创建租户、邮箱验证

**Architecture:** 
- 后端使用 Repository 模式连接 SQLite 数据库
- 注册时自动创建租户，用户设为 owner
- 支持邮箱验证流程
- 前端使用 Next.js + React Hook Form

**Tech Stack:** Rust + Actix-web, SQLx, SQLite, Next.js 14, TypeScript, Tailwind

---

## Task 1: 扩展 UserRepository trait

**Files:**
- Modify: `backend/crates/domain/src/repository.rs`
- Modify: `backend/crates/domain/src/user.rs`

**Step 1: 添加 email_verified 字段到 User 模型**

在 `user.rs` 的 User 结构体中添加：
```rust
pub email_verified: bool,
pub verify_token: Option<String>,
pub verified_at: Option<DateTime<Utc>>,
pub reset_token: Option<String>,
pub reset_expires_at: Option<DateTime<Utc>>,
```

**Step 2: 扩展 UserRepository trait**

在 `repository.rs` 中添加新方法：
```rust
async fn find_by_verify_token(&self, token: &str) -> Result<Option<User>>;
async fn verify_email(&self, id: Uuid) -> Result<()>;
async fn find_by_reset_token(&self, token: &str) -> Result<Option<User>>;
async fn set_reset_token(&self, id: Uuid, token: &str, expires_at: DateTime<Utc>) -> Result<()>;
async fn update_password(&self, id: Uuid, password_hash: &str) -> Result<()>;
```

**Step 3: 运行 cargo check 验证**

---

## Task 2: 创建 SqliteUserRepository

**Files:**
- Create: `backend/crates/infra/src/db/user_repo.rs`

**Step 1: 创建文件并实现 UserRepository**

```rust
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use common::error::{AppError, Result};
use domain::repository::UserRepository;
use domain::user::{NewUser, UpdateUser, User, UserRole, TenantRole};
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct SqliteUserRepository {
    pool: SqlitePool,
}

impl SqliteUserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for SqliteUserRepository {
    // 实现所有方法...
}
```

**Step 2: 更新 infra/src/db/mod.rs 导出**

---

## Task 3: 创建 SqliteTenantRepository

**Files:**
- Create: `backend/crates/infra/src/db/tenant_repo.rs`

**Step 1: 实现 TenantRepository**

类似 Task 2，实现所有 trait 方法。

**Step 2: 更新 mod.rs 导出**

---

## Task 4: 更新 auth_handlers 使用数据库

**Files:**
- Modify: `backend/crates/api/src/handlers/auth_handlers.rs`

**Step 1: 更新 AuthState 结构体**

```rust
pub struct AuthState {
    pub jwt: JwtHandler,
    pub user_repo: Arc<dyn UserRepository>,
    pub tenant_repo: Arc<dyn TenantRepository>,
    pub smtp_host: Option<String>,
    pub smtp_from: Option<String>,
}
```

**Step 2: 更新 register handler**

使用数据库事务：
1. 创建租户 (如果提供了 slug)
2. 创建用户
3. 生成验证 token
4. 提交事务

**Step 3: 更新 login handler**

从数据库查询用户。

**Step 4: 更新 verify_email handler**

使用数据库查询 token。

---

## Task 5: 更新 main.rs 初始化

**Files:**
- Modify: `backend/src/main.rs`

**Step 1: 创建数据库连接池**

**Step 2: 初始化 repositories**

**Step 3: 注入到 AuthState**

---

## Task 6: 前端注册页面

**Files:**
- Create: `frontend/src/app/(auth)/register/page.tsx`

**Step 1: 创建注册表单组件**

使用 React Hook Form + zod 验证：
- email (必填，邮箱格式)
- username (必填，3-50字符)
- password (必填，8-128字符)
- tenant_name (可选)
- tenant_slug (可选)

**Step 2: 集成 API 调用**

调用 `POST /api/v1/auth/register`

**Step 3: 添加样式和错误处理**

---

## Task 7: 前端邮箱验证页面

**Files:**
- Create: `frontend/src/app/(auth)/verify-email/page.tsx`

**Step 1: 从 URL 读取 token 参数**

**Step 2: 调用验证 API**

`POST /api/v1/auth/verify-email`

**Step 3: 显示成功/失败状态**

---

## Task 8: 测试验证

**Step 1: 构建后端**

```bash
cd backend && cargo build
```

**Step 2: 启动服务**

```bash
./scripts/dev.sh
```

**Step 3: 测试注册流程**

```bash
# 注册新用户
curl -X POST http://localhost:8080/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "username": "testuser",
    "password": "password123",
    "tenant_slug": "test-tenant"
  }'

# 验证邮箱
curl -X POST http://localhost:8080/api/v1/auth/verify-email \
  -H "Content-Type: application/json" \
  -d '{"token": "<verification_token>"}'

# 登录
curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "password123"
  }'
```

---

## 依赖关系

```
Task 1 (扩展 trait)
    ↓
Task 2 (UserRepository) → Task 3 (TenantRepository)
    ↓
Task 4 (更新 handlers) ← Task 5 (更新 main.rs)
    ↓
Task 8 (测试)

Task 6 (前端注册页) / Task 7 (前端验证页) 可以并行
```

## 验收标准

- [ ] 用户可以通过 API 注册
- [ ] 注册时自动创建租户（如果提供 slug）
- [ ] 用户收到邮箱验证 token
- [ ] 验证邮箱后可以登录
- [ ] 前端注册页面正常工作
- [ ] 前端验证页面正常工作
