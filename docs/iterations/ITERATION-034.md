# Iteration 034: Skill/CLI 规范兼容数据模型基线

> 文档状态：Planned / Ready for activation
> 计划发布日期：2026-06-01
> 计划目标：用一个可审计切片完成 `EVO-049-A`，让 Skill 与 CLI interface 的数据库、domain、DTO 和 repository 能承载 Agent Skills 规范字段与结构化 CLI 命令字段。
>
> 基线保护：本文件一旦提交，以下“发布计划基线”内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

- 将 `EVO-049` 父 Epic 的 Phase 1 数据模型升级拆成独立可验收 Story。
- 完成 SQLite/PostgreSQL、domain、DTO 和 repository 层的最小字段基线。
- 保留旧 `skill_md` 与 legacy snippet/CLI 字段兼容，不在本轮做 parser 接线或制品化。
- 解锁 [EVO-049-B](../backlog/PRODUCT-BACKLOG.md) parser 接线与校验报告。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|---------|--------|---------------|
| EVO-049-A | Skill/CLI 规范兼容数据模型基线 | EVO-049 | P0 | 无硬依赖；激活前核对迁移范围与 EVO-027/028/046/050 依赖边界 |

## 3. 发布计划基线：不做事项

- 不接线 `SkillParser` / `CliInterfaceParser` 到 handler；归 EVO-049-B。
- 不实现 ZIP/Git/SkillHub 导入和制品下载。
- 不实现 CLI/MCP serverless 执行。
- 不实现评分、搜索排序或复杂前端编辑器。
- 不删除旧 API 路径或旧字段。

## 4. 发布计划基线：计划验收标准

- Story 格式与 BDD 适用性：
  - [x] Technical Story；BDD 不适用，使用双数据库、repository 和 contract 验收。
- [ ] SQLite 与 PostgreSQL migration 对 skills / CLI interface 新字段保持语义一致。
- [ ] domain model、Create/Update DTO、Response DTO 和 repository 映射覆盖新增字段。
- [ ] 旧字段兼容路径仍可工作，未删除 legacy snippet API。
- [ ] repository 测试覆盖新增字段 create/update/read。
- [ ] API contract 或 format reference 说明本轮字段基线和 parser 接线后续边界。

## 5. 发布计划基线：计划验证

```bash
cargo test -p infra
cargo test -p api
cargo check --workspace
cargo clippy --workspace -- -D warnings
cargo test --workspace
git diff --check
```

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| 字段一次性过多导致迁移风险 | 只承载规范兼容和 CLI 结构化最小字段，执行/评分/制品另拆 |
| SQLite/PostgreSQL schema 漂移 | 双轨 migration 同步审查，并用 repository 测试覆盖 |
| 父 Epic 被误当作完成 | 只更新 EVO-049-A；EVO-049 仍按子项表判断完成 |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 计划：Skill/CLI 规范兼容数据模型基线，不启动实现 |
| 产物 | 双数据库 migration、domain/DTO/repository 映射、测试、reference/contract 更新 |
| 状态同步归口 | EVO-049-A、EVO-049 子 Story 表、Iteration 034、API/format reference |
| Story/BDD 归口 | Technical Story；双数据库和命令级验收 |
| 验证证据 | infra/api/workspace 测试、clippy/check/diff 检查 |
| 残余工作归口 | parser 接线归 EVO-049-B；制品化归后续 EVO-049 子 Story / EVO-046 |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-06-01 | planning | 发布计划基线；未启动实现。 |

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|

## 10. Review

- 完成：
- 未完成：
- 验证结果：
- 闭环状态：`Blocked`（仅计划发布，尚未激活）
- 残余归口：激活后按第 7 节闭环。

## 11. Retrospective

- 做得好的：
- 需要调整的：
- 写入 EVOLUTION：
