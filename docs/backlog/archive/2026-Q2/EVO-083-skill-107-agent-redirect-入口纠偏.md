# EVO-083 skill 1.0.7 agent redirect 入口纠偏

> Archived backlog item. Routing index: [2026 Q2 Archive](INDEX.md). Active routing surface: [Product Backlog](../../PRODUCT-BACKLOG.md).

- Type: governance
- Status: Done
- Priority: P1
- Source: 用户反馈 2026-06-05 / skill 更新
- Decision Context: 补 `CLAUDE.md` / `GEMINI.md` 单行重定向入口，并同步 manifest 与文档地图

#### Source Detail Snapshot

- 类型：governance
- 优先级：P1
- 状态：Done
- 父 Epic：无
- Story 形态：Governance
- 失败模式：
  - `agent-project-governance` skill 1.0.7 要求初始化/采用时在 `AGENTS.md` 旁创建 `CLAUDE.md` 和 `GEMINI.md` 单行 redirect。
  - 本项目已处于 conformant，但缺少这两个入口，Claude Code / Gemini CLI 可能无法自动发现统一治理规则。
- 范围：
  - 新增 `CLAUDE.md` 和 `GEMINI.md`，内容均为单行重定向到 `AGENTS.md`。
  - 更新 `.agent-governance/manifest.yaml` entrypoints 和审计日期。
  - 更新 `docs/README.md` 文档地图。
- 不做：
  - 不复制 `AGENTS.md` 内容到 redirect 文件。
  - 不改现有 Agent 规则、SOP 或业务代码。
- 验收标准：
  - [x] `CLAUDE.md` 与 `GEMINI.md` 存在，且只指向 `AGENTS.md`。
  - [x] manifest 记录 redirect 入口。
  - [x] 文档地图可从 root entrypoints 找到 redirect 文件。
  - [x] governance validator、Markdown 链接检查和 `git diff --check` 通过。
- 依赖或阻塞：无。
- 解锁内容：符合 skill 1.0.7 的多 Agent 启动入口标准。
- 影响范围：root agent entrypoints / manifest / docs。
- 最小验证方式：governance validator；Markdown 链接检查；`git diff --check`。
