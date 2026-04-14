# 用户与权限系统设计

## 1. 概述

Evolith 作为 SaaS 平台，需要完整的用户和权限系统来管理：
- 用户注册与认证
- 角色基于访问控制 (RBAC)
- 租户成员管理
- API Key 认证

## 2. 用户模型

### 2.1 用户表

```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- 身份信息
    email VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR(64) UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    
    -- 租户关联
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    tenant_role VARCHAR(32) DEFAULT 'member',  -- owner | admin | member
    
    -- 用户资料
    full_name VARCHAR(128),
    avatar_url VARCHAR(512),
    phone VARCHAR(32),
    
    -- 状态
    status VARCHAR(32) DEFAULT 'active',  -- active | inactive | suspended
    email_verified BOOLEAN DEFAULT FALSE,
    
    -- 安全
    failed_login_attempts INTEGER DEFAULT 0,
    locked_until TIMESTAMPTZ,
    last_login_at TIMESTAMPTZ,
    password_changed_at TIMESTAMPTZ,
    
    -- MFA
    mfa_enabled BOOLEAN DEFAULT FALSE,
    mfa_secret TEXT,
    
    -- 时间戳
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

CREATE INDEX idx_users_tenant ON users(tenant_id);
CREATE INDEX idx_users_email ON users(email);
```

### 2.2 角色定义

| 角色 | 描述 | 权限 |
|------|------|------|
| owner | 租户所有者 | 完全控制，删除租户，管理计费 |
| admin | 租户管理员 | 管理成员，配置，设置 |
| member | 普通成员 | 使用平台功能 |
| viewer | 只读成员 | 仅查看，不能修改 |

### 2.3 权限矩阵

| 操作 | owner | admin | member | viewer |
|------|-------|-------|--------|--------|
| 查看工具/技能/片段 | ✅ | ✅ | ✅ | ✅ |
| 创建工具/技能/片段 | ✅ | ✅ | ✅ | ❌ |
| 编辑自己的资源 | ✅ | ✅ | ✅ | ❌ |
| 编辑所有资源 | ✅ | ✅ | ❌ | ❌ |
| 删除资源 | ✅ | ✅ | ❌ | ❌ |
| 管理成员 | ✅ | ✅ | ❌ | ❌ |
| 管理 API Key | ✅ | ✅ | ❌ | ❌ |
| 查看账单 | ✅ | ✅ | ❌ | ❌ |
| 升级套餐 | ✅ | ❌ | ❌ | ❌ |
| 删除租户 | ✅ | ❌ | ❌ | ❌ |

## 3. 认证系统

### 3.1 注册流程

```
用户注册 → 邮箱验证 → 创建租户 → 分配 owner 角色
```

```typescript
// POST /api/v1/auth/register
Request:
{
  "email": "user@example.com",
  "username": "john",
  "password": "SecurePass123!",
  "tenant_name": "My Company",       // 可选
  "tenant_slug": "my-company"          // 可选
}

Response:
{
  "success": true,
  "data": {
    "user": { "id": "...", "email": "...", "tenant_role": "owner" },
    "tenant": { "id": "...", "name": "My Company", "slug": "my-company" },
    "token": "jwt_token"
  }
}
```

### 3.2 登录流程

```typescript
// POST /api/v1/auth/login
Request:
{
  "email": "user@example.com",
  "password": "SecurePass123!"
}

// 首次登录需要邮箱验证
Response (未验证):
{
  "success": false,
  "error": { "code": "EMAIL_NOT_VERIFIED", "message": "请先验证邮箱" }
}

Response (成功):
{
  "success": true,
  "data": {
    "user": { "id": "...", "email": "...", "tenant_role": "owner" },
    "tenant": { "id": "...", "name": "My Company", "plan": "free" },
    "token": "jwt_token",
    "expires_at": 1709000000
  }
}
```

### 3.3 JWT Token 载荷

```typescript
interface JWTPayload {
  sub: string;        // user_id
  email: string;
  tenant_id: string;
  tenant_role: 'owner' | 'admin' | 'member' | 'viewer';
  iat: number;
  exp: number;
}
```

### 3.4 密码安全

