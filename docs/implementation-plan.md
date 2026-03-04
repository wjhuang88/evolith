# 生产级实现计划

## 概述

本文档定义了将 Evolith 从 MVP 提升到生产级 SaaS 系统所需的所有功能实现。

## 当前状态

| 阶段 | 功能 | 状态 |
|------|------|------|
| P1 | 核心框架（配置、错误处理、日志、Auth 中间件） | ✅ 完成 |
| P2 | 业务逻辑（用户认证、Tool/Skill/Snippet CRUD、MCP） | ✅ 完成 |
| P3 | 核心功能（参数验证、Skill 执行、片段引用） | ✅ 完成 |
| P4 | 前端集成（UI 组件、页面） | ✅ 完成 |
| P5 | 多租户 & i18n | ✅ 完成 |
| P6 | 用户与权限系统 | 🔄 进行中 |
| P7 | 套餐与计费系统 | ⏳ 待开始 |

## 最新进展 (2026-03-04)

### 架构审查完成 ✅

已完成对后端代码库的全面架构审查，发现以下主要问题：

**关键发现:**
- 81+ `.unwrap()`/`.expect()` 潜在panic点
- 739行 auth_handlers.rs 混合业务逻辑
- Cache模块是占位符，未实现
- SandboxedExecutor 未实现 (todo!())
- Handler文件过大需拆分
- 重复CRUD模式可抽取

**详细分析见: 架构重构计划章节**

---

## 实现计划

### Phase 6: 用户与权限系统

#### 6.1 数据库迁移

```sql
-- 004_user_permissions.sql
-- 邮箱验证
ALTER TABLE users ADD COLUMN verify_token TEXT;
ALTER TABLE users ADD COLUMN verified_at TEXT;

-- 密码重置
ALTER TABLE users ADD COLUMN reset_token TEXT;
ALTER TABLE users ADD COLUMN reset_expires_at TEXT;

-- API Keys 表
CREATE TABLE IF NOT EXISTS api_keys (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id),
    user_id TEXT NOT NULL REFERENCES users(id),
    name TEXT NOT NULL,
    key_hash TEXT NOT NULL,
    key_prefix TEXT NOT NULL,
    permissions TEXT DEFAULT '["read"]',
    expires_at TEXT,
    last_used_at TEXT,
    rate_limit INTEGER DEFAULT 1000,
    status TEXT DEFAULT 'active',
    request_count INTEGER DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- 审计日志表
CREATE TABLE IF NOT EXISTS audit_logs (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id),
    user_id TEXT REFERENCES users(id),
    action TEXT NOT NULL,
    resource_type TEXT,
    resource_id TEXT,
    details TEXT DEFAULT '{}',
    ip_address TEXT,
    user_agent TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_api_keys_tenant ON api_keys(tenant_id);
CREATE INDEX IF NOT EXISTS idx_api_keys_prefix ON api_keys(key_prefix);
CREATE INDEX IF NOT EXISTS idx_audit_tenant ON audit_logs(tenant_id, created_at);
```

#### 6.2 后端 API 实现

| 功能 | 端点 | 方法 |
|------|------|------|
| 发送验证邮件 | `/api/v1/auth/send-verify` | POST |
| 验证邮箱 | `/api/v1/auth/verify-email` | POST |
| 请求密码重置 | `/api/v1/auth/forgot-password` | POST |
| 重置密码 | `/api/v1/auth/reset-password` | POST |
| 刷新 Token | `/api/v1/auth/refresh` | POST |
| 邀请成员 | `/api/v1/tenant/members/invite` | POST |
| 接受邀请 | `/api/v1/auth/join` | POST |
| 成员列表 | `/api/v1/tenant/members` | GET |
| 移除成员 | `/api/v1/tenant/members/{id}` | DELETE |
| 变更角色 | `/api/v1/tenant/members/{id}/role` | PATCH |
| 创建 API Key | `/api/v1/tenant/api-keys` | POST |
| 列出 API Keys | `/api/v1/tenant/api-keys` | GET |
| 撤销 API Key | `/api/v1/tenant/api-keys/{id}` | DELETE |
| 审计日志 | `/api/v1/tenant/audit-logs` | GET |

#### 6.3 前端页面

- `/login` - 登录（含忘记密码入口）
- `/register` - 注册
- `/forgot-password` - 忘记密码
- `/reset-password` - 重置密码
- `/tenant/members` - 成员管理
- `/tenant/api-keys` - API Key 管理
- `/tenant/settings` - 租户设置

---

### Phase 7: 套餐与计费系统

