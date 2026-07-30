# Evolith 实施路线图

> 制定日期：2026-05-15  
> 最近更新：2026-07-30（全面体检后增加 Production Readiness Stabilization）  
> 目标：维护阶段优先级、实施顺序和 Backlog / Proposals / Release Gate 归口关系。

本文档不是任务池。Agent 不应直接从本文档开工：

- 可执行 Story 归口到 [Product Backlog](../backlog/PRODUCT-BACKLOG.md)。
- 当前生产事实与 Gate 归口到 [Production Readiness Baseline](../reference/PRODUCTION-READINESS-BASELINE.md)。
- 当前执行排序归口到 [Production Readiness Plan](PRODUCTION-READINESS-PLAN-2026-07.md)。
- 未成熟方向归口到 [Proposals](../proposals/README.md)。
- 重大取舍归口到 [ADR](../decisions/README.md)。

## 1. 当前判断

### 1.1 产品方向

2026-06-23 的 Git-centric 决策继续有效：

> Evolith 是 Git 托管 + Vibe Coding + Pages 式 Skill/CLI/MCP 能力发现平台。

Git Repo 是代码与版本历史的事实源；Skill/CLI/MCP 是由 Repo 内容派生的能力。详见 ADR-0004、ADR-0005、ADR-0006 和 Git-Centric Proposal。

### 1.2 成熟度

截至 2026-07-30：

- 工程骨架较完整，模块化单体方向合理。
- Repo CRUD、Smart HTTP、真实 clone/push/pull、Context API 已形成 Git 后端 Alpha。
- Web 产品仍以旧 Tools/Skills/Interfaces 为主入口，Repo UI 未落地。
- Commit/Promote、Agent Session、Webhook、Vibe Coding、Indexer 未形成产品闭环。
- 权限、SSRF、Git 数据耐久性和生产构建存在发布阻断项。

因此项目不得描述为生产就绪，也不应继续把用户可见功能置于安全和数据 Gate 之前。

## 2. 保留的技术路线

### 2.1 部署形态

继续采用模块化单体：

```text
React + Vite + Bun
        |
        | build/embed
        v
Evolith Actix Application
- HTTP API / Middleware
- Git Smart HTTP
- Repo Context
- Compatibility APIs
        |
        +--> PostgreSQL / SQLite
        +--> Git Storage on persistent filesystem
        +--> Redis optional by responsibility
```

- 默认发布物是一个 Actix 进程；可增加同仓库 Worker 运行模式。
- Nginx/平台网关只作为 TLS、反代和负载均衡层。
- 不以拆微服务、Kafka 或 Kubernetes 作为解决当前问题的前提。

### 2.2 事实源

| 数据 | 事实源 |
|------|--------|
| 代码、Commit、Branch、Tag、Repo 内能力描述 | Git Repository |
| 用户、租户、权限、Repo 元数据、索引、事件状态 | PostgreSQL |
| Lite 开发和本地测试 | SQLite |
| 普通缓存 | Redis 可选 |
| Session、分布式限流、锁或安全状态 | 必须有明确生产一致性方案，不能静默进程内降级 |

### 2.3 目标内部边界

后续跨层能力优先进入 Application Service：

```text
Adapters
  -> RepoApplicationService / GitWriteService / CredentialService
  -> PolicyEvaluator / AgentSessionService
  -> CapabilityIndexer / WebhookDeliveryService
  -> Domain + Repository / GitStorage / Outbox / EgressPolicy
```

Handler 不应长期直接编排权限、DB、文件系统、审计和异步任务。

## 3. 已完成阶段

| Phase | 状态 | 结果 |
|-------|------|------|
| Phase A API 对齐 | Done | 修复主要前后端合约漂移 |
| Phase B React + Vite + Bun | Done | 去除 Next runtime，静态 SPA 落地 |
| Phase C Auth lifecycle | Done/partial production hardening | 注册、登录、重置、验证、邀请已具备；Session/revocation 仍待 Agent/Auth 后续 |
| Phase D MCP Tool execution | Done/Release blocked | HTTP Tool 可执行；授权与 SSRF 硬化转 EVO-118-B/C |
| Phase E'-1 Git Service foundation | Done/Alpha | EVO-101/102/103/113/115/116；不等于生产就绪 |

## 4. 当前阶段：Production Readiness Stabilization

归口：[EVO-118](../backlog/active/EVO-118-production-readiness-and-security-hardening.md)。

### S0 Governance Baseline

- EVO-118-A：生产就绪事实、Security SOP、Backlog、Roadmap、Release Gate。

### S1 P0 Release Blockers

| Story | Gate | 完成结果 |
|-------|------|----------|
| EVO-118-B | SEC-01 | API Key Owner/Admin 管理、Typed Capability、MCP execute、负向权限测试 |
| EVO-118-C | SEC-02 | HTTP Tool DNS/IP/Redirect/Metadata/私网防护和受控 Egress |
| EVO-118-D | DATA-01 | Git 持久卷、PostgreSQL+Git 备份、空环境恢复演练 |
| EVO-118-E | DEPLOY-01 | Embedded Frontend 单一交付、clean build 和生产 Smoke Test |

