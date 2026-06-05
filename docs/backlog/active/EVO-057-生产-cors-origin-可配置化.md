# EVO-057 生产 CORS Origin 可配置化

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- [Config reference](../../reference/CONFIG.md)

## Summary

| Field | Value |
| --- | --- |
| ID | EVO-057 |
| Type | tech-debt |
| Status | Ready |
| Priority | P2 |
| Parent Epic | None recorded |
| Source | 跨层一致性审查 2026-06-01 |

## Problem Or Outcome

docker-compose.prod.yml 的 `CORS__ALLOWED_ORIGINS` 被忽略，main.rs 硬编码 origin；新增 CorsConfig 使其可配置

## Goal And Non-Goals

- Goal: deliver the outcome described by the title and decision context without expanding scope during implementation.
- Non-goals: do not implement adjacent backlog items unless they are listed as hard dependencies in this file.

## Dependencies And Blockers

- No hard blocker recorded in the source backlog.

## Governing ADRs, Specs Or Decisions

- [Config reference](../../reference/CONFIG.md)

## Acceptance Criteria

- 类型：tech-debt
- 优先级：P2
- 状态：Ready
- 父 Epic：无
- Story 形态：Technical
- 用户价值或技术目标：
  - 为了：让生产 CORS 允许来源可通过环境变量配置，而非硬编码。
  - 运维者需要：`docker-compose.prod.yml:69` 设置的 `CORS__ALLOWED_ORIGINS` 真正生效。
  - 以便：部署到非 `evolith.io`/`app.evolith.io` 域名时无需改代码。
- 背景：`config.rs` 无 `CorsConfig` 结构；`main.rs:222` 生产 origin 硬编码为两个域名，环境变量被静默忽略。
- 范围（本次做）：
  1. 新增 `CorsConfig`（嵌套双下划线键 `CORS__ALLOWED_ORIGINS`，逗号分隔多 origin），并入 `AppConfig`。
  2. `main.rs` 生产分支按配置构建 CORS allowed origins；保留开发宽松策略。
  3. 同步 `docs/reference/CONFIG.md` 与 AGENTS.md 配置键说明（注意 AGENTS 现写的是单数 `CORS__ALLOWED_ORIGIN`，需统一）。
- 不做：
  - 不改 CSRF / 安全头中间件。
  - 不放宽开发模式策略。
- 验收标准：
  - [ ] 设置 `CORS__ALLOWED_ORIGINS` 后生产构建按其值放行，未设置时回退到安全默认。
  - [ ] `config.rs` 存在 `CorsConfig`，键格式为双下划线。
  - [ ] `CONFIG.md` 与 AGENTS.md 配置键描述一致（单复数统一）。
  - [ ] `cargo test --workspace` 通过。
- 依赖或阻塞：无。
- 解锁内容：生产多域名部署无需改代码。
- 影响范围：backend(infra/api) / docs
- 最小验证方式：设置不同 `CORS__ALLOWED_ORIGINS` 启动验证响应头；`cargo test --workspace`。

## Validation Evidence Required

- Record real command output or manual evidence in the selected iteration before marking this item Done.
- Run checks matching the changed surface from [Testing SOP](../../sop/TESTING.md).

## Residual Work Destination

- Update this file and `docs/backlog/PRODUCT-BACKLOG.md` if scope, status, dependencies or Required Reads change.
- Move completed or deferred execution context to `docs/backlog/archive/<period>/` during backlog compaction.

## Source Snapshot

- Source: 跨层一致性审查 2026-06-01
- Decision context: docker-compose.prod.yml 的 `CORS__ALLOWED_ORIGINS` 被忽略，main.rs 硬编码 origin；新增 CorsConfig 使其可配置
