# 用户、租户角色与 API Key 权限

> 本文档描述当前实现的稳定权限事实。API 具体请求/响应见
> [API Contract](API-CONTRACT.md)；API Key 与 MCP 的安全合约见
> [API Key Authorization Contract](API-KEY-AUTHORIZATION.md)。

## 1. 权限模型

Evolith 当前采用两层身份与授权模型：

1. **用户身份**：浏览器或 REST 客户端使用 JWT；用户属于一个 Tenant。
2. **机器身份**：Git、MCP 或外部客户端使用 API Key；Key 绑定一个 Tenant 和创建者。

所有资源访问都必须同时满足：

- 凭证有效且未过期/撤销；
- 请求 Tenant 与凭证 Tenant 一致；
- 调用者角色或 API Key capability 允许目标 Action；
- 资源级所有权、可见性和 Repository Policy 等附加条件通过。

不得仅因“已认证”就允许状态变更或执行操作。

## 2. 当前租户角色

当前代码只支持三种 Tenant Role：

| 角色 | 含义 | 典型权限 |
|------|------|----------|
| `owner` | 租户所有者 | 完整租户管理、计费、成员与 API Key 管理 |
| `admin` | 租户管理员 | 成员、设置、资源与 API Key 管理；不能替代 Owner 的所有权动作 |
| `member` | 普通成员 | 使用平台和管理自己允许的资源；不能管理成员、计费或 API Key |

`viewer` 不是当前 `TenantRole` 枚举成员。历史文档或设计草案中的 viewer 只能作为未来提案，不能用于当前 JWT、数据库或前端权限判断。

### 2.1 当前角色矩阵

| 操作 | owner | admin | member |
|------|-------|-------|--------|
| 查看租户内可见资源 | ✅ | ✅ | ✅ |
| 创建 Tool / Skill / CLI compatibility resource | ✅ | ✅ | ✅ |
| 编辑自己拥有的资源 | ✅ | ✅ | ✅ |
| 管理所有租户资源 | ✅ | ✅ | ❌ |
| 邀请、移除成员或修改成员角色 | ✅ | ✅ | ❌ |
| 管理 API Key | ✅ | ✅ | ❌ |
| 管理计费或升级套餐 | ✅ | 受限 | ❌ |
| 转移/删除租户 | ✅ | ❌ | ❌ |

具体页面显示由 `frontend/src/hooks/usePermission.ts` 决定；服务端仍是最终授权边界。

## 3. JWT 用户授权

JWT Claims 当前包含：

```text
sub          user id
tenant_id    tenant id
tenant_role  owner | admin | member
role         system role
iat / exp    issued / expiry timestamp
```

浏览器主要使用 httpOnly Cookie JWT，并对状态变更执行 CSRF 校验。Header JWT 可用于 API 客户端。

### 3.1 API Key 管理门禁

以下端点只允许 **同租户 Owner/Admin 的 JWT**：

```text
GET    /api/v1/tenant/{tenant_id}/api-keys
POST   /api/v1/tenant/{tenant_id}/api-keys
DELETE /api/v1/tenant/{tenant_id}/api-keys/{key_id}
```

拒绝条件：

- Member 调用；
- JWT Tenant 与路径 Tenant 不一致；
- 使用 API Key 自身调用 Key Management；
- 缺少有效 JWT。

已认证但无权的请求返回 `403 FORBIDDEN`，并写入：

```text
action = api_key.management.denied
resource_type = api_key
details.operation = list | create | revoke
details.reason = api_key_authentication | tenant_mismatch | insufficient_tenant_role
details.caller_tenant_id = credential tenant
details.target_tenant_id = path tenant
```

安全顺序与审计归属：

- Create API 先执行身份/角色/Tenant 门禁，再反序列化 Typed capability；
- 未授权调用者即使发送 `permissions: ["admin"]` 或非法 JSON，也返回 403 并写拒绝审计；
- `AuditLog.tenant_id` 始终是调用者 Tenant；
- `user_id`、IP、User-Agent 描述调用者；
- 跨租户目标只记录在 `details.target_tenant_id`，不会向目标 Tenant 审计流写入 foreign identity；
- 审计失败不会把拒绝请求变成成功。

## 4. API Key 存储与生命周期

数据库仍以 `Vec<String>` / JSON 存储 permission token，以兼容历史 Key 和滚动升级。新签发入口使用 Typed `ApiKeyCapability`，不会接受任意字符串。

API Key 有三种状态：

| 状态 | 行为 |
|------|------|
| `active` | 在未过期且 capability 允许时可使用 |
| `revoked` | Repo、MCP 和其他受保护操作拒绝 |
| `expired` 或 `expires_at < now` | Repo、MCP 和其他受保护操作拒绝 |

Secret 只在创建响应中返回一次；数据库仅存 SHA-256 hash 和显示 prefix。

## 5. Canonical API Key Capability

新签发 Key 只允许以下 token：

| Capability | 授权 Action | 不隐含 |
|------------|-------------|--------|
| `read` | 通用只读兼容能力；当前包含 Repo read | Repo write、Tool execute、Promote、Key management |
| `repo:read` | Repo metadata、Context、clone/read | Repo write、Tool execute、Promote |
| `repo:write` | Repo read + Repo write | Tool execute、Promote、Key management |
| `execute` | MCP `tools/call` | Repo read/write、Promote、Key management |
| `promote` | Promote Action 的未来/配套授权边界 | Repo write、Tool execute、Key management |

规则：

