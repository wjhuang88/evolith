# Iteration 008: Epic 与子需求拆分治理规则

> 时间：2026-05-26
> 目标：完成 EVO-034，为大需求拆分、父子追踪和迭代选取建立可执行规则，并同步治理 skill。

## 1. 本轮目标

- 补齐 Evolith 的 Epic / Story 需求治理方法，消除编号、依赖和 DoR 口径缺口。
- 把项目中验证有效的规则抽象进 `agent-project-governance` skill，供已有和新项目适配。

## 2. 选入故事

| ID | 标题 | 优先级 | 负责人 | 状态 |
|----|------|--------|--------|------|
| EVO-034 | Epic 与子需求拆分治理规则 | P1 | Agent | Done |

## 3. 不做事项

- 不批量改写既有 `EVO-*` 编号或历史迭代记录。
- 不改变业务代码、API 合约、数据库或交付形态。
- 不触碰 skill 仓库中与本故事无关的已有外部修改。

## 4. 验收标准

- [x] 需求进入 SOP 可以判断何时建 Epic，以及如何拆为可独立验证的 Story。
- [x] 父子编号、依赖闭包、分层 DoR 与跨 Epic 迭代选择规则可以直接执行。
- [x] 迭代开始、迭代执行和文档检查承接上述约束。
- [x] skill 中形成可迁移而非照搬 Evolith 文档的同类方法论。
- [x] 验证结果如实记录。

## 5. 验证计划

```bash
python3 - <<'PY'
# Verify local Markdown links under project governance documents.
PY
git diff --check
python3 /Users/GHuang/.codex/skills/.system/skill-creator/scripts/quick_validate.py /private/tmp/agent-project-governance-epic
```

## 6. 风险与回滚

| 风险 | 处理 |
|------|------|
| 新规则与历史 `EVO-002` 拆分编号不一致 | 规则仅约束新建父子关系，明确历史 ID 不因格式迁移 |
| 规则要求过重，阻止小故事推进 | Epic 仅在多结果、多依赖或明显超出交付窗口时建立 |
| skill 仓库已有外部未提交改动被覆盖 | 在临时副本编辑并仅发布本次涉及文件，保留 `README.md` 改动 |

## 7. 执行记录

| 日期 | 记录 |
|------|------|
| 2026-05-26 | Iteration 008 started. 选入 EVO-034；审计确认现有 SOP 只有粗粒度 Epic 拆分规则，缺少编号、依赖、DoR 和迭代选择定义。 |
| 2026-05-26 | Skill target audit: `agent-project-governance` 已有用户未提交的 `README.md` 修改；本轮隔离编辑，不覆盖该文件。 |
| 2026-05-26 | Driver implementation: 更新需求进入、迭代开始/执行、文档检查和 backlog 说明，定义 Epic/Story、编号、依赖与选取规则；建立通用 skill 发布包。 |
| 2026-05-26 | Validation: Markdown link check passed；`git diff --check` passed；隔离 skill 发布包经 `quick_validate.py` 校验通过。 |
| 2026-05-26 | Publish blocked: 写入 `/Users/GHuang/WorkSpace/AiProjects/skill-sources/agent-project-governance` 的审批超时，目标 skill 尚未收到本轮变更；保留 EVO-034 为 In Progress。 |
| 2026-05-26 | Publish completed after approval: 仅发布 skill 主说明及 Epic/Story 相关 references，不覆盖已存在的 `README.md` 外部修改；目标目录 `quick_validate.py` 与 `git diff --check` 通过。 |
| 2026-05-26 | Additional governance audit: `validate_project_governance.py /Users/GHuang/WorkSpace/AiProjects/evolith` failed because `.agent-governance/manifest.yaml` is absent；登记为独立的 EVO-035，不将失败虚报为本轮通过。 |

## 8. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|
|  | clarification / scope-change / product-pivot / urgent-fix | 接受 / 拆分 / 暂缓 / 拒绝 |  | 保留 / 移除 / 后续清理 |

## 9. Review

- 完成：Evolith 项目内的 Epic / Story 方法论和承接检查已实现；通用规则已发布至 `agent-project-governance` skill。
- 未完成：Evolith 尚未建立 skill 识别所需的 governance manifest，已拆为 EVO-035；本轮未处理 skill 仓库中预存的 `README.md` 修改。
- 验证结果：项目 Markdown 链接检查通过；项目 `git diff --check` 通过；目标 skill `quick_validate.py` 与 `git diff --check` 通过；项目 bundled governance audit 因缺少 manifest 失败并已登记后续事项。

## 10. Retrospective

- 做得好的：
- 需要调整的：外部 skill 目录发布需要先获得写入权限，不能把本地验证等同于发布完成。
- 写入 EVOLUTION：已写入 Epic 拆分不能只定义大小阈值。
