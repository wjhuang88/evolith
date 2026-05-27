# Iteration 016: 迭代启动前库存盘点与既有计划优先规则

> 文档状态：Closed
> 计划发布日期：2026-05-27
> 计划目标：修复“开始迭代时直接从 backlog 选故事、未先识别已有未完成或已规划
> iteration”的流程缺口。
>
> 基线保护：本文件一旦提交，以下“发布计划基线”内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

本轮建立 iteration inventory gate：用户提出开始迭代时，Agent 应先盘点仍处于
推进、Review、计划或阻塞状态的 iteration，记录如何继续或处理后，才可进入 backlog
选择新的工作。当前已观察到 Iteration 010 与 backlog 完成状态不一致，作为真实缺陷
一并修复追踪口径。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 状态 | 启动条件/依赖 |
|----|------|---------|--------|------|---------------|
| EVO-039 | 迭代启动前库存盘点与既有计划优先规则 | 无 | P1 | Done | Iteration 010 / 012 状态审计完成 |

## 3. 发布计划基线：不做事项

- 不核验或补实现 Iteration 010 中仍未证明完成的验收项。
- 不激活 Iteration 012 或重新决定 EVO-016 技术范围。
- 不将外部 skill 仓库的独立修改纳入 Evolith 仓库提交。

## 4. 发布计划基线：计划验收标准

- [x] 开始迭代 SOP 把 iteration inventory disposition 置于 backlog selection 之前。
- [x] 相关入口、目录说明与一致性检查可以识别跳过未完成/已规划迭代的问题。
- [x] Iteration 010 状态修复为待收口 Review，未将缺证据项目虚报为 Done。
- [x] 治理 skill 体现同类规则并通过结构验证。
- [x] 项目 Markdown 链接与 `git diff --check` 通过。

## 5. 发布计划基线：计划验证

```bash
# 检查项目文档相对链接
git diff --check
# 验证 agent-project-governance skill 结构
```

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| 将 Blocked 计划误解为永久禁止其他工作 | 要求先记录 disposition；允许明确保留阻塞后再排新工作 |
| 为修复状态而虚构 Iteration 010 验收证据 | 转入 Review，保留待核验项目作为残余 |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 开始迭代时先盘点既有 iteration，再决定是否读取 backlog 新选 story |
| 产物 | 启动/迭代/检查/入口规则；Iteration 010 状态修复；治理 skill 增量规则 |
| 状态同步归口 | EVO-039、Iteration 016、Iteration 010、Iteration 012、`EVOLUTION.md` |
| 验证证据 | 项目 Markdown 链接检查；`git diff --check`；skill `quick_validate.py` |
| 残余工作归口 | Iteration 010 待核验项保留在其 Review；Iteration 012 继续保持 Blocked |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-05-27 | activation | 用户指出开始迭代时应先检查在途或已规划但未完成的迭代，而非直接查 backlog 剩余故事。 |
| 2026-05-27 | inventory | 审计发现 Iteration 010 标为 `In Progress` 但 EVO-006/EVO-009 已 Done 且仍有未勾选验收项；Iteration 012 为 `Planned, Blocked for activation`。 |
| 2026-05-27 | implementation | 在 `AGENTS.md`、启动/迭代/文档检查/收口流程与目录说明中加入 inventory-before-backlog 门禁；Iteration 010 转为 `Review`，Iteration 012 保持 `Blocked`。 |
| 2026-05-27 | skill | 将非终态 iteration 盘点、处置和状态漂移审计规则同步到 `agent-project-governance` skill；skill 工作区保持独立，不纳入 Evolith 提交。 |
| 2026-05-27 | validation | 项目 Markdown 相对链接检查与 `git diff --check` 通过；skill `quick_validate.py` 与 `git diff --check` 通过。 |

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|

## 10. Review

- 完成：iteration inventory gate 已落地；Iteration 010 / 012 的既有未闭环状态已在
  启动路径中可见；skill 同步完成。
- 未完成：不属于本轮范围的 Iteration 010 未核验验收项和 Iteration 012 阻塞未解除。
- 验证结果：项目 Markdown 相对链接检查、项目 `git diff --check`、skill
  `quick_validate.py` 与 skill `git diff --check` 均通过。
- 闭环状态：`Complete`。
- 残余归口：Iteration 010 维持 `Review` 待证据核对；Iteration 012 维持
  `Planned / Blocked` 待 EVO-016 refinement 重新排期；外部 skill 修改保持独立。

## 11. Retrospective

- 做得好的：从用户指出的启动顺序缺口追溯到了真实的 iteration 状态漂移，并以
  disposition 门禁而不是一次性修正文案收口。
- 需要调整的：后续开始迭代时先执行库存审计，避免将 backlog 余量误作当前承诺状态。
- 写入 EVOLUTION：已新增“开始迭代必须先盘点既有 iteration”经验条目。