- 至少选择一个 capability；空列表返回 `400`。
- 重复 capability 在写入前去重。
- 已授权 Owner/Admin 提交未知 token 返回 `400`，不会创建数据库记录。
- 未授权调用者的 body 不参与 capability 判断，始终先按身份边界返回 403。
- API Key 永远不能创建、列出或吊销其他 API Key。
- capability 只表达调用资格；Tenant、资源可见性和 Policy 仍需单独检查。

## 6. Legacy Permission 兼容与轮换

历史数据库可能存在：

```text
write
admin
commit
commit:<branch>
```

这些 token：

- **不可通过新 Create API 再签发**；
- 为避免升级时立即中断现有 Git 客户端，暂时只在 Repo read/write 授权 helper 中兼容；
- **不会**获得 MCP `execute`、Promote 或 API Key management；
- 在 Owner/Admin 列表 API 中被识别并写 warning，管理员应轮换为 canonical capability；
- `commit:<branch>` 当前仍是历史通用 Repo write 兼容，不代表真实 branch scope。真正的 branch/path scoped Agent Token 归 EVO-105/EVO-106。

推荐轮换：

1. 列出 Key 并识别 legacy token；
2. 创建最小权限 canonical Key；
3. 更新客户端凭证并验证；
4. 吊销旧 Key；
5. 检查审计与最后使用时间。

## 7. MCP 授权

### 7.1 Discovery

`initialize`、公共 `tools/list` 和 `GET /mcp/tools` 保持发现语义。提供有效 Key 时，只按该 Key Tenant 发现对应 Tool，其他 Tenant 的 private Tool 不出现在结果中。

### 7.2 Execution

`POST /mcp` 的 `tools/call` 必须满足：

- 有效、Active、未过期 API Key；
- Key 包含精确 `execute` capability；
- Tool 与 Key 属于同一 Tenant；
- 参数 Schema 验证通过；
- 后续 EVO-118-C 出站网络策略通过。

缺少 `execute` 时返回 JSON-RPC 错误：

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

撤销或过期 Key 返回统一 `-32001 Invalid or expired API key`。Tool 不存在和 Tool 属于其他 Tenant 也使用完全相同的 `-32001 not found or access denied` 语义，调用者不能通过错误差异枚举外租户资源。

授权检查发生在副作用之前：

- 缺 `execute` 时不会查询 Tool；
- 撤销/过期 Key 不进入 Tool 路径；
- 不存在或外租户 Tool 不调用 executor；
- 只有同租户 Tool 在 Schema 通过后执行。

## 8. Repo Action 映射

统一授权 helper 使用 Action，而不是在 Handler 中散落字符串判断：

```text
ApiKeyAction::RepoRead
ApiKeyAction::RepoWrite
ApiKeyAction::ToolExecute
ApiKeyAction::Promote
```

Canonical 和 legacy 映射：

| Stored token | RepoRead | RepoWrite | ToolExecute | Promote |
|--------------|----------|-----------|-------------|---------|
| `read` | ✅ | ❌ | ❌ | ❌ |
| `repo:read` | ✅ | ❌ | ❌ | ❌ |
| `repo:write` | ✅ | ✅ | ❌ | ❌ |
| `execute` | ❌ | ❌ | ✅ | ❌ |
| `promote` | ❌ | ❌ | ❌ | ✅ |
| legacy `write/admin/commit/commit:*` | ✅ | ✅ | ❌ | ❌ |
| unknown/empty | ❌ | ❌ | ❌ | ❌ |

## 9. 安全测试基线

权限变更至少覆盖：

- Member 对 list/create/revoke API Key 均为 403；
- Member + invalid capability 仍是 403，且写调用者租户审计；
- API Key caller 对 list/create/revoke 均为 403，三类拒绝均审计；
- 跨租户 Owner/Admin 的 list/create/revoke 均为 403；
- 跨租户审计仅存在于调用者 Tenant，目标 Tenant 审计流为空；
- Owner/Admin canonical 签发成功；
- `admin`、`manage_keys`、`write`、`commit:*` 和 unknown 新签发均为 400；
- 空 capability 为 400；
- 拒绝请求不创建或吊销 Key；
- `repo:read` Key 调用 `tools/call` 返回 `-32003`，executor 命中数为 0；
- `execute` Key 可进入同租户 Tool 执行；
- revoked/expired Key 对 Repo 和 MCP 均 fail closed；
- 外租户 Tool 不可发现，且与不存在 Tool 返回相同错误、executor 命中数为 0。

实现测试：

```text
backend/crates/api/tests/api_key_authorization_security_tests.rs
backend/crates/api/tests/api_key_scope_e2e_tests.rs
backend/crates/api/tests/mcp_tool_execution_tests.rs
```

PR 门禁由 `.github/workflows/ci.yml` 的 `pull_request` trigger 执行前端 type-check/build 和后端 fmt/check/clippy/workspace tests。

## 10. 后续边界

- HTTP Tool SSRF/DNS/Redirect/Egress：EVO-118-C。
- Branch/Path scoped Agent Token：EVO-105/EVO-106。
- API Key rate limit 实际接线：EVO-118-G。
- ESLint 10 flat-config 迁移：独立前端/CI 基线工作，不作为 EVO-118-B 安全 Gate。
- JWT session revoke/token version：后续独立 Auth Story。

## 11. 相关文档

- [API Key Authorization Contract](API-KEY-AUTHORIZATION.md)
- [Security Review SOP](../sop/SECURITY-REVIEW.md)
- [Production Readiness Baseline](PRODUCTION-READINESS-BASELINE.md)
- [EVO-118-B](../backlog/active/EVO-118-B-api-key-mcp-authorization-hardening.md)
- [Iteration 051](../iterations/ITERATION-051.md)