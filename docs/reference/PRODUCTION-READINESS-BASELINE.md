# 生产就绪与项目完成度基线

> 基线日期：2026-07-30  
> 状态：Active release gate  
> 归口：[EVO-118 Production Readiness and Security Hardening](../backlog/active/EVO-118-production-readiness-and-security-hardening.md)  
> 复核方式：代码、配置、部署文件、测试证据、Backlog 与路线图交叉审查。

## 1. 结论

Evolith 当前应被描述为：

> **Git 托管后端 Alpha 基础已经形成，但 AI-native Git 产品闭环尚未完成，生产发布仍被安全、数据耐久性和交付链路问题阻断。**

项目不需要推倒重构。当前模块化单体、Rust Workspace、PostgreSQL/SQLite 双轨、Git Smart HTTP + `gix` 的总体方向合理。后续开发的首要任务不是继续扩展页面或拆微服务，而是先关闭发布阻断项，再恢复 Repo UI、Agent Session、Commit/Promote、Vibe Coding 与 Indexer 主线。

## 2. 完成度口径

以下是 2026-07-30 的能力快照。百分比只用于表达阶段差距，不作为燃尽或绩效指标。

| 目标口径 | 估算完成度 | 当前判断 |
|----------|------------|----------|
| Rust 平台工程骨架 | 70%~75% | 模块、认证、Repository、双数据库、基础测试和配置体系较完整 |
| Git 托管后端 Alpha | 60%~70% | Repo CRUD、Smart HTTP、真实 clone/push/pull、Context API 已具备 |
| AI-native Git 产品 MVP | 35%~45% | Repo UI、受控写入、Agent Session、Webhook、Vibe Coding、Indexer 尚未闭环 |
| 可对外试用 SaaS | 25%~35% | 存在权限、SSRF、Git 持久化、备份和生产构建阻断 |
| 可规模化生产部署 | 20%~30% | 多副本 Git 存储、事件可靠性、恢复演练和完整发布门禁尚未形成 |

任何文档、PR 或发布说明不得只引用“Git clone/push 已通过”来推导“平台已可生产使用”。

## 3. 能力成熟度矩阵

| 能力 | 当前状态 | 可依赖范围 | 主要残余 |
|------|----------|------------|----------|
| 模块化单体 | Stable direction | 继续作为默认部署形态 | Handler/AppState 需要逐步收敛为 Application Service 边界 |
| Repo CRUD | Alpha | 本地/测试环境 | DB 与文件系统生命周期非原子；Seed 未形成真实 Commit |
| Git Smart HTTP | Alpha | 有权限的单实例环境 | Agent 写入策略、Ref/Branch scope、请求/仓库配额仍需硬化 |
| Repo Context API | Alpha/usable | 受控仓库读取 | 继续保持 Tree/Blob/Diff/Commit 上限和 blocking timeout |
| API Key / RBAC | Hardened baseline | Owner/Admin Key management、Typed Capability、MCP execute 与跨租户隐藏语义可依赖 | SEC-01 已解除；legacy Key 仍需按授权合约轮换 |
| MCP HTTP Tool | Release blocked | 仅受控开发环境 | 缺 SSRF、DNS/Redirect、私网和 Egress 边界 |
| Embedded Frontend | Direction accepted | 本地构建链 | 生产 Compose/Docker 构建形态与嵌入式交付不一致 |
| Git 数据持久化 | Release blocked | 临时单实例 | 生产 Compose 未挂载 Git 持久卷；备份只覆盖 PostgreSQL |
| Repo UI | Not implemented | 不可作为产品入口 | `/repos`、详情页和 repo-centric Dashboard 未落地 |
| Commit/Promote | Not implemented | 不可依赖 | PolicyEvaluator、写路径一致性、审计和冲突语义未实现 |
| Agent Session | Not implemented | 不可依赖 | Session、Scoped Token、撤销和事件日志未实现 |
| Webhook/Indexer | Proposed | 不可依赖 | 需要 Durable Outbox/Worker，不能依赖内存 spawn |

## 4. 发布阻断项

下列问题未关闭前，不得声明“生产就绪”，不得将生产发布作为已完成验收：

