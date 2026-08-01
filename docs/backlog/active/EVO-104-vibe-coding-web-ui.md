# EVO-104 Vibe Coding Web UI

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- [Design System — Figma tokens](../../reference/DESIGN.md)
- [Vibe Coding UI Design Decisions](../../design/vibe-coding-ui-decisions.md)
- [Git-Centric Platform Proposal](../../proposals/GIT-CENTRIC-PLATFORM.md)
- [Backend Repo Context API item file](EVO-103-repo-context-and-smart-http.md)

## Summary

| Field | Value |
| --- | --- |
| ID | EVO-104 |
| Type | feature |
| Status | Proposed |
| Priority | P0 |
| Parent Epic | None recorded |
| Source | 用户反馈 2026-06-23（vibe coding 重新定位 / GitHub Pages 类比） |
| UX Gate | Resolved by [Vibe Coding UI Design Decisions](../../design/vibe-coding-ui-decisions.md) |

## Problem Or Outcome

为 git-centric 平台提供在线 vibe coding 体验：用户在 Web UI 中浏览/编辑 repo 文件、与外部 agent engine 协作完成开发、提交 commit。Repo 是主用例，skill/mcp/cli 是 Pages 式衍生能力（详见 EVO-108）。

## Goal And Non-Goals

- **Goal**：实现可用的 Codex/TUI-style vibe coding 交互体验：以 agent session stream 为主，文件树按需打开，文件内容和 diff 作为 contextual artifact 呈现，支持 commit / promote / branch 切换 / diff 查看 / 与 agent 实时协作。
- **Non-goals**：
  - 不实现实时多人协作（Yjs/CRDT）— 后期评估。
  - 不实现 live preview pane（web app 沙箱预览）— 后期评估。
  - 不实现 web 终端 — 后期评估。
  - 不实现 skill/mcp 详情页 UI（EVO-109 范围）。

## UX Decisions Required Before Implementation

> U-01 ~ U-05 已在 [Vibe Coding UI Design Decisions](../../design/vibe-coding-ui-decisions.md)
> 完成决策。EVO-104 不再被 UX 调研门禁阻塞；仍需等待 EVO-103 / EVO-105 /
> EVO-106 / EVO-112 等产品与 API 依赖满足后才能进入实现迭代。

### P0 — 阻塞实施，必须先决策

- [x] **U-01 编辑器选型** [Decided: CodeMirror 6]
  - 候选：A. Monaco Editor（VS Code 内核，~5MB bundle）；B. CodeMirror 6（轻量，~200KB）；C. Theia（重型 IDE 框架）。
  - 默认推荐：**B. CodeMirror 6**（bundle 体积小，Vite chunk 拆分友好；可独立引入语言包）。
  - 决策维度：bundle 体积、语言支持（Python/TS/Go/Rust 等）、diff 模式、多光标、性能、LSP client 集成能力。
  - **待确认**：(a) 体积 vs 功能优先级的取舍；(b) 是否需要 Monaco 多光标 / Inlay Hint / 高级代码补全能力；(c) 是否依赖 LSP（Language Server Protocol）做跳转/补全；(d) Monaco bundle 通过 rust-embed-for-web 嵌入后端发布物时的体积叠加。

- [x] **U-02 主布局形态** [Revised: conversation-first workspace；file tree 抽屉化；文件内容作为 artifact]
  - 候选：A. 三栏（file tree / editor / chat panel，宽度可拖动）；B. 两栏 + Tab（file tree + editor，chat 走 Tab 切换）；C. 单栏 + 命令面板（VS Code 风格，cmd+k 唤起 chat）。
  - 旧推荐：**A. 三栏**（桌面端）+ 移动端折叠 chat panel。
  - 修订决策：**Codex/TUI-style conversation-first workspace**。主区域显示 session stream；file tree 通过 rail / drawer 打开；文件内容、diff、commit history 作为 contextual artifact 在 stream 或 artifact view 中打开。
  - 决策维度：屏幕空间利用、移动端 / 平板适配、agent 操作时的视觉焦点、多文件切换体验。
  - **已确认**：默认不让 editor 和 chat 常驻并列；用户焦点集中在 agent 交互和执行事件流上。

- [x] **U-03 Chat 流式架构** [Decided: SSE 下行事件流 + POST 上行命令]
  - 候选：A. WebSocket（双向）；B. SSE / Server-Sent Events（单向 server→client）；C. Long polling。
  - 默认推荐：**B. SSE**（单向，足够 agent 推送 token 增量与事件；client 用普通 POST 发送消息）。
  - 决策维度：外部 agent engine 是否支持流式、断线重连策略、是否需要"双向"（用户在 agent 执行中追加指令 / 中断 agent）。
  - **待确认**：(a) agent engine 的事件协议是否已就绪；(b) "用户中断 agent" 是否 MVP 必需（决定是否必须 WebSocket）；(c) SSE 在代理（Nginx / CDN）后的兼容性。

