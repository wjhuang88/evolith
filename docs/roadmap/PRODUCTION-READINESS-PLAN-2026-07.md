# Evolith 生产就绪优先级重排（2026-07-30）

> **2026-09-10 状态变更**：本文完整保留 2026-07/08 的计划与执行证据，作为 **Historical execution baseline**；当前激活顺序已由 [Current Execution Plan 2026-09](CURRENT-EXECUTION-PLAN-2026-09.md) 替代。不得依据本文的旧“下一候选”启动新工作，也不得改写以下历史计划来承载 EVO-126。  
> 状态：Historical execution baseline / superseded for activation on 2026-09-10  
> 最近同步：2026-08-10（EVENT-01 已解除；以下正文保留当时逐步同步记录）  
> 触发：2026-07-30 全面项目体检  
> 归口：[EVO-118](../backlog/active/EVO-118-production-readiness-and-security-hardening.md)  
> 说明：本文曾替代原“两个月执行规划”作为当前激活顺序；2026-09-10 后同样作为历史计划基线保留，不覆写其原始目标和日期。

## 1. 重排原因

原计划要求：

- 2026-07-12 Repo UI 可用；
- 2026-07-26 Commit/Promote API 可用；
- 2026-08-09 Agent Session + Webhook 可联调；
- 2026-08-23 Vibe Coding + Indexer MVP。

截至 2026-07-30，EVO-112、EVO-105、EVO-106、EVO-107、EVO-108 仍未进入实现迭代。与此同时，静态代码与生产配置审查确认了权限、SSRF、Git 数据持久化、备份和生产构建等发布阻断项。

因此执行原则调整为：

> **先让底座安全、可持久化、可构建、可恢复，再把底座暴露给用户和 Agent。**

截至 2026-08-08，SEC-01、SEC-02、DATA-01、DATA-02 已关闭；按 ADR-0010，继续推进 G/H 与产品改造，DEPLOY-01 在最终发布阶段执行。

## 2. 当前产品判断（历史快照）

| 领域 | 判断 |
|------|------|
| Git 底座 | 已达到后端 Alpha，可继续硬化 |
| Web 产品 | 仍是旧 Registry 主入口，Repo-centric UI 未形成 |
| Agent 闭环 | Commit/Promote、Session、Scoped Token、Webhook 未实现 |
| 生产安全 | API Key/MCP 授权与 HTTP Tool SSRF Gate 已关闭；未来出站能力必须复用统一 Egress Policy |
| 数据耐久性 | DATA-01 单实例联合耐久性已关闭；Repo 生命周期一致性与多副本共享仍归后续 Gate |
| 交付链路 | Embedded Frontend 与生产 Docker/Compose 口径不一致 |

> 该表是当时快照，后续 Repo Web、DATA-02、EVENT-01 等已继续推进。2026-09 当前事实请读 [Project Status Baseline](../reference/PROJECT-STATUS-BASELINE-2026-09-10.md)。

## 3. 当时的阶段顺序

### Stabilization S0 — 治理基线

归口：EVO-118-A（Done）。

交付：

- 生产就绪事实基线；
- 安全审查 SOP；
- P0/P1 可执行 Backlog；
- Board、Roadmap、Release Gate 和 Agent 入口同步。

该阶段不修运行时代码，只建立不允许被后续开发绕过的约束。

### Stabilization S1 — P0 安全与数据门禁

按 WIP 一次只推进一个 Story：

1. **EVO-118-B API Key / MCP Authorization — Done / SEC-01 Closed**
   - PR #3 merged `6de7845e1231efc04f94f16cb9ab0a410f6ad2d9`；最终 CI `30567361095` 全绿；
   - API Key 管理限制为 Owner/Admin；
   - Permission 强类型化；
   - MCP `tools/call` 强制 `execute` capability；
   - 现有 Key 审计与负向测试。
2. **EVO-118-C HTTP Tool Egress Security — Done / SEC-02 Closed**
   - Navigator accepted runtime security；CI #125 / run `30653767138` 全绿；
   - SSRF、DNS、Redirect、私网、Metadata 和特殊用途地址拦截；
   - Tool create/update 同租户 Owner/Admin JWT 门禁；
   - 受控 Egress Client、总 deadline、连接固定、审计隐藏；
   - `PERMISSIONS.md` 与 `API-CONTRACT.md` 稳定契约完成。
3. **EVO-118-D Git Storage Durability and Recovery — Done / DATA-01 Closed**
   - PR #7 final Head `158ba98fb2d1e33fe5821f2e75431e86a5cf6ffd`，required workflows 全绿，Navigator Complete；
   - squash merge commit `932def05717b678f6f44dc23f137933d56158957`；
   - 生产 Git 持久卷、Git readiness、PostgreSQL + Git 联合备份、安全恢复、inventory、故障矩阵和空环境演练完成；
   - Subscription credential 与 `psqlrc` 环境隔离边界闭合。

