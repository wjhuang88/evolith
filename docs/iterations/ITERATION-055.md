# Iteration 055 — Repo Lifecycle Consistency

> 文档状态：Closed / Complete
> 目标 Story：[EVO-118-F](../backlog/active/EVO-118-F-repo-lifecycle-consistency.md)
> 父 Epic：[EVO-118](../backlog/active/EVO-118-production-readiness-and-security-hardening.md)
> 激活日期：2026-08-08

## 1. 迭代目标

将 Repo 创建、Seed、删除和 DB/Filesystem 盘点从 best-effort 行为收敛为可观察、可补偿、可恢复的生命周期边界，关闭 DATA-02。

## 2. 选入 Story 与依赖

| Story | 状态 | 依赖 | 顺序 |
| --- | --- | --- | --- |
| EVO-118-F | Done / Complete | EVO-118-D Done | 单一 Story |

## 3. 不做事项

- 不实现 Commit/Promote、Agent Session、Vibe Coding 或生产部署。
- 不实现跨区域复制、LFS、对象存储 Git 后端或通用工作流引擎。
- 不修改已发布生产镜像；最终部署归 EVO-118-E。

## 4. 验收与验证计划

- 创建：Git 初始化失败不返回可用 Repo；DB 状态进入 ERROR 或可恢复回滚。
- Seed：README 与 `.evolith/*` 写入真实 Blob/Tree/Initial Commit，并更新默认 Ref/HEAD。
- 删除：失败进入 DELETING/ERROR 或 Trash，保留可盘点记录。
- 盘点：识别 DB-only、disk-only、正常 Repo，并输出可操作分类。
- SQLite/PostgreSQL repository 与 migration 行为保持一致。
- 验证：`cargo fmt --all -- --check`、`cargo test -p infra`、`cargo test --workspace`；真实 Git client E2E；失败注入和跨租户负向测试。

## 5. 风险与回滚

| 风险 | 处理 |
| --- | --- |
| 现有 Repo 数据没有 lifecycle 字段 | 新增配对 migration，默认历史记录为 ACTIVE，并先验证 upgrade 路径 |
| Git 文件系统操作失败 | DB 状态优先保留可盘点状态；物理清理不静默报告成功 |
| Seed plumbing 改动破坏 clone | 先写失败测试，再用最小 gix plumbing 实现并保留旧无 seed 创建路径 |

### 安全审查

| 项目 | 内容 |
| --- | --- |
| 受保护资产 | Tenant Repo metadata、Git object/ref、磁盘仓库与隔离目录 |
| 攻击者/调用者 | Member、Owner/Admin、API Key、跨租户 JWT 调用者 |
| 入口 | Repo create/delete/reconcile HTTP route、DB repository、Git storage filesystem |
| 信任边界 | Browser/API client -> API -> tenant-scoped DB + Git storage |
| 失败模式 | 跨租户盘点、API Key 提权、初始化假成功、删除后孤儿、误删 disk-only 数据、路径逃逸 |
| 安全默认 | fail closed；失败标记 ERROR；disk-only 只隔离不删除；Reconcile 仅 Owner/Admin JWT |
| 验证证据 | Member/API Key/跨租户负向测试、DB/FS 失败注入、盘点分类与隔离测试、路径范围审查 |

## 6. 闭环台账

| 项目 | 本轮记录 |
| --- | --- |
| 请求结果 | 完成 Repo 生命周期状态、真实 Initial Commit、Reconciler 与双数据库一致性，关闭 DATA-02 |
| 产物 | 生命周期 domain/repository/service/API；SQLite/PostgreSQL migration 010；真实 Git Seed；失败注入、权限、Git client 与 PostgreSQL 测试；API Contract |
| 状态同步归口 | EVO-118-F、EVO-118、Product Backlog、Board、Iteration 055 |
| 验证证据 | workspace test、strict Clippy、Reconcile 权限负向测试、真实 PostgreSQL 009 -> 010 upgrade/round trip、Navigator 阶段复核均通过 |
| 残余工作归口 | Runtime reliability -> EVO-118-G；durable event -> EVO-118-H；最终部署 -> EVO-118-E；多实例共享存储另行评估 |

## 7. 执行记录

| 日期 | 类型 | 记录 |
| --- | --- | --- |
| 2026-08-08 | activation | 盘点历史 Blocked/Superseded iterations；无可继续 Active/Review/Planned runtime 迭代；EVO-118-F 满足 DoR，激活为 In Progress。 |
| 2026-08-08 | implementation | 引入 `CREATING / ACTIVE / ERROR / DELETING`、应用服务、migration 010、真实 Initial Commit 和 Owner/Admin JWT Reconcile API。 |
| 2026-08-08 | negative tests | 初始化/Seed/删除失败注入通过；Member、跨租户 JWT、API Key、tenant storage symlink 均 fail closed；disk-only 只隔离不删除。 |
| 2026-08-08 | database | SQLite repository/migration 测试通过；PostgreSQL 16 真实 009 -> 010 upgrade 与 lifecycle round trip 通过。 |
| 2026-08-08 | final validation | `cargo test --workspace`、`cargo clippy --workspace --all-targets -- -D warnings`、`cargo fmt --all -- --check` 通过。 |
| 2026-08-08 | navigator | Navigator 阶段复核 `Complete`；权限、Git 数据保护、双数据库与失败恢复未发现阻断项。 |

## 8. Review

- 完成：Repo lifecycle 状态机、真实 Initial Commit、DB/FS Reconciler、隔离策略、双数据库 migration/repository 与权限审计边界。
- 验证结果：workspace 全量测试与严格 Clippy 通过；真实 PostgreSQL upgrade/round trip 通过；安全负向矩阵与 Git clone/read 验证通过。
- Navigator：`Complete`，无阻断项。
- 闭环状态：`Complete`
- 残余归口：EVO-118-G/H/E 及多实例共享存储；均不在本迭代授权范围内。

## 9. Retrospective

- 状态机 + 补偿 + Reconciler 能在不伪造跨介质 ACID 的前提下消除 HTTP 假成功，并保留可操作恢复状态。
- 未知 disk-only 数据默认隔离而非删除，降低自动修复的数据损坏风险。
- 用户可见文档不受本轮影响；稳定 API 合约已同步。Iteration 055 关闭后不自动启动 EVO-118-G，下一迭代仍需重新执行 START-ITERATION。
