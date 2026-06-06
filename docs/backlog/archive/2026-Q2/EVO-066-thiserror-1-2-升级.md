# EVO-066 thiserror 1→2 升级

> Archived backlog item. Routing index: [2026 Q2 Archive](INDEX.md). Active routing surface: [Product Backlog](../../PRODUCT-BACKLOG.md).

- Type: tech-debt
- Status: Done
- Priority: P2
- Source: Iteration 032 依赖审计暂缓项
- Closure: Completed by EVO-086 on 2026-06-06.
- Evidence: `thiserror = 2.0.18`; workspace remains Rust edition 2021, MSRV raised to 1.88 due latest dependency requirements; check, clippy and tests passed.
