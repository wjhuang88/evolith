# EVO-114 Repo create 原子化硬化（Superseded）

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- [Replacement: EVO-118-F Repo Lifecycle Consistency](EVO-118-F-repo-lifecycle-consistency.md)
- [Production Readiness Baseline](../../reference/PRODUCTION-READINESS-BASELINE.md)
- [ITERATION-044](../../iterations/ITERATION-044.md) §10 Review 历史局限

## Summary

- 类型：tech-debt
- 优先级：P2（历史）
- 状态：Superseded
- 替代日期：2026-07-30
- 替代项：EVO-118-F

## Supersession Decision

本 Story 原本只处理 Repo create 的单点问题：DB 记录插入后 `gix::init_bare` 失败时回滚 DB。

2026-07-30 全面体检确认同一边界还包括：

- Delete 先删 DB、磁盘清理失败后形成不可追踪孤儿；
- Seed Template 直接写 bare repo 目录，没有形成 Git Commit；
- 缺少 `CREATING / ACTIVE / ERROR / DELETING` 或等价生命周期状态；
- 缺少 Trash、Reconciler 和 DB-only/disk-only 对账；
- 生产 Git Storage 持久化与恢复要求需要和生命周期共同验证。

因此不再启动本单点 Story。全部未完成范围由 [EVO-118-F](EVO-118-F-repo-lifecycle-consistency.md) 覆盖。

## Historical Goal

原计划目标保留作为决策证据：

1. disk init 失败时不返回 201 假成功；
2. DB 不保留不可用 Repo 记录；
3. 正常 create 仍产生 DB + bare Repo；
4. 不引入分布式事务或两阶段提交。

上述目标仍是 EVO-118-F 的必需验收子集，但实现方案不再预设为“简单回滚 DB”；可以采用状态机、补偿和 Reconciler。

## Residual Work Destination

- Create/Delete/Seed/Reconcile：EVO-118-F。
- Git 持久卷、备份和恢复：EVO-118-D。
- Production build/Volume 权限：EVO-118-E。
