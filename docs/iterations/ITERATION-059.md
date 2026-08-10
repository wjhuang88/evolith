# Iteration 059: Production Dependency Fail Closed

> 文档状态：Closed / Complete
> 计划目标：完成 EVO-118-G-D，使 SMTP、Redis 和安全状态按生产职责 fail closed。

## 1. 发布计划基线

- 故事：[EVO-118-G-D](../backlog/active/EVO-118-G-D-production-dependency-fail-closed.md)
- 依赖：EVO-118-G-C Done / Complete。
- 不做：Durable Outbox 和最终 production deployment smoke，分别归 H/E。

## 2. 验收与验证

- 生产启用 SMTP 但配置无效时启动失败；开发可显式 ConsoleMailer fallback。
- ConsoleMailer 不记录完整 token 或可直接使用的链接。
- 生产 Redis 不可用时启动失败；开发普通 cache 可显式内存降级。
- caller-aware limiter 独立于普通 cache，middleware 缺失时不放行。
- 验证 production/development 配置矩阵、infra 专项、workspace 门禁和文档一致性。

## 3. 执行记录

| 日期 | 类型 | 记录 |
| --- | --- | --- |
| 2026-08-08 | activation | Iteration 058 Closed / Complete；激活 G-D。 |
| 2026-08-08 | validation | Infra 20 tests、workspace check、strict Clippy 与文档链接检查通过；生产 SMTP/Redis 负向和开发 fallback 矩阵通过。 |
| 2026-08-08 | closure | G-D Done / Complete；H/E 仍按依赖顺序保留。 |

## 4. 闭环台账

| 项目 | 记录 |
| --- | --- |
| 产物 | SMTP/Redis factory fail-closed、ConsoleMailer 脱敏、配置矩阵测试。 |
| 状态同步 | G-D、父 G、Backlog、Board、Iteration Index、CONFIG/Architecture。 |
| 残余 | Durable event -> H；最终 deployment smoke -> E。 |
| 状态 | `Complete`。 |
