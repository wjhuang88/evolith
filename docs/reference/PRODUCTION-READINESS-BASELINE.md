# 生产就绪与项目完成度基线

> 基线日期：2026-08-02
> 状态：Active release gate
> 归口：[EVO-118 Production Readiness and Security Hardening](../backlog/active/EVO-118-production-readiness-and-security-hardening.md)
> 当前执行：[EVO-118-D](../backlog/active/EVO-118-D-git-storage-durability-and-recovery.md) / [Iteration 053](../iterations/ITERATION-053.md) / Draft PR #7
> 复核方式：代码、配置、部署文件、测试证据、Backlog 与路线图交叉审查。

## 1. 结论

Evolith 当前应被描述为：

> **Git 托管后端 Alpha 与 Repo UI Shell 已形成，API Key/MCP 与 HTTP Tool 两项安全发布 Gate 已关闭并合入 main；EVO-118-D 正在最新主线上实施 DATA-01，但在恢复演练、exact-head CI、Navigator 和 post-merge 收口完成前，数据耐久性与生产发布仍保持阻断。**

项目不需要推倒重构。当前模块化单体、Rust Workspace、PostgreSQL/SQLite 双轨、Git Smart HTTP + `gix` 的总体方向合理。EVO-112-A Repo UI Shell 已由 PR #8 合入，但 Repo Detail、Agent Session、Commit/Promote、Vibe Coding 与 Indexer 主线仍必须服从 EVO-118 S1 Gate。

## 2. 完成度口径

以下是 2026-08-02 的能力快照。百分比只用于表达阶段差距，不作为燃尽或绩效指标。

| 目标口径 | 估算完成度 | 当前判断 |
|----------|------------|----------|
| Rust 平台工程骨架 | 70%~75% | 模块、认证、Repository、双数据库、基础测试和配置体系较完整 |
| Git 托管后端 Alpha | 65%~70% | Repo CRUD、Smart HTTP、真实 clone/push/pull、Context API 与 Repo UI Shell 已具备 |
| AI-native Git 产品 MVP | 40%~50% | Repo Shell 已完成；详情、受控写入、Agent Session、Webhook、Vibe Coding、Indexer 尚未闭环 |
| 可对外试用 SaaS | 30%~40% | 权限与 HTTP Tool SSRF Gate 已关闭；Git 数据恢复和生产构建仍阻断 |
| 可规模化生产部署 | 20%~30% | 多副本 Git 存储、事件可靠性、最终恢复演练和完整发布门禁尚未形成 |

任何文档、PR 或发布说明不得只引用“Git clone/push 已通过”“Repo UI 已合入”“安全 Gate 已关闭”或“备份脚本已提交”来推导“平台已可生产使用”。

## 3. 能力成熟度矩阵

| 能力 | 当前状态 | 可依赖范围 | 主要残余 |
|------|----------|------------|----------|
| 模块化单体 | Stable direction | 继续作为默认部署形态 | Handler/AppState 需要逐步收敛为 Application Service 边界 |
| Repo CRUD | Alpha | 本地/测试环境 | DB 与文件系统生命周期非原子；Seed 未形成真实 Commit |
| Git Smart HTTP | Alpha | 有权限的单实例环境 | Agent 写入策略、Ref/Branch scope、请求/仓库配额仍需硬化 |
| Repo Context API | Alpha/usable | 受控仓库读取 | 继续保持 Tree/Blob/Diff/Commit 上限和 blocking timeout |
| Repo UI Shell | Alpha / merged | Repo list/create、repo-centric navigation 与 Dashboard | Repo Detail 归 EVO-112-B；不代表 DATA-01 或 S1 关闭 |
| API Key / RBAC | Hardened baseline | Owner/Admin Key management、Typed Capability、MCP execute 与跨租户隐藏语义可依赖 | SEC-01 已解除；legacy Key 仍需按授权合约轮换 |
| MCP HTTP Tool | Hardened baseline | 受控 Internal Alpha；仅 HTTPS 公网目标并受统一 Egress Policy 约束 | SEC-02 已解除；未来 Webhook/其他租户可控出站能力必须复用同一边界 |
| Embedded Frontend | Direction accepted | 本地构建链 | 生产 Compose/Docker 构建形态与嵌入式交付不一致 |
| Git 数据持久化 | Release blocked / In Progress | 单实例候选实现 | Draft PR #7 正在实施持久卷、readiness、联合 backup/restore、inventory 和恢复演练；最新主线门禁尚未完成 |
| Commit/Promote | Not implemented | 不可依赖 | PolicyEvaluator、写路径一致性、审计和冲突语义未实现 |
| Agent Session | Not implemented | 不可依赖 | Session、Scoped Token、撤销和事件日志未实现 |
| Webhook/Indexer | Proposed | 不可依赖 | 需要 Durable Outbox/Worker，不能依赖内存 spawn；出站调用必须复用 Egress Policy |

## 4. 发布阻断项

下列未关闭问题不得被忽略；在全部 P0 Gate 关闭前，不得声明“生产就绪”，不得将生产发布作为已完成验收：

