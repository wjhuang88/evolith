# Operating Board

> Derived operating view only. Owner docs define status, scope, acceptance criteria,
> validation evidence and lifecycle state. Update owner docs first, then reflect the current
> operating state here.
>
> **2026-08-10 执行进展**：SEC-01、SEC-02、DATA-01、DATA-02 已解除。ADR-0010 将 EVO-118-E / DEPLOY-01 调整为目标产品开发与清理后的最终发布 Gate；关闭前可继续开发，但不得上线。H-A/H-B / Iterations 066/067 已 Closed / Complete；H-C / Iteration 068 Review / Partial。

## Now

| Item | State | Owner Doc | Gate |
|------|-------|-----------|------|
| EVO-118 Production Readiness Epic | In Progress | [Epic](backlog/active/EVO-118-production-readiness-and-security-hardening.md) | A/B/C/D/F、G-B/C/D、H-A/H-B Done；G-A Partial，H-C Review / Partial；E 保留为最终发布 Gate。 |
| EVO-118-G-A CI Merge Gates | Review / Partial | [Item](backlog/active/EVO-118-G-A-ci-merge-gates.md) | Iteration 056；本地门禁通过，Branch Protection 远端 403 residual。 |
| EVO-118-H-C Durable Push Event Integration | Review / Partial | [Item](backlog/active/EVO-118-H-C-durable-push-event-integration.md) | 核心 producer/subscriber 与双数据库证据通过；完整 Smart HTTP E2E pull clone 失败归 EVO-125。 |
| Iteration 068 | Review / Partial | [Iteration](iterations/ITERATION-068.md) | H-C 核心 durable event 闭环通过；等待 EVO-125 修复后重跑完整 Git E2E。 |

## Review

Iteration 056 已 Partial 收口；Iteration 060 Closed / Partial；Iterations 057~059、061~067 Closed / Complete。Iteration 068 当前为唯一 Review / Partial runtime Iteration，等待 EVO-125 处置。

## Done

