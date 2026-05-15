# 多租户设计

## 1. 概述

Evolith 是一个 SaaS 平台，需要支持多租户（Multi-Tenancy）架构，允许多个组织/团队独立使用同一套系统，数据相互隔离。

## 2. 租户隔离策略

### 2.1 方案选择

| 方案 | 优点 | 缺点 | 适用场景 |
|------|------|------|----------|
| 独立数据库 | 最高隔离性、合规友好 | 成本高、维护复杂 | 大型企业、金融 |
| 共享数据库 + Schema 隔离 | 较高隔离性、备份方便 | 迁移复杂、连接池管理难 | 中型企业 |
| **共享数据库 + 租户ID** | 成本低、维护简单、扩展性好 | 隔离性一般、需要严格过滤 | **SaaS 平台（推荐）** |

**选择方案**：共享数据库 + 租户ID字段（Row-Level Security）

### 2.2 隔离实现

```
┌─────────────────────────────────────────────────────────────┐
│                    Shared Database                           │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌──────────────────────────────────────────────────────┐   │
│  │                    tenants 表                         │   │
│  │  tenant_1 (Acme Corp)                                │   │
│  │  tenant_2 (Startup Inc)                              │   │
│  │  tenant_3 (Dev Team)                                 │   │
│  └──────────────────────────────────────────────────────┘   │
│                           │                                  │
│                           ▼                                  │
│  ┌──────────────────────────────────────────────────────┐   │
│  │              业务表 (带 tenant_id)                     │   │
│  │                                                       │   │
│  │  tools:      tenant_id → 工具数据                     │   │
│  │  skills:     tenant_id → 技能数据                     │   │
│  │  snippets:   tenant_id → 片段数据                     │   │
│  │  users:      tenant_id → 用户数据                     │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

## 3. 租户识别

### 3.1 识别方式

支持两种租户识别方式：

#### 方式一：子域名（推荐）

```
https://acme.evolith.io     → tenant_id = acme
https://startup.evolith.io  → tenant_id = startup
https://app.evolith.io      → 默认租户或租户选择页
```

**优点**：
- 用户友好，品牌独立
- 浏览器自动隔离 Cookie
- 易于配置自定义域名

#### 方式二：请求头（API 调用）

```
X-Tenant-ID: acme
X-Tenant-Slug: acme
```

**优点**：
- 适合 API 集成
- 适合 CLI 工具
- 调试方便

### 3.2 识别优先级

```
1. 子域名 (host 解析)
2. X-Tenant-ID Header
3. JWT Token 中的 tenant_id
4. 默认租户（仅限开发环境）
```

### 3.3 租户中间件流程

```
┌──────────────┐
│   Request    │
└──────┬───────┘
       │
       ▼
┌──────────────────────────────────────┐
│         Tenant Middleware            │
├──────────────────────────────────────┤
│                                      │
│  1. 从 Host 提取子域名                │
│     acme.evolith.io → "acme"         │
│                                      │
│  2. 查询租户缓存                       │
│     Cache: slug → tenant_id          │
│                                      │
│  3. 缓存未命中？查询数据库              │
│     DB: SELECT * FROM tenants        │
│         WHERE slug = 'acme'          │
│                                      │
│  4. 验证租户状态                       │
│     - 是否存在？                       │
│     - 是否激活？                       │
│     - 是否过期？                       │
│                                      │
│  5. 注入租户上下文                     │
│     request.tenant = {               │
│       id: "uuid",                    │
│       slug: "acme",                  │
│       name: "Acme Corp",             │
│       plan: "pro"                    │
│     }                                │
│                                      │
└──────────────┬───────────────────────┘
               │
               ▼
