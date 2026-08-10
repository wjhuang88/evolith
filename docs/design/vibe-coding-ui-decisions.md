# Vibe Coding UI Design Decisions

> **状态**：Decided
> **决策日期**：2026-06-25
> **适用范围**：EVO-104 Vibe Coding Web UI 的 U-01 ~ U-05 前置 UX 门禁
> **结论**：U-01 ~ U-05 均已决策。2026-06-25 根据用户反馈修正 U-02：
> 主体验从三栏 IDE 式布局改为 Codex/TUI-style conversation-first workspace。
> EVO-104 不再被 UX 调研门禁阻塞；仍需等待 EVO-103 / EVO-105 / EVO-106 / EVO-112 等后端与基础 UI 依赖满足后才能进入实现迭代。

## 引用关系

- 上游：[EVO-100 Epic](../backlog/active/EVO-100-git-centric-platform-foundation.md) Phase E'-2 启动门禁
- 上游：[EVO-104 Vibe Coding Web UI](../backlog/active/EVO-104-vibe-coding-web-ui.md)
- 上游：[GIT-CENTRIC-PLATFORM §7 UX 调研前置门禁](../proposals/GIT-CENTRIC-PLATFORM.md)
- 设计系统约束：[docs/reference/DESIGN.md](../reference/DESIGN.md)

## Decision Summary

| ID | 议题 | 决策 | 状态 |
|----|------|------|------|
| U-01 | 编辑器选型 | CodeMirror 6 | Decided |
| U-02 | 主布局形态 | Codex/TUI-style conversation-first workspace；文件树抽屉化；文件内容作为 contextual artifact | Revised |
| U-03 | Chat 流式架构 | SSE 下行事件流 + POST 上行命令 | Decided |
| U-04 | Agent Branch 心智模型 | 每个 agent session 使用自动分支 `agent/{session_id}/{feature_slug}` | Decided |
| U-05 | 直推直合三态视觉反馈 | 持续状态徽章 + inline policy explanation + commit result event | Decided |

## U-01：编辑器选型

| 字段 | 值 |
|------|-----|
| 状态 | `Decided` |
| 决策 | **CodeMirror 6** |
| 不采用 | Monaco Editor / Theia |

### 理由

- Vibe Coding MVP 是嵌入式 repo 编辑体验，不是完整 Web IDE；CodeMirror 6 的体积、启动速度和 Vite chunk 拆分更符合 rust-embed-for-web 交付形态。
- MVP 需要语法高亮、文件编辑、基础快捷键、diff 集成和可组合扩展；不需要 Monaco 的完整 VS Code 能力。
- LSP、Inlay Hint、高级补全不进入 MVP。后续如果需要，按语言懒加载扩展或单独评估 Monaco 迁移，不在 EVO-104 阻塞。

### 实施约束

- 语言包按需加载；首屏不得一次性打包所有语言。
- 编辑器组件必须和文件树、diff viewer、chat panel 解耦，避免后续替换编辑器时牵连页面状态。
- MVP 快捷键只保留 `Cmd/Ctrl+S` 保存草稿、`Cmd/Ctrl+Enter` 触发 commit；全键位快捷键归入 U-13。

## U-02：主布局形态

| 字段 | 值 |
|------|-----|
| 状态 | `Revised` |
| 决策 | **Codex/TUI-style conversation-first workspace；文件树抽屉化；文件内容作为 contextual artifact** |
| 修订来源 | 2026-06-25 用户反馈：三栏会分散注意力，文件内容和聊天窗口不应常驻并列 |

### 理由

- Vibe Coding 的核心价值不是“Web IDE + Chat 并排”，而是让用户聚焦在 agent session 的任务流：提出目标、观察计划、查看执行事件、审查 diff、确认 commit/promote。
- 文件树和文件内容仍然重要，但它们是上下文与产物，不应在默认状态下和对话主线争夺注意力。
- Codex/TUI-style 的顺序事件流更适合 agent-native 体验：用户能连续理解 agent 正在做什么、改了什么、验证结果如何。
- 桌面端不再以三栏常驻为默认；桌面和平板都共享“主会话流 + contextual artifact”的心智模型，降低跨设备差异。

