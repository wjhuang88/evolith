# 生产就绪与项目完成度基线

> 基线日期：2026-08-09
> 状态：Active release gate
> 归口：[EVO-118 Production Readiness and Security Hardening](../backlog/active/EVO-118-production-readiness-and-security-hardening.md)
> 当前执行：EVO-118-D/F 与 DATA-01/DATA-02 已关闭；G-B/C/D Complete，G-A Partial；H-A/H-B / Iterations 066/067 Done / Complete，H-C / Iteration 068 Review / Partial；EVO-112-A/B/C、EVO-120 与 EVO-121-C/D 已完成。EVO-118-E 按 ADR-0010 保留为最终发布 Gate。
> 复核方式：代码、配置、部署文件、测试证据、Backlog 与路线图交叉审查。

## 1. 结论

Evolith 当前应被描述为：

> **Git 托管后端 Alpha、Repo UI Shell 与 Repo Detail、单实例 PostgreSQL + Git 联合耐久性及 Repo 生命周期一致性基线已形成；SEC-01、SEC-02、DATA-01、DATA-02 已关闭。EVO-118-E / DEPLOY-01 仍开放并作为最终发布 Gate，生产发布仍保持阻断，但不阻塞普通产品开发。**

项目不需要推倒重构。当前模块化单体、Rust Workspace、PostgreSQL/SQLite 双轨、Git Smart HTTP + `gix` 的总体方向合理。EVO-112-A Repo UI Shell 已由 PR #8 合入，EVO-112-B/C Repo Detail/Commit Evidence 与 EVO-120 First-run Repo Onboarding 已完成；ADR-0008/0009 已确定最终 Repo-centric 交互和“不建设预上线兼容层”的目标，但 Agent Session、Commit/Promote、Vibe Coding、Indexer 与其余 Experience Convergence 尚未实现。它们可在非生产环境按依赖推进，最终发布仍服从 EVO-118-E。

## 2. 完成度口径

以下是 2026-08-02 的能力快照。百分比只用于表达阶段差距，不作为燃尽或绩效指标。

| 目标口径 | 估算完成度 | 当前判断 |
|----------|------------|----------|
| Rust 平台工程骨架 | 70%~75% | 模块、认证、Repository、双数据库、基础测试和配置体系较完整 |
| Git 托管后端 Alpha | 65%~70% | Repo CRUD、Smart HTTP、真实 clone/push/pull、Context API 与 Repo UI Shell 已具备 |
| AI-native Git 产品 MVP | 45%~55% | Repo Shell、Detail、Commit Evidence 与 Onboarding 已完成；Public/Auth Entry 正在实施；受控写入、Agent Session、Vibe Coding、Activity、Discovery 尚未实现 |
| 可对外试用 SaaS | 30%~40% | 权限、HTTP Tool SSRF、Git durability 与 Repo lifecycle Gate 已关闭；runtime reliability、durable events 和生产构建仍阻断 |
| 可规模化生产部署 | 20%~30% | 多副本 Git 存储、事件可靠性、最终恢复演练和完整发布门禁尚未形成 |

任何文档、PR 或发布说明不得只引用“Git clone/push 已通过”“Repo UI 已合入”“安全 Gate 已关闭”或“备份脚本已提交”来推导“平台已可生产使用”。

## 3. 能力成熟度矩阵

| 能力 | 当前状态 | 可依赖范围 | 主要残余 |
|------|----------|------------|----------|
| 模块化单体 | Stable direction | 继续作为默认部署形态 | Handler/AppState 需要逐步收敛为 Application Service 边界 |
| Repo CRUD | Alpha | 本地/测试环境 | lifecycle 状态、失败补偿、真实 Initial Commit 与 tenant Reconciler 已具备；仍需 runtime/readiness 与最终发布门禁 |
| Git Smart HTTP | Alpha | 有权限的单实例环境 | Agent 写入策略、Ref/Branch scope、请求/仓库配额仍需硬化 |
| Repo Context API | Alpha/usable | 受控仓库读取 | 继续保持 Tree/Blob/Diff/Commit 上限和 blocking timeout |
| Repo UI | Alpha / partial merge | Repo list/create、首次 onboarding、历史第一阶段 navigation、Dashboard、Repo Detail read-only 与 Commit Evidence | EVO-112-B/C、EVO-120 已完成但尚未提交/合并；最终 Experience 归 EVO-121 |
| API Key / RBAC | Hardened baseline | Owner/Admin Key management、Typed Capability、MCP execute 与跨租户隐藏语义可依赖 | SEC-01 已解除；legacy Key 仍需按授权合约轮换 |
| MCP HTTP Tool | Hardened baseline | 受控 Internal Alpha；仅 HTTPS 公网目标并受统一 Egress Policy 约束 | SEC-02 已解除；未来 Webhook/其他租户可控出站能力必须复用同一边界 |
| Embedded Frontend | Direction accepted | 本地构建链 | 生产 Compose/Docker 构建形态与嵌入式交付不一致 |
| Git 数据持久化 | Hardened single-instance baseline / merged | 单实例 PostgreSQL + RWO/named Git volume、受控维护窗口恢复 | PR #7 merged `932def0`；联合 backup/restore、inventory、readiness、负向 rollback 与真实容器/应用恢复通过；多副本共享仍不声明 |
| Commit/Promote | Not implemented | 不可依赖 | PolicyEvaluator、写路径一致性、审计和冲突语义未实现 |
| Agent Session | Not implemented | 不可依赖 | Session、Scoped Token、撤销和事件日志未实现 |
| Webhook/Indexer | Proposed | 不可依赖 | 需要 Durable Outbox/Worker，不能依赖内存 spawn；出站调用必须复用 Egress Policy |

