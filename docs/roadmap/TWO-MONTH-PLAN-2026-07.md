# Evolith 两个月执行规划（2026-07 ~ 2026-08）

> 发布状态：Historical planning baseline（首次发布 2026-06-29）  
> 当前激活状态：Superseded for activation ordering on 2026-07-30  
> 原范围：Phase E' Git 托管 + Vibe Coding 主线。  
> 当前替代顺序：[Production Readiness Plan](PRODUCTION-READINESS-PLAN-2026-07.md)。

## 0. 基线保护与偏差记录

本文保留 2026-06-29 发布时的目标、周计划、里程碑和风险规则，用于对照实际执行，不因延期或重排被改写为另一项工作的完成记录。

2026-07-30 复核结论：

- 原计划的 Repo UI、Commit/Promote、Agent Session、Webhook、Vibe Coding 和 Indexer 均未按目标日期进入完成状态。
- 全面体检确认 API Key/MCP 授权、HTTP Tool SSRF、Git 数据持久化/备份和 production build 为发布阻断项。
- 因此当前激活排序改为 EVO-118 S1/S2 → EVO-112 → EVO-105/106/107/104 → EVO-108/109/110。
- 本文中的目标日期只保留为计划偏差证据，不再授权 Agent 直接启动对应 Week Story。
- 实际开工仍必须从 [Product Backlog](../backlog/PRODUCT-BACKLOG.md) 选择满足 DoR 的 Ready Story，并按 START-ITERATION 盘点库存。

## 1. 2026-06-29 发布计划基线：当前基线

- EVO-101 / EVO-102 / EVO-103 / EVO-113 / EVO-115 / EVO-116 已完成，Git Repo 后端基础、Smart HTTP、Repo Context API、权限边界和资源边界可作为后续 UI / Agent / Indexer 基础。
- EVO-100 Epic 仍为 `In Progress`。剩余主线集中在 Repo UI、Commit/Promote、Agent Session、Webhook、Vibe Coding、Indexer/Discovery、旧表兼容和 Sandbox 删除。
- EVO-104 UX Gate U-01~U-05 已解除，但实现依赖 EVO-112、EVO-105 和 EVO-106。
- Iteration 025/026 是 Phase F 独立 blocked plan，不应抢占 Phase E' 主线。

> 2026-07-30 校准：上述“Git 后端基础”是 Alpha 基础，不代表 Production Readiness；新增 Gate 归 EVO-118。

## 2. 2026-06-29 发布计划基线：两个月目标

到 2026-08 末，原计划建议达成：

1. Web 应用从旧 Tools / Skills / CLI 入口切换为 Repo-centric 主入口，用户能在 UI 创建、浏览和管理 Git Repo。
2. API 支持受 Policy 控制的 Commit / Promote，为 Agent 写代码和直推直合提供闭环。
3. 外部 Agent Engine 通过 Session + Scoped Token 读取 Repo、提交变更、触发 Promote，并留下审计与事件。
4. Webhook Out 把 Push / Promote / Agent Event 通知外部 Engine，具备 HMAC、重试和失败记录。
5. Vibe Coding UI MVP：conversation-first workspace + Repo context + Diff / Commit 状态反馈。
6. Git Push/Commit 触发 Skill / CLI / MCP Indexer。
7. 前述链路稳定后启动 Sandbox 删除；风险超预期则顺延 EVO-111。

这些产品目标仍有效，但执行前增加 EVO-118 安全、耐久性和生产交付门禁。

## 3. 2026-06-29 发布计划基线：推荐节奏

| 周期 | 主目标 | 候选 Backlog | 原计划交付结果 | 原计划关键验证 | 2026-07-30 状态 |
|------|--------|--------------|----------------|------------------|-----------------|
| Week 1 | Repo UI 基础入口 | EVO-112-A | `/repos`、创建、导航、Dashboard | type-check/build/Playwright | 未启动；S1 后恢复 |
| Week 2 | Repo Detail 只读浏览 | EVO-112-B | Files/Commits/Settings/clone URL | push 后 UI 查看 | 未启动；S1 后恢复 |
| Week 3 | Commit API + Policy | EVO-105-A | file ops、commit、三态决策 | policy matrix/repo E2E/audit | 未启动；依赖 EVO-118-B/F/H |
| Week 4 | Promote + 写路径硬化 | EVO-105-B | FF/merge/conflict | promote/RBAC/clippy | 未启动；依赖 EVO-118-B/F/H |
| Week 5 | Agent Session + Token | EVO-106-A/B | Session、Scoped Token、Event | scope 403/state/agent mock | 未启动；依赖 EVO-118-B/H |
| Week 6 | Webhook Out | EVO-107 | HMAC/retry/failure record | mock server/retry/audit | 未启动；依赖 EVO-118-C/H |
| Week 7 | Vibe Coding UI MVP | EVO-104-A | conversation workspace/context/status | Playwright/mock flow | 未启动；依赖前述链路 |
| Week 8 | Indexer MVP | EVO-108-A | push/commit 索引三类能力 | 5s E2E/damaged input/perf | 未启动；依赖 EVO-118-H |

