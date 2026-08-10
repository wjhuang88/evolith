# Iteration 058: Caller-aware Rate Limits

> 文档状态：Closed / Complete
> 计划目标：完成 EVO-118-G-C，让匿名、JWT 和 API Key 请求按稳定调用者身份独立计数。

## 1. 发布计划基线

- 故事：[EVO-118-G-C](../backlog/active/EVO-118-G-C-caller-aware-rate-limits.md)
- 依赖：EVO-118-G-B Done / Complete。
- 不做：多实例 Redis-backed 计数、Session 或分布式锁；归 EVO-118-G-D。

## 2. 验收与验证

- 匿名按 peer IP、JWT 按 user ID、API Key 按 key ID 分桶。
- 同一 IP 的不同已认证调用者互不消耗配额。
- API Key 使用单 Key `rate_limit` 与全局 API Key 上限中的较小值。
- 超限返回 429，窗口结束后恢复；认证失败仍由 RBAC 拒绝。
- 执行 middleware 集成矩阵、Backend 全量门禁、API Contract 和配置文档检查。

## 3. 威胁模型

| 项目 | 内容 |
| --- | --- |
| 资产 | API 可用性、调用配额、公平性 |
| 调用者 | 匿名、JWT、API Key、共享 NAT 用户 |
| 入口 | 全局 HTTP middleware |
| 失败模式 | 身份串桶、Key 限额绕过、未知身份 fail open |
| 默认 | 身份解析后限流；无身份按 peer IP；存储错误不得放行 |

## 4. 执行记录

| 日期 | 类型 | 记录 |
| --- | --- | --- |
| 2026-08-08 | activation | Iteration 057 Closed / Complete；激活 G-C。 |
| 2026-08-08 | validation | Caller-aware 专项 5/5、API Key DTO 5/5、workspace check、strict Clippy 与授权环境 Git clone/push/pull 集成测试通过。 |
| 2026-08-08 | closure | G-C Done / Complete；多实例 Redis-backed 安全状态归 G-D。 |

## 5. 闭环台账

| 项目 | 记录 |
| --- | --- |
| 产物 | caller identity、共享 limiter、请求链接线、隔离/429/恢复测试。 |
| 状态同步 | G-C、父 G、Backlog、Board、Iteration Index、API Contract、CONFIG。 |
| 残余 | 多实例 Redis fail-closed 归 G-D。 |
| 状态 | `Complete`。 |
