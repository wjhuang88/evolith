# Iteration 051: API Key 与 MCP 授权边界硬化

> 文档状态：Closed / Complete
> 计划发布日期：2026-07-30  
> 计划目标：关闭 SEC-01，阻止 Member/API Key 管理凭证，并要求 MCP `tools/call` 具备 `execute` capability。  
> PR：#3 `security: harden API key and MCP authorization`  
>
> 基线保护：本文件一旦提交，以下“发布计划基线”内容不可因实施或改线而覆写；同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。  
> 闭环步骤：按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 0. Iteration 库存处置

| 既有 Iteration | 状态 | 本轮处置 |
|----------------|------|----------|
| Iteration 050 | Closed / Complete | PR #2 已 merge；EVO-118-A Done，解除本 Story 启动依赖 |
| Iteration 018~020、027 | Blocked / Superseded | 继续不激活 |
| Iteration 025~026 | Blocked for activation | 继续阻塞，不抢占 EVO-118 S1 |
| EVO-112 | Proposed | 继续暂缓实现，仅允许 refinement |

盘点结论：没有未处置的 Active/In Progress/Review Iteration；按 Production Readiness Plan 选择最高优先级 Ready Story EVO-118-B。

## 1. 发布计划基线：目标

- API Key 管理仅允许同租户 Owner/Admin 的 JWT 调用。
- 新签发 API Key 使用 Typed capability 白名单，拒绝未知、legacy 和管理类 token。
- API Key Create 必须先鉴权/审计，再解析 Typed DTO。
- MCP `tools/call` 必须由包含 `execute` 的有效 API Key 调用。
- 跨租户拒绝审计归调用者 Tenant，不污染目标 Tenant 审计流。
- 不存在与外租户 Tool 不可通过错误差异区分。
- 撤销/过期 Key 在 Repo 返回 401、MCP 返回统一无效凭证错误。
- 历史 `write/admin/commit:*` Key 保持 Repo 运行兼容，但不可新签发，并给出轮换策略。
- 权限负向测试、前端能力选择、Reference/Contract 和 PR CI 同步完成。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|-----------|--------|---------------|
| EVO-118-B | API Key 与 MCP 授权边界硬化 | EVO-118 | P0 | EVO-118-A Done；Security Review SOP 已生效 |

## 3. 发布计划基线：不做事项

- 不实现 Agent Session、Refresh Token、Branch/Path scoped token。
- 不实现 HTTP Tool SSRF/Egress 校验；归 EVO-118-C。
- 不修改 API Key 数据库 schema。
- 不删除或自动迁移已有 legacy Key。
- 不改变 Git Smart HTTP 的 legacy permission compatibility 行为。
- 不在本 Story 迁移 ESLint 10 flat config。

## 4. Story / BDD 适用性

EVO-118-B 为 Permission / Security Story，使用 Given/When/Then 负向行为验收：

- Member list/create/revoke API Key → 403 + 调用者租户安全审计。
- Member + invalid capability / invalid JSON → 仍先返回 403 + audit。
- API Key caller list/create/revoke → 403 + audit，不创建或吊销 Key。
- Cross-tenant Owner/Admin list/create/revoke → 403；审计仅在 caller tenant。
- Owner/Admin 创建 canonical capability Key → 成功。
- Owner/Admin 创建 `admin` / `manage_keys` / `write` / `commit:*` / empty / unknown → 400，无 DB 记录。
- `read` / `repo:read` Key 调用 MCP `tools/call` → `-32003`，executor 不执行。
- `execute` Key 调用同租户 Tool → 保持成功。
- Revoked/expired Key 调用 Repo/MCP → 401 / `-32001`。
- Foreign Tool 不可发现；与 missing Tool 返回同一通用错误且 executor 不执行。

## 5. 发布计划基线：计划验证

