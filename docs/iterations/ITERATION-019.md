# Iteration 019: Skill 多来源导入闭环

> 文档状态：Planned / Blocked for activation
> 计划发布日期：2026-05-27
> 计划目标：为标准 Skill 包建立 ZIP、Git 和 SkillHub 来源的统一导入流程与导入报告。
>
> 基线保护：本文件一旦提交，以下“发布计划基线”内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

本轮候选承接 `SKILL-FORMAT.md` 和 `API-CONTRACT.md` 中已有的导入目标契约，但只在
[Iteration 018](ITERATION-018.md) 建立必要的存储/注册边界并使 `EVO-027` 通过 DoR
后允许激活。导入来源涉及 ZIP 解包和外部同步，实施时必须纳入安全审查。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|---------|--------|---------------|
| EVO-027 | Skill 多来源创建 | 无 | P1 | EVO-020 相关前置切片完成；来源安全策略和统一导入报告边界确认 |

## 3. 发布计划基线：不做事项

- 不在本轮实现版本列表、默认版本切换或回滚；归 Iteration 020。
- 不把低质量描述提示扩展为搜索/排序能力；归 EVO-029 后续计划。
- 不对外部 Git/SkillHub 进行不受控网络探测；测试使用 mock provider 或可控来源。

## 4. 发布计划基线：计划验收标准

- [ ] ZIP、Git、SkillHub 来源均进入统一导入流程并输出一致的 import report。
- [ ] 非法 ZIP、越界路径、非法符号链接、超限内容和失败同步不会创建可用 Skill。
- [ ] 来源元数据、上游版本、同步时间与资源清单可追溯。
- [ ] API contract、前端导入入口和必要数据模型随实现同步。
- [ ] 安全相关测试与适用 backend/frontend 验证通过并记录结果。

## 5. 发布计划基线：计划验证

```bash
cargo test -p service-skill
cargo test -p api
cargo test --workspace
cargo clippy --workspace -- -D warnings
cd frontend
bun run type-check
bun run build
```

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| ZIP 解包或来源同步引入路径穿越/凭据泄露/不受控出站 | 先定义安全边界；使用隔离暂存、显式限制和可控测试 |
| 三类来源一次交付扩大单轮范围 | 激活时允许按统一 pipeline + 单来源适配器拆为顺序子 story，不覆写本基线 |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | future plan only；未启动外部来源接入 |
| 产物 | 激活后应交付统一导入 pipeline、来源适配与导入报告 |
| 状态同步归口 | EVO-027、Iteration 019、API contract、SKILL-FORMAT/reference |
| 验证证据 | 后端安全/集成测试、前端构建及导入向导手工核验 |
| 残余工作归口 | 版本与回滚进入 Iteration 020；描述发现质量继续由 EVO-029 承担 |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-05-27 | planning | 发布 future plan 基线；因 EVO-020 前置与安全 refinement 未完成，本迭代保持阻塞。 |

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|

## 10. Review

- 完成：
- 未完成：
- 验证结果：
- 闭环状态：`Blocked`（仅计划发布，前置依赖未完成）
- 残余归口：激活门禁见第 1 节。

## 11. Retrospective

- 做得好的：
- 需要调整的：
- 写入 EVOLUTION：
