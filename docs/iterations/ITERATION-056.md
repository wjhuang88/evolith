# Iteration 056: PR/Main Automatic Quality Gates

> 文档状态：Closed / Partial
> 计划发布日期：2026-08-08
> 计划目标：完成 EVO-118-G-A，让 PR 与 main push 自动执行完整前后端质量门禁。
>
> 基线保护：本文件一旦提交，以下“发布计划基线”内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

补齐 `push main` 与 PR Frontend lint hard gate，同时保留 exact-head、Markdown、Backend、
Frontend 和 DATA-01 恢复矩阵，使 CI 配置能够作为 main 合并门禁的真实基础。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
| --- | --- | --- | --- | --- |
| EVO-118-G-A | PR/Main 自动质量门禁 | EVO-118-G | P1 | EVO-118-B Done；Iteration 055 Closed |

## 3. 发布计划基线：不做事项

- 不修改 runtime readiness、限流、SMTP/Redis；分别由 G-B/C/D 后续迭代承载。
- 不执行最终 production Compose build/startup 或发布 Smoke；归 EVO-118-E。
- 不削弱 DATA-01 既有恢复矩阵。

## 4. 发布计划基线：计划验收标准

- Story 形态：Technical / CI；BDD 不适用，以 workflow 结构、实际命令和远端规则证据验收。
- [ ] `pull_request -> main`、`push -> main`、semver Tag 与 manual trigger 同时存在。
- [ ] PR/main 执行 Frontend type-check/build/lint 和 Backend fmt/check/clippy/test。
- [ ] exact-head、Markdown 与 DATA-01 现有门禁保持。
- [ ] 当前工作树实际执行前后端门禁通过。
- [ ] 查询并记录 Branch Protection required check；远端未配置时作为明确 residual，不虚报完成。

## 5. 发布计划基线：计划验证

```bash
cd frontend
bun run type-check
bun run build
bun run lint

cd ../backend
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace

cd ..
git diff --check
python3 scripts/tests/check-markdown-links.py
```

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
| --- | --- |
| Frontend lint 当前不稳定 | 先实际复现并修复本 Story 相关或既有阻断；不得用 conditional 绕过 PR |
| main push 与 Tag trigger 冲突 | 同一 `push` 下同时声明 branches/tags，并验证 workflow 结构 |
| 误删耐久性门禁 | 仅修改 trigger/lint 条件，diff 审查确认现有步骤完整保留 |
| Branch Protection 无远端权限 | 记录查询结果与 residual；不把 workflow 文件存在等同规则已启用 |

## 7. 闭环台账

| 项目 | 本轮记录 |
| --- | --- |
| 请求结果 | 朝“完成所有已规划迭代”推进，完成 PR/main 自动质量门禁子 Story |
| 产物 | CI workflow trigger/lint 接线、结构/命令证据、远端 required-check 证据 |
| 状态同步归口 | EVO-118-G-A、父 EVO-118-G、Product Backlog、Iteration 056、Board、Iteration Index |
| Story/BDD 归口 | Technical Story；BDD 不适用，使用命令级与远端状态验收 |
| 验证证据 | Frontend/Backend 全量门禁、workflow 结构、GitHub Branch Protection 查询、治理检查 |
| 残余工作归口 | Runtime readiness -> G-B；限流 -> G-C；SMTP/Redis -> G-D；最终 production smoke -> E |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
| --- | --- | --- |
| 2026-08-08 | activation | Iteration 055 已关闭；018/019/020/027 继续 Superseded/Blocked，025/026 继续按原阻塞处置且不影响当前主线。EVO-118-G 拆为 G-A/B/C/D，激活 G-A。 |
| 2026-08-08 | verification | Frontend lint/type-check/build、Backend fmt/check/clippy/test、workflow YAML 解析通过；Branch Protection API 返回 403，required-check 远端配置无法核验。 |

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
| --- | --- | --- | --- | --- |
| 2026-08-08 | clarification | 拆分 | G 从过大 Story 变为父 Epic，四个独立子 Story 分批验收；目标和 REL-01 边界不变 | 现有 CI/readiness 实现作为当前事实保留并逐项复验 |

## 10. Review

- 完成：workflow 接线与本地质量门禁。
- 未完成：远端 required check 配置证据（外部权限 residual）。
- 验证结果：本地前后端门禁及 YAML 解析通过。
- 闭环状态：`Partial`
- 残余归口：EVO-118-G-A；其他 G 子项按父项表执行。

## 11. Retrospective

- 待迭代完成后填写。
