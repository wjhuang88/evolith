# Evolith 生产就绪优先级重排（2026-07-30）

> 状态：Current execution ordering  
> 触发：2026-07-30 全面项目体检  
> 归口：[EVO-118](../backlog/active/EVO-118-production-readiness-and-security-hardening.md)  
> 说明：本文替代原“两个月执行规划”作为当前激活顺序；原规划保留为历史计划基线，不覆写其原始目标和日期。

## 1. 重排原因

原计划要求：

- 2026-07-12 Repo UI 可用；
- 2026-07-26 Commit/Promote API 可用；
- 2026-08-09 Agent Session + Webhook 可联调；
- 2026-08-23 Vibe Coding + Indexer MVP。

截至 2026-07-30，EVO-112、EVO-105、EVO-106、EVO-107、EVO-108 仍未进入实现迭代。与此同时，静态代码与生产配置审查确认了权限、SSRF、Git 数据持久化、备份和生产构建等发布阻断项。

因此执行原则调整为：

> **先让底座安全、可持久化、可构建、可恢复，再把底座暴露给用户和 Agent。**

## 2. 当前产品判断

| 领域 | 判断 |
|------|------|
| Git 底座 | 已达到后端 Alpha，可继续硬化 |
| Web 产品 | 仍是旧 Registry 主入口，Repo-centric UI 未形成 |
| Agent 闭环 | Commit/Promote、Session、Scoped Token、Webhook 未实现 |
| 生产安全 | 被 API Key/MCP 授权和 HTTP Tool SSRF 阻断 |
| 数据耐久性 | 被 Git 目录持久化、备份恢复和 Repo 生命周期一致性阻断 |
| 交付链路 | Embedded Frontend 与生产 Docker/Compose 口径不一致 |

详见 [生产就绪与项目完成度基线](../reference/PRODUCTION-READINESS-BASELINE.md)。

## 3. 新的阶段顺序

### Stabilization S0 — 治理基线

归口：EVO-118-A。

交付：

- 生产就绪事实基线；
- 安全审查 SOP；
- P0/P1 可执行 Backlog；
- Board、Roadmap、Release Gate 和 Agent 入口同步。

该阶段不修运行时代码，只建立不允许被后续开发绕过的约束。

### Stabilization S1 — P0 安全与数据门禁

按 WIP 一次只推进一个 Story：

1. **EVO-118-B API Key / MCP Authorization — Done**
   - PR #3 merged `6de7845e1231efc04f94f16cb9ab0a410f6ad2d9`；最终 CI `30567361095` 全绿，SEC-01 已解除；
   - API Key 管理限制为 Owner/Admin；
   - Permission 强类型化；
   - MCP `tools/call` 强制 `execute` capability；
   - 现有 Key 审计与负向测试。
2. **EVO-118-C HTTP Tool Egress Security**
   - SSRF、DNS、Redirect、私网和 Metadata 拦截；
   - Tool 创建角色门禁；
   - 受控 Egress Client/Proxy。
3. **EVO-118-D Git Storage Durability and Recovery**
   - 生产 Git 持久卷；
   - PostgreSQL + Git 联合备份；
   - 从空环境恢复演练；
   - 容量与磁盘告警基线。
4. **EVO-118-E Production Build and Deployment Convergence**
   - Embedded Frontend 单一交付形态；
   - 修复 Docker build context；
   - clean build、启动和 Smoke Test；
   - 明确 Nginx 仅作为可选 Gateway。

S1 全部关闭前：

- 不发布外部 Alpha；
- 不宣称生产就绪；
- 不让新的 Agent 写入能力进入生产；
- EVO-112 可以做设计/refinement，但不应抢占实现 WIP。

### Stabilization S2 — 数据一致性与可靠性

1. **EVO-118-F Repo Lifecycle Consistency**
   - `CREATING/ACTIVE/ERROR` 或等价状态；
   - 删除 trash/reconcile；
   - Seed Template 形成真实 Initial Commit。
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

S2 可以在 S1 后按依赖推进；不得为了恢复产品进度跳过 DATA-02 或 EVENT-01。

### Product P1 — Repo-centric Web

S1 关闭后恢复：

1. EVO-112-A Repo UI Shell；
2. EVO-112-B Repo Detail Read-only。

目标仍是尽快让用户看到真正的 Git-centric 产品，但不能以牺牲发布门禁为代价。

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

### Product P3 — Capability Index and Discovery

依赖：EVO-118-H + 稳定 Push/Commit 事件。

1. EVO-108 Indexer；
2. EVO-109 Discovery API/UI；
3. EVO-110 旧表兼容；
4. EVO-111 Sandbox 删除收尾。

## 4. 启动顺序

当前建议严格使用以下顺序：

```text
EVO-118-A Done
→ EVO-118-B Done
→ EVO-118-C
→ EVO-118-D
→ EVO-118-E
→ EVO-118-F / G / H（按依赖）
→ EVO-112
→ EVO-105 / 106 / 107 / 104
→ EVO-108 / 109 / 110
→ EVO-111
```

如果出现安全或数据损坏修复，可以插队；普通 UI、视觉优化、内部文档页、计费增强不得绕过 S1。

## 5. Gate 解除标准

### Security Gate

- Member 无法创建或授予高权限 API Key；
- 无 `execute` capability 的 Key 无法调用 MCP Tool；
- localhost、私网、Metadata、DNS Rebinding 和 Redirect SSRF 测试通过；
- 跨租户访问统一拒绝且不泄露资源存在性。

### Durability Gate

- 后端容器重建后 Git Repo/Commit/Tag 保留；
- PostgreSQL + Git 备份可以在空环境恢复；
- Repo 创建/删除故障不产生不可追踪分裂；
- Git 存储容量和写入失败可观测。

### Deployment Gate

- `docker compose -f docker-compose.prod.yml build --no-cache` 通过；
- 生产镜像包含正确 Embedded Frontend；
- `/api/v1`、`/repos/`、`/mcp`、`/assets/`、SPA fallback 不互相截获；
- readiness 在依赖故障时返回 503；
- Smoke Test 覆盖登录、Repo、Git clone/push 和恢复后 clone。

### Agent Gate

- Commit/Promote 失败时 Ref 不移动；
- Agent Session 可撤销；
- Policy 三态实际生效；
- 事件在进程重启后不丢；
- Webhook/Indexer 幂等且失败有记录。

## 6. 计划维护规则

1. 本计划是当前顺序 owner doc；Board 只反映，不得自行改变顺序。
2. 每个 Story 关闭后更新 EVO-118 父项、Backlog、Board、Iteration 和生产基线。
3. 日期不再作为完成证据，Gate 和实际验证优先于日历承诺。
4. 原 [两个月执行规划](TWO-MONTH-PLAN-2026-07.md) 保留为 2026-06-29 发布的历史基线；不得把其原计划段改写为本轮新目标。
5. 如果业务要求提前恢复 Repo UI，必须显式记录风险接受者、未关闭 Gate 和环境限制；不得默认视为可外部发布。
