# EVO-118 Epic: Production Readiness and Security Hardening

- **类型**：Epic / Security / Reliability
- **状态**：In Progress
- **优先级**：P0
- **触发**：2026-07-30 全面项目体检
- **父 Epic**：无

## 总体目标

在继续 Repo UI、Agent Session、Commit/Promote、Webhook 和 Indexer 主线前，关闭已确认的权限、SSRF、Git 数据耐久性、生产构建和运行可靠性缺口，使 Evolith 从“Git 后端 Alpha 基础”进入可安全推进外部 Alpha 的状态。

本 Epic 不替代 [EVO-100 Git-Centric Platform Foundation](EVO-100-git-centric-platform-foundation.md)。它作为 Phase E'-1 与 Repo UI/Agent 集成之间的稳定化门禁，完成后恢复 EVO-112 → EVO-105/106/107/104 → EVO-108/109/110 的产品顺序。

## 当前判断

- 模块化单体、Git Smart HTTP + `gix`、双数据库和 Embedded Frontend 的总体方向保留。
- Git 后端基础已可用于受控 Alpha，但产品主入口仍是旧 Registry UI。
- 当前不具备生产发布条件；P0 Gate 详见 [生产就绪基线](../../reference/PRODUCTION-READINESS-BASELINE.md)。
- 解决问题的方式是小批次安全与耐久性 Story，不拆微服务、不引入 Kafka/Kubernetes 复杂度来替代闭环。

## 子 Story

| 子 Story | 独立结果 | 状态 | 优先级 | 依赖 | 所属迭代 |
|----------|----------|------|--------|------|----------|
| [EVO-118-A](EVO-118-A-project-health-governance-baseline.md) | 建立体检事实基线、安全 SOP、路线图和发布门禁 | Review | P0 | 无 | Iteration 050 |
| [EVO-118-B](EVO-118-B-api-key-mcp-authorization-hardening.md) | API Key/RBAC/MCP execute 权限边界闭合 | Ready | P0 | EVO-118-A merge | - |
| [EVO-118-C](EVO-118-C-http-tool-egress-security.md) | HTTP Tool SSRF 与出站网络边界闭合 | Ready | P0 | EVO-118-A merge | - |
| [EVO-118-D](EVO-118-D-git-storage-durability-and-recovery.md) | Git 持久卷、联合备份和恢复演练闭合 | Ready | P0 | EVO-118-A merge | - |
| [EVO-118-E](EVO-118-E-production-build-deployment-convergence.md) | Embedded Frontend 与生产构建/部署收敛 | Ready | P0 | EVO-118-A merge | - |
| [EVO-118-F](EVO-118-F-repo-lifecycle-consistency.md) | Repo DB/FS 生命周期一致性与真实 Initial Commit | Proposed | P1 | EVO-118-D/E |
| [EVO-118-G](EVO-118-G-runtime-reliability-gates.md) | PR CI、readiness、限流和生产 fail-closed 接线 | Proposed | P1 | EVO-118-B/E |
| [EVO-118-H](EVO-118-H-durable-outbox-events.md) | Durable Outbox/Worker 为 Webhook/Indexer/Agent Event 提供可靠事件 | Proposed | P1 | EVO-118-E/G |

## 阶段 Gate

### S1：外部 Alpha 前 P0

必须完成：EVO-118-B、C、D、E。

S1 未完成前：

- 不声明生产就绪；
- 不发布外部 Alpha；
- 不让 Agent 写入能力进入生产；
- EVO-112 可以 refinement，但不应抢占实现 WIP。

### S2：Agent Beta 前可靠性

必须完成：EVO-118-F、G、H，并与 EVO-105/106/107 的实现验收共同关闭。

## Epic 完成条件

- [ ] A~H 全部 Done，或非必需项有明确 Deferred 决策、风险接受者和不影响目标的证明。
- [ ] [生产就绪基线](../../reference/PRODUCTION-READINESS-BASELINE.md) 中对应 Gate 全部解除。
- [ ] 生产 clean build、持久化重建、DB+Git 恢复、权限负向测试和 SSRF 测试均有实际证据。
- [ ] Backlog、Board、Roadmap、Release SOP 和相关 Reference 状态一致。
- [ ] EVO-112/105/106/107/108 的依赖和实现约束已同步到新安全/事件边界。

## 不做事项

- 不在本 Epic 内重写前端产品页面。
- 不拆分微服务，不引入 Kafka 或 Service Mesh。
- 不实现 SSH、LFS、跨仓搜索或完整 PR/MR。
- 不以文档完成替代运行时代码、测试和恢复演练。

## 验证归口

- 安全变更遵守 [Security Review SOP](../../sop/SECURITY-REVIEW.md)。
- 发布变更遵守 [Release SOP](../../sop/RELEASE.md)。
- 每个子 Story 独立记录命令、失败注入、负向测试和残余。

## 残余工作归口

- Repo UI 与 Vibe Coding：EVO-112 / EVO-104。
- Commit/Promote 与 Agent Session：EVO-105 / EVO-106。
- Webhook/Indexer：EVO-107 / EVO-108，依赖 EVO-118-H。
- Sandbox 删除：EVO-111。
