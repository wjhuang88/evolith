# Iteration 027: Skill 发现质量与描述治理

> 文档状态：Planned / Blocked for activation
> 计划发布日期：2026-05-28
> 计划目标：用一周推进 `EVO-029`，提升 Skill description、触发场景和发现质量，
> 但不混入版本模型或多来源导入实现。
>
> 基线保护：本文件一旦提交，以下“发布计划基线”内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

- 建立 Skill 描述质量、触发关键词、能力边界和兼容性元数据的最小治理规则。
- 为后续 Skill 搜索/发现排序提供可验证基础。
- 不依赖多来源导入全部完成，但需与 `SKILL-FORMAT.md` 和 parser 现状兼容。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|---------|--------|---------------|
| EVO-029 | Skill 专业描述与发现质量提升 | 无 | P2 | EVO-009 parser 已完成；激活前确认与 EVO-028 的边界 |

## 3. 发布计划基线：不做事项

- 不实现 ZIP/Git/SkillHub 导入；归 Iteration 019。
- 不实现版本回滚或正确性报告；归 Iteration 020。
- 不引入语义搜索基础设施；只做可验证的描述和元数据质量基线。

## 4. 发布计划基线：计划验收标准

- Story 格式与 BDD 适用性：
  - [ ] Technical/API Story 补齐描述质量规则、错误/警告口径和 UI/API 可观察结果。
- [ ] 定义 Skill description 质量规则：能力、触发场景、输入/输出、限制和关键词。
- [ ] parser/API/UI 能展示或校验必要元数据，且与 Agent Skills spec 不冲突。
- [ ] 低质量描述有可解释 warning，不阻断结构正确的 Skill 除非违反硬性格式。

## 5. 发布计划基线：计划验证

```bash
cargo test -p service-skill
cargo test -p api
cd frontend
bun run type-check
bun run build
```

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| 描述质量与正确性校验混淆 | error 只用于结构/兼容硬错误，quality warning 不阻断基础使用 |
| 搜索排序范围过大 | 本轮只建立字段、规则和展示，不引入语义搜索基础设施 |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 一周计划：建立 Skill 描述与发现质量最小基线 |
| 产物 | 描述规则、parser/API/UI 或 reference 更新、质量测试 |
| 状态同步归口 | EVO-029、Iteration 027、SKILL-FORMAT/reference |
| Story/BDD 归口 | Technical/API Story；激活前补齐可观察验收 |
| 验证证据 | service-skill/api 测试、frontend type-check/build |
| 残余工作归口 | 语义搜索和高级排序另建后续 backlog |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-05-28 | planning | 发布一周计划基线；激活前需核对与 Iteration 019/020 的依赖边界。 |

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|

## 10. Review

- 完成：
- 未完成：
- 验证结果：
- 闭环状态：`Blocked`（仅计划发布，候选需 refinement）
- 残余归口：激活门禁见第 4 节。

## 11. Retrospective

- 做得好的：
- 需要调整的：
- 写入 EVOLUTION：
