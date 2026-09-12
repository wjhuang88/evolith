# Operating Board

> Derived operating view only. Owner docs define status, scope, acceptance criteria, validation evidence and lifecycle state. Update owner docs first, then reflect the current operating state here.
>
> **2026-09-10 replan**：当前整体状态以 [Project Status Baseline](reference/PROJECT-STATUS-BASELINE-2026-09-10.md) 为准；当前执行顺序以 [Current Execution Plan 2026-09](roadmap/CURRENT-EXECUTION-PLAN-2026-09.md) 为准。新增 [EVO-126](backlog/active/EVO-126-walgit-git-data-plane-refactor.md) / GIT-DP-01，先完成 WalGit-backed Git Data Plane，再恢复 Agent Write Loop。

## Now

| Item | State | Owner Doc | Gate / Next action |
|------|-------|-----------|--------------------|
| EVO-126 WalGit Git Data Plane | Proposed / accepted direction | [Epic](backlog/active/EVO-126-walgit-git-data-plane-refactor.md) | GIT-DP-01 Open；Epic 不直接进入 Iteration |
| EVO-126-A Dependency/Toolchain Boundary | **Ready / next** | [Item](backlog/active/EVO-126-A-walgit-dependency-toolchain-boundary.md) | 新建 Iteration 后执行 Rust 1.90 + exact WalGit pin + license boundary |
| EVO-118-G-A CI Merge Gates | Review / Partial residual | [Item](backlog/active/EVO-118-G-A-ci-merge-gates.md) | Iteration 056 保留；Branch Protection 远端证据在 release closure 前处理 |

## Planned Sequence

| Order | Item | State | Dependency |
|------:|------|-------|------------|
| 1 | EVO-126-A toolchain/dependency | Ready | none |
| 2 | EVO-126-B service-git v2 engine boundary | Proposed | A |
| 3 | EVO-126-C Smart HTTP read | Proposed | B |
| 4 | EVO-126-E Repo Context parity | Proposed | B |
| 5 | EVO-126-D WAL receive-pack | Proposed | B |
| 6 | EVO-126-F object-store operations/migration | Proposed | B + final parity C/D/E |
| 7 | EVO-126-G bundle-uri | Proposed | C/F |
| 8 | EVO-126-H cutover/legacy Git engine removal | Proposed | C/D/E/F/G |

C/E/D 的具体激活顺序可在 refinement 时按依赖和风险调整；A/B/H 的边界不可跳过。

## Paused / Waiting for GIT-DP-01

| Item | State | Owner Doc | Reason |
|------|-------|-----------|--------|
| EVO-105 Commit/Promote | Proposed / paused for activation | [Item](backlog/active/EVO-105-commit-and-promote-api.md) | 避免在旧 write path 实现后再次重构；GIT-DP-01 后恢复 |
| EVO-106 Agent Session/Scoped Token | Proposed / paused for activation | [Item](backlog/active/EVO-106-agent-session-and-scoped-token.md) | 与最终 GitWrite/Policy boundary 对齐后恢复 |
| EVO-107 Webhook / EVO-104 Vibe Coding | Proposed | [EVO-100](backlog/active/EVO-100-git-centric-platform-foundation.md) | 依赖 Agent write loop |
| EVO-108/109 Index/Discovery | Proposed | [EVO-100](backlog/active/EVO-100-git-centric-platform-foundation.md) | 等稳定新 Push/Commit provenance |
| EVO-121-A/B/E/F Experience | Proposed / paused | [Epic](backlog/active/EVO-121-product-experience-convergence.md) | 按真实 Activity/Discovery/cleanup 依赖恢复 |
| EVO-111 / EVO-122 Legacy Cleanup | Proposed / paused | [EVO-111](backlog/active/EVO-111-deprecate-sandbox-runtime.md) / [EVO-122](backlog/active/EVO-122-retire-prelaunch-registry-backend.md) | 在目标产品路径承接后清理 |
| EVO-118-E Final Production Convergence | Proposed / final gate | [Item](backlog/active/EVO-118-E-production-build-deployment-convergence.md) | GIT-DP-01、REL-01 residual、产品与 cleanup 后关闭 DEPLOY-01 |

## Completed Foundation That Must Be Preserved

| Capability / Gate | State | Evidence owner |
|-------------------|-------|----------------|
| SEC-01 API Key / MCP authorization | Closed | [EVO-118-B](backlog/active/EVO-118-B-api-key-mcp-authorization-hardening.md) |
| SEC-02 Egress / SSRF | Closed | [EVO-118-C](backlog/active/EVO-118-C-http-tool-egress-security.md) |
| DATA-01 filesystem single-instance durability | Closed historical | [EVO-118-D](backlog/active/EVO-118-D-git-storage-durability-and-recovery.md) / Iteration 053 |
| DATA-02 Repo lifecycle | Closed historical | [EVO-118-F](backlog/active/EVO-118-F-repo-lifecycle-consistency.md) / Iteration 055 |
| EVENT-01 Durable Outbox + Push Event | Closed | [EVO-118-H](backlog/active/EVO-118-H-durable-outbox-events.md) / Iterations 066-068 |
| Runtime readiness/rate-limit/fail-closed | G-B/C/D Complete | [EVO-118-G](backlog/active/EVO-118-G-runtime-reliability-gates.md) |
| Repo UI / Detail / Commit Evidence | Done / Complete | [EVO-112](backlog/active/EVO-112-repo-management-ui.md) |
| First-run onboarding | Done / Complete | [EVO-120](backlog/active/EVO-120-first-run-repo-onboarding.md) |
| Public/Auth Entry + Settings IA | Done / Complete | [EVO-121](backlog/active/EVO-121-product-experience-convergence.md) |
| Stable bounded real Git E2E runner | Done / Complete | [EVO-125](backlog/active/EVO-125-git-smart-http-e2e-hang.md) |

这些完成项不会因为 EVO-126 被改写为“未完成”；新 data plane 必须保持其安全/行为语义，并在需要时重新证明 storage-specific evidence。

## Release Gates

| Gate | State | Meaning |
|------|-------|---------|
| SEC-01 | Closed | 不允许 WalGit 接入降低授权边界 |
| SEC-02 | Closed | 后续 Webhook 等仍复用 Egress Policy |
| DATA-01 | Closed historical | 证明旧 filesystem engine 的单实例 durability，不代表 object-store cutover |
| DATA-02 | Closed historical | lifecycle 语义继续有效，reconciler 实现随 storage 迁移 |
| EVENT-01 | Closed | WAL receive path 必须保持 Durable Outbox contract |
| REL-01 | Partial | G-A remote Branch Protection evidence residual |
| **GIT-DP-01** | **Open** | EVO-126 A-H + migration/recovery/protocol/context/bundle/cutover review |
| DEPLOY-01 | Open / final | 最终 clean build/smoke 必须在 WalGit-backed data plane 上执行 |

## Independent Later Candidates

EVO-124 SQLite memory pool、EVO-123 dev port detection、EVO-080 Wasmer/WASI spike、EVO-081 internal docs、EVO-057 CORS、EVO-012/013/014/044 等继续保留在 [Product Backlog](backlog/PRODUCT-BACKLOG.md)，不得静默抢占当前 P0，除非出现安全/数据损坏/基础构建紧急问题。

## Operating Rule

Board 不定义新状态。状态变化先更新 owner item/Iteration/Baseline，再反映到这里。EVO-126 实施必须按 `START-ITERATION.md` 新建 Iteration，不得把本次 product-pivot 写进旧 Planned/Closed Iteration。