| Item | State | Owner Doc | Evidence |
|------|-------|-----------|----------|
| EVO-118-H-B Outbox Worker Runtime | Done / Complete | [Item](backlog/active/EVO-118-H-B-outbox-worker-runtime.md) | Iteration 067；独立 Worker、bounded delivery、SQLite/PG 101 backlog/crash recovery、confirmed replay、workspace gates 与 Navigator 闭合。 |
| Iteration 067 | Closed / Complete | [Iteration](iterations/ITERATION-067.md) | continuous/once/replay、SIGINT、stable error code、双数据库进程证据完成；EVO-125 独立承接既有 Git E2E 不稳定。 |
| EVO-118-H-A Recoverable Outbox Claims | Done / Complete | [Item](backlog/active/EVO-118-H-A-recoverable-outbox-claims.md) | Iteration 066；paired 012 upgrade、SQLite 8/8、真实 PostgreSQL concurrent/upgrade 1/1、infra/check/clippy 与 Navigator 闭合。 |
| Iteration 066 | Closed / Complete | [Iteration](iterations/ITERATION-066.md) | Crash/reclaim、fencing、max-attempt dead-letter、非法边界 fail closed 与双数据库升级证据完成。 |
| EVO-121-D Settings Information Architecture | Done / Complete | [Item](backlog/active/EVO-121-D-settings-information-architecture.md) | Iteration 065；五个 `/settings/*`、owner/admin/member 矩阵、forbidden、500→Retry、truthful read-only、桌面/390px 与旧路由 404 闭合。 |
| Iteration 065 | Closed / Complete | [Iteration](iterations/ITERATION-065.md) | Focused 4/4、frontend 三门禁、locale 730、Markdown/diff/old-link checks、浏览器矩阵与 Navigator 无 blocking finding。 |
| EVO-121-C Public Entry And Auth Routing | Done / Complete | [Item](backlog/active/EVO-121-C-public-entry-and-auth-routing.md) | Iteration 064；完整 deep link、统一 Repo resolver、root/login/join/register/verification、403/5xx + Retry、truthful landing、桌面/390px 与安全负向矩阵闭合。 |
| Iteration 064 | Closed / Complete | [Iteration](iterations/ITERATION-064.md) | Pure policy 4/4、frontend gates、locale parity、Markdown/diff/governance、真实 SQLite + Git Storage 浏览器证据与 Navigator 无 blocking finding。 |
| EVO-112-C Commit Evidence Detail | Done / Complete | [Item](backlog/active/EVO-112-C-repo-commit-evidence-detail.md) | Iteration 063；tenant-scoped detail API、Ref reachability、root/structured diff、list deep link、404、桌面/390px 与全量门禁闭合。 |
| Iteration 063 | Closed / Complete | [Iteration](iterations/ITERATION-063.md) | Backend/Frontend/full workspace gates、locale parity、Markdown/diff/governance checks、浏览器证据与 Navigator 无 blocking finding。 |
| EVO-112 Repo Management UI | Done / Complete | [Item](backlog/active/EVO-112-repo-management-ui.md) | A/B/C 三个子 Story 全部 Done；Repo shell/list/create、Overview-first detail 与 Commit evidence deep link 形成基础 Repo Web 闭环。 |
| EVO-120 First-run Repo Onboarding | Done / Complete | [Item](backlog/active/EVO-120-first-run-repo-onboarding.md) | Iteration 062；seeded create -> Overview、普通/既有 Repo 登录分流、安全 redirect、500/403/409/Retry、桌面/390px 与键盘证据通过。 |
| Iteration 062 | Closed / Complete | [Iteration](iterations/ITERATION-062.md) | 前端三门禁、Markdown links、diff/governance checks 与 Navigator 复核闭合；EVO-124 独立承接 SQLite memory pool 缺陷。 |
| EVO-112-B Repo Detail Read-only | Done / Complete | [Item](backlog/active/EVO-112-B-repo-detail-read-only.md) | Iteration 061；四子路由、真实 Repo Context/CRUD、README/空/二进制、clipboard、PATCH/DELETE、404 与桌面/390px 浏览器证据通过。 |
| Iteration 061 | Closed / Complete | [Iteration](iterations/ITERATION-061.md) | 前端三门禁、Markdown links、diff/governance checks 与 Navigator 复核闭合。 |
| EVO-118-G-B Runtime Readiness | Done / Complete | [Item](backlog/active/EVO-118-G-B-runtime-readiness.md) | Health tests 10/10；runtime live=200、dependency failure ready=503、恢复后 ready=200；Docker/Compose 使用 `/health/ready`。 |
| Iteration 057 | Closed / Complete | [Iteration](iterations/ITERATION-057.md) | Readiness 单元、运行态故障/恢复和静态配置证据闭合。 |
| EVO-118-G-C Caller-aware Rate Limits | Done / Complete | [Item](backlog/active/EVO-118-G-C-caller-aware-rate-limits.md) | Caller-aware 专项矩阵、429、窗口恢复和 Git E2E 通过。 |
| Iteration 058 | Closed / Complete | [Iteration](iterations/ITERATION-058.md) | 限流实现与契约同步完成。 |
| EVO-118-G-D Dependency Fail Closed | Done / Complete | [Item](backlog/active/EVO-118-G-D-production-dependency-fail-closed.md) | SMTP/Redis 生产 fail-closed、开发 fallback 与 token 脱敏测试通过。 |
| Iteration 059 | Closed / Complete | [Iteration](iterations/ITERATION-059.md) | 生产关键依赖和日志安全边界闭合。 |
| EVO-118-F Repo Lifecycle | Done / Complete | [Item](backlog/active/EVO-118-F-repo-lifecycle-consistency.md) | Lifecycle 状态机、真实 Initial Commit、Reconciler、双数据库 migration/repository、失败注入和权限负向矩阵通过；DATA-02 Closed。 |
| Iteration 055 | Closed / Complete | [Iteration](iterations/ITERATION-055.md) | Workspace tests、strict Clippy、PostgreSQL 009 -> 010 upgrade/round trip 与 Navigator 阶段复核通过。 |
| EVO-118-D Git Durability and Recovery | Done / Complete / Merged | [Item](backlog/active/EVO-118-D-git-storage-durability-and-recovery.md) | PR #7 final Head `158ba98fb2d1e33fe5821f2e75431e86a5cf6ffd`；CI #204 / `30838250911` 与 container #50 / `30838250875` 成功；Navigator Complete；merge `932def05717b678f6f44dc23f137933d56158957`；DATA-01 Closed。 |
| Iteration 053 | Closed / Complete | [Iteration](iterations/ITERATION-053.md) | PostgreSQL + Git 联合备份恢复、负向矩阵、独立复验、合并与治理索引收口完成。 |
| EVO-118-C HTTP Tool Egress Security | Done / Complete / Merged | [Item](backlog/active/EVO-118-C-http-tool-egress-security.md) | PR #5 于 2026-08-02 合并，merge commit `936ed3b26a62840ddd94cf10e5075fd19e0a1c5c`；final-head CI #137 / run `30682419168` 全绿；SEC-02 已解除。 |
| Iteration 052 | Closed / Complete | [Iteration](iterations/ITERATION-052.md) | 统一 Egress、总 deadline、管理门禁、审计隐藏、负向安全矩阵、稳定契约和合并证据完成。 |
| EVO-112-A Repo UI Shell | Done / Complete / Merged | [Item](backlog/active/EVO-112-A-repo-ui-shell.md) | PR #8 于 2026-08-02 合并，merge commit `1216624`；exact-head CI run `30712179586` 全绿；创建成功返回 `/repos`。 |
| Iteration 054 | Closed / Complete | [Iteration](iterations/ITERATION-054.md) | 本地历史成果恢复、冲突迁移、主线重验、Navigator 与 PR #8 merge evidence 已闭环；不关闭最终 DEPLOY-01。 |
| EVO-119 Global Theme Consistency | Done / Complete | [Item](backlog/active/EVO-119-global-theme-consistency.md) | `:root.dark` 全局语义 token 修复；前端类型检查与构建通过。 |

