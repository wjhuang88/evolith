# SOP: 需求进入与 Backlog 整理

> 本 SOP 负责把想法、缺陷、技术债和提案整理成可执行 backlog。Agent 只能从满足 DoR 的 backlog 条目开工。

## 触发条件

- 用户提出新需求、新想法、缺陷修复或技术债。
- `docs/proposals/` 中的提案准备进入实施。
- 需要拆分、排期或调整 `docs/backlog/PRODUCT-BACKLOG.md`。
- 迭代开始前需要确认候选故事是否 Ready。
- 当前迭代期间用户提出新想法，但它不改变当前 `In Progress` story 的验收标准或实现方向。

> 如果新输入会改变当前 `In Progress` story 的验收标准、领域概念、优先级或交付范围，改走 [迭代中需求变更](CHANGE-CONTROL.md)。如果只是给未来新增任务，不要污染当前 iteration 的 change request，按本文进入 backlog 或 proposal。

## 输入分流

| 输入类型 | 放置位置 | 处理方式 |
|----------|----------|----------|
| 远期想法、方向探索、范围不清 | `docs/proposals/` | 写成提案或补充现有提案；Agent 不直接开工 |
| 可在 0.5-2 天内完成的功能/缺陷/技术债 | `docs/backlog/PRODUCT-BACKLOG.md` | 补齐必填字段，满足 DoR 后才可排期 |
| 阻塞运行、安全或数据损坏问题 | `docs/backlog/PRODUCT-BACKLOG.md` | 标记 P0，可插队；事后补齐记录 |
| 重大技术或产品取舍 | `docs/decisions/` | 先写 ADR，再把可执行部分拆入 backlog |

## Backlog 必填字段

新增 backlog 条目最少包含：

- ID
- 标题
- 用户价值或技术目标
- 类型：feature / bug / chore / tech-debt / spike
- 优先级：P0 / P1 / P2 / P3
- 状态
- 验收标准
- 依赖或阻塞
- 影响范围：backend / frontend / db / docs / deploy
- 最小验证方式

## Backlog 结构规则

`docs/backlog/PRODUCT-BACKLOG.md` 采用“总表 + 详情块”结构：

| 区域 | 责任 | 防呆规则 |
|------|------|----------|
| 当前需求池总表 | 路由、排序、状态和来源索引 | 每个 backlog item 必须有一行；备注只写一句关键状态，不写完整需求 |
| 待细化故事详情块 | 用户价值、验收标准、依赖、影响范围和验证方式 | `Ready`、`In Progress`、`Review`、`Done` 的 story 必须有详情块或明确链接到等价文档 |
| Proposal / ADR / Roadmap | 背景、远期方向和重大取舍 | 不替代 backlog 详情；进入实施前仍要有 backlog item |

`Proposed` item 可以先只有总表行；晋升 `Ready` 前必须补齐详情块。Agent 选择任务时不得只凭总表备注开工。

## Epic / Story 方法论

### 定义与判定

| 类型 | 用途 | 进入迭代方式 | 完成判断 |
|------|------|--------------|----------|
| Story | 一次可独立验收的用户价值、风险降低或工程结果；通常在 0.5-2 天内交付 | 满足 Story DoR 后直接选入 | 验收标准和验证证据完成 |
| Epic | 组织多个 Story 的目标容器；本身不是实现批次 | 不直接选入；只能选择其 Ready 子 Story | 必需子 Story 已 Done，或明确 Deferred / Dropped 并说明目标是否仍达成 |

满足下列任一条件时，应将事项作为 Epic，而不是勉强写成一个 Story：

1. 明显超过 2 天交付窗口，或无法在一次小批次内验证。
2. 包含两个或更多可分别验收的结果，例如“构建入口”“部署切换”“遗留清理”。
3. 必须按先后顺序推进基础、迁移、集成、发布或清理阶段。
4. 风险或未知项需要先用 spike / 安全基线 / 兼容验证解锁后续实现。
5. 跨越迭代或路线图阶段，单一 `Done` 无法真实表达完成边界。