- [x] **U-04 Agent Branch 心智模型** [Decided: 自动 agent session branch]
  - 问题：用户在 UI 中如何理解"agent 在一个独立分支上工作"？
  - 候选：A. 自动分支 `agent/{session-id}/{feature-name}`，顶栏显示当前分支；B. 每个 vibe session 视为独立 workspace，不直接映射 git branch，commit 时才合并；C. 每条用户消息创建一个分支（`agent/msg-{n}`），UI 显示分支链。
  - 默认推荐：**A. 自动分支**（最贴近 git 原生心智；与 `.evolith/policy.yaml` 的 auto_merge / require_review 配合直接）。
  - 决策维度：与 auto_merge / require_review 配合的清晰度、用户对 branch 的心智负担、与后续 promote 流程的衔接。
  - **待确认**：(a) 是否需要"work in progress"显式分支 vs 直推临时分支；(b) agent 完成任务后，分支是自动 archive 还是保留可继续迭代；(c) 多 agent 并行协作（同一 repo 两个 session）的分支命名冲突处理。

- [x] **U-05 直推直合三态视觉反馈** [Decided: 持续状态徽章 + inline policy explanation]
  - 问题：commit 进入 `auto_merge` / `require_review` / `block` 三种状态时，UI 如何视觉区分？
  - 候选：file tree 上加状态徽章 / 底部 commit 按钮颜色变化 / toast 通知 / commit history 列表染色 / 顶栏 branch 标签染色。
  - 默认推荐：**状态徽章（file tree + branch 标签）+ commit 按钮旁 inline 提示**。
  - 决策维度：用户首次使用如何告知"auto_merge 已生效"、错误状态的明显程度（block 应显眼但不应恐慌）。
  - **待确认**：(a) 徽章颜色规范（沿用 Figma design system token：`semantic-success` `#1ea64a`、`block-coral` `#f3c9b6` 等）；(b) 是否需要"动效"提示状态变化；(c) block 状态是否需要阻止 UI 后续操作（form 禁用）。

### P1 — 实施期可定，不阻塞进入迭代

- [ ] **U-06 Diff Viewer 形态** [实施期调研]
  - 候选：inline（VS Code 默认）/ side-by-side / 混合（用户切换）。
  - 多文件 diff 展示策略：单页多文件 diff / 文件树式 diff 选择 / 顶部 tab 切换。
  - 默认推荐：**inline**，提供切换按钮。
  - **待确认**：是否需要在 diff 上展示 agent reasoning 摘要（"agent 改这一行的原因"）。

- [ ] **U-07 Commit 消息生成** [实施期调研]
  - 候选：用户手写 / LLM 基于 diff 生成 / Conventional Commits 模板填充（`feat:` / `fix:` 引导）。
  - 默认推荐：**LLM 生成 + 用户可编辑**。
  - **待确认**：(a) agent 自己 commit 时的消息格式（是否带 `(agent)` 后缀或 `Co-authored-by` trailer）；(b) 多文件 commit 是否生成统一消息还是按文件拆 commit。

- [ ] **U-08 文件冲突处理（agent vs user）** [实施期调研]
  - 场景：agent 在 branch 上编辑，user 在 main 上编辑同一文件，promote 时冲突。
  - 候选：乐观锁（agent commit 失败提示）/ 悲观锁（agent 编辑时锁文件）/ 自动 3-way merge / 推迟到 promote 阶段处理。
  - 默认推荐：**MVP 不处理**（agent branch 与 main 隔离，无冲突场景）；Phase 5 promote 阶段再评估。
  - **待确认**：是否需要在 agent 编辑时显示 "locked by agent" 提示，避免用户重复编辑。

### P2 — 后期扩展，明确 out of MVP scope

- [ ] **U-09 实时多人协作** [out of MVP] — Yjs / CRDT 集成；显式 Phase 6+ 评估。
- [ ] **U-10 Live Preview Pane** [out of MVP] — Web app 开发场景下的 sandbox + iframe preview；Phase 5+。
- [ ] **U-11 终端面板** [out of MVP] — 用户在 vibe session 中运行命令（web shell 直连 repo sandbox）；Phase 5+。
- [ ] **U-12 文件拖拽上传** [out of MVP] — 用户直接拖拽文件到 file tree；后期。
- [ ] **U-13 键盘快捷键全键位** [out of MVP] — VS Code 风格全套快捷键（cmd+shift+p 等）；MVP 只实现核心快捷键（cmd+s 保存、cmd+enter commit）。

