# Iteration 051: API Key 与 MCP 授权边界硬化

> 文档状态：Review  
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
- MCP `tools/call` 必须由包含 `execute` 的有效 API Key 调用。
- 历史 `write/admin/commit:*` Key 保持 Repo 运行兼容，但不可新签发，并给出轮换策略。
- 权限负向测试、前端能力选择和 Reference/Contract 同步完成。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|-----------|--------|---------------|
| EVO-118-B | API Key 与 MCP 授权边界硬化 | EVO-118 | P0 | EVO-118-A Done；Security Review SOP 已生效 |

## 3. 发布计划基线：不做事项

- 不实现 Agent Session、Refresh Token、Branch/Path scoped token。
- 不实现 HTTP Tool SSRF/Egress 校验；归 EVO-118-C。
- 不修改 API Key 数据库 schema。
- 不删除或自动迁移已有 legacy Key。
- 不改变 Git Smart HTTP 的 legacy compatibility 行为。

## 4. Story / BDD 适用性

EVO-118-B 为 Permission / Security Story，使用 Given/When/Then 负向行为验收：

- Member list/create/revoke API Key → 403 + 安全审计。
- API Key 调用 API Key management → 403。
- Owner/Admin 创建 canonical capability Key → 成功。
- Owner/Admin 创建 `admin` / `manage_keys` / `write` / `commit:*` / unknown → 400，无 DB 记录。
- `read` / `repo:read` Key 调用 MCP `tools/call` → MCP 授权错误，executor 不执行。
- `execute` Key 调用同租户 Tool → 保持成功。

## 5. 发布计划基线：计划验证

```bash
cd backend
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace

cd ../frontend
bun run type-check
bun run build
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
| Typed DTO 使已有客户端提交 legacy token 失败 | 这是新签发的预期 fail-closed；授权合约和前端同步 canonical 列表 |
| 历史 Key 被意外失效 | scope helper 保留 `write/admin/commit:*` Repo 读取兼容；只禁止新签发 |
| MCP discovery 被误阻断 | `initialize` / `tools/list` 保持原行为，仅 `tools/call` 强制 `execute` |
| 审计写入失败影响授权 | 授权仍 fail-closed；审计失败仅记录 error，不把拒绝变成成功 |
| 回滚 | 回退本 PR；无 migration 和数据变更 |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 按计划开发 EVO-118-B 并通过 PR 提交 |
| 产物 | Domain capability、授权 helper、API/MCP handler、拒绝审计、测试、前端和文档 |
| 状态同步归口 | EVO-118/B、Iteration 051、Product Backlog、Board、Permissions/API Key Authorization Contract |
| Story/BDD 归口 | Permission / Security；负向 Given/When/Then |
| 验证证据 | GitHub compare、文件级静态核对、新增测试代码；运行门禁待本地/CI |
| 残余工作归口 | 本 PR 的 Rust/前端门禁；SSRF → EVO-118-C；Agent token → EVO-105/106；legacy Key 轮换 → Permissions Reference |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-07-30 | activation | Iteration 050 已关闭；选择 EVO-118-B，状态更新为 In Progress |
| 2026-07-30 | progress | 新增 Typed `ApiKeyCapability` 与统一 `ApiKeyAction` helper；保留 legacy Repo compatibility |
| 2026-07-30 | progress | API Key list/create/revoke 限定同租户 Owner/Admin JWT；拒绝写安全审计 |
| 2026-07-30 | progress | MCP `tools/call` 在 Tool 查询/executor 前强制 `execute`，无权限返回 `-32003` |
| 2026-07-30 | progress | 新增安全 E2E、前端 capability 选择、Permissions 与授权子合约 |
| 2026-07-30 | navigator | 修复无请求体 DELETE 契约和 MCP `prompts/list` 空列表响应，避免无关回归 |
| 2026-07-30 | validation | Compare 0 behind；只含 EVO-118-B backend/frontend/tests/docs，无 migration/deploy/scripts |
| 2026-07-30 | review | Draft PR #3 已创建；因 hard-required 运行门禁未执行，状态保持 Review / Partial |

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|
| - | - | - | - | - |

## 10. Review

- 完成：授权模型、API Key 管理门禁、拒绝审计、MCP execute 检查、安全测试代码、前端和 Reference 已形成并提交 Draft PR #3。
- 未完成：当前环境无私有仓库本地 checkout，仓库 CI 为 tag-only；`cargo fmt/clippy/test` 与 `bun type-check/build` 未实际执行。
- 验证结果：静态接口和分支 compare 已通过；hard-required 运行门禁没有证据，不能勾选行为验收或标记 Done。
- 闭环状态：`Partial`
- 残余归口：PR #3 在可运行环境补齐全部门禁；全部通过并合并后关闭 EVO-118-B/Iteration 051，随后启动 EVO-118-C。

## 11. Retrospective

- 做得好的：把权限字符串收敛为 Typed issuance + Action authorization，并用 executor hit count 证明拒绝发生在副作用之前。
- 需要调整的：tag-only CI 无法为普通 PR 提供强制门禁，后续由 EVO-118-G 建立 PR/Main CI。
- 写入 EVOLUTION：本轮没有新增超出既有 Security Review/Production Gate 的通用陷阱。