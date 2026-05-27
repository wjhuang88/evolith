# SOP: 开始一次迭代

> 本 SOP 负责先处置既有未关闭或已规划 iteration，再决定是否把 Ready backlog 转入
> 一次可执行迭代。不要直接从剩余 story 开始选取。

## 触发条件

- 用户要求开始一个新迭代。
- 当前迭代结束后，需要选择下一批故事。
- 需要把一个 Ready backlog story 转为 `In Progress`。

## 前置检查

- [ ] 已运行 `git status --short --branch`，确认工作区状态。
- [ ] 已读取 [迭代目录](../iterations/README.md) 并扫描所有 iteration 文档状态，
      列出 `Active / In Progress / Review / Planned / Blocked` 项。
- [ ] 已为每个非终态 iteration 记录处置结论：继续、收口、激活、维持阻塞、延期、
      改线或允许本次治理/紧急修复插队。
- [ ] 已读取 [需求进入与 Backlog 整理](REQUIREMENT-INTAKE.md)，确认候选故事满足 DoR。
- [ ] 已读取 [特性迭代工作流](ITERATION-WORKFLOW.md)，确认 WIP 限制和 DoD。
- [ ] 当前没有未处置的 `Active / In Progress / Review` iteration，且 backlog story
      状态与 iteration 记录一致。
- [ ] 如用户在开始迭代时补充需求，已按 [迭代中需求变更](CHANGE-CONTROL.md) 分类记录。
- [ ] 如果目标编号已有已发布 `Planned` 文档，已核对实际选择是否仍与其计划基线同范围。
- [ ] 已读取 [任务收口与完成声明](TASK-CLOSURE.md)，并准备在 iteration 中记录闭环台账。

## Iteration 库存盘点门禁

当用户说“开始迭代”“继续下一轮”或要求选择下一批工作时，第一动作是查看
`docs/iterations/`，而不是扫描 backlog 剩余故事。为每个非终态 iteration 建立处置
记录：

| 既有状态 | 开始新工作前的必做处理 | 是否可直接从 backlog 新选 story |
|----------|------------------------|---------------------------------|
| `Active` / `In Progress` | 继续推进，或按变更控制明确暂停/切换/关闭理由 | 否，除非记录 urgent-fix 或治理修复插队 |
| `Review` | 完成验证和状态同步，或登记未完成/阻塞与后续归口 | 否，未处置前不得另开产品工作 |
| `Planned` | 判断是否应按原计划激活；若优先级改变，保留基线并记录延期/改线 | 仅在处置结论明确后 |
| `Blocked` | 复核阻塞是否仍成立、是否影响新工作，并记录继续阻塞或解锁条件 | 可，但必须先记录 disposition |
| `Closed` / `Deferred` | 无需阻塞启动；必要时只读核对依赖 | 是 |

如果 inventory 发现 iteration 和 backlog 状态冲突，先修正为真实状态或登记审查
缺口，再进行新的选取。`Done` stories 不足以证明包含它们的 iteration 已收口。

## 新 Story 选取规则

只有在上述 inventory disposition 完成，且没有必须继续处理的既有 iteration 后，
才执行以下规则：

1. 只从 [Product Backlog](../backlog/PRODUCT-BACKLOG.md) 中选择 `Ready` story。
2. 默认选择最高优先级且最靠近当前路线图主线的 story。
3. 如果多个 P0 并列，优先选择能解除后续阻塞的工程门禁或产品概念迁移。
4. 不从 `docs/proposals/` 直接开工；提案必须先晋升到 backlog。
5. 单个 Agent 会话默认只选 1 个 story，除非用户明确要求多故事迭代。
6. 如果候选项是 Epic，只能选择已拆出的 `Ready` 子 story；不要把整个 Epic 放入一次 Agent 微迭代。
7. 常规迭代可以跨 Epic 选择子 story，但必须有一致的迭代目标、满足 WIP 限制，并在计划中写明每项父 Epic。
8. 选择存在依赖关系的多个 story 时，前置项必须已 `Done`，或在本轮明确执行/验证顺序；不得选入循环依赖或未闭合依赖集合。

## 已发布计划基线保护

`docs/iterations/ITERATION-<N>.md` 一旦已提交并标记为 `Planned`，其中的计划目标、候选
story、范围、不做事项、计划验收、计划验证和风险即构成发布计划基线：

1. 实际启动仍服务于原计划目标时，保留基线内容，只在执行记录、实际验证、Review 和
   Retrospective 中追加事实；允许在页首更新执行状态，但不得把计划目标替换成执行结论。
2. 实际要处理另一组 story、另一目标或另一 Epic 时，不启动该编号；在原计划中追加
   `Deferred / Superseded / Blocked` 说明，并按最大编号新建 iteration 文档承载新工作。
3. 若依赖某计划的后续 iteration 已发布，而前置计划被延后或改线，必须将后续计划标注
   为 `Blocked for activation`，直至新的前置 iteration 明确完成。
4. 发现历史上已发生就地改线时，不删除实际执行证据；在原文档补回发布计划基线和
   流程偏差说明，并登记流程修复事项。

