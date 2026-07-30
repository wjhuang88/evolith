# EVO-118-B API Key 与 MCP 授权边界硬化

- **类型**：Permission / Security
- **状态**：In Progress
- **优先级**：P0
- **父 Epic**：[EVO-118](EVO-118-production-readiness-and-security-hardening.md)
- **依赖**：EVO-118-A Done
- **所属迭代**：[Iteration 051](../../iterations/ITERATION-051.md)
- **影响范围**：backend / frontend / docs / tests

## 用户价值

作为租户 Owner，我需要低权限成员和只读 API Key 无法创建高权限凭证或执行未授权 MCP Tool，从而保证租户权限不会被绕过。

## 已确认失败模式

- API Key 管理只检查 `tenant_id`，未限制 Owner/Admin。
- `permissions` 接受任意字符串列表，调用者可能授予未定义或高权限 token。
- MCP `tools/call` 验证 Key 有效和租户归属，但未强制 `execute` capability。
- 新签发能力与历史 `write/admin/commit:*` token 缺少清晰兼容策略。

## 验收场景

### Scenario 1：Member 不能管理 API Key

- **Given** 当前用户为 Tenant Member
- **When** 列出、创建或吊销 API Key
- **Then** 返回 403，且写入安全审计

### Scenario 2：新 Key 只能使用白名单 capability

- **Given** 调用者为 Tenant Owner 或 Admin
- **When** 创建包含 `admin`、`manage_keys`、`write`、`commit:*` 或未知 token 的 Key
- **Then** 请求返回 400，不创建数据库记录

### Scenario 3：MCP 执行必须有 execute

- **Given** API Key 只有 `read` 或 `repo:read`
- **When** 调用 MCP `tools/call`
- **Then** 返回明确授权错误，Tool 不被执行

### Scenario 4：跨租户与撤销生效

- **Given** Key 已撤销、过期或属于其他 Tenant
- **When** 调用 Repo、MCP 或 Key Management
- **Then** 统一拒绝且不泄露资源存在性

## 工程要求

- 在 domain 定义 Typed `ApiKeyCapability`，新签发仅允许 `read`、`repo:read`、`repo:write`、`execute`、`promote`。
- 在 API 层建立统一 `ApiKeyAction` 授权 helper；Repo、MCP 和未来 Promote 复用同一入口。
- API Key 管理默认 JWT + Owner/Admin；API Key 自身不得管理其他 Key。
- 历史 `write/admin/commit:*` 只做运行兼容，不允许通过新 DTO 再签发。
- 同步 `PERMISSIONS.md`、API Contract、前端 capability 选择。
- 无 schema 变化；无需 SQLite/PostgreSQL migration。

## 不做事项

- 不在本 Story 实现 Agent Session 或 Refresh Token。
- 不修改 HTTP Tool 网络目标校验；归 EVO-118-C。
- 不实现 Branch/Path scoped Agent Token；归 EVO-105/106。
- 不删除已有 legacy Key；管理员通过列表识别后自行轮换/吊销。

## 最小验证

- API Key 管理角色矩阵 E2E。
- Typed capability 序列化、白名单和 legacy compatibility 单元测试。
- MCP execute scope E2E，证明无 `execute` 时 executor 不被调用。
- 跨租户、撤销、过期负向测试保持通过。
- `cargo fmt --all -- --check`、`cargo clippy --workspace --all-targets -- -D warnings`、`cargo test --workspace`。
- `bun run type-check`、`bun run build`。

## 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 关闭 SEC-01：成员/API Key 不可管理 Key，只读 Key 不可执行 MCP Tool |
| 产物 | Typed capability、统一授权 helper、handler 门禁、E2E/单元测试、前端和 Reference 同步 |
| 状态同步归口 | EVO-118/B、Iteration 051、Product Backlog、Board、Permissions/API Contract |
| 验证证据 | Rust/前端门禁、负向授权测试、PR compare/CI |
| 残余工作归口 | SSRF 归 EVO-118-C；Agent scoped token 归 EVO-105/106；legacy Key 轮换策略记录在 Reference |

## 解锁内容

解除 SEC-01 Gate；允许继续 Agent Token 和外部 Alpha 权限设计。