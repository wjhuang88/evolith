# Operating Board

> Derived operating view only. Owner docs define status, scope, acceptance criteria,
> validation evidence and lifecycle state. Update owner docs first, then reflect the current
> operating state here.

## Now

| Item | State | Owner Doc | Gate |
|------|-------|-----------|------|
| EVO-048 Serverless 执行架构设计 Spike | Active / In Progress | [Iteration 033](iterations/ITERATION-033.md) | Close when serverless runtime proposal/ADR, reuse conclusion, interface boundary and follow-up dependency map are recorded and validated. |

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
| EVO-049-A Skill/CLI 规范兼容数据模型基线 | Planned / Ready for activation | [Iteration 034](iterations/ITERATION-034.md) | Activate after Iteration 033 is closed or explicitly paused; preserve the published baseline. |
| EVO-056 沙箱降级静默成功修复 | Ready | [Product Backlog](backlog/PRODUCT-BACKLOG.md) | Start as a separate micro-iteration if execution-layer risk should be reduced before EVO-049-A. |
| EVO-060 dev.sh EMBEDDED_FRONTEND/ZIP 死代码清理 | Ready | [Product Backlog](backlog/PRODUCT-BACKLOG.md) | Start as a separate script-change micro-iteration and include script release notes. |

## Later

| Item | State | Owner Doc | Gate |
|------|-------|-----------|------|
| EVO-045 CLI 命令执行引擎（Serverless） | Proposed | [Product Backlog](backlog/PRODUCT-BACKLOG.md) | Refine only after EVO-048 records the runtime architecture and dependency map. |
| EVO-047 MCP 工具 Serverless 执行 | Proposed | [Product Backlog](backlog/PRODUCT-BACKLOG.md) | Refine only after EVO-048 records the shared execution boundary. |
| EVO-027 / EVO-028 / EVO-046 / EVO-050 Skill 生态能力 | Proposed | [Product Backlog](backlog/PRODUCT-BACKLOG.md) | Refine after EVO-049-A/B establish model and parser baselines. |
| EVO-061 至 EVO-076 依赖升级 | Proposed | [Product Backlog](backlog/PRODUCT-BACKLOG.md) | Pull forward only when a specific compatibility risk exceeds the current P0 product mainline. |

## Operating Review

Current iteration ordering is reasonable:

- Keep `EVO-048 / Iteration 033` as the active Spike because CLI/MCP serverless execution depends
  on its architecture answer.
- Keep `EVO-049-A / Iteration 034` as the next planned product-mainline slice because it selects
  a Ready child Story rather than the parent Epic.
- Keep Iterations 018-020 and 025-027 blocked until their refinement/dependency gates are met.
- Use P2 Ready items only as explicit micro-iteration interruptions; do not silently bypass the
  active Spike.
