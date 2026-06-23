# Vibe Coding UI Design Decisions

> **状态**：模板（Phase E'-2 启动门禁）
> **目的**：在 EVO-104 Vibe Coding Web UI 进入迭代前，完成 U-01 ~ U-05 的 UX 决策。
> **流程**：每个议题由 Agent 或人类 owner 填入「决策记录」段；状态从 `Pending` 变为 `Decided` 后写入 ADR 或独立设计文件并链接回此处。

## 引用关系

- 上游：[EVO-100 Epic](../backlog/active/EVO-100-git-centric-platform-foundation.md) Phase E'-2 启动门禁
- 上游：[EVO-104 Vibe Coding Web UI](../backlog/active/EVO-104-vibe-coding-web-ui.md)（P0 阻塞项 U-01 ~ U-05）
- 上游：[GIT-CENTRIC-PLATFORM §7 UX 调研前置门禁](../proposals/GIT-CENTRIC-PLATFORM.md)
- 设计系统约束：[docs/reference/DESIGN.md](../reference/DESIGN.md)（Figma token，所有颜色 / 字号 / 间距禁止硬编码）

## U-01：编辑器选型

| 字段 | 值 |
|------|-----|
| 状态 | `Pending` |
| Owner | 待指派 |
| 默认推荐 | **CodeMirror 6** |
| 决策记录 | _待填_ |

### 候选方案

| 方案 | Bundle 体积 | 功能完整度 | 性能 | 与 rust-embed-for-web 兼容性 |
|------|-----------|----------|------|---------------------------|
| A. Monaco Editor（VS Code 内核） | ~5MB | ★★★★★（多光标 / Inlay Hint / LSP / 高亮全） | 中等 | bundle 体积叠加（Vite chunk 拆分压力大） |
| B. **CodeMirror 6**（推荐） | ~200KB | ★★★★（语言包可独立引入；diff 模式完整） | 快 | Vite 友好；体积可控 |
| C. Theia（IDE 框架） | ~3MB+ | ★★★★★ | 中等 | 过重；适合完整 IDE 而非嵌入式编辑器 |

### 待确认点

- [ ] 体积 vs 功能取舍
- [ ] 是否需要 Monaco 高级能力（多光标 / Inlay Hint）
- [ ] 是否依赖 LSP（Language Server Protocol）做跳转 / 补全
- [ ] 折中方案：CodeMirror 6 + 关键 LSP 子集（按语言懒加载）

### 决策记录

> _Owner 在此处填入：决策方案 / 决策理由 / 折中点 / 后续影响_

---

## U-02：主布局形态

| 字段 | 值 |
|------|-----|
| 状态 | `Pending` |
| Owner | 待指派 |
| 默认推荐 | **A. 三栏（file tree / editor / chat panel），宽度可拖动**；移动端折叠 chat panel |
| 决策记录 | _待填_ |

### 候选方案

| 方案 | 描述 | 优点 | 缺点 |
|------|------|------|------|
| A. **三栏（推荐）** | file tree / editor / chat panel；宽度可拖动 | 同时可见；agent 操作焦点清晰；与 VS Code / Cursor 一致 | 小屏拥挤；需拖动分隔条 |
| B. 两栏 + Tab | file tree + editor；chat 走 Tab 切换 | 主区空间大 | 切换 chat 打断工作流 |
| C. 单栏 + 命令面板（VS Code 风格） | cmd+k 唤起 chat | 极简；最大空间 | 学习成本高；非 IDE 用户不熟悉 |

### 待确认点

- [ ] 平板（iPad）支持是否 MVP 必需（影响三栏 vs 两栏选择）
- [ ] 是否需要"专注模式"（临时隐藏 chat panel）
- [ ] 拖动分隔条是否需要记忆（按用户偏好持久化到 localStorage / DB）

### 决策记录

> _Owner 在此处填入_

---

## U-03：Chat 流式架构

| 字段 | 值 |
|------|-----|
| 状态 | `Pending` |
| Owner | 待指派 |
| 默认推荐 | **B. SSE（Server-Sent Events，单向 server→client）**；client 用 POST 发送消息 |
| 决策记录 | _待填_ |

### 候选方案

| 方案 | 描述 | 优点 | 缺点 |
|------|------|------|------|
| A. WebSocket（双向） | 全双工长连接 | 双向通信；支持"用户在 agent 执行中追加指令" | 实现复杂；agent engine 协议未必支持；连接管理成本 |
| B. **SSE（推荐）** | 单向 server→client；client 用 POST 发消息 | 实现简单；agent 推送 token 增量与事件够用 | 双向需 fallback 到 POST；中断 agent 需独立路径 |
| C. Long polling | 客户端轮询 | 兼容性好；防火墙友好 | 延迟高；服务端资源消耗大 |

### 待确认点

- [ ] 外部 agent engine（用户其他项目）是否已支持流式响应（决定 SSE 是否够用）
- [ ] "用户中断 agent" 是否 MVP 必需（若必需 → WebSocket；若可接受"等当前 batch 完成后接受新指令" → SSE 够用）
- [ ] SSE 在 Nginx / CDN 后兼容性（需 `X-Accel-Buffering: no` 头或 streaming pass-through）

