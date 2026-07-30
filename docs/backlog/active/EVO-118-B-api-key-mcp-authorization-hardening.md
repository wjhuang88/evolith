# EVO-118-B API Key 与 MCP 授权边界硬化

- **类型**：Permission / Security
- **状态**：Review
- **优先级**：P0
- **父 Epic**：[EVO-118](EVO-118-production-readiness-and-security-hardening.md)
- **依赖**：EVO-118-A Done
- **所属迭代**：[Iteration 051](../../iterations/ITERATION-051.md)
- **影响范围**：backend / frontend / docs / tests
- **PR**：#3 `security: harden API key and MCP authorization`

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

## 工程要求与实施结果

- [x] 在 domain 定义 Typed `ApiKeyCapability`，新签发仅允许 `read`、`repo:read`、`repo:write`、`execute`、`promote`。
- [x] 在 API 层建立统一 `ApiKeyAction` 授权 helper；Repo、MCP 和未来 Promote 复用同一入口。
- [x] API Key 管理限定为同租户 Owner/Admin JWT；API Key 自身不得管理其他 Key。
- [x] API Key 管理拒绝写入 `api_key.management.denied` 审计。
- [x] 历史 `write/admin/commit:*` 只做 Repo 运行兼容，不允许通过新 DTO 再签发。
- [x] MCP `tools/call` 在 Tool 查询和 executor 调用前强制 `execute`。
- [x] 前端提供 canonical capability 选择，Member 页面不再发起无权 list 请求。
- [x] 同步 `PERMISSIONS.md` 与 [API Key Authorization Contract](../../reference/API-KEY-AUTHORIZATION.md)。
- [x] 无 schema 变化；无需 SQLite/PostgreSQL migration。

## 不做事项

- 不在本 Story 实现 Agent Session 或 Refresh Token。
- 不修改 HTTP Tool 网络目标校验；归 EVO-118-C。
- 不实现 Branch/Path scoped Agent Token；归 EVO-105/106。
- 不删除已有 legacy Key；管理员通过列表识别后自行轮换/吊销。

## 验证状态

已完成静态审查：

- `main...agent/harden-api-key-mcp-auth`：分支基于 PR #2 merge commit，0 behind。
- Compare 只包含 EVO-118-B 的 backend/frontend/tests/docs，无 migration、部署配置或脚本改动。
- 已对照 `AuditLog`、`AuditRepository`、`AppState`、API Key 路由、JWT role helper 和前端权限 hook。
- 已建立安全 E2E，覆盖 Member 403+审计、canonical/legacy 签发、MCP executor 命中计数。

尚未执行的 hard-required 门禁：

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `bun run type-check`
- `bun run build`

原因：当前环境没有私有仓库本地 checkout，仓库现有 CI 仍为 tag-only，PR 不会自动执行这些命令。在真实运行证据出现前本 Story 保持 `Review / Partial`。

## 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 关闭 SEC-01：成员/API Key 不可管理 Key，只读 Key 不可执行 MCP Tool |
| 产物 | Typed capability、统一授权 helper、handler 门禁、拒绝审计、E2E/单元测试、前端和 Reference 同步 |
| 状态同步归口 | EVO-118/B、Iteration 051、Product Backlog、Board、Permissions/API Key Authorization Contract |
| 验证证据 | GitHub compare、文件级静态核对、新增自动化测试代码；运行门禁待本地/CI |
| 残余工作归口 | 本 PR 的 Rust/前端门禁；SSRF 归 EVO-118-C；Agent scoped token 归 EVO-105/106；legacy Key 轮换策略记录在 Reference |

## 解锁内容

PR #3 的 hard-required 门禁全部通过并合并后，解除 SEC-01 Gate；下一项按计划启动 EVO-118-C。