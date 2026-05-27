# Iteration 020: Skill 版本与正确性验证

> 文档状态：Planned / Blocked for activation
> 计划发布日期：2026-05-27
> 计划目标：在多来源导入模型稳定后，为 Skill 建立版本追踪、回滚和 Agent Skills
> 兼容校验报告。
>
> 基线保护：本文件一旦提交，以下“发布计划基线”内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

本轮候选以 `EVO-028` 为唯一实施目标，建立可审核的版本和校验结果模型。其输入模型
必须建立在 [Iteration 019](ITERATION-019.md) 的导入报告与来源元数据之上；不得在
多来源导入尚不稳定时先行固定版本接口或数据结构。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|---------|--------|---------------|
| EVO-028 | Skill 版本管理与正确性验证 | 无 | P1 | EVO-027 完成；校验器策略与版本唯一性/默认版本语义确认 |

## 3. 发布计划基线：不做事项

- 不在本轮承担描述搜索排序或专业描述编辑体验；该范围继续归 EVO-029。
- 不未经 ADR/契约核对改写现有 Skill 执行模型。
- 不重建 GitHub CI/CD；项目最终构建和部署形态稳定后再处理 EVO-030。

## 4. 发布计划基线：计划验收标准

- [ ] 创建、导入和更新形成可追溯版本记录，默认版本切换与回滚语义明确。
- [ ] 校验覆盖 Agent Skills 关键结构、frontmatter 规则和兼容字段，区分 error/warning。
- [ ] API 与前端能展示版本和校验报告，并对阻塞错误禁止发布可用版本。
- [ ] 数据模型、迁移、双数据库 repository 和测试在需要时同步覆盖。
- [ ] 适用 backend/frontend 验证通过并逐命令记录。

## 5. 发布计划基线：计划验证

```bash
cargo test -p service-skill
cargo test -p infra
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
| 版本模型在导入模型前固定，导致重复迁移 | 仅在 Iteration 019 结果稳定后激活，并先核对 contract/ADR |
| 正确性校验与描述质量范围混淆 | error 仅约束结构/兼容正确性；质量提示和搜索提升留给 EVO-029 |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | future plan only；未启动版本/校验实现 |
| 产物 | 激活后应交付版本模型、校验报告、API/UI 和必要数据库变更 |
| 状态同步归口 | EVO-028、Iteration 020、API contract、SKILL-FORMAT、migration/reference |
| 验证证据 | parser/validator/repository/API 测试，前端构建和报告显示核验 |
| 残余工作归口 | 专业描述和发现质量留在 EVO-029；CI/CD 留在 EVO-030 |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-05-27 | planning | 发布 future plan 基线；依赖 Iteration 019 / EVO-027，当前不可激活。 |

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
