# Iteration 022: 敏捷实践与 BDD 验收格式适配规则

> 文档状态：Closed
> 计划发布日期：2026-05-28
> 计划目标：明确 Evolith iteration 与传统 Sprint、Story 格式与 BDD 验收的结合方式。
>
> 基线保护：本文件一旦提交，以下“发布计划基线”内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

- 建立项目级方法论：Evolith 借鉴 Scrum Sprint 的小批次和复盘，但 iteration 是可审计
  工作批次，不机械采用完整 Scrum 仪式。
- 优化用户故事格式：按 Product / API / Technical / Governance / Spike 区分写法和必填字段。
- 明确 BDD 适用边界：行为类工作使用 Given/When/Then；技术、治理和 Spike 使用等价
  技术验收。
- 将规则接入需求准入、迭代计划、文档检查、模板、入口约束和经验记录。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|-----------|--------|---------------|
| EVO-041 | 敏捷实践与 BDD 验收格式适配规则 | 无 | P1 | 作为治理修复插队；不激活产品 planned iteration |

### Iteration Inventory Disposition

| Iteration | 当前状态 | 处置结论 |
|-----------|----------|----------|
| Iteration 009 | Closed | EVO-040 已完成收口；不阻塞本轮治理改进 |
| Iteration 010 | Closed | EVO-040 已完成收口；不阻塞本轮治理改进 |
| Iteration 012 | Planned / Blocked | 继续保持 EVO-016 refinement 阻塞；本轮不激活 |
| Iteration 017 | Planned / Blocked | 继续保持激活阻塞；本轮不激活 |
| Iteration 018 | Planned / Blocked | 继续按 Phase E 依赖链保留；本轮不激活 |
| Iteration 019 | Planned / Blocked | 继续依赖 Iteration 018；本轮不激活 |
| Iteration 020 | Planned / Blocked | 继续依赖 Iteration 019；本轮不激活 |

## 3. 发布计划基线：不做事项

- 不引入完整 Scrum 仪式、团队容量统计、燃尽图或固定冲刺承诺。
- 不修改业务代码、测试代码、CI/CD 或部署策略。
- 不激活 Iteration 012 / 017 / 018 / 019 / 020。
- 不提交外部 `agent-project-governance` skill；仅按用户要求同步内容。

## 4. 发布计划基线：计划验收标准

- Story 格式与 BDD 适用性：
  - [x] 每个候选 Story 已标明 Product / API / Technical / Governance / Spike 形态。
  - [x] 行为类 Story 使用 Given/When/Then 场景，或记录 BDD 不适用原因。
  - [x] 技术、治理或 Spike 使用等价技术验收、状态归口和残余归口。
- [x] `REQUIREMENT-INTAKE.md` 定义 Story 格式规范、BDD 适用边界与等价技术验收。
- [x] `ITERATION-WORKFLOW.md` 定义 Evolith iteration 与传统 Sprint 的关系。
- [x] `DOC-CHECK.md` 提供 Story / BDD 一致性检查。
- [x] `ITERATION-TEMPLATE.md` 承接 Story/BDD 计划验收与闭环归口。
- [x] `AGENTS.md` 和 `EVOLUTION.md` 写入入口约束与经验。
- [x] `agent-project-governance` skill 同步体现该方法论。

