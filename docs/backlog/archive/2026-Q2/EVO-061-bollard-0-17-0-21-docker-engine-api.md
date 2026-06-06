# EVO-061 bollard 0.17→0.21 Docker Engine API 升级

> Archived backlog item. Routing index: [2026 Q2 Archive](INDEX.md). Active routing surface: [Product Backlog](../../PRODUCT-BACKLOG.md).

- Type: tech-debt
- Status: Done
- Priority: P3
- Source: Iteration 032 依赖审计暂缓项
- Closure: Completed by EVO-086 on 2026-06-06.
- Evidence: `bollard = 0.21.0`; `service-skill` container create/start/delete migrated to `bollard::models::ContainerCreateBody` and `bollard::query_parameters`; `cargo check --workspace --all-targets`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace` passed.
