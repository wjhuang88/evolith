# Operating Board

> Derived operating view only. Owner docs define status, scope, acceptance criteria,
> validation evidence and lifecycle state. Update owner docs first, then reflect the current
> operating state here.
>
> **2026-07-30 生产就绪重排**：Git-centric 产品方向保持不变，但 EVO-118 S1 安全、数据耐久性和生产构建 Gate 已前置到 Repo UI。事实见 [Production Readiness Baseline](reference/PRODUCTION-READINESS-BASELINE.md)，顺序见 [Production Readiness Plan](roadmap/PRODUCTION-READINESS-PLAN-2026-07.md)。

## Now

| Item | State | Owner Doc | Gate |
|------|-------|-----------|------|
| EVO-118 Production Readiness Epic | In Progress | [Epic](backlog/active/EVO-118-production-readiness-and-security-hardening.md) | S1 必须按 WIP 逐项关闭，不能整包声称完成。 |
| EVO-118-B API Key / MCP Authorization | Review / Partial | [Item](backlog/active/EVO-118-B-api-key-mcp-authorization-hardening.md)<br>[Iteration 051](iterations/ITERATION-051.md) | Draft PR #3 已提交；等待 Rust/前端 hard-required 门禁。 |

## Review

| Item | State | Owner Doc | Gate |
|------|-------|-----------|------|
| PR #3 API Key 与 MCP 授权硬化 | Draft / Review | [EVO-118-B](backlog/active/EVO-118-B-api-key-mcp-authorization-hardening.md)<br>[Authorization Contract](reference/API-KEY-AUTHORIZATION.md) | `cargo fmt/clippy/test` 与 `bun type-check/build` 全通过后才可合并并解除 SEC-01。 |
| Iteration 051 | Review / Partial | [Iteration 051](iterations/ITERATION-051.md) | 未关闭前不启动 EVO-118-C 或 Repo UI。 |

## Blocked Or Paused

| Item | State | Owner Doc | Gate |
|------|-------|-----------|------|
| EVO-112 Repo Management UI | Proposed / paused for implementation | [Item](backlog/active/EVO-112-repo-management-ui.md) | EVO-118 S1（B/C/D/E）关闭后恢复；当前可 refinement。 |
| EVO-105/106/107/104 Agent Write Loop | Proposed | [EVO-100](backlog/active/EVO-100-git-centric-platform-foundation.md) | 依赖 EVO-118-B/F/H 与 Repo UI 基础；不得绕过 Typed Capability、Policy 和 Durable Event。 |
| EVO-108/109/110 Index/Discovery | Proposed | [EVO-100](backlog/active/EVO-100-git-centric-platform-foundation.md) | 依赖稳定 Push/Commit Event 与 EVO-118-H。 |
| Iterations 018~020、027 | Superseded / Blocked for activation | [Iteration index](iterations/README.md) | 旧 Registry 主线被 Git-centric 方向替代，不 deliberate replan 则不激活。 |
| Iterations 025/026 | Blocked for activation | [Iteration index](iterations/README.md) | Phase F 独立候选，不抢占 EVO-118 S1。 |

## Next

| Item | State | Owner Doc | Gate |
|------|-------|-----------|------|
| EVO-118-C HTTP Tool Egress Security | Ready | [Item](backlog/active/EVO-118-C-http-tool-egress-security.md) | SEC-02；仅在 EVO-118-B 合并并关闭后启动。 |
| EVO-118-D Git Durability and Recovery | Ready | [Item](backlog/active/EVO-118-D-git-storage-durability-and-recovery.md) | DATA-01；持久卷 + DB/Git 恢复演练。 |
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
- EVO-118-B 实现已进入 Draft PR #3，但缺 hard-required 运行验证，SEC-01 尚未解除。
- 当前启动顺序：完成 PR #3 验证/合并 → EVO-118-C → D → E → F/G/H → EVO-112 → EVO-105/106/107/104 → EVO-108/109/110 → EVO-111。
- 安全、数据损坏或生产构建问题允许显式 P0 插队；普通 UI、视觉、计费或内部文档不得静默绕过 S1。
- Board 只反映 owner docs；Gate 关闭必须由 Story 验收和实际验证证明。