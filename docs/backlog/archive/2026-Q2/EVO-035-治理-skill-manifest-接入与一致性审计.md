# EVO-035 治理 skill manifest 接入与一致性审计

> Archived backlog item. Routing index: [2026 Q2 Archive](INDEX.md). Active routing surface: [Product Backlog](../../PRODUCT-BACKLOG.md).

- Type: tech-debt
- Status: Done
- Priority: P2
- Source: Iteration 008 验证残余
- Decision Context: Iteration 023；已建立 manifest 并通过 bundled validator

#### Source Detail Snapshot

- 类型：tech-debt
- 优先级：P2
- 状态：Done
- 用户价值或技术目标：让 Evolith 已有治理文档能被 `agent-project-governance` skill 明确识别为初始化/采用状态，并通过一致性审计发现后续漂移。
- 范围：
  - 按当前项目治理现状建立 `.agent-governance/manifest.yaml`。
  - 核对 capability 状态、标准入口与已有 SOP / reference / backlog / iteration 映射。
  - 运行 skill bundled validator，并将真实残余写回 backlog 或治理文档。
- 不做：
  - 不在 EVO-034 中补造 manifest 以掩盖附加审计失败。
  - 不顺带改业务逻辑或重写既有迭代历史。
- 验收标准：
  - [x] manifest 能准确表达 Evolith 的治理 profile、入口和能力状态。
  - [x] `validate_project_governance.py /Users/GHuang/WorkSpace/AiProjects/evolith` 通过，或将剩余问题拆为明确事项。
- 依赖或阻塞：EVO-034 已完成；本轮按当前治理状态采用 `high-risk / conformant` profile。
- 影响范围：docs
- 最小验证方式：运行 `agent-project-governance/scripts/validate_project_governance.py`。
