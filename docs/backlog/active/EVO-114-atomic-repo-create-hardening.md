# EVO-114 Repo create 原子化硬化（disk init 失败回滚 DB）

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- 依赖: [EVO-103-A](EVO-103-A-repo-crud.md)（Done）
- [ITERATION-044](../../iterations/ITERATION-044.md) §10 Review 已知局限

## Summary

- 类型：tech-debt
- 优先级：P2
- 状态：Proposed
- 依赖：EVO-103-A（Done）

## Problem Or Outcome

当前 `POST /api/v1/tenant/{tenant_id}/repos` 在 DB 记录插入后、磁盘 `gix::init_bare` 失败时仅记录 warn 日志并保留 DB 记录（best-effort）。这导致 DB 中存在指向不存在裸仓的 repo 记录，后续 Smart HTTP（EVO-103-B）或 Context API（EVO-103-C）读取 storage_path 定位仓库时会失败。

## Goal And Non-Goals

- **Goal**：
  1. 使 `POST /api/v1/tenant/{tenant_id}/repos` 原子化：disk init 失败时回滚 DB 记录并返回 5xx。
  2. 新增测试：create-succeeds（正常路径）+ create-disk-failure-rolls-back（disk init 失败时 DB 回滚）。
- **Non-goals**：
  - 不改变 delete 的 best-effort 磁盘清理语义（DB 为权威源，磁盘清理失败 warn 即可）。
  - 不引入分布式事务或两阶段提交。

## Dependencies And Blockers

- 硬依赖 EVO-103-A（Done）：现有 create handler + service-git crate。
- 无阻塞项。

## Acceptance Criteria

- [ ] disk init 失败时 DB 记录已回滚（查询不到该 repo）
- [ ] disk init 失败时返回 5xx（非 201 + warn）
- [ ] 正常 create 路径行为不变（201 + DB + disk）
- [ ] `cargo test --workspace` 与 `cargo clippy --workspace --all-targets -- -D warnings` 全绿

## Validation Evidence Required

- 测试 `create_succeeds`：正常路径验证 201 + DB record + disk bare repo。
- 测试 `create_disk_failure_rolls_back`：模拟 disk init 失败（如权限错误/路径不可写），验证 DB 无记录 + 返回 5xx。

## Residual Work Destination

- 无。

## Source Snapshot

- Source: ITERATION-044 §10 Review 已知局限（2026-06-25）。
- Decision context: EVO-103-A 收口时确认 storage_path 已正确对齐，best-effort 可接受；但为后续 Smart HTTP / Context API 可靠性，需硬化为原子 create。
