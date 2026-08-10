# Product Backlog

> Compact routing and prioritization surface. Executable context lives in `docs/backlog/active/`; completed, deferred and dropped history lives in `docs/backlog/archive/`.
> Status and DoR rules: [Requirement Intake](../sop/REQUIREMENT-INTAKE.md). Completion rules: [Iteration Workflow](../sop/ITERATION-WORKFLOW.md). Compaction protocol: `agent-project-governance/references/backlog-compaction.md`.

> **2026-08-09 执行重排**：SEC-01、SEC-02、DATA-01、DATA-02 已解除。按 ADR-0010，EVO-118-E / DEPLOY-01 移到目标产品开发与 legacy cleanup 完成后的最终发布阶段；EVO-120 / Iteration 062 已 Closed / Complete。

## Current Priorities

| ID | Title | Status | Priority | Decision Context | Required Reads |
| --- | --- | --- | --- | --- | --- |
| EVO-118-E | 最终 Embedded Frontend 生产构建与部署收敛 | Proposed / final release gate | P0 | 目标产品开发、F/G/H 与 legacy cleanup 完成后执行；关闭 DEPLOY-01，生产交付口径见 ADR-0007/0010 | [Item file](active/EVO-118-E-production-build-deployment-convergence.md)<br>[Release SOP](../sop/RELEASE.md)<br>[Readiness plan](../roadmap/PRODUCTION-READINESS-PLAN-2026-07.md)<br>[ADR-0010](../decisions/ADR-0010-final-production-convergence-after-product-completion.md) |
| EVO-118-F | Repo 生命周期一致性与 Initial Commit | Done / Complete | P1 | Iteration 055 Closed；DATA-02 Closed | [Item file](active/EVO-118-F-repo-lifecycle-consistency.md)<br>[Iteration 055](../iterations/ITERATION-055.md) |
| EVO-112 | Repo Management UI（仓库列表 / 创建 / 详情 / Commit 证据） | Done / Complete | P0 | EVO-112-A/B/C 全部 Done；Iteration 063 Closed / Complete，普通 Git Commit list -> evidence deep link 已闭合 | [Item file](active/EVO-112-repo-management-ui.md)<br>[EVO-112-A](active/EVO-112-A-repo-ui-shell.md)<br>[EVO-112-B](active/EVO-112-B-repo-detail-read-only.md)<br>[EVO-112-C](active/EVO-112-C-repo-commit-evidence-detail.md)<br>[Iteration 063](../iterations/ITERATION-063.md)<br>[Interaction Architecture](../design/PRODUCT-INTERACTION-ARCHITECTURE.md)<br>[ADR-0008](../decisions/ADR-0008-repo-centric-interaction-architecture.md) |
| EVO-120 | First-run Repo Onboarding | Done / Complete | P0 | Iteration 062 Closed；无 Repo 创建后进入 seeded Overview，已有 Repo 进入 `/dashboard`，完整 entry resolver 仍归 EVO-121-C | [Item file](active/EVO-120-first-run-repo-onboarding.md)<br>[Iteration 062](../iterations/ITERATION-062.md)<br>[ADR-0008](../decisions/ADR-0008-repo-centric-interaction-architecture.md) |
| EVO-118-H-A | 可回收 Outbox Claim 与 PostgreSQL 并发语义 | Done / Complete | P1 | Iteration 066 Closed；crash recovery、fencing、paired upgrade 与 PostgreSQL concurrency 已闭合 | [Item file](active/EVO-118-H-A-recoverable-outbox-claims.md)<br>[Parent H](active/EVO-118-H-durable-outbox-events.md)<br>[Iteration 066](../iterations/ITERATION-066.md) |
| EVO-118-H-B | Outbox Worker 运行生命周期 | Done / Complete | P1 | Iteration 067 Closed；独立 worker、bounded delivery、101 backlog、crash recovery 与 confirmed replay 已闭合 | [Item file](active/EVO-118-H-B-outbox-worker-runtime.md)<br>[Parent H](active/EVO-118-H-durable-outbox-events.md)<br>[Iteration 067](../iterations/ITERATION-067.md)<br>[Architecture](../reference/ARCHITECTURE.md)<br>[Security Review](../sop/SECURITY-REVIEW.md) |
| EVO-105 | Commit API + 直推直合 + Promote API | Proposed | P0 | 依赖 EVO-118-B/F/H；Agent 写入必须经过 Typed Capability、PolicyEvaluator 与 Durable Event；Policy 优先级已由 ADR-0007 固定 | [Item file](active/EVO-105-commit-and-promote-api.md)<br>[EVO-100](active/EVO-100-git-centric-platform-foundation.md)<br>[Baseline](../reference/PRODUCTION-READINESS-BASELINE.md)<br>[ADR-0007](../decisions/ADR-0007-agent-write-and-production-delivery-boundaries.md) |
| EVO-106 | Agent Session API + Scoped Token | Proposed | P0 | 依赖 EVO-118-B/F/H；Scoped Token 采用独立、可撤销 opaque credential，不退化为通用 Smart HTTP write | [Item file](active/EVO-106-agent-session-and-scoped-token.md)<br>[EVO-100](active/EVO-100-git-centric-platform-foundation.md)<br>[Security Review](../sop/SECURITY-REVIEW.md)<br>[ADR-0007](../decisions/ADR-0007-agent-write-and-production-delivery-boundaries.md) |
| EVO-080 | Spike: 验证 Wasmer/WASI 替代 Docker sandbox 可行性 | Ready | P1 | 仅在 EVO-111 删除后仍需要 runtime 候选时启动；不得绕过 EVO-118 S1 | [Item file](active/EVO-080-spike-验证-wasmer-wasi-替代-docker-sandbox-可行性.md)<br>[Serverless runtime proposal](../proposals/SERVERLESS-RUNTIME.md) |
| EVO-081 | 内部文档页面基于独立 Markdown 目录渲染 | Ready | P1 | 可作为独立产品 Story，但不得抢占 EVO-118 S1 | [Item file](active/EVO-081-内部文档页面基于独立-markdown-目录渲染.md)<br>[Product requirements](../reference/product/REQUIREMENTS.md) |
| EVO-057 | 生产 CORS Origin 可配置化 | Ready | P2 | 可与 EVO-118-G/部署硬化联动，不单独绕过 S1 | [Item file](active/EVO-057-生产-cors-origin-可配置化.md)<br>[Config reference](../reference/CONFIG.md) |