## 4. 发布阻断项与最终 Gate

下列未关闭问题不得被忽略；在全部 P0 Gate 关闭前，不得声明“生产就绪”，不得将生产发布作为已完成验收：

| Gate | 风险 | 归口 | 解除条件 / 证据 |
|------|------|------|-----------------|
| SEC-01 | **已解除（2026-07-31）**：API Key 管理、Typed Capability、MCP execute、跨租户审计/隐藏语义已闭合 | [EVO-118-B](../backlog/active/EVO-118-B-api-key-mcp-authorization-hardening.md) | PR #3 merged `6de7845e1231efc04f94f16cb9ab0a410f6ad2d9`；CI `30567361095` 全绿；负向授权与撤销/过期/跨租户测试通过 |
| SEC-02 | **已解除并合入 main（2026-08-02）**：HTTP Tool 统一经过严格 Egress Policy，禁止 localhost、私网、Metadata、特殊用途地址、DNS/Redirect 绕过与不受控代理 | [EVO-118-C](../backlog/active/EVO-118-C-http-tool-egress-security.md) | final head `de762e2dae6bf5716e54c64e2277cb2e26592e36` 的 CI #137 / run `30682419168` 全绿；PR #5 merged `936ed3b26a62840ddd94cf10e5075fd19e0a1c5c` |
| DATA-01 | **已解除并合入 main（2026-08-04）**：单实例 Git 持久卷、Git readiness、版本化 PostgreSQL + Git 联合备份、安全 restore、inventory、容器重建和应用级空环境恢复均已闭环 | [EVO-118-D](../backlog/active/EVO-118-D-git-storage-durability-and-recovery.md) / [Iteration 053](../iterations/ITERATION-053.md) | final Head `158ba98fb2d1e33fe5821f2e75431e86a5cf6ffd`；CI #204 / `30838250911` 与 container #50 / `30838250875` success；Navigator Complete；PR #7 merged `932def05717b678f6f44dc23f137933d56158957`；post-merge governance closed |
| DEPLOY-01 | Embedded Frontend 与生产 Docker/Compose 仍存在双交付和构建上下文冲突 | [EVO-118-E](../backlog/active/EVO-118-E-production-build-deployment-convergence.md) | 干净环境镜像构建、单一交付形态和生产 Smoke Test 通过 |

以下为高优先级架构债，应在外部 Alpha 或 Agent Beta 前关闭：

| Gate | 风险 | 归口 |
|------|------|------|
| DATA-02 | **已关闭（2026-08-08）**：Repo 创建/删除使用可观察 lifecycle 与 Reconciler；Seed 为真实 Initial Commit；未知 disk-only 数据只隔离不删除 | [EVO-118-F](../backlog/active/EVO-118-F-repo-lifecycle-consistency.md) / [Iteration 055](../iterations/ITERATION-055.md) |
| REL-01 | 综合 PR CI、readiness、分级限流、邮件 fail-closed 等门禁尚未完整接线；Iteration 053 只处理直接服务 DATA-01 的 Git Storage readiness | [EVO-118-G](../backlog/active/EVO-118-G-runtime-reliability-gates.md) |
| EVENT-01 | Push 后元数据、Webhook、Indexer 和 Agent Event 缺少持久事件交付 | [EVO-118-H](../backlog/active/EVO-118-H-durable-outbox-events.md) |

## 5. 必须保留的架构判断