S1 安全与数据基线已关闭。DEPLOY-01 不属于普通开发前置：

- 不发布外部 Alpha；
- 不宣称生产就绪；
- 不让新的 Agent 写入能力进入生产；
- EVO-112、Onboarding、Agent、Discovery 和 Experience Convergence 可按下方依赖继续开发；仍不得发布外部 Alpha。

### Stabilization S2 — 数据一致性与可靠性

1. **EVO-118-F Repo Lifecycle Consistency — Done / DATA-02 Closed**
   - `CREATING/ACTIVE/ERROR/DELETING` 状态与失败补偿；
   - DB/FS Reconcile 与 disk-only quarantine；
   - Seed Template 形成真实 Initial Commit；
   - SQLite/PostgreSQL、权限负向和失败注入证据通过。
2. **EVO-118-G Runtime Reliability Gates**
   - PR/Main CI；
   - readiness 失败返回 503；
   - authenticated/API Key 限流接线；
   - SMTP production fail closed；
   - 关键依赖降级语义明确。
3. **EVO-118-H Durable Outbox and Events**
   - Push/Commit/Promote 事件持久化；
   - Worker 重试、幂等和失败记录；
   - 为 EVO-107 Webhook 和 EVO-108 Indexer 提供可靠基础。

2026-08-09 refinement：Iteration 060 的 schema/repository boundary 保持 Closed / Partial；剩余范围拆为 H-A recoverable claims、H-B Worker runtime、H-C durable Push integration，按依赖顺序执行。尚不存在的 Commit/Promote/Agent Session producer 由 EVO-105/106 在本边界完成后接入，避免 H 与产品 Story 形成循环依赖。

H-A / Iteration 066 已 Done / Closed / Complete：paired 012 upgrade、lease/fencing、stale recovery 与 PostgreSQL concurrent claim 已验证。H-B / Iteration 067 已 Done / Closed / Complete：独立 Worker、bounded delivery、双数据库积压恢复与 confirmed replay 已验证。H-C / Iteration 068 已 Done / Closed / Complete：真实 Push producer/subscriber、reconcile、SQLite/PostgreSQL 与完整 Smart HTTP E2E 已验证，EVENT-01 Durable Outbox 基础 Gate 已关闭。

S2 在 S1 后按依赖推进；DATA-02 与 EVENT-01 已关闭，REL-01 仍保留 G-A 远端证据 residual；当时下一候选按 DoR 从 EVO-105/106 产品链选择。**该“下一候选”已于 2026-09-10 被 ADR-0011/EVO-126 replan 取代。**

### Product P1 — Repo-centric Web

S1 关闭后恢复：

1. EVO-112-A Repo UI Shell（Done）；
2. EVO-120 First-run Repo Onboarding；
3. EVO-112-B Overview-first Repo Detail Read-only；
4. EVO-112-C Repo Commit Evidence Detail（Done / Iteration 063 Closed / Complete）。

2026-08-08 交互架构校准后，首次使用不再创建 Tool，而是创建/导入 Repo 并进入 Overview。该调整仅重排未来未启动工作，不覆写既有 Planned Iteration 基线。产品流程以 [Product Interaction Architecture](../design/PRODUCT-INTERACTION-ARCHITECTURE.md) 和 [ADR-0008](../decisions/ADR-0008-repo-centric-interaction-architecture.md) 为准。

### Product P2 — Agent Write Loop

依赖：EVO-118-B/F/H + EVO-112。

1. EVO-105-A Commit API + PolicyEvaluator；
2. EVO-105-B Promote；
3. EVO-106 Agent Session + Scoped Token；
4. EVO-107 Webhook Out；
5. EVO-104 Vibe Coding UI MVP。

约束：

- Agent Token 不得直接获得不受策略控制的通用 Smart HTTP Push。
- `commit:<scope>` 必须落实为 branch/path capability。
- 所有写入产生可审计事件并进入 Durable Outbox。
- Webhook 和其他租户可控出站调用必须复用 EVO-118-C Egress Policy。

> 2026-09-10 amendment：该阶段仍是产品目标，但激活前新增 GIT-DP-01 / EVO-126-H 依赖，避免在旧 filesystem/subprocess write path 上固定最终 Agent write architecture。

### Product P3 — Capability Index and Discovery

依赖：EVO-118-H + 稳定 Push/Commit 事件。

1. EVO-108 Indexer；
2. EVO-109 Discovery API/UI。

