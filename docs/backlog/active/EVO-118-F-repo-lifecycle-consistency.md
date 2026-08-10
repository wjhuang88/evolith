# EVO-118-F Repo 生命周期一致性与 Initial Commit

- **类型**：Technical / Data Consistency
- **状态**：Done / Complete
- **优先级**：P1
- **父 Epic**：[EVO-118](EVO-118-production-readiness-and-security-hardening.md)
- **依赖**：EVO-118-D（Done）
- **当前迭代**：Iteration 055
- **影响范围**：backend / domain / db / git storage / tests / docs

## 工程目标

消除 Repo 数据库记录与 Git 文件系统之间的 best-effort 分裂，使创建、初始化、删除和 Seed Template 具有可观察状态、补偿路径和可恢复性。

## 已确认失败模式

- Repo 创建先写 DB，磁盘初始化失败只记录 warning，仍返回 201。
- Repo 删除先删 DB，磁盘删除失败只记录 warning，可能留下不可追踪孤儿。
- Seed Template 直接把 README 和 `.evolith/*` 写入 bare repo 目录，没有创建 Blob/Tree/Commit。
- 缺少 ERROR/DELETING 状态、Trash、Reconciler 和一致性盘点。

## 验收场景

### Scenario 1：创建失败不返回假成功

- **Given** Git Storage 不可写或初始化失败
- **When** 创建 Repo
- **Then** 不返回可用 Repo；记录进入 ERROR/回滚，且可重试或清理

### Scenario 2：删除失败可恢复

- **Given** 磁盘删除失败
- **When** 删除 Repo
- **Then** Repo 进入 DELETING/ERROR 或移动到 Trash，仍可被 Reconciler 定位

### Scenario 3：Seed 是真实 Commit

- **Given** 创建 Repo 时启用 Seed Template
- **When** 用户 clone 默认分支
- **Then** 可以看到 README、`.evolith/policy.yaml`、`.evolith/agents.yaml` 和可验证的 Initial Commit

### Scenario 4：Reconciler 修复分裂

- **Given** 存在 DB-only 或 disk-only Repo
- **When** 运行盘点/Reconciler
- **Then** 输出明确分类并按策略恢复、归档或清理，不静默忽略

## 工程要求

- 引入 Repo lifecycle 状态或等价工作流：`CREATING / ACTIVE / ERROR / DELETING`。
- 封装 `GitStorage`/RepoApplicationService，Handler 不直接编排 DB 与文件系统。
- 创建失败采用事务回滚、补偿或可重试状态；删除优先 Trash/标记后异步物理清理。
- 使用 gix plumbing 或临时工作树创建 Initial Commit 并更新默认 Ref/HEAD。
- 增加一致性盘点命令、Admin API 或 Worker Job。
- 所有失败路径验证 Ref、DB 状态和磁盘目录的实际结果。
- SQLite/PostgreSQL 双轨 migration/repository/test。

## 不做事项

- 不实现跨区域复制、LFS 或对象存储 Git 后端。
- 不在本 Story 实现 Commit/Promote API。
- 不把 Reconciler 扩展成通用工作流引擎。

## 最小验证

- 磁盘初始化失败注入与 DB 状态测试。
- 删除失败/Trash/Reconcile 测试。
- Seed Repo clone 后文件与 Commit 校验。
- DB-only、disk-only、正常 Repo 盘点测试。
- SQLite/PostgreSQL 双轨 + 真实 Git client E2E。

## 解锁内容

解除 DATA-02 Gate；为 Repo UI、Commit/Promote 和生产恢复提供可信生命周期边界。

## 验收结果

- [x] 创建先落 `CREATING`；初始化、Seed 或激活失败返回错误并保留 `ERROR`，不返回假成功。
- [x] 删除先进入 `DELETING`；磁盘删除失败保留 `ERROR` 元数据供 Reconciler 定位。
- [x] Seed 生成真实 Initial Commit、默认 Branch/HEAD、README 和 `.evolith/*`。
- [x] Reconcile dry-run 分类 DB-only / disk-only / consistent；apply 将 DB-only 标记 `ERROR`，disk-only 仅隔离不删除。
- [x] Reconcile 仅允许同租户 Owner/Admin JWT；Member、跨租户 JWT 和 API Key 均被拒绝。
- [x] SQLite/PostgreSQL migration、repository 与历史 009 -> 010 upgrade 行为一致。
- [x] Tenant storage symlink 路径逃逸被拒绝；初始化、Seed 和删除失败均有注入测试。

## 验证证据

- `cargo fmt --all -- --check`：通过。
- `cargo test --workspace`：通过，包含 API、Git client、Repo Context、生命周期与文档测试。
- `cargo clippy --workspace --all-targets -- -D warnings`：通过；仅有上游 `proc-macro-error2` future-incompat 提示。
- `cargo test -p api --test repo_e2e_tests test_reconcile`：2 passed，覆盖 repair、Member 与跨租户 JWT 拒绝。
- PostgreSQL 16 专用测试库执行 `postgres_repo_lifecycle_upgrade_and_repository_round_trip`：通过；历史记录升级为 `ACTIVE`，新记录从 `CREATING` 完成状态 round trip。
- Navigator 阶段复核：`Complete`；未发现阻断项，确认未知 disk-only Git 数据只隔离、权限 fail closed、双数据库证据完整。

## 残余工作

- Durable event、runtime readiness/CI 与最终生产构建分别继续归 EVO-118-H、EVO-118-G、EVO-118-E；不属于本 Story。
- 多实例共享 Git Storage、跨区域复制、LFS 和对象存储未纳入本 Story。
