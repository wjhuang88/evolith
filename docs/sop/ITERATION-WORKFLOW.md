# SOP: 特性迭代工作流

> Evolith 使用轻量敏捷 + XP 实践推进功能：需求先进 backlog，迭代小步交付，代码以测试和可运行验证闭环。

## 触发条件

- 新需求、新想法、缺陷修复或技术债需要进入开发。
- 需要从当前 backlog 中选择下一批任务。
- 需要结束一次迭代并沉淀经验。

## 核心原则

1. **Backlog first**：未进入需求池的功能不直接开工，紧急修复除外。
2. **Small batch**：一个故事应能在 0.5-2 天内完成；过大则拆分。
3. **Acceptance first**：每个故事必须有验收标准。
4. **Test first when useful**：缺陷修复和核心业务逻辑优先写失败测试，再实现。
5. **Always integrated**：保持 `main` 可构建、可测试、可部署。
6. **Docs live with code**：代码、测试、API 合约、SOP/Reference 同步更新。
7. **Retrospective writes memory**：迭代中踩坑写入 `EVOLUTION.md`。

## 角色映射

| XP/敏捷角色 | 在本项目中的含义 |
|-------------|------------------|
| Product Owner | 用户或维护者，决定优先级和验收口径 |
| Developer | 人类开发者和 Agent |
| Customer Tests | API 测试、前端回归、手工验收清单 |
| Coach | `AGENTS.md` + SOP 约束 |

## 工作流

### 1. 需求进入 Backlog

新增需求写入 [Product Backlog](../backlog/PRODUCT-BACKLOG.md)，最少包含：

- ID
- 标题
- 用户价值
- 类型：feature / bug / chore / tech-debt / spike
- 优先级：P0 / P1 / P2 / P3
- 验收标准
- 依赖或阻塞

#### Proposals 与 Backlog 的关系

Evolith 的执行者是 AI Agent。Agent 从 Backlog 选取任务时会直接开工，无法自行判断需求是否足够成熟。因此采用两级结构：

- **`docs/proposals/`** — 想法暂存区（holding tank）。Agent 不从中选取任务。
- **`docs/backlog/`** — 可执行指令源。所有条目必须满足 DoR。

远期想法先放入 `docs/proposals/`，满足以下晋升条件后迁入 Backlog：

1. 有明确的用户价值或技术目标。
2. 范围足够小（0.5-2 天可完成），过大则先拆分。
3. 有至少一条可验证的验收标准。
4. 依赖已识别。
5. 需要改哪些层已明确（backend / frontend / db / docs / deploy）。

晋升操作：在 Backlog 新增条目 → 在 Proposals 标注已晋升或移除 → 提案文档保留作为背景参考。

### 2. Backlog Refinement

每次开始迭代前检查：

- 故事是否足够小。
- 验收标准是否可验证。
- 是否需要 API 合约、数据库 migration、前端页面或部署变更。
- 是否需要 spike 先验证技术路线。
- 是否有明确测试策略。

满足 Definition of Ready 后，才允许进入迭代。

### 3. Iteration Planning

创建或更新 `docs/iterations/ITERATION-<N>.md`：

- 迭代目标。
- 选入故事列表。
- 不做什么。
- 验证命令。
- 风险和回滚点。

默认迭代长度：1 周。对 Agent 工作可以使用更短的“微迭代”：一次会话只完成一个故事或一个清晰任务切片。

### 4. XP 开发循环

对每个故事按以下循环推进：

1. 写下验收标准和测试计划。
2. 如果是缺陷，先补失败测试。
3. 小步实现。
4. 运行局部验证。
5. 重构，保持接口清晰。
6. 运行迭代要求的验证。
7. 更新文档和 backlog 状态。

### 5. 迭代中需求变更

开发中收到需求变更时，先暂停扩大实现范围，按以下顺序处理。不要凭直觉把新需求直接混进当前代码。

