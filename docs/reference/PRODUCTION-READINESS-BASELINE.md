# 生产就绪与项目完成度基线

> 基线日期：2026-09-10  
> 状态：Active release gate  
> 归口：[EVO-118 Production Readiness and Security Hardening](../backlog/active/EVO-118-production-readiness-and-security-hardening.md)  
> 整体项目状态：[Project Status Baseline 2026-09-10](PROJECT-STATUS-BASELINE-2026-09-10.md)  
> 当前执行顺序：[Current Execution Plan 2026-09](../roadmap/CURRENT-EXECUTION-PLAN-2026-09.md)

本文只负责**发布 Gate / 环境准入**。当前实现、未来架构和产品计划的统一判断由 Project Status Baseline 负责。

> **2026-09-10 amendment**：此前“filesystem Git + subprocess/gix 是长期总体方向、无需架构重构”的判断已被 [ADR-0011](../decisions/ADR-0011-walgit-backed-git-data-plane.md) 更新。当前 runtime 在 EVO-126-H 前仍是 filesystem-backed；长期目标改为 WalGit-backed object-store/WAL Git Data Plane。旧 DATA-01/DATA-02 的完成证据继续有效，但只证明旧引擎边界，不能被用来推导 GIT-DP-01 已关闭。

## 1. 当前发布结论

Evolith 当前可以描述为：

> **Git hosting / Repo Web Internal Alpha 基础可用；SEC-01、SEC-02、DATA-01、DATA-02、EVENT-01 已有闭环证据；REL-01 仍有 G-A 远端门禁 residual；WalGit-backed data-plane migration Gate GIT-DP-01 与最终 DEPLOY-01 均未关闭。因此不得声明 External Alpha / Agent Beta / Production ready。**

普通开发可以继续，但 2026-09 当前优先级先执行 EVO-126。最终发布 smoke、recovery、readiness 与 Git protocol 证据必须在 **EVO-126-H cutover 后的实际 data plane** 上重新验证。

## 2. 已关闭 Gate 与可继承语义

| Gate | 状态 | 已证明范围 | 新架构要求 |
|------|------|------------|------------|
| SEC-01 | Closed | API Key Owner/Admin、Typed Capability、MCP execute、tenant hiding | WalGit 接入不得降低授权边界 |
| SEC-02 | Closed | HTTP Tool DNS/IP/Redirect/Metadata/私网 Egress 防护 | Webhook/其他出站继续复用，不与 Git storage 变更混淆 |
| DATA-01 | Closed / historical filesystem engine | 单实例 persistent Git volume + PostgreSQL 联合 backup/restore、inventory、recovery | object-store durability/recovery 由 GIT-DP-01/EVO-126-F/H 重新证明 |
| DATA-02 | Closed / historical lifecycle | Repo state/compensation/reconciler/Initial Commit、DB/FS 分裂处理 | lifecycle 语义保留，实现迁移为 DB/WalGit namespace reconcile |
| EVENT-01 | Closed | recoverable Outbox claim、Worker、Push producer/subscriber/reconcile | 新 receive-pack/WAL write path 必须保持 durable event contract |

### 关键历史证据

- EVO-118-B / SEC-01：PR #3 merged `6de7845e1231efc04f94f16cb9ab0a410f6ad2d9`。
- EVO-118-C / SEC-02：PR #5 merged `936ed3b26a62840ddd94cf10e5075fd19e0a1c5c`；final-head CI 已通过。
- EVO-118-D / DATA-01：PR #7 merged `932def05717b678f6f44dc23f137933d56158957`；final head `158ba98fb2d1e33fe5821f2e75431e86a5cf6ffd`；联合 backup/restore、容器/应用恢复与 Navigator 证据闭合。
- EVO-118-F / DATA-02：Iteration 055 Closed / Complete；lifecycle、Initial Commit、Reconciler 与双数据库负向矩阵完成。
- EVO-118-H-A/B/C / EVENT-01：Iterations 066/067/068 Closed / Complete；Outbox claim/worker/Push event 与真实 Git E2E 完成。

上述记录仍由各 item/Iteration owner 保存；本文件不替代历史 Review evidence。

## 3. 当前开放 Gate

### REL-01 — Partial

归口：[EVO-118-G](../backlog/active/EVO-118-G-runtime-reliability-gates.md)。

- G-B Runtime Readiness：Done / Complete。
- G-C Caller-aware Rate Limits：Done / Complete。
- G-D Production Dependency Fail Closed：Done / Complete。
- G-A PR/Main Merge Gates：Review / Partial；Branch Protection 远端证据 residual。

该 residual 不要求把 EVO-126 塞入 Iteration 056，但 External Alpha/Production closure 前必须处置。

### GIT-DP-01 — Open

归口：[EVO-126](../backlog/active/EVO-126-walgit-git-data-plane-refactor.md)，决策：[ADR-0011](../decisions/ADR-0011-walgit-backed-git-data-plane.md)。

关闭条件：