## Blocked Or Paused

| Item | State | Owner Doc | Gate |
|------|-------|-----------|------|
| EVO-118-H Durable Event Epic | In Progress | [Item](backlog/active/EVO-118-H-durable-outbox-events.md) | H-A/H-B Done；H-C / Iteration 068 Review / Partial，EVENT-01 未关闭。 |
| EVO-121 Product Experience Convergence | In Progress | [Epic](backlog/active/EVO-121-product-experience-convergence.md) | EVO-121-C/D Done；其余四个子 Story 按各自依赖激活，不得用空路由、假数据或 Legacy 兼容页提前拼 Shell。 |
| EVO-105/106/107/104 Agent Write Loop | Proposed | [EVO-100](backlog/active/EVO-100-git-centric-platform-foundation.md) | 依赖 EVO-118-B/F/H 与 Repo UI 基础；不得绕过 Typed Capability、Policy 和 Durable Event。 |
| EVO-108/109 Index/Discovery | Proposed | [EVO-100](backlog/active/EVO-100-git-centric-platform-foundation.md) | 依赖稳定 Push/Commit Event 与 EVO-118-H；EVO-110 已 Dropped。 |
| EVO-122 Pre-launch Registry Backend Retirement | Proposed / paused | [Epic](backlog/active/EVO-122-retire-prelaunch-registry-backend.md) | Repo-derived execute 承接后直接删除 legacy runtime/table；不建设双写兼容。 |
| Iterations 018~020、027 | Superseded / Blocked for activation | [Iteration index](iterations/README.md) | 旧 Registry 主线被 Git-centric 方向替代，不 deliberate replan 则不激活。 |
| Iterations 025/026 | Blocked for activation | [Iteration index](iterations/README.md) | Phase F 独立候选，不抢占 EVO-118 S1。 |

## Next

| Item | State | Owner Doc | Gate |
|------|-------|-----------|------|
| EVO-118-H-C Durable Push Event Integration | Review / Partial | [Item](backlog/active/EVO-118-H-C-durable-push-event-integration.md) | Iteration 068；核心 producer/subscriber 通过，完整 Git E2E pull clone 认证失败归 EVO-125。 |

