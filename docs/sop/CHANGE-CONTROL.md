# SOP: 迭代中需求变更

> 开发中收到需求变更时，先停手、分类、记录，再继续。不要凭直觉把新需求直接混进当前代码。

## 触发条件

- 当前故事已经进入 `In Progress`，用户提出新的需求、约束或方向变化。
- 需求描述改变了验收标准、领域概念、优先级或交付范围。
- 实现过程中发现原故事无法按原计划完成，需要缩小、暂停或切换。
- 已发布的未来 iteration 在启动前需要改选另一组 story 或改变目标。

## 非触发条件

以下情况不走本 SOP，改走 [需求进入与 Backlog 整理](REQUIREMENT-INTAKE.md)：

- 用户提出未来要做的新能力，但当前 `In Progress` story 的验收标准不变。
- 用户指出页面或文档中的残留问题，结论是“后续单独处理”，不改变当前实现切片。
- 用户补充远期目标、候选方案或参考资料，只需要进入 proposal/backlog。

防呆判断：先问一句“这个输入是否要求我改变当前 story 的验收标准或正在写的代码？”如果答案是否，记录到 backlog/proposal，不写入当前 iteration 的 change request 表。

> 对尚未启动但已提交的 iteration，目标调整不是“编辑草稿”：该文件已经是发布计划
> 基线。换目标时只能追加改线结论，并为新工作创建新 iteration 编号。

## 先停手规则

收到变更后立即执行：

1. 暂停扩大实现范围，只允许读取上下文和整理记录。
2. 在当前 iteration `执行记录` 增加 change request 行。
3. 选定唯一变更类型。
4. 完成 backlog / ADR / iteration 的记录后，再继续代码或文档改动。

如果判断为“非触发条件”，仍应暂停扩大实现范围，先把新需求按 `REQUIREMENT-INTAKE.md` 记录清楚，再回到当前 story。

## 变更分类

| 输入信号 | 变更类型 | 必做文档动作 | 当前代码动作 |
|----------|----------|--------------|--------------|
| 用户只是解释字段、文案、边界，没有改变验收标准 | clarification | 更新当前 iteration 执行记录；必要时改验收标准文字 | 继续当前故事 |
| 用户新增/删除一个小范围要求，但领域概念不变 | scope-change | 更新当前 backlog item 和 iteration 验收标准 | 只改仍属于当前故事的代码 |
| 用户改变产品概念、命名、主线能力或长期方向 | product-pivot | 新增 ADR；新增 backlog item；旧故事改 Deferred/缩小范围；更新 roadmap | 暂停旧方向代码，保留通用修复，移除或记录不再适用的半成品 |
| 用户要求修复阻塞运行/安全/数据损坏问题 | urgent-fix | 新增或标记紧急 backlog item；iteration 记录插队原因 | 允许插队，但完成后恢复或重排原故事 |
| 尚未启动的已发布 iteration 被更高优先级 story 取代 | replan | 在原 iteration 追加延期/阻塞说明；新建编号承载新 story；同步后续依赖 | 不把原计划内容改写为新工作的实施记录 |

## 处理步骤

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
8. 如果受影响对象是已发布的 `Planned` iteration，保留其计划基线；改线内容写入新编号，依赖其结果的后续计划同步标注激活阻塞。
9. 调整当前切片后，按 [任务收口与完成声明](TASK-CLOSURE.md) 更新闭环台账中的
   产物、验证和残余归口；旧台账不得继续作为新范围的完成依据。

## 防呆检查表

- [ ] 已确认该输入确实改变当前 `In Progress` story，而不是独立新需求。
- [ ] 已在当前 iteration `执行记录` 增加 change request 行。
- [ ] 已选定唯一变更类型。
- [ ] 如果是 `product-pivot`，已新增 ADR。
- [ ] 已新增或更新 backlog item。
- [ ] 已明确当前故事继续、缩小、暂停还是切换。
- [ ] 已列出半成品代码处理方式：保留 / 移除 / 后续清理。
- [ ] 已更新验证计划，避免验证已经废弃的范围。
- [ ] 如涉及已发布 planned iteration，原目标/范围/依赖仍保留可读；改线使用新编号。
- [ ] 已随新范围更新闭环台账及最终可使用的完成声明条件。

## 记录模板

变更记录模板：

```markdown
| YYYY-MM-DD | Change request: <一句话描述>。类型：<clarification/scope-change/product-pivot/urgent-fix/replan>；决策：<接受/拆分/暂缓>；影响：<范围变化>。 |
```

半成品处理模板：

```markdown
| YYYY-MM-DD | Work-in-progress handling: <文件/模块> -> <保留/移除/后续清理>；原因：<一句话>。 |
```

## 决策结果

| 结果 | 适用场景 | 后续动作 |
|------|----------|----------|
| Continue | 变更只是澄清，原故事仍成立 | 更新记录后继续 |
| Narrow | 原故事过大或部分被替代 | 缩小验收标准，剩余部分回 backlog |
| Switch | 新需求优先级更高 | 当前故事改 Deferred 或 Blocked，新故事进入 In Progress |
| Drop | 原方向明确废弃 | 旧故事改 Dropped，保留 ADR/iteration 说明 |
| Replan | 已发布计划尚未启动但优先顺序改变 | 原计划追加延期/阻塞说明，新目标使用新 iteration 编号 |

## 失败处理

- 无法判断变更类型：先按 `scope-change` 处理，并在 iteration 记录“不确定点”；涉及产品方向时升级为 `product-pivot`。
- 已经写了不适用代码：不要继续扩散；在半成品处理记录中说明保留、移除或后续清理。
- 忘记更新 backlog：先补 backlog 状态和验收标准，再继续实现。
- 变更导致验证计划失效：更新验证命令和验收清单，避免验证旧目标。
- 变更后仍沿用旧闭环台账：将当前结论降为 `Partial`，补齐新范围的状态、验证和
  残余归口后再收尾。

## 相关文档

- [需求进入与 Backlog 整理](REQUIREMENT-INTAKE.md)
- [特性迭代工作流](ITERATION-WORKFLOW.md)
- [决策记录](../decisions/README.md)
- [Product Backlog](../backlog/PRODUCT-BACKLOG.md)
- [任务收口与完成声明](TASK-CLOSURE.md)
