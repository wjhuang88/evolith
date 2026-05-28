# Iteration 023: 治理 Skill Manifest 接入与一致性审计

> 文档状态：Closed
> 计划发布日期：2026-05-28
> 计划目标：补齐 `.agent-governance/manifest.yaml`，让 Evolith 能被
> `agent-project-governance` skill validator 识别为已初始化并可审计。
>
> 基线保护：本文件一旦提交，以下“发布计划基线”内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

- 为 Evolith 建立 `.agent-governance/manifest.yaml`，记录 profile、entrypoints、
  capability 状态、风险门禁和迁移映射。
- 运行 `agent-project-governance` bundled validator，修复 EVO-035 暴露的 manifest 缺失。
- 同步 backlog、iteration、目录和经验记录，避免 validator 失败只停留在对话中。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|-----------|--------|---------------|
| EVO-035 | 治理 skill manifest 接入与一致性审计 | 无 | P2 | EVO-034 已完成；EVO-041 暴露 validator 仍失败 |

### Iteration Inventory Disposition

| Iteration | 当前状态 | 处置结论 |
|-----------|----------|----------|
| Iteration 009 | Closed | 不阻塞本轮治理修复 |
| Iteration 010 | Closed | 不阻塞本轮治理修复 |
| Iteration 012 | Planned / Blocked | 继续保持 EVO-016 refinement 阻塞；本轮不激活 |
| Iteration 017 | Planned / Blocked | 继续保持激活阻塞；本轮不激活 |
| Iteration 018 | Planned / Blocked | 继续按 Phase E 依赖链保留；本轮不激活 |
| Iteration 019 | Planned / Blocked | 继续依赖 Iteration 018；本轮不激活 |
| Iteration 020 | Planned / Blocked | 继续依赖 Iteration 019；本轮不激活 |
| Iteration 022 | Closed | 已完成 Story/BDD 方法论收口；其 validator 残余由本轮修复 |

## 3. 发布计划基线：不做事项

- 不修改业务代码、构建脚本、CI/CD 或部署策略。
- 不重写既有 SOP、Backlog、Iteration 历史结构。
- 不提交或推送外部 `agent-project-governance` 仓库；该仓库已单独本地提交，但无 remote。

## 4. 发布计划基线：计划验收标准

- Story 格式与 BDD 适用性：
  - [x] 每个候选 Story 已标明 Product / API / Technical / Governance / Spike 形态。
  - [x] 行为类 Story 使用 Given/When/Then 场景，或记录 BDD 不适用原因。
  - [x] 技术、治理或 Spike 使用等价技术验收、状态归口和残余归口。
- [x] `.agent-governance/manifest.yaml` 存在，并记录 Evolith 当前 governance profile。
- [x] manifest entrypoints 指向真实存在的 Agent guide、docs、backlog、iterations、
  decisions、roadmap、proposals、reference 和 SOP。
- [x] manifest capability 状态与当前标准治理文件一致。
- [x] bundled validator 通过。

## 5. 发布计划基线：计划验证

```bash
python3 /Users/GHuang/WorkSpace/AiProjects/skill-sources/agent-project-governance/scripts/validate_project_governance.py /Users/GHuang/WorkSpace/AiProjects/evolith

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
| manifest 过度声明能力 | validator 与现有 SOP/AGENTS 路由对照；仅声明已有能力 |
| 后续 SOP 变动后 manifest 漂移 | manifest `next_actions` 要求治理变更后重新运行 validator |
| 外部 skill 无 remote 影响推送 | 记录为外部仓库配置问题，不阻塞本项目 manifest 修复 |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 修复本项目 validator 暴露的 manifest 缺失问题 |
| 产物 | `.agent-governance/manifest.yaml`、EVO-035、Iteration 023、Iteration README、EVOLUTION |
| 状态同步归口 | backlog、iteration、manifest、EVOLUTION |
| Story/BDD 归口 | Governance Story；非行为类，使用 validator、Markdown 链接和 diff 检查作为等价验收 |
| 验证证据 | bundled validator；Markdown 链接检查；`git diff --check` |
| 残余工作归口 | 外部 skill 无 remote，无法 push；需用户配置 remote 后推送 |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-05-28 | activation | 作为 EVO-035 治理修复启动；不激活产品 planned iteration |
| 2026-05-28 | progress | 新增 `.agent-governance/manifest.yaml` 并同步 backlog / iteration |
| 2026-05-28 | progress | 修复 validator 暴露的 active governance 过期源文件路径：`AGENTS.md` i18n 路径与 `TESTING.md` crate 测试路径 |
| 2026-05-28 | validation | `validate_project_governance.py /Users/GHuang/WorkSpace/AiProjects/evolith`：通过，`Governance validation passed: 0 warning(s).` |
| 2026-05-28 | validation | Markdown 链接检查：通过，`all markdown links exist` |
| 2026-05-28 | validation | `git diff --check`：通过 |
| 2026-05-28 | completion | EVO-035 完成；manifest 接入与一致性审计通过 |

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|
| 2026-05-28 | urgent-fix | 接受 | 用户要求修复本项目问题；validator 缺失 manifest 是已登记 EVO-035 | 以新 iteration 023 收口，不改写 Iteration 022 |

## 10. Review

- 完成：
  - 新增 `.agent-governance/manifest.yaml`，记录 Evolith `high-risk / conformant`
    governance profile、entrypoints、capabilities、risk gates 和 migration 映射。
  - 修复 validator 发现的过期 active governance 源文件路径。
  - 同步 EVO-035、Iteration 023、Iteration README 和 EVOLUTION。
- 未完成：
  - 外部 `agent-project-governance` 仓库已本地提交，但没有 remote，无法 push。
- 验证结果：
  - bundled validator：通过，0 warnings。
  - Markdown 链接检查：通过。
  - `git diff --check`：通过。
- 闭环状态：`Complete`
- 残余归口：
  - 外部 skill push 等待配置 remote。

## 11. Retrospective

- 做得好的：
  - 将 validator 暴露的问题回收到既有 EVO-035，而不是忽略失败。
- 需要调整的：
  - 治理能力声明变更后应同步运行 bundled validator。
- 写入 EVOLUTION：
  - 已写入 `2026-05-28 Governance manifest 是 skill adoption 的可验证入口`。
