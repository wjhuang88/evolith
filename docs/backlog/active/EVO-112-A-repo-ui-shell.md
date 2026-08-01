# EVO-112-A Repo UI Shell

## Required Reads

- Product Backlog: [`../PRODUCT-BACKLOG.md`](../PRODUCT-BACKLOG.md)
- 父 Epic: [`EVO-100`](EVO-100-git-centric-platform-foundation.md)
- 父 Story: [`EVO-112`](EVO-112-repo-management-ui.md)
- 同级子 Story: [`EVO-112-B`](EVO-112-B-repo-detail-read-only.md)（详情只读；EVO-118 S1 后恢复）
- Two-month plan: [`../../roadmap/TWO-MONTH-PLAN-2026-07.md`](../../roadmap/TWO-MONTH-PLAN-2026-07.md)
- Design system: [`../../reference/DESIGN.md`](../../reference/DESIGN.md)（Figma tokens；颜色 / 字号 / 间距必须用 token，不硬编码）
- API contract: [`../../reference/API-CONTRACT.md`](../../reference/API-CONTRACT.md)（§16 Repos）
- 治理：ADR-0004 Git-Centric Storage；ADR-0006 Smart HTTP via git subprocess
- 依赖：`EVO-103-A`（Done，Repo CRUD）、`EVO-116`（Done，权限边界与资源边界）

## Summary