## Active Items

| ID | Title | Status | Priority | Decision Context | Required Reads |
| --- | --- | --- | --- | --- | --- |
| EVO-118 | Epic: Production Readiness and Security Hardening | In Progress | P0 | A/B/C/D/F Done；G-A 在 Iteration 056；G-B/C/D/H 后续；E 为目标开发与清理后的最终发布 Gate | [Item file](active/EVO-118-production-readiness-and-security-hardening.md)<br>[Baseline](../reference/PRODUCTION-READINESS-BASELINE.md)<br>[Readiness plan](../roadmap/PRODUCTION-READINESS-PLAN-2026-07.md) |
| EVO-118-A | 项目体检治理基线与优先级重排 | Done | P0 | Governance Story；Iteration 050 Closed；PR #2 merged | [Item file](active/EVO-118-A-project-health-governance-baseline.md)<br>[Iteration 050](../iterations/ITERATION-050.md) |
| EVO-118-B | API Key 与 MCP 授权边界硬化 | Done | P0 | Iteration 051 Closed；PR #3 merged `6de7845e1231efc04f94f16cb9ab0a410f6ad2d9`；CI `30567361095` 全绿，SEC-01 已解除 | [Item file](active/EVO-118-B-api-key-mcp-authorization-hardening.md)<br>[Iteration 051](../iterations/ITERATION-051.md) |
| EVO-118-C | HTTP Tool 出站安全与 SSRF 防护 | Done | P0 | Iteration 052 Closed / Complete；Navigator accepted runtime security；CI #125 / run `30653767138` 全绿；稳定契约完成，SEC-02 已解除 | [Item file](active/EVO-118-C-http-tool-egress-security.md)<br>[Iteration 052](../iterations/ITERATION-052.md)<br>[Permissions](../reference/PERMISSIONS.md)<br>[API Contract](../reference/API-CONTRACT.md) |
| EVO-118-D | Git 存储持久化、备份与恢复演练 | Done / Complete / Merged | P0 | Iteration 053 Closed；DATA-01 Closed；PR #7 final Head `158ba98fb2d1e33fe5821f2e75431e86a5cf6ffd`，merge `932def05717b678f6f44dc23f137933d56158957`，Navigator Complete，required workflows success | [Item file](active/EVO-118-D-git-storage-durability-and-recovery.md)<br>[Iteration 053](../iterations/ITERATION-053.md)<br>[PR #7](https://github.com/wjhuang88/evolith/pull/7) |
| EVO-118-E | 最终 Embedded Frontend 生产构建与部署收敛 | Proposed / final release gate | P0 | 目标 MVP、F/G/H 与 legacy cleanup 完成后执行，解除 DEPLOY-01 | [Item file](active/EVO-118-E-production-build-deployment-convergence.md) |
| EVO-118-F | Repo 生命周期一致性与 Initial Commit | Done / Complete | P1 | Iteration 055 Closed；扩展并替代 EVO-114 的单点回滚范围；DATA-02 Closed | [Item file](active/EVO-118-F-repo-lifecycle-consistency.md)<br>[Iteration 055](../iterations/ITERATION-055.md) |
| EVO-118-G | Epic: 运行可靠性与发布门禁接线 | In Progress | P1 | G-A/B/C/D 分批关闭 REL-01；G-A 在 Iteration 056 | [Item file](active/EVO-118-G-runtime-reliability-gates.md) |
| EVO-118-G-A | PR/Main 自动质量门禁 | Review / Partial | P1 | Iteration 056；本地门禁完成，Branch Protection 远端证据 residual | [Item file](active/EVO-118-G-A-ci-merge-gates.md)<br>[Iteration 056](../iterations/ITERATION-056.md) |
| EVO-118-G-B | Runtime Readiness 真实反映依赖 | Done / Complete | P1 | Iteration 057；DB/Git Storage 200/503 runtime smoke | [Item file](active/EVO-118-G-B-runtime-readiness.md)<br>[Iteration 057](../iterations/ITERATION-057.md) |
| EVO-118-G-C | Caller-aware 分层限流 | Done / Complete | P1 | Iteration 058；匿名/JWT/API Key/单 Key 隔离与单 Key 上限 | [Item file](active/EVO-118-G-C-caller-aware-rate-limits.md)<br>[Iteration 058](../iterations/ITERATION-058.md) |
| EVO-118-G-D | 生产关键依赖 Fail Closed | Done / Complete | P1 | Iteration 059；SMTP/Redis/log redaction | [Item file](active/EVO-118-G-D-production-dependency-fail-closed.md)<br>[Iteration 059](../iterations/ITERATION-059.md) |
| EVO-118-H | Epic: Durable Outbox 与可靠事件交付 | In Progress | P1 | Iteration 060 Partial；H-A/H-B Done，H-C 待续 | [Item file](active/EVO-118-H-durable-outbox-events.md)<br>[H-A](active/EVO-118-H-A-recoverable-outbox-claims.md)<br>[H-B](active/EVO-118-H-B-outbox-worker-runtime.md)<br>[Iteration 067](../iterations/ITERATION-067.md) |
| EVO-118-H-A | 可回收 Outbox Claim 与 PostgreSQL 并发语义 | Done / Complete | P1 | Iteration 066；lease/fencing、stale recovery、PG concurrent claim 与 paired upgrade | [Item file](active/EVO-118-H-A-recoverable-outbox-claims.md)<br>[Iteration 066](../iterations/ITERATION-066.md) |
| EVO-118-H-B | Outbox Worker 运行生命周期 | Done / Complete | P1 | Iteration 067 Closed；独立运行、积压恢复、受控 replay 与双数据库进程证据完成 | [Item file](active/EVO-118-H-B-outbox-worker-runtime.md)<br>[Iteration 067](../iterations/ITERATION-067.md) |
| EVO-118-H-C | Durable Push Event 业务接入 | Proposed | P1 | 依赖 H-B；现有 Push 派生工作迁移到持久事件 | [Item file](active/EVO-118-H-C-durable-push-event-integration.md) |
| EVO-113 | Direction Pivot Review Remediation | Done | P0 | 2026-06-25 方向变更评审缺口修复完成 | [Item file](active/EVO-113-direction-pivot-review-remediation.md) |
| EVO-115 | git Smart HTTP WWW-Authenticate + 真实 git-client E2E | Done | P1 | ITERATION-047 完成 | [Item file](active/EVO-115-git-www-authenticate-and-real-e2e.md) |
| EVO-116 | EVO-103 验收硬化 | Done | P0 | ITERATION-049 闭环；不等于平台生产就绪，后续 Gate 归 EVO-118 | [Item file](active/EVO-116-evo-103-acceptance-hardening.md)<br>[EVO-103 acceptance](../review/EVO-103-acceptance.md) |
| EVO-117 | 当前架构与依赖文档基线校准 | Done | P1 | 2026-07-29 文档漂移修复 | [Item file](active/EVO-117-current-documentation-baseline.md) |
| EVO-100 | Epic: Git-Centric Platform Foundation | In Progress | P0 | 产品方向不变；当前受 EVO-118-F/G/H 直接边界约束，最终发布由 EVO-118-E 收口 | [Item file](active/EVO-100-git-centric-platform-foundation.md)<br>[Proposal](../proposals/GIT-CENTRIC-PLATFORM.md)<br>[ADR-0004](../decisions/ADR-0004-git-centric-storage.md) |
| EVO-101 | `git_repos` 表 + 双轨 migration | Done | P0 | Iteration 042 | [Item file](active/EVO-101-git-repos-schema.md) |
| EVO-102 | `.evolith/policy.yaml` 规范 + 解析器 | Done | P1 | Parser Done；策略执行仍归 EVO-105/118-B | [Item file](active/EVO-102-evolith-policy-yaml.md) |
| EVO-103 | Repo CRUD + Smart HTTP + Repo Context API | Done | P0 | 后端 Git Alpha 基础；生命周期和生产 Gate 仍归 EVO-118 | [Item file](active/EVO-103-repo-context-and-smart-http.md) |
| EVO-103-A | Repo CRUD（gix::init + RBAC） | Done | P0 | ITERATION-044 | [Item file](active/EVO-103-A-repo-crud.md) |
| EVO-103-B | Smart HTTP git 协议 | Done | P0 | B-1/B-2 Done | [Item file](active/EVO-103-B-smart-http-git-protocol.md) |
| EVO-103-B-1 | Git 客户端认证基础设施 | Done | P0 | ITERATION-045 | [Item file](active/EVO-103-B-1-git-client-auth-infra.md) |
| EVO-103-B-2 | Smart HTTP 端点实现 | Done | P0 | ITERATION-046 | [Item file](active/EVO-103-B-2-smart-http-endpoints.md) |
| EVO-103-C | Repo Context API | Done | P0 | ITERATION-048 | [Item file](active/EVO-103-C-repo-context-api.md) |
| EVO-112 | Repo Management UI | Done / Complete | P0 | EVO-112-A/B/C Done；Iterations 054/061/063 Closed / Complete | [Item file](active/EVO-112-repo-management-ui.md) |
| EVO-112-A | Repo UI Shell | Done | P0 | `/repos` 列表与创建、repo-centric 导航和 Dashboard；Iteration 054 Closed / Complete；PR #8 merged `1216624` | [Item file](active/EVO-112-A-repo-ui-shell.md)<br>[Iteration 054](../iterations/ITERATION-054.md)<br>[PR #8](https://github.com/wjhuang88/evolith/pull/8) |
| EVO-119 | 全局主题切换保持界面颜色一致 | Done | P1 | `:root.dark` 覆盖全局语义 token，Header/Sidebar/卡片/表单跟随主题 | [Item file](active/EVO-119-global-theme-consistency.md) |
| EVO-120 | First-run Repo Onboarding | Done / Complete | P0 | Iteration 062 Closed；首次使用创建 seeded Repo 后进入真实 Overview，不再创建 legacy Tool | [Item file](active/EVO-120-first-run-repo-onboarding.md)<br>[Iteration 062](../iterations/ITERATION-062.md)<br>[ADR-0008](../decisions/ADR-0008-repo-centric-interaction-architecture.md) |
| EVO-121 | Epic: Product Experience Convergence | In Progress | P0 | EVO-121-C/D Done / Complete；其余子 Story 按依赖独立激活 | [Item file](active/EVO-121-product-experience-convergence.md)<br>[Interaction Architecture](../design/PRODUCT-INTERACTION-ARCHITECTURE.md)<br>[ADR-0008](../decisions/ADR-0008-repo-centric-interaction-architecture.md) |
| EVO-121-A | Target Application Shell | Proposed / paused | P0 | 无死链主导航；依赖 Discover、Settings、Activity 真实路由 | [Item file](active/EVO-121-A-target-application-shell.md) |
| EVO-121-B | Task-first Dashboard | Proposed / paused | P0 | Active work / Review queue / Recent repos / Repo health；不吞错为 empty | [Item file](active/EVO-121-B-task-first-dashboard.md) |
| EVO-121-C | Git-centric Public Entry And Auth Routing | Done / Complete | P0 | Iteration 064 Closed / Complete；完整 URL、安全 redirect、Repo 分流/失败重试、403、join/verification continuity 与 truthful landing 已闭合 | [Item file](active/EVO-121-C-public-entry-and-auth-routing.md)<br>[Iteration 064](../iterations/ITERATION-064.md)<br>[ADR-0008](../decisions/ADR-0008-repo-centric-interaction-architecture.md) |
| EVO-121-D | Settings Information Architecture | Done / Complete | P1 | Iteration 065 Closed / Complete；`/settings/*`、角色 deep-link、user menu、error/retry 与响应式证据闭合 | [Item file](active/EVO-121-D-settings-information-architecture.md)<br>[Iteration 065](../iterations/ITERATION-065.md)<br>[Permissions](../reference/PERMISSIONS.md)<br>[API Key Authorization](../reference/API-KEY-AUTHORIZATION.md) |
| EVO-121-E | Auditable Activity Timeline | Proposed / paused | P0 | Durable event read model 关联 Session/Policy/Commit/Repo | [Item file](active/EVO-121-E-auditable-activity-timeline.md) |
| EVO-121-F | Retire Legacy Registry UI | Proposed / paused | P1 | Discover/Resources 可用后直接删除旧 CRUD UI；不建设兼容体验 | [Item file](active/EVO-121-F-retire-legacy-registry-ui.md) |
| EVO-112-B | Repo Detail Read-only | Done / Complete | P0 | Iteration 061 Closed / Complete；四子路由、Repo Context/CRUD、桌面/移动浏览器验收与删除闭环完成 | [Item file](active/EVO-112-B-repo-detail-read-only.md)<br>[Iteration 061](../iterations/ITERATION-061.md) |
| EVO-112-C | Repo Commit Evidence Detail | Done / Complete | P0 | Iteration 063 Closed / Complete；tenant-scoped API、list -> detail、404、桌面/390px 与全量门禁闭合 | [Item file](active/EVO-112-C-repo-commit-evidence-detail.md)<br>[Iteration 063](../iterations/ITERATION-063.md)<br>[ADR-0008](../decisions/ADR-0008-repo-centric-interaction-architecture.md) |
| EVO-104 | Vibe Coding Web UI | Proposed | P0 | 依赖 EVO-103/105/106/112 与 EVO-118-F/G/H 直接边界；Repo 内 conversation-first Workspace | [Item file](active/EVO-104-vibe-coding-web-ui.md)<br>[Design Decisions](../design/vibe-coding-ui-decisions.md)<br>[Interaction Architecture](../design/PRODUCT-INTERACTION-ARCHITECTURE.md) |
| EVO-105 | Commit API + Promote API | Proposed | P0 | 依赖 EVO-118-B/F/H | [Item file](active/EVO-105-commit-and-promote-api.md) |
| EVO-106 | Agent Session API + Scoped Token | Proposed | P0 | 依赖 EVO-118-B/F/H | [Item file](active/EVO-106-agent-session-and-scoped-token.md) |
| EVO-107 | Webhook Out | Proposed | P1 | 依赖 EVO-105/106 与 EVO-118-C/H；出站调用必须复用统一 Egress Policy | [Item file](active/EVO-107-webhook-out.md) |
| EVO-108 | Skill / CLI / MCP Indexer | Proposed | P0 | 依赖稳定 Push/Commit Event 与 EVO-118-H | [Item file](active/EVO-108-skill-cli-mcp-indexer.md) |
| EVO-109 | Discovery API + Pages 式发现 UI | Proposed | P1 | 依赖 EVO-108；结果必须保留 Repo/Ref/Path/Commit provenance | [Item file](active/EVO-109-discovery-api-and-pages-ui.md)<br>[ADR-0008](../decisions/ADR-0008-repo-centric-interaction-architecture.md) |
| EVO-122 | Epic: Retire Pre-launch Registry Backend | Proposed / paused | P1 | ADR-0009 取消双写；先 Repo-derived execute，再删 legacy runtime 与双数据库表 | [Item file](active/EVO-122-retire-prelaunch-registry-backend.md)<br>[ADR-0009](../decisions/ADR-0009-no-prelaunch-registry-compatibility.md) |
| EVO-122-A | Repo-derived MCP Execution | Proposed / paused | P0 | 以 Repo/Commit/Path 身份替代旧 tool UUID；保留 Typed Capability/Egress/Audit | [Item file](active/EVO-122-A-repo-derived-mcp-execution.md) |
| EVO-122-B | Remove Legacy Registry Runtime | Proposed / paused | P1 | 依赖 EVO-122-A/EVO-111/EVO-121-F；删除旧 CRUD/API/domain/repository | [Item file](active/EVO-122-B-remove-legacy-registry-runtime.md) |
| EVO-122-C | Drop Legacy Registry Tables | Proposed / paused | P1 | 依赖 EVO-122-B；SQLite/PostgreSQL 配对删除 | [Item file](active/EVO-122-C-drop-legacy-registry-tables.md) |
| EVO-123 | 本地启动端口检测只识别监听进程 | Proposed | P2 | `dev.sh` 当前会把到远端同号端口的 established 连接误判为本机占用；不阻塞 Iteration 061 | [Item file](active/EVO-123-dev-port-listener-detection.md)<br>[Local Dev SOP](../sop/LOCAL-DEV.md) |
| EVO-124 | SQLite 内存数据库连接池共享一致性 | Proposed | P1 | `:memory:` + 多连接 pool 可导致运行中缺表/数据不可见；Iteration 062 用临时 SQLite 文件规避，修复独立排期 | [Item file](active/EVO-124-sqlite-memory-pool-isolation.md)<br>[Local Dev SOP](../sop/LOCAL-DEV.md) |
| EVO-125 | Git Smart HTTP E2E 偶发阻塞 | Proposed | P1 | Iteration 067 全量 Gate 中真实 `git clone` 两次出现 224s/30425s 长等待；独立修复测试运行时与子进程超时，不混入 H-B | [Item file](active/EVO-125-git-smart-http-e2e-hang.md)<br>[Testing SOP](../sop/TESTING.md) |
| EVO-111 | 废弃 Skill 沙箱执行 | Proposed | P0 | 产品收尾；不早于 EVO-108/110 稳定 | [Item file](active/EVO-111-deprecate-sandbox-runtime.md)<br>[ADR-0005](../decisions/ADR-0005-deprecate-sandbox-runtime.md) |
| EVO-012 | 租户设置保存 | Proposed | P2 | Phase F 独立候选 | [Item file](active/EVO-012-租户设置保存.md) |
| EVO-013 | Audit log detail 接口 | Proposed | P2 | Phase F 独立候选 | [Item file](active/EVO-013-audit-log-detail-接口.md) |
| EVO-014 | Stripe webhook 恢复 | Proposed | P2 | Phase F 独立候选 | [Item file](active/EVO-014-stripe-webhook-恢复.md) |
| EVO-044 | 前端 CLI 命名简化 | Proposed | P1 | 普通产品候选，不抢占 S1 | [Item file](active/EVO-044-前端-cli-命名简化.md) |

