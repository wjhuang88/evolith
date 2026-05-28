# Iteration 009: 邮箱验证闭环

> 状态：Closed
> 计划目标：完成 Phase C 剩余的邮箱验证闭环，使注册、密码恢复、邀请与邮箱确认形成一致的用户生命周期路径。
> 完成日期：2026-05-27
>
> 收口说明（2026-05-27）：Iteration 021 / EVO-040 已修复 `API-CONTRACT.md`、`TESTING.md`
> 与 `IMPLEMENTATION-ROADMAP.md` 中的状态漂移。`cargo test -p api` → 48 passed, 0 failed，
> 含 `test_send_verification_email_returns_success`、`test_verify_email_with_valid_token`、
> `test_verify_email_with_invalid_token` 三个邮箱验证 e2e 测试。所有验收标准满足，迭代关闭。

## 1. 计划边界

EVO-018 已补齐详情块并满足 DoR。本轮实施 `send-verify` 和 `verify-email` 两个 handler。

## 2. 候选故事

| ID | 标题 | 所属 Epic | 优先级 | 当前状态 | 启动门禁 |
|----|------|-----------|--------|----------|----------|
| EVO-018 | 邮箱验证发送与确认闭环 | 无 | P1 | Done | Done |

## 3. 目标范围

- 实现 `send-verify` 与 `verify-email` 对应的后端能力和前端使用路径。
- 邮件公开链接继续使用 `APP__PUBLIC_URL`，与现有 reset / invite 规则一致。
- 补齐公开入口的 RBAC、CSRF 与 API contract 验证。

## 4. 不做事项

- 不进入 Skill 生命周期功能（EVO-006 / EVO-009）。
- 不启动前端嵌入后端发布物（EVO-016）。
- 不创建 GitHub CI/CD workflow（EVO-030）。

## 5. 计划验收标准

- [x] 邮箱验证发送和确认 API 不再返回 501，并有可复现测试。
- [x] 用户收到的验证链接落到公开前端路由，使用正确外部 origin（`APP__PUBLIC_URL`）。
- [x] API 合约、RBAC / CSRF 例外与前端页面保持一致（`send-verify` 已加入公开路径和 CSRF 豁免）。
- [x] 风险匹配的后端和前端验证命令被逐项记录。

## 6. 计划验证

```bash
cargo test -p api
cargo test --workspace
bun run type-check
bun run build
```

同时验证公开验证链接、CSRF / RBAC 边界和邮件 URL 配置。

## 7. 风险与进入条件

| 风险 | 计划控制 |
|------|----------|
| 公开 token 入口被错误拦截或放行过度 | 在启动前把 RBAC / CSRF 路径纳入验收 |
| 邮件链接使用内部后端地址 | 固定检查 `APP__PUBLIC_URL` 生成结果 |
| 候选项仍只有总表记录 | 启动前回到需求进入 SOP 补齐 EVO-018 详情块 |

## 8. 计划记录

| 日期 | 记录 |
|------|------|
| 2026-05-26 | Future iteration planned only. 候选 EVO-018；未启动、未改变 backlog 状态。 |
| 2026-05-27 | Iteration 009 started. EVO-018 补齐 DoR 详情块，基础设施（DTO/repo/mailer）已就绪，只需实现两个 handler。 |
| 2026-05-27 | EVO-018 完成。`send_verification_email` + `verify_email` handler 实现，CSRF/RBAC 路径已更新，3 个 e2e 测试通过。`cargo test -p api` → 18 passed。 |
| 2026-05-27 | Inventory audit: 代码与 e2e 记录存在，但 API contract / testing / roadmap reference 仍声明相关接口未实现或为旧状态；登记 EVO-040，迭代转为 `Review` 待真实收口。 |
| 2026-05-27 | EVO-040 / Iteration 021 完成参考文档修复。`API-CONTRACT.md` 中 `send-verify` / `verify-email` 从 `⚠️ Not implemented (501)` 更新为完整响应描述。`TESTING.md` TC-ATH-011 从 `501` 更新为 `200`。`IMPLEMENTATION-ROADMAP.md` Phase C 邮箱验证状态更新为 Done。`cargo test -p api` → 48 passed, 0 failed。迭代转为 `Closed`。 |

## 10. Review

- 完成：EVO-018 邮箱验证发送与确认闭环 — 两个 handler 实现、3 个 e2e 测试、RBAC/CSRF 路径更新、API contract / testing / roadmap 参考文档收口。
- 未完成：无。
- 验证结果：`cargo test -p api` → 48 passed, 0 failed（含 `test_send_verification_email_returns_success`、`test_verify_email_with_valid_token`、`test_verify_email_with_invalid_token`）。
- 闭环状态：`Complete`
- 残余归口：无。

## 11. Retrospective

- 做得好的：handler 实现与测试在一次迭代中完成；收口缺口通过 EVO-040 明确归口并闭环。
- 需要调整的：实现完成后应立即同步参考文档，避免 Review 漂移。
- 写入 EVOLUTION：参考文档状态漂移是迭代收口的常见陷阱；实现完成后需同步更新 API contract / testing / roadmap。
