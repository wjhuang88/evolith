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

## 最新进展 (2026-03-02)

### MCP 端点实现 ✅

已完成跨租户 MCP 端点实现：

**架构设计:**
- 统一端点: `/mcp` (POST)
- 认证方式: `Authorization: Bearer <key>` 或 `X-API-Key: <key>`
- 工具过滤: 根据 API Key 关联的 tenant_id 返回不同工具列表
- 公开工具: 无需认证即可访问 (is_public=true)
- 私有工具: 需要有效 API Key 且 tenant_id 匹配

**已验证功能:**
- `initialize` - 返回服务器能力
- `tools/list` - 返回工具列表（无认证返回公开工具）
- `tools/call` - 执行工具调用
- 权限检查 - 私有工具拒绝无认证访问

**关键文件:**
- `backend/crates/api/src/handlers/mcp_handlers.rs` - MCP 处理器
- `backend/crates/api/src/handlers/tool_handlers.rs` - ToolStore 和 StoredTool
- `backend/crates/api/src/routes/mcp.rs` - 路由配置
- `backend/src/main.rs` - McpState 初始化

**技术决策:**
- MCP 使用工具名称而非 UUID 进行查找
- API Key 同时支持 Bearer token 和 X-API-Key header
- ToolStore 使用内存存储（开发模式）

**测试命令:**
```bash
# 健康检查
curl -s http://localhost:8080/health

# MCP initialize
curl -s -X POST http://localhost:8080/mcp \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}'

# MCP tools/list (无认证)
curl -s -X POST http://localhost:8080/mcp \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}'

# MCP tools/call
curl -s -X POST http://localhost:8080/mcp \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"filesystem_read","arguments":{"path":"/tmp/test.txt"}}}'
```

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

-- API Keys 表（扩展自文档）
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