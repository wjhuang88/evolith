# SOP: 特性迭代工作流

> Evolith 使用轻量敏捷 + XP 实践推进功能：需求先进 backlog，迭代小步交付，代码以测试和可运行验证闭环。

## 触发条件

- 已有 Ready backlog 需要进入开发。
- 需要从当前 backlog 中选择下一批任务并形成迭代计划。
- 需要结束一次迭代并沉淀经验。

开始一次新迭代前先按 [开始一次迭代](START-ITERATION.md) 执行固定步骤。新需求进入、拆分、排期前先按 [需求进入与 Backlog 整理](REQUIREMENT-INTAKE.md) 检查 DoR。开发中收到需求变更时按 [迭代中需求变更](CHANGE-CONTROL.md) 处理。

## 核心原则

1. **Backlog first**：未进入需求池的功能不直接开工，紧急修复除外。
2. **Small batch**：一个故事应能在 0.5-2 天内完成；过大则拆分。
3. **Acceptance first**：每个故事必须有验收标准。
4. **Test first when useful**：缺陷修复和核心业务逻辑优先写失败测试，再实现。
5. **Always integrated**：保持 `main` 可构建、可测试、可部署。
6. **Docs live with code**：代码、测试、API 合约、SOP/Reference 同步更新。
7. **Retrospective writes memory**：迭代中踩坑写入 `EVOLUTION.md`。
8. **Plan baseline is evidence**：已发布的迭代计划用于对照实际执行，不因启动、改线或完成而被覆盖。
9. **Closure before completion**：产物完成不等于故事完成；声明完成前必须按
   [任务收口与完成声明](TASK-CLOSURE.md) 核对状态、证据和残余归口。

## 角色映射

| XP/敏捷角色 | 在本项目中的含义 |
|-------------|------------------|
| Product Owner | 用户或维护者，决定优先级和验收口径 |
| Developer | 人类开发者和 Agent |
| Customer Tests | API 测试、前端回归、手工验收清单 |
| Coach | `AGENTS.md` + SOP 约束 |

## 工作流

### 1. Iteration Planning

开始前先确认选入故事已经满足 [Definition of Ready](REQUIREMENT-INTAKE.md#definition-of-ready)。

创建或更新 `docs/iterations/ITERATION-<N>.md`：

- 迭代目标。
- 选入 Story 列表、所属 Epic（如有）以及依赖顺序。
- 不做什么。
- 验证命令。
- 风险和回滚点。
- 闭环台账：产物、状态同步归口、验证证据和残余工作归口。

默认迭代长度：1 周。对 Agent 工作可以使用更短的“微迭代”：一次会话只完成一个故事或一个清晰任务切片。

已提交的 future iteration 不是可随意复用的空槽位。启动时：

- 同一目标：保留计划段，追加实际选入、执行证据和结果偏差。
- 不同目标：原文档只追加延期/阻塞/被替代说明；为新目标创建新的 iteration 编号。
- 已有后续计划依赖原文档：同步标记后续激活阻塞，直到前置工作重新排期并完成。

### 2. XP 开发循环

对每个故事按以下循环推进：

1. 写下验收标准和测试计划。
2. 如果是缺陷，先补失败测试。
3. 小步实现。
4. 运行局部验证。
5. 重构，保持接口清晰。
6. 运行迭代要求的验证。
7. 更新文档和 backlog 状态。
8. 按 [任务收口与完成声明](TASK-CLOSURE.md) 核对闭环结论，再报告完成或提交。

如果开发中收到需求变更，立即按 [迭代中需求变更](CHANGE-CONTROL.md) 先停手、分类、记录，再继续。

### 3. Review

故事完成时更新：

- backlog 状态：Done
- backlog 详情块：状态、验收标准勾选、最小验证方式和“不做事项”同步到实际完成范围
- 如果 Story 属于 Epic：同步父项子 Story 表；只有父项完成条件满足时才更新 Epic 为 `Done`
- 相关迭代文档：完成情况、验证结果
- API 合约或 Reference：如果行为变化
- `EVOLUTION.md`：如果发现新坑

Review 只能追加实际结果或说明计划偏差，不能反向改写已经发布的目标、范围或风险基线。

验证结果必须逐命令记录实际执行结果。验收标准中的 `[x]` 只能对应已经执行且通过的命令或人工核验；局部测试、历史结果、预期 CI 或“仅已有 warning”不能替代失败的全量门禁。任何必需门禁失败时，story 保持 `Review` 或转入修复 story，不得标为 `Done`。

Review 必须写明 `Complete / Partial / Blocked` 收口结论及残余归口。若实施产物已形成
但状态、验证或已知残余尚未同步，保留为 `Review` 并使用 `Partial`，不要以完成叙述
掩盖收口缺口。

### 4. Retrospective

迭代结束时记录：

- 完成了什么。
- 没完成什么，原因是什么。
- 下轮需要调整的工程实践。
- 是否有需要归档的决策或经验。

## Definition of Done

故事完成必须满足：

- [ ] 代码实现完成。
- [ ] 测试或验证完成，并记录结果。
- [ ] 验收勾选与逐命令验证结果一致；没有把失败、未运行或局部替代结果标为通过。
- [ ] API 合约、Reference、SOP 或 Roadmap 已按需更新。
- [ ] Backlog 总表和详情块状态一致；已完成 story 的验收标准已勾选或说明未完成项。
- [ ] 无无关变更混入。
- [ ] 如果修改脚本行为，更新 `docs/reference/SCRIPTS-RELEASE-NOTES.md`。
- [ ] 如果发现新经验，更新 `EVOLUTION.md`。
- [ ] 如需提交，commit 遵守 [Git 工作流](GIT-WORKFLOW.md)。
- [ ] 涉及认证、权限、公开入口、出站执行、数据库或发布路径时，已有 Navigator 结论及残余风险记录。
- [ ] 已发布计划基线仍可阅读；如实际范围发生变化，已有追加说明或新 iteration 编号，而非就地换目标。
- [ ] 已按 `TASK-CLOSURE.md` 完成闭环台账核对；Review 包含完成声明、验证证据和残余归口。

## WIP 限制

- 单个 Agent 会话最多同时推进 1 个 `In Progress` 故事。
- 一个迭代默认最多 3 个 P0/P1 故事。
- 技术 spike 必须有时间盒，默认不超过半天。

## 相关文档

- [需求进入与 Backlog 整理](REQUIREMENT-INTAKE.md)
- [开始一次迭代](START-ITERATION.md)
- [分阶段结对开发](PAIRING-WORKFLOW.md)
- [迭代中需求变更](CHANGE-CONTROL.md)
- [Product Backlog](../backlog/PRODUCT-BACKLOG.md)
- [迭代目录](../iterations/README.md)
- [实施路线图](../roadmap/IMPLEMENTATION-ROADMAP.md)
- [Git 工作流](GIT-WORKFLOW.md)
- [任务收口与完成声明](TASK-CLOSURE.md)
