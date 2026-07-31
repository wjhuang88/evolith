# EVO-118-B API Key 与 MCP 授权边界硬化

- **类型**：Permission / Security
- **状态**：Done
- **优先级**：P0
- **父 Epic**：[EVO-118](EVO-118-production-readiness-and-security-hardening.md)
- **依赖**：EVO-118-A Done
- **所属迭代**：[Iteration 051](../../iterations/ITERATION-051.md)
- **影响范围**：backend / frontend / docs / tests / CI
- **PR**：#3 `security: harden API key and MCP authorization`

## 用户价值

作为租户 Owner，我需要低权限成员和只读 API Key 无法创建高权限凭证或执行未授权 MCP Tool，从而保证租户权限不会被绕过。

## 已确认失败模式

- API Key 管理只检查 `tenant_id`，未限制 Owner/Admin。
- `permissions` 接受任意字符串列表，调用者可能授予未定义或高权限 token。
- MCP `tools/call` 验证 Key 有效和租户归属，但未强制 `execute` capability。
- 新签发能力与历史 `write/admin/commit:*` token 缺少清晰兼容策略。
- Typed DTO 在 handler 授权前反序列化，未授权调用者可用非法 capability 提前得到 400 并绕过 403/拒绝审计。
- 跨租户拒绝审计错误写入目标 Tenant，向目标审计流带入调用者身份。
- MCP 对“不存在 Tool”和“属于其他 Tenant 的 Tool”返回不同错误，可用于存在性枚举。
- `RbacError` 只覆盖响应体、未覆盖 `ResponseError::status_code()`，导致无效/撤销/过期 API Key 的错误对象报告 500 而非 401。

## 验收场景

### Scenario 1：Member 不能管理 API Key

- **Given** 当前用户为 Tenant Member
- **When** 列出、创建或吊销 API Key，包括提交非法 capability 或非法 JSON
- **Then** 授权先于 DTO 解析，返回 403，且写入调用者 Tenant 的安全审计

### Scenario 2：新 Key 只能使用白名单 capability

- **Given** 调用者为同租户 Owner 或 Admin
- **When** 创建包含 `admin`、`manage_keys`、`write`、`commit:*`、空列表或未知 token 的 Key
- **Then** 请求返回 400，不创建数据库记录

### Scenario 3：MCP 执行必须有 execute

- **Given** API Key 只有 `read` 或 `repo:read`
- **When** 调用 MCP `tools/call`
- **Then** 返回明确授权错误，Tool 不被查询或执行

### Scenario 4：跨租户与无效 Key fail closed

- **Given** Key 已撤销、过期或属于其他 Tenant
- **When** 调用 Repo、MCP 或 Key Management
- **Then** Repo 返回 401、MCP 返回统一授权错误、管理接口返回 403，且不泄露资源存在性

### Scenario 5：跨租户审计归属正确

- **Given** Tenant A 的 Owner/Admin 请求管理 Tenant B 的 API Key
- **When** list/create/revoke 被拒绝
- **Then** 拒绝审计仅写入 Tenant A，`target_tenant_id` 只作为详情，不向 Tenant B 写入 foreign identity

## 工程要求与实施结果

- [x] 在 domain 定义 Typed `ApiKeyCapability`，新签发仅允许 `read`、`repo:read`、`repo:write`、`execute`、`promote`。
- [x] 在 API 层建立统一 `ApiKeyAction` 授权 helper；Repo、MCP 和未来 Promote 复用同一入口。
- [x] API Key 管理限定为同租户 Owner/Admin JWT；API Key 自身不得管理其他 Key。
- [x] Create handler 接受 raw `Bytes`，先鉴权和审计，再反序列化 Typed DTO。
- [x] API Key 管理拒绝写入 `api_key.management.denied` 审计。
- [x] 拒绝审计归调用者 Tenant，目标 Tenant 仅写入 `details.target_tenant_id`。
- [x] 历史 `write/admin/commit:*` 只做 Repo 运行兼容，不允许通过新 DTO 再签发。
- [x] MCP `tools/call` 在 Tool 查询和 executor 调用前强制 `execute`。
- [x] 不存在与外租户 Tool 使用同一通用错误，不回显 Tool name，避免存在性枚举。
- [x] `RbacError::status_code()` 显式映射 Unauthorized/InvalidToken=401、Forbidden/TenantMismatch=403。
- [x] 前端提供 canonical capability 选择，Member 页面不再发起无权 list 请求。
- [x] 同步 `PERMISSIONS.md` 与 [API Key Authorization Contract](../../reference/API-KEY-AUTHORIZATION.md)。
- [x] 为 PR 接入前端 type-check/build 和 Rust fmt/check/clippy/workspace test 只读门禁。
- [x] 无 schema 变化；无需 SQLite/PostgreSQL migration。

