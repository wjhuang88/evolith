# Iteration 032: 后端依赖全量版本审计与迁移

> 文档状态：Planned / Ready for activation
> 计划发布日期：2026-06-01
> 计划目标：用一个 Agent 微迭代完成 `EVO-043` 后端 workspace 依赖审计、可升级项迁移和暂缓项记录，降低后续 CI 重建返工风险。
>
> 基线保护：本文件一旦提交，以下“发布计划基线”内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

- 审计 `backend/Cargo.toml` 的 workspace dependencies 和当前 `Cargo.lock`。
- 升级可安全迁移的后端依赖，尤其处理已观察到的 `sqlx-postgres` future-incompat 风险。
- 为暂缓的大版本升级写清原因、风险和后续归口。
- 为 [Iteration 029](ITERATION-029.md) 的 CI 重建提供更稳定的后端依赖基线。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|---------|--------|---------------|
| EVO-043 | 后端依赖全量版本审计与迁移 | 无 | P1 | 无硬依赖；建议在 Iteration 029 前执行 |

## 3. 发布计划基线：不做事项

- 不升级前端依赖；前端依赖另行规划。
- 不升级 Rust edition 或 MSRV。
- 不为迁移依赖而扩大业务重构。
- 不引入 MySQL repository 支持；MySQL 仍只是配置层预留。

## 4. 发布计划基线：计划验收标准

- Story 格式与 BDD 适用性：
  - [x] Technical Story；BDD 不适用，使用依赖清单和命令级验收。
- [ ] 依赖审计表记录当前版本、目标版本、动作和暂缓原因。
- [ ] 可安全升级的依赖已迁移，破坏性升级只做有依据的最小适配。
- [ ] `cargo check --workspace` 通过。
- [ ] `cargo clippy --workspace -- -D warnings` 通过，或失败项被真实记录并拆出后续修复。
- [ ] `cargo test --workspace` 通过，或失败项被真实记录并拆出后续修复。

## 5. 发布计划基线：计划验证

```bash
cargo check --workspace
cargo clippy --workspace -- -D warnings
cargo test --workspace
git diff --check
```

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| 大版本升级引入 API 破坏 | 逐组升级，必要时暂缓并记录，不把大范围适配混入本轮 |
| Cargo.lock 大幅变动难以审查 | 记录升级分组和原因，提交前审查 `git diff --cached` |
| 依赖审计需要网络 | 若本地命令无法查询 registry，记录受限项并只做可验证的本地迁移 |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 计划：后端依赖审计与迁移，不启动实现 |
| 产物 | Cargo 依赖变更、审计记录、暂缓项说明 |
| 状态同步归口 | EVO-043、Iteration 032、必要时 TESTING/TECH-STACK reference |
| Story/BDD 归口 | Technical Story；命令级验收 |
| 验证证据 | check/clippy/test/diff 检查 |
| 残余工作归口 | 大版本暂缓项或失败门禁另建 backlog |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-06-01 | planning | 发布计划基线；未启动实现。库存处置：Iteration 028 可优先激活；本轮可在 028 后、029 前执行。 |

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|

## 10. Review

- 完成：
- 未完成：
- 验证结果：
- 闭环状态：`Blocked`（仅计划发布，尚未激活）
- 残余归口：激活后按第 7 节闭环。

## 11. Retrospective

- 做得好的：
- 需要调整的：
- 写入 EVOLUTION：
