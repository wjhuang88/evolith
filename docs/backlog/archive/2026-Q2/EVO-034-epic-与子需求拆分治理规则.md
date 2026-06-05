# EVO-034 Epic 与子需求拆分治理规则

> Archived backlog item. Routing index: [2026 Q2 Archive](INDEX.md). Active routing surface: [Product Backlog](../../PRODUCT-BACKLOG.md).

- Type: tech-debt
- Status: Done
- Priority: P1
- Source: 流程缺口 2026-05-26
- Decision Context: Iteration 008；已补齐父子编号、依赖、分层 DoR 与跨 Epic 选取约束

#### Source Detail Snapshot

- 类型：tech-debt
- 优先级：P1
- 状态：Done
- 用户价值或技术目标：让维护者和 Agent 能把较大需求稳定地组织为 Epic 与可执行子 Story，避免把多阶段工作当成一个模糊任务，或在迭代中遗漏依赖和完成边界。
- 范围：
  - 明确 Epic 与普通 Story 的判定标准、拆分维度和子 Story 细度。
  - 为 Evolith 建立保留 `EVO-*` 前缀的父子编号、依赖记录、DoR 与迭代选取规则。
  - 在文档一致性检查与经验记录中加入相应防呆。
  - 将可迁移的方法论同步到 `agent-project-governance` skill，不覆盖该仓库已有外部改动。
- 不做：
  - 不批量重编号历史 `EVO-*` 事项；历史拆分关系继续保留。
  - 不改业务代码、构建流程或部署配置。
  - 不自动为现有 Proposed 事项建立新的 Epic 层级。
- 验收标准：
  - [x] `REQUIREMENT-INTAKE.md` 定义 Epic 判定、拆分维度/粒度、父子编号、依赖校验、Epic/子 Story DoR 和跨 Epic 迭代规则。
  - [x] `START-ITERATION.md`、`ITERATION-WORKFLOW.md` 与 `DOC-CHECK.md` 承接父子和依赖检查。
  - [x] `agent-project-governance` skill 提供可按项目编号前缀适配的 Epic/Story 方法论与评估场景。
  - [x] Markdown 链接校验和 `git diff --check` 通过，skill 结构校验通过。
- 依赖或阻塞：无；skill 目标目录存在用户未提交的 `README.md` 变更，实施时不得改写该文件。
- 影响范围：docs / external skill
- 最小验证方式：执行文档链接校验；`git diff --check`；运行 skill 的 `quick_validate.py`。