┌──────────────────────────────────────┐
│           Route Handler              │
│   自动使用 request.tenant.id          │
└──────────────────────────────────────┘
```

## 4. 数据模型

### 4.1 租户表

```sql
CREATE TABLE tenants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- 基本信息
    name VARCHAR(128) NOT NULL,              -- 租户名称
    slug VARCHAR(64) UNIQUE NOT NULL,        -- URL 标识 (acme-corp)
    domain VARCHAR(255) UNIQUE,              -- 自定义域名 (app.acme.com)
    
    -- 联系信息
    owner_id UUID NOT NULL,                  -- 创建者/所有者
    billing_email VARCHAR(255),              -- 账单邮箱
    
    -- 订阅计划
    plan VARCHAR(32) NOT NULL DEFAULT 'free', -- free | starter | pro | enterprise
    plan_status VARCHAR(32) DEFAULT 'active', -- active | past_due | cancelled
    
    -- 配额限制
    max_users INTEGER DEFAULT 5,
    max_tools INTEGER DEFAULT 10,
    max_skills INTEGER DEFAULT 20,
    max_snippets INTEGER DEFAULT 100,
    max_api_calls_per_month INTEGER DEFAULT 10000,
    max_storage_mb INTEGER DEFAULT 500,
    
    -- 使用统计
    current_users INTEGER DEFAULT 0,
    current_tools INTEGER DEFAULT 0,
    current_skills INTEGER DEFAULT 0,
    current_snippets INTEGER DEFAULT 0,
    current_api_calls INTEGER DEFAULT 0,
    current_storage_mb INTEGER DEFAULT 0,
    
    -- 配置
    settings JSONB DEFAULT '{}',             -- 租户级配置
    features JSONB DEFAULT '{}',             -- 启用的功能特性
    
    -- 状态
    status VARCHAR(32) DEFAULT 'active',     -- active | suspended | deleted
    
    -- 时间戳
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,
    
    -- 索引
    CONSTRAINT fk_owner FOREIGN KEY (owner_id) REFERENCES users(id)
);

CREATE INDEX idx_tenants_slug ON tenants(slug);
CREATE INDEX idx_tenants_domain ON tenants(domain);
CREATE INDEX idx_tenants_status ON tenants(status);
```

### 4.2 修改用户表

```sql
ALTER TABLE users ADD COLUMN tenant_id UUID NOT NULL REFERENCES tenants(id);
ALTER TABLE users ADD COLUMN role VARCHAR(32) DEFAULT 'member'; -- owner | admin | member

CREATE INDEX idx_users_tenant ON users(tenant_id);
```

### 4.3 修改业务表

```sql
-- 工具表
ALTER TABLE tools ADD COLUMN tenant_id UUID NOT NULL REFERENCES tenants(id);
CREATE INDEX idx_tools_tenant ON tools(tenant_id);

-- 技能表
ALTER TABLE skills ADD COLUMN tenant_id UUID NOT NULL REFERENCES tenants(id);
CREATE INDEX idx_skills_tenant ON skills(tenant_id);

-- 代码片段表
ALTER TABLE snippets ADD COLUMN tenant_id UUID NOT NULL REFERENCES tenants(id);
CREATE INDEX idx_snippets_tenant ON snippets(tenant_id);
```

### 4.4 租户邀请表

```sql
CREATE TABLE tenant_invitations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    email VARCHAR(255) NOT NULL,
    role VARCHAR(32) NOT NULL DEFAULT 'member',
    token VARCHAR(64) UNIQUE NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    accepted_at TIMESTAMPTZ,
    created_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(tenant_id, email)
);
```

### 4.5 租户使用日志

```sql
CREATE TABLE tenant_usage_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    resource_type VARCHAR(32) NOT NULL,  -- tool_call | skill_exec | api_call
    resource_id UUID,
    action VARCHAR(64) NOT NULL,
    details JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_usage_logs_tenant_date ON tenant_usage_logs(tenant_id, created_at);
```

## 5. API 变更

### 5.1 认证 API

#### 注册（创建租户）

```typescript
POST /api/v1/auth/register

Request:
{
  // 租户信息
  "tenant_name": "Acme Corporation",
  "tenant_slug": "acme-corp",       // 将成为 acme-corp.evolith.io
  
  // 用户信息
  "username": "john",
  "email": "john@acme.com",
  "password": "SecurePass123!"
}

Response:
{
  "success": true,
  "data": {
    "tenant": {
      "id": "uuid",
      "name": "Acme Corporation",
      "slug": "acme-corp"
    },
    "user": {
      "id": "uuid",
      "email": "john@acme.com",
      "role": "owner"
    },
    "token": "jwt_token"
  }
}
```

#### 登录

```typescript
POST /api/v1/auth/login

