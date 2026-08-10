# EVO-112 Repo Management UI

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- [Git-Centric Platform Proposal](../../proposals/GIT-CENTRIC-PLATFORM.md)
- [ADR-0004 Git-Centric Storage](../../decisions/ADR-0004-git-centric-storage.md)
- [Design System — Figma tokens](../../reference/DESIGN.md)
- [Product Interaction Architecture](../../design/PRODUCT-INTERACTION-ARCHITECTURE.md)
- [ADR-0008 Repo-centric Interaction Architecture](../../decisions/ADR-0008-repo-centric-interaction-architecture.md)
- 父 Epic: [EVO-100](EVO-100-git-centric-platform-foundation.md)
- 依赖: [EVO-103](EVO-103-repo-context-and-smart-http.md)（Repo CRUD API + Repo Context API）
- 子 Story: [EVO-112-A](EVO-112-A-repo-ui-shell.md)（Repo UI Shell / ITERATION-054）、[EVO-112-B](EVO-112-B-repo-detail-read-only.md)（Repo Detail Read-only）、[EVO-112-C](EVO-112-C-repo-commit-evidence-detail.md)（Commit Evidence）

## Sub-Stories

| 子 Story | 独立结果 | 状态 | 依赖 | 所属迭代 |
|----------|----------|------|------|----------|
| EVO-112-A | `/repos` 列表 + `/repos/new` 创建 + repo-centric 导航 + Dashboard repo-centric 改版 | Done | EVO-103-A (Done) + EVO-116 (Done) | ITERATION-054（历史成果恢复） |
| EVO-112-B | `/repos/:id` Overview / Files / Commits / Settings | Done / Complete | EVO-112-A + EVO-103-C (Done) + EVO-118-F/G/H 边界 | ITERATION-061（Closed / Complete） |
| EVO-112-C | `/repos/:id/commits/:sha` 可追溯 Commit 证据页 | Done / Complete | EVO-112-B + EVO-103-C | ITERATION-063（Closed / Complete） |

父项完成条件：三个子 Story 全部 Done，Repo 与 Commit 结果 deep link 均真实可用。

## Summary

- 类型：feature / frontend
- 优先级：P0
- 状态：Done / Complete（EVO-112-A/B/C 全部 Done；Iterations 054/061/063 Closed / Complete）
- 父 Epic: EVO-100
- Source: 用户反馈 2026-06-24（git 仓库管理页面缺失）

## Problem Or Outcome

Evolith 战略转型为 Git-centric 平台后，前端没有任何 Git 仓库相关页面。当前导航仍是 `Dashboard / Tools / Skills / CLI Interfaces`（旧定位），用户打开应用后看不到任何与 Git 仓库相关的内容。需要在 EVO-103（后端 API）和 EVO-104（完整 Vibe Coding 编辑器）之间插入一层基础仓库管理 UI，让平台真正以 Git 仓库为中心。

## Goal And Non-Goals

- **Goal**：
  1. 仓库列表页（`/repos`）：展示当前 tenant 所有仓库，支持搜索 / 筛选 / 排序
  2. 创建仓库页（`/repos/new`）：表单创建新仓库（name / description / visibility / default_branch / auto_merge / require_review）
  3. 仓库详情页（`/repos/:id`）：Overview-first 布局——Overview / Files / Commits / Settings
  4. EVO-112-A 已交付历史第一阶段导航；最终 App Shell、Settings IA 与旧 UI 删除归 EVO-121
  5. Dashboard 改为 repo-centric：展示仓库数量、最近活动、快捷入口

- **Non-goals**：
  - 不实现代码编辑器（EVO-104 范围）
  - 不实现 AI Chat panel（EVO-104 范围）
  - 不实现 commit / promote 操作 UI（EVO-105 后端 + EVO-104 前端）
  - 不实现 diff viewer（EVO-104 范围）
  - 不实现 branch 切换 UI（EVO-104 范围）
  - 不实现 Skill / CLI / MCP 发现页（EVO-109 范围）
  - 不实现 git clone / push / pull 的 Smart HTTP 前端（后端 EVO-103 范围；前端仅需展示 clone URL）
  - 本父项不删除旧 Tools / Skills / Interfaces 页面代码；最终删除归 EVO-121-F，不形成 UI 兼容承诺

## Pages

### 1. 仓库列表页（`/repos`）

- 卡片网格布局，每个仓库卡片显示：name、description、visibility badge、default_branch、last_commit_sha（截断）、last_committed_at（相对时间）
- 顶部搜索框（name / description 模糊匹配）
- 排序：最近更新 / 名称 / 创建时间
- 空状态：引导用户创建第一个仓库
- 右上角 "New Repo" 按钮
- API: `GET /api/v1/tenant/{tenant_id}/repos`

### 2. 创建仓库页（`/repos/new`）

- 表单字段：
  - name（必填，tenant 内唯一）
  - description（可选）
  - visibility（public / private，默认 private）
  - default_branch（默认 main）
  - auto_merge（开关，默认 off；需 admin 显式开启）
  - require_review（开关，默认 on）
- 创建成功后跳转到仓库详情页
- API: `POST /api/v1/tenant/{tenant_id}/repos`

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
- API: `GET/PATCH/DELETE /api/v1/tenant/{tenant_id}/repos/{id}`

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
- EVO-112-A 历史切片暂时保留旧路由作为当前实现事实；ADR-0008 已决定最终由 EVO-121-F 直接删除

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