| Gate | 风险 | 归口 | 解除条件 |
|------|------|------|----------|
| SEC-01 | **已解除（2026-07-31）**：API Key 管理、Typed Capability、MCP execute、跨租户审计/隐藏语义已闭合 | [EVO-118-B](../backlog/active/EVO-118-B-api-key-mcp-authorization-hardening.md) | PR #3 merged `6de7845e1231efc04f94f16cb9ab0a410f6ad2d9`；CI `30567361095` 全绿；负向授权与撤销/过期/跨租户测试通过 |
| SEC-02 | HTTP Tool 可访问 localhost、私网、Metadata 或内部服务 | [EVO-118-C](../backlog/active/EVO-118-C-http-tool-egress-security.md) | Scheme/DNS/IP/Redirect/Egress 防护与 SSRF 测试通过 |
| DATA-01 | Git 仓库目录未形成生产持久卷，数据库备份不包含 Git 对象 | [EVO-118-D](../backlog/active/EVO-118-D-git-storage-durability-and-recovery.md) | 持久卷、DB+Git 备份、恢复演练和监控证据通过 |
| DEPLOY-01 | Embedded Frontend 与生产 Docker/Compose 仍存在双交付和构建上下文冲突 | [EVO-118-E](../backlog/active/EVO-118-E-production-build-deployment-convergence.md) | 干净环境镜像构建、单一交付形态和生产 Smoke Test 通过 |

以下为高优先级架构债，应在外部 Alpha 或 Agent Beta 前关闭：

| Gate | 风险 | 归口 |
|------|------|------|
| DATA-02 | Repo 创建/删除存在 DB 与文件系统分裂；Seed 文件不是 Git Commit | [EVO-118-F](../backlog/active/EVO-118-F-repo-lifecycle-consistency.md) |
| REL-01 | PR CI、readiness、分级限流、邮件 fail-closed 等门禁未真正接线 | [EVO-118-G](../backlog/active/EVO-118-G-runtime-reliability-gates.md) |
| EVENT-01 | Push 后元数据、Webhook、Indexer 和 Agent Event 缺少持久事件交付 | [EVO-118-H](../backlog/active/EVO-118-H-durable-outbox-events.md) |

## 5. 必须保留的架构判断

1. **继续采用模块化单体**。当前问题是边界和闭环，不是服务数量；不得以“架构升级”为名提前拆微服务。
2. **Git 是代码与历史的事实源，PostgreSQL 是身份、权限、元数据、索引和事件状态的事实源**。
3. **DB 与文件系统采用状态机、补偿和 Reconciler**，不以 best-effort warning 伪装成功，也不追求不可实现的跨介质 ACID。
4. **权限使用 Typed Capability 或受控枚举**。不得把任意 `Vec<String>` 直接解释为授权。
5. **Agent 写入必须经过 Commit/Promote + PolicyEvaluator**。不得把 `commit:<scope>` 简化为通用 Smart HTTP 写权限。
6. **出站 HTTP 必须经过统一 Egress Policy**。业务 Handler 和 Tool Executor 不得直接信任租户提供的 URL。
7. **Webhook、Indexer、Audit 和 Agent Event 使用 Durable Outbox/Worker**。内存 `spawn` 只能用于可丢失的辅助工作。
8. **生产环境对关键依赖 fail closed**。认证、Git 存储、邮件交付和安全配置不能静默降级后继续声称可用。

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
| Internal Alpha | SEC-01、SEC-02 完成；Git 目录必须持久化；测试数据允许手动恢复 |
| External Alpha | SEC-01、SEC-02、DATA-01、DEPLOY-01、DATA-02、REL-01 全部完成 |
| Agent Beta | External Alpha 条件 + Commit/Promote、Agent Session、PolicyEvaluator、EVENT-01 完成 |
| Production | Agent Beta 条件 + 备份恢复演练、容量告警、PR/Main CI、回滚验证和安全复核完成 |

## 8. 优先级规则

1. EVO-118 的 P0 发布阻断项优先于 EVO-112 Repo UI 和新的用户可见功能。
2. 单次迭代仍遵守 WIP：默认只推进一个 Ready Story，不把多个 P0 打包成不可验收的大改造。
3. 完成安全或耐久性 Story 后，应重新评估是否解除对应发布 Gate，而不是只把代码合并即视为解除。
4. EVO-112 可在 SEC-01~02、DATA-01、DEPLOY-01 关闭后恢复；EVO-105/106/107 必须同时遵守本基线的 Typed Capability、Policy 和 Durable Event 边界。
5. 新发现的安全、数据损坏或生产构建问题，先进入 EVO-118 或新 P0 Story，不得只留在 PR 评论或对话中。

## 9. 证据与复核

- 本基线中的“确认问题”来自 2026-07-30 对主分支代码和配置的静态审查。
- 独立编译、Docker clean build、渗透测试和恢复演练仍需由对应 Story 提供实际命令证据。
- SEC-01 已于 2026-07-31 由 EVO-118-B 解除：PR #3 merged `6de7845e1231efc04f94f16cb9ab0a410f6ad2d9`，最终 CI `30567361095` 全绿。
- 每关闭一个发布 Gate，应更新本文件、EVO-118 父项、Backlog、Board 和相关 Iteration。
- 当代码行为与本基线冲突时，以实际验证结果为准，并同步修正文档；不得保持已知漂移。