| Gate | 风险 | 归口 | 解除条件 / 证据 |
|------|------|------|-----------------|
| SEC-01 | **已解除（2026-07-31）**：API Key 管理、Typed Capability、MCP execute、跨租户审计/隐藏语义已闭合 | [EVO-118-B](../backlog/active/EVO-118-B-api-key-mcp-authorization-hardening.md) | PR #3 merged `6de7845e1231efc04f94f16cb9ab0a410f6ad2d9`；CI `30567361095` 全绿；负向授权与撤销/过期/跨租户测试通过 |
| SEC-02 | **已解除并合入 main（2026-08-02）**：HTTP Tool 统一经过严格 Egress Policy，禁止 localhost、私网、Metadata、特殊用途地址、DNS/Redirect 绕过与不受控代理 | [EVO-118-C](../backlog/active/EVO-118-C-http-tool-egress-security.md) | final head `de762e2dae6bf5716e54c64e2277cb2e26592e36` 的 CI #137 / run `30682419168` 全绿；PR #5 merged `936ed3b26a62840ddd94cf10e5075fd19e0a1c5c` |
| DATA-01 | **Open / In Progress**：旧主线没有生产 Git 持久卷、联合恢复或 Git readiness；当前候选实现尚未通过最新主线 exact-head 全门禁与独立复验 | [EVO-118-D](../backlog/active/EVO-118-D-git-storage-durability-and-recovery.md) / [Iteration 053](../iterations/ITERATION-053.md) | 持久卷、Git Storage readiness、版本化 DB+Git 联合备份、安全 restore、inventory、容器重建和应用级空环境恢复全部通过；exact-head CI 与 Navigator 无 blocker；post-merge 治理关闭 |
| DEPLOY-01 | Embedded Frontend 与生产 Docker/Compose 仍存在双交付和构建上下文冲突 | [EVO-118-E](../backlog/active/EVO-118-E-production-build-deployment-convergence.md) | 干净环境镜像构建、单一交付形态和生产 Smoke Test 通过 |

以下为高优先级架构债，应在外部 Alpha 或 Agent Beta 前关闭：

| Gate | 风险 | 归口 |
|------|------|------|
| DATA-02 | Repo 创建/删除存在 DB 与文件系统分裂；Seed 文件不是 Git Commit | [EVO-118-F](../backlog/active/EVO-118-F-repo-lifecycle-consistency.md) |
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
| Internal Alpha | SEC-01、SEC-02 已完成；Git 目录必须持久化；在 DATA-01 完成前只允许受控环境和人工恢复限制 |
| External Alpha | SEC-01、SEC-02、DATA-01、DEPLOY-01、DATA-02、REL-01 全部完成 |
| Agent Beta | External Alpha 条件 + Commit/Promote、Agent Session、PolicyEvaluator、EVENT-01 完成 |
| Production | Agent Beta 条件 + 备份恢复演练、容量告警、PR/Main CI、回滚验证和安全复核完成 |

## 8. 优先级规则

1. EVO-118 的 P0 发布阻断项优先于 EVO-112-B 和新的用户可见功能。
2. 单次迭代仍遵守 WIP：默认只推进一个 Ready Story，不把多个 P0 打包成不可验收的大改造。
3. 完成安全或耐久性 Story 后，应重新评估是否解除对应发布 Gate，而不是只把代码合并即视为解除。
4. EVO-112-A 已完成并合入；EVO-112-B 等后续实现仍等待 DATA-01、DEPLOY-01 关闭。
5. EVO-105/106/107 必须遵守本基线的 Typed Capability、Policy 和 Durable Event 边界。
6. 新发现的安全、数据损坏或生产构建问题，先进入 EVO-118 或新 P0 Story，不得只留在 PR 评论或对话中。

## 9. 当前执行入口

- 当前 `main`：`38c19b19cff5aab7a08ac40a1cf417e1712e1b07`，包含 PR #8 Repo UI 与 PR #9 / Iteration 054 merge closure。
- 当前开放 PR：Draft PR #7，EVO-118-D / DATA-01。
- 当前 Story：EVO-118-D `In Progress`。
- 当前 Iteration：[Iteration 053](../iterations/ITERATION-053.md) `Active`；Iteration 054 已 Closed / Complete，不占用 runtime WIP。
- PR #7 已因主线推进执行重新基线；旧 Head `608ad6acd20229498898eaf16afaff6ec8a79878` 的 CI 结果只能作为定位线索，新 Head 必须重新通过全部 exact-head 门禁。
- EVO-118-E 保持 Ready，不与 D 并行抢占 WIP；D post-merge 收口后才重新评估下一激活。
- DATA-01 保持 Open；治理激活、代码提交、PR 创建、旧 CI 或 Repo UI 合并均不能单独关闭 Gate。

## 10. 证据与复核

- DATA-01 原始失败基线由 2026-08-02 对当时 `main` 的核对确认：生产 Compose 未挂载 Backend Git Storage，`scripts/backup.sh` 仅执行 PostgreSQL dump，readiness 未检查 Git Storage。
- 旧基线 CI #142 已证明联合 PostgreSQL+Git 恢复矩阵和跨容器 named Volume clone/refs 可执行，但因 rustfmt 失败未形成完整门禁；#143 通过 fmt/check/clippy。这些结果不替代最新主线 exact-head 证据。
- 当前候选实现包含：单实例持久 Volume、Git Storage fail-closed readiness、停写维护窗口联合 backup、staging-first restore、DB/Git inventory、恶意/损坏归档拒绝、应用级登录/list/Smart HTTP clone/refs/restart 恢复演练。
- Frontend type-check/build 必须验证 PR #8 Repo UI 在 DATA-01 分支上无回归；Rust fmt/check/clippy/SQLite workspace tests 与真实 PostgreSQL 恢复矩阵必须全部执行。
- Production Compose clean build/startup 是 DEPLOY-01 诊断；若仍因 Embedded Frontend 构建形态失败，应继续归 EVO-118-E，不可用 DATA-01 测试代替。
- DATA-01 只有在最新 PR Head 的实际恢复演练、exact-head CI、Navigator、稳定文档和治理关闭全部完成后才能解除；实现 PR 合并后仍需 post-merge 回写 merge commit 和当前入口。
- 当代码行为与本基线冲突时，以实际验证结果为准，并同步修正文档；不得保持已知漂移。
