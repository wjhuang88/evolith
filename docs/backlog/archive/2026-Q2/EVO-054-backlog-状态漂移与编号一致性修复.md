# EVO-054 backlog 状态漂移与编号一致性修复

> Archived backlog item. Routing index: [2026 Q2 Archive](INDEX.md). Active routing surface: [Product Backlog](../../PRODUCT-BACKLOG.md).

- Type: bug
- Status: Done
- Priority: P1
- Source: 代码健康审查 2026-06-01 / Iteration 035
- Decision Context: 2026-06-01 完成：EVO-016-B 详情块 In Progress → Done、EVO-026 详情块 Ready → Done（额外漂移）、EVO-042 缺号登记 Dropped 行；详情块 100% 与总表一致

#### Source Detail Snapshot

- 类型：bug
- 优先级：P1
- 状态：Done
- 父 Epic：无
- Story 形态：Governance / Docs
- 用户价值或技术目标：
  - 为了：防止 backlog 总表与详情块状态不一致、编号断档导致 Agent 误判工作状态（本项目已有 EVO-040 同类漂移修复先例）。
  - 维护者需要：对齐 EVO-016-B 状态，并对 EVO-042 缺号给出明确处置记录。
  - 以便：backlog 作为可执行指令源保持自洽，符合 REQUIREMENT-INTAKE「总表 + 详情块一致」防呆规则。
- 范围（本次做）：
  1. 将 EVO-016-B 详情块（PRODUCT-BACKLOG.md 第 181 行）状态由 `In Progress` 改为 `Done`，与总表及 Iteration 031 收口结论一致。
  2. 登记 EVO-042 缺号处置：确认为有意跳过则在 backlog 加一行说明（status `Dropped`/说明），否则按需补占位。
  3. 按 DOC-CHECK 核对是否存在其他总表/详情块状态漂移。
- 不做：
  - 不改任何代码或运行时行为。
  - 不回填 EVO-042 为真实需求（仅做编号一致性处置）。
- 验收标准：
  - [ ] EVO-016-B 总表与详情块状态一致（均为 `Done`）。
  - [ ] EVO-042 在 backlog 有明确处置记录（跳过说明或占位）。
  - [ ] `rg "状态：In Progress" docs/backlog/PRODUCT-BACKLOG.md` 不再误标已完成项。
- 依赖或阻塞：无。
- 解锁内容：恢复 backlog 状态自洽，避免后续库存盘点误判。
- 影响范围：docs
- 最小验证方式：DOC-CHECK 一致性核对；总表与详情块逐项比对；`git diff --check`。
