# Iteration 028: Rustfmt 基线与 CI 命令准备

> 文档状态：Closed（2026-06-01 收口）
> 计划发布日期：2026-05-28
> 计划目标：用一周处理 `EVO-033` 历史格式基线和 stable rustfmt 配置问题，为
> `EVO-030` GitHub CI/CD 重建提供稳定命令前置。
>
> 基线保护：本文件一旦提交，以下“发布计划基线”内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

- 清理历史 rustfmt 差异和 nightly-only 配置告警，形成可放入 CI 的格式门禁。
- 不做功能改动，只处理工程质量基线。
- 为 Iteration 029 / EVO-030 提供确定的 fmt/check/clippy/test 命令。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|---------|--------|---------------|
| EVO-033 | Rustfmt 全量格式基线与 stable 配置清理 | 无 | P2 | 无业务依赖；激活前确认不混入功能改动 |

## 3. 发布计划基线：不做事项

- 不修业务逻辑、不重构模块、不处理 clippy 新功能问题。
- 不创建 GitHub workflow；归 Iteration 029。
- 不把格式化造成的海量 diff 混入其他 story。

## 4. 发布计划基线：计划验收标准

- Story 格式与 BDD 适用性：
  - [x] Technical Story；BDD 不适用，使用命令级验收。
- [x] `cargo fmt --all -- --check` 在 stable toolchain 下通过。
- [x] rustfmt 配置不再产生 nightly-only warning，或 warning 有明确归口。
- [x] `cargo check --workspace` 与 `cargo clippy --workspace -- -D warnings` 结果真实记录。
- [x] 变更只包含格式/配置基线，不混入功能行为。

