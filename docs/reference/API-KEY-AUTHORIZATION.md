# API Key 与 MCP 授权合约

> 状态：Current implementation contract  
> Owner Story：[EVO-118-B](../backlog/active/EVO-118-B-api-key-mcp-authorization-hardening.md)  
> 本文档补充 [API-CONTRACT.md](API-CONTRACT.md) 的 API Keys、MCP 和 API key scope 章节；两者冲突时，本文件对这些安全边界优先。

## 1. API Key Management

### 1.1 Authentication and role

```text
GET    /api/v1/tenant/{tenant_id}/api-keys
POST   /api/v1/tenant/{tenant_id}/api-keys
DELETE /api/v1/tenant/{tenant_id}/api-keys/{key_id}
```

要求：

- 有效 JWT；
- JWT `tenant_id` 等于路径 `tenant_id`；
- `tenant_role` 为 `owner` 或 `admin`；
- API Key authentication 即使含历史 `admin` token 也不能调用这些端点。

拒绝响应：

```http
HTTP/1.1 403 Forbidden
Content-Type: application/json
```

```json
{
  "success": false,
  "error": {
    "code": "FORBIDDEN",
    "message": "API key management requires tenant owner or admin JWT authentication"
  }
}
```

### 1.2 Create API Key

```http
POST /api/v1/tenant/{tenant_id}/api-keys
```

```typescript
interface CreateApiKeyRequest {
  name: string; // 1..128 chars
  permissions?: ApiKeyCapability[]; // default ["read"], non-empty
  expires_in_days?: number;
  rate_limit?: number;
}

type ApiKeyCapability =
  | "read"
  | "repo:read"
  | "repo:write"
  | "execute"
  | "promote";
```

Example：

```json
{
  "name": "MCP Production",
  "permissions": ["repo:read", "execute"],
  "expires_in_days": 90
}
```

创建响应中的 `key` 只返回一次：

```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "name": "MCP Production",
    "key": "evo_sk_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
    "key_prefix": "evo_sk_xxxxxxxx...",
    "permissions": ["repo:read", "execute"],
    "expires_at": "2026-10-28T00:00:00Z",
    "rate_limit": 1000,
    "status": "active",
    "created_at": "2026-07-30T00:00:00Z"
  }
}
```

错误：

| 场景 | HTTP | Code | 数据副作用 |
|------|------|------|------------|
| 空 capability | 400 | `VALIDATION_ERROR` | 不创建 Key |
| `admin` / `manage_keys` | 400 | JSON/validation error | 不创建 Key |
| legacy `write` / `commit:*` | 400 | JSON/validation error | 不创建 Key |
| unknown token | 400 | JSON/validation error | 不创建 Key |
| Member / API Key caller | 403 | `FORBIDDEN` | 不创建 Key，写拒绝审计 |
| Tenant mismatch | 403 | `FORBIDDEN` | 不创建 Key，写拒绝审计 |

重复 canonical capability 在持久化前去重。

### 1.3 List API Keys

```http
GET /api/v1/tenant/{tenant_id}/api-keys
```

返回 stored permission strings。历史 Key 可能仍显示：

```text
write
admin
commit
commit:<branch>
```

这些值是 legacy runtime compatibility，不是可新签发 capability。Owner/Admin 应轮换并吊销旧 Key。

### 1.4 Revoke API Key

```http
DELETE /api/v1/tenant/{tenant_id}/api-keys/{key_id}
```

- 不要求请求体；
- Key 不存在和 Key 属于其他 Tenant 使用同一 `404 NOT_FOUND` 响应；
- 成功后 Key 的受保护操作立即失败。

```json
{
  "success": true,
  "data": {
    "message": "API key revoked successfully"
  }
}
```

## 2. Capability Action Matrix

| Capability | Repo read | Repo write | MCP tools/call | Promote | Key management |
|------------|-----------|------------|----------------|---------|----------------|
| `read` | ✅ | ❌ | ❌ | ❌ | ❌ |
| `repo:read` | ✅ | ❌ | ❌ | ❌ | ❌ |
| `repo:write` | ✅ | ✅ | ❌ | ❌ | ❌ |
| `execute` | ❌ | ❌ | ✅ | ❌ | ❌ |
| `promote` | ❌ | ❌ | ❌ | ✅ | ❌ |

Legacy `write/admin/commit/commit:*` 只兼容 Repo read/write；不授予 Tool Execute、Promote 或 Key Management。

## 3. MCP Contract

### 3.1 Discovery

```http
POST /mcp
GET /mcp/tools
```

- `initialize`：可匿名；
- `tools/list`：匿名时只发现 public tools；有效 Key 时按 Key Tenant 发现；
- `GET /mcp/tools`：API Key optional，同上。

### 3.2 Execute

`tools/call` 必须提供有效 Key 且含 `execute`：

```http
POST /mcp
X-API-Key: evo_sk_xxx
Content-Type: application/json
```

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "example-tool",
    "arguments": {}
  }
}
```

无 Key或 Key 无效：

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "error": {
    "code": -32001,
    "message": "A valid API key is required to execute tools"
  }
}
```

缺 `execute`：

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "error": {
    "code": -32003,
    "message": "API key lacks the execute capability"
  }
}
```

Tool 属于其他 Tenant：

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "error": {
    "code": -32001,
    "message": "Tool 'example-tool' not found or access denied"
  }
}
```

授权顺序：

```text
validate key status/expiry
→ require execute
→ parse params
→ find Tool
→ verify Tenant
→ validate schema
→ execute
```

因此缺 `execute` 的请求不会查找 Tool，也不会触发 HTTP executor。

## 4. Security Audit Contract

API Key Management 拒绝事件：

```json
{
  "action": "api_key.management.denied",
  "resource_type": "api_key",
  "details": {
    "operation": "list | create | revoke",
    "reason": "api_key_authentication | tenant_mismatch | insufficient_tenant_role",
    "caller_tenant_id": "uuid",
    "tenant_role": "owner | admin | member"
  }
}
```

不记录：

- API Key secret；
- Authorization header；
- key hash；
- Tool arguments。

## 5. Compatibility

- Schema 未变化，SQLite/PostgreSQL migration 不适用。
- Stored permissions 继续是 string array。
- 新 DTO 是 fail-closed typed enum。
- 旧 Key 不自动删除，以避免升级中断；轮换策略见 [PERMISSIONS.md](PERMISSIONS.md#6-legacy-permission-兼容与轮换)。
- Branch/path scoped Agent Token 不由本合约承诺，归 EVO-105/EVO-106。

## 6. Verification

最低门禁：

```bash
cd backend
cargo test -p api --test api_key_authorization_security_tests
cargo test -p api --test api_key_scope_e2e_tests
cargo test -p api --test mcp_tool_execution_tests
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace

cd ../frontend
bun run type-check
bun run build
```

对应实现：

```text
backend/crates/domain/src/api_key.rs
backend/crates/api/src/dto/api_key_dto.rs
backend/crates/api/src/middleware/api_key_scope.rs
backend/crates/api/src/handlers/api_key_handlers.rs
backend/crates/api/src/handlers/mcp_handlers.rs
frontend/src/lib/api/types.ts
frontend/src/app/tenant/api-keys/page.tsx
```