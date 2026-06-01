# Iteration 033: Serverless 执行架构设计 Spike

> 文档状态：Planned / Ready for activation
> 计划发布日期：2026-06-01
> 计划目标：用一个 Spike 明确 `EVO-048` 的本地 serverless runtime、外部执行代理和远期 Vercel 模式演进路径，解锁 CLI/MCP 执行引擎设计。
>
> 基线保护：本文件一旦提交，以下“发布计划基线”内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

- 评估 Phase 7 Docker sandbox 是否可复用为本地 serverless runtime 基础。
- 定义内部 serverless 执行与外部 HTTP 执行代理的统一接口边界。
- 输出 ADR 或 proposal，明确 CLI（EVO-045）与 MCP（EVO-047）后续实施顺序。
- 给出冷启动基准测试方案；如实际执行基准，记录环境和数据。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|---------|--------|---------------|
| EVO-048 | Serverless 执行架构设计 Spike | 无 | P0 | 无硬依赖；建议在 CI 基线后执行 |

## 3. 发布计划基线：不做事项

- 不实现 serverless runtime。
- 不实现 CLI 执行 endpoint 或 MCP serverless tool。
- 不实现 Vercel 完整模式。
- 不改数据库 schema；schema 变更待 Spike 结论进入后续 Story。

## 4. 发布计划基线：计划验收标准

- Story 格式与 BDD 适用性：
  - [x] Spike；BDD 不适用，使用时间盒、设计输出和基准证据。
- [ ] 输出 ADR 或 proposal，包含本地版实现方案和远期演进路径。
- [ ] Phase 7 sandbox 复用结论明确：复用、部分复用或替换。
- [ ] 内部 serverless 与外部执行代理的接口边界明确。
- [ ] 冷启动基准方法和目标（默认 < 2s）明确；如已测量则记录数据。
- [ ] 将后续实施 Story 的依赖关系写回 backlog 或 iteration 计划。

## 5. 发布计划基线：计划验证

```bash
# 文档一致性
python3 /Users/GHuang/WorkSpace/AiProjects/skill-sources/agent-project-governance/scripts/validate_project_governance.py /Users/GHuang/WorkSpace/AiProjects/evolith
git diff --check

# 如执行冷启动基准，按 Spike 输出中的命令记录结果
```

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| Spike 变成实现任务 | 明确时间盒，只输出设计、接口和基准结论 |
| 过早绑定 Vercel 或 Docker | 将本地运行形态和远期替换点分离 |
| CLI/MCP 共用层边界不清 | 用统一执行接口和后续 Story 依赖图表达边界 |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 计划：Serverless 执行架构 Spike，不启动实现 |
| 产物 | ADR/proposal、复用结论、接口草图、后续 Story 依赖图 |
| 状态同步归口 | EVO-048、EVO-045、EVO-047、Iteration 033、decisions/proposals |
| Story/BDD 归口 | Spike；设计输出与基准证据 |
| 验证证据 | 文档治理校验、基准命令输出（如执行） |
| 残余工作归口 | CLI 执行引擎、MCP serverless、外部执行代理拆入后续 backlog |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-06-01 | planning | 发布计划基线；未启动实现。该 Spike 应在工程门禁与 CI 基线稳定后进入。 |

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