// 登录时自动识别租户
// 从 Host header 或 X-Tenant-ID 获取

Request:
{
  "email": "john@acme.com",
  "password": "SecurePass123!"
}

Response:
{
  "success": true,
  "data": {
    "tenant": {
      "id": "uuid",
      "name": "Acme Corporation",
      "slug": "acme-corp",
      "plan": "pro"
    },
    "user": {
      "id": "uuid",
      "email": "john@acme.com",
      "role": "owner"
    },
    "token": "jwt_token",
    "expires_at": 1709000000
  }
}
```

### 5.2 租户管理 API

```typescript
// 获取当前租户信息
GET /api/v1/tenant

Response:
{
  "success": true,
  "data": {
    "id": "uuid",
    "name": "Acme Corporation",
    "slug": "acme-corp",
    "plan": "pro",
    "usage": {
      "users": { "current": 3, "max": 10 },
      "tools": { "current": 5, "max": 50 },
      "skills": { "current": 8, "max": 100 },
      "snippets": { "current": 45, "max": 500 },
      "api_calls": { "current": 5000, "max": 50000 },
      "storage_mb": { "current": 120, "max": 1000 }
    }
  }
}

// 更新租户设置
PATCH /api/v1/tenant

Request:
{
  "name": "Acme Corp",
  "settings": {
    "default_visibility": "private",
    "require_2fa": true
  }
}

// 获取租户成员
GET /api/v1/tenant/members

// 邀请成员
POST /api/v1/tenant/members/invite

Request:
{
  "email": "newuser@acme.com",
  "role": "member"  // member | admin
}

// 移除成员
DELETE /api/v1/tenant/members/{user_id}

// 更新成员角色
PATCH /api/v1/tenant/members/{user_id}

Request:
{
  "role": "admin"
}
```

### 5.3 租户切换（仅限超级管理员）

```typescript
// 超级管理员模拟租户
POST /api/v1/admin/tenants/{tenant_id}/impersonate

// 返回该租户的 JWT
```

## 6. 订阅计划

### 6.1 计划定义

| 计划 | Free | Starter | Pro | Enterprise |
|------|------|---------|-----|------------|
| 价格 | $0 | $29/月 | $99/月 | 定制 |
| 用户数 | 3 | 10 | 50 | 无限 |
| 工具数 | 5 | 20 | 100 | 无限 |
| 技能数 | 10 | 50 | 200 | 无限 |
| 片段数 | 50 | 200 | 1000 | 无限 |
| API调用/月 | 1,000 | 10,000 | 100,000 | 无限 |
| 存储 | 100MB | 1GB | 10GB | 无限 |
| 自定义域名 | ❌ | ❌ | ✅ | ✅ |
| 优先支持 | ❌ | ❌ | ✅ | ✅ |
| SLA | ❌ | ❌ | 99.9% | 99.99% |
| SSO/SAML | ❌ | ❌ | ❌ | ✅ |

### 6.2 功能开关

```typescript
interface TenantFeatures {
  // 基础功能
  tools: boolean;
  skills: boolean;
  snippets: boolean;
  
  // 高级功能
  custom_domain: boolean;
  api_access: boolean;
  webhooks: boolean;
  audit_logs: boolean;
  
  // 企业功能
  sso_saml: boolean;
  role_customization: boolean;
  ip_whitelist: boolean;
  advanced_analytics: boolean;
}
```

## 7. 前端变更

### 7.1 租户上下文

```typescript
// stores/tenantStore.ts
interface TenantState {
  tenant: Tenant | null;
  members: User[];
  usage: UsageStats;
  
  fetchTenant: () => Promise<void>;
  updateTenant: (data: Partial<Tenant>) => Promise<void>;
  inviteMember: (email: string, role: string) => Promise<void>;
  removeMember: (userId: string) => Promise<void>;
}
```

### 7.2 路由变更

```
/                           → 首页（租户信息）
/tenant/settings            → 租户设置
/tenant/members             → 成员管理
/tenant/billing             → 账单（计划升级）
/tenant/usage               → 使用统计

