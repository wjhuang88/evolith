# Product Backlog

> Compact routing and prioritization surface. Executable context lives in `docs/backlog/active/`; completed/deferred/dropped history stays in owner item/archive records.
> Status and DoR rules: [Requirement Intake](../sop/REQUIREMENT-INTAKE.md). Completion rules: [Iteration Workflow](../sop/ITERATION-WORKFLOW.md).

> **2026-09-10 product/architecture replan**：Evolith 接受 [ADR-0011](../decisions/ADR-0011-walgit-backed-git-data-plane.md)，新增 P0 [EVO-126](active/EVO-126-walgit-git-data-plane-refactor.md) 与 GIT-DP-01。当前 runtime 仍是 filesystem-backed Git；Agent Write Loop 暂停激活，待 EVO-126-H cutover。当前整体状态见 [Project Status Baseline](../reference/PROJECT-STATUS-BASELINE-2026-09-10.md)，当前顺序只以 [2026-09 Execution Plan](../roadmap/CURRENT-EXECUTION-PLAN-2026-09.md) 为 owner。

## Current Priorities

| ID | Title | Status | Priority | Decision Context | Required Reads |
| --- | --- | --- | --- | --- | --- |
| EVO-126 | WalGit-backed Git Data Plane Refactor | Proposed / accepted direction | **P0** | 新增 GIT-DP-01；object store/WAL 为长期 Git data plane，Evolith 保持 control-plane ownership | [Epic](active/EVO-126-walgit-git-data-plane-refactor.md)<br>[ADR-0011](../decisions/ADR-0011-walgit-backed-git-data-plane.md)<br>[Design](../design/WALGIT-GIT-DATA-PLANE-REFACTOR.md)<br>[Current plan](../roadmap/CURRENT-EXECUTION-PLAN-2026-09.md) |
| EVO-126-A | WalGit dependency and toolchain boundary | **Ready** | **P0** | 下一个可激活 Story；Rust 1.90 + exact upstream pin + MIT/supply-chain boundary；不改 runtime Git path | [Item](active/EVO-126-A-walgit-dependency-toolchain-boundary.md)<br>[Epic](active/EVO-126-walgit-git-data-plane-refactor.md)<br>[ADR-0011](../decisions/ADR-0011-walgit-backed-git-data-plane.md) |
| EVO-118-G-A | PR/Main 自动质量门禁 | Review / Partial | P1 | Iteration 056 保留；Branch Protection 远端证据 residual，release 前必须闭合，不阻塞新 Iteration 的 EVO-126-A | [Item](active/EVO-118-G-A-ci-merge-gates.md)<br>[Iteration 056](../iterations/ITERATION-056.md) |
| EVO-105 | Commit API + Promote API | Proposed / paused for activation | P0 | 原产品下一步；为避免旧/new Git write path 双重重构，新增依赖 GIT-DP-01 / EVO-126-H | [Item](active/EVO-105-commit-and-promote-api.md)<br>[ADR-0007](../decisions/ADR-0007-agent-write-and-production-delivery-boundaries.md)<br>[ADR-0011](../decisions/ADR-0011-walgit-backed-git-data-plane.md) |
| EVO-106 | Agent Session API + Scoped Token | Proposed / paused for activation | P0 | 与 EVO-105 共同恢复；不得在旧 generic receive-pack 上固定最终 Agent write architecture | [Item](active/EVO-106-agent-session-and-scoped-token.md)<br>[Security Review](../sop/SECURITY-REVIEW.md)<br>[ADR-0011](../decisions/ADR-0011-walgit-backed-git-data-plane.md) |
| EVO-118-E | 最终 Embedded Frontend 生产构建与部署收敛 | Proposed / final release gate | P0 | 产品、legacy cleanup、REL-01 residual 与 GIT-DP-01 完成后关闭 DEPLOY-01 | [Item](active/EVO-118-E-production-build-deployment-convergence.md)<br>[Release SOP](../sop/RELEASE.md)<br>[Project baseline](../reference/PROJECT-STATUS-BASELINE-2026-09-10.md) |

## EVO-126 Execution Family

