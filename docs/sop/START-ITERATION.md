# SOP: 开始一次迭代

> 本 SOP 负责把 Ready backlog 转入一次可执行迭代。不要只创建 iteration 文件；必须同步 backlog 状态、验证计划和执行记录。

## 触发条件

- 用户要求开始一个新迭代。
- 当前迭代结束后，需要选择下一批故事。
- 需要把一个 Ready backlog story 转为 `In Progress`。

## 前置检查

- [ ] 已运行 `git status --short --branch`，确认工作区状态。
- [ ] 已读取 [需求进入与 Backlog 整理](REQUIREMENT-INTAKE.md)，确认候选故事满足 DoR。
- [ ] 已读取 [特性迭代工作流](ITERATION-WORKFLOW.md)，确认 WIP 限制和 DoD。
- [ ] 当前没有另一个未关闭的 `In Progress` story，或已明确为什么允许切换。
- [ ] 如用户在开始迭代时补充需求，已按 [迭代中需求变更](CHANGE-CONTROL.md) 分类记录。

## 选故事规则

1. 只从 [Product Backlog](../backlog/PRODUCT-BACKLOG.md) 中选择 `Ready` story。
2. 默认选择最高优先级且最靠近当前路线图主线的 story。
3. 如果多个 P0 并列，优先选择能解除后续阻塞的工程门禁或产品概念迁移。
4. 不从 `docs/proposals/` 直接开工；提案必须先晋升到 backlog。
5. 单个 Agent 会话默认只选 1 个 story，除非用户明确要求多故事迭代。
6. 如果候选项是 Epic，只能选择已拆出的 `Ready` 子 story；不要把整个 Epic 放入一次 Agent 微迭代。

## 操作步骤

1. 确定下一个迭代编号：查看 `docs/iterations/ITERATION-*.md`，按最大编号加 1。
2. 创建 `docs/iterations/ITERATION-<N>.md`，优先使用 [迭代模板](../iterations/ITERATION-TEMPLATE.md)。
3. 在迭代文件中写清楚：
   - 本轮目标；
   - 选入故事；
   - 不做事项；
   - 验收标准；
   - 验证计划；
   - 风险与回滚；
   - 第一条执行记录。
4. 把选入 story 的 backlog 状态改为 `In Progress`，备注中写明迭代编号。
5. 更新 [迭代目录](../iterations/README.md)，加入新迭代链接。
6. 如果开始迭代时发生范围补充或优先级调整，在迭代文件 `变更请求` 中记录。
7. 运行文档一致性验证。
8. 在任何实现提交前，确保 iteration 与 backlog 已处于 `In Progress` 并包含验收与验证计划。紧急修复允许先止血，但必须在同一会话补记插队原因，不能在收尾提交中同时伪装“开始并已完成”。

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
- 已有 `In Progress` story：先结束、暂停或记录切换原因，再开始新迭代。
- 忘记同步 backlog：先补 backlog 状态，再继续。
- 用户开始迭代时改变范围：按 `CHANGE-CONTROL.md` 记录，不要直接覆盖原计划。
- 断链或目录未更新：先修复链接和 `docs/iterations/README.md`，再提交。
- 实现提交已经出现但 iteration 尚未开始：将其记录为流程偏差或 urgent-fix，先补审查与验收证据，不得直接补写为无异常完成。

## 相关文档

- [需求进入与 Backlog 整理](REQUIREMENT-INTAKE.md)
- [特性迭代工作流](ITERATION-WORKFLOW.md)
- [迭代中需求变更](CHANGE-CONTROL.md)
- [Product Backlog](../backlog/PRODUCT-BACKLOG.md)
- [迭代目录](../iterations/README.md)
