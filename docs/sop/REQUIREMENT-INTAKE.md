# SOP: 需求进入与 Backlog 整理

> 本 SOP 负责把想法、缺陷、技术债和提案整理成可执行 backlog。Agent 只能从满足 DoR 的 backlog 条目开工。

## 触发条件

- 用户提出新需求、新想法、缺陷修复或技术债。
- `docs/proposals/` 中的提案准备进入实施。
- 需要拆分、排期或调整 `docs/backlog/PRODUCT-BACKLOG.md`。
- 迭代开始前需要确认候选故事是否 Ready。

## 输入分流

| 输入类型 | 放置位置 | 处理方式 |
|----------|----------|----------|
| 远期想法、方向探索、范围不清 | `docs/proposals/` | 写成提案或补充现有提案；Agent 不直接开工 |
| 可在 0.5-2 天内完成的功能/缺陷/技术债 | `docs/backlog/PRODUCT-BACKLOG.md` | 补齐必填字段，满足 DoR 后才可排期 |
| 阻塞运行、安全或数据损坏问题 | `docs/backlog/PRODUCT-BACKLOG.md` | 标记 P0，可插队；事后补齐记录 |
| 重大技术或产品取舍 | `docs/decisions/` | 先写 ADR，再把可执行部分拆入 backlog |

## Backlog 必填字段

新增 backlog 条目最少包含：

- ID
- 标题
- 用户价值或技术目标
- 类型：feature / bug / chore / tech-debt / spike
- 优先级：P0 / P1 / P2 / P3
- 状态
- 验收标准
- 依赖或阻塞
- 影响范围：backend / frontend / db / docs / deploy
- 最小验证方式

## Proposal 晋升

Evolith 的执行者包括 AI Agent。Agent 从 Backlog 选取任务时会直接开工，无法自行判断需求是否足够成熟。因此采用两级结构：

- `docs/proposals/`：想法暂存区。Agent 不从中选取任务。
- `docs/backlog/`：可执行指令源。所有条目必须满足 DoR。

远期想法满足以下条件后才能迁入 Backlog：

1. 有明确的用户价值或技术目标。
2. 范围足够小，0.5-2 天可完成；过大则先拆分。
3. 有至少一条可验证的验收标准。
4. 依赖已识别。
5. 需要改哪些层已明确：backend / frontend / db / docs / deploy。

晋升操作：

1. 在 Backlog 新增条目。
2. 在 Proposal 标注 `已晋升`，或保留为背景参考。
3. 如果提案被 ADR 替代，补充 ADR 链接和替代关系。
4. 更新相关 roadmap 的归口映射。

## Backlog Refinement

每次开始迭代前检查：

- 故事是否足够小。
- 验收标准是否可验证。
- 是否需要 API 合约、数据库 migration、前端页面或部署变更。
- 是否需要 spike 先验证技术路线。
- 是否有明确测试策略。
- 是否与已有 ADR、roadmap、proposal 冲突。

满足 Definition of Ready 后，才允许进入迭代。

## Definition of Ready

故事进入迭代前必须满足：

- [ ] 有清晰用户价值或技术目标。
- [ ] 有验收标准。
- [ ] 依赖明确。
- [ ] 能在 0.5-2 天内完成，或已拆分。
- [ ] 知道需要改哪些层：backend / frontend / db / docs / deploy。
- [ ] 有最小验证方式。
- [ ] 如果涉及重大取舍，已有 ADR 或明确说明不需要 ADR。

## 状态定义

| 状态 | 含义 |
|------|------|
| Proposed | 新进入，尚未整理 |
| Ready | 已满足 DoR，可排期 |
| In Progress | 正在实现 |
| Review | 已实现，等待验证或审查 |
| Done | 验收完成 |
| Blocked | 依赖未解决 |
| Deferred | 暂缓 |
| Dropped | 明确放弃 |

## 标准记录模板

```markdown
### EVO-XXX <标题>

- 类型：feature / bug / chore / tech-debt / spike
- 优先级：P0 / P1 / P2 / P3
- 状态：Proposed / Ready / In Progress / Review / Done / Blocked / Deferred / Dropped
- 用户价值或技术目标：
- 验收标准：
  - [ ] 
- 依赖或阻塞：
- 影响范围：backend / frontend / db / docs / deploy
- 最小验证方式：
```

## 失败处理

- 需求范围不清：放入 `docs/proposals/`，不要伪装成 Ready backlog。
- 条目过大：拆成多个 story，或先建 spike。
- 验收标准不可验证：保持 `Proposed`，补清楚后再排期。
- 发现与 ADR 冲突：先更新 ADR 或补替代关系，再调整 backlog。

## 相关文档

- [Product Backlog](../backlog/PRODUCT-BACKLOG.md)
- [提案目录](../proposals/README.md)
- [决策记录](../decisions/README.md)
- [特性迭代工作流](ITERATION-WORKFLOW.md)