## 5. 发布计划基线：计划验证

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

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| 机械套用 Scrum 导致流程过重 | 明确 iteration 是可审计工作批次，不强制完整 Scrum 仪式 |
| 所有任务都被硬写成用户故事 | 按 Story 形态区分格式，技术/治理/Spike 使用等价表达 |
| BDD 变成空泛模板 | 要求 Then 对应可观察行为或可验证状态，并映射验证证据 |
| 外部 skill 目录误提交 | 本轮仅同步内容，不在 Evolith 提交中包含外部仓库 |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 设计敏捷/Sprint/BDD 与 Evolith 迭代治理的结合方式，重点优化用户故事格式，并同步到治理 skill |
| 产物 | `REQUIREMENT-INTAKE.md`、`ITERATION-WORKFLOW.md`、`DOC-CHECK.md`、`ITERATION-TEMPLATE.md`、`AGENTS.md`、`EVOLUTION.md`、backlog、iteration、外部 skill references |
| 状态同步归口 | EVO-041、Iteration 022、Iteration README、EVOLUTION |
| Story/BDD 归口 | Story 格式规范、BDD 适用边界、等价技术验收、模板和文档检查 |
| 验证证据 | Markdown 链接检查；`git diff --check`；skill 结构校验 |
| 残余工作归口 | 外部 skill 只同步不提交；后续安装/发布由用户处理 |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-05-28 | activation | 作为治理修复插队；已记录既有 planned / blocked iteration disposition |
| 2026-05-28 | progress | 更新需求准入、迭代工作流、文档检查、模板、入口约束和 backlog |
| 2026-05-28 | progress | 按用户要求将 Story 格式规范作为重点，区分 Product / API / Technical / Governance / Spike |
| 2026-05-28 | progress | 同步 `agent-project-governance` skill 的方法论与评估用例 |
| 2026-05-28 | validation | `python3` Markdown 链接检查：通过，`all markdown links exist` |
| 2026-05-28 | validation | `git diff --check`：通过 |
| 2026-05-28 | validation | skill Markdown 链接检查：通过，`all skill markdown links exist` |
| 2026-05-28 | validation | skill `git diff --check`：通过 |
| 2026-05-28 | validation | skill 结构检查：通过，`skill structure check passed` |
| 2026-05-28 | validation | `validate_project_governance.py`：失败，缺少 `.agent-governance/manifest.yaml`；该残余已由 EVO-035 归口，不阻塞本轮 Story/BDD 方法论改造 |
| 2026-05-28 | completion | EVO-041 文档与 skill 同步完成；外部 skill 不在本轮 Evolith 提交中提交 |

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|
| 2026-05-28 | scope-change | 接受 | 用户明确要求同步 `agent-project-governance` skill | 将 EVO-041 范围从本项目文档扩展为 docs / external skill，不提交外部 skill |
| 2026-05-28 | clarification | 接受 | 用户要求重点优化用户故事格式规范 | 扩展 Story 格式字段、推荐写法、质量标准和反例 |

## 10. Review

- 完成：
  - 新增 EVO-041，定义 Sprint / iteration / Story / BDD 的项目适配规则。
  - 更新需求准入、迭代工作流、文档检查、迭代模板、入口约束和经验记录。
  - 将重点放到用户故事格式规范：Story 形态、标准字段、推荐写法、质量标准和反例。
  - 同步 `agent-project-governance` skill 的入口、方法论 reference、Story/BDD reference 和评估用例。
- 未完成：
  - 未提交外部 skill 仓库；按用户此前约束，skill 目录由用户自行处理提交。
  - 未补 `.agent-governance/manifest.yaml`；已由 EVO-035 归口。
- 验证结果：
  - Markdown 链接检查：通过。
  - `git diff --check`：通过。
  - skill Markdown 链接检查：通过。
  - skill `git diff --check`：通过。
  - skill 结构检查：通过。
  - `validate_project_governance.py` 对 Evolith 失败，原因是缺少 manifest；这是既有 EVO-035 范围。
- 闭环状态：`Complete`
- 残余归口：
  - 外部 skill 提交/发布不在本轮 Evolith 提交范围内。
  - Evolith manifest 接入和 bundled validator 完整通过归口 EVO-035。

## 11. Retrospective

- 做得好的：
  - 将 Sprint、iteration、Story 和 BDD 拆成可执行规则，而不是抽象讨论。
  - Story 格式规则覆盖行为、技术、治理和 Spike，避免假用户故事。
- 需要调整的：
  - 后续评审需求时优先看 Story 格式和验收可验证性，再看排期。
- 写入 EVOLUTION：
  - 已写入 `2026-05-28 Story 格式要按任务性质分型，而不是机械套用户故事`。
