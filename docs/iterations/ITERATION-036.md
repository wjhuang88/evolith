# Iteration 036: Governance board operating view

> 文档状态：Closed
> 计划发布日期：2026-06-03
> 计划目标：按 agent-project-governance skill 标准新增派生运营看板，帮助 Agent 快速判断
> Now / Review / Blocked / Next / Later，同时复核当前迭代安排是否合理。
>
> 基线保护：本文件一旦提交，以下“发布计划基线”内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

- 新增 [Operating Board](../BOARD.md)，作为派生运营视图，不成为新的状态源。
- 每个看板行必须链接 owner doc，并写明 exit / resume / activation / deferral gate。
- 同步文档地图与 Agent 会话检查项，避免 board 漂移。
- 复核当前 `Iteration 033` / `Iteration 034` / blocked planned iterations 的排期合理性。
- 忽略本地 Agent 工具目录，避免 `.codex/` / `.opencode/` 进入待提交列表。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|---------|--------|---------------|
| EVO-077 | Governance board 派生运营视图 | 无 | P1 | 治理修复插队；不关闭 Iteration 033，只同步 owner docs 后更新 board |

## 3. 发布计划基线：不做事项

- 不创建前端看板页面。
- 不把 board 放入 `docs/backlog/`，不建立第二个 backlog。
- 不在 board 中维护 story 详情、验收清单、执行日志或完整历史。
- 不关闭 `Iteration 033`，不改写 `Iteration 034` 已发布计划基线。
- 不处理 `.codex/` / `.opencode/` 目录内容本身，只更新 `.gitignore`。

## 4. 发布计划基线：计划验收标准

- Story 格式与 BDD 适用性：
  - [x] Governance Story；BDD 不适用，使用 owner doc、gate 和文档一致性验证。
- [x] `docs/BOARD.md` 存在，并明确是 derived operating view。
- [x] `docs/BOARD.md` 只使用 `Item / State / Owner Doc / Gate` 四列。
- [x] 每条实际工作行都有 owner doc 链接和明确 gate。
- [x] `docs/README.md` 链接 `docs/BOARD.md`。
- [x] `AGENTS.md` Session End Checklist 包含 owner docs 先于 board 同步的检查项。
- [x] `.gitignore` 忽略 `.codex/` 和 `.opencode/`。
- [x] board 状态不与 backlog、iteration README 和 active iteration 冲突。
- [x] 当前迭代合理性结论已记录：Iteration 033 继续 active；Iteration 034 合理保持 next；blocked planned iterations 继续 blocked。

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

sh /Users/GHuang/WorkSpace/AiProjects/skill-sources/agent-project-governance/skills/agent-project-governance/scripts/validate_project_governance.sh /Users/GHuang/WorkSpace/AiProjects/evolith
git diff --check
```

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| board 变成第二个 backlog | Board 标题和规则明确 derived view；每行只链接 owner doc 和 gate |
| 新治理修复绕过 active iteration | 在本迭代记录为治理修复插队，Iteration 033 保持 Active / In Progress |
| 状态漂移 | 先同步 PRODUCT-BACKLOG / iterations README / owner iteration，再同步 BOARD |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 建立项目治理看板文档，复核迭代是否合理，并顺带忽略本地 Agent 工具目录 |
| 产物 | `docs/BOARD.md`、`docs/README.md`、`AGENTS.md`、`docs/backlog/PRODUCT-BACKLOG.md`、`docs/iterations/ITERATION-036.md`、`docs/iterations/README.md`、`.gitignore` |
| 状态同步归口 | EVO-077、Iteration 036、docs README、AGENTS checklist、derived board |
| Story/BDD 归口 | Governance Story；BDD 不适用；以 owner doc 链接、gate、文档一致性和 validator 作为验收 |
| 验证证据 | Markdown 链接检查、governance validator、`git diff --check` |
| 残余工作归口 | 无；board 后续维护归 AGENTS checklist 和 docs README 规则 |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-06-03 | activation | 治理修复插队：用户要求项目治理看板文档并评估迭代合理性；Iteration 033 保持 Active / In Progress，不关闭、不改线。 |
| 2026-06-03 | change request | 用户澄清“看板”不是前端页面，而是项目治理看板文档。类型：scope-change；决策：接受并修正为 `docs/BOARD.md` 派生运营视图；半成品处理：删除误放的 `docs/backlog/KANBAN.md`，未创建前端代码。 |
| 2026-06-03 | validation | Markdown 链接检查通过：`all markdown links exist`。 |
| 2026-06-03 | validation | Governance validator 通过：`Governance validation passed: 0 warning(s).` |
| 2026-06-03 | validation | `git diff --check` 通过，无输出。 |
| 2026-06-03 | completion | EVO-077 状态 `In Progress` → `Done`；Iteration 036 `Active / Review` → `Closed`；`docs/BOARD.md` 移除已完成 EVO-077 行，只保留当前真实在途/阻塞/下一步工作。 |

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|
| 2026-06-03 | scope-change | 接受 | 从前端看板页修正为 governance skill 标准的 `docs/BOARD.md` | 删除误放的 `docs/backlog/KANBAN.md`；保留 `.gitignore` 本地工具目录忽略 |

## 10. Review

- 完成：新增派生 [Operating Board](../BOARD.md)；同步 `docs/README.md`、`AGENTS.md`、`PRODUCT-BACKLOG.md`、`docs/iterations/README.md` 和 `.gitignore`。
- 未完成：无。
- 验证结果：
  - `python3 ... markdown link check`：通过，输出 `all markdown links exist`。
  - `sh .../validate_project_governance.sh /Users/GHuang/WorkSpace/AiProjects/evolith`：通过，输出 `Governance validation passed: 0 warning(s).`
  - `git diff --check`：通过，无输出。
- 闭环状态：`Complete`
- 残余归口：无；board 后续维护归 `AGENTS.md` Session End Checklist。

## 11. Retrospective

- 做得好的：最终按 skill 标准将 board 放到 `docs/BOARD.md`，并保持派生视图边界。
- 需要调整的：执行治理 skill 任务时必须读到 `standard-structure.md` 的 artifact responsibility 后再落文件，避免凭项目既有目录直觉放错 owner。
- 写入 EVOLUTION：是，用户指出遗漏后补写经验。