#### 7.1 数据库迁移

```sql
-- 005_billing.sql

-- 套餐表
CREATE TABLE IF NOT EXISTS plans (
    id TEXT PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    display_name TEXT NOT NULL,
    description TEXT,
    monthly_price REAL DEFAULT 0,
    yearly_price REAL,
    price_per_user REAL,
    max_users INTEGER DEFAULT 3,
    max_tools INTEGER DEFAULT 5,
    max_skills INTEGER DEFAULT 10,
    max_snippets INTEGER DEFAULT 50,
    max_api_calls_per_month INTEGER DEFAULT 1000,
    max_storage_mb INTEGER DEFAULT 100,
    features TEXT DEFAULT '{}',
    is_active BOOLEAN DEFAULT 1,
    is_builtin BOOLEAN DEFAULT 1,
    sort_order INTEGER DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- 订阅表
CREATE TABLE IF NOT EXISTS subscriptions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id),
    plan_id TEXT NOT NULL REFERENCES plans(id),
    status TEXT DEFAULT 'trialing',
    billing_cycle TEXT DEFAULT 'monthly',
    current_period_start TEXT NOT NULL,
    current_period_end TEXT NOT NULL,
    trial_start_at TEXT,
    trial_end_at TEXT,
    canceled_at TEXT,
    cancellation_reason TEXT,
    payment_method_id TEXT,
    last_payment_at TEXT,
    next_payment_at TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- 支付记录表
CREATE TABLE IF NOT EXISTS payments (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id),
    subscription_id TEXT REFERENCES subscriptions(id),
    amount REAL NOT NULL,
    currency TEXT DEFAULT 'CNY',
    payment_method TEXT,
    provider_payment_id TEXT,
    provider_response TEXT,
    status TEXT DEFAULT 'pending',
    invoice_id TEXT,
    invoice_url TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- 发票表
CREATE TABLE IF NOT EXISTS invoices (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id),
    payment_id TEXT REFERENCES payments(id),
    invoice_number TEXT UNIQUE NOT NULL,
    status TEXT DEFAULT 'draft',
    subtotal REAL NOT NULL,
    tax REAL DEFAULT 0,
    total REAL NOT NULL,
    currency TEXT DEFAULT 'CNY',
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    tax_rate REAL DEFAULT 0,
    tax_number TEXT,
    billing_address TEXT,
    pdf_url TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    issued_at TEXT,
    paid_at TEXT
);

-- 使用量表
CREATE TABLE IF NOT EXISTS usage_records (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id),
    subscription_id TEXT REFERENCES subscriptions(id),
    resource_type TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    unit_price REAL,
    overage_amount REAL,
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- 每日汇总表
CREATE TABLE IF NOT EXISTS daily_usage (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id),
    resource_type TEXT NOT NULL,
    date TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    UNIQUE(tenant_id, resource_type, date)
);

-- Webhook 事件表
CREATE TABLE IF NOT EXISTS webhook_events (
    id TEXT PRIMARY KEY,
    event_type TEXT NOT NULL,
    provider TEXT NOT NULL,
    provider_event_id TEXT NOT NULL,
    payload TEXT NOT NULL,
    status TEXT DEFAULT 'pending',
    attempts INTEGER DEFAULT 0,
    last_error TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    processed_at TEXT
);

-- 插入内置套餐数据
INSERT INTO plans (id, name, display_name, monthly_price, yearly_price, max_users, max_tools, max_skills, max_snippets, max_api_calls_per_month, features, sort_order) VALUES
    ('plan-free', 'free', '免费版', 0, 0, 3, 5, 10, 50, 1000, '{"custom_domain": false, "sso": false, "priority_support": false, "audit_logs": false}', 1),
    ('plan-starter', 'starter', '基础版', 29, 290, 10, 20, 50, 200, 10000, '{"custom_domain": false, "sso": false, "priority_support": false, "audit_logs": true}', 2),
    ('plan-pro', 'pro', '专业版', 99, 990, 50, 100, 200, 1000, 100000, '{"custom_domain": true, "sso": false, "priority_support": true, "audit_logs": true}', 3),
    ('plan-enterprise', 'enterprise', '企业版', NULL, NULL, -1, -1, -1, -1, -1, '{"custom_domain": true, "sso": true, "priority_support": true, "audit_logs": true, "dedicated_support": true, "sla": "99.99%"}', 4);

CREATE INDEX IF NOT EXISTS idx_subscriptions_tenant ON subscriptions(tenant_id);
CREATE INDEX IF NOT EXISTS idx_subscriptions_status ON subscriptions(status);
CREATE INDEX IF NOT EXISTS idx_payments_tenant ON payments(tenant_id);
CREATE INDEX IF NOT EXISTS idx_usage_tenant_period ON usage_records(tenant_id, period_start);
CREATE INDEX IF NOT EXISTS idx_daily_usage ON daily_usage(tenant_id, date);
```