```bash
cd frontend
bun install --frozen-lockfile
bun run type-check
bun run build

cd ../backend
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

定向验证：

```bash
cd backend
cargo test -p api --test api_key_authorization_security_tests
cargo test -p api --test api_key_scope_e2e_tests
cargo test -p api --test mcp_tool_execution_tests
cargo test -p api api_key_scope
```

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| Typed DTO 使已有客户端提交 legacy token 失败 | 这是同租户 Owner/Admin 新签发的预期 fail-closed；授权合约和前端同步 canonical 列表 |
| 未授权非法 body 绕过 403/audit | Create 接受 raw Bytes，先鉴权再解析；负向 E2E 锁定 |
| 跨租户审计暴露 foreign identity | AuditLog 归 caller tenant，target 只作为 detail；目标审计流为空的 E2E 锁定 |
| 历史 Key 被意外失效 | scope helper 保留 `write/admin/commit:*` Repo 兼容；只禁止新签发 |
| MCP discovery/错误泄露外租户 Tool | discovery 按 tenant 过滤；missing/foreign 使用同一 generic error |
| 撤销/过期 Key 返回 500 | `RbacError::status_code()` 显式映射 401/403；Repo+MCP E2E 锁定 |
| MCP discovery 被误阻断 | `initialize` / `tools/list` 保持原行为，仅 `tools/call` 强制 `execute` |
| 审计写入失败影响授权 | 授权仍 fail-closed；审计失败仅记录 error，不把拒绝变成成功 |
| 回滚 | 回退本 PR；无 migration 和数据变更 |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 按计划开发 EVO-118-B，并根据 PR review comments 修正安全边界 |
| 产物 | Domain capability、授权 helper、auth-before-parse、caller-owned audit、MCP hidden response、RBAC status mapping、测试、前端、文档和 PR CI |
| 状态同步归口 | EVO-118/B、Iteration 051、Product Backlog、Board、Permissions/API Key Authorization Contract |
| Story/BDD 归口 | Permission / Security；负向 Given/When/Then |
| 验证证据 | GitHub Actions run `30567361095`：前端 install/type-check/build 与 Rust fmt/check/clippy/workspace tests 全部通过 |
| 残余工作归口 | SEC-01 已解除；SSRF → EVO-118-C；Agent token → EVO-105/106；legacy Key 轮换 → Permissions Reference |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-07-30 | activation | Iteration 050 已关闭；选择 EVO-118-B，状态更新为 In Progress |
| 2026-07-30 | progress | 新增 Typed `ApiKeyCapability` 与统一 `ApiKeyAction` helper；保留 legacy Repo compatibility |
| 2026-07-30 | progress | API Key list/create/revoke 限定同租户 Owner/Admin JWT；拒绝写安全审计 |
| 2026-07-30 | progress | MCP `tools/call` 在 Tool 查询/executor 前强制 `execute`，无权限返回 `-32003` |
| 2026-07-30 | progress | 新增安全 E2E、前端 capability 选择、Permissions 与授权子合约 |
| 2026-07-30 | navigator | 修复无请求体 DELETE 契约和 MCP `prompts/list` 空列表响应，避免无关回归 |
| 2026-07-30 | validation | Compare 0 behind；只含 EVO-118-B backend/frontend/tests/docs/CI，无 migration/deploy/scripts |
| 2026-07-30 | review | Draft PR #3 创建，进入 Review / Partial |
| 2026-07-31 | review-feedback | 评论 `3684325549`：Create 改为 raw Bytes，授权和审计先于 DTO 解析 |
| 2026-07-31 | review-feedback | 评论 `3684325560`：跨租户拒绝审计改为 caller tenant owning，target tenant 仅作 detail |
| 2026-07-31 | security | Missing/foreign MCP Tool 收敛为不带 Tool name 的相同通用错误，避免存在性枚举 |
| 2026-07-31 | tests | 补 Member invalid capability、API Key caller、cross-tenant Owner/Admin、revoked/expired、foreign Tool 对照 E2E |
| 2026-07-31 | CI | 为 PR 接入 frontend install/type-check/build + backend fmt/check/clippy/workspace test 门禁；最终配置 `contents: read` |
| 2026-07-31 | validation | Frontend install/type-check/build、Rust fmt/check/clippy 已在 GitHub Actions 通过 |
| 2026-07-31 | bugfix | Workspace test 暴露 `RbacError` 默认 status_code=500；补显式 401/403 映射 |
| 2026-07-31 | validation | 最终只读 CI run `30567361095` 全绿，包含完整 `cargo test --workspace` |
| 2026-07-31 | merge | PR #3 已合并，merge commit `6de7845e1231efc04f94f16cb9ab0a410f6ad2d9`；EVO-118-B Done，Iteration 051 Closed / Complete |

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|
| 2026-07-31 | review correction | 接受 | 调整 Create extractor/授权顺序、审计归属、MCP 隐藏语义、RBAC 状态映射和测试矩阵 | 保持同一 Story/PR；不扩展到 SSRF/Agent Session |

## 10. Review

- 完成：两条 inline review comment 与 reviewer 测试矩阵均已落实；实现、前端、Reference 和 PR CI 已同步。
- 已通过：frontend install/type-check/build；backend fmt/check/clippy；完整 workspace tests；reviewer 边界复验。
- 未完成：无；后续工作已归口到 EVO-118-C 及既有 Agent/legacy Key Story。
- 闭环状态：`Complete`
- 残余归口：下一 Story 为 EVO-118-C；本 Iteration 不包含其实现。

## 11. Retrospective

- 做得好的：review comment 触发了 auth-before-parse、caller-owned audit、hidden resource response 和真实 HTTP status 的运行验证，而非只改测试期望。
- 需要调整的：安全 Story 应在最初就为 PR 接入可运行门禁，避免静态审查代替行为证据；该能力由本 PR 建立，后续 EVO-118-G 再做平台化收敛。
- 写入 EVOLUTION：新增经验——实现 `ResponseError::error_response()` 时必须同步覆盖 `status_code()`，否则 middleware/error-object 路径可能仍报告 500。