一个 Story 可以触碰多层代码，只要它仍是一个独立可验证结果；不要仅因同时修改 backend 与 frontend 就机械建 Epic。

### 拆分优先级与粒度

按以下顺序选择拆分维度：

1. **端到端价值切片**：优先拆成用户或调用方能验证的纵向结果。
2. **风险先行切片**：未知技术、权限/数据/迁移风险先拆成有时间盒且能给出结论的 Story。
3. **交付阶段或依赖边界**：基础能力、兼容迁移、集成切换、清理/收尾可以成为有依赖关系的 Story。
4. **模块或层级**：只有该模块产出可独立验证或明确解锁下一步时才单拆。

不要以团队归属、目录边界或“前端一项/后端一项”作为首选拆分方式；如果两半单独都无法验收，它们仍是同一 Story 的实现工作。

子 Story 应满足：

- 只有一个可清晰描述的交付结果。
- 通常在 0.5-2 天内完成并验证。
- 可独立审查、合并和记录状态。
- 有验收标准、最小验证方式和明确不做范围。
- 依赖其他子项时，能说明自己在依赖完成后独立提供什么结果。

### 编号与父子标识

新建 Epic 家族采用同一项目编号前缀：

```text
EVO-120       Epic 父项
EVO-120-A     子 Story：第一个独立切片
EVO-120-B     子 Story：第二个独立切片
```

- Evolith 固定使用 `EVO-` 前缀；其他项目可替换项目前缀，不照搬 `PB-` 或 `EVO-`。
- 父项详情块必须列出子 Story；子项详情块必须写 `父 Epic：EVO-120`。
- 已有独立 ID 或历史拆分关系不为统一格式而重编号。例如 `EVO-002` 已拆为 `EVO-021` 至 `EVO-025`，继续作为历史有效关系保留。
- 现有 Story 后续扩大成 Epic 时，保留原 ID 作为父项，从下一切片开始创建后缀子 Story。
- 默认只使用父项和一层子项。子 Story 再度过大时应重新 refinement，避免深层编号树掩盖范围失控。

### 依赖管理与校验

Epic 详情块至少维护下列子项表：

```markdown
| 子 Story | 独立结果 | 状态 | 依赖 | 所属迭代 |
|----------|----------|------|------|----------|
| EVO-120-A | ... | Ready | 无 | - |
| EVO-120-B | ... | Proposed | EVO-120-A | - |
```

每个子 Story 的详情块还必须记录 `父 Epic`、`依赖或阻塞` 和 `解锁内容`。依赖检查规则：

1. 依赖必须指向具体 Story、外部条件或 ADR，不写模糊的“前置完成后”。
2. 有未完成硬依赖的子 Story 不得标为 `Ready` 或 `In Progress`；应保持 `Proposed` 或 `Blocked`。
3. Refinement 和迭代开始时检查依赖无循环，且选入集合具有依赖闭包：前置项已 Done，或在同一迭代中明确先执行、先验证。
4. 子项状态变化时同步父项子项表；必需子项被 Deferred / Dropped 时说明 Epic 目标是否缩减或失效。

### Epic 与 Story 的 DoR

| 项目 | Epic 可进入路线图/细化队列 | 子 Story 可进入迭代 |
|------|----------------------------|--------------------|
| 目标 | 明确总体结果、边界和为何需要拆分 | 明确单一价值或技术目标 |
| 拆分 | 至少列出首批子 Story 或明确下一步拆分活动 | 无需再拆，符合 0.5-2 天窗口 |
| 验收 | 定义父项完成条件和必需/可选子项口径 | 有可执行验收标准和最小验证方式 |
| 依赖 | 有阶段、风险与外部依赖图 | 硬依赖已完成，或同迭代顺序已获确认 |
| 状态 | 可以是 `Proposed` / `In Progress`，但不直接开发 | 满足下方 Story DoR 后才可 `Ready` |

### 迭代选取规则

