# EVO-118-B API Key 与 MCP 授权边界硬化

- **类型**：Permission / Security
- **状态**：Ready
- **优先级**：P0
- **父 Epic**：[EVO-118](EVO-118-production-readiness-and-security-hardening.md)
- **依赖**：EVO-118-A merge
- **影响范围**：backend / db / docs / tests

## 用户价值

作为租户 Owner，我需要低权限成员和只读 API Key 无法创建高权限凭证或执行未授权 MCP Tool，从而保证租户权限不会被绕过。

## 已确认失败模式

- API Key 管理只检查 `tenant_id`，未限制 Owner/Admin。
- `permissions` 接受任意字符串列表，调用者可能授予高于自身的能力。
- MCP `tools/call` 验证 Key 有效和租户归属，但未强制 `execute` capability。
- JWT/API Key/未来 Agent Token 的权限语义尚未统一。

## 验收场景

### Scenario 1：Member 不能管理 API Key

- **Given** 当前用户为 Tenant Member
- **When** 列出、创建或吊销 API Key
- **Then** 返回 403，且写入安全审计

### Scenario 2：不能授予高于自身的权限

- **Given** 调用者不具备 `repo:write` / `admin` / `manage_keys`
- **When** 创建包含上述 capability 的 Key
- **Then** 请求被拒绝，不创建数据库记录

### Scenario 3：MCP 执行必须有 execute

- **Given** API Key 只有 `read` 或 `repo:read`
- **When** 调用 MCP `tools/call`
- **Then** 返回明确授权错误，Tool 不被执行

### Scenario 4：跨租户与撤销生效

- **Given** Key 已撤销、过期或属于其他 Tenant
- **When** 调用 Repo、MCP 或 Key Management
- **Then** 统一拒绝且不泄露资源存在性

## 工程要求

- 使用枚举/Typed Capability 替代任意权限字符串的直接授权。
- 建立单一 `authorize(caller, action, resource)` 或等价 helper 边界。
- API Key 管理默认 JWT + Owner/Admin；API Key 自身不得管理其他 Key。
- 明确 `read`、`repo:read`、`repo:write`、`execute`、`promote`、`manage_keys` 的包含关系。
- 对已有 Key 执行一次权限审计或迁移策略。
- 同步 `PERMISSIONS.md`、API Contract 和前端可选权限。

## 不做事项

- 不在本 Story 实现 Agent Session 或 Refresh Token。
- 不修改 HTTP Tool 网络目标校验；归 EVO-118-C。
- 不实现 Branch/Path scoped Agent Token；归 EVO-105/106。

## 最小验证

- API Key 管理角色矩阵集成测试。
- 权限白名单/反序列化单元测试。
- MCP execute scope E2E。
- 跨租户、撤销、过期负向测试。
- SQLite/PostgreSQL 双侧 repository/migration 验证（如 schema 变化）。
- `cargo test --workspace` 与 clippy 门禁。

## 解锁内容

解除 SEC-01 Gate；允许继续 Agent Token 和外部 Alpha 权限设计。