S1 未关闭前，不发布外部 Alpha，不将 Agent 写入能力部署到生产；Repo UI 可以 refinement，但不抢占实现 WIP。

### S2 Consistency and Reliability

| Story | Gate | 完成结果 |
|-------|------|----------|
| EVO-118-F | DATA-02 | Repo lifecycle 状态、补偿/Reconcile、真实 Initial Commit |
| EVO-118-G | REL-01 | PR/Main CI、readiness 503、分级限流、邮件/Redis fail-closed |
| EVO-118-H | EVENT-01 | Durable Outbox + Worker + 幂等重试和死信 |

## 5. 恢复后的产品阶段

### Phase E'-1.5 Repo-centric Web

归口：EVO-112，建议拆为：

1. Repo UI Shell：`/repos`、创建、导航、Dashboard；
2. Repo Detail Read-only：Files、Commits、Settings、Clone URL。

进入条件：EVO-118 S1 关闭。

### Phase E'-2 Agent Write Loop + Vibe Coding

归口：EVO-105、106、107、104。

进入条件：

- EVO-118-B/F/H 完成；
- Repo UI 基础可用；
- API Contract 和 Policy 语义稳定。

硬约束：

- Agent Token 不得直接获得不受策略控制的通用 `git-receive-pack`。
- `commit:<scope>` 必须落实为 Branch/Path capability。
- Commit/Promote 失败时目标 Ref 不移动。
- Policy `auto_merge / require_review / block` 必须由行为测试证明。
- Push/Commit/Promote/Session/Webhook 事件进入 Durable Outbox。

### Phase E'-3 Capability Index and Discovery

归口：EVO-108、109、110。

进入条件：

- Push/Commit 事件语义稳定；
- EVO-118-H 完成；
- Parser 失败隔离和资源上限明确。

结果：

- Repo 中 `SKILL.md`、`interface.yaml`、`tool.yaml` 可在有界时间内进入索引；
- Discovery API/UI 可跨 Repo 查询；
- 旧 API 在迁移窗口内保持兼容。

### Phase E'-4 Legacy Cleanup

归口：EVO-111。

- 删除 legacy Docker Skill Sandbox、bollard 和执行 facade；
- 不在核心链路稳定前为清理而冒险。

## 6. 当前严格启动顺序

```text
EVO-118-A Review/merge
→ EVO-118-B
→ EVO-118-C
→ EVO-118-D
→ EVO-118-E
→ EVO-118-F / G / H（按依赖和 WIP）
→ EVO-112
→ EVO-105 / 106 / 107 / 104
→ EVO-108 / 109 / 110
→ EVO-111
```

P0 安全、数据损坏或生产构建问题允许显式插队；普通 UI、视觉、内部文档、计费增强不得静默绕过 S1。

## 7. 阶段完成标准

### External Alpha

- SEC-01、SEC-02、DATA-01、DEPLOY-01、DATA-02、REL-01 关闭；
- clean production build 和启动通过；
- 容器重建不丢 Git 数据；
- DB+Git 从空环境恢复成功；
- readiness 故障返回 503；
- 权限和 SSRF 负向测试通过。

### Agent Beta

在 External Alpha 基础上：

- Commit/Promote + PolicyEvaluator 可用；
- Agent Session/Scoped Token 可创建、审计、撤销；
- Durable Event 在进程重启后不丢；
- Webhook 和 Indexer 幂等、有界重试、失败可观察；
- Vibe Coding UI 完成最小用户闭环。

### Production

在 Agent Beta 基础上：

- 备份恢复演练、容量和磁盘告警、发布回滚验证完成；
- PR/Main Branch Protection 和必要质量门禁生效；
- 生产安全配置 fail closed；
- 已知 P0/P1 残余有明确风险接受或关闭证据。

## 8. 暂缓事项

| 事项 | 暂缓原因 |
|------|----------|
| 微服务拆分 | 当前瓶颈是安全、数据和产品闭环，不是部署边界 |
| Kafka/NATS | Outbox + Worker 足以满足 MVP 可靠事件需求 |
| SSH / LFS | Smart HTTP MVP 与生产 Gate 先闭合 |
| 多地域 Git 复制 | 先完成单实例持久化、恢复和一致性 |
| Live preview / Web terminal / Yjs | 超出 Vibe Coding MVP |
| Wasmer/WASI runtime | Sandbox 删除后仍有明确执行需求再评估 |
| Phase F 计费增强 | 不抢占 EVO-118 与 Git 产品主线 |

## 9. 计划维护规则

1. 实施只从满足 DoR 的 Backlog Story 启动。
2. 每关闭一个 Gate，同步 Baseline、EVO-118、Backlog、Board、Iteration 和本路线图。
3. 原 [Two-Month Plan](TWO-MONTH-PLAN-2026-07.md) 保留为 2026-06-29 发布的历史计划基线；当前激活顺序以 Production Readiness Plan 为准。
4. 日期目标不能替代实际命令、负向测试、clean build 或恢复演练。
5. 新发现的越权、SSRF、数据损坏、事件丢失或生产构建问题必须进入 P0/P1 Backlog，不只留在评论或对话。
6. 重大边界变化继续写 ADR；本次优先级重排没有改变模块化单体和 Git-centric 已接受决策。
