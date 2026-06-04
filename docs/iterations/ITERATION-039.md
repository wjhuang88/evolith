# Iteration 039: 后端死代码与误导性注释清理

> 文档状态：Closed（2026-06-04）
> 计划发布日期：2026-06-04
> 计划目标：完成 EVO-051，清除后端误导性注释、字段命名语义修正和未接线死代码删除。
>
> 基线保护：本文件一旦提交，以下"发布计划基线"内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

- 删除误导性 `// TODO: Hash password` 注释（密码已在 handler 层哈希）。
- 将 `NewUser.password` 重命名为 `password_hash`，贯通构造点与 repository 绑定。
- 修正 `db/sqlite.rs` / `db/postgres.rs` 中失效的 "TODO: Implement repositories" 注释。
- 删除未接线的后端死代码模块（guards.rs / registry.rs / reference.rs / search.rs / error.rs）。
- 不改任何运行时行为。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|---------|--------|---------------|
| EVO-051 | 误导性注释、命名与后端死代码清理 | 无 | P2 | 无硬依赖 |

## 3. 发布计划基线：不做事项

- 不动 `db/mysql.rs` 注释（MySQL 确实未实现，注释属实）。
- 不删除 `infra/src/storage.rs`、`service-auth/src/{rbac,session}.rs`、`service-tool/src/discovery.rs` 等"未来功能占位"。
- 不清理前端死代码（归 EVO-058）。
- 不改任何运行时行为。

## 4. 发布计划基线：计划验收标准

- Story 格式与 BDD 适用性：
  - [x] Technical Story；BDD 不适用，使用命令级验收。
- [x] `rg "TODO: Hash password" backend/` 无结果。
- [x] `NewUser.password` 全仓无引用，`password_hash` 字段贯通。
- [x] `db/sqlite.rs` / `db/postgres.rs` 不再出现失效 TODO 描述。
- [x] 死代码模块已删除，`mod` 声明清理干净。
- [x] `rg "dev_secret_key_for_testing_only" backend/` 无结果。
- [x] `cargo test --workspace` 与 `cargo clippy --workspace -- -D warnings` 通过。

## 5. 发布计划基线：计划验证

```bash
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
rg "TODO: Hash password" backend/
rg "NewUser\.password[^_]" backend/
rg "dev_secret_key_for_testing_only" backend/
git diff --check
```

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| password 重命名遗漏引用点 | cargo check 会立即报错，逐一修复 |
| 死代码模块被其他模块间接引用 | cargo check + test 覆盖 |
| 误删未来功能占位 | 严格按 EVO-051 范围列表执行，不扩大 |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 清除后端误导性注释 + password_hash 重命名 + 死代码删除 |
| 产物 | 代码修改（注释/命名/模块删除） |
| 状态同步归口 | EVO-051、Iteration 039、iterations/README.md |
| Story/BDD 归口 | Technical Story；命令级验收 |
| 验证证据 | cargo check/clippy/test + rg 检查 |
| 残余工作归口 | 前端死代码归 EVO-058；MySQL 注释归 EVO-052 |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-06-04 | activation | 状态 → Active / In Progress。前置：Iteration 034 Closed，无 Active/Review 迭代。EVO-051 Ready → In Progress。委托 deep agent 实施。 |
| 2026-06-04 | execution | Deep agent 完成实施：删除 10 个死代码文件 + 6 个 mod 声明清理 + TODO 注释移除 + NewUser.password → password_hash 全量重命名（domain/handler/repo/8 个测试文件）。 |
| 2026-06-04 | closure | 独立验证通过：cargo check 0 errors / clippy 0 errors / cargo test 282 passed / 5 项 rg 清理检查全绿。状态 → Closed。 |

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|

## 10. Review

- 完成：
  - 删除 10 个死代码文件（guards.rs / 3× registry.rs / repository.rs / reference.rs / search.rs / error.rs / sqlite.rs / postgres.rs / mysql.rs）
  - 6 个模块声明文件清理（api/lib.rs / middleware/mod.rs / service-skill/lib.rs / service-tool/lib.rs / service-snippet/lib.rs / infra/db/mod.rs）
  - 移除 `user_repo.rs:53` 的 `// TODO: Hash password` 误导注释
  - `NewUser.password` → `NewUser.password_hash` 全量重命名（domain struct + 2 handler + 2 repo + domain test helper + 8 个测试文件共 21 处）
  - `LoginRequest.password` 保留不变（代表用户明文输入，语义正确）
- 未完成：无
- 验证结果：
  - `cargo check --workspace` → 0 errors ✅
  - `cargo clippy --workspace --all-targets -- -D warnings` → 0 errors ✅
  - `cargo test --workspace` → 282 passed, 0 failed ✅
  - `rg "TODO: Hash password"` → 0 hits ✅
  - `rg "dev_secret_key_for_testing_only"` → 0 hits ✅
  - `rg "pub mod guards"` → 0 hits ✅
  - `rg "pub mod registry" service-skill/ service-tool/` → 0 hits ✅
  - `rg "pub mod sqlite|postgres|mysql" infra/db/mod.rs` → 0 hits ✅
- 闭环状态：`Complete`
- 残余归口：前端死代码归 EVO-058；MySQL 注释归 EVO-052

## 11. Retrospective

- 做得好的：
  - Explore agent 提供了完整的文件清单和引用计数，使 deep agent 能一次性完成所有修改
  - `NewUser.password` → `password_hash` 重命名消除了语义歧义，后续维护者不会再被误导
  - 严格区分了 `NewUser.password_hash`（已哈希）和 `LoginRequest.password`（明文输入），保留了正确的语义
- 需要调整的：
  - 无
- 写入 EVOLUTION：无新陷阱。本次为纯清理，未触发代码变更或流程问题。