## Dependencies And Blockers

- 依赖 EVO-100 / EVO-101（Git Service + `git_repos` 表）
- 依赖 EVO-103（Repo Context API + Commit/Promote API 后端）
- 依赖 EVO-106（Agent Session API + Scoped Token）
- **UX 调研前置门禁**：已解除。U-01 ~ U-05 已写入 [Vibe Coding UI Design Decisions](../../design/vibe-coding-ui-decisions.md)。
- 设计约束：所有 UI 实现须遵循 `docs/reference/DESIGN.md` 的 Figma token；颜色 / 字号 / 间距不允许硬编码。

## Acceptance Criteria

- 类型：feature
- 优先级：P0
- 状态：Proposed
- 用户价值：用户在 Evolith Web UI 中即可完成代码浏览 / 编辑 / commit / 与 agent 协作，无需本地 git 客户端
- 核心设计：
  1. Conversation-first session workspace（user prompt + agent plan + execution events + artifact cards）
  2. File tree 作为 rail / drawer 按需打开
  3. CodeMirror 6 editor 作为 artifact / editor mode 打开（含 diff 模式）
  4. 实时 agent stream（SSE 流式，含 markdown 渲染、代码块语法高亮、tool/file events）
  5. 直推直合 commit / promote 流程，含 `auto_merge` / `require_review` / `block` 三态视觉反馈
  6. branch 切换 + diff viewer + commit history
- 验收标准：
  - [x] UX 议题 U-01 ~ U-05 完成决策并写入 design doc；design doc 链接加到本文件 Required Reads
  - [ ] 桌面端 MVP：session stream 为默认主界面；file tree 可从 rail / drawer 打开；文件和 diff 可作为 artifact 打开
  - [ ] 用户可创建文件、编辑、提交 commit（`auto_merge` / `require_review` / `block` 三态都验证）
  - [ ] 用户可与 agent 通过 prompt / session stream 交互；agent 操作以 event + artifact card 形式实时反映（通过 SSE 流式推送）
  - [ ] 用户可切换 branch、查看 diff、promote to main
  - [ ] 移动端基础布局：session stream 为主，file tree 和 artifact 使用抽屉 / 全屏 sheet
  - [ ] 所有 UI 颜色 / 字号使用 `docs/reference/DESIGN.md` token，无硬编码
- 依赖或阻塞：EVO-100 / EVO-103 / EVO-105 / EVO-106 / EVO-112 后端和基础 UI 依赖就绪；UX 决策已完成
- 影响范围：frontend（新增页面 + 组件 + lib/api + stores）；不涉及 backend
- 最小验证方式：Playwright 截图（桌面 session stream / file tree drawer / artifact editor / diff card / commit 状态徽章 / 移动 sheet / agent stream）+ 手工 vibe coding 全流程（创建文件 → commit → agent 介入 → diff → promote）
- 不做：实时多人协作（U-09）；live preview（U-10）；终端面板（U-11）；文件拖拽（U-12）；全键位快捷键（U-13）

## Validation Evidence Required

- UX 决策记录：[Vibe Coding UI Design Decisions](../../design/vibe-coding-ui-decisions.md)
- Playwright 截图：桌面 session stream、file tree drawer、artifact editor、diff card、移动 sheet、commit 状态徽章、agent stream 渲染
- 手工验收报告：完整 vibe coding 流程（创建 → 编辑 → agent 协作 → diff → promote）
- 视觉对照：所有截图与 Figma 设计 token 一致（`bun run lint:design-tokens` 或类似门禁通过）

## Residual Work Destination

- U-06 ~ U-13 议题列表保留在本文件；实施期如有结论，更新到 design doc 或本文件
- 与本 epic 相关但未列入的 UX 子议题另开 EVO
- 后期扩展项（U-09 / U-10 / U-11 / U-12 / U-13）独立评估后另开 EVO

## Source Snapshot

- Source: 用户反馈 2026-06-23（vibe coding 重新定位）
- Decision context: Evolith 重新定位为 git 托管 + vibe coding 平台；skill/mcp/cli 是 Pages 式衍生能力（git repo 是 substrate，特殊能力是 Pages 式增强）
- Prior discussion: 2026-06-23 多轮迭代讨论确认 MVP 范围（仓库浏览 + 直推直合，无 PR/MR UI）+ 仓库模型（per-resource 已弃用，改为通用 git 托管）
