# Iteration 009: 邮箱验证闭环

> 状态：Planned，未启动
> 计划目标：完成 Phase C 剩余的邮箱验证闭环，使注册、密码恢复、邀请与邮箱确认形成一致的用户生命周期路径。

## 1. 计划边界

本计划仅编排未来工作，不将 backlog story 改为 `In Progress`。启动本轮前必须先确认
`EVO-018` 已补齐详情并满足 DoR。

## 2. 候选故事

| ID | 标题 | 所属 Epic | 优先级 | 当前状态 | 启动门禁 |
|----|------|-----------|--------|----------|----------|
| EVO-018 | 邮箱验证发送与确认闭环 | 无 | P1 | Proposed | 细化验收、公开链接与验证方案后置为 Ready |

## 3. 目标范围

- 实现 `send-verify` 与 `verify-email` 对应的后端能力和前端使用路径。
- 邮件公开链接继续使用 `APP__PUBLIC_URL`，与现有 reset / invite 规则一致。
- 补齐公开入口的 RBAC、CSRF 与 API contract 验证。

## 4. 不做事项

- 不进入 Skill 生命周期功能（EVO-006 / EVO-009）。
- 不启动前端嵌入后端发布物（EVO-016）。
- 不创建 GitHub CI/CD workflow（EVO-030）。

## 5. 计划验收标准

- [ ] 邮箱验证发送和确认 API 不再返回 501，并有可复现测试。
- [ ] 用户收到的验证链接落到公开前端路由，使用正确外部 origin。
- [ ] API 合约、RBAC / CSRF 例外与前端页面保持一致。
- [ ] 风险匹配的后端和前端验证命令被逐项记录。

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
