# SOP: 分阶段结对开发

> 本项目采用单上下文、分阶段的 Driver / Navigator 结对模式。不要在同一段推理里并行扮演两个角色互相争论；应先实现，再切换到检查表式审查。

## 触发条件

以下场景建议启用分阶段结对：

- 跨 backend / frontend / db / docs / deploy 的复杂改动。
- API 合约、数据库 migration、权限、认证、审计或发布流程变更。
- 迭代中发生需求变更。
- 重要技术或产品决策。
- 提交前用户要求“检查”“review”“看别人改的有没有问题”。
- Agent 判断当前改动风险较高、范围容易扩大或已有多次返工。

以下场景可以跳过：

- 单纯错别字、断链、轻量文档补充。
- 不改变行为的小范围格式修正。
- 用户明确要求只做一个简单查询或命令输出。

## 角色定义

| 角色 | 职责 | 禁止事项 |
|------|------|----------|
| Driver | 推进实现、编辑文件、运行验证、更新状态 | 不在实现中随意扩大 story 范围 |
| Navigator | 检查方向、SOP、DoR/DoD、风险、无关改动和验证缺口 | 不提出无来源的新需求，不凭感觉否定实现 |

Navigator 的意见必须落到至少一种依据：

- backlog / iteration 验收标准
- ADR 或 roadmap
- SOP / reference 文档
- 具体文件、行或 diff
- 可验证风险：测试、兼容性、安全、数据、部署

## 工作流

### 1. Driver: 准备切片

Driver 在开始实现前确认：

- 当前 story 和验收标准。
- 本次最小切片。
- 需要修改的层：backend / frontend / db / docs / deploy。
- 需要运行的最小验证。

如果切片明显超过 0.5-2 天，回到 [需求进入与 Backlog 整理](REQUIREMENT-INTAKE.md) 拆分。

### 2. Driver: 实现小步

Driver 执行：

1. 读取必要上下文。
2. 编辑文件。
3. 运行局部验证。
4. 更新必要文档和 iteration 执行记录。

实现阶段不要同时做 Navigator 式自我辩论。发现范围变化时，立即转入 [迭代中需求变更](CHANGE-CONTROL.md)。

### 3. Navigator: 小切片审查

Driver 完成一个可检查切片后暂停扩大改动，Navigator 检查：

- 是否仍服务于当前 story。
- 是否符合 ADR / roadmap / SOP。
- 是否修改了无关文件。
- 是否漏了 API contract、migration、双数据库、权限、审计、i18n 或测试。
- 是否有旧概念、旧路由或旧术语误导。
- 验证结果是否足以覆盖风险。

Navigator 输出必须是：

```markdown
Finding:
- 问题：
- 依据：
- 建议动作：
```

没有发现问题时写：

```markdown
Navigator check: no blocking findings. Residual risk: <一句话说明或 none>.
```

### 4. Driver: 修正

Driver 只处理 Navigator 明确指出的问题。

如果 Navigator 的发现意味着范围变化：

1. 暂停实现。
2. 按 [迭代中需求变更](CHANGE-CONTROL.md) 分类。
3. 更新 backlog / iteration / ADR。
4. 再继续。

### 5. Navigator: 提交前检查

提交前 Navigator 检查：

- [ ] `git status --short --branch` 已看过。
- [ ] staged diff 只包含本主题。
- [ ] DoD 已满足或未满足项已说明。
- [ ] backlog / iteration 状态已同步。
- [ ] 验证命令和结果已记录。
- [ ] 需要写 `EVOLUTION.md` 的经验已写入。
- [ ] commit message 有语义前缀和 `[model: <name>]`。

## 防呆规则

- 不要在同一段文字中模拟 Driver 和 Navigator 来回争论。
- Navigator 不接管实现；只给检查结论和可执行修正项。
- Driver 不接受无依据的“感觉不对”。
- 小任务不强制启用 Navigator，避免流程成本超过收益。
- 如果启用了 Navigator，最终回复或迭代记录应说明审查结论。

## 失败处理

- 角色混乱：停下，回到当前阶段，只保留一个角色目标。
- Navigator 提出新需求：按 `CHANGE-CONTROL.md` 处理，不直接实现。
- Driver 忽略审查发现：在 iteration 执行记录中说明原因，否则不得提交。
- 审查发现过多：缩小当前切片，剩余内容回 backlog。

## 相关文档

- [特性迭代工作流](ITERATION-WORKFLOW.md)
- [迭代中需求变更](CHANGE-CONTROL.md)
- [开始一次迭代](START-ITERATION.md)
- [Git 工作流](GIT-WORKFLOW.md)
