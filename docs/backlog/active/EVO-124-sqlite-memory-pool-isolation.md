# EVO-124 SQLite 内存数据库连接池共享一致性

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- [Local Development SOP](../../sop/LOCAL-DEV.md)
- [Testing SOP](../../sop/TESTING.md)
- `backend/crates/infra/src/db/mod.rs`

## Summary

- 类型：bug / backend / local-dev
- 优先级：P1
- 状态：Proposed
- 父 Epic：无

## Problem Or Outcome

本地默认 `DATABASE__URL=:memory:` 与多连接 SQLite pool 组合时，各连接可能获得独立内存数据库。Iteration 062 的真实登录流程在同一运行进程中出现 `no such table: users`：migrations/注册使用的连接与后续登录连接看不到同一 schema/data。

目标是让 lite/local 多请求流程稳定共享同一 SQLite 数据库，不能把随机命中已迁移连接当作成功。

## Goal And Non-Goals

- Goal：确定并实现唯一受支持的内存共享策略（shared-cache URI + pool 约束，或 local 默认临时文件），并用跨连接 migration/register/login 测试证明一致性。
- Non-goals：不改变 PostgreSQL 生产行为，不在 EVO-120 内顺手修改 infra。

## Dependencies And Blockers

- 依赖当前 SQLite pool 构造与 migration 流程的代码审查。
- 不阻塞 Iteration 062；其浏览器验收改用显式临时 SQLite 文件。

## Acceptance Criteria

- [ ] 测试在至少两个 pool connection 上验证 schema 与注册数据可见。
- [ ] `./scripts/dev.sh lite` 连续 register/login/list Repo 不再出现缺表或数据消失。
- [ ] SQLite 文件模式与 PostgreSQL 行为不退化。
- [ ] Local Dev/Config 文档与实际默认值一致。

## Validation Evidence Required

- `cargo test -p infra` 与相关 auth/repo API 集成测试。
- 实际 lite runtime 多请求 smoke。
- Markdown links 与 `git diff --check`。

## Residual Work Destination

- 与本缺陷无关的端口监听误判继续归 EVO-123。

## Source Snapshot

- 2026-08-09 Iteration 062：同一 backend 进程中注册曾成功，随后 browser login 返回 500；日志明确为 SQLite `(code: 1) no such table: users`。
