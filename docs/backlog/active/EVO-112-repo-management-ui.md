# EVO-112 Repo Management UI

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- [Git-Centric Platform Proposal](../../proposals/GIT-CENTRIC-PLATFORM.md)
- [ADR-0004 Git-Centric Storage](../../decisions/ADR-0004-git-centric-storage.md)
- [Design System (Figma tokens)](../../reference/DESIGN.md)
- 父 Epic: [EVO-100](EVO-100-git-centric-platform-foundation.md)
- 依赖: [EVO-103](EVO-103-repo-context-and-smart-http.md)（Repo CRUD API + Repo Context API）

## Summary

- 类型：feature / frontend
- 优先级：P0
- 状态：Proposed
- 父 Epic: EVO-100
- Source: 用户反馈 2026-06-24（git 仓库管理页面缺失）

## Problem Or Outcome

Evolith 战略转型为 Git-centric 平台后，前端没有任何 Git 仓库相关页面。当前导航仍是 `Dashboard / Tools / Skills / CLI Interfaces`（旧定位），用户打开应用后看不到任何与 Git 仓库相关的内容。需要在 EVO-103（后端 API）和 EVO-104（完整 Vibe Coding 编辑器）之间插入一层基础仓库管理 UI，让平台真正以 Git 仓库为中心。

## Goal And Non-Goals

- **Goal**：
  1. 仓库列表页（`/repos`）：展示当前 tenant 所有仓库，支持搜索 / 筛选 / 排序
  2. 创建仓库页（`/repos/new`）：表单创建新仓库（name / description / visibility / default_branch / auto_merge / require_review）
  3. 仓库详情页（`/repos/:id`）：Tab 布局——Files（只读文件树 + 文件内容查看）/ Commits（提交历史）/ Settings（仓库元数据 + policy 可视化）
  4. 导航重构：侧边栏改为 `Repos / Dashboard / Settings`；旧 Tools / Skills / Interfaces 降级为二级菜单或隐藏
  5. Dashboard 改为 repo-centric：展示仓库数量、最近活动、快捷入口

- **Non-goals**：
  - 不实现代码编辑器（EVO-104 范围）
  - 不实现 AI Chat panel（EVO-104 范围）
  - 不实现 commit / promote 操作 UI（EVO-105 后端 + EVO-104 前端）
  - 不实现 diff viewer（EVO-104 范围）
  - 不实现 branch 切换 UI（EVO-104 范围）
  - 不实现 Skill / CLI / MCP 发现页（EVO-109 范围）
  - 不实现 git clone / push / pull 的 Smart HTTP 前端（后端 EVO-103 范围；前端仅需展示 clone URL）
  - 不删除旧 Tools / Skills / Interfaces 页面代码（保留路由可访问；仅从主导航移除）

## Pages

### 1. 仓库列表页（`/repos`）

- 卡片网格布局，每个仓库卡片显示：name、description、visibility badge、default_branch、last_commit_sha（截断）、last_committed_at（相对时间）
- 顶部搜索框（name / description 模糊匹配）
- 排序：最近更新 / 名称 / 创建时间
- 空状态：引导用户创建第一个仓库
- 右上角 "New Repo" 按钮
- API: `GET /api/v1/repos`

### 2. 创建仓库页（`/repos/new`）

- 表单字段：
  - name（必填，tenant 内唯一）
  - description（可选）
  - visibility（public / private，默认 private）
  - default_branch（默认 main）
  - auto_merge（开关，默认 on）
  - require_review（开关，默认 off）
- 创建成功后跳转到仓库详情页
- API: `POST /api/v1/repos`

### 3. 仓库详情页（`/repos/:id`）

Tab 布局，三个标签页：

#### Files Tab（默认）
- 左侧文件树（调用 `GET /repos/{id}/file-tree`）
- 右侧文件内容查看（调用 `GET /repos/{id}/blobs/{sha}`）
- 只读模式——无编辑器（EVO-104 范围）
- 顶部显示当前 ref（默认 main）、clone URL（可复制）
- 支持路径导航（面包屑）

#### Commits Tab
- 提交历史列表（调用 `GET /repos/{id}/commits`）
- 每条记录显示：sha（截断）、message（首行）、author、committed_at（相对时间）
- 分页加载

#### Settings Tab
- 仓库元数据编辑（name / description / visibility）
- 策略展示（`.evolith/policy.yaml` 解析后的可视化——default_action / protected_paths / agents 列表）
- 仓库删除（需二次确认）
- API: `GET/PATCH/DELETE /api/v1/repos/{id}`

### 4. 导航重构

当前侧边栏：
```
Dashboard / Tools / Skills / CLI Interfaces
```

