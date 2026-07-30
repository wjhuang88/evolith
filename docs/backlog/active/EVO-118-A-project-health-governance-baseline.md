# EVO-118-A 项目体检治理基线与优先级重排

- **类型**：Governance / Documentation
- **状态**：Review
- **优先级**：P0
- **父 Epic**：[EVO-118](EVO-118-production-readiness-and-security-hardening.md)
- **所属迭代**：[Iteration 050](../../iterations/ITERATION-050.md)
- **触发**：2026-07-30 全面项目体检

## 工程目标

把静态代码、部署配置、测试证据和产品完成度审查结论写入 owner docs，形成后续 Agent 和开发者无法绕过的发布门禁、执行顺序和可启动 Backlog。

## Story 形态

Governance Story。BDD 不适用；使用文档一致性、状态同步和可追踪性验收。

## 实施范围

- 新增生产就绪与完成度事实基线。
- 新增安全敏感变更审查 SOP。
- 建立 EVO-118 Epic 及独立子 Story。
- 新增生产就绪优先级重排计划。
- 更新 Product Backlog、Board、Implementation Roadmap、Release SOP、Agent Task Router、文档地图和 Iteration 目录。
- 保留原两个月计划作为历史基线，只追加偏差/替代说明。
- 不修改运行时代码、数据库、脚本或 API 行为。

## 验收标准

- [x] 项目成熟度明确区分工程骨架、Git Alpha、AI-native MVP 和生产就绪。
- [x] API Key/MCP、HTTP Tool SSRF、Git 持久化/备份、生产构建被标为 P0 发布阻断项。
- [x] Repo 生命周期、运行门禁和 Durable Event 被拆成独立 P1 Story。
- [x] 路线图将 EVO-118 S1 放在 EVO-112 之前。
- [x] Security Review SOP 定义角色矩阵、负向测试、SSRF、Git 写入、耐久性和发布阻断规则。
- [x] Release SOP 不再只验证数据库和前端，而是覆盖 Git 数据、恢复和 clean build。
- [x] Board、Backlog、Iteration、Roadmap 和 Reference 使用同一归口关系。
- [ ] Draft PR 已审查并合并到 `main`。

## 验证计划

- 核对所有新增相对链接目标存在。
- 使用 `main...agent/project-health-governance` compare 检查只包含授权治理文档。
- 检查 EVO-118 父子状态、依赖和优先级一致。
- 检查原 Iteration/两个月计划基线未被改写为不同目标。
- 本地 checkout 可用时运行 DOC-CHECK 与 `git diff --check`。

## 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 将项目体检结论沉淀为可执行治理文档并提交仓库 |
| 产物 | Baseline、Security SOP、Roadmap、Epic/Stories、Iteration、Board/Backlog/入口同步 |
| 状态同步归口 | EVO-118/A、Iteration 050、Product Backlog、Board、Roadmap |
| Story/BDD 归口 | Governance；BDD 不适用，使用文档与状态一致性验收 |
| 验证证据 | GitHub 文件读取、分支 compare、PR diff；本地 DOC-CHECK 待可用环境补跑 |
| 残余工作归口 | PR merge；运行时代码修复分别归 EVO-118-B~H |

## 完成声明

**Partial / Review**：治理产物已形成并提交分支；在 PR 合并和本地 DOC-CHECK 证据完成前不标记 Done。