1. 迭代只选 Story，不选裸 Epic。
2. 单个 Agent 微迭代默认只选一个 Ready Story。
3. 一个常规迭代可以选取来自多个 Epic 的子 Story，前提是它们各自满足 DoR、依赖兼容、迭代目标仍然连贯且不突破 WIP 限制。
4. 同一 Epic 的多个子 Story 可以在同一迭代推进，但必须记录顺序；存在硬依赖时不得并行声称完成。
5. 子 Story 完成只更新自身状态和父项子项表；除非父项完成条件满足，不得顺手把 Epic 标成 `Done`。

## Proposal 晋升

Evolith 的执行者包括 AI Agent。Agent 从 Backlog 选取任务时会直接开工，无法自行判断需求是否足够成熟。因此采用两级结构：

- `docs/proposals/`：想法暂存区。Agent 不从中选取任务。
- `docs/backlog/`：可执行指令源。所有条目必须满足 DoR。

远期想法满足以下条件后才能迁入 Backlog：

1. 有明确的用户价值或技术目标。
2. 范围足够小，0.5-2 天可完成；过大则先拆分。
3. 有至少一条可验证的验收标准。
4. 依赖已识别。
5. 需要改哪些层已明确：backend / frontend / db / docs / deploy。

晋升操作：

1. 在 Backlog 新增条目。
2. 在 Proposal 标注 `已晋升`，或保留为背景参考。
3. 如果提案被 ADR 替代，补充 ADR 链接和替代关系。
4. 更新相关 roadmap 的归口映射。

## Backlog Refinement

每次开始迭代前检查：

- 故事是否足够小。
- 验收标准是否可验证。
- 是否需要 API 合约、数据库 migration、前端页面或部署变更。
- 是否需要 spike 先验证技术路线。
- 是否有明确测试策略。
- 是否与已有 ADR、roadmap、proposal 冲突。

满足 Definition of Ready 后，才允许进入迭代。

## Definition of Ready

本节是 Story 的进入迭代标准。Epic 的准备度按上方分层表判断，不能用 Epic 替代待实现 Story：

- [ ] 有清晰用户价值或技术目标。
- [ ] 有验收标准。
- [ ] 依赖明确。
- [ ] 能在 0.5-2 天内完成，或已拆分。
- [ ] 知道需要改哪些层：backend / frontend / db / docs / deploy。
- [ ] 有最小验证方式。
- [ ] 如果涉及重大取舍，已有 ADR 或明确说明不需要 ADR。

## 状态定义

| 状态 | 含义 |
|------|------|
| Proposed | 新进入，尚未整理 |
| Ready | 已满足 DoR，可排期 |
| In Progress | 正在实现 |
| Review | 已实现，等待验证或审查 |
| Done | 验收完成 |
| Blocked | 依赖未解决 |
| Deferred | 暂缓 |
| Dropped | 明确放弃 |

## 标准记录模板

```markdown
### EVO-XXX <Story 标题>

- 类型：feature / bug / chore / tech-debt / spike
- 优先级：P0 / P1 / P2 / P3
- 状态：Proposed / Ready / In Progress / Review / Done / Blocked / Deferred / Dropped
- 父 Epic：（非子 Story 填无）
- 用户价值或技术目标：
- 验收标准：
  - [ ] 
- 依赖或阻塞：
- 解锁内容：
- 影响范围：backend / frontend / db / docs / deploy
- 最小验证方式：
```

Epic 详情块在上述基础上增加：

```markdown
- 类型：epic
- Epic 完成条件：
- 拆分理由：
| 子 Story | 独立结果 | 状态 | 依赖 | 所属迭代 |
|----------|----------|------|------|----------|
| EVO-XXX-A |  | Proposed | 无 | - |
```

## 失败处理

- 需求范围不清：放入 `docs/proposals/`，不要伪装成 Ready backlog。
- 条目过大：拆成多个 story，或先建 spike。
- 验收标准不可验证：保持 `Proposed`，补清楚后再排期。
- 发现与 ADR 冲突：先更新 ADR 或补替代关系，再调整 backlog。

## 相关文档

- [Product Backlog](../backlog/PRODUCT-BACKLOG.md)
- [提案目录](../proposals/README.md)
- [决策记录](../decisions/README.md)
- [特性迭代工作流](ITERATION-WORKFLOW.md)
