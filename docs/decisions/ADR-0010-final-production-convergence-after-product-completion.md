# ADR-0010: Final Production Convergence After Product Completion

## 状态

Accepted（2026-08-08）

## 背景

EVO-118-E 同时验证 Embedded Frontend 最终产物、生产 Docker/Compose、Gateway 路径、完整协议 Smoke 和发布配置。此前路线把它作为 Repo UI、Agent loop 和 Discovery 的开发前置 Gate。

项目尚未上线，目标页面、API、事件边界和 legacy runtime 仍将持续变化。此时先冻结最终镜像与生产 Compose，会让后续每个产品切片重复修改和重验部署产物，也把“DEPLOY-01 未关闭前不得上线”错误扩大为“DEPLOY-01 未关闭前不得继续开发”。

## 选项

1. 维持 EVO-118-E 为下一 Story，并继续阻塞普通产品开发。
2. 拆出临时构建 Story，最终再执行第二次部署收敛。
3. 保留 EVO-118-E 的完整验收边界，将其移动到目标 MVP 开发和 legacy cleanup 完成之后。

## 决策

选择选项 3：

- EVO-118-E 是最终生产构建与部署收敛步骤，在目标 MVP 产品能力、可靠性边界和 legacy cleanup 完成后执行。
- DEPLOY-01 始终保持生产发布 Gate；关闭前不得发布 External Alpha、生产环境或声明生产就绪。
- DEPLOY-01 不再作为 Repo UI、Onboarding、Settings、Agent、Discovery 或清理工作的开发前置条件。
- EVO-118-F/G/H 先完成数据一致性、运行可靠性和持久事件边界；运行时状态由 Board 与 Iteration owner 管理。
- 最终构建仍必须从 clean checkout 完成 Embedded Frontend 单一镜像、Compose/Gateway、API/Git/MCP/SPA/Worker 路径和完整 Smoke，不缩减原 EVO-118-E 的发布验收。

## 后果

### 正向

- 最终生产镜像验证稳定的产品面，减少反复修改 Docker/Compose 和重复 Smoke。
- 产品开发不再被尚无必要关闭的最终发布 Gate 阻塞。
- “可以继续开发”和“允许上线”成为两个清晰状态。

### 约束与风险

- 开发环境和 CI 仍必须保持可构建、可测试；不得把明显构建破坏全部推迟到 EVO-118-E。
- 任何安全、数据损坏或基础编译失败仍可作为 P0 插队，不因本 ADR 延后。
- EVO-118-E 启动前必须重新盘点全部目标 route、API、worker、migration、配置和发布脚本，禁止引用早期局部 Smoke 代替 final-head 证据。

## 变更控制

- 类型：`replan`，同时修正部署 Gate 的产品边界。
- 原因：用户明确 EVO-118-E 应在改造和开发完成后执行。
- 当前无 Active runtime Iteration，未覆写已发布 Planned Iteration；历史 Iteration 事实保持不变。
- 半成品处理：本轮无 EVO-118-E 产品代码，只调整未来 Story 依赖和当前路线基线。

## 相关链接

- [EVO-118-E Final Production Convergence](../backlog/active/EVO-118-E-production-build-deployment-convergence.md)
- [EVO-118 Production Readiness](../backlog/active/EVO-118-production-readiness-and-security-hardening.md)
- [Production Readiness Plan](../roadmap/PRODUCTION-READINESS-PLAN-2026-07.md)
- [Production Readiness Baseline](../reference/PRODUCTION-READINESS-BASELINE.md)
