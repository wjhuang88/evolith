# EVO-038 本项目实施任务闭环 SOP 与完成声明门禁

> Archived backlog item. Routing index: [2026 Q2 Archive](INDEX.md). Active routing surface: [Product Backlog](../../PRODUCT-BACKLOG.md).

- Type: tech-debt
- Status: Done
- Priority: P1
- Source: 用户反馈 2026-05-27
- Decision Context: Iteration 015；将闭环协议落实到 Evolith 自身流程

#### Source Detail Snapshot

- 类型：tech-debt
- 优先级：P1
- 状态：Done
- 用户价值或技术目标：让 Evolith 自身的 Agent 执行流程具备统一、可机械遵循的
  收口门禁，避免实现或文档更新完成一部分后遗漏状态、验证、残余登记就宣称完成。
- 范围：
  - 新增通用任务收口 SOP，定义闭环台账、执行阶段和 `Complete / Partial / Blocked`
    完成声明条件。
  - 将入口约束、迭代推进、变更控制、结对审查、Git 与文档检查路由到收口门禁。
  - 更新 iteration 模板，使迭代记录显式承载闭环责任和完成结论。
  - 将已写入的闭环经验扩展到本项目 SOP 落地结论。
- 不做：
  - 不改业务代码、测试实现或部署配置。
  - 不重复实现专业测试、发布、数据库或 API SOP 已拥有的检查细节。
  - 不继续修改外部 skill；该部分已在 EVO-037 完成。
- 验收标准：
  - [x] `TASK-CLOSURE.md` 定义实施任务开始前建账、结束前核验/同步/交付的固定步骤。
  - [x] `AGENTS.md` 与相关 SOP/模板均能将 Agent 引导到通用收口门禁。
  - [x] 完成声明不能绕过验证结果、状态同步或残余工作登记。
  - [x] Markdown 链接检查和 `git diff --check` 通过。
- 依赖或阻塞：EVO-037 已完成外部 skill 的闭环协议；本轮将同类规则本地化。
- 影响范围：docs
- 最小验证方式：执行文档相对链接检查；`git diff --check`。
