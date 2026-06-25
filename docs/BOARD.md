# Operating Board

> Derived operating view only. Owner docs define status, scope, acceptance criteria,
> validation evidence and lifecycle state. Update owner docs first, then reflect the current
> operating state here.
>
> **2026-06-23 方向调整**：主线从 "Skill/CLI/MEP Registry" 转向 "Git 托管 + Vibe Coding 平台"。详见 [Git-Centric Platform Proposal](proposals/GIT-CENTRIC-PLATFORM.md) + [ADR-0004](decisions/ADR-0004-git-centric-storage.md) + [ADR-0005](decisions/ADR-0005-deprecate-sandbox-runtime.md)。

## Now

| Item | State | Owner Doc | Gate |
|------|-------|-----------|------|
| EVO-103-B-1 Git Client Auth Infra（Basic-Auth + RBAC + CSRF） | In Progress | [EVO-103-B-1](backlog/active/EVO-103-B-1-git-client-auth-infra.md)<br>[Iteration 045](iterations/ITERATION-045.md) | Phase E'-1b auth 基础设施；依赖 EVO-103-A Done；为 EVO-103-B-2 Smart HTTP 端点解锁 git 客户端鉴权。 |

## Review

| Item | State | Owner Doc | Gate |
|------|-------|-----------|------|
| None | - | - | - |

## Blocked Or Paused

| Item | State | Owner Doc | Gate |
|------|-------|-----------|------|
| Iteration 018 Skill 导入基础能力细化与基线 | Blocked for activation | [Iteration 018](iterations/ITERATION-018.md) | Blocked by 2026-06-23 方向调整（superseded by EVO-100/108）；如不 deliberate replanning 则不再激活。 |
| Iteration 019 Skill 多来源导入闭环 | Blocked for activation | [Iteration 019](iterations/ITERATION-019.md) | Blocked by 2026-06-23 方向调整（superseded）；如不 deliberate replanning 则不再激活。 |
| Iteration 020 Skill 版本与正确性验证 | Blocked for activation | [Iteration 020](iterations/ITERATION-020.md) | Blocked by 2026-06-23 方向调整（superseded）；如不 deliberate replanning 则不再激活。 |
| Iteration 025 租户设置与审计详情补齐 | Blocked for activation | [Iteration 025](iterations/ITERATION-025.md) | Resume only after EVO-012 / EVO-013 are refined with acceptance and validation. |
| Iteration 026 Stripe Webhook 与计费闭环恢复 | Blocked for activation | [Iteration 026](iterations/ITERATION-026.md) | Resume only after webhook security and mock-validation scope are refined. |
| Iteration 027 Skill 发现质量与描述治理 | Blocked for activation | [Iteration 027](iterations/ITERATION-027.md) | Blocked by 2026-06-23 方向调整（superseded by EVO-108/109）；如不 deliberate replanning 则不再激活。 |

## Next

| Item | State | Owner Doc | Gate |
|------|-------|-----------|------|
| EVO-057 生产 CORS Origin 可配置化 | Ready | [Product Backlog](backlog/PRODUCT-BACKLOG.md) | Start as a separate deploy/config micro-iteration if production custom-domain support should be hardened before product-mainline work. |
| EVO-080 Wasmer/WASI 替代 Docker sandbox 可行性 Spike | Ready | [Product Backlog](backlog/PRODUCT-BACKLOG.md) | Run after Phase E'-4 (EVO-111) if a non-Docker runtime candidate is still needed. |
| EVO-081 内部文档页面基于独立 Markdown 目录渲染 | Ready | [Product Backlog](backlog/PRODUCT-BACKLOG.md) | Start as a focused frontend/content story when internal docs should become visible in-app. |

## Later

| Item | State | Owner Doc | Gate |
|------|-------|-----------|------|
| EVO-100 Phase E'-2 (Agent 集成 + Vibe Coding UI) | Proposed | [EVO-105/106/107/104](backlog/active/EVO-100-git-centric-platform-foundation.md)<br>[UX Decisions](design/vibe-coding-ui-decisions.md) | Refine after Phase E'-1 closes; UX U-01~U-05 gate resolved，仍需 API / agent session 依赖就绪。 |
| EVO-100 Phase E'-3 (Indexer + Discovery + 旧表双写) | Proposed | [EVO-108/109/110](backlog/active/EVO-100-git-centric-platform-foundation.md) | Refine after Phase E'-2 closes; 与现有 skill/cli/mcp API 兼容是迁移期硬约束。 |
| EVO-100 Phase E'-4 (Sandbox 废弃收尾) | Proposed | [EVO-111](backlog/active/EVO-111-deprecate-sandbox-runtime.md) | Refine after Phase E'-3 closes; ADR-0005 收口。 |
| Phase 5+ 扩展（SSH / LFS / 跨仓搜索 / 资源级 ACL） | 待提案 | — | Independent EVOs after Phase E' closes. |

## Operating Review

Current iteration ordering after 2026-06-23 direction pivot:

- 主线切换：从 Phase E（Skill/CLI/MEP Lifecycle）切换为 Phase E'（Git 托管 + Vibe Coding）。
- `EVO-100` 是 Epic，已进入 In Progress；EVO-101/102 已由 Iteration 042 完成，EVO-113 已完成方向变更评审缺口修复。
- 后续启动顺序建议 EVO-103（后端 Repo CRUD + Smart HTTP）→ EVO-112（仓库管理 UI）→ Phase E'-2（EVO-104/105/106/107，UX gate 已解除）→ Phase E'-3 → Phase E'-4。
- 旧 Phase E 相关 Iterations 018-020、027 因方向调整标 superseded；Iterations 025/026 保持独立（与方向调整无关）。
- 旧 backlog 项（EVO-019/020/027/028/029/045/046/047/049/049-B/050）已在 PRODUCT-BACKLOG.md Archived Index 标注 Superseded/Dropped。
- `EVO-061` 至 `EVO-076` 已由 `EVO-086` 一次性 latest 迁移收口。
- UX 调研前置门禁：EVO-104 Vibe Coding Web UI 的 U-01~U-05 已在 `docs/design/vibe-coding-ui-decisions.md` 决策完成；不阻塞 EVO-103 后端基础能力，也不再阻塞 Phase E'-2 refinement。EVO-104 实现仍需等待 EVO-103/105/106/112 依赖。
- Use P2 Ready items only as explicit micro-iteration interruptions; do not silently bypass the selected product-mainline order.