改为：
```
Repos / Dashboard / Settings
```

- **Repos** → `/repos`（仓库列表，新主入口）
- **Dashboard** → `/dashboard`（repo-centric 仪表盘）
- **Settings** → `/tenant/settings`（组织设置，已有页面）
- 旧 Tools / Skills / Interfaces 路由保留（`/tools`、`/skills`、`/interfaces` 仍可访问），但从主导航移除

### 5. Dashboard 改版（repo-centric）

统计卡片从 `0 Tools / 0 Skills / 0 CLI Interfaces / 881 API Calls` 改为：
- `N Repos` — 仓库总数
- `N Branches` — 分支总数（如果 API 支持；否则用 commits 数）
- `N Commits This Month` — 本月提交数
- `N Storage Used` — 已用存储

快捷操作从 `Add Tool / Create Skill / Add CLI Interface` 改为：
- `New Repo` → `/repos/new`
- `Browse Repos` → `/repos`
- `Team Members` → `/tenant/members`
- `API Keys` → `/tenant/api-keys`

## UX Decisions Required

> 实施期可定，不阻塞进入迭代。

- [ ] **U-17 仓库卡片信息密度** — 候选：精简卡片（name + description + badge）/ 详细卡片（含 last commit message + contributor avatars）。默认推荐精简卡片。
- [ ] **U-18 文件树交互** — 候选：点击文件名直接展开内容（单栏）/ 左侧树 + 右侧内容（双栏）。默认推荐双栏。
- [ ] **U-19 旧页面降级策略** — 候选：完全隐藏旧导航 / 降级为 "Legacy" 二级菜单 / 保留但加 deprecated 标签。默认推荐 "Legacy" 二级菜单。

## Dependencies And Blockers

- 依赖 EVO-103（Repo CRUD API + Repo Context API: file-tree / blobs / commits）
- 依赖 EVO-101 `git_repos` 表（Done）
- 软依赖 EVO-102 `.evolith/policy.yaml` parser（Done；Settings Tab 展示策略时使用）
- 设计约束：所有 UI 实现须遵循 `docs/reference/DESIGN.md` 的 Figma token；颜色 / 字号 / 间距不允许硬编码

## Governing ADRs, Specs Or Decisions

- [ADR-0004 Git-Centric Storage](../../decisions/ADR-0004-git-centric-storage.md)

## Acceptance Criteria

- [ ] `/repos` 仓库列表页可用：卡片展示 + 搜索 + 排序 + 空状态
- [ ] `/repos/new` 创建仓库页可用：表单提交成功后跳转详情页
- [ ] `/repos/:id` 仓库详情页可用：Files / Commits / Settings 三个 Tab
- [ ] Files Tab：文件树 + 文件内容查看（只读）
- [ ] Commits Tab：提交历史列表（分页）
- [ ] Settings Tab：元数据编辑 + policy 可视化 + 删除确认
- [ ] 侧边栏导航改为 `Repos / Dashboard / Settings`
- [ ] Dashboard 统计卡片改为 repo-centric 数据
- [ ] 旧 Tools / Skills / Interfaces 路由仍可访问（不 404），但从主导航移除
- [ ] 所有 UI 颜色 / 字号使用 `docs/reference/DESIGN.md` token
- [ ] `bun run build` 0 errors
- [ ] `bun run type-check` 0 errors
- [ ] Playwright 截图：仓库列表（含空状态）/ 创建仓库 / 仓库详情 Files Tab / Commits Tab / Settings Tab / 新 Dashboard / 新导航

## Validation Evidence Required

- Playwright 截图覆盖所有新页面（桌面 + 移动端基础布局）
- 手工验收：创建仓库 → 查看文件树 → 查看提交历史 → 编辑设置 → 返回列表
- 视觉检查：导航重构后旧页面仍可路由访问

## Residual Work Destination

- 代码编辑器 + AI Chat（EVO-104）
- commit / promote / branch 切换 UI（EVO-104 + EVO-105）
- diff viewer（EVO-104）
- Skill / CLI / MCP 发现 UI（EVO-109）
- 移动端深度适配（后续 EVO 评估）

## Source Snapshot

- Source: 用户反馈 2026-06-24（"git 相关的页面都没有出现,应该有一个围绕 git 仓库为中心的逻辑"）
- Decision context: EVO-103 是纯后端 API，EVO-104 是完整 Vibe Coding 编辑器（阻塞于 UX U-01~U-05），中间缺少基础仓库管理 UI 层
- Prior discussion: 2026-06-24 Playwright 测试发现 Dashboard 仍显示旧 Tools/Skills/Interfaces 统计，导航无 Git 仓库入口
