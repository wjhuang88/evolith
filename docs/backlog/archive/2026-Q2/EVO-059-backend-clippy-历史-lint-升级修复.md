# EVO-059 Backend clippy 历史 lint 升级修复

> Archived backlog item. Routing index: [2026 Q2 Archive](INDEX.md). Active routing surface: [Product Backlog](../../PRODUCT-BACKLOG.md).

- Type: tech-debt
- Status: Done
- Priority: P2
- Source: Iteration 028 验证残余 / Iteration 029 配套
- Decision Context: 2026-06-01 Iteration 029 收口：21 个 `-D warnings` 错误归零（原估算 18，实际 13× unwrap_used + 3× dead_code + 4× unnecessary_min_or_max + 1× field_reassign_with_default）。修复策略：unwarp_used 在 7 个 test 文件加文件级 `#![allow(clippy::unwrap_used)]`（workspace deny 覆盖 clippy.toml 行为）；dead_code 移除未使用字段而非 `#[allow]`；unnecessary_min_or_max 移除 `.max(3)` 因 MIN_RPM=30 保障 rpm/10>=3；field_reassign_with_default 改 struct update syntax

#### Source Detail Snapshot

- 类型：tech-debt
- 优先级：P2
- 状态：Ready
- 来源：Iteration 028 验证残余 / 2026-06-01
- 用户/工程价值：消除 `cargo clippy --workspace -- -D warnings` 18 个错误，让 release 构建前可重启用 clippy 严格门禁。
- 背景（Iteration 028 rustfmt 基线收口时发现；Iteration 032 重新确认 18 个错误分布）：
  - 10× `unwrap_used`（`crates/infra/tests/{api_key,user,audit,snippet,skill}_repo_tests.rs`），工作区 `[workspace.lints.clippy] unwrap_used = "deny"` 应用到所有 target 含 tests（**注**：项目无 `clippy.toml` allow-unwrap-in-tests 配置）。
  - 3× `dead_code`（`crates/api/tests/auth_e2e_tests.rs:49/55/76`，三个 struct 字段 `message` / `expires_at` / `plan` 定义但未读）。
  - 4× `unnecessary_min_or_max`（`crates/api/src/middleware/rate_limit.rs:111-114`）。
  - 1× `field_reassign_with_default`（`crates/service-payment/src/config.rs:53`）。
  - 跨 8 个文件、18 个 `-D warnings` 错误，全部非业务问题。
- 范围（本次做）：
  1. `infra/tests/*_repo_tests.rs`：评估将 `[workspace.lints.clippy] unwrap_used` 从 `deny` 降为 `warn`；或对必要 `unwrap` 加 `#[expect(clippy::unwrap_used)]` + 解释。
  2. `api/tests/auth_e2e_tests.rs`：对未读取字段加 `#[allow(dead_code)]` 或 `#[expect(dead_code)]` + 注释；或删除未使用字段。
  3. `api/src/middleware/rate_limit.rs:111-114`：用 `.min(...).max(...)` 替换 `min(...).min(...)` 模式或反向比较。
  4. `service-payment/src/config.rs:53`：使用结构体更新语法或显式字段赋值替换。
- 不做：
  - 不改业务逻辑。
  - 不调整工作区 `deny(clippy::panic)` / `deny(clippy::expect_used)` / `deny(clippy::todo)` 等其他 `deny` 级别。
  - 不新增 suppress，不批量 `#[allow(...)]`。
- 验收标准：
  - [ ] `cargo clippy --workspace --all-targets -- -D warnings` 0 错误。
  - [ ] `cargo test --workspace` 仍 0 失败（验证非行为变更）。
  - [ ] 无新增 `#[allow(...)]` / `#[expect(...)]` 除非带注释说明。
  - [ ] `[workspace.lints.clippy]` 配置变化有注释说明。
- 依赖或阻塞：无。
- 解锁内容：Iteration 028 验证残余清零，release 流程可启用 clippy 严格门禁。
- 影响范围：backend（含 tests）
- 最小验证方式：`cargo clippy --workspace --all-targets -- -D warnings`；`cargo test --workspace`。