| ID | Result | Status | Dependency | GitHub |
| --- | --- | --- | --- | --- |
| [EVO-126-A](active/EVO-126-A-walgit-dependency-toolchain-boundary.md) | Rust 1.90 + exact WalGit pin + license boundary | **Ready** | none | [#13](https://github.com/wjhuang88/evolith/issues/13) |
| [EVO-126-B](active/EVO-126-B-service-git-v2-engine-boundary.md) | service-git v2 + Store/Registry lifecycle | Proposed | A | [#14](https://github.com/wjhuang88/evolith/issues/14) |
| [EVO-126-C](active/EVO-126-C-smart-http-read-path.md) | Smart HTTP read path v0/v2 | Proposed | B | [#15](https://github.com/wjhuang88/evolith/issues/15) |
| [EVO-126-D](active/EVO-126-D-wal-receive-pack-write-path.md) | receive-pack -> WAL publish + durable Push event | Proposed | B | [#16](https://github.com/wjhuang88/evolith/issues/16) |
| [EVO-126-E](active/EVO-126-E-repo-context-walgit.md) | Repo Context on RepoHandle/ObjectAccess | Proposed | B | [#17](https://github.com/wjhuang88/evolith/issues/17) |
| [EVO-126-F](active/EVO-126-F-object-store-operations-migration.md) | object-store config/readiness/migration/recovery | Proposed | B; acceptance with C/D/E | [#18](https://github.com/wjhuang88/evolith/issues/18) |
| [EVO-126-G](active/EVO-126-G-bundle-uri-agent-clone.md) | secure bundle-uri Agent clone path | Proposed | C/F | [#19](https://github.com/wjhuang88/evolith/issues/19) |
| [EVO-126-H](active/EVO-126-H-cutover-retire-legacy-git-engine.md) | default cutover + legacy Git engine retirement | Proposed | C/D/E/F/G | [#20](https://github.com/wjhuang88/evolith/issues/20) |

Parent tracking: [EVO-126 GitHub #12](https://github.com/wjhuang88/evolith/issues/12). Epic 不直接进入 Iteration；一次只选择满足 DoR 的子 Story。

## Active Product / Platform Items

| ID | Title | Status | Priority | Decision Context | Required Reads |
| --- | --- | --- | --- | --- | --- |
| EVO-118 | Production Readiness and Security Hardening | In Progress | P0 | SEC-01/02、DATA-01/02、EVENT-01 已关闭；G-A residual；E 为最终 gate | [Item](active/EVO-118-production-readiness-and-security-hardening.md)<br>[Production baseline](../reference/PRODUCTION-READINESS-BASELINE.md) |
| EVO-118-G | Runtime Reliability Gates | In Progress / residual | P1 | G-B/C/D Done；G-A Review/Partial | [Item](active/EVO-118-G-runtime-reliability-gates.md) |
| EVO-118-H | Durable Outbox and Events | Done / Complete | P1 | H-A/B/C Done；新 WAL write path 必须保持 EVENT-01 | [Item](active/EVO-118-H-durable-outbox-events.md) |
| EVO-100 | Git-Centric Platform Foundation | In Progress | P0 | 产品方向继续有效；Git data-plane 实现层由 ADR-0011 更新 | [Item](active/EVO-100-git-centric-platform-foundation.md)<br>[Project baseline](../reference/PROJECT-STATUS-BASELINE-2026-09-10.md) |
| EVO-112 | Repo Management UI | Done / Complete | P0 | A/B/C Done；Repo list/create/detail/Commit Evidence 基础已形成 | [Item](active/EVO-112-repo-management-ui.md) |
| EVO-120 | First-run Repo Onboarding | Done / Complete | P0 | seeded Repo -> Overview 已闭合 | [Item](active/EVO-120-first-run-repo-onboarding.md) |
| EVO-121 | Product Experience Convergence | In Progress | P0 | C/D Done；A/B/E/F 等待后续依赖 | [Item](active/EVO-121-product-experience-convergence.md)<br>[Interaction Architecture](../design/PRODUCT-INTERACTION-ARCHITECTURE.md) |
| EVO-104 | Vibe Coding Web UI | Proposed / paused for activation | P0 | 在 GIT-DP-01 + EVO-105/106 后恢复 | [Item](active/EVO-104-vibe-coding-web-ui.md) |
| EVO-105 | Commit API + Promote API | Proposed / paused for activation | P0 | GIT-DP-01 后恢复；Policy/Outbox 边界保持 | [Item](active/EVO-105-commit-and-promote-api.md) |
| EVO-106 | Agent Session + Scoped Token | Proposed / paused for activation | P0 | GIT-DP-01 后与 Agent write foundation 恢复 | [Item](active/EVO-106-agent-session-and-scoped-token.md) |
| EVO-107 | Webhook Out | Proposed | P1 | 依赖 EVO-105/106，复用 Egress/Outbox | [Item](active/EVO-107-webhook-out.md) |
| EVO-108 | Skill / CLI / MCP Indexer | Proposed | P0 | 等稳定的新 Push/Commit provenance | [Item](active/EVO-108-skill-cli-mcp-indexer.md) |
| EVO-109 | Discovery API + UI | Proposed | P1 | 依赖 EVO-108 | [Item](active/EVO-109-discovery-api-and-pages-ui.md) |
| EVO-111 | Deprecate Skill Sandbox Runtime | Proposed | P0 | 产品/legacy cleanup 阶段 | [Item](active/EVO-111-deprecate-sandbox-runtime.md) |
| EVO-122 | Retire Pre-launch Registry Backend | Proposed / paused | P1 | Repo-derived execute 后删除 legacy runtime/table，不做双写 | [Item](active/EVO-122-retire-prelaunch-registry-backend.md)<br>[ADR-0009](../decisions/ADR-0009-no-prelaunch-registry-compatibility.md) |

## Independent Candidates / Debt

| ID | Title | Status | Priority | Decision Context | Required Reads |
| --- | --- | --- | --- | --- | --- |
| EVO-124 | SQLite 内存数据库连接池共享一致性 | Proposed | P1 | 独立运行时债务；不与 EVO-126 混改 | [Item](active/EVO-124-sqlite-memory-pool-isolation.md) |
| EVO-123 | 本地启动端口检测只识别监听进程 | Proposed | P2 | 独立 dev tooling 债务 | [Item](active/EVO-123-dev-port-listener-detection.md) |
| EVO-080 | Wasmer/WASI runtime spike | Ready / later | P1 | 仅在 Sandbox 删除后仍有明确 runtime 需求时评估 | [Item](active/EVO-080-spike-验证-wasmer-wasi-替代-docker-sandbox-可行性.md) |
| EVO-081 | 内部 Markdown 文档页面 | Ready / later | P1 | 独立产品候选，不抢占 Git Data Plane P0 | [Item](active/EVO-081-内部文档页面基于独立-markdown-目录渲染.md) |
| EVO-057 | 生产 CORS Origin 可配置化 | Ready / later | P2 | release hardening 候选 | [Item](active/EVO-057-生产-cors-origin-可配置化.md) |
| EVO-012 | 租户设置保存 | Proposed / later | P2 | Phase F 独立候选 | [Item](active/EVO-012-租户设置保存.md) |
| EVO-013 | Audit log detail 接口 | Proposed / later | P2 | Phase F 独立候选 | [Item](active/EVO-013-audit-log-detail-接口.md) |
| EVO-014 | Stripe webhook 恢复 | Proposed / later | P2 | Phase F 独立候选 | [Item](active/EVO-014-stripe-webhook-恢复.md) |
| EVO-044 | 前端 CLI 命名简化 | Proposed / later | P1 | 普通产品候选 | [Item](active/EVO-044-前端-cli-命名简化.md) |

## Superseded / Historical Routing

| ID / Group | State | Current owner / history |
| --- | --- | --- |
| EVO-114 Repo create 原子化硬化 | Superseded | [EVO-118-F](active/EVO-118-F-repo-lifecycle-consistency.md) 已覆盖完整 lifecycle |
| EVO-110 旧表双写适配 | Dropped | [ADR-0009](../decisions/ADR-0009-no-prelaunch-registry-compatibility.md)；[archive](archive/2026-Q3/EVO-110-old-table-dual-write.md) |
| EVO-019/020/027/028/029/045/046/047/049/049-B/050 | Superseded / Re-scoped / Dropped | 历史 owner item 保留；新工作分别归 EVO-100/103/105/108/109/122，不从旧 item 启动 |
| EVO-001~086 completed/deferred historical work | Historical | [2026-Q2 Archive Index](archive/2026-Q2/INDEX.md) 与各 owner item/Iteration 保留证据 |

## Reading Rules

1. 先读本文件确定 priority/status，再读目标行全部 `Required Reads`。
2. Git data-plane 工作必须同时读 Project Status Baseline、ADR-0011、Refactor Design 和 Current Execution Plan。
3. `Ready` 只代表 Story 满足 DoR；开始 Iteration 前仍需按 `START-ITERATION.md` 盘点现有非终态库存。
4. Current implementation 与 Accepted target 不得混写：EVO-126-H 前 filesystem/subprocess 仍是 runtime fact。
5. 历史 DATA-01/DATA-02/EVENT-01 完成证据继续有效；GIT-DP-01 是架构迁移新增 Gate，不追溯否定历史结果。
6. 不在本 compact backlog 保存长验收/执行日志；进入各 active item、Iteration 或 Review evidence。