1. **继续采用模块化单体**。当前问题是边界和闭环，不是服务数量；不得以“架构升级”为名提前拆微服务。
2. **Git 是代码与历史的事实源，PostgreSQL 是身份、权限、元数据、索引和事件状态的事实源**。
3. **DB 与文件系统采用状态机、补偿和 Reconciler**，不以 best-effort warning 伪装成功，也不追求不可实现的跨介质 ACID。
4. **权限使用 Typed Capability 或受控枚举**。不得把任意 `Vec<String>` 直接解释为授权。
5. **Agent 写入必须经过 Commit/Promote + PolicyEvaluator**。不得把 `commit:<scope>` 简化为通用 Smart HTTP 写权限。
6. **出站 HTTP 必须经过统一 Egress Policy**。业务 Handler、Tool Executor、未来 Webhook 或其他租户可控出站组件不得直接信任用户提供的 URL。
7. **Webhook、Indexer、Audit 和 Agent Event 使用 Durable Outbox/Worker**。内存 `spawn` 只能用于可丢失的辅助工作。
8. **生产环境对关键依赖 fail closed**。认证、Git 存储、邮件交付和安全配置不能静默降级后继续声称可用。
9. **联合备份必须定义一致性点**。顺序执行 `pg_dump` 与文件归档不自动构成一致快照；当前 DATA-01 方案采用明确停写维护窗口。
10. **恢复默认不得覆盖非空生产目标**。manifest、版本、必需文件、条目类型和 checksum 必须在写入目标前验证；中途失败不得报告成功。
11. **本地/RWO Git Volume 只证明单实例持久性**。不得据此宣称多实例共享、复制或自动故障转移。
12. **未上线阶段不建设旧 Registry 双写兼容**。ADR-0009 已 Drop EVO-110；Repo-derived read/execute 承接后由 EVO-121-F/EVO-122 直接删除旧 UI/runtime/table，且表删除仍必须满足双数据库与安全门禁。

## 6. 推荐目标边界

```text
HTTP / Git Protocol Adapters
            |
            v
Application Services
- RepoApplicationService
- GitWriteService
- PolicyEvaluator
- CredentialService
- AgentSessionService
- CapabilityIndexer
- WebhookDeliveryService
            |
            v
Domain
- Repo / Policy / Capability / Session / Event
            |
            v
Infrastructure
- PostgreSQL / SQLite Repositories
- GitStorage
- Durable Outbox
- Mail Delivery
- HTTP Egress Gateway
- Cache
```

该结构仍可运行在一个仓库、一个主二进制和一个可选 Worker 中。引入 Application Service 不等于拆微服务。

## 7. 环境级发布门禁

| 环境 | 允许条件 |
|------|----------|
| Local development | 可以使用 SQLite、临时 Git 目录和 Mock 外部服务；必须明确非生产 |
| Internal Alpha | SEC-01、SEC-02、DATA-01、DATA-02 已完成；允许按单实例持久卷、Repo lifecycle 与受控维护窗口恢复契约试用，但 DEPLOY-01 未关闭前仍需受控交付 |
| External Alpha | SEC-01、SEC-02、DATA-01、DEPLOY-01、DATA-02、REL-01 全部完成 |
| Agent Beta | External Alpha 条件 + Commit/Promote、Agent Session、PolicyEvaluator、EVENT-01 完成 |
| Production | Agent Beta 条件 + 备份恢复演练、容量告警、PR/Main CI、回滚验证和安全复核完成 |

## 8. 优先级规则

1. EVO-118 的 P0 发布阻断项优先于 EVO-112-B 和新的用户可见功能。
2. 单次迭代仍遵守 WIP：默认只推进一个 Ready Story，不把多个 P0 打包成不可验收的大改造。
3. 完成安全或耐久性 Story 后，应重新评估是否解除对应发布 Gate，而不是只把代码合并即视为解除。
4. EVO-112-A 已完成并合入；DATA-01/DATA-02 已关闭，EVO-120、EVO-112-B/C、EVO-121 等后续产品实现按 G/H 与各自直接依赖推进，不再等待最终 DEPLOY-01。
5. EVO-105/106/107 必须遵守本基线的 Typed Capability、Policy 和 Durable Event 边界。
6. 新发现的安全、数据损坏或生产构建问题，先进入 EVO-118 或新 P0 Story，不得只留在 PR 评论或对话中。

## 9. 当前执行入口

- PR #7 已于 2026-08-04 合并到 `main`；final Head `158ba98fb2d1e33fe5821f2e75431e86a5cf6ffd`，merge commit `932def05717b678f6f44dc23f137933d56158957`。
- EVO-118-D：Done / Complete / Merged。
- Iteration 053：Closed / Complete。
- DATA-01：Closed。
- EVO-118-F：Done / Complete；Iteration 055：Closed / Complete；DATA-02：Closed。
- EVO-118-G-B/C/D：Done / Complete；G-A 因远端 Branch Protection 403 保持 Review / Partial。EVO-118-H / Iteration 060 Closed / Partial 后拆为 H-A/B/C；H-A/H-B / Iterations 066/067 Done / Complete，H-C / Iteration 068 Review / Partial，EVENT-01 未关闭；完整 Git Smart HTTP E2E 残余归 EVO-125。
- EVO-112-A/B/C、EVO-120 与 EVO-121-C/D：Done / Complete；Iterations 054/061/062/063/064/065 Closed / Complete。依赖复核确认 EVO-105/106 不能绕过 H；当前由 Iteration 068 推进 H-C。
- EVO-118-E 保持 Proposed / final release gate。
- SEC-01、SEC-02、DATA-01、DATA-02 已解除；DEPLOY-01 仍开放，因此仍不得声明生产就绪或发布外部 Alpha。
- Subscription conninfo 不属于 DATA-01 普通联合归档，必须通过独立安全运维流程重建；多副本共享 Git 存储、runtime reliability 与 durable events 继续由后续 Story 归口。