#### 7.2 后端 API 实现

| 功能 | 端点 | 方法 |
|------|------|------|
| 套餐列表 | `/api/v1/tenant/plans` | GET |
| 当前订阅 | `/api/v1/tenant/subscription` | GET |
| 创建订阅 | `/api/v1/tenant/subscription` | POST |
| 更新订阅 | `/api/v1/tenant/subscription` | PATCH |
| 取消订阅 | `/api/v1/tenant/subscription` | DELETE |
| 发票列表 | `/api/v1/tenant/invoices` | GET |
| 使用量统计 | `/api/v1/tenant/usage` | GET |

#### 7.3 前端页面

- `/tenant/billing/plans` - 套餐选择
- `/tenant/billing` - 账单管理（含发票）
- `/tenant/usage` - 使用量仪表盘

---

### Phase 8: 生产级改进

#### 8.1 安全加固

- 实现速率限制（基于 IP 和账户）
- 添加请求 ID 追踪
- 完善错误消息脱敏
- 实现 CSRF 保护

#### 8.2 性能优化

- 添加 Redis 缓存层
- 实现数据库连接池优化
- 添加请求日志慢查询检测

#### 8.3 监控与告警

- 添加健康检查端点
- 实现指标收集
- 添加关键错误告警

#### 8.4 部署配置

- Docker Compose 配置
- Nginx 配置
- 环境变量文档

---

## 实施顺序

1. **Week 1**: P6 - 用户与权限系统
   - 数据库迁移
   - 后端 API
   - 前端页面

2. **Week 2**: P7 - 套餐与计费系统
   - 数据库迁移
   - 后端 API
   - 前端页面

3. **Week 3**: 生产级改进
   - 安全加固
   - 性能优化
   - 监控告警

4. **Week 4**: 测试与部署
   - 集成测试
   - 文档完善
   - 部署配置

---

## 验收标准

### 功能验收

- [ ] 所有 P0 功能正常工作
- [ ] 通过功能测试用例
- [ ] 无阻塞性 Bug

### 性能验收

- [ ] API 响应时间 < 100ms (P95)
- [ ] 并发用户支持 1000+

### 安全验收

- [ ] 通过安全扫描
- [ ] 无高危漏洞
- [ ] 审计日志完整

### 文档验收

- [ ] API 文档完整
- [ ] 部署文档完整
- [ ] 用户手册完整

---

## 架构重构计划 (Technical Debt & Refactoring)

基于代码审查，发现以下需要重构的问题。优先级：**P0=紧急, P1=高, P2=中**

### R1: 提取通用CRUD服务 [P1]

**问题**: `handlers/tool_handlers.rs`、`handlers/skill_handlers.rs`、`handlers/snippet_handlers.rs` 存在大量重复的CRUD模式。

**建议**:
- 创建通用 `CrudService<T>` trait
- 抽取 `create`, `update`, `delete`, `list`, `get_by_id` 通用实现
- 位置: `backend/crates/service-common/src/crud.rs`

**工作量**: 2-3天

---

### R2: 业务逻辑从Handler抽取到Service层 [P0]

**问题**: `handlers/auth_handlers.rs` (739行) 包含业务逻辑和内存存储，不符合分层架构。

**当前问题**:
```rust
// auth_handlers.rs - 业务逻辑在Handler层
struct InMemoryUserStore { ... }  // 不应该在handlers中

impl InMemoryUserStore {
    fn generate_token(&self, ...) -> ... { ... }  // 应该移到service层
}
```

**建议**:
- 创建 `service-auth` crate 的业务逻辑模块
- 将 token 生成、用户存储移至 service 层
- Handler 只负责请求/响应转换

**工作量**: 3-5天

---

### R3: 实现缓存层 [P0]

**问题**: `infra/src/cache.rs` 是占位符，未实现实际缓存。

**建议**:
```rust
// 实现 RedisCache
pub struct RedisCache {
    client: redis::Client,
}

impl Cache for RedisCache {
    async fn get(&self, key: &str) -> Option<String> { ... }
    async fn set(&self, key: &str, value: &str, ttl: Duration) { ... }
    async fn delete(&self, key: &str) { ... }
}
```

