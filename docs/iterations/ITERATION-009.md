# Iteration 009: 邮箱验证闭环

> 状态：Review（实现记录存在，参考文档与验证收口待核对）
> 计划目标：完成 Phase C 剩余的邮箱验证闭环，使注册、密码恢复、邀请与邮箱确认形成一致的用户生命周期路径。
> 完成日期：2026-05-27
>
> 状态审计说明（2026-05-27）：本页记录 handler 与 e2e 已完成，但当前
> `API-CONTRACT.md` / `TESTING.md` / roadmap 仍将邮箱验证接口描述为未实现或旧状态。
> 已登记 EVO-040；完成修复和复验前，本迭代不维持 `Done` 声明。

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
