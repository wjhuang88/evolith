# EVO-082 evolution feedback SOP 缺失导致治理 validator 失败修复

> Archived backlog item. Routing index: [2026 Q2 Archive](INDEX.md). Active routing surface: [Product Backlog](../../PRODUCT-BACKLOG.md).

- Type: governance
- Status: Done
- Priority: P1
- Source: 本轮治理验证 2026-06-05
- Decision Context: 补 `docs/sop/EVOLUTION-FEEDBACK.md` 并从 AGENTS/docs README 路由，恢复 governance validator

#### Source Detail Snapshot

- 类型：governance
- 优先级：P1
- 状态：Done
- 父 Epic：无
- Story 形态：Governance
- 失败模式：
  - `.agent-governance/manifest.yaml` 将 `evolution_feedback` 标为 conformant，但仓库缺少 `docs/sop/EVOLUTION-FEEDBACK.md`，且 `AGENTS.md` 未路由该 SOP。
  - governance validator 因此失败，阻塞本轮收口。
- 范围：
  - 新增 `docs/sop/EVOLUTION-FEEDBACK.md`。
  - 更新 `AGENTS.md` 的经验写回规则和 Task Router。
  - 更新 `docs/README.md` 文档地图。
- 不做：
  - 不重构 `EVOLUTION.md` 历史内容。
  - 不修改治理 skill validator。
- 验收标准：
  - [x] `docs/sop/EVOLUTION-FEEDBACK.md` 存在。
  - [x] `AGENTS.md` 包含 `docs/sop/EVOLUTION-FEEDBACK.md` 路由。
  - [x] governance validator 通过。
- 依赖或阻塞：无。
- 解锁内容：恢复项目治理声明与实际文档一致。
- 影响范围：docs / AGENTS.md。
- 最小验证方式：governance validator；Markdown 链接检查；`git diff --check`。