1. 记录变更来源、时间和一句话描述到当前迭代 `执行记录`。
2. 判断变更类型：clarification / scope-change / product-pivot / urgent-fix。
3. 如果只是澄清且不改变验收标准，可直接更新当前故事。
4. 如果改变验收标准、领域概念或优先级，必须新增或更新 backlog item。
5. 如果属于重大产品或技术取舍，补 ADR。
6. 重新检查当前迭代 DoR/DoD，明确：
   - 继续完成当前故事；
   - 缩小当前故事范围；
   - 暂停当前故事并切换到新故事。
7. 已产生的半成品代码只保留仍服务于新范围的部分，其余作为后续清理任务或在当前提交前移除。

#### 防呆规则

后续 Agent 必须按表执行，不允许跳步：

| 输入信号 | 变更类型 | 必做文档动作 | 当前代码动作 |
|----------|----------|--------------|--------------|
| 用户只是解释字段、文案、边界，没有改变验收标准 | clarification | 更新当前 iteration 执行记录；必要时改验收标准文字 | 继续当前故事 |
| 用户新增/删除一个小范围要求，但领域概念不变 | scope-change | 更新当前 backlog item 和 iteration 验收标准 | 只改仍属于当前故事的代码 |
| 用户改变产品概念、命名、主线能力或长期方向 | product-pivot | 新增 ADR；新增 backlog item；旧故事改 Deferred/缩小范围；更新 roadmap | 暂停旧方向代码，保留通用修复，移除或记录不再适用的半成品 |
| 用户要求修复阻塞运行/安全/数据损坏问题 | urgent-fix | 新增或标记紧急 backlog item；iteration 记录插队原因 | 允许插队，但完成后恢复或重排原故事 |

#### 变更处理检查表

- [ ] 已在当前 iteration `执行记录` 增加 change request 行。
- [ ] 已选定唯一变更类型。
- [ ] 如果是 `product-pivot`，已新增 ADR。
- [ ] 已新增或更新 backlog item。
- [ ] 已明确当前故事继续、缩小、暂停还是切换。
- [ ] 已列出半成品代码处理方式：保留 / 移除 / 后续清理。
- [ ] 已更新验证计划，避免验证已经废弃的范围。

变更记录模板：

```markdown
| YYYY-MM-DD | Change request: <一句话描述>。决策：<接受/拆分/暂缓>；影响：<范围变化>。 |
```

半成品处理模板：

```markdown
| YYYY-MM-DD | Work-in-progress handling: <文件/模块> -> <保留/移除/后续清理>；原因：<一句话>。 |
```

### 6. Review

故事完成时更新：

- backlog 状态：Done
- 相关迭代文档：完成情况、验证结果
- API 合约或 Reference：如果行为变化
- `EVOLUTION.md`：如果发现新坑

### 7. Retrospective

迭代结束时记录：

- 完成了什么。
- 没完成什么，原因是什么。
- 下轮需要调整的工程实践。
- 是否有需要归档的决策或经验。

## Definition of Ready

故事进入迭代前必须满足：

- [ ] 有清晰用户价值或技术目标。
- [ ] 有验收标准。
- [ ] 依赖明确。
- [ ] 能在 0.5-2 天内完成，或已拆分。
- [ ] 知道需要改哪些层：backend / frontend / db / docs / deploy。
- [ ] 有最小验证方式。

## Definition of Done

故事完成必须满足：

- [ ] 代码实现完成。
- [ ] 测试或验证完成，并记录结果。
- [ ] API 合约、Reference、SOP 或 Roadmap 已按需更新。
- [ ] 无无关变更混入。
- [ ] 如果修改脚本行为，更新 `docs/reference/SCRIPTS-RELEASE-NOTES.md`。
- [ ] 如果发现新经验，更新 `EVOLUTION.md`。
- [ ] 如需提交，commit 遵守 [Git 工作流](GIT-WORKFLOW.md)。

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

## WIP 限制

- 单个 Agent 会话最多同时推进 1 个 `In Progress` 故事。
- 一个迭代默认最多 3 个 P0/P1 故事。
- 技术 spike 必须有时间盒，默认不超过半天。

## 相关文档

- [Product Backlog](../backlog/PRODUCT-BACKLOG.md)
- [迭代目录](../iterations/README.md)
- [开发计划](../roadmap/DEVELOPMENT-PLAN.md)
- [Git 工作流](GIT-WORKFLOW.md)
