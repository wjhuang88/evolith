# Operating Board

> Derived operating view only. Owner docs define status, scope, acceptance criteria,
> validation evidence and lifecycle state. Update owner docs first, then reflect the current
> operating state here.

## Now

| Item | State | Owner Doc | Gate |
|------|-------|-----------|------|
| None | - | - | - |

## Review

| Item | State | Owner Doc | Gate |
|------|-------|-----------|------|
| None | - | - | - |

## Blocked Or Paused

| Item | State | Owner Doc | Gate |
|------|-------|-----------|------|
| Iteration 018 Skill 导入基础能力细化与基线 | Blocked for activation | [Iteration 018](iterations/ITERATION-018.md) | Resume only after candidate stories satisfy DoR/refinement. |
| Iteration 019 Skill 多来源导入闭环 | Blocked for activation | [Iteration 019](iterations/ITERATION-019.md) | Resume only after Iteration 018 is completed or deliberately replanned. |
| Iteration 020 Skill 版本与正确性验证 | Blocked for activation | [Iteration 020](iterations/ITERATION-020.md) | Resume only after Iteration 019 is completed or deliberately replanned. |
| Iteration 025 租户设置与审计详情补齐 | Blocked for activation | [Iteration 025](iterations/ITERATION-025.md) | Resume only after EVO-012 / EVO-013 are refined with acceptance and validation. |
| Iteration 026 Stripe Webhook 与计费闭环恢复 | Blocked for activation | [Iteration 026](iterations/ITERATION-026.md) | Resume only after webhook security and mock-validation scope are refined. |
| Iteration 027 Skill 发现质量与描述治理 | Blocked for activation | [Iteration 027](iterations/ITERATION-027.md) | Resume only after the boundary with Skill import/version work is confirmed. |

## Next

| Item | State | Owner Doc | Gate |
|------|-------|-----------|------|
| EVO-057 生产 CORS Origin 可配置化 | Ready | [Product Backlog](backlog/PRODUCT-BACKLOG.md) | Start as a separate deploy/config micro-iteration if production custom-domain support should be hardened before product-mainline work. |
| EVO-080 Wasmer/WASI 替代 Docker sandbox 可行性 Spike | Ready | [Product Backlog](backlog/PRODUCT-BACKLOG.md) | Run before adding more Docker-specific execution work if daemon dependency remains a product risk. |
| EVO-081 内部文档页面基于独立 Markdown 目录渲染 | Ready | [Product Backlog](backlog/PRODUCT-BACKLOG.md) | Start as a focused frontend/content story when internal docs should become visible in-app. |

## Later

| Item | State | Owner Doc | Gate |
|------|-------|-----------|------|
| EVO-045 CLI 命令执行引擎（Serverless） | Proposed | [Product Backlog](backlog/PRODUCT-BACKLOG.md) | Refine through child stories after EVO-045-A establishes the shared execution provider. |
| EVO-047 MCP 工具 Serverless 执行 | Proposed | [Product Backlog](backlog/PRODUCT-BACKLOG.md) | Refine after EVO-045-A establishes the shared execution boundary. |
| EVO-027 / EVO-028 / EVO-046 / EVO-050 Skill 生态能力 | Proposed | [Product Backlog](backlog/PRODUCT-BACKLOG.md) | Refine after EVO-049-A/B establish model and parser baselines. |

## Operating Review

Current iteration ordering is reasonable after the 2026-06-04 repair:

- `EVO-048 / Iteration 033` is Closed and should not appear in Now.
- `EVO-049-A / Iteration 034` and `EVO-045-A / Iteration 041` are Closed.
- `EVO-045-A` follow-up restored lite/local startup so Docker is required only when sandbox is
  explicitly enabled.
- `EVO-052 / EVO-056` were completed in Iteration 038; the remaining "do not delegate casually"
  candidate is `EVO-057`; `EVO-058` is already Closed.
- `EVO-080` captures the non-Docker runtime question as a Spike before committing to Wasmer or
  Wasmtime.
- `EVO-081` captures the internal Markdown-backed docs page as a Ready product story.
- `EVO-061` 至 `EVO-076` 已由 `EVO-086` 一次性 latest 迁移收口，不再作为 Later 候选项保留。
- Keep Iterations 018-020 and 025-027 blocked until their refinement/dependency gates are met.
- Use P2 Ready items only as explicit micro-iteration interruptions; do not silently bypass the
  selected product-mainline order.
