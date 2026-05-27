# Iteration 018: Skill 导入基础能力细化与基线

> 文档状态：Planned / Blocked for activation
> 计划发布日期：2026-05-27
> 计划目标：在多来源导入前明确 registry 与 storage 的职责边界，并形成满足 DoR 的
> 最小基础能力切片。
>
> 基线保护：本文件一旦提交，以下“发布计划基线”内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

`EVO-027` 已声明依赖 Storage 且需要处理外部来源安全边界，不能在基础责任未明确时
直接启动导入实现。本轮计划用于把 `EVO-019` / `EVO-020` 从 placeholder 候选细化为
可执行切片，并仅在拆分后的 story 通过 DoR 后激活实现。

激活门禁：

- [Iteration 009](ITERATION-009.md) 与 [Iteration 010](ITERATION-010.md) 已关闭，或对
  各自残余给出不阻塞本链的明确归口。
- `EVO-019` 与 `EVO-020` 至少拆出本轮选入的 0.5-2 天 story，补齐验收、依赖和验证。
- 明确外部包落盘、暂存/清理、租户边界与导入报告持久化是否归属 Storage。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|---------|--------|---------------|
| EVO-019 | Skill registry 服务化 | 无 | P2 | 激活前完成 DoR/refinement，并明确其是否支撑同步来源 |
| EVO-020 | Storage 能力落地 | 无 | P2 | 激活前完成 DoR/refinement；为 EVO-027 的包暂存与资源清单提供基础 |

## 3. 发布计划基线：不做事项

- 不在基础边界未确认前实现 ZIP、Git 或 SkillHub 的端到端导入。
- 不同时实现版本回滚或描述质量评分。
- 不将外部网络访问、安全策略和部署配置留作隐式实现决定。

## 4. 发布计划基线：计划验收标准

- [ ] EVO-019 / EVO-020 的职责和依赖被细化为可执行 story，且选入项满足 DoR。
- [ ] 明确 EVO-027 所需的存储、来源元数据、临时文件清理和安全边界。
- [ ] 如实施基础切片，配套 repository/service 测试与必要 reference/contract 更新完成。
- [ ] 验证结果和未实现的后续导入范围被真实记录。

## 5. 发布计划基线：计划验证

```bash
# 按激活时选入的后端 crate 缩小验证范围，并在收口运行适用 workspace 门禁
cargo test -p service-skill
cargo test --workspace
cargo clippy --workspace -- -D warnings
```

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| 把 storage/registry placeholder 扩成无边界平台重构 | 激活前拆为可验证 story，只选最小前置切片 |
| 导入临时文件或外部来源安全要求被遗漏 | 在 refinement 中显式列出大小、路径、清理、网络和凭据边界 |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | future plan only；先规划 EVO-027 前置能力 |
| 产物 | 激活后应产出可验证的 registry/storage 最小切片及导入依赖结论 |
| 状态同步归口 | EVO-019、EVO-020、Iteration 018、EVO-027 依赖说明 |
| 验证证据 | refinement 结果与激活后对应 backend 测试/检查 |
| 残余工作归口 | 未选入的基础切片继续留在 backlog；导入能力归 Iteration 019 |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-05-27 | planning | 发布 future plan 基线；候选仍为 `Proposed` 且需拆分/DoR，本迭代暂不可激活。 |

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|

## 10. Review

- 完成：
- 未完成：
- 验证结果：
- 闭环状态：`Blocked`（仅计划发布，候选需 refinement）
- 残余归口：激活门禁见第 1 节。

## 11. Retrospective

- 做得好的：
- 需要调整的：
- 写入 EVOLUTION：
