# EVO-122-C Drop Legacy Registry Tables

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- [Parent Epic](EVO-122-retire-prelaunch-registry-backend.md)
- [ADR-0009](../../decisions/ADR-0009-no-prelaunch-registry-compatibility.md)
- [EVO-122-B Runtime Removal](EVO-122-B-remove-legacy-registry-runtime.md)
- [Database Migration SOP](../../sop/DATABASE-MIGRATION.md)
- [Release SOP](../../sop/RELEASE.md)

## Summary

- 类型：tech-debt / db
- 优先级：P1
- 状态：Proposed / paused
- 父 Epic：`EVO-122`
- 影响范围：db、backend migrations、tests、reference

## Problem Or Outcome

Runtime consumer 删除后，旧表若继续留在 schema，会诱导新代码重新依赖，并让备份/恢复与数据模型继续携带无主资产。

为了完成单一事实源收敛，维护者需要在两个数据库实现中删除旧表，并证明 fresh install 与 upgrade 都一致。

## Goal And Non-goals

- Goal：新增配对 migration 删除 `tools` / `skills` / `snippets` 及仅服务它们的关联对象；更新 schema/reference/fixtures；验证 fresh 与 upgrade。
- Non-goals：不迁移旧表内容到 Git；不触碰身份、权限、Repo metadata、index 或 audit 数据。

## Dependencies And Blockers

- EVO-122-B Done，consumer inventory 为零。
- 启动时确认没有需要保留的外部/生产 Registry 数据；若假设不成立，暂停并新建决策，不执行 destructive migration。

## Acceptance Criteria

- [ ] SQLite 与 PostgreSQL 配对 migration 均存在并语义一致。
- [ ] 从空库执行全量 migration 后旧表不存在，目标 Repo/index/event schema 正常。
- [ ] 从包含 legacy fixture 的升级路径执行后旧表删除，其他数据保持完整。
- [ ] 应用启动、readiness、Repo/Discovery/MCP execute tests 通过。
- [ ] rollback/restore 策略明确；任何失败不返回假成功。

## Validation Evidence Required

- SQLite + PostgreSQL fresh/upgrade integration tests。
- `cargo test -p infra`、`cargo test --workspace` 及全量 baseline。
- schema inventory before/after、备份恢复 smoke、Navigator data review。
- Reference/Project Map/production backup scope 同步。

## Residual Work Destination

- 发现未知 consumer 或需保留数据时保持 Blocked，并建立独立盘点/导出 Story；不恢复 EVO-110 双写。
