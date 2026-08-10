# EVO-118 Epic: Production Readiness and Security Hardening

- **类型**：Epic / Security / Reliability
- **状态**：In Progress
- **优先级**：P0
- **触发**：2026-07-30 全面项目体检
- **父 Epic**：无

## 总体目标

关闭已确认的权限、SSRF、Git 数据耐久性和运行可靠性缺口，并在目标产品开发完成后执行最终生产构建与部署收敛，使 Evolith 从“Git 后端 Alpha 基础”进入可安全发布外部 Alpha 的状态。

本 Epic 不替代 [EVO-100 Git-Centric Platform Foundation](EVO-100-git-centric-platform-foundation.md)。安全、数据一致性和事件边界是后续产品能力的前置；EVO-118-E 则按 ADR-0010 在目标产品与清理完成后执行，只门禁发布，不再门禁开发。

## 当前判断

- 模块化单体、Git Smart HTTP + `gix`、双数据库和 Embedded Frontend 的总体方向保留。
- Git 后端 Alpha 与 EVO-112-A Repo UI Shell 已进入 `main`；EVO-112-B Repo Detail 已在 Iteration 061 完成但尚未提交/合并，Agent write loop 和生产交付仍未闭环。
- SEC-01、SEC-02、DATA-01、DATA-02 已关闭；DEPLOY-01 仍开放并保留为最终发布 Gate，详见 [生产就绪基线](../../reference/PRODUCTION-READINESS-BASELINE.md)。
- EVO-118-D / [Iteration 053](../../iterations/ITERATION-053.md) 已 Done / Closed，DATA-01 已解除；EVO-118-F / [Iteration 055](../../iterations/ITERATION-055.md) 已 Done / Closed，DATA-02 已解除。
- Iteration 054 已 Closed / Complete，PR #8 / #9 的 Repo UI 与治理收口事实必须保留，但不替代 DATA-01 owner。
- 解决问题的方式是小批次安全与耐久性 Story，不拆微服务、不引入 Kafka/Kubernetes 复杂度来替代闭环。

## 子 Story

| 子 Story | 独立结果 | 状态 | 优先级 | 依赖 | 所属迭代 |
|----------|----------|------|--------|------|----------|
| [EVO-118-A](EVO-118-A-project-health-governance-baseline.md) | 建立体检事实基线、安全 SOP、路线图和发布门禁 | Done | P0 | 无 | Iteration 050 |
| [EVO-118-B](EVO-118-B-api-key-mcp-authorization-hardening.md) | API Key/RBAC/MCP execute 权限边界闭合 | Done | P0 | EVO-118-A Done | Iteration 051 / PR #3 merged |
| [EVO-118-C](EVO-118-C-http-tool-egress-security.md) | HTTP Tool SSRF 与出站网络边界闭合 | Done / Merged | P0 | EVO-118-A/B Done | Iteration 052 Closed / PR #5 merged `936ed3b26a62840ddd94cf10e5075fd19e0a1c5c` |
| [EVO-118-D](EVO-118-D-git-storage-durability-and-recovery.md) | Git 持久卷、联合备份和恢复演练闭合 | Done / Merged | P0 | EVO-118-A/B/C Done | Iteration 053 Closed / PR #7 merged `932def0` |
| [EVO-118-E](EVO-118-E-production-build-deployment-convergence.md) | 最终 Embedded Frontend 生产构建/部署收敛 | Proposed / final release gate | P0 | F/G/H + 目标 MVP + legacy cleanup | 最后执行 |
| [EVO-118-F](EVO-118-F-repo-lifecycle-consistency.md) | Repo DB/FS 生命周期一致性与真实 Initial Commit | Done / Complete | P1 | EVO-118-D Done | Iteration 055 Closed |
| [EVO-118-G](EVO-118-G-runtime-reliability-gates.md) | PR CI、readiness、限流和生产 fail-closed 接线 | In Progress | P1 | EVO-118-B Done | G-A Review/Partial；G-B/C/D Complete |
| [EVO-118-H](EVO-118-H-durable-outbox-events.md) | Durable Outbox/Worker 为 Webhook/Indexer/Agent Event 提供可靠事件 | Done / Complete | P1 | EVO-118-G | Iteration 060 Partial；066/067/068 Complete |

## 阶段 Gate

