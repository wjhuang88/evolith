# Operating Board

> Derived operating view only. Owner docs define status, scope, acceptance criteria,
> validation evidence and lifecycle state. Update owner docs first, then reflect the current
> operating state here.
>
> **2026-08-01 生产就绪进展**：Git-centric 产品方向保持不变；EVO-118 S1 中 SEC-01、SEC-02 已关闭，剩余 P0 Gate 为 DATA-01 与 DEPLOY-01。事实见 [Production Readiness Baseline](reference/PRODUCTION-READINESS-BASELINE.md)，顺序见 [Production Readiness Plan](roadmap/PRODUCTION-READINESS-PLAN-2026-07.md)。

## Now

| Item | State | Owner Doc | Gate |
|------|-------|-----------|------|
| EVO-118 Production Readiness Epic | In Progress | [Epic](backlog/active/EVO-118-production-readiness-and-security-hardening.md) | A/B/C Done；S1 继续 D → E。 |

## Review

当前没有处于 Review 的激活 Iteration 或 Story。PR #5 在关闭文档 final-head CI 完成前仍保持 Draft，但 EVO-118-C 的运行时安全结论与治理 owner 状态已经关闭。

## Done

| Item | State | Owner Doc | Evidence |
|------|-------|-----------|----------|
| EVO-118-C HTTP Tool Egress Security | Done / Complete | [Item](backlog/active/EVO-118-C-http-tool-egress-security.md) | Navigator accepted runtime security；CI #125 / run `30653767138` 全绿；权限/API 稳定契约已同步；SEC-02 已解除。 |
| Iteration 052 | Closed / Complete | [Iteration](iterations/ITERATION-052.md) | 统一 Egress、总 deadline、管理门禁、审计隐藏、负向安全矩阵与关闭状态完成。 |

## Blocked Or Paused

| Item | State | Owner Doc | Gate |
|------|-------|-----------|------|
| EVO-112 Repo Management UI | Proposed / paused for implementation | [Item](backlog/active/EVO-112-repo-management-ui.md) | EVO-118 S1（D/E）关闭后恢复；当前可 refinement。 |
| EVO-105/106/107/104 Agent Write Loop | Proposed | [EVO-100](backlog/active/EVO-100-git-centric-platform-foundation.md) | 依赖 EVO-118-B/F/H 与 Repo UI 基础；不得绕过 Typed Capability、Policy 和 Durable Event。 |
| EVO-108/109/110 Index/Discovery | Proposed | [EVO-100](backlog/active/EVO-100-git-centric-platform-foundation.md) | 依赖稳定 Push/Commit Event 与 EVO-118-H。 |
| Iterations 018~020、027 | Superseded / Blocked for activation | [Iteration index](iterations/README.md) | 旧 Registry 主线被 Git-centric 方向替代，不 deliberate replan 则不激活。 |
| Iterations 025/026 | Blocked for activation | [Iteration index](iterations/README.md) | Phase F 独立候选，不抢占 EVO-118 S1。 |

## Next

| Item | State | Owner Doc | Gate |
|------|-------|-----------|------|
| EVO-118-D Git Durability and Recovery | Ready | [Item](backlog/active/EVO-118-D-git-storage-durability-and-recovery.md) | DATA-01；当前下一条 P0；持久卷 + DB/Git 恢复演练。 |
| EVO-118-E Production Build Convergence | Ready | [Item](backlog/active/EVO-118-E-production-build-deployment-convergence.md) | DEPLOY-01；clean build + 单一交付 + Smoke Test。 |
| EVO-118-F Repo Lifecycle | Proposed | [Item](backlog/active/EVO-118-F-repo-lifecycle-consistency.md) | 依赖 D/E；DATA-02。 |
| EVO-118-G Runtime Reliability Gates | Proposed | [Item](backlog/active/EVO-118-G-runtime-reliability-gates.md) | 依赖 B/E；REL-01。 |
| EVO-118-H Durable Outbox | Proposed | [Item](backlog/active/EVO-118-H-durable-outbox-events.md) | 依赖 E/G；EVENT-01。 |

## Later

| Item | State | Owner Doc | Gate |
|------|-------|-----------|------|
| Repo-centric Web | Proposed | [EVO-112](backlog/active/EVO-112-repo-management-ui.md) | S1 关闭后恢复。 |
| Agent Integration + Vibe Coding | Proposed | [EVO-105/106/107/104](backlog/active/EVO-100-git-centric-platform-foundation.md) | S2 与 Repo UI 依赖满足。 |
| Indexer + Discovery + Compatibility | Proposed | [EVO-108/109/110](backlog/active/EVO-100-git-centric-platform-foundation.md) | Durable Event + Agent write semantics 稳定。 |
| Sandbox removal | Proposed | [EVO-111](backlog/active/EVO-111-deprecate-sandbox-runtime.md) | Index/compat 迁移稳定后收尾。 |
| SSH / LFS / Cross-repo search / Resource ACL | Proposal later | — | Phase E' 和 EVO-118 关闭后独立评估。 |

## Operating Review

- EVO-118-A/Iteration 050 已由 PR #2 合并并关闭。
- 当前产品方向仍是 Git hosting + Vibe Coding + capability discovery；EVO-118 是稳定化 Gate，不是产品回退。
- EVO-118-B/Iteration 051 已由 PR #3 合并并关闭；merge commit `6de7845e1231efc04f94f16cb9ab0a410f6ad2d9`，最终 CI run `30567361095` 全绿，SEC-01 已解除。
- EVO-118-C/Iteration 052 已关闭：Navigator 明确接受运行时安全实现，CI #125 / run `30653767138` 全绿，稳定权限/API 契约与派生治理状态已同步，SEC-02 已解除。
- 当前启动顺序：EVO-118-D → E → F/G/H → EVO-112 → EVO-105/106/107/104 → EVO-108/109/110 → EVO-111。
- 安全、数据损坏或生产构建问题允许显式 P0 插队；普通 UI、视觉、计费或内部文档不得静默绕过 S1。
- Board 只反映 owner docs；Gate 关闭必须由 Story 验收和实际验证证明。
