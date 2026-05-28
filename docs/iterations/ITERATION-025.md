# Iteration 025: 租户设置与审计详情补齐

> 文档状态：Planned / Blocked for activation
> 计划发布日期：2026-05-28
> 计划目标：用一周补齐 Phase F 中最小管理后台闭环：租户设置保存与审计日志详情接口。
>
> 基线保护：本文件一旦提交，以下“发布计划基线”内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

- 将 `EVO-012` 和 `EVO-013` 从总表 Proposed 细化为可执行 story。
- 后端补齐 audit log detail contract，前端租户设置页不再只停留在 TODO/mock。
- 控制在一周内，只做管理后台最小可用闭环，不扩展 billing 或 quota。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|---------|--------|---------------|
| EVO-012 | 租户设置保存 | 无 | P2 | 激活前补齐详情块、字段范围和权限验收 |
| EVO-013 | Audit log detail 接口 | 无 | P2 | 激活前确认 route/path 与 API contract 当前 501 位置 |

## 3. 发布计划基线：不做事项

- 不修改计费、配额、Stripe webhook。
- 不重做 members / api keys 已完成页面。
- 不改变 tenant identity / subdomain 模型。

## 4. 发布计划基线：计划验收标准

- Story 格式与 BDD 适用性：
  - [ ] EVO-012 作为 Product/API Story 补齐角色、目标、价值和 Given/When/Then。
  - [ ] EVO-013 作为 API Story 补齐权限、存在/不存在、跨租户访问和错误映射场景。
- [ ] 租户设置保存字段、权限和成功/失败反馈明确。
- [ ] `GET /api/v1/tenant/{tenant_id}/audit-logs/{log_id}` contract 与实现计划一致。
- [ ] 前后端最小验证命令和手工检查路径明确。

## 5. 发布计划基线：计划验证

```bash
cargo test -p api
cargo test -p infra
cd frontend
bun run type-check
bun run build
```

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| 设置字段范围不清导致扩大实现 | 激活前只选最小字段集合，其他设置另建 story |
| 审计详情跨租户越权 | BDD 场景必须覆盖跨租户访问与不存在记录 |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 一周计划：补齐租户设置保存与审计详情最小闭环 |
| 产物 | 后端接口、前端设置保存、API contract/testing reference 更新 |
| 状态同步归口 | EVO-012、EVO-013、Iteration 025、API contract |
| Story/BDD 归口 | Product/API Story；激活前补齐 BDD |
| 验证证据 | api/infra 测试、frontend type-check/build、关键页面手工验证 |
| 残余工作归口 | billing/quota 进入 Iteration 026；更多设置项另建 backlog |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-05-28 | planning | 发布一周计划基线；候选仍需 DoR refinement。 |

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|

## 10. Review

- 完成：
- 未完成：
- 验证结果：
- 闭环状态：`Blocked`（仅计划发布，候选需 refinement）
- 残余归口：激活门禁见第 4 节。

## 11. Retrospective

- 做得好的：
- 需要调整的：
- 写入 EVOLUTION：