**优先级**: 
- API Key 缓存 (高)
- Skill/Snippet 结果缓存 (中)
- 用户会话缓存 (中)

**工作量**: 2-3天

---

### R4: 实现沙箱执行器 [P0]

**问题**: `service-skill/src/executor.rs` 的 `SandboxedExecutor::execute()` 是 `todo!()`。

**建议**:
- 使用 WebAssembly (wasmtime) 或 firecracker
- 实现资源限制 (CPU/内存/执行时间)
- 添加超时控制

**工作量**: 5-7天

---

### R5: Handler文件拆分 [P1]

**问题**: 某些Handler文件过大。

| 文件 | 行数 | 建议拆分 |
|------|------|----------|
| auth_handlers.rs | 739 | auth_service + token_service |
| member_handlers.rs | 563 | member_service + invitation_service |

**建议**:
```
api/src/handlers/
├── auth/
│   ├── mod.rs
│   ├── login.rs
│   ├── register.rs
│   └── password.rs
├── members/
│   ├── mod.rs
│   ├── list.rs
│   ├── invite.rs
│   └── role.rs
```

**工作量**: 2-3天

---

### R6: 消除unwrap()风险 [P1]

**问题**: 全项目81+处 `.unwrap()` 和 `.expect()`。

**当前风险示例**:
```rust
// user_repo.rs
let user = sqlx::query_as(...)
    .fetch_one(&pool)
    .await
    .unwrap();  // 可能panic
```

**建议**:
- 使用 `?` 运算符传播错误
- 添加 `Result` 类型别名
- 在关键路径使用 `ok_or()` + 错误信息

**工作量**: 1-2天

---

### R7: 添加分页支持 [P2]

**问题**: 所有 `find_all` 端点无分页，大数据量时性能问题。

**建议**:
```rust
// 统一分页参数
#[derive(Deserialize)]
pub struct Pagination {
    #[serde(default = "default_limit")]
    limit: u32,
    #[serde(default)]
    offset: u32,
}

// 响应中添加分页元数据
pub struct PaginatedResponse<T> {
    data: Vec<T>,
    pagination: PaginationMeta,
}
```

**工作量**: 1-2天

---

### R8: 移除空抽象层 [P2]

**问题**: `service-snippet/src/repository.rs` 只有 `todo!()`。

**建议**:
- 如果不需要，删除该模块
- 如果需要，实现或委托给 domain trait

**工作量**: 0.5天

---

### R9: 统一错误处理 [P2]

**问题**: 部分handler返回不一致的错误格式。

**建议**:
- 确保所有错误通过 `ApiResponse<T>::error()` 返回
- 添加全局错误中间件统一处理

**工作量**: 0.5天

---

### R10: 配置外部化 [P2]

**问题**: 硬编码值散落各处。

**当前问题**:
```rust
// service-tool/src/mcp.rs
const TOOL_TIMEOUT: u64 = 30;  // 硬编码

// service-skill/src/sandbox.rs
const MAX_MEMORY_MB: u64 = 128;  // 硬编码
```

**建议**:
- 抽取到 `infra/src/config.rs`
- 支持环境变量覆盖

**工作量**: 1天

---

### 重构优先级矩阵

| 优先级 | 重构项 | 影响 | 工作量 |
|--------|--------|------|--------|
| P0 | R2: 业务逻辑抽取 | 架构 | 3-5天 |
| P0 | R3: 缓存实现 | 性能 | 2-3天 |
| P0 | R4: 沙箱执行器 | 功能 | 5-7天 |
| P1 | R1: 通用CRUD | 可维护性 | 2-3天 |
| P1 | R5: Handler拆分 | 可维护性 | 2-3天 |
| P1 | R6: 消除unwrap | 稳定性 | 1-2天 |
| P2 | R7: 分页支持 | 性能 | 1-2天 |
| P2 | R8: 空抽象移除 | 清洁度 | 0.5天 |
| P2 | R9: 错误处理统一 | 一致性 | 0.5天 |
| P2 | R10: 配置外部化 | 可维护性 | 1天 |

**总计**: ~18-24天

---

### 重构与功能开发并行计划

建议在完成当前Phase 6后，优先完成P0重构再开始Phase 7：

1. **Week 1-2**: 完成P6收尾 (RBAC中间件集成)
2. **Week 3**: P0重构 (R2, R3, R4) - 业务逻辑、缓存、沙箱
3. **Week 4**: P1重构 (R1, R5, R6) - CRUD、拆分、错误处理
4. **Week 5**: P7功能开发 (套餐与计费)
5. **Week 6**: P2重构 + P7收尾