## Review 评论处理

| 评论 | 处理结果 |
|------|----------|
| `3684325549`：DTO 在授权前反序列化 | 已修复：Create handler 改为 raw body，Member/API Key/cross-tenant caller 即使发送非法 capability 也先得到 403 + audit |
| `3684325560`：跨租户拒绝审计污染目标 Tenant | 已修复：`AuditLog.tenant_id` 绑定调用者 Tenant，目标 Tenant 仅记录在 details |
| 角色/API Key caller 负向矩阵 | 已补 Member、API Key caller、cross-tenant Owner/Admin 的 list/create/revoke E2E |
| revoked / expired Key | 已补 Repo + MCP E2E，并修复 RBAC 401 状态映射 |
| 外租户 Tool 发现与执行 | 已补 discovery 隔离、通用错误和 executor=0 对照测试 |

## 不做事项

- 不在本 Story 实现 Agent Session 或 Refresh Token。
- 不修改 HTTP Tool 网络目标校验；归 EVO-118-C。
- 不实现 Branch/Path scoped Agent Token；归 EVO-105/106。
- 不删除已有 legacy Key；管理员通过列表识别后自行轮换/吊销。
- 不在本 Story 迁移 ESLint 10 flat config；该既有前端基线问题不属于 SEC-01。

## 验证状态

已获得的实际 GitHub Actions 证据：

- `bun install --frozen-lockfile`：通过。
- `bun run type-check`：通过。
- `bun run build`：通过。
- `cargo fmt --all -- --check`：通过。
- `cargo check --workspace --all-targets`：通过。
- `cargo clippy --workspace --all-targets -- -D warnings`：通过。
- 新增安全 E2E 中 Member、API Key caller、cross-tenant Owner/Admin、canonical issuance、MCP execute 和 Tool tenant isolation 均已通过。
- 诊断运行暴露并修复了两个真实边界：隐藏响应仍带 caller tool name、RBAC error object 默认状态为 500。
- 最终只读 GitHub Actions run `30567361095` 在 head `192627a32d7a6f9a780624227db2f357b7350b36` 上全部通过，包括 `cargo test --workspace`。

PR CI 对 pull request 使用 `contents: read`，不再包含自动修改分支的步骤。Frontend lint 因仓库既有 ESLint 10 flat-config 缺失仅保留在 tag/manual；PostgreSQL 扩展测试同样保留为 tag/manual，本 Story 的 required gate 使用 SQLite workspace tests。

## 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 按 PR 评论关闭 SEC-01：成员/API Key 不可管理 Key，只读 Key 不可执行 MCP Tool，跨租户审计与资源隐藏正确 |
| 产物 | Typed capability、统一授权 helper、auth-before-parse、caller-owned audit、MCP hidden response、RBAC status mapping、E2E、前端、Reference、PR CI |
| 状态同步归口 | EVO-118/B、Iteration 051、Product Backlog、Board、Permissions/API Key Authorization Contract |
| 验证证据 | GitHub Actions run `30567361095`：前端 install/type-check/build 与 Rust fmt/check/clippy/workspace tests 全部通过 |
| 残余工作归口 | SEC-01 已解除；SSRF 归 EVO-118-C；Agent scoped token 归 EVO-105/106；legacy Key 轮换记录在 Reference |

## 解锁内容

PR #3 已于 2026-07-31 12:30:50 +08:00 合并，merge commit `6de7845e1231efc04f94f16cb9ab0a410f6ad2d9`；最终 CI run `30567361095` 全绿，SEC-01 Gate 已解除。下一项按计划进入 EVO-118-C。

## 完成记录

- PR：#3 `security: harden API key and MCP authorization`
- 合并时间：2026-07-31 12:30:50 +08:00
- Merge commit：`6de7845e1231efc04f94f16cb9ab0a410f6ad2d9`
- 验证 head：`192627a32d7a6f9a780624227db2f357b7350b36`
- 最终只读 CI：`30567361095`，全部 required gates 通过
- 闭环结论：`Complete`；SEC-01 已解除
