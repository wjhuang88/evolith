# Iteration 024: Embedded Frontend 交付形态 Refinement

> 文档状态：Planned / Ready for activation
> 计划发布日期：2026-05-28
> 计划目标：用一周完成 EVO-016-A refinement，明确前端嵌入后端发布物的交付边界，
> 为 Iteration 012 / EVO-016-B 解除前置设计阻塞。
>
> 基线保护：本文件一旦提交，以下“发布计划基线”内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

- 明确当前 Nginx 静态托管与终局 embedded frontend 的关系。
- 明确后端服务 Vite 静态产物、SPA fallback、API/health/MCP 路由优先级和 Docker 形态。
- 产出 `EVO-016-B` 可实施边界，供 [Iteration 012](ITERATION-012.md) 激活时使用。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|---------|--------|---------------|
| EVO-016-A | Embedded Frontend 交付形态 refinement | EVO-016 | P2 | Vite SPA 与 Bun 构建已完成；本轮只做方案和实施边界 |

### Iteration Inventory Disposition

| Iteration | 当前状态 | 处置结论 |
|-----------|----------|----------|
| Iteration 012 | Planned / Blocked | 继续阻塞，等待本轮 EVO-016-A 输出 EVO-016-B 边界 |
| Iteration 017 | Planned | 009/010 阻塞已解除；仍需激活前核对 EVO-026 DoR，可与本轮之后择优启动 |
| Iteration 018-020 | Planned / Blocked | 保留 Phase E 依赖链，不在本轮激活 |

## 3. 发布计划基线：不做事项

- 不实现静态文件服务、不改 Dockerfile、不调整生产 Nginx。
- 不重建 GitHub CI/CD。
- 不处理 Skill 生命周期或租户管理功能。

## 4. 发布计划基线：计划验收标准

- Story 格式与 BDD 适用性：
  - [x] 候选为 Technical Story，使用等价技术验收。
  - [x] 行为类 BDD 不适用；验收以文档一致性和实施边界为准。
- [ ] 明确 embedded frontend 的目标形态、过渡形态和 Nginx 最终角色。
- [ ] 明确后端 route ordering、SPA fallback、`/assets/`、`/api/v1`、`/health`、`/mcp` 验证矩阵。
- [ ] 明确 Docker/build 输入输出和本地验证命令。
- [ ] 更新 Iteration 012 激活条件，或记录仍阻塞原因。

## 5. 发布计划基线：计划验证

```bash
rg -n "nginx|assets|fallback|EMBEDDED|VITE_API_URL|APP__PUBLIC_URL" docs deploy backend frontend
python3 /Users/GHuang/WorkSpace/AiProjects/skill-sources/agent-project-governance/scripts/validate_project_governance.py /Users/GHuang/WorkSpace/AiProjects/evolith
git diff --check
```

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| refinement 变成实现任务 | 本轮只改文档和计划，实施另走 Iteration 012 |
| SPA fallback 截获 API/health/MCP | 先写验证矩阵，实施时逐项验证 |
| CI 过早绑定过渡部署 | 明确 CI 仍归 EVO-030，在最终命令稳定后处理 |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 一周计划：完成 embedded frontend 方案和 EVO-016-B 激活边界 |
| 产物 | 方案文档、Iteration 012 激活条件更新、EVO-016-B 范围确认 |
| 状态同步归口 | EVO-016-A、Iteration 024、Iteration 012、roadmap/reference（如适用） |
| Story/BDD 归口 | Technical Story；等价技术验收 |
| 验证证据 | 文档搜索、governance validator、`git diff --check` |
| 残余工作归口 | 实施进入 Iteration 012；CI/CD 进入 EVO-030 |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-05-28 | planning | 发布一周计划基线；未启动实现。 |

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