/tools                      → 工具（租户内）
/skills                     → 技能（租户内）
/snippets                   → 片段（租户内）
```

### 7.3 租户选择页

对于访问 `app.evolith.io` 的用户：

1. 如果未登录 → 登录页
2. 如果用户属于多个租户 → 租户选择页
3. 如果用户只属于一个租户 → 自动跳转到 `{slug}.evolith.io`

## 8. 安全考虑

### 8.1 数据隔离验证

```rust
// 中间件：确保所有查询都带 tenant_id
pub fn with_tenant_scope(query: &str, tenant_id: Uuid) -> String {
    // 自动注入 WHERE tenant_id = $tenant_id
}

// Repository trait 强制租户参数
pub trait ToolRepository {
    async fn find_by_id(&self, id: Uuid, tenant_id: Uuid) -> Result<Option<Tool>>;
    async fn list(&self, tenant_id: Uuid, params: ListParams) -> Result<Vec<Tool>>;
}
```

### 8.2 越权防护

```rust
// 检查资源所有权
pub async fn check_tenant_access(
    resource_tenant_id: Uuid,
    user_tenant_id: Uuid,
) -> Result<()> {
    if resource_tenant_id != user_tenant_id {
        return Err(AppError::Forbidden("Resource not found".into()));
    }
    Ok(())
}
```

### 8.3 配额检查

```rust
pub async fn check_quota(
    tenant_id: Uuid,
    resource_type: ResourceType,
) -> Result<()> {
    let tenant = tenant_repo.find_by_id(tenant_id).await?;
    let usage = usage_repo.get_current(tenant_id).await?;
    
    let (current, max) = match resource_type {
        ResourceType::Tool => (usage.tools, tenant.max_tools),
        ResourceType::Skill => (usage.skills, tenant.max_skills),
        // ...
    };
    
    if current >= max {
        return Err(AppError::QuotaExceeded);
    }
    Ok(())
}
```

## 9. 迁移计划

数据库结构变更的通用执行流程已归口到 [数据库迁移 SOP](../sop/DATABASE-MIGRATION.md)。本节仅保留多租户改造的历史迁移计划和领域上下文。

### 9.1 数据库迁移

```sql
-- Step 1: 创建 tenants 表
CREATE TABLE tenants (...);

-- Step 2: 创建默认租户
INSERT INTO tenants (id, name, slug, owner_id)
SELECT 
    gen_random_uuid(),
    'Default Tenant',
    'default',
    (SELECT id FROM users LIMIT 1);

-- Step 3: 添加 tenant_id 列（带默认值）
ALTER TABLE users ADD COLUMN tenant_id UUID DEFAULT 
    (SELECT id FROM tenants WHERE slug = 'default');

-- Step 4: 更新现有数据
UPDATE tools SET tenant_id = (SELECT id FROM tenants WHERE slug = 'default') 
    WHERE tenant_id IS NULL;
UPDATE skills SET tenant_id = ...;
UPDATE snippets SET tenant_id = ...;

-- Step 5: 设置非空约束
ALTER TABLE users ALTER COLUMN tenant_id SET NOT NULL;
ALTER TABLE tools ALTER COLUMN tenant_id SET NOT NULL;
-- ...

-- Step 6: 创建索引
CREATE INDEX idx_users_tenant ON users(tenant_id);
CREATE INDEX idx_tools_tenant ON tools(tenant_id);
-- ...
```

### 9.2 代码迁移

1. 添加租户中间件
2. 修改 Repository trait，添加 tenant_id 参数
3. 修改所有查询，添加租户过滤
4. 添加配额检查
5. 更新前端租户上下文

## 10. 监控与运维

### 10.1 租户监控指标

- 活跃租户数
- 各租户 API 调用量
- 配额使用率
- 租户活跃度（日活/周活）

### 10.2 告警规则

- 租户配额接近上限（>80%）
- 租户状态异常（suspended）
- 高频 API 调用（可能的滥用）
- 存储空间不足

## 11. 相关文档

- [架构设计](./ARCHITECTURE.md) - 整体架构
- [API 合约](./API-CONTRACT.md) - API 接口
- [数据库迁移](../backend/migrations/) - SQL 迁移文件
