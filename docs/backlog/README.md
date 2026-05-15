# Backlog 目录

本目录是 Evolith 的可执行需求池。Agent 从这里选取任务并直接开工，因此所有条目必须满足 [Definition of Ready](../sop/ITERATION-WORKFLOW.md#definition-of-ready)。

## 与 Proposals 的关系

`docs/proposals/` 是想法暂存区，Agent 不从中选取任务。提案经过人工确认和细化，满足晋升条件后才能进入 Backlog。详见 [Proposals 晋升流程](../proposals/README.md#晋升流程proposal--backlog)。

## 文件

- [Product Backlog](PRODUCT-BACKLOG.md) — 当前统一需求池。

## 规则

1. 新功能、缺陷、技术债先进入 backlog。
2. 远期想法先放 `docs/proposals/`，满足晋升条件后再进入 backlog。
3. 每个 backlog item 必须有 ID、优先级、状态和验收标准。
4. 进入迭代前必须满足 [Definition of Ready](../sop/ITERATION-WORKFLOW.md#definition-of-ready)。
5. 完成后必须满足 [Definition of Done](../sop/ITERATION-WORKFLOW.md#definition-of-done)。
6. 迭代中需求变更必须按 [迭代中需求变更](../sop/ITERATION-WORKFLOW.md#5-迭代中需求变更) 处理，不允许直接覆盖原故事。

## 防呆字段

新增或变更 backlog item 时，至少写清楚：

- `类型`：feature / bug / chore / tech-debt / spike / product-change。
- `状态`：只能使用 SOP 中定义的状态。
- `取代关系`：如果替代旧故事，在备注中写 “replaces EVO-XXX” 或 “supersedes <旧概念>”。
- `不做`：明确本故事不处理的范围。
- `验收标准`：至少 1 条可验证检查项。
