# EVO-075 rand 0.8 私有依赖收敛

> Archived backlog item. Routing index: [2026 Q2 Archive](INDEX.md). Active routing surface: [Product Backlog](../../PRODUCT-BACKLOG.md).

- Type: tech-debt
- Status: Done
- Priority: P3
- Source: Iteration 032 依赖审计暂缓项
- Closure: Completed by EVO-086 on 2026-06-06.
- Evidence: `service-auth` no longer directly depends on `rand`; Argon2 salt generation now uses `argon2::password_hash::rand_core::OsRng`, avoiding `rand_core` version mismatch while removing the outdated direct dependency.
