# EVO-055 前后端 API 契约漂移修复

> Archived backlog item. Routing index: [2026 Q2 Archive](INDEX.md). Active routing surface: [Product Backlog](../../PRODUCT-BACKLOG.md).

- Type: bug
- Status: Done
- Priority: P1
- Source: 跨层一致性审查 2026-06-01 / Iteration 035
- Decision Context: 2026-06-01 完成：删除 `membersApi.acceptInvitation` 死分支 / `billing/page.tsx` 加 `BILLING_ENABLED` 闸门 + 「待计费」静态页 / `cliInterfacesApi.update` 改 `throw new Error` 指向 create+delete / `API-CONTRACT.md` Billing 段补「待计费」标注

#### Source Detail Snapshot

- 类型：bug
- 优先级：P1
- 状态：Done
- 父 Epic：无
- Story 形态：API
- 用户故事：作为使用 SPA 的租户成员，我希望邀请接受、CLI 接口更新和计费相关操作不会因前端调用了后端不存在的路径而静默失败（404/501），以便这些功能真实可用。
- 背景（跨层一致性审查 2026-06-01 确认）：
  - `frontend/src/lib/api/members.ts:42` 调用 `POST /auth/accept-invite`，后端真实路由为 `POST /api/v1/invitations/accept`（`auth.ts:87` 已有正确实现，`membersApi.acceptInvitation` 为死分支）。
  - `frontend/src/lib/api/billing.ts` 与 `components/billing/PaymentMethods.tsx` 调用 8 个 billing/payment-method 端点（create/update/cancel subscription、invoices、payment-methods 全套），后端无对应路由。
  - `frontend/src/lib/api/cli-interfaces.ts:78` 调用 `PUT /snippets/{id}`，后端返回 501（`SnippetRepository` trait 无 `update`）。
- 范围（本次做）：
  1. 删除/修正 `membersApi.acceptInvitation` 死分支，统一走 `/invitations/accept`。
  2. 对无后端实现的 billing/payment-method 端点：在前端禁用入口并标注「待计费迭代」，或补后端 stub 路由返回结构化「未实现」——二选一，refinement 时定。
  3. CLI 接口更新：前端在 `PUT /snippets/{id}` 返回 501 时给出明确不可用提示；真实 update 能力归 EVO-049/后续。
  4. 同步 `docs/reference/API-CONTRACT.md`，纠正 skills list 等响应 shape 描述与真实路由。
- 不做：
  - 不实现真实计费业务逻辑（billing 仍为已知 stub，归计费迭代）。
  - 不实现 `SnippetRepository::update`（归 EVO-049/后续）。
- 验收标准：
  - [ ] 前端不存在调用 `/auth/accept-invite` 的代码路径；邀请接受走 `/invitations/accept` 并成功。
  - [ ] 前端不再向无后端实现的 billing/payment-method 端点发起请求（禁用或补 stub），无新增 404。
  - [ ] CLI 接口更新在不支持时给出明确提示，不再静默 501。
  - [ ] `docs/reference/API-CONTRACT.md` 与真实路由/响应 shape 一致。
- 依赖或阻塞：billing 端点处置方向需与计费迭代规划协调。
- 解锁内容：消除 SPA 关键流程的功能性 404，恢复邀请/接口可用性。
- 影响范围：frontend / backend(api) / docs
- 最小验证方式：手动走通邀请接受流程；`rg "/auth/accept-invite" frontend/` 无结果；前端构建通过；契约文档比对。
