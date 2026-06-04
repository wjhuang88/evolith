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
| EVO-049-A Skill/CLI 规范兼容数据模型基线 | Planned / Ready for activation | [Iteration 034](iterations/ITERATION-034.md) | Activate via START-ITERATION inventory; preserve the published baseline. |
| EVO-045-A ExecutionProvider 统一 trait + Docker 容器池化 | Ready | [Product Backlog](backlog/PRODUCT-BACKLOG.md) | Start as a new iteration only after choosing to prioritize execution-layer infrastructure over the published Iteration 034 data-model baseline. |
| EVO-057 生产 CORS Origin 可配置化 | Ready | [Product Backlog](backlog/PRODUCT-BACKLOG.md) | Start as a separate deploy/config micro-iteration if production custom-domain support should be hardened before product-mainline work. |
| EVO-058 前端死代码与类型卫生清理 | Ready | [Product Backlog](backlog/PRODUCT-BACKLOG.md) | Start as a separate frontend hygiene micro-iteration if type-contract drift should be reduced before larger frontend changes. |

## Later

| Item | State | Owner Doc | Gate |
|------|-------|-----------|------|
| EVO-045 CLI 命令执行引擎（Serverless） | Proposed | [Product Backlog](backlog/PRODUCT-BACKLOG.md) | Refine through child stories after EVO-045-A establishes the shared execution provider. |
| EVO-047 MCP 工具 Serverless 执行 | Proposed | [Product Backlog](backlog/PRODUCT-BACKLOG.md) | Refine after EVO-045-A establishes the shared execution boundary. |
| EVO-027 / EVO-028 / EVO-046 / EVO-050 Skill 生态能力 | Proposed | [Product Backlog](backlog/PRODUCT-BACKLOG.md) | Refine after EVO-049-A/B establish model and parser baselines. |
| EVO-061 至 EVO-076 依赖升级 | Proposed | [Product Backlog](backlog/PRODUCT-BACKLOG.md) | Pull forward only when a specific compatibility risk exceeds the current P0 product mainline. |

## Operating Review

Current iteration ordering is reasonable after the 2026-06-04 repair:

- `EVO-048 / Iteration 033` is Closed and should not appear in Now.
- Keep `EVO-049-A / Iteration 034` as the next planned product-mainline slice because it selects
  a Ready child Story rather than the parent Epic.
- `EVO-045-A` is now recorded as a Ready child Story and may be started as a new iteration if the
  execution-layer path should interrupt the planned data-model baseline.
- `EVO-052 / EVO-056` were completed in Iteration 038; the remaining "do not delegate casually"
  candidates are `EVO-057` and `EVO-058`.
- Keep Iterations 018-020 and 025-027 blocked until their refinement/dependency gates are met.
- Use P2 Ready items only as explicit micro-iteration interruptions; do not silently bypass the
  selected product-mainline order.
