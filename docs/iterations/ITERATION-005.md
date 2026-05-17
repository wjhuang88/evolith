# Iteration 005: 认证闭环 — 忘记密码与邀请接受（完成）

> 时间：2026-05-17
> 目标：完成 EVO-003（忘记密码/重置密码）和 EVO-004（邀请接受/Join），实现用户生命周期关键流程闭环。

## 1. 本轮目标

- 实现忘记密码 → 邮件发送重置链接 → 重置密码的完整流程。
- 实现邀请链接 → 注册并加入租户的完整流程。
- 两个 501 handler 改为真实实现。

## 2. 选入故事

| ID | 标题 | 优先级 | 负责人 | 状态 |
|----|------|--------|--------|------|
| EVO-003 | 忘记密码与重置密码闭环 | P0 | Agent | Done |
| EVO-004 | 邀请接受 / Join 流程 | P0 | Agent | Done |

## 3. 不做事项

- 不实现邮箱验证发送（EVO-018，P1，后续迭代）。
- 不修改 DTO 结构。
- 不创建或恢复 GitHub CI/CD workflow（EVO-030）。

## 4. 验收标准

- [x] `POST /api/v1/auth/forgot-password` 接受邮箱，存在时发送重置邮件，不存在时静默返回成功。
- [x] `POST /api/v1/auth/reset-password` 接受 token + 新密码，验证通过后更新密码。
- [x] Password reset token 有效期 1 小时，过期返回明确错误。
- [x] `POST /api/v1/tenant/{tenant_id}/members/join` 接受 token + username + password，创建用户并加入租户。
- [x] 邀请 token 过期/已使用返回明确错误。
- [x] `cargo test -p api` 通过（34 passed）。
- [x] Playwright 验证前端忘记密码 → 重置密码 → 新密码登录全流程。

## 5. 验证结果

```
cargo test -p api        → 34 passed, 0 failed
cargo check --workspace  → 0 errors
npm run type-check       → 0 errors
Playwright: forgot-password → "Check your email" → reset-password → "Password Reset Complete" → login with new password → success
```

## 6. 风险与回滚

| 风险 | 处理 |
|------|------|
| Token 安全性 | crypto-random 生成，防邮箱枚举（始终返回成功） |
| 邮件未配置 | ConsoleMailer 打印 token 到日志，开发环境可用 |

## 7. 执行记录

| 日期 | 记录 |
|------|------|
| 2026-05-17 | Iteration 005 started. 选入 EVO-003 和 EVO-004，补齐 backlog 详情块。 |
| 2026-05-17 | 发现 UserRepository 已有 set_reset_token/find_by_reset_token/clear_reset_token 方法，migration 已有 reset_token 列，无需新建表或 trait。 |
| 2026-05-17 | EVO-003 Done：forgot_password + reset_password handler 实现。前端 forgot-password 和 reset-password 页面 API 调用从 TODO 改为真实调用。 |
| 2026-05-17 | EVO-004 Done：accept_invitation handler 实现（验证 token → 创建用户 → 加入租户 → 返回 JWT）。 |
| 2026-05-17 | Playwright 全流程验证通过。 |

## 8. 变更请求

无。

## 9. Review

- 完成：EVO-003/004 全部完成，用户生命周期关键流程闭环。
- 未完成：EVO-018 邮箱验证（P1，不在本次迭代）。
- 验证结果：cargo test 34 passed，Playwright forgot→reset→login 全通过。

## 10. Retrospective

- 做得好的：实现前先检查已有 infrastructure（repository trait、migration、mailer），发现不需要新建任何基础设施。
- 需要调整的：前端 reset-password 和 forgot-password 的 API 调用之前被注释为 TODO，应更早发现并接入。
- 写入 EVOLUTION：UserRepository 已有完整的 reset token 方法（set_reset_token/find_by_reset_token/clear_reset_token），不需要独立 password_reset_tokens 表。