### 实施约束

- 默认第一屏是 session stream：user prompt、agent plan、tool/file events、diff summary、verification result、policy result 按时间线排列。
- 左侧只保留窄 rail / 抽屉入口；文件树按需展开，不常驻占用主工作区。
- 文件内容、diff 和 commit history 作为 artifact panel 呈现：
  - agent 引用文件时在 stream 内展示 file/diff card；
  - 用户点击文件树或 file card 时，主区域切换到 artifact view；
  - 需要长期编辑时进入 editor mode，而不是默认并列打开 editor 和 chat。
- 命令入口采用 TUI 风格：`/open`、`/diff`、`/commit`、`/promote`、`/policy`，并提供 `Cmd/Ctrl+K` command palette。
- 顶部持续显示 repo、agent branch、policy state；底部保留 prompt input 和主要 action，不引入第三个常驻面板。
- 移动端与桌面端使用相同模型：session stream 为主，file tree 和 artifacts 通过抽屉 / 全屏 sheet 打开。

## U-03：Chat 流式架构

| 字段 | 值 |
|------|-----|
| 状态 | `Decided` |
| 决策 | **SSE 下行事件流 + POST 上行命令** |
| 不采用 | WebSocket 作为 MVP 主通道；Long polling |

### 理由

- MVP 的主要流量是 agent token、tool event、file changed、commit result 等 server-to-client 事件；SSE 足够表达。
- 用户发消息、取消任务、请求 commit 等动作可用普通 POST，权限和 CSRF 处理更直接。
- WebSocket 会引入连接生命周期、双向协议和反代配置复杂度；当前没有必须全双工的硬需求。

### 事件模型

| Event | 用途 |
|-------|------|
| `message.delta` | agent 文本增量 |
| `message.done` | agent 消息完成 |
| `tool.started` / `tool.finished` | agent 调用外部工具的可视化状态 |
| `file.changed` | 文件内容或 file tree 变化，默认渲染为 stream 内 artifact card |
| `policy.evaluated` | commit policy 三态评估结果 |
| `commit.created` | commit 已创建 |
| `promote.completed` | promote / merge 完成 |
| `error` | 可恢复错误 |

### 实施约束

- client 发送消息：`POST /agent-sessions/{id}/messages`。
- client 中断 agent：`POST /agent-sessions/{id}/cancel`，由 EVO-106/API 设计承接。
- SSE 响应必须设置禁用代理缓冲的 header；Nginx 配置需要允许 streaming pass-through。
- 断线重连使用 session event cursor；cursor 机制如果 EVO-106 未实现，MVP 可退化为重新拉取 session transcript。

## U-04：Agent Branch 心智模型

| 字段 | 值 |
|------|-----|
| 状态 | `Decided` |
| 决策 | **每个 agent session 使用自动分支 `agent/{session_id}/{feature_slug}`** |

### 理由

- 显式 branch 最符合 Git-Centric 平台定位，能直接承接后续 `commit`、`promote`、policy 评估和审计。
- session workspace 隐藏 git 细节会把复杂度推迟到 commit/promote 阶段，反而增加合并和追溯成本。
- 每条消息一个分支会制造过多引用，不适合 MVP。

### 用户心智

- 顶栏展示当前工作区：`main -> agent/<short-session>/<slug>`。
- UI 文案使用 “Agent branch” / “代理分支”，避免把它描述成临时草稿。
- agent 完成后分支默认保留，用户可继续迭代或 promote；自动 archive 不进 MVP。
- 多 agent 并行通过 session UUID 避免分支命名冲突。

### 实施约束

