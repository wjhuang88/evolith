# Iteration 050: 项目体检治理基线与生产就绪重排

> 文档状态：Review  
> 计划发布日期：2026-07-30  
> 计划目标：将全面体检结论沉淀为 owner docs、发布门禁和可执行 Backlog。  
> Draft PR：#2 `docs: 建立生产就绪与安全治理主线`  
>
> 基线保护：本文件一旦提交，以下“发布计划基线”内容不可因实施或改线而覆写；同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。  
> 闭环步骤：按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 0. Iteration 库存处置

| 既有 Iteration | 状态 | 本轮处置 |
|----------------|------|----------|
| Iteration 049 | Closed | 保持关闭，EVO-103 验收硬化作为本轮审查基础 |
| Iteration 018~020、027 | Superseded / Blocked for activation | 继续由 Git-centric 方向替代，不激活 |
| Iteration 025 | Blocked for activation | 继续阻塞；租户设置/审计不抢占生产就绪 P0 |
| Iteration 026 | Blocked for activation | 继续阻塞；计费闭环不抢占生产就绪 P0 |
| EVO-112 候选 | Proposed，尚无 Iteration | 暂缓实现；允许本治理/P0 修复在 Repo UI 前插队 |

盘点结论：开始本轮前没有必须继续推进的 Active/In Progress/Review 产品 Iteration。由于全面体检发现发布阻断级安全和数据耐久性问题，本轮以治理修复 Iteration 插队，不激活既有产品计划。

## 1. 发布计划基线：目标

- 建立 2026-07-30 生产就绪与项目完成度事实基线。
- 把权限、SSRF、Git 数据持久化、备份恢复和生产构建列为明确 P0 Gate。
- 将运行可靠性、Repo 生命周期和 Durable Event 拆成可独立验收 Story。
- 更新 Agent、Backlog、Board、Roadmap、Release 和文档地图，使后续开发默认遵守同一顺序。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|-----------|--------|---------------|
| EVO-118-A | 项目体检治理基线与优先级重排 | EVO-118 | P0 | 用户授权直接修改并提交；代码/配置静态审查已完成 |

## 3. 发布计划基线：不做事项

- 不修复运行时代码、数据库、Dockerfile、Compose 或脚本。
- 不把 EVO-118-B~H 伪装为本轮已完成。
- 不启动 Repo UI、Commit/Promote、Agent Session 或 Indexer。
- 不覆写原 2026-06-29 两个月计划基线；只追加偏差和替代关系。

## 4. 发布计划基线：计划验收标准

- Story 格式与 BDD 适用性：
  - [x] EVO-118-A 为 Governance Story。
  - [x] BDD 不适用；采用文档一致性、状态同步和可追踪性验收。
- [x] 新增生产就绪事实基线，明确当前产品成熟度和发布状态。
- [x] 新增安全敏感变更 SOP，覆盖授权、SSRF、Git 写入、耐久性和生产降级。
- [x] 新增 EVO-118 Epic 及 A~H 子 Story。
- [x] 新增当前生产就绪执行计划，明确 S1/S2 与产品主线恢复条件。
- [x] Backlog、Board、Roadmap、Release、Agent 入口和文档地图完成同步。
- [x] Draft PR #2 已创建并保持 Draft。
- [ ] Draft PR 审查并合并到 `main`。

## 5. 发布计划基线：计划验证

```text
1. GitHub fetch: 核对新增/更新文档和相对链接目标
2. GitHub compare: main...agent/project-health-governance
3. PR file list / diff: 检查只包含授权治理范围
4. 本地 checkout 可用时：DOC-CHECK + git diff --check
```

代码测试不适用：本轮不改变运行时行为。

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| 治理文档过多导致入口不清 | Baseline 为事实 owner，Roadmap 为顺序 owner，Backlog/Iteration 为执行 owner，Board 只派生 |
| 与原两个月计划冲突 | 保留原基线并新增当前执行计划，不就地改写旧目标 |
| 把静态审查当运行验证 | 所有代码修复子 Story 都要求独立测试、clean build 或恢复演练 |
| P0 过多形成大批次 | A~H 独立 Story；默认 WIP 一次一个 |
| 需要回滚 | 关闭 PR 或回退本治理分支；不影响运行时代码和数据 |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 体检结论写入治理文档并提交 GitHub 仓库 |
| 产物 | Production Baseline、Security SOP、Readiness Plan、EVO-118/A~H、Iteration 050、入口与状态同步 |
| 状态同步归口 | Product Backlog、Board、Implementation Roadmap、Two-Month Plan、AGENTS、docs README、Iteration README |
| Story/BDD 归口 | Governance；BDD 不适用，使用文档和状态一致性验收 |
| 验证证据 | GitHub 文件读取、branch compare、Draft PR #2；本地 DOC-CHECK 待可用环境补跑 |
| 残余工作归口 | PR review/merge；运行时代码分别归 EVO-118-B~H |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-07-30 | activation | 完成 Iteration 库存盘点；允许生产就绪治理作为 P0 插队，不激活 EVO-112 |
| 2026-07-30 | progress | 新增 Production Readiness Baseline、Security Review SOP 和重排计划 |
| 2026-07-30 | progress | 建立 EVO-118 Epic 与 A~H 可执行 Story |
| 2026-07-30 | progress | 同步 Backlog、Board、Roadmap、Release、Architecture、Agent 入口和文档地图 |
| 2026-07-30 | validation | `main...branch` compare 仅包含 `AGENTS.md` 与 `docs/**`；无运行时代码/DB/脚本/部署配置变更 |
| 2026-07-30 | review | Draft PR #2 已创建；等待人工审查、merge 与可用本地环境补跑 DOC-CHECK |

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|
| 2026-07-30 | urgent-fix / replan | 接受 | EVO-118 S1 插入 EVO-103 与 EVO-112 之间；旧两个月计划转为历史基线 | 保留旧计划；新建当前执行计划 |

## 10. Review

- 完成：治理 owner docs、P0/P1 Backlog、执行顺序和发布门禁已形成，分支 compare 确认变更仅限治理文档，Draft PR #2 已创建。
- 未完成：PR 尚未合并；本环境没有本地 checkout，DOC-CHECK Python、Markdown link checker 和 `git diff --check` 尚未实际运行。
- 验证结果：GitHub Contents API 文件级复核完成；Compare 无代码或配置混入；代码测试不适用。
- 闭环状态：`Partial`
- 残余归口：合并并补齐文档检查后将 EVO-118-A/Iteration 050 更新为 Done/Closed；运行时修复按 EVO-118-B~H 逐项启动。

## 11. Retrospective

- 做得好的：将主观体检结论转换成发布 Gate、子 Story、失败模式和命令级验收，避免只留在对话中。
- 需要调整的：后续每轮必须验证 owner docs 与派生 Board 的状态，避免治理文档再次领先于真实代码。
- 写入 EVOLUTION：如 PR 审查发现新的治理陷阱，再按 EVOLUTION-FEEDBACK 判断写回。