- 使用 Argon2id 哈希
- 最小长度：8 字符
- 必须包含：大小写字母 + 数字
- 90 天强制更换（可选）
- 登录失败 5 次后锁定 15 分钟

## 4. 成员管理

### 4.1 邀请成员

```typescript
// POST /api/v1/tenant/members/invite
Request:
{
  "email": "colleague@company.com",
  "role": "member",
  "message": "欢迎加入我们的团队！"  // 可选
}

// 生成邀请链接和 token
Response:
{
  "success": true,
  "data": {
    "id": "uuid",
    "email": "colleague@company.com",
    "role": "member",
    "invite_url": "https://evolith.io/join/abc123",
    "expires_at": "2024-02-01T00:00:00Z"
  }
}
```

### 4.2 接受邀请

```typescript
// POST /api/v1/auth/join
Request:
{
  "token": "abc123",
  "password": "SecurePass123!"
}
```

### 4.3 成员列表

```typescript
// GET /api/v1/tenant/members
Response:
{
  "success": true,
  "data": [
    {
      "id": "uuid",
      "email": "owner@company.com",
      "username": "owner",
      "full_name": "John Doe",
      "role": "owner",
      "status": "active",
      "joined_at": "2024-01-01T00:00:00Z",
      "last_login_at": "2024-01-15T00:00:00Z"
    },
    {
      "id": "uuid",
      "email": "admin@company.com",
      "username": "admin",
      "role": "admin",
      "status": "active",
      "joined_at": "2024-01-05T00:00:00Z"
    }
  ]
}
```

## 5. API Key 系统

### 5.1 API Key 表

```sql
CREATE TABLE api_keys (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- 关联
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    user_id UUID NOT NULL REFERENCES users(id),
    
    -- Key 信息
    name VARCHAR(128) NOT NULL,
    key_hash TEXT NOT NULL,           -- SHA256 hash
    key_prefix VARCHAR(16) NOT NULL,  -- 前缀用于显示 (evo_xxxxxxxx)
    
    -- 权限
    permissions JSONB DEFAULT '["read"]',  -- ["read", "write", "admin"]
    
    -- 限制
    expires_at TIMESTAMPTZ,
    last_used_at TIMESTAMPTZ,
    rate_limit INTEGER DEFAULT 1000,  -- 每小时请求数
    
    -- 状态
    status VARCHAR(32) DEFAULT 'active',  -- active | revoked | expired
    
    -- 使用统计
    request_count INTEGER DEFAULT 0,
    
    -- 时间戳
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_api_keys_tenant ON api_keys(tenant_id);
CREATE INDEX idx_api_keys_prefix ON api_keys(key_prefix);
```

### 5.2 创建 API Key

```typescript
// POST /api/v1/tenant/api-keys
Request:
{
  "name": "Production API",
  "permissions": ["read", "write"],
  "expires_in_days": 90,  // 可选，默认永不过期
  "rate_limit": 5000      // 可选，每小时
}

Response:
{
  "success": true,
  "data": {
    "id": "uuid",
    "name": "Production API",
    "key": "evo_sk_xxxxxxxxxxxxxxxxxxxx",  // ⚠️ 只在创建时返回一次
    "key_prefix": "evo_sk_xxxxxxxx",
    "permissions": ["read", "write"],
    "expires_at": "2024-04-15T00:00:00Z",
    "rate_limit": 5000
  }
}
```

### 5.3 使用 API Key

```bash
# Header 认证
curl -H "Authorization: Bearer evo_sk_xxxxxxxxxxxxxxxxxxxx" \
     https://api.evolith.io/v1/tools

# 或者使用 X-API-Key
curl -H "X-API-Key: evo_sk_xxxxxxxxxxxxxxxxxxxx" \
     https://api.evolith.io/v1/tools
```

### 5.4 API Key 验证

