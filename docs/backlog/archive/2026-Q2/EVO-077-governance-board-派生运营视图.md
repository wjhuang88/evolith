# EVO-077 Governance board 派生运营视图

> Archived backlog item. Routing index: [2026 Q2 Archive](INDEX.md). Active routing surface: [Product Backlog](../../PRODUCT-BACKLOG.md).

- Type: governance
- Status: Done
- Priority: P1
- Source: 用户反馈 2026-06-03 / Iteration 036
- Decision Context: 按 agent-project-governance skill 标准新增 `docs/BOARD.md`，只汇总 owner docs 与 gate，不作为新状态源；验证通过并收口

#### Source Detail Snapshot

- 类型：governance
- 优先级：P1
- 状态：Done
- 父 Epic：无
- Story 形态：Governance
- 用户故事或技术目标：
  - 作为/为了：维护者和 Agent 需要一个快速判断当前工作流向的派生运营视图。
  - 我希望/需要：新增符合 agent-project-governance skill 的 `docs/BOARD.md`。
  - 以便：在不复制 backlog / iteration 状态源的前提下，快速回答 Now / Review / Blocked / Next / Later 和每项 gate。
- 范围：
  - 新增 `docs/BOARD.md`，标明派生视图规则。
  - 看板行仅包含 `Item / State / Owner Doc / Gate`。
  - 每行链接 owner doc，并写明 exit / resume / activation / deferral gate。
  - 同步 `docs/README.md` 文档地图和 `AGENTS.md` Session End Checklist。
  - 建立 `Iteration 036` 记录本次治理修复与迭代合理性评估。
- 不做：
  - 不创建前端看板页面或运行时代码。
  - 不把看板放进 `docs/backlog/`，不让看板成为第二个 backlog。
  - 不在看板里维护 story 详情、验收清单或执行日志。
  - 不关闭 `Iteration 033` 或改写 `Iteration 034` 的计划基线。
- 验收标准：
  - 非行为类：
    - [x] `docs/BOARD.md` 存在，并明确是 derived operating view。
    - [x] `docs/BOARD.md` 只使用 `Item / State / Owner Doc / Gate` 四列。
    - [x] 看板每条实际工作行都有 owner doc 链接和明确 gate。
    - [x] `docs/README.md` 链接 `docs/BOARD.md`。
    - [x] `AGENTS.md` Session End Checklist 包含 owner docs 先于 board 同步的检查项。
    - [x] `docs/BOARD.md` 不与 backlog / iteration README / active iteration 状态冲突。
    - [x] `Iteration 036` 记录本次插队治理修复和 Iteration 033/034 合理性评估。
    - [x] 文档链接检查、governance validator 和 `git diff --check` 通过。
- 技术备注：
  - Skill 标准结构将 board 定义为可选的 `docs/BOARD.md`，不是 backlog 子文档。
  - Board 只解决运营扫描问题；状态权威仍在 owner docs。
- 依赖或阻塞：无。
- 解锁内容：开始新迭代前可先扫 board，但仍必须按 START-ITERATION 扫描 owner docs。
- 影响范围：docs / AGENTS.md / .gitignore
- 最小验证方式：文档链接检查；`sh /Users/GHuang/WorkSpace/AiProjects/skill-sources/agent-project-governance/skills/agent-project-governance/scripts/validate_project_governance.sh /Users/GHuang/WorkSpace/AiProjects/evolith`；`git diff --check`。