## 4. 原拆分建议

### EVO-112

- EVO-112-A Repo UI Shell：列表、创建、导航、Dashboard。
- EVO-112-B Repo Detail Read-only：Files、Commits、Settings。

### EVO-105

- EVO-105-A Commit API：file ops、Commit、Policy 三态。
- EVO-105-B Promote：FF、Merge Commit、冲突、Audit。

### EVO-106

- EVO-106-A Session Schema + Token。
- EVO-106-B Session Event + Middleware。

### EVO-104

- EVO-104-A Workspace MVP。
- EVO-104-B 后置 Diff/Conflict/Shortcut/Mobile 深化。

### EVO-108

- EVO-108-A Index Schema + Push Trigger。
- EVO-108-B Parser reuse + failure isolation + benchmark。

> 2026-07-30 补充：以上拆分继续有效，但必须将 Typed Capability、Egress Policy、Repo Lifecycle 和 Durable Outbox 作为硬依赖，而非在 Story 内临时补救。

## 5. 原依赖与门禁

| 依赖 | 原影响 | 原门禁 | 2026-07-30 增补 |
|------|--------|--------|-----------------|
| EVO-112 依赖 API contract | UI 暴露字段和错误 | 实施前重读 Contract | S1 关闭后实现 |
| EVO-105 写裸 Repo Ref | 数据损坏 | 临时 Repo E2E，失败 Ref 不移动 | 依赖 EVO-118-F/H |
| PolicyEvaluator | auto_merge 安全默认 | 默认 require_review + protected path | Typed Capability，Agent 不直接通用 receive-pack |
| EVO-106 Token | 越权风险 | scope helper + 403/audit | 依赖 EVO-118-B；可撤销 Session |
| EVO-107 出站请求 | 重试/雪崩 | timeout/retry/HMAC | 复用 EVO-118-C Egress + H Outbox |
| EVO-108 读 Git 内容 | 大 Commit/坏 Frontmatter | 文件数/大小/耗时限制 | 依赖持久事件和幂等 |

## 6. 原计划不纳入窗口

- SSH / LFS / protected branch 完整规则；
- 多人协作、live preview、web terminal、Yjs；
- 全仓 federation 或跨租户公开 discover；
- Wasmer/WASI 替代 Docker Sandbox；
- Phase F 计费/租户增强。

该判断继续有效。

## 7. 原验收总门槛

- 后端：相关 crate test，跨边界 Story 最终 workspace test + clippy。
- 前端：type-check、build、Playwright 桌面/移动截图。
- DB：SQLite/PostgreSQL 双轨 migration/repository/test。
- API Contract：先文档、再实现、同步 client。
- 治理：Iteration 记录真实命令、状态和残余；链接与 diff check。

2026-07-30 新增：

- Security negative tests；
- Git persistent volume；
- DB+Git restore drill；
- clean production build；
- readiness 503；
- Outbox restart/idempotency；
- 对应 Gate 未关闭时不得使用日期或局部测试替代。

## 8. 原里程碑与实际偏差

| 原日期目标 | 原里程碑 | 截至 2026-07-30 |
|------------|----------|-----------------|
| 2026-07-12 | Repo UI 可用 | 未完成，EVO-112 Proposed |
| 2026-07-26 | Commit / Promote API 可用 | 未完成，EVO-105 Proposed |
| 2026-08-09 | Session + Webhook 可联调 | 尚未启动，且依赖新的安全/事件 Gate |
| 2026-08-23 | Vibe Coding + Indexer MVP | 尚未启动 |
| 2026-08-31 | Phase E' 收尾复核 | 改为先复核 EVO-118 Gate，再重新估算产品里程碑 |

## 9. 原风险调整规则

- EVO-112 超过两周则只保留列表/创建/详情基础闭环。
- EVO-105 出现 Ref 一致性问题则停止 EVO-106/107，先修数据损坏。
- 外部 Agent Engine 不可用则使用本地 Mock。
- Indexer 事件不稳定则先使用显式 Job/DB Poll。
- 两个月只完成到 EVO-107 时也不为进度提前删除 Sandbox。

2026-07-30 追加：

- 任何越权、SSRF、Git 数据丢失或 clean build 失败立即阻断产品主线。
- 新里程碑只有在 EVO-118 S1 关闭后重新制定；不得继续引用旧日期作为承诺。

## 10. 当前下一步

1. 关闭 Iteration 050 / EVO-118-A Review。
2. 启动 EVO-118-B 单 Story 安全微迭代。
3. 按 B → C → D → E 顺序关闭 S1。
4. 再 refinement EVO-118-F/G/H 与 EVO-112。
5. 每周更新 [Production Readiness Plan](PRODUCTION-READINESS-PLAN-2026-07.md)；本文只保留历史基线和偏差证据。
