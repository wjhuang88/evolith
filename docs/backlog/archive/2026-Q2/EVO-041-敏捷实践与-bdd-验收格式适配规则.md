# EVO-041 敏捷实践与 BDD 验收格式适配规则

> Archived backlog item. Routing index: [2026 Q2 Archive](INDEX.md). Active routing surface: [Product Backlog](../../PRODUCT-BACKLOG.md).

- Type: tech-debt
- Status: Done
- Priority: P1
- Source: 用户方法论反馈 2026-05-28
- Decision Context: Iteration 022；明确 Evolith iteration 与传统 Sprint、Story 与 BDD 的适配口径

#### Source Detail Snapshot

- 类型：tech-debt
- 优先级：P1
- 状态：Done
- 父 Epic：无
- 用户价值或技术目标：让 Evolith 的 Agent 迭代治理能吸收传统敏捷和 BDD 的可验证性，
  同时保留本项目“计划基线、库存盘点、命令级证据、闭环归口”的执行边界，避免后续
  Agent 机械套用 Scrum 或把所有任务都写成不合适的用户故事。
- 范围：
  - 定义 Evolith `iteration` 与传统 Scrum `Sprint` 的关系、差异和适用节奏。
  - 为产品故事、API/权限/状态故事、技术故事、治理文档故事和 spike 定义不同表述方式。
  - 明确哪些任务必须使用 Given/When/Then BDD 场景，哪些任务可用等价技术验收。
  - 将 Story / BDD 质量检查接入 DoR、迭代计划、文档一致性检查和模板。
  - 写回本次方法论经验，作为后续 Agent 判断依据。
- 不做：
  - 不引入完整 Scrum 仪式、团队容量统计或固定冲刺承诺。
  - 不修改业务代码、测试代码、CI/CD 或部署策略。
  - 不提交外部 `agent-project-governance` skill；仅按用户要求同步内容，提交由用户另行处理。
- 验收标准：
  - [x] `REQUIREMENT-INTAKE.md` 定义 Story 类型、BDD 适用规则与等价技术验收规则。
  - [x] `ITERATION-WORKFLOW.md` 说明 Evolith iteration 与传统 Sprint 的映射和差异。
  - [x] `DOC-CHECK.md` 能检查 Story / BDD / 技术验收的一致性。
  - [x] `ITERATION-TEMPLATE.md` 在计划验收和闭环台账中承接 BDD 适用性。
  - [x] `AGENTS.md` 入口约束和 `EVOLUTION.md` 经验记录覆盖该方法论。
  - [x] `agent-project-governance` skill 同步体现 Sprint / iteration / Story / BDD 适配方法。
  - [x] Markdown 链接检查和 `git diff --check` 通过。
- 依赖或阻塞：无；本故事为治理改进，可在不激活产品 planned iteration 的情况下实施。
- 影响范围：docs / external skill
- 最小验证方式：执行 Markdown 相对链接检查；`git diff --check`；运行 skill 结构校验。
