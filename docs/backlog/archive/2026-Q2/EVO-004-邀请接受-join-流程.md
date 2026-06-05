# EVO-004 邀请接受 / Join 流程

> Archived backlog item. Routing index: [2026 Q2 Archive](INDEX.md). Active routing surface: [Product Backlog](../../PRODUCT-BACKLOG.md).

- Type: feature
- Status: Done
- Priority: P0
- Source: Iteration 005
- Decision Context: handler 实现，cargo test 通过

#### Source Detail Snapshot

- 类型：feature
- 优先级：P0
- 状态：Done
- 用户价值或技术目标：被邀请的用户可以通过邮件中的公开 `/join` 链接注册账号并自动加入租户，完成邀请闭环。
- 范围：
  - 实现规范化公开入口 `POST /api/v1/invitations/accept`，并保留 tenant-scoped join 兼容入口。
  - `accept_invitation` 验证 token → 检查过期/邮箱冲突 → 创建用户（Argon2id）→ 标记邀请为 accepted → 分配租户成员角色 → 返回 JWT token。
  - 利用已有的 `InvitationRepository::find_by_token` 查找邀请。
  - 利用已有的 `UserRepository::create` 创建用户。
  - `invite_member` 通过 Mailer 发送指向 `APP__PUBLIC_URL` 的 `/join` 邀请链接。
  - 增加 SPA `/join` 页面和中英文文案。
- 不做：
  - 不修改 DTO（`AcceptInviteRequest { token, password, username }` 已定义）。
- 验收标准：
  - [x] `POST /api/v1/invitations/accept` 接受 token + username + password，创建用户并加入租户。
  - [x] Token 过期或已使用返回明确错误。
  - [x] 同邮箱已注册时返回 `EMAIL_EXISTS` 冲突错误。
  - [x] 成功后返回 JWT token（用户可直接使用系统）。
  - [x] 前端公开 `/join` 路由和邀请邮件链接对齐。
  - [x] `cargo test -p api` 覆盖公开入口冲突路径、RBAC public path 和 CSRF exempt path。
- 技术备注：
  - `InvitationRepository` trait 已有 `find_by_token` 方法。
  - `AcceptInviteRequest` DTO 已定义（token, password, username）。
  - `invite_member` handler 已实现，会生成 token 并通过 Mailer 发送邀请邮件。
  - 规范化路由已注册：`/invitations/accept`；`/tenant/{tenant_id}/members/join` 保持兼容。
- 依赖：Mailer（已有）、EVO-003（可共用 token 验证模式，但无硬依赖）。
- 影响范围：backend
- 最小验证方式：`cargo test -p api` 覆盖 accept_invitation 端到端。

## 待细化故事
