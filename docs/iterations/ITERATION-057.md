# Iteration 057: Runtime Readiness

> 文档状态：Closed / Complete
> 计划目标：完成 EVO-118-G-B，证明运行态 readiness 真实反映数据库和 Git Storage。
> 最终生产构建与部署仍归 EVO-118-E，按 ADR-0010 在所有产品开发和 legacy cleanup 完成后执行。

## 1. 发布计划基线

- 故事：[EVO-118-G-B](../backlog/active/EVO-118-G-B-runtime-readiness.md)
- 依赖：EVO-118-G-A 已完成本地门禁；远端 Branch Protection 仍为外部 residual。
- 不做：最终 production Compose build/startup、发布 Smoke。

## 2. 验收与验证

- `/health/live` 始终反映进程存活并返回 200。
- `/health/ready` 在 DB 或 Git Storage 不可用时返回 503，恢复后返回 200，并返回逐项检查结果。
- Dockerfile 和 production Compose backend healthcheck 使用 `/health/ready`。
- 验证：API health 测试、workspace 测试、Docker/Compose 静态检查、开发运行态 smoke。

## 3. 执行记录

| 日期 | 类型 | 记录 |
| --- | --- | --- |
| 2026-08-08 | activation | Iteration 056 收口为 Partial；按依赖激活 G-B。 |
| 2026-08-08 | validation | Health 单元测试 10/10；默认 Git Storage 缺失时 live=200、ready=503，配置可写目录后 ready=200；Dockerfile/Compose 静态检查通过。 |
| 2026-08-08 | closure | G-B Done / Complete；最终 production build/startup 仍由 EVO-118-E 承担。 |

## 4. 闭环台账

| 项目 | 记录 |
| --- | --- |
| 产物 | readiness 路由、依赖探针、容器 healthcheck 和故障测试。 |
| 残余 | 最终生产部署归 EVO-118-E。 |
| 状态 | `Complete`。 |