- 类型：feature / frontend
- 优先级：P0
- 状态：Done
- 父 Epic：`EVO-100`
- 父 Story：`EVO-112`
- 所属迭代：ITERATION-054（2026-08-02 恢复本地历史成果；Closed / Complete）
- Source: 用户反馈 2026-06-24（"git 相关的页面都没有出现"）；Two-Month Plan §3 Week 1 主线。
- Merge evidence: [PR #8](https://github.com/wjhuang88/evolith/pull/8) merged as `1216624`；exact-head CI run `30712179586` passed；Navigator no blocking findings。

## Problem Or Outcome

Evolith 战略转型为 Git-centric 平台后，前端没有任何 Git 仓库相关页面。当前导航仍是 `Dashboard / Tools / Skills / CLI Interfaces`（旧定位），用户打开应用后看不到任何与 Git 仓库相关的内容。本 Story 把"仓库管理 UI"的第一屏骨架搭建起来，让平台以 Git 仓库为中心；详情页（Files / Commits / Settings）和编辑能力在 `EVO-112-B` 中独立交付。

## Goal And Non-Goals

### Goal

1. 仓库列表页 `/repos`：
   - 卡片网格展示当前 tenant 所有仓库
   - 顶部搜索框（name / description 模糊匹配）
   - 排序：最近更新 / 名称 / 创建时间
   - 空状态：引导创建第一个仓库
   - 右上角 "New Repo" 按钮
2. 创建仓库页 `/repos/new`：
   - 表单：name（必填，tenant 内唯一）/ description / visibility（public / private，默认 private）/ default_branch（默认 main）/ auto_merge（默认 off）/ require_review（默认 on）
   - 成功后返回 `/repos`，避免详情页交付前跳到不存在的路由
3. 侧边栏导航重构为 repo-centric：
   - 主导航：`Repos` / `Dashboard` / `Settings`
   - 旧 `Tools / Skills / Interfaces` 降级为 "Legacy" 二级菜单（仍可路由访问，但不作为主入口）
4. Dashboard 改为 repo-centric：
   - 统计卡片改为仓库总数 / 最近 commit 数 / 成员数 / 本月 commit
   - 快捷操作改为 `New Repo` / `Browse Repos` / `Team Members` / `API Keys`
5. 所有 UI 颜色 / 字号 / 间距严格使用 DESIGN.md token，不硬编码 hex / px

### Non-goals（显式排除）

- 不实现 `/repos/:id` 详情页（Files / Commits / Settings 三 Tab）— 归 `EVO-112-B`
- 不实现代码编辑器、AI Chat、commit / promote / branch 切换 UI — 归 `EVO-104`
- 不实现 diff viewer — 归 `EVO-104`
- 不实现 Skill / CLI / MCP 发现页 — 归 `EVO-109`
- 不实现 Smart HTTP clone / push / pull 前端 — 后端 `EVO-103-B` 范围；前端仅在 Settings Tab 展示 clone URL（属于 `EVO-112-B`）
- 不删除旧 Tools / Skills / Interfaces 页面代码 — 保留路由可访问；从主导航降级
- 不引入新的设计 token；颜色 / 字号 / 间距沿用 DESIGN.md 现有定义（必要时复用现有 Tailwind 配置变量）
- 不动后端 Rust 代码（`EVO-103-A` 已稳定）

## Dependencies And Blockers

- 依赖 `EVO-103-A`（Done，2026-06-25 Iteration 044）：`GET/POST/PATCH/DELETE /api/v1/tenant/{tenant_id}/repos` 路由已可用
- 依赖 `EVO-116`（Done，2026-06-26 Iteration 049）：权限边界、Context API 资源边界、push metadata、API contract、性能证据
- 软依赖 `EVO-101`（Done）、`EVO-102`（Done）
- 软依赖 `EVO-044`（Proposed，前端 CLI 命名简化）— 暂未实现，但本 Story 内不做 CLI 页面文案改动
- 阻塞：无

## Governing ADRs, Specs Or Decisions

- [ADR-0004 Git-Centric Storage](../../decisions/ADR-0004-git-centric-storage.md)：本 Story 是该决策的可视化入口
- [ADR-0006 Smart HTTP via git subprocess](../../decisions/ADR-0006-smart-http-via-git-subprocess.md)：影响 Smart HTTP URL 展示（仅 `EVO-112-B` 涉及）
- [DESIGN.md](../../reference/DESIGN.md)：所有颜色 / 字号 / 间距必须遵循现有 token（block-lime / block-lilac / pill / rounded-lg / typography.body 等）

## Acceptance Criteria

### 用户故事 BDD 场景

> 作为已登录的 Evolith 开发者，
> 我希望登录后第一屏能看到仓库并能创建新仓库，
> 以便把 Evolith 当作 Git 平台而非工具/技能注册中心。

```gherkin
Scenario: 已登录用户看到新导航与 Dashboard repo-centric 数据
  Given 用户已登录且 tenant 内有 0 个或更多仓库
  When 用户访问任意带 main layout 的页面
  Then 侧边栏主导航依次显示 "Repos / Dashboard / Settings"
  And "Repos" 链接指向 /repos
  And "Dashboard" 链接指向 /dashboard
  And "Settings" 链接指向 /tenant/settings
  And 旧 Tools / Skills / Interfaces 出现在 "Legacy" 二级菜单中（折叠可见）
  And Dashboard 页面顶部出现 "Repositories" / "Recent commits" 统计卡片

Scenario: 用户浏览仓库列表（空状态）
  Given tenant 内尚无任何仓库
  When 用户访问 /repos
  Then 页面显示空状态："Create your first repository" 引导文案
  And 右上角 "New Repo" 按钮可见且可点击
  And 不显示报错或 loading 骨架

Scenario: 用户浏览仓库列表（非空）
  Given tenant 内存在 N 个仓库
  When 用户访问 /repos
  Then 仓库以卡片网格展示（响应式：1 / 2 / 3 列）
  And 每个卡片显示 name / description / visibility badge / default_branch / last_committed_at 相对时间
  And 顶部搜索框输入时可按 name / description 模糊匹配（前端 client-side filter）
  And 排序切换为最近更新 / 名称 / 创建时间

Scenario: 用户创建仓库
  Given 用户已登录且 tenant 有写权限
  When 用户访问 /repos/new
  And 填写 name = "demo-repo"、description = "demo"、visibility = private、auto_merge = false、require_review = true
  And 点击 "Create repository"
  Then 浏览器跳转到 /repos
  And 后端返回 201 + RepoResponse（name / tenant 内唯一）
  And 新仓库出现在 /repos 列表顶部

Scenario: 旧 Tools / Skills / Interfaces 路由仍可访问
  Given 用户从 /dashboard 点击 Legacy 菜单的 "Tools"
  When 浏览器加载 /tools
  Then 页面正常渲染（不 404）
  And URL 不被改写到 /repos

Scenario: 未登录用户访问 /repos
  Given 用户未登录
  When 用户访问 /repos
  Then AuthGuard 重定向到 /login?redirect=/repos
```

### 技术验收

- [x] `frontend/src/lib/api/repos.ts` 新增 `reposApi`：`list(tenantId)` / `create(tenantId, req)` / `get(tenantId, repoId)`
- [x] `frontend/src/lib/api/types.ts` 新增 `Repo` 类型（与 `RepoResponse` 对齐；字段 `last_commit_sha: string | null` / `last_committed_at: string | null`）
- [x] `frontend/src/components/layout/Header.tsx` 侧边栏 `navigation` 改为 `Repos / Dashboard / Settings`，加 Legacy 二级菜单（折叠展开的次级 nav）
- [x] `frontend/src/app/repos/page.tsx` 实现 `/repos`（卡片 + 搜索 + 排序 + 空状态）
- [x] `frontend/src/app/repos/new/page.tsx` 实现 `/repos/new`（表单 + 提交 + 返回列表）
- [x] `frontend/src/app/dashboard/page.tsx` 改为 repo-centric：调用 `reposApi.list` + `authStore.user` + `membersApi.list`（不再调用 `toolsApi.list` / `skillsApi.list` / `cliInterfacesApi.list` 作为主统计）
- [x] `frontend/src/main-spa.tsx` 注册 `/repos` 和 `/repos/new` 路由（详情页路由属于 `EVO-112-B`，暂不挂载）
- [x] `frontend/src/locales/en.json` 和 `frontend/src/locales/zh-CN.json` 同步新增 `nav.repos` / `nav.legacy` / `repos.*` / `dashboard.repoCentric.*` 文案
- [x] 所有颜色 / 字号 / 间距使用 DESIGN.md token（`bg-[var(--primary)]` / `rounded-[50px]` / `rounded-[24px]` / `text-sm` / `text-base` 等已存在变量；不引入新的 hex 或 px）
- [x] `bun run type-check` 0 errors
- [x] `bun run build` 0 errors
- [x] Playwright 截图（桌面 1280×800 + 移动 390×844）：
  - `/repos` 空状态
  - `/repos` 列表（1+ 仓库）
  - `/repos/new` 表单（填一半）
  - `/dashboard` 新统计卡片
  - 侧边栏 Legacy 二级菜单展开（看旧 Tools/Skills/Interfaces 入口）
- [x] `markdown link check`（[DOC-CHECK](../../sop/DOC-CHECK.md) 命令）全通过
- [x] `git diff --check` clean

## Validation Evidence Required

1. `bun run type-check` 输出 `0 errors`
2. `bun run build` 输出成功（无 error，含 "✓ built in" 字样）
3. Python `markdown link check` 输出 `all markdown links exist`
4. `git diff --check` 输出 `clean`
5. Playwright 截图 ≥ 5 张（覆盖上述场景）
6. 手工冒烟（Playwright 实际操作）：注册 → 登录 → /repos → 创建 "demo-repo" → 返回 /repos → 列表顶部出现新仓库
7. Legacy 路由冒烟：从 /dashboard 点 Legacy → Tools → /tools 渲染旧 Tools 列表（不报错）

## Residual Work Destination

- `/repos/:id` 详情页（Files / Commits / Settings Tab）— 归 `EVO-112-B`（EVO-118 S1 后重新排期）
- Dashboard "Recent commits" 列表（按 repo 聚合）— 需要 commit 历史 API（`EVO-105` 范围）；本 Story 仅做静态计数（`repos[i].last_commit_sha` 存在即 +1），不做"最近 5 条 commit 列表"
- Repo 详情 Settings Tab 的 policy.yaml 可视化、删除二次确认、clone URL 复制 — `EVO-112-B`
- 移动端深度适配（NavigationDrawer / hamburger） — 本 Story 保留现有 MobileNav 模式，未做深度改造；后续 EVO 评估
- 旧 Tools / Skills / Interfaces 页面迁移为 Pages 式发现 — 归 `EVO-109`
- `EVO-044` 前端 CLI 命名简化 — 与本 Story 解耦，可独立微迭代

## Source Snapshot

- Source: 用户反馈 2026-06-24（"git 相关的页面都没有出现"）+ Two-Month Plan §3 Week 1 主线 + EVO-116 验收硬化的 EVO-103 稳定基础
- Decision context: EVO-103 是纯后端 API；EVO-104 是完整 Vibe Coding 编辑器；中间需要一层"仓库管理 UI 骨架"作为后续所有写路径（commit / promote / agent session）的入口
- Prior discussion: 2026-06-29 用户明确"先把仓库列表 + 创建 + 导航 + Dashboard 改版做掉；详情页另起 EVO-112-B"
