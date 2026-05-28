# Iteration 026: Stripe Webhook 与计费闭环恢复

> 文档状态：Planned / Blocked for activation
> 计划发布日期：2026-05-28
> 计划目标：用一周恢复 `EVO-014` Stripe webhook 最小闭环，明确支付事件、订阅状态和
> 本地可控测试策略。
>
> 基线保护：本文件一旦提交，以下“发布计划基线”内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

- 恢复 Stripe webhook handler 的最小可用路径。
- 明确签名校验、事件幂等、订阅状态同步和失败重试记录。
- 不扩展复杂计费策略，只让计费闭环具备可验证基础。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|---------|--------|---------------|
| EVO-014 | Stripe webhook 恢复 | 无 | P2 | 激活前补齐事件范围、签名校验和本地 mock 验收 |

## 3. 发布计划基线：不做事项

- 不实现完整套餐管理 UI。
- 不设计新的配额计费模型；如需要另拆 story。
- 不依赖真实 Stripe 网络回调作为唯一验收。

## 4. 发布计划基线：计划验收标准

- Story 格式与 BDD 适用性：
  - [ ] API Story 补齐合法签名、非法签名、重复事件、未知事件和状态同步场景。
- [ ] Webhook route、CSRF/RBAC 例外和签名校验边界明确。
- [ ] 事件幂等和审计/日志记录策略明确。
- [ ] 本地 mock payload 覆盖成功、失败和重复事件。

## 5. 发布计划基线：计划验证

```bash
cargo test -p service-payment
cargo test -p api
cargo test --workspace
```

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| Webhook 被误放开且无签名校验 | 激活前将签名校验和 CSRF/RBAC 例外作为 BDD 验收 |
| 依赖真实 Stripe 环境导致测试不稳定 | 使用本地构造 payload 和签名验证测试 |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 一周计划：恢复 Stripe webhook 最小计费闭环 |
| 产物 | webhook handler、事件处理测试、API/reference 更新 |
| 状态同步归口 | EVO-014、Iteration 026、API contract、testing reference |
| Story/BDD 归口 | API Story；激活前补齐 BDD |
| 验证证据 | service-payment/api/workspace 测试 |
| 残余工作归口 | 套餐 UI、配额策略和高级 billing 能力另建 backlog |

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