## Later

| Item | State | Owner Doc | Gate |
|------|-------|-----------|------|
| Repo-centric Web | Done / Complete | [EVO-120](backlog/active/EVO-120-first-run-repo-onboarding.md) / [EVO-112](backlog/active/EVO-112-repo-management-ui.md) | EVO-112-A/B/C 与 EVO-120 Done；最终 Entry/App Shell/Dashboard/Settings/Activity 收敛仍归 EVO-121。 |
| Product Experience Convergence | In Progress | [EVO-121](backlog/active/EVO-121-product-experience-convergence.md) | Entry 已完成；App Shell / Dashboard / Settings / Activity / Legacy UI 删除按依赖分批完成。 |
| Agent Integration + Vibe Coding | Proposed | [EVO-105/106/107/104](backlog/active/EVO-100-git-centric-platform-foundation.md) | S2 与 Repo UI 依赖满足。 |
| Indexer + Discovery | Proposed | [EVO-108/109](backlog/active/EVO-100-git-centric-platform-foundation.md) | Durable Event + Agent write semantics 稳定。 |
| Legacy Runtime Removal | Proposed / paused | [EVO-111](backlog/active/EVO-111-deprecate-sandbox-runtime.md) / [EVO-122](backlog/active/EVO-122-retire-prelaunch-registry-backend.md) | Repo-derived read/execute 已承接，consumer inventory 为零。 |
| EVO-118-E Final Production Convergence | Proposed / final gate | [Item](backlog/active/EVO-118-E-production-build-deployment-convergence.md) | 目标 MVP、F/G/H 与 legacy cleanup 全部完成后关闭 DEPLOY-01。 |
| Sandbox removal | Proposed | [EVO-111](backlog/active/EVO-111-deprecate-sandbox-runtime.md) | Index/compat 迁移稳定后收尾。 |
| SSH / LFS / Cross-repo search / Resource ACL | Proposal later | — | Phase E' 和 EVO-118 关闭后独立评估。 |

## Operating Review

- EVO-118-A/Iteration 050 已由 PR #2 合并并关闭。
- 当前产品方向仍是 Git hosting + Vibe Coding + capability discovery；EVO-118 是稳定化 Gate，不是产品回退。
- EVO-118-B/Iteration 051 已由 PR #3 合并并关闭；merge commit `6de7845e1231efc04f94f16cb9ab0a410f6ad2d9`，最终 CI run `30567361095` 全绿，SEC-01 已解除。
- EVO-118-C/Iteration 052 已由 PR #5 合并并关闭；实现 head `de762e2dae6bf5716e54c64e2277cb2e26592e36`，merge commit `936ed3b26a62840ddd94cf10e5075fd19e0a1c5c`，final-head CI #137 / run `30682419168` 全绿，SEC-02 已解除。
- EVO-112-A 的 2026-06-29 本地历史成果已按冲突恢复流程迁移为 Iteration 054；历史事实保留，当前顺序由 ADR-0010 重排。
- EVO-118-D / Iteration 053 已由 PR #7 合并并关闭；final Head `158ba98fb2d1e33fe5821f2e75431e86a5cf6ffd`，merge commit `932def05717b678f6f44dc23f137933d56158957`，required workflows 全绿，独立 Navigator `Complete`，DATA-01 已解除。
- 当前启动顺序：EVO-118-G-B/C/D、EVO-120、EVO-112-C 与 EVO-121-C 已完成，G-A/H 保留 Partial residual；下一候选须重新执行 DoR，并按 EVO-121-D → EVO-105/106/107/104 → EVO-121-E/B → EVO-108/109 → EVO-121-A/F → EVO-111 → EVO-122-A/B/C → EVO-118-E（最终生产收敛）推进。
- 安全、数据损坏或基础构建失败允许显式 P0 插队；DEPLOY-01 关闭前不得上线，但不阻塞普通产品开发。
- Board 只反映 owner docs；Gate 关闭必须由 Story 验收、最新 exact-head CI、独立 Navigator 和实际验证共同证明。