### S1：开发前安全与耐久性基线

必须完成：EVO-118-B、C、D。三项均已完成。

当前进度：

- EVO-118-A/B/C 已 Done。
- SEC-01、SEC-02 已解除并合入 `main`。
- EVO-118-C 实现 head `de762e2dae6bf5716e54c64e2277cb2e26592e36` 通过 final-head CI #137 / run `30682419168`；PR #5 于 2026-08-02 合并，merge commit `936ed3b26a62840ddd94cf10e5075fd19e0a1c5c`。
- EVO-118-D / Iteration 053 已完成；final Head `158ba98fb2d1e33fe5821f2e75431e86a5cf6ffd` required workflows 全绿、Navigator Complete，PR #7 merged `932def05717b678f6f44dc23f137933d56158957`，DATA-01 Closed。
- 主线随后合入 EVO-112-A / Iteration 054；该完成事实不自动关闭 DEPLOY-01，最终部署仍须在产品开发与清理完成后按 EVO-118-E 重新验证。

DEPLOY-01 未关闭前：

- 不声明生产就绪；
- 不发布外部 Alpha；
- 不让 Agent 写入能力进入生产；
- 可以继续目标 MVP 开发与清理，但所有环境必须明确为非生产或受控内部开发环境。

### S2：Agent Beta 前可靠性

必须完成：EVO-118-F、G、H，并与 EVO-105/106/107 的实现验收共同关闭。

### S3：最终发布收敛

EVO-118-E 在目标 MVP、EVO-118-F/G/H 和 legacy cleanup 完成后执行。它关闭 DEPLOY-01，是 External Alpha/生产发布的最后 Gate，不是普通开发的前置条件。

## Epic 完成条件

- [ ] A~H 全部 Done，或非必需项有明确 Deferred 决策、风险接受者和不影响目标的证明。
- [ ] [生产就绪基线](../../reference/PRODUCTION-READINESS-BASELINE.md) 中对应 Gate 全部解除。
- [ ] 生产 clean build、持久化重建、DB+Git 恢复、权限负向测试和 SSRF 测试均有实际证据。
- [ ] Backlog、Board、Roadmap、Release SOP 和相关 Reference 状态一致。
- [ ] EVO-112/105/106/107/108 的依赖和实现约束已同步到新安全/事件边界。

## 不做事项

- 不在本 Epic 内重写或扩展 Repo UI；EVO-112-A 已合入，EVO-112-B 独立排期。
- 不拆分微服务，不引入 Kafka 或 Service Mesh。
- 不实现 SSH、LFS、跨仓搜索或完整 PR/MR。
- 不以文档完成替代运行时代码、测试和恢复演练。

## 验证归口

- 安全变更遵守 [Security Review SOP](../../sop/SECURITY-REVIEW.md)。
- 发布变更遵守 [Release SOP](../../sop/RELEASE.md)。
- 每个子 Story 独立记录命令、失败注入、负向测试和残余。

## 当前执行入口

1. Iteration 060 保持 Closed / Partial 历史；H-A/B/C 与 Iterations 066/067/068 均 Done / Closed / Complete，EVENT-01 Durable Outbox 基础 Gate 已解除。
2. PR #7 final Head `158ba98fb2d1e33fe5821f2e75431e86a5cf6ffd`，merge commit `932def05717b678f6f44dc23f137933d56158957`；DATA-01 Closed。
3. G-B/C/D 已完成本地验收；G-A 仍有远端 Branch Protection residual；H 已关闭 claim recovery、PG concurrency、Worker runtime、Push durable event 与完整 Smart HTTP E2E，EVO-125 已 Done / Complete。
4. EVO-118-E 按 ADR-0010 最后执行；当前仍不得把安全/数据 Gate 完成解释为平台生产就绪。

## 残余工作归口

- EVO-118-E：关闭 Embedded Frontend 与生产构建/部署收敛，对应 DEPLOY-01。
- Repo Detail 已由 EVO-112-B 完成；Vibe Coding 仍归 EVO-104。
- Commit/Promote 与 Agent Session：EVO-105 / EVO-106。
- Webhook/Indexer：EVO-107 / EVO-108，必须复用 EVO-118-C Egress Policy 并依赖 EVO-118-H。
- Sandbox 删除：EVO-111。
