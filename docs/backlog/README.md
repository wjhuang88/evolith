# Backlog 目录

本目录是 Evolith 的可执行需求池。Agent 从这里选取任务并直接开工，因此所有条目必须满足 [Definition of Ready](../sop/REQUIREMENT-INTAKE.md#definition-of-ready)。

## 结构

- [Product Backlog](PRODUCT-BACKLOG.md) — 决策入口，只保留优先级、状态、决策上下文和 Required Reads。
- [Active item files](active/) — Proposed / Ready / Blocked 等仍可能执行或影响排期的条目详情。
- [Archive](archive/) — Done / Deferred / Dropped 等非活跃历史。

## 读取规则

1. 先读 [Product Backlog](PRODUCT-BACKLOG.md)。
2. 找到目标行后，读取该行 `Required Reads` 中列出的所有路径。
3. 只有当 Required Reads 指向 archive、条目声明依赖/取代历史，或用户询问历史原因时，才读取 archive。

## 与 Proposals 的关系

`docs/proposals/` 是想法暂存区，Agent 不从中选取任务。提案经过人工确认和细化，满足晋升条件后才能进入 Backlog。详见 [Proposals 晋升流程](../proposals/README.md#晋升流程proposal--backlog)。

## 规则

1. 新功能、缺陷、技术债先进入 backlog。
2. 远期想法先放 `docs/proposals/`，满足晋升条件后再进入 backlog。
3. 主 backlog 不承载长验收、讨论或执行日志；这些内容放入 item file 或 archive。
4. 每个活跃/阻塞 item 必须有 item file，写清目标、不做、依赖、验收、验证和残余归口。
5. 进入迭代前必须满足 [Definition of Ready](../sop/REQUIREMENT-INTAKE.md#definition-of-ready)。
6. 完成后必须满足 [Definition of Done](../sop/ITERATION-WORKFLOW.md#definition-of-done)。
7. 迭代中需求变更必须按 [迭代中需求变更](../sop/CHANGE-CONTROL.md) 处理，不允许直接覆盖原故事。
8. 多结果、多阶段或超过交付窗口的事项按 [Epic / Story 方法论](../sop/REQUIREMENT-INTAKE.md#epic--story-方法论) 拆分；新父子关系采用 `EVO-NNN` / `EVO-NNN-A`。
