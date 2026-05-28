# Iteration 029: GitHub CI/CD 重建

> 文档状态：Planned / Blocked for activation
> 计划发布日期：2026-05-28
> 计划目标：用一周完成 `EVO-030` GitHub CI/CD 重建，基于最终 Bun/Vite、Rust 和
> 部署命令建立可维护 workflow。
>
> 基线保护：本文件一旦提交，以下“发布计划基线”内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

- 重建 GitHub Actions CI，覆盖后端 fmt/check/clippy/test 与前端 type-check/build。
- 如部署形态已稳定，再决定是否恢复 deploy workflow；否则仅保留 CI。
- 不再引用 Next.js workflow 或 npm 命令，统一使用 Bun/Vite。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|---------|--------|---------------|
| EVO-030 | GitHub CI/CD 重建 | 无 | P2 | Iteration 028 格式/命令基线完成；部署形态无未决阻塞 |

## 3. 发布计划基线：不做事项

- 不在构建/部署形态仍变化时硬写 deploy 自动化。
- 不恢复 npm/Next.js 旧 workflow。
- 不把业务修复混入 CI rebuild。

## 4. 发布计划基线：计划验收标准

- Story 格式与 BDD 适用性：
  - [x] Technical/Governance Story；BDD 不适用，使用命令级和结构验收。
- [ ] CI workflow 使用 Bun 安装/构建前端，执行 `bun run type-check` 与 `bun run build`。
- [ ] 后端 workflow 执行稳定的 fmt/check/clippy/test 门禁。
- [ ] workflow path、cache、matrix 和 secrets 使用最小必要范围。
- [ ] 如果 deploy workflow 仍不成熟，明确 Deferred 并写入 release/CI reference。

## 5. 发布计划基线：计划验证

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo clippy --workspace -- -D warnings
cargo test --workspace
cd frontend
bun run type-check
bun run build
```

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| CI 固化过渡部署策略 | deploy workflow 必须等 embedded frontend 或部署边界明确后再恢复 |
| 重引入 npm/Next.js 命令 | workflow review 中搜索 `npm`、`next`、`.next` |
| CI 与本地命令漂移 | 同步 `docs/reference/TECH-STACK.md`、`TESTING.md` 或 release reference |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 一周计划：重建 GitHub CI/CD |
| 产物 | `.github/workflows/*`、CI/reference 更新、验证记录 |
| 状态同步归口 | EVO-030、Iteration 029、TECH-STACK/TESTING/Release reference |
| Story/BDD 归口 | Technical/Governance Story；命令级验收 |
| 验证证据 | 本地命令、workflow lint/审查、push 后 GitHub check 结果 |
| 残余工作归口 | deploy 自动化若未恢复，明确 Deferred 条件 |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-05-28 | planning | 发布一周计划基线；依赖 Iteration 028 和部署边界确认。 |

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|

## 10. Review

- 完成：
- 未完成：
- 验证结果：
- 闭环状态：`Blocked`（仅计划发布，前置命令基线/部署边界未完成）
- 残余归口：激活门禁见第 2 节与第 4 节。

## 11. Retrospective

- 做得好的：
- 需要调整的：
- 写入 EVOLUTION：