- feature slug 来自用户首条消息或 session title，需 sanitize；冲突时追加短 UUID。
- `EVO-105` 的 commit/promote API 必须接受 agent branch 作为一等输入。
- `EVO-106` 的 scoped token 只允许读当前 repo、写当前 agent branch、请求 promote；禁止 admin / force push。

## U-05：直推直合三态视觉反馈

| 字段 | 值 |
|------|-----|
| 状态 | `Decided` |
| 决策 | **持续状态徽章 + inline policy explanation + commit result event** |

### 状态定义

| Policy State | UI Label | 主视觉 | 行为 |
|--------------|----------|--------|------|
| `auto_merge` | Auto merge | `semantic-success` | commit 后可自动进入目标分支，显示成功事件 |
| `require_review` | Review required | `block-lime` | commit 保留在 agent branch，promote 需要人工确认 |
| `block` | Blocked | `block-coral` | commit / promote 按钮禁用，并展示阻止原因 |

### 理由

- 状态徽章放在顶部 branch 标签、prompt/action bar 和 stream 内 commit event 中，提供持续可见的安全边界。
- 仅改变按钮颜色不够，因为用户需要在提交前、提交后和历史回看时都理解 policy 结果。
- toast 只作为补充反馈，不作为唯一状态来源。

### 实施约束

- 状态颜色必须来自 [DESIGN.md](../reference/DESIGN.md) token，不允许硬编码十六进制值到组件。
- `block` 状态必须禁用危险动作，并在 stream 内展示 policy reason；仍允许用户继续编辑、查看 diff、复制错误信息。
- `require_review` 是默认安全状态。缺失 `.evolith/policy.yaml` 时 UI 也必须显示 review required，而不是 auto merge。
- 首次进入 vibe session 时，用 inline hint 解释当前 repo policy；不使用阻塞式 onboarding modal。

## U-06 ~ U-08：实施交互决策

| ID | 状态 | 决策 | 归口 |
|----|------|------|------|
| U-06 | Decided | Diff 默认 inline，允许切换 side-by-side；多文件通过 changed-files rail 选择 | EVO-104 |
| U-07 | Decided | Agent/LLM 生成 Conventional Commit 候选，用户可编辑；提交前必须展示最终 message 与 diff summary | EVO-105 / EVO-104 |
| U-08 | Decided | Session 记录 base Ref；promote 时 target 已前移则返回 conflict、保留 agent branch，并要求 refresh/rebase 后重试；MVP 不静默 auto-merge | EVO-105 |

U-08 的“agent branch 与 main 隔离”只降低编辑期冲突，不消除 promote 时的并发变化。UI 必须把 conflict 作为可恢复终态展示，不能将目标 Ref 未移动误报为 promote 成功。

## Out Of MVP

| ID | 议题 | 归口 |
|----|------|------|
| U-09 | 实时多人协作 | Phase 6+ 独立评估 |
| U-10 | Live Preview Pane | Phase 5+ 独立评估 |
| U-11 | 终端面板 | Phase 5+ 独立评估 |
| U-12 | 文件拖拽上传 | Phase 5+ 独立评估 |
| U-13 | 键盘快捷键全键位 | Phase 5+ 独立评估 |

## 启动门禁状态

- UX 门禁：解除。U-01 ~ U-08 均为 `Decided`。
- Product / API 依赖：未解除。EVO-104 仍依赖 EVO-103 / EVO-105 / EVO-106 / EVO-112 达到可集成状态。
- 下一步：EVO-104 item file 可以引用本文件作为 Required Read，并将 UX 阻塞项改为已完成；不要仅因本文件完成就把 EVO-104 标为 `Ready`。

## Change Log

| 日期 | 类型 | 说明 |
|------|------|------|
| 2026-08-08 | decision closure | U-06 ~ U-08 按推荐方向定案；promote conflict 明确 fail closed，不静默合并。 |
| 2026-06-25 | scope-change | U-02 从桌面三栏常驻布局修订为 Codex/TUI-style conversation-first workspace；文件树抽屉化，文件内容和 diff 作为 contextual artifact 呈现。 |
