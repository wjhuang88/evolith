# Iteration 028: Rustfmt 基线与 CI 命令准备

> 文档状态：Planned / Ready for activation
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
- [ ] `cargo fmt --all -- --check` 在 stable toolchain 下通过。
- [ ] rustfmt 配置不再产生 nightly-only warning，或 warning 有明确归口。
- [ ] `cargo check --workspace` 与 `cargo clippy --workspace -- -D warnings` 结果真实记录。
- [ ] 变更只包含格式/配置基线，不混入功能行为。

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
| 请求结果 | 一周计划：建立 Rust 格式和 CI 命令前置基线 |
| 产物 | rustfmt 配置/格式基线、验证记录 |
| 状态同步归口 | EVO-033、Iteration 028、testing/reference（如命令口径变化） |
| Story/BDD 归口 | Technical Story；命令级验收 |
| 验证证据 | fmt/check/clippy/test/diff 检查 |
| 残余工作归口 | CI workflow 进入 Iteration 029；无关失败另建 backlog |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-05-28 | planning | 发布一周计划基线；未启动实现。 |

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