## UX Decisions

> 已按最终产品任务流定案。

- [x] **U-17 仓库列表信息密度** — 桌面采用可扫描的紧凑列表/行，显示 name、visibility、default branch、last commit/time；移动端使用堆叠行，不采用装饰性卡片网格或 contributor avatars。
- [x] **U-18 文件树交互** — 桌面左侧树 + 右侧 blob 双栏；移动端文件树 drawer + 全宽 blob，保留路径面包屑。
- [x] **U-19 旧页面策略** — ADR-0008 已决定：产品未上线，不建设 Legacy/迁移体验；Discover/Resources 可用后由 EVO-121-F 直接删除旧 UI。

## Dependencies And Blockers

- 依赖 EVO-103（Repo CRUD API + Repo Context API: file-tree / blobs / commits）
- 依赖 EVO-101 `git_repos` 表（Done）
- 软依赖 EVO-102 `.evolith/policy.yaml` parser（Done；Settings Tab 展示策略时使用）
- 设计约束：所有 UI 实现须遵循 `docs/reference/DESIGN.md` 的 Figma token；颜色 / 字号 / 间距不允许硬编码

## Governing ADRs, Specs Or Decisions

- [ADR-0004 Git-Centric Storage](../../decisions/ADR-0004-git-centric-storage.md)
- [ADR-0008 Repo-centric Interaction Architecture](../../decisions/ADR-0008-repo-centric-interaction-architecture.md)

## Acceptance Criteria

- [ ] `/repos` 仓库列表页可用：卡片展示 + 搜索 + 排序 + 空状态
- [ ] `/repos/new` 创建仓库页可用：表单提交成功后跳转详情页
- [ ] `/repos/:id` 规范化到 Overview；Overview / Files / Commits / Settings 可 deep-link
- [ ] Files Tab：文件树 + 文件内容查看（只读）
- [ ] Commits Tab：提交历史列表（分页）
- [ ] Settings Tab：元数据编辑 + policy 可视化 + 删除确认
- [ ] 侧边栏导航改为 `Repos / Dashboard / Settings`
- [ ] Dashboard 统计卡片改为 repo-centric 数据
- [ ] 本父项最终验收不要求旧 UI 路由兼容；删除归 EVO-121-F
- [ ] 所有 UI 颜色 / 字号使用 `docs/reference/DESIGN.md` token
- [ ] `bun run build` 0 errors
- [ ] `bun run type-check` 0 errors
- [ ] Playwright 截图：仓库列表（含空状态）/ 创建仓库 / 仓库详情 Files Tab / Commits Tab / Settings Tab / 新 Dashboard / 新导航

## Validation Evidence Required

- Playwright 截图覆盖所有新页面（桌面 + 移动端基础布局）
- 手工验收：创建仓库 → 查看文件树 → 查看提交历史 → 编辑设置 → 返回列表
- 视觉检查：Repo 页面与目标设计系统一致；最终导航收敛归 EVO-121-A

## Residual Work Destination

- 代码编辑器 + AI Chat（EVO-104）
- commit / promote / branch 切换 UI（EVO-104 + EVO-105）
- diff viewer（EVO-104）
- Skill / CLI / MCP 发现 UI（EVO-109）
- 最终 App Shell / Dashboard / Settings / Activity / Legacy UI 删除（EVO-121）
- 移动端深度适配（后续 EVO 评估）

## Closure Record

- EVO-112 已按 A/B/C 三个可独立验收的子 Story 交付：Repo shell/list/create、Overview-first Repo detail、稳定 Commit evidence deep link。
- EVO-112-C 在 Iteration 063 完成 tenant-scoped detail API、Ref reachability、root/structured diff、浏览器正常/404/390px 证据与全量门禁。
- 父项早期页面清单中的细节以拆分后的子 Story BDD 为验收 owner；Commits 当前明确为 bounded `limit=50`，文本 diff、Workspace/Agent/Activity 和最终 App Shell 仍由 EVO-104/105/121 承接，不作为 EVO-112 残余缺口。
- 父项闭环结论：`Complete`。

## Source Snapshot

- Source: 用户反馈 2026-06-24（"git 相关的页面都没有出现,应该有一个围绕 git 仓库为中心的逻辑"）
- Decision context: EVO-103 是纯后端 API，EVO-104 是完整 Vibe Coding 编辑器（阻塞于 UX U-01~U-05），中间缺少基础仓库管理 UI 层
- Prior discussion: 2026-06-24 Playwright 测试发现 Dashboard 仍显示旧 Tools/Skills/Interfaces 统计，导航无 Git 仓库入口
- 2026-06-29 拆分：本 Story 拆为 EVO-112-A（列表 / 创建 / 导航 / Dashboard）+ EVO-112-B（详情三 Tab）。拆分理由：单 Story 不超过 0.5-2 天交付窗口；EVO-112-A 是 EVO-112-B 的硬依赖，但两者范围独立可分别验收。
- 2026-08-02 恢复：EVO-112-A 的本地历史成果因主线已占用 Iteration 050，迁移到 Iteration 054；EVO-112-B/C 在 F/G/H 边界稳定后重新排期，不等待最终 EVO-118-E。