```rust
pub async fn validate_api_key(key: &str, pool: &Pool) -> Result<ApiKey> {
    // 1. 提取前缀
    let prefix = key.split('_').last()?;
    
    // 2. 查找 key_prefix
    let stored = sqlx::query_as::<_, ApiKey>(
        "SELECT * FROM api_keys WHERE key_prefix = ? AND status = 'active'"
    )
    .bind(prefix)
    .fetch_one(pool)
    .await?;
    
    // 3. 验证 hash
    let key_hash = sha256(key);
    if key_hash != stored.key_hash {
        return Err(AppError::Unauthorized);
    }
    
    // 4. 检查过期
    if let Some(expires) = stored.expires_at {
        if expires < Utc::now() {
            return Err(AppError::ApiKeyExpired);
        }
    }
    
    // 5. 更新最后使用时间
    sqlx::query("UPDATE api_keys SET last_used_at = NOW() WHERE id = ?")
        .bind(&stored.id)
        .execute(pool)
        .await?;
    
    Ok(stored)
}
```

## 6. 前端实现

### 6.1 用户状态管理

```typescript
// stores/authStore.ts
interface AuthState {
  user: User | null;
  tenant: Tenant | null;
  token: string | null;
  isAuthenticated: boolean;
  
  login: (email: string, password: string) => Promise<void>;
  logout: () => Promise<void>;
  register: (data: RegisterRequest) => Promise<void>;
  refreshToken: () => Promise<void>;
}
```

### 6.2 权限检查

```typescript
// hooks/usePermission.ts
function usePermission() {
  const { user } = useAuth();
  
  const can = (action: string) => {
    const permissions = {
      owner: ['*'],
      admin: ['read', 'write', 'manage_members', 'manage_settings'],
      member: ['read', 'write'],
      viewer: ['read']
    };
    
    const userPerms = permissions[user?.tenant_role] || [];
    return userPerms.includes('*') || userPerms.includes(action);
  };
  
  return { can };
}

// 使用
function DeleteButton({ resource }) {
  const { can } = usePermission();
  
  if (!can('delete')) return null;
  
  return <Button onClick={deleteResource}>删除</Button>;
}
```

### 6.3 API Key 管理页面

```typescript
// app/tenant/api-keys/page.tsx
export default function ApiKeysPage() {
  const [keys, setKeys] = useState<ApiKey[]>([]);
  
  // 列出 API Keys
  const { data } = useQuery(['api-keys'], () => api.listKeys());
  
  // 创建 API Key
  const createKey = async (data) => {
    const result = await api.createKey(data);
    // ⚠️ 显示 key 一次
    showKeyModal(result.key);
  };
  
  // 撤销 API Key
  const revokeKey = async (id) => {
    await api.revokeKey(id);
    refetch();
  };
  
  return (
    <div>
      <h1>API Keys</h1>
      <Button onClick={() => setShowCreate(true)}>创建 API Key</Button>
      
      <table>
        <thead>
          <tr>
            <th>名称</th>
            <th>Key</th>
            <th>权限</th>
            <th>使用次数</th>
            <th>最后使用</th>
            <th>操作</th>
          </tr>
        </thead>
        <tbody>
          {data?.map(key => (
            <tr key={key.id}>
              <td>{key.name}</td>
              <td>{key.key_prefix}****</td>
              <td>{key.permissions.join(', ')}</td>
              <td>{key.request_count}</td>
              <td>{key.last_used_at}</td>
              <td>
                <Button onClick={() => revokeKey(key.id)}>撤销</Button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
```

## 7. 安全考虑

### 7.1 速率限制

```rust
// 登录限流
- 同一 IP：5 次/分钟
- 同一账户：10 次/15分钟

// API 限流（基于套餐）
- Free: 100/小时
- Pro: 1000/小时  
- Enterprise: 10000/小时
```

### 7.2 审计日志

```sql
CREATE TABLE audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID REFERENCES tenants(id),
    user_id UUID REFERENCES users(id),
    
    action VARCHAR(64) NOT NULL,
    resource_type VARCHAR(32),
    resource_id UUID,
    details JSONB,
    ip_address VARCHAR(45),
    user_agent TEXT,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_audit_tenant ON audit_logs(tenant_id, created_at);
```

### 7.3 敏感操作

以下操作需要记录审计日志：
- 登录/登出
- 密码更改
- API Key 创建/撤销
- 成员邀请/移除
- 角色变更
- 套餐变更
- 删除资源

## 8. 相关文档

- [多租户设计](./multi-tenant.md)
- [套餐与计费](./billing.md)
- [API 合约](./api-contract.md)