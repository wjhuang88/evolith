# Iteration 037: 最近开发任务治理漂移修复

> 文档状态：Closed（2026-06-04）
> 计划发布日期：2026-06-04
> 计划目标：修复近期开发任务在 backlog、iteration 目录、派生看板和文档分层之间的治理漂移。
>
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 目标

- 对齐 `docs/BOARD.md` 与 owner docs 状态，移除已关闭/已完成事项的活跃口径。
- 修复 `docs/iterations/README.md` 中过期的未来候选和下一周建议。
- 修复 `Iteration 029` 顶部状态与正文收口记录不一致。
- 删除 `Iteration 033` 重复复盘段落。
- 将 Figma 设计系统文档从仓库根目录归入 `docs/reference/`。
- 修复 `docs/reference/MULTI-TENANT.md` 的 migrations 相对链接。
- 补齐 EVO-048 输出中已被引用但缺失的 `EVO-045-A` 子任务。

## 2. 候选故事与依赖

| ID | 标题 | 类型 | 状态 | 启动条件/依赖 |
|----|------|------|------|---------------|
| EVO-078 | 最近开发任务治理漂移修复 | governance | Done | 用户要求按审计建议实施 |

## 3. 不做事项

- 不修改 Figma 导入产生的前端代码。
- 不回滚近期提交。
- 不启动 Iteration 034。
- 不覆写已发布 iteration 的计划基线。
- 不将 `docs/BOARD.md` 作为状态源。

## 4. 验收标准

- [x] `docs/BOARD.md` 与 backlog / iteration owner docs 不冲突。
- [x] `docs/iterations/README.md` 不再把 Iteration 033 作为未来候选，不再建议 EVO-060。
- [x] `docs/iterations/ITERATION-029.md` 顶部状态为 Closed。
- [x] `docs/iterations/ITERATION-033.md` 只保留一个 `## 11. Retrospective`。
- [x] `docs/reference/DESIGN.md` 存在，根目录 `DESIGN.md` 不存在。
- [x] `docs/reference/MULTI-TENANT.md` migrations 链接可解析。
- [x] `EVO-045-A` 已补入 backlog，并保持 `Ready`。
- [x] 文档链接检查、治理 validator 和 `git diff --check` 通过。

## 5. 验证

```bash
python3 - <<'PY'
from pathlib import Path
import re

root = Path('.')
missing = []
skip_parts = {'.git', 'node_modules', '.opencode'}
for path in root.rglob('*.md'):
    if any(part in skip_parts for part in path.parts):
        continue
    text = path.read_text(encoding='utf-8')
    for match in re.finditer(r'\[[^\]]+\]\(([^)]+)\)', text):
        target = match.group(1).strip()
        if target.startswith(('http://', 'https://', 'mailto:', '#')):
            continue
        if target.startswith('/'):
            continue
        file_part = target.split('#', 1)[0]
        if not file_part:
            continue
        if not (path.parent / file_part).exists():
            missing.append(f'{path}:{match.start()}: {target}')

if missing:
    print('\n'.join(missing))
    raise SystemExit(1)
print('project markdown links ok')
PY

test -f docs/reference/DESIGN.md
test ! -f DESIGN.md
sh /Users/GHuang/WorkSpace/AiProjects/skill-sources/agent-project-governance/skills/agent-project-governance/scripts/validate_project_governance.sh /Users/GHuang/WorkSpace/AiProjects/evolith
git diff --check
```

## 6. 执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-06-04 | audit | 检查近期提交发现 Figma 设计系统变更无 backlog / iteration 归口、EVO-060 已 Done 但 Board/iteration README 仍列为候选、EVO-048 已 Closed 但 Board 仍为 Now、EVO-045-A 被建议但 backlog 缺失、Iteration 029 顶部状态仍 Active、Iteration 033 重复复盘。 |
| 2026-06-04 | repair | 更新 backlog、iteration 目录、Iteration 029/033、Board 和 docs README；将 `DESIGN.md` 移到 `docs/reference/DESIGN.md`；修复 `MULTI-TENANT.md` migrations 断链。 |
| 2026-06-04 | validation | 文档链接检查通过；设计文档归位检查通过；governance validator 通过；`git diff --check` 通过。 |

## 7. Review

- 完成：
  - `EVO-078` 记录本次治理修复并置为 Done。
  - `EVO-045-A` 作为 EVO-048 输出后的 Ready 子任务补齐。
  - `docs/iterations/ITERATION-029.md` 顶部状态同步为 Closed。
  - `docs/BOARD.md` 同步为派生运营视图，不再显示过期 Now / Next。
  - `docs/iterations/README.md` 的未来计划和下一周建议与 owner docs 对齐。
  - `docs/iterations/ITERATION-033.md` 去重复复盘。
  - `docs/reference/DESIGN.md` 纳入文档地图。
  - `docs/reference/MULTI-TENANT.md` migrations 相对链接修复。
- 未完成：
  - Figma 设计系统代码变更本身未重新评审；若要调整视觉实现，应另建 story。
- 闭环状态：`Complete`
- 残余归口：
  - 继续产品主线：Iteration 034。
  - 执行架构关键路径：EVO-045-A。
  - P2 健康修复：EVO-051 / 052 / 056 / 057 / 058。

## 8. Retrospective

- 做得好的：
  - 先修 backlog / iteration owner docs，再同步 Board，避免派生看板成为第二状态源。
  - 将最近偏离统一收口到一个治理微迭代，避免继续零散修文档。
- 需要调整的：
  - 之后导入设计系统或做大范围样式变更前，应先进入 backlog 并建立 iteration 记录。
- 写入 EVOLUTION：本次是既有治理规则的执行修复，未发现新的稳定陷阱。