ADR-0009 已取消 EVO-110 双写兼容；旧 runtime/schema 清理由 EVO-111/EVO-122 在 Repo-derived 合约承接后直接完成。

### Product P4 — Experience Convergence

归口：[EVO-121](../backlog/active/EVO-121-product-experience-convergence.md)，按依赖穿插在 P1-P3 之后：

1. EVO-121-C/D：public/auth entry resolver 与 Settings IA；
2. EVO-121-E/B：Durable Activity 与 task-first Dashboard；
3. EVO-121-A/F：最终 App Shell 与旧 Registry UI 删除。

目标页面、路由和状态边界以 [Product Interaction Architecture](../design/PRODUCT-INTERACTION-ARCHITECTURE.md) 为准。旧 UI/API 不承担线上兼容义务；后端按 ADR-0009 / EVO-122 直接收敛。

### Final Release P5 — Production Convergence

归口：EVO-118-E。

原进入条件：EVO-118-F/G/H、目标 MVP owner Story、EVO-111 与 EVO-122-A/B/C 全部完成。

2026-09-10 amendment：最终 release 还必须等待 GIT-DP-01 关闭，并在 WalGit-backed data plane 上重跑 clean build、readiness/recovery 与完整 Git smoke。

## 4. 历史启动顺序

本文当时记录的顺序为：

```text
EVO-118-A Done
→ EVO-118-B Done
→ EVO-118-C Done
→ EVO-118-D Done
→ EVO-118-F Done
→ EVO-118-G / H
→ EVO-120 / EVO-112-A/B/C
→ EVO-121-C / D
→ EVO-105 / 106 / 107 / 104
→ EVO-121-E / B
→ EVO-108 / 109
→ EVO-121-A / F
→ EVO-111
→ EVO-122-A / B / C
→ EVO-118-E
```

**当前不得按上述历史顺序启动。** 2026-09-10 后执行顺序见 [CURRENT-EXECUTION-PLAN-2026-09.md](CURRENT-EXECUTION-PLAN-2026-09.md)：先 EVO-126 A-H / GIT-DP-01，再恢复 Agent Write Loop。

## 5. Gate 解除标准（历史 owner + 当前 amendment）

### Security Gate — Closed

- Member 无法创建或授予高权限 API Key；
- 无 `execute` capability 的 Key 无法调用 MCP Tool；
- localhost、私网、Metadata、DNS Rebinding 和 Redirect SSRF 测试通过；
- 跨租户访问统一拒绝且不泄露资源存在性；
- Tool create/update 权限和 API 契约已与实现同步。

### Historical Durability Gate — Closed for filesystem engine

- 后端容器重建后 Git Repo/Commit/Tag 保留；
- PostgreSQL + Git 备份可以在空环境恢复；
- Repo 创建/删除故障不产生不可追踪分裂；
- Git 存储容量和写入失败可观测。

以上由 DATA-01/DATA-02 历史证据关闭。它不代表新 object-store/WAL 架构已经验证；2026-09 新增 **GIT-DP-01** 负责 WalGit-backed migration/readiness/recovery/cutover。

### Deployment Gate — Open

- `docker compose -f docker-compose.prod.yml build --no-cache` 通过；
- 生产镜像包含正确 Embedded Frontend；
- `/api/v1`、`/repos/`、`/mcp`、`/assets/`、SPA fallback 不互相截获；
- readiness 在依赖故障时返回 503；
- Smoke Test 覆盖登录、Repo、Git clone/push 和恢复后 clone；
- 2026-09 起这些 Git smoke 必须使用最终 WalGit-backed data plane。

### Agent Gate — Future

- Commit/Promote 失败时 Ref 不移动；
- Agent Session 可撤销；
- Policy 三态实际生效；
- 事件在进程重启后不丢；
- Webhook/Indexer 幂等且失败有记录；
- GIT-DP-01 已关闭。

## 6. 计划维护规则

1. 本文从 2026-09-10 起是历史执行基线，不再是当前顺序 owner；当前 owner 是 [Current Execution Plan 2026-09](CURRENT-EXECUTION-PLAN-2026-09.md)。
2. 本文已有 2026-07/08 目标、范围和执行证据必须保留；后续改线只追加 amendment，不将旧段改写成 EVO-126 的实施记录。
3. 当前 Story 状态与 Required Reads 以 [Product Backlog](../backlog/PRODUCT-BACKLOG.md) 为准。
4. 原 [Two-Month Plan](TWO-MONTH-PLAN-2026-07.md) 同样保留为更早历史计划基线。
5. 日期不作为完成证据，Gate、真实命令、负向测试、migration/recovery 和 Review 优先于日历承诺。
