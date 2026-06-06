# EVO-086 全量依赖 latest 迁移

> Archived backlog item. Routing index: [2026 Q2 Archive](INDEX.md). Active routing surface: [Product Backlog](../../PRODUCT-BACKLOG.md).

- Type: tech-debt
- Status: Done
- Priority: P1
- Source: 用户反馈 2026-06-06
- Decision Context: 升级过程不考虑兼容保守策略，后端/前端依赖直接追 latest；遇到 breaking change 同步迁移代码。

## Closure Summary

- 后端 workspace 依赖升级到查得 latest：Actix、sqlx、bollard、validator、jsonwebtoken、jsonschema、reqwest、hmac、sha2、redis、actix-governor、actix-web-prom、prometheus、lettre、async-stripe 等。
- 前端依赖升级到查得 latest：React 19.2.7、Vite 8.0.16、Tailwind 4.3.0、TypeScript 6.0.3、ESLint 10.4.1、React Router 7.17.0、Zustand 5.0.14 等。
- Rust MSRV 提升到 1.88，`backend/Dockerfile` 和 TECH-STACK 同步。
- `async-stripe 1.0.0-rc.6` 生成类型重排后，service-payment helper 改为 Stripe REST JSON；计费业务闭环仍归口 EVO-014。
- `serde_yaml` 替换为 `serde_norway`；`service-auth` 移除直接 `rand` 依赖。

## Validation Evidence

- `cargo fmt --all -- --check` passed.
- `cargo check --workspace --all-targets` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed with elevated permissions for local mock HTTP server binding.
- `bun outdated` returned no outdated direct frontend packages.
- `bun run type-check` passed.
- `bun run build` passed; remaining output is Vite chunk-size warning and Node `module.register()` deprecation warning.

## Residual Work

- Stripe webhook / billing business closure remains EVO-014.
- Vite chunk-size warning is informational and can be handled as a separate frontend performance item if it becomes a delivery concern.
