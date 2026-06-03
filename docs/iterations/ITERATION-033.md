# Iteration 033: Serverless 执行架构设计 Spike

> 文档状态：Closed（2026-06-03）
> 计划发布日期：2026-06-01
> 计划目标：用一个 Spike 明确 `EVO-048` 的本地 serverless runtime、外部执行代理和远期 Vercel 模式演进路径，解锁 CLI/MCP 执行引擎设计。
>
> 基线保护：本文件一旦提交，以下"发布计划基线"内容不可因实施或改线而覆写；
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
| 请求结果 | 输出 `docs/proposals/SERVERLESS-RUNTIME.md`：Phase 7 sandbox 复用结论、统一执行接口、本地版架构、远期 Vercel 演进路径、冷启动基准方法学、EVO-045/047 后续 Story 依赖图 |
| 产物 | `docs/proposals/SERVERLESS-RUNTIME.md`；`docs/proposals/AGENT-RUNTIME.md` / `AI-GATEWAY.md` 关联归口同步 |
| 状态同步归口 | EVO-048 → In Progress（执行中）/ Done（收口时）、EVO-045 / EVO-047 依赖图写回 PRODUCT-BACKLOG.md、Iteration 033 → Closed |
| Story/BDD 归口 | Spike；BDD 不适用；设计输出 + 复用决策 + 接口草图 + 后续 Story 依赖图 + 基准方法学 + 已知环境约束 |
| 验证证据 | 文档链接检查（DOC-CHECK） / `git diff --check`；设计文档内部 cross-check（与 ARCHITECTURE.md §Execution Layer / Phase 7 sandbox 一致性） |
| 残余工作归口 | 冷启动**实测**（需 Docker，本机无 → 基准方法学 + 已知约束归口 EVO-048 详情块"不做"或补 E2E Story）；CLI 执行引擎 → EVO-045；MCP serverless tool → EVO-047；外部执行代理 → EVO-048 派生 |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-06-01 | planning | 发布计划基线；未启动实现。该 Spike 应在工程门禁与 CI 基线稳定后进入。 |
| 2026-06-01 | activation | 状态 `Planned / Ready` → `Active / In Progress`。前置：Iteration 028/029/031/032/035 均 Closed，CI 基线（`.github/workflows/ci.yml` tag-only trigger）就绪，工程门禁稳定，满足 plan baseline §"建议在 CI 基线后执行"。本机无 Docker（`command not found: docker`）→ 冷启动实测 conditional，方法学必出、实测值标注为待 Docker 环境执行。激活后第一动作：盘点 Phase 7 sandbox 与 service-tool HTTP executor 现有实现 → 写 `docs/proposals/SERVERLESS-RUNTIME.md`。 |
| 2026-06-03 | execution | Phase 7 sandbox 分析完成：docker_executor.rs 322 行，per-request 容器生命周期（create → exec → remove），SandboxConfig 复用、容器池管理需重写。service-tool HTTP executor 分析完成：reqwest 转发，无容器。两个 trait 形状不兼容（ExecuteRequest/ExecuteResponse 字段不同）。 |
| 2026-06-03 | design | 输出 `docs/proposals/SERVERLESS-RUNTIME.md`（11 节）：Phase 7 复用结论（部分复用）+ 统一 ExecutionProvider 接口 + 容器池架构 + 冷启动方法学（已缓存 ~350ms-1800ms，池模式 ~50-200ms）+ Vercel 演进路径 + 组件替换清单 + EVO-045/047 依赖图。 |
| 2026-06-03 | closure | Spike 完成，状态 → Closed。闭环结论：Complete（冷启动实测待 Docker 环境）。残余：bollard 升级 EVO-061 建议在 EVO-045-A 前提升优先级。 |

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|

## 10. Review

- 完成：
  - 输出 `docs/proposals/SERVERLESS-RUNTIME.md`（§1-11，含架构、接口、冷启动方法学、依赖图）
  - Phase 7 sandbox 复用结论：**部分复用**（SandboxConfig/容器创建/exec/镜像/bollard 连接复用；容器池管理需重写）
  - 统一执行接口设计：`ExecutionProvider` trait + `ExecutionPayload` 枚举（Code/Command/HttpProxy）+ 路由规则
  - 冷启动方法学定义（§5.3）+ 理论分析（已缓存镜像 ~350ms-1800ms，池模式 ~50-200ms）
  - 远期 Vercel 模式组件替换清单（§6.2）
  - 后续 Story 依赖图 + 推荐实施顺序（§8）
- 未完成：
  - 冷启动实测数据（本机无 Docker，方法学已定义，实测待 Docker 环境执行）
- 验证结果：
  - 文档内部一致性：与 ARCHITECTURE.md §Execution Layer / Phase 7 sandbox 实现对齐
  - 接口设计与现有 `SkillExecutor` / `ToolExecutor` 兼容（Phase 1 facade 策略）
  - 依赖图与 PRODUCT-BACKLOG EVO-045/047/049-A 关系一致
- 闭环状态：`Complete`
- 残余归口：
  - 冷启动实测 → 待 Docker 环境就绪后执行，可补入 EVO-045-A 验收或创建独立 benchmark story
  - bollard 0.17→0.18+ 升级 → EVO-061（P3 Proposed，建议在 EVO-045-A 前提升优先级）
  - EVO-045/047 拆分 → 按 §8 推荐顺序，EVO-045-A 是下一个关键路径

## 11. Retrospective

- 做得好的：
  - Spike 严格遵循时间盒和不做约束，未滑入实现
  - 对 Phase 7 sandbox 做了逐行分析（322 行 docker_executor.rs），复用结论有代码级证据
  - 冷启动分析区分了"已缓存 vs 未缓存"和"per-request vs pool"两个维度
  - 统一接口设计考虑了迁移策略（facade → 直接调用），避免 big-bang 重写
- 需要调整的：
  - 本机无 Docker 导致实测缺失 — 下次 Spike 应提前确认环境能力
  - EVO-048 详情块中的冷启动基准测试写法（`目标 < 2s`）暗示需要实测数据，但 Spike plan baseline 已写明"如实际执行"，优先级判断正确
- 写入 EVOLUTION：无新陷阱。本 Spike 为纯设计输出，未触发代码变更或流程问题。

## 11. Retrospective

- 做得好的：
  - Spike 严格遵循时间盒和不做约束，未滑入实现
  - 对 Phase 7 sandbox 做了逐行分析（322 行 docker_executor.rs），复用结论有代码级证据
  - 冷启动分析区分了"已缓存 vs 未缓存"和"per-request vs pool"两个维度
  - 统一接口设计考虑了迁移策略（facade → 直接调用），避免 big-bang 重写
- 需要调整的：
  - 本机无 Docker 导致实测缺失 — 下次 Spike 应提前确认环境能力
  - EVO-048 详情块中的冷启动基准测试写法（`目标 < 2s`）暗示需要实测数据，但 Spike plan baseline 已写明"如实际执行"，优先级判断正确
- 写入 EVOLUTION：无新陷阱。本 Spike 为纯设计输出，未触发代码变更或流程问题。