## Blocked Items

| ID | Title | Status | Priority | Decision Context | Required Reads |
| --- | --- | --- | --- | --- | --- |
| EVO-114 | Repo create 原子化硬化 | Superseded | P2 | 2026-07-30 扩展为 EVO-118-F，后者覆盖 create/delete/seed/reconcile 全生命周期 | [Original item](active/EVO-114-atomic-repo-create-hardening.md)<br>[Replacement](active/EVO-118-F-repo-lifecycle-consistency.md) |

## Archived Index

| ID | Title | Status | Priority | Decision Context | Archive |
| --- | --- | --- | --- | --- | --- |
| EVO-110 | 旧表双写适配 | Dropped | P1 | ADR-0009：未上线，不实施双写、回填或旧 API 兼容；由 EVO-122 直接收敛 | [2026-Q3](archive/2026-Q3/EVO-110-old-table-dual-write.md) |
| EVO-001 | 前后端 API 对齐 | Done | P0 | [实施路线图 Phase A](../roadmap/IMPLEMENTATION-ROADMAP.md#phase-a--现状校准与-api-对齐done); 修正 method、字段和不存在的前端 API 调用；snippet 范围已转入 EVO-017 | [2026-Q2](archive/2026-Q2/EVO-001-前后端-api-对齐.md) |
| EVO-002 | 前端迁移到 React + Vite + Bun | Done | P0 | [实施路线图 Phase B](../roadmap/IMPLEMENTATION-ROADMAP.md#phase-b--前端迁移到-react--vite--bunp0--高优先级); EVO-021 至 EVO-025 全部完成 | [2026-Q2](archive/2026-Q2/EVO-002-前端迁移到-react-+-vite-+-bun.md) |
| EVO-003 | 忘记密码与重置密码闭环 | Done | P0 | Iteration 005; handler 实现 + 前端 API 接入，Playwright 验证通过 | [2026-Q2](archive/2026-Q2/EVO-003-忘记密码与重置密码闭环.md) |
| EVO-004 | 邀请接受 / Join 流程 | Done | P0 | Iteration 005; handler 实现，cargo test 通过 | [2026-Q2](archive/2026-Q2/EVO-004-邀请接受-join-流程.md) |
| EVO-005 | MCP 工具真实执行 | Done | P0 | 需求 F1.1.3 / Iteration 006; HTTP handler type 真实执行，Function 类型暂不支持 | [2026-Q2](archive/2026-Q2/EVO-005-mcp-工具真实执行.md) |
| EVO-006 | Skill 更新接口 | Done | P1 | API 501 / Iteration 010 | [2026-Q2](archive/2026-Q2/EVO-006-skill-更新接口.md) |
| EVO-007 | Snippet 更新接口 | Deferred | P1 | 被 EVO-017 替代 | [2026-Q2](archive/2026-Q2/EVO-007-snippet-更新接口.md) |
| EVO-008 | Snippet reference 格式增强 | Deferred | P1 | 被 EVO-017 替代 | [2026-Q2](archive/2026-Q2/EVO-008-snippet-reference-格式增强.md) |
| EVO-009 | SKILL.md 与 CLI interface parser | Done | P1 | Iteration 010 | [2026-Q2](archive/2026-Q2/EVO-009-skillmd-与-cli-interface-frontmatter-parser.md) |
| EVO-010 | 租户 Members 页面接真实 API | Done | P1 | Iteration 011 | [2026-Q2](archive/2026-Q2/EVO-010-租户-members-页面接真实-api.md) |
| EVO-011 | API Key 页面接真实 API | Done | P1 | Iteration 011 | [2026-Q2](archive/2026-Q2/EVO-011-api-key-页面接真实-api.md) |
| EVO-015 | Rust CLI 子项目 | Deferred | P3 | API 稳定后启动 | [2026-Q2](archive/2026-Q2/EVO-015-rust-cli-子项目.md) |
| EVO-016 | 前端嵌入后端发布物 | Done | P3 | EVO-016-B 完成 | [2026-Q2](archive/2026-Q2/EVO-016-前端嵌入后端发布物.md) |
| EVO-016-A | Embedded Frontend refinement | Deferred | P2 | 被 Iteration 031 覆盖 | [2026-Q2](archive/2026-Q2/EVO-016-A-embedded-frontend-交付形态-refinement.md) |
| EVO-016-B | 前端静态服务迁移到 rust-embed-for-web | Done | P0 | Iteration 031 | [2026-Q2](archive/2026-Q2/EVO-016-B-前端静态服务迁移到-rust-embed-for-web.md) |
| EVO-017 | Snippet 迁移为 CLI 友好接口 | Done | P0 | ADR-0002 | [2026-Q2](archive/2026-Q2/EVO-017-snippet-迁移为-cli-友好接口.md) |
| EVO-018 | 邮箱验证发送与确认闭环 | Done | P1 | Iteration 009/021 | [2026-Q2](archive/2026-Q2/EVO-018-邮箱验证发送与确认闭环.md) |
| EVO-021 | 前端路由适配层 | Done | P0 | Iteration 003 | [2026-Q2](archive/2026-Q2/EVO-021-前端路由适配层.md) |
| EVO-022 | Vite + Bun 构建骨架 | Done | P0 | Iteration 004 | [2026-Q2](archive/2026-Q2/EVO-022-vite-+-bun-构建骨架.md) |
| EVO-023 | 前端运行时配置迁移 | Done | P0 | Iteration 004 | [2026-Q2](archive/2026-Q2/EVO-023-前端运行时配置迁移.md) |
| EVO-024 | Docker / Nginx 切换到静态 SPA | Done | P0 | 过渡形态，后续由 Embedded Frontend 取代 | [2026-Q2](archive/2026-Q2/EVO-024-docker-nginx-切换到静态-spa.md) |
| EVO-025 | 移除 Next.js 依赖和遗留入口 | Done | P0 | Iteration 004 | [2026-Q2](archive/2026-Q2/EVO-025-移除-nextjs-依赖和遗留入口.md) |
| EVO-026 | 前端 Snippets 入口迁移 | Done | P1 | Iteration 017 | [2026-Q2](archive/2026-Q2/EVO-026-前端-snippets-入口迁移为-cli-友好接口.md) |
| EVO-030 | GitHub CI/CD 重建 | Done | P2 | Tag-only CI；PR/Main trigger 残余归 EVO-118-G | [2026-Q2](archive/2026-Q2/EVO-030-github-cicd-重建.md) |
| EVO-031 | Iteration 004/005 质量修复 | Done | P0 | 2026-05-17 | [2026-Q2](archive/2026-Q2/EVO-031-iteration-004005-质量修复与流程防呆.md) |
| EVO-032 | MCP 执行质量修复 | Done | P0 | 2026-05-25；新授权/SSRF残余归 EVO-118-B/C | [2026-Q2](archive/2026-Q2/EVO-032-iteration-006-mcp-执行质量修复与流程防呆.md) |
| EVO-033 | Rustfmt 基线 | Done | P2 | Iteration 028 | [2026-Q2](archive/2026-Q2/EVO-033-rustfmt-全量格式基线与-stable-配置清理.md) |
| EVO-034 | Epic 与子需求拆分治理 | Done | P1 | Iteration 008 | [2026-Q2](archive/2026-Q2/EVO-034-epic-与子需求拆分治理规则.md) |
| EVO-035 | 治理 skill manifest | Done | P2 | Iteration 023 | [2026-Q2](archive/2026-Q2/EVO-035-治理-skill-manifest-接入与一致性审计.md) |
| EVO-036 | 计划基线保护 | Done | P1 | Iteration 013 | [2026-Q2](archive/2026-Q2/EVO-036-已发布迭代计划基线保护与改线防呆.md) |
| EVO-037 | 弱模型闭环防呆 | Done | P1 | Iteration 014 | [2026-Q2](archive/2026-Q2/EVO-037-治理-skill-弱模型闭环执行防呆.md) |
| EVO-038 | 任务闭环 SOP | Done | P1 | Iteration 015 | [2026-Q2](archive/2026-Q2/EVO-038-本项目实施任务闭环-sop-与完成声明门禁.md) |
| EVO-039 | Iteration 库存盘点规则 | Done | P1 | Iteration 016 | [2026-Q2](archive/2026-Q2/EVO-039-迭代启动前库存盘点与既有计划优先规则.md) |
| EVO-040 | 完成声明与 Reference 修复 | Done | P1 | Iteration 021 | [2026-Q2](archive/2026-Q2/EVO-040-已实现接口完成声明与参考文档状态修复.md) |
| EVO-041 | 敏捷/BDD 适配 | Done | P1 | Iteration 022 | [2026-Q2](archive/2026-Q2/EVO-041-敏捷实践与-bdd-验收格式适配规则.md) |
| EVO-042 | 编号保留位 | Dropped | — | 历史缺号处置 | [2026-Q2](archive/2026-Q2/EVO-042-编号保留位（缺号处置）.md) |
| EVO-043 | 后端依赖审计与迁移 | Done | P1 | Iteration 032 | [2026-Q2](archive/2026-Q2/EVO-043-后端依赖全量版本审计与迁移.md) |
| EVO-045-A | ExecutionProvider + Docker 池化 | Done | P0 | Iteration 041；legacy | [2026-Q2](archive/2026-Q2/EVO-045-A-executionprovider-统一-trait-+-docker-容器池化.md) |
| EVO-048 | Serverless 执行架构 Spike | Done | P0 | Iteration 033 | [2026-Q2](archive/2026-Q2/EVO-048-serverless-执行架构设计-spike.md) |
| EVO-049-A | Skill/CLI 数据模型基线 | Done | P0 | Iteration 034；legacy compatibility | [2026-Q2](archive/2026-Q2/EVO-049-A-skillcli-规范兼容数据模型基线.md) |
| EVO-051 | 后端死代码清理 | Done | P2 | Iteration 039 | [2026-Q2](archive/2026-Q2/EVO-051-误导性注释、命名与后端死代码清理.md) |
| EVO-052 | MySQL 快速失败 | Done | P2 | Iteration 038 | [2026-Q2](archive/2026-Q2/EVO-052-mysql-半接线收敛与快速失败.md) |
| EVO-054 | Backlog 状态漂移修复 | Done | P1 | Iteration 035 | [2026-Q2](archive/2026-Q2/EVO-054-backlog-状态漂移与编号一致性修复.md) |
| EVO-055 | API 契约漂移修复 | Done | P1 | Iteration 035 | [2026-Q2](archive/2026-Q2/EVO-055-前后端-api-契约漂移修复.md) |
| EVO-056 | Sandbox 假成功修复 | Done | P2 | Iteration 038 | [2026-Q2](archive/2026-Q2/EVO-056-沙箱降级静默成功修复.md) |
| EVO-058 | 前端死代码清理 | Done | P2 | Iteration 040 | [2026-Q2](archive/2026-Q2/EVO-058-前端死代码与类型卫生清理.md) |
| EVO-059 | Backend clippy 修复 | Done | P2 | Iteration 029 | [2026-Q2](archive/2026-Q2/EVO-059-backend-clippy-历史-lint-升级修复.md) |
| EVO-060 | dev.sh Embedded Frontend 清理 | Done | P2 | 2026-06-03 | [2026-Q2](archive/2026-Q2/EVO-060-devsh-embeddedfrontendzip-死代码-+-关联-proposal-状态清理.md) |
| EVO-061~076 | 依赖升级子项 | Done | P1~P3 | EVO-086 统一收口 | [2026-Q2 Index](archive/2026-Q2/INDEX.md) |
| EVO-077 | Governance Board | Done | P1 | Iteration 036 | [2026-Q2](archive/2026-Q2/EVO-077-governance-board-派生运营视图.md) |
| EVO-078 | 治理漂移修复 | Done | P1 | Iteration 037 | [2026-Q2](archive/2026-Q2/EVO-078-最近开发任务治理漂移修复.md) |
| EVO-079 | Sandbox 默认关闭 | Done | P1 | 2026-06-05 | [2026-Q2](archive/2026-Q2/EVO-079-sandbox-默认启用导致本地启动依赖-docker-回归修复.md) |
| EVO-082 | Evolution Feedback SOP | Done | P1 | 2026-06-05 | [2026-Q2](archive/2026-Q2/EVO-082-evolution-feedback-sop-缺失导致治理-validator-失败修复.md) |
| EVO-083 | Agent redirect 入口 | Done | P1 | 2026-06-05 | [2026-Q2](archive/2026-Q2/EVO-083-skill-107-agent-redirect-入口纠偏.md) |
| EVO-084 | Backlog compaction | Done | P1 | 2026-06-05 | [2026-Q2](archive/2026-Q2/EVO-084-backlog-compaction-标准结构迁移.md) |
| EVO-085 | Archive index compaction | Done | P1 | 2026-06-05 | [2026-Q2](archive/2026-Q2/EVO-085-archive-index-二次压缩与-item-file-拆分纠偏.md) |
| EVO-086 | 全量依赖 latest 迁移 | Done | P1 | 2026-06-06 | [2026-Q2](archive/2026-Q2/EVO-086-全量依赖-latest-迁移.md) |
| EVO-019 | Skill registry 服务化 | Superseded | P2 | 被 EVO-100 + EVO-108 覆盖 | [Item file](active/EVO-019-skill-registry-服务化.md) |
| EVO-020 | Storage 能力落地 | Superseded | P2 | Git filesystem 替代 MinIO 主路径 | [Item file](active/EVO-020-storage-能力落地.md) |
| EVO-027 | Skill 多来源创建 | Re-scoped | P1 | 被 EVO-103 + EVO-108 覆盖 | [Item file](active/EVO-027-skill-多来源创建.md) |
| EVO-028 | Skill 版本管理 | Superseded | P1 | Git 原生版本管理 | [Item file](active/EVO-028-skill-版本管理与正确性验证.md) |
| EVO-029 | Skill 发现质量 | Re-scoped | P2 | 归 EVO-108/109 | [Item file](active/EVO-029-skill-专业描述与发现质量提升.md) |
| EVO-045 | CLI Serverless 引擎 | Superseded | P0 | 被 EVO-105 + EVO-108 覆盖 | [Item file](active/EVO-045-cli-命令执行引擎-serverless.md) |
| EVO-046 | Skill 下载/安装 | Superseded | P1 | Repo clone + Vibe UI | [Item file](active/EVO-046-skill-可下载制品与-agent-一键安装.md) |
| EVO-047 | MCP Serverless 执行 | Superseded | P1 | 保持 HTTP Proxy；安全硬化由 EVO-118-C 完成 | [Item file](active/EVO-047-mcp-工具-serverless-执行.md) |
| EVO-049 | Skill/CLI 生态兼容 Epic | Dropped | P0 | 剩余由 EVO-108 覆盖 | [Item file](active/EVO-049-skill-cli-生态兼容与行业标准对齐.md) |
| EVO-049-B | Parser 接线与报告 | Superseded | P0 | 归 EVO-108 | [Item file](active/EVO-049-B-skill-cli-parser-接线与校验报告.md) |
| EVO-050 | Skill/CLI 评分体系 | Dropped | P1 | 超出 MVP | [Item file](active/EVO-050-skill-cli-评分与质量体系.md) |

## Reading Rules

1. Read this file first for routing, priority, status and decision context.
2. Before implementation or prioritization, open every path in `Required Reads` for the target row.
3. Read archive files only when `Required Reads` points there, an item says it supersedes or depends on archive history, or the user asks for historical rationale.
4. Do not mark an item Ready when a governing ADR, spec, dependency or archive record is only implicit. Add it to `Required Reads` and the item file first.
5. Do not store long acceptance criteria or execution logs in this file; put them in item files or archive records.
