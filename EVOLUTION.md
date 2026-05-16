# Evolith 故障排查与经验积累

> 本文件用于把项目中的非直觉经验沉淀为可检索规则。
> 任务开始排查问题时，先看 Part 1；任务中发现新坑时，按 Part 3 写回。

---

## Part 1: 问题速查表

| # | 现象 | 可能原因 | 快速解决 |
|---|------|----------|----------|
| 1 | Docker Compose 后端没有连 PostgreSQL | 使用了 `DATABASE_TYPE` 而不是 `DATABASE__DATABASE_TYPE` | 检查 compose/env 是否使用双下划线配置键 |
| 2 | 容器化前端请求 `/auth/login` 404 | `NEXT_PUBLIC_API_URL` 缺少 `/api/v1` | 设置为 `http://localhost:8080/api/v1` 或生产 API 前缀 |
| 3 | 修改 ConfigMap/Nginx/Compose 后线上不生效 | Git 提交不等于部署刷新 | 按发布 SOP 执行重建、重启或重新 apply |
| 4 | SQLite 与 PostgreSQL 行为不一致 | 只改了一侧 migration/repository | 同步修改 `migrations/sqlite`、`migrations/postgres` 和两套 repository |
| 5 | CSRF 403 | 状态变更请求缺少 `csrf_token` cookie 或 `X-CSRF-Token` header | 先完成登录/刷新，再由 API client 自动带 header |
| 6 | Skill 执行未进入 Docker 沙箱 | Docker 初始化失败后降级到 default executor | 查看后端启动日志中的 sandbox warn |

---

## Part 2: 经验条目

> 新经验按时间倒序追加。避免重复记录同一问题。

### 2026-05-16 过长 SOP 要按任务入口拆分
**现象**: `ITERATION-WORKFLOW.md` 同时承载需求准入、Backlog refinement、迭代执行、变更控制和状态定义，AGENTS Task Router 只能把多个不同任务都指向同一个长文档。
**根因**: 初次流程改造优先保证闭环，把相邻流程放在一起；随着防呆要求提升，过长 SOP 会让后续 Agent 难以定位必读步骤。
**方案**: 拆出 `docs/sop/REQUIREMENT-INTAKE.md` 和 `docs/sop/CHANGE-CONTROL.md`，`docs/sop/ITERATION-WORKFLOW.md` 只保留迭代执行主循环，并在 AGENTS Task Router 中分别索引。
**教训**: SOP 应按任务入口拆分；当一个 SOP 同时回答“需求怎么进来”和“开发中怎么变更”时，就需要拆成独立文件并让 AGENTS 明确路由。

### 2026-05-15 迭代中需求变更需要明确变更控制入口
**现象**: EVO-001 开发到一半时，产品方向从旧 snippet 概念切换为 CLI 友好接口；原 SOP 只描述了进入迭代和完成迭代，没有说明中途变更如何暂停、拆分和重定范围。
**根因**: 迭代工作流缺少 change request 处理步骤，容易把产品 pivot 混进当前故事，造成验收标准漂移和半成品代码扩大。
**方案**: 在 `docs/sop/ITERATION-WORKFLOW.md` 增加“迭代中需求变更”流程和防呆表；新增 ADR-0002 和 EVO-017；当前迭代停止扩大 snippet 对齐工作，把旧 snippet 相关事项转入迁移故事。
**教训**: 中途变更先记录、分类、重定范围，再继续写代码；产品概念变化必须用 ADR 和 backlog story 固化。

### 2026-05-16 执行流程动作前必须先查 Task Router 和 SOP
**现象**: 用户要求"添加远期需求"，Agent 直接读了 backlog 文件并在表格中插入一行，没有先查 AGENTS.md Task Router 确认正确流程。SOP 明确规定远期想法放 `docs/proposals/`，排期才进 backlog。
**根因**: Agent 把"需求进入"当成 trivial 操作跳过了流程检查，先动手后查规则。AGENTS.md Task Router 已有明确映射"需求进入/拆分/排期 → 必读 ITERATION-WORKFLOW.md"，但未被遵循。
**方案**: 回滚 backlog 错误条目，按 SOP 将远期需求写入 `docs/proposals/AI-GATEWAY.md` 并在索引中注册。
**教训**: 任何涉及流程的操作（需求进入、迭代变更、发布部署等），先查 AGENTS.md Task Router 找到必读 SOP，再动手。即使操作本身看起来简单，流程约束可能不简单。

### 2026-05-15 工程化文档需要分层入口
**现象**: 项目文档较完整，但 AGENTS.md 同时承担项目介绍、状态记录、流程说明和任务路由，后续 agent 需要读大量内容才能找到操作步骤。
**根因**: 文档按功能主题沉淀，但缺少 reference / sop / roadmap / proposals / archive 的职责边界。
**方案**: 新增 `docs/README.md` 文档地图、`docs/sop/` 标准流程、`docs/reference/` 稳定事实、`EVOLUTION.md` 经验写回；将原有专题文档迁移到新分层目录，并在 AGENTS.md 中建立任务路由。
**教训**: 启动文档应该短而硬，复杂步骤放 SOP，稳定事实放 reference，失败经验写 EVOLUTION。

### 2026-05-15 Agent 提交需要可追溯模型和变更边界
**现象**: Agent 生成的提交如果只写普通 commit message，后续难以追踪生成模型、验证范围和脚本行为变更是否同步记录。
**根因**: Git 规则没有进入项目级 SOP，提交前检查、模型标识和脚本 release notes 依赖个人习惯。
**方案**: 新增 `docs/sop/GIT-WORKFLOW.md`，要求语义前缀、提交末尾 `[model: <name>]`、提交前查看 staged diff，脚本行为变更同步 `docs/reference/SCRIPTS-RELEASE-NOTES.md`。
**教训**: AI 参与提交必须保留模型和变更边界，Git 规范要写成 SOP，而不是口头约定。

---

## Part 3: 维护规则

触发以下情况时，会话结束前应追加经验条目：

- 操作失败后找到了根因。
- 发现文档未记录的非直觉行为。
- 多次尝试后才解决问题。
- 用户指出了遗漏、误解或业务口径错误。
- 修改流程、脚本、部署方式后发现新的操作顺序要求。

写回格式：

```markdown
### <YYYY-MM-DD> <一句话总结>
**现象**: <具体表现>
**根因**: <深层原因>
**方案**: <解决步骤>
**教训**: <一句话，供未来 Agent 记忆>
```

写回流程：

1. 读取本文全文，确认没有重复条目。
2. 如果 Part 2 超过 30 条，先把较老条目归档到 `docs/archive/evolution-<YYYY-MM-DD>.md`。
3. 追加新条目到 Part 2。
4. 在最终回复中说明已写回的经验。
