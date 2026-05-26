# SOP: 文档一致性检查

## 触发条件

- 移动、重命名或新增文档目录。
- ADR 替代旧概念、旧模块或旧路线。
- proposal 晋升为 backlog，或 backlog 被 proposal/ADR 替代。
- Agent 完成文档整理、路线图更新或流程改造。

## 必查项

| 检查项 | 防呆规则 |
|--------|----------|
| Markdown 链接 | 所有相对 `.md` 链接必须存在 |
| 目录边界 | 远期想法只能在 `docs/proposals/`，可执行任务必须在 `docs/backlog/` |
| 旧术语 | ADR 替代旧概念后，旧术语只能出现在“迁移参考/兼容说明/历史实现”语境 |
| Backlog 状态 | 只能使用 SOP 定义的状态 |
| Proposal 状态 | 必须使用 `远期想法` / `待整理` / `远期目标` / `已晋升` / `已废弃` |
| 替代关系 | 被替代故事必须标注 `Deferred`、`Dropped` 或 `superseded/replaces` |
| Epic 父子关系 | 新 Epic 采用 `<前缀>-NNN` / `<前缀>-NNN-A`；父项列子项，子项反向标父项；历史 ID 可显式保留 |
| Story 依赖 | Ready / In Progress 子项不得有未处理硬依赖；同迭代依赖需记录顺序，父项汇总状态必须同步 |
| AGENTS 入口 | 新 SOP 或关键入口必须出现在 Task Router 或 docs README |
| 验证记录 | 迭代文档必须记录实际执行过的验证命令和结果 |

## 标准命令

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

## 概念替代检查

当 ADR 声明 A 替代 B 时：

1. 搜索旧概念 B。
2. 对每个命中位置判断：
   - 当前实现事实：可以保留，但必须标明 legacy/current implementation。
   - 迁移参考：可以保留。
   - 新路线/新需求：必须改成 A。
3. 更新 backlog 的替代关系。
4. 更新 roadmap 的执行顺序和进入条件。
5. 更新 proposals 的前置依赖。
6. 在相关 backlog item 的验收标准或 DoD 中加入旧术语搜索命令，直到迁移完成。

旧术语命中必须分类：

| 分类 | 是否允许 | 处理 |
|------|----------|------|
| 用户可见新功能文案 | 不允许 | 改成新概念 |
| 新 roadmap / backlog 目标 | 不允许 | 改成新概念，或新增替代关系 |
| 当前实现事实 | 允许 | 标注 legacy / compatibility / current implementation |
| 历史迭代、ADR 背景、迁移参考 | 允许 | 保留，但避免作为下一步目标 |

示例：

```bash
rg -n "snippet|Snippet|片段" docs AGENTS.md README.md
```

如果旧概念仍存在前端页面或 API client，必须新增或更新 backlog item，写清楚迁移范围、依赖和验证方式；不要只在对话中说明“后续处理”。

## Proposal 晋升检查

Proposal 进入 backlog 前必须满足：

- [ ] 有明确用户价值或技术目标。
- [ ] 范围能在 0.5-2 天内完成，或已拆分。
- [ ] 有至少一条可验证验收标准。
- [ ] 依赖明确。
- [ ] 需要修改的层明确。
- [ ] Proposal README 状态同步为 `已晋升` 或保留为背景参考。

## Epic / Story 一致性检查

当新增 Epic、拆出子 Story 或选择多个 Story 进入迭代时检查：

- [ ] 父项确实包含多个独立结果、阶段/风险依赖或超出 Story 交付窗口，而非仅因跨模块而创建。
- [ ] 新父子关系使用项目既有前缀和后缀子编号；历史编号关系未被无意义改写。
- [ ] 父项列出子 Story、状态、依赖和所属迭代；子项写明父 Epic、依赖和解锁内容。
- [ ] 子项符合 Story DoR；尚有未完成硬依赖的子项未被标为 `Ready` / `In Progress`。
- [ ] 跨 Epic 选取时，迭代目标、WIP 限制和依赖顺序均有记录。
- [ ] 父 Epic 的 `Done` 与所有必需子项状态或明确范围缩减决定一致。

## 失败处理

- 发现断链：先修链接，再继续其他检查。
- 发现旧术语误导：改成新术语，或补 `legacy/current implementation` 说明。
- 发现 proposal 误入 backlog：回滚 backlog 项，除非用户明确确认排期且已满足 DoR。
- 发现验证记录缺失：补迭代执行记录，不要只在对话里说明。
