# EVO-052 MySQL 半接线收敛与快速失败

> Archived backlog item. Routing index: [2026 Q2 Archive](INDEX.md). Active routing surface: [Product Backlog](../../PRODUCT-BACKLOG.md).

- Type: tech-debt
- Status: Done
- Priority: P2
- Source: 代码健康审查 2026-06-01 / Iteration 038
- Decision Context: 2026-06-04 完成：config validate + create_pool 对 mysql 快速失败；main.rs 后置 MySql 分支移除；CONFIG/EVOLUTION 同步

#### Source Detail Snapshot

- 类型：tech-debt
- 优先级：P2
- 状态：Ready
- 父 Epic：无
- Story 形态：Technical
- 用户价值或技术目标：
  - 为了：消除"配置接受、连接池建立成功、运行时才拒绝"的误导性接线陷阱。
  - 维护者/运维者需要：当 `DATABASE__DATABASE_TYPE=mysql` 时尽早得到清晰、确定的拒绝，而不是先连上池再 panic 式拒绝。
  - 以便：不会误以为 MySQL 已可用（`infra/src/config.rs`、`db/pool.rs` 均接受 mysql，但 `backend/src/main.rs:156` 才拒绝，无 repository 实现）。
- 范围（本次做）：
  1. 在配置解析期对 `mysql` 做 fail-fast，给出明确"MySQL 未实现，请使用 SQLite/PostgreSQL"的错误。
  2. `db/pool.rs` 对 `mysql` 直接返回 `ConfigError`，不建立连接池；`DatabasePool::MySql` 与 `main.rs` 后置拒绝分支已移除。
  3. 同步 `EVOLUTION.md` 问题速查表与 `docs/reference/CONFIG.md` MySQL 现状说明。
- 不做：
  - 不实现任何 MySQL repository。
  - 不改 SQLite / PostgreSQL 行为。
- 验收标准：
  - [x] 设置 `DATABASE__DATABASE_TYPE=mysql` 启动时，在建立连接池之前即返回明确错误并退出。
  - [x] 代码中不再存在"接受 mysql 但延后到 main.rs 才拒绝"的分裂路径。
  - [x] `EVOLUTION.md` 速查表与 `CONFIG.md` 记录 MySQL 现状与处置。
  - [x] `cargo check --workspace` 通过；局部测试覆盖 config / pool 快速失败。
- 依赖或阻塞：无。
- 解锁内容：减少部署期对数据库支持范围的误判。
- 影响范围：backend / docs
- 最小验证方式：`cargo test -p infra config`；`cargo test -p infra mysql`；`cargo check --workspace`。
