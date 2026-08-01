# Operating Board

> Derived operating view only. Owner docs define status, scope, acceptance criteria,
> validation evidence and lifecycle state. Update owner docs first, then reflect the current
> operating state here.
>
> **2026-08-02 生产就绪进展**：Git-centric 产品方向保持不变；EVO-118 S1 中 SEC-01、SEC-02 已关闭，[Draft PR #7](https://github.com/wjhuang88/evolith/pull/7) 正在以 Iteration 053 推进 DATA-01，DEPLOY-01 仍开放。事实见 [Production Readiness Baseline](reference/PRODUCTION-READINESS-BASELINE.md)，顺序见 [Production Readiness Plan](roadmap/PRODUCTION-READINESS-PLAN-2026-07.md)。

## Now

| Item | State | Owner Doc | Gate |
|------|-------|-----------|------|
| EVO-118-D Git Durability and Recovery | In Progress in Draft PR #7 | [Draft PR #7](https://github.com/wjhuang88/evolith/pull/7) | DATA-01；Iteration 053 Active；本恢复分支不修改其实现。 |
| EVO-118 Production Readiness Epic | In Progress | [Epic](backlog/active/EVO-118-production-readiness-and-security-hardening.md) | A/B/C Done；D 在 PR #7 实施；E 为下一 Gate。 |

## Review

当前没有处于 Review 的 Story 或 Iteration。EVO-118-D / Iteration 053 在 Draft PR #7 中处于实现阶段，尚未进入 Review。

## Done

| Item | State | Owner Doc | Evidence |
|------|-------|-----------|----------|
| EVO-118-C HTTP Tool Egress Security | Done / Complete / Merged | [Item](backlog/active/EVO-118-C-http-tool-egress-security.md) | PR #5 于 2026-08-02 合并，merge commit `936ed3b26a62840ddd94cf10e5075fd19e0a1c5c`；final-head CI #137 / run `30682419168` 全绿；SEC-02 已解除。 |
| Iteration 052 | Closed / Complete | [Iteration](iterations/ITERATION-052.md) | 统一 Egress、总 deadline、管理门禁、审计隐藏、负向安全矩阵、稳定契约和合并证据完成。 |
| EVO-112-A Repo UI Shell | Done / Complete | [Item](backlog/active/EVO-112-A-repo-ui-shell.md) | 本地历史成果在最新主线上恢复并迁移为 Iteration 054；创建成功返回 `/repos`，不再进入未实现详情页。 |
| Iteration 054 | Closed / Complete | [Iteration](iterations/ITERATION-054.md) | 仓库列表/创建、repo-centric 导航与 Dashboard、i18n 和历史截图证据；不关闭 EVO-118 S1。 |

## Blocked Or Paused

| Item | State | Owner Doc | Gate |
|------|-------|-----------|------|
| EVO-112 Repo Management UI | In Progress / paused for implementation | [Item](backlog/active/EVO-112-repo-management-ui.md) | EVO-112-A Done；EVO-112-B 在 EVO-118 S1（D/E）关闭后恢复。 |
| EVO-105/106/107/104 Agent Write Loop | Proposed | [EVO-100](backlog/active/EVO-100-git-centric-platform-foundation.md) | 依赖 EVO-118-B/F/H 与 Repo UI 基础；不得绕过 Typed Capability、Policy 和 Durable Event。 |
| EVO-108/109/110 Index/Discovery | Proposed | [EVO-100](backlog/active/EVO-100-git-centric-platform-foundation.md) | 依赖稳定 Push/Commit Event 与 EVO-118-H。 |
| Iterations 018~020、027 | Superseded / Blocked for activation | [Iteration index](iterations/README.md) | 旧 Registry 主线被 Git-centric 方向替代，不 deliberate replan 则不激活。 |
| Iterations 025/026 | Blocked for activation | [Iteration index](iterations/README.md) | Phase F 独立候选，不抢占 EVO-118 S1。 |

## Next

| Item | State | Owner Doc | Gate |
|------|-------|-----------|------|
| EVO-118-E Production Build Convergence | Ready | [Item](backlog/active/EVO-118-E-production-build-deployment-convergence.md) | DEPLOY-01；D 收口后按 WIP 启动，clean build + 单一交付 + Smoke Test。 |
| EVO-118-F Repo Lifecycle | Proposed | [Item](backlog/active/EVO-118-F-repo-lifecycle-consistency.md) | 依赖 D/E；DATA-02。 |
| EVO-118-G Runtime Reliability Gates | Proposed | [Item](backlog/active/EVO-118-G-runtime-reliability-gates.md) | 依赖 B/E；REL-01。 |
| EVO-118-H Durable Outbox | Proposed | [Item](backlog/active/EVO-118-H-durable-outbox-events.md) | 依赖 E/G；EVENT-01。 |

## Later

| Item | State | Owner Doc | Gate |
|------|-------|-----------|------|
| Repo-centric Web | In Progress / paused | [EVO-112](backlog/active/EVO-112-repo-management-ui.md) | Shell 已完成；详情页在 S1 关闭后恢复。 |
| Agent Integration + Vibe Coding | Proposed | [EVO-105/106/107/104](backlog/active/EVO-100-git-centric-platform-foundation.md) | S2 与 Repo UI 依赖满足。 |
| Indexer + Discovery + Compatibility | Proposed | [EVO-108/109/110](backlog/active/EVO-100-git-centric-platform-foundation.md) | Durable Event + Agent write semantics 稳定。 |
| Sandbox removal | Proposed | [EVO-111](backlog/active/EVO-111-deprecate-sandbox-runtime.md) | Index/compat 迁移稳定后收尾。 |
| SSH / LFS / Cross-repo search / Resource ACL | Proposal later | — | Phase E' 和 EVO-118 关闭后独立评估。 |

## Operating Review

- EVO-118-A/Iteration 050 已由 PR #2 合并并关闭。
- 当前产品方向仍是 Git hosting + Vibe Coding + capability discovery；EVO-118 是稳定化 Gate，不是产品回退。
- EVO-118-B/Iteration 051 已由 PR #3 合并并关闭；merge commit `6de7845e1231efc04f94f16cb9ab0a410f6ad2d9`，最终 CI run `30567361095` 全绿，SEC-01 已解除。
- EVO-118-C/Iteration 052 已由 PR #5 合并并关闭；实现 head `de762e2dae6bf5716e54c64e2277cb2e26592e36`，merge commit `936ed3b26a62840ddd94cf10e5075fd19e0a1c5c`，final-head CI #137 / run `30682419168` 全绿，SEC-02 已解除。
- EVO-112-A 的 2026-06-29 本地历史成果已按冲突恢复流程迁移为 Iteration 054；该恢复不改变 EVO-118-D/E 的 S1 优先顺序。
- Draft PR #7 正在推进 EVO-118-D / Iteration 053；当前没有 Review Iteration，DATA-01 仍未关闭。
- 当前启动顺序：EVO-118-D（In Progress）→ E → F/G/H → EVO-112-B（A 已恢复）→ EVO-105/106/107/104 → EVO-108/109/110 → EVO-111。
- 安全、数据损坏或生产构建问题允许显式 P0 插队；普通 UI、视觉、计费或内部文档不得静默绕过 S1。
- Board 只反映 owner docs；Gate 关闭必须由 Story 验收和实际验证证明。