- Rust 1.90 + exact WalGit dependency/license boundary 完成；
- `service-git v2` Store/Registry engine boundary 完成；
- Smart HTTP read v0/v2 与 WAL-backed receive-pack 完成；
- Repo Context 迁移到 RepoHandle/ObjectAccess 并保持原资源门禁；
- object-store config/readiness、existing repo migration、recovery rehearsal 完成；
- secure bundle-uri Agent clone path 完成；
- default production Git durable truth 切到 object store，本地 repo/pack 仅为 disposable cache；
- legacy filesystem Git engine consumer inventory 为零并清理/隔离；
- Push Outbox/Audit provenance 在最终路径通过；
- Security/Navigator/Data review 无 blocking finding。

**ADR 被接受、依赖被加入、或部分 E2E 通过都不能单独关闭 GIT-DP-01。**

### DEPLOY-01 — Open / final release gate

归口：[EVO-118-E](../backlog/active/EVO-118-E-production-build-deployment-convergence.md)。

只有目标产品、legacy cleanup、REL-01 residual 与 GIT-DP-01 全部满足后执行：

- `docker compose -f docker-compose.prod.yml build --no-cache`；
- Embedded Frontend 单一事实交付；
- production config fail closed；
- `/api/v1`、Git Smart HTTP、MCP、assets、SPA fallback 路由互不截获；
- login/Repo/clone/push/pull/Context/bundle/readiness/recovery smoke；
- 回滚/恢复边界和发布说明闭合。

## 4. 能力成熟度

| 能力 | 当前判断 | 发布含义 |
|------|----------|----------|
| 模块化单体 | Stable direction | 继续保持；WalGit 不作为第二个强制产品服务 |
| Auth/RBAC/API Key | Hardened baseline | 可作为 Internal Alpha 基础 |
| Repo CRUD / Lifecycle | Alpha + historical durability evidence | 新 storage cutover 后需重新做 migration/recovery 验证 |
| Git Smart HTTP | Alpha / current filesystem implementation | clone/push/pull 可用；长期实现正在 EVO-126 重构 |
| Repo Context / Commit Evidence | Alpha/usable | 语义需在新 engine 上保持 |
| Repo-centric Web | Alpha/usable foundation | list/create/detail/files/commits/evidence/onboarding/entry/settings 已有 |
| Durable Outbox / Push Event | Hardened baseline | 新 WAL write path 必须保持 |
| Commit/Promote | Not implemented | Agent Beta 不可依赖 |
| Agent Session / Scoped Token | Not implemented | Agent Beta 不可依赖 |
| Webhook / Indexer / Discovery | Proposed | 不可作为发布能力声明 |
| WalGit object-store data plane | Accepted target / not implemented | GIT-DP-01 Open |
| Final production delivery | Not complete | DEPLOY-01 Open |

## 5. 环境准入

| 环境 | 当前允许条件 |
|------|--------------|
| Local Development | 可使用 SQLite、临时 current Git storage；EVO-126 实施期间可使用 memory/MinIO/object-store test backend，必须明确非生产 |
| Internal Alpha | 当前已具备基础；仍须明确 filesystem-backed current runtime 与未关闭 GIT-DP-01/DEPLOY-01 |
| External Alpha | **REL-01 + GIT-DP-01 + DEPLOY-01** 关闭，且最终 data plane 上安全/恢复/smoke 通过 |
| Agent Beta | External Alpha + EVO-105/106 Commit/Promote/Policy/Agent Session 可用并可撤销/审计 |
| Production | Agent Beta + Webhook/Indexer/目标产品/legacy cleanup、备份恢复演练、容量/告警、回滚与安全复核闭合 |

## 6. 发布不变量

1. Current implementation 与 Accepted target 必须区分；EVO-126-H 前不得写“Git durable truth 已在 object store”。
2. WalGit auth/UI/product policy 不替代 Evolith Tenant/RBAC/Agent/Policy owner。
3. Agent generic `receive-pack` 默认 fail closed；最终 Agent write 仍经过 Commit/Promote + PolicyEvaluator。
4. Push/Commit/Promote/Webhook/Indexer 的不可丢事件继续走 Durable Outbox/Worker；WAL 不替代 Outbox。
5. Object store 失败必须进入 readiness/fail-closed，不允许因 local cache 尚可读而静默宣称完全健康。
6. 恢复/迁移不能覆盖未知非空目标；必须有 inventory、validation、dry-run/idempotency 或明确回滚。
7. DATA-01/DATA-02 历史完成可作为迁移必须保持的原则，不能冒充新 data plane 的完成证据。
8. 最终 production smoke 必须从 clean environment 对真实 Git client 验证，不以 object key/单元测试替代。

## 7. Owner 文档

- [Project Status Baseline](PROJECT-STATUS-BASELINE-2026-09-10.md) — 当前整体状态。
- [ADR-0011](../decisions/ADR-0011-walgit-backed-git-data-plane.md) — Git data-plane 决策。
- [WalGit Refactor Design](../design/WALGIT-GIT-DATA-PLANE-REFACTOR.md) — 目标结构和 migration/test matrix。
- [Current Execution Plan](../roadmap/CURRENT-EXECUTION-PLAN-2026-09.md) — 当前唯一激活顺序。
- [EVO-126](../backlog/active/EVO-126-walgit-git-data-plane-refactor.md) — GIT-DP-01 实施 owner。
- [EVO-118-E](../backlog/active/EVO-118-E-production-build-deployment-convergence.md) — DEPLOY-01 最终 owner。