## 5. 发布计划基线：计划验证

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo clippy --workspace -- -D warnings
cargo test --workspace
git diff --check
```

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| 格式化 diff 过大掩盖功能改动 | 单独提交，只允许格式和 rustfmt 配置变更 |
| clippy/test 暴露无关历史问题 | 不虚报通过；必要时拆出独立修复 story |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 建立 stable rustfmt 与格式基线，供 Iteration 029 CI 重建使用 |
| 产物 | `backend/rustfmt.toml`（移除 7 个 nightly-only 选项）+ 34 个 `.rs` 文件格式重写 + 验证记录 |
| 状态同步归口 | EVO-033（Proposed → In Progress → Done）、Iteration 028（Planned → Active → Closed）、`docs/reference/TESTING.md`（如命令口径变化） |
| Story/BDD 归口 | Technical Story；命令级验收；BDD 不适用 |
| 验证证据 | `cargo fmt --all -- --check`、`cargo check --workspace`、`cargo clippy --workspace -- -D warnings`、`cargo test --workspace`、`git diff --check` 全部真实结果 |
| 残余工作归口 | 任何 check/clippy/test 历史失败将作为新 backlog 登记；CI workflow 归 Iteration 029 |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-05-28 | planning | 发布一周计划基线；未启动实现。 |
| 2026-06-01 | activation | Iteration 028 按 SOP 激活：iteration inventory 完整盘点，无 Active/In Progress/Review 需处置；backlog 中 EVO-033 仍 Proposed；用户明确选择"按推荐顺序启动 028"；EVO-033 同步到 In Progress。 |
| 2026-06-01 | baseline | `cargo fmt --all -- --check` 退出码 1；34 个唯一文件存在格式 diff；161 个 nightly-only warning（7 个选项：wrap_comments / comment_width / normalize_comments / fn_single_line / where_single_line / imports_granularity / group_imports）。stable rustfmt 1.9.0 静默忽略这 7 个选项。 |
| 2026-06-01 | fix | `backend/rustfmt.toml` 移除 7 个 nightly-only 选项，保留 stable-only 选项（`max_width` / `hard_tabs` / `tab_spaces` / `newline_style` / `use_small_heuristics` / `reorder_imports` / `reorder_modules` / `remove_nested_parens` / `edition`）。|
| 2026-06-01 | fix | 应用 `cargo fmt --all`：13 个 .rs 文件格式变更（不混入逻辑），净 -2 行（98 insertions / 100 deletions）。git diff --check 通过。 |
| 2026-06-01 | validation | 最终验证：`cargo fmt --all -- --check` exit 0 ✅；`cargo check --workspace` exit 0 ✅（仅 sqlx-postgres 0.7.4 future-incompat warning，归 EVO-043）；`cargo test --workspace` exit 0 ✅（274 tests passed, 0 failed）。 |
| 2026-06-01 | validation | `cargo clippy --workspace --all-targets -- -D warnings` exit 101 ❌；18 个 pre-existing errors，跨 8 个文件：<br>• 10× `unwrap_used`（`infra/tests/*_repo_tests.rs` × 6 个文件，工作区 `deny` 覆盖了 `clippy.toml` `allow-unwrap-in-tests`）<br>• 3× `dead_code`（`api/tests/auth_e2e_tests.rs`：`ErrorInfo.message` / `AuthResponseData.expires_at` / `TenantInfo.plan`）<br>• 4× `unnecessary_min_or_max`（`api/src/middleware/rate_limit.rs:111-114`：`X.max(3)` 始终 >= 3）<br>• 1× `field_reassign_with_default`（`service-payment/src/config.rs:53`）<br>全部为历史 clippy/rustc 升级遗留，非本迭代范围，已登记为新 backlog **EVO-059**。 |

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|

## 10. Review

- 完成：
  - `backend/rustfmt.toml` 移除 7 个 nightly-only 配置，保留 9 个 stable 选项；不再产生 nightly warning。
  - 应用 `cargo fmt --all` 重写 13 个 `.rs` 文件，diff 仅限格式（行拆分、链式布局、参数重排），无逻辑改动。
  - 同步 EVO-033 backlog 状态：Proposed → In Progress → Done。
  - 同步 `docs/iterations/README.md` 库存盘点（028 从 Ready 移入 Closed，030-1 后 iteration 仍 Ready）。
- 未完成：无
- 验证结果（按 SOP 验证矩阵执行，真实结果）：
  - `cargo fmt --all -- --check` → exit 0（修复前为 1）
  - `cargo check --workspace` → exit 0（仅 1 个 sqlx-postgres 0.7.4 future-incompat warning）
  - `cargo test --workspace` → exit 0（274 tests passed, 0 failed）
  - `cargo clippy --workspace --all-targets -- -D warnings` → exit 101（**18 个 pre-existing errors**，归 EVO-059）
  - `git diff --check` → exit 0
  - Markdown 链接校验 → 0 missing
- 闭环状态：`Complete`
- 残余归口：
  - **18 个 pre-existing clippy errors**（10× unwrap_used in tests, 3× dead_code, 4× unnecessary_min_or_max, 1× field_reassign_with_default）→ 新 backlog **EVO-059**（P2 Ready）。
  - sqlx-postgres 0.7.4 future-incompat warning → 既有 backlog **EVO-043**（依赖审计与迁移）。
  - CI workflow 实施 → 既定归口 Iteration 029 / EVO-030。

## 11. Retrospective

- 做得好的：
  - 严格执行 [开始一次迭代 SOP](../sop/START-ITERATION.md) 的 inventory 盘点，先确认无在途 iteration 才激活 028。
  - 严格区分本迭代范围（fmt baseline）与历史问题（clippy pre-existing error），不虚报通过、不混入功能改动。
  - 使用 `cargo fmt --all` 单次原子应用，避免按文件手改。
  - 验证矩阵与 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 对齐，每个命令退出码都真实记录。
  - 残余工作明确归口到新 EVO-059 与既有 EVO-043 / Iteration 029。
- 需要调整的：
  - 用户在排程阶段可能希望同时看到"推荐 next 5 + 新进 Ready"的完整视图；当前 README 优先展示推荐顺序，新进 Ready 需要靠 SOP 提示。考虑在 iteration README 末尾追加"近期新增 Ready"小节。
- 写入 EVOLUTION：rustfmt 历史配置在 stable toolchain 升级后被静默忽略为 nightly-only 警告的陷阱；执行 fmt baseline 时必须先检查 rustfmt.toml 是否有被 stable 拒绝的选项，再决定"移除"或"切换到 nightly toolchain"。