## 操作步骤

1. 盘点 `docs/iterations/ITERATION-*.md`，列出所有非终态 iteration 和其关联 backlog
   状态；在当前工作记录中写明 disposition。
2. 优先处理 disposition：
   - 可继续的既有 Active/Review iteration：继续或收口该文档，不新建编号；
   - 可激活的 Planned iteration：按原计划激活该文档；
   - 需延期/阻塞/改线的 Planned/Blocked iteration：先更新其记录和依赖影响；
   - 无可继续既有工作或已有明确插队结论：才进入下一步。
3. 如需新建迭代，再从 backlog 选取 `Ready` story，并按最大编号加 1 创建
   `docs/iterations/ITERATION-<N>.md`，优先使用
   [迭代模板](../iterations/ITERATION-TEMPLATE.md)。
4. 在激活或新建的迭代文件中写清楚：
   - 本轮目标；
   - 选入故事及其父 Epic（如有）；
   - 选入故事之间的依赖和执行顺序（如有）；
   - 不做事项；
   - 验收标准；
   - 验证计划；
   - 风险与回滚；
   - 闭环台账：请求结果、产物、状态同步归口、验证证据和残余工作归口；
   - 第一条执行记录。
5. 把实际选入 story 的 backlog 状态改为 `In Progress`，备注中写明迭代编号。
6. 更新 [迭代目录](../iterations/README.md)，加入新迭代或更新其非终态/阻塞说明。
7. 如果开始迭代时发生范围补充或优先级调整，在迭代文件 `变更请求` 中记录。
8. 运行文档一致性验证。
9. 在任何实现提交前，确保 iteration 与 backlog 已处于一致状态并包含验收与验证计划。紧急修复允许先止血，但必须在同一会话补记插队原因，不能在收尾提交中同时伪装“开始并已完成”。
10. 如果激活已发布 planned iteration，先执行“已发布计划基线保护”检查；目标不一致时必须改用新编号。
11. 进入实现前核对闭环台账已填；台账缺项时 iteration 不能声称具备完成路径。

## 验证

```bash
python3 - <<'PY'
from pathlib import Path
import re
missing=[]
for path in list(Path('docs').rglob('*.md'))+[Path('README.md'),Path('AGENTS.md'),Path('EVOLUTION.md')]:
    if not path.exists():
        continue
    text=path.read_text(encoding='utf-8')
    for m in re.finditer(r'\[[^\]]+\]\(([^)]+\.md)(?:#[^)]+)?\)', text):
        target=m.group(1)
        if '://' in target:
            continue
        p=(path.parent/target).resolve()
        if not p.exists():
            missing.append((str(path),target))
if missing:
    print('MISSING LINKS:')
    for src,target in missing:
        print(f'{src} -> {target}')
    raise SystemExit(1)
print('all markdown links exist')
PY

git diff --check
```

## 失败处理

- 候选 story 不满足 DoR：不要创建迭代；先回到 `REQUIREMENT-INTAKE.md` 补齐 backlog。
- 候选 story 只有总表行、没有详情块：先补用户价值、验收标准、依赖、影响范围和最小验证方式，再开始迭代。
- 候选 story 是父级 Epic：先拆出 0.5-2 天子 story，并让子 story 满足 DoR。
- 候选子 story 的依赖未完成或出现循环：保持 `Proposed` / `Blocked`，先调整依赖或完成前置项。
- 已有 `In Progress` story：先结束、暂停或记录切换原因，再开始新迭代。
- 存在 `Active / In Progress / Review` iteration：先继续、收口或明确插队结论，
  不得仅因 backlog 看不到在途 story 就创建新迭代。
- 存在 `Planned / Blocked` iteration：先记录激活、继续阻塞、延期或改线结论；
  未处置前不得直接转向 backlog 选新工作。
- iteration 与 backlog 状态不一致：以证据为准修正状态；无法确认完成证据时转入
  `Review`，不得直接关闭。
- 忘记同步 backlog：先补 backlog 状态，再继续。
- 用户开始迭代时改变范围：按 `CHANGE-CONTROL.md` 记录，不要直接覆盖原计划。
- 断链或目录未更新：先修复链接和 `docs/iterations/README.md`，再提交。
- 实现提交已经出现但 iteration 尚未开始：将其记录为流程偏差或 urgent-fix，先补审查与验收证据，不得直接补写为无异常完成。
- 已发布计划被改造成不同目标的执行记录：保留实际证据，补回原计划基线并登记偏差；后续不得继续复用该编号承载原计划。
- iteration 已开始但没有闭环台账：按 `TASK-CLOSURE.md` 补写产物、状态、证据和残余归口，再推进实现或收尾。

## 相关文档

- [需求进入与 Backlog 整理](REQUIREMENT-INTAKE.md)
- [特性迭代工作流](ITERATION-WORKFLOW.md)
- [迭代中需求变更](CHANGE-CONTROL.md)
- [任务收口与完成声明](TASK-CLOSURE.md)
- [Product Backlog](../backlog/PRODUCT-BACKLOG.md)
- [迭代目录](../iterations/README.md)