### 决策记录

> _Owner 在此处填入_

---

## U-04：Agent Branch 心智模型

| 字段 | 值 |
|------|-----|
| 状态 | `Pending` |
| Owner | 待指派 |
| 默认推荐 | **A. 自动分支 `agent/{session-id}/{feature-slug}`**；顶栏显示当前分支 |
| 决策记录 | _待填_ |

### 候选方案

| 方案 | 描述 | 优点 | 缺点 |
|------|------|------|------|
| A. **自动分支（推荐）** | agent 每次 session 创建 `agent/{session-id}/{feature}` 分支；UI 顶栏显示 | 与 git 原生心智一致；与 `.evolith/policy.yaml` 的 auto_merge / require_review 直接配合 | 用户需要理解 branch 概念；多 session 分支命名冲突需处理 |
| B. session 独立 workspace | session 不映射 git branch；commit 时合并 | 隐藏 git 细节；UI 更简单 | commit 时合并逻辑复杂；无法 partial revert |
| C. 每条消息一个分支 | `agent/msg-{n}`；UI 显示分支链 | 极细粒度历史 | 分支爆炸；管理成本 |

### 待确认点

- [ ] 是否需要"work in progress"显式分支 vs 直推临时分支
- [ ] agent 完成任务后，分支是自动 archive 还是保留可继续迭代
- [ ] 多 agent 并行协作（同一 repo 两个 session）的分支命名冲突（建议用 session UUID 避免冲突）
- [ ] 顶栏分支切换 UX：dropdown 选择 vs 显示当前 active session

### 决策记录

> _Owner 在此处填入_

---

## U-05：直推直合三态视觉反馈

| 字段 | 值 |
|------|-----|
| 状态 | `Pending` |
| Owner | 待指派 |
| 默认推荐 | **状态徽章（file tree + branch 标签）+ commit 按钮旁 inline 提示** |
| 决策记录 | _待填_ |

### 候选方案

| 方案 | 描述 | 优点 | 缺点 |
|------|------|------|------|
| A. 状态徽章（推荐） | file tree 节点 + branch 标签加彩色徽章 | 视觉稳定；不抢焦点 | 首次使用需用户学习徽章含义 |
| B. commit 按钮颜色变化 | auto_merge 绿 / require_review 黄 / block 红 | 直观；按钮位置显眼 | 单点信号，不覆盖全场景 |
| C. toast 通知 | commit 完成后弹 toast | 即时反馈 | 易被忽略；不持续可见 |
| D. commit history 列表染色 | 历史列表按状态染色 | 信息密度高 | 历史一长难定位 |

### 待确认点

- [ ] 徽章颜色规范（沿用 [DESIGN.md](../reference/DESIGN.md) token：`semantic-success: #1ea64a` 绿 / `block-coral: #f3c9b6` 红 / `block-lime: #dceeb1` 黄）
- [ ] 是否需要"动效"提示状态变化（如徽章淡入淡出）
- [ ] block 状态是否需要阻止 UI 后续操作（form 禁用 + 红框）
- [ ] 用户首次使用时如何告知"auto_merge 已生效"（引导 onboarding / 文档链接）

### 决策记录

> _Owner 在此处填入_

---

## 实施期可定议题（不阻塞 EVO-104 进入迭代）

- **U-06** Diff viewer 形态（inline / side-by-side；多文件 diff 展示策略）— EVO-104 P1
- **U-07** Commit 消息生成（用户手写 / LLM 生成 / 模板填充）— EVO-104 P1
- **U-08** 文件冲突处理（agent vs user）— EVO-104 P1

> 这三项在迭代实施期可定，不阻塞 EVO-104 进入 Ready 状态。

## 后期扩展议题（out of MVP）

- **U-09** 实时多人协作（Yjs / CRDT）— Phase 6+
- **U-10** Live preview pane（web app sandbox + iframe）— Phase 5+
- **U-11** 终端面板（web shell 直连 repo sandbox）— Phase 5+
- **U-12** 文件拖拽上传 — Phase 5+
- **U-13** 键盘快捷键全键位 — Phase 5+

> 这五项明确 out of MVP scope，不在本设计 doc 决策。

## 决策产物存放规则

每个 U-XX 决策完成后：

1. 在本文件对应 section 的「决策记录」段填入最终方案
2. 状态从 `Pending` 改为 `Decided` + 决策日期
3. 重大决策（影响架构 / 安全 / 数据）写入 `docs/decisions/ADR-000X-*.md` 并在本文件添加 ADR 链接
4. 小决策在本文件直接记录即可
5. EVO-104 item file 的 `Required Reads` 同步追加本文件链接

## 启动门禁解除条件

U-01 ~ U-05 五项**全部** `Decided` 后，EVO-104 可从 `Proposed` 晋升 `Ready` 并选入迭代（推荐 ITERATION-NEW，紧接 ITERATION-042 Phase E'-1 完成后启动 Phase E'-2）。
