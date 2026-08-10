# EVO-112-B Repo Detail Read-only

## Required Reads

- Product Backlog: [`../PRODUCT-BACKLOG.md`](../PRODUCT-BACKLOG.md)
- 父 Epic: [`EVO-100`](EVO-100-git-centric-platform-foundation.md)
- 父 Story: [`EVO-112`](EVO-112-repo-management-ui.md)
- 同级子 Story: [`EVO-112-A`](EVO-112-A-repo-ui-shell.md)（Repo UI Shell，ITERATION-054，列表 + 创建 + 导航 + Dashboard 改版）
- Two-month plan: [`../../roadmap/TWO-MONTH-PLAN-2026-07.md`](../../roadmap/TWO-MONTH-PLAN-2026-07.md)
- Design system: [`../../reference/DESIGN.md`](../../reference/DESIGN.md)
- Product interaction architecture: [`../../design/PRODUCT-INTERACTION-ARCHITECTURE.md`](../../design/PRODUCT-INTERACTION-ARCHITECTURE.md)
- ADR-0008: [`../../decisions/ADR-0008-repo-centric-interaction-architecture.md`](../../decisions/ADR-0008-repo-centric-interaction-architecture.md)
- API contract: [`../../reference/API-CONTRACT.md`](../../reference/API-CONTRACT.md)（§18 Repo Context API）
- ADR-0004 / ADR-0006

## Summary

- 类型：feature / frontend
- 优先级：P0
- 状态：Done / Complete（Iteration 061 Closed / Complete）
- 父 Epic：`EVO-100`
- 父 Story：`EVO-112`
- Source: Two-Month Plan §3 Week 2 主线；EVO-112-A 创建后立刻需要的"详情入口"。

## Problem Or Outcome

`EVO-112-A` 已完成仓库列表 / 创建 / 主导航 / Dashboard 改版；创建成功暂时返回 `/repos`，不会跳到不存在的详情路由。EVO-118-F/G/H 的生命周期、readiness 与事件边界稳定后，实现 Overview-first 的 `/repos/:id` 详情页，只读消费 EVO-103-C Repo Context API；最终 DEPLOY-01 不再是本 Story 的开发前置。

## Goal And Non-Goals

### Goal

1. `/repos/:id` 规范化到 `/repos/:id/overview`，本 Story 提供四个只读/管理入口：
   - **Overview（默认）**：README 或空仓状态、default branch、Policy 状态、latest commit，以及 Browse code 主操作；Start Workspace 只在 EVO-104 可用后由其加入
   - **Files**：左侧文件树（调用 `GET /repos/{id}/file-tree?ref={default_branch}`）；右侧文件内容查看（点击文件节点调用 `GET /repos/{id}/blobs/{sha}`）；面包屑导航
   - **Commits Tab**：提交历史列表（调用 `GET /repos/{id}/commits?ref={default_branch}&limit=50`）；每条记录显示 sha（截断 7 位）/ message（首行）/ author / committed_at（相对时间）；分页 / 滚动加载
   - **Settings Tab**：仓库元数据编辑（name / description / visibility / auto_merge / require_review）；`.evolith/policy.yaml` 解析结果可视化（如有）；clone URL 展示与复制；删除按钮（二次确认 Modal）
2. 顶部 header：当前 ref（默认 `main`，可显示但暂不切换 — 切换 UI 归 EVO-104）/ clone URL（可复制）
3. 详情页未找到时：友好 404 卡片 + 返回列表链接
4. 与 EVO-112-A 列表 / 创建保持设计语言一致；颜色 / 字号 / 间距全部沿用 DESIGN.md token

### Non-goals（显式排除）

- 不实现代码编辑器 / 写文件 — 归 `EVO-104`
- 不实现 commit 创建 / promote — 归 `EVO-105`
- 不实现 branch 切换 / tag 浏览 — 归 `EVO-104`
- 不实现 diff viewer（blob 内部仅做高亮） — 归 `EVO-104`
- 不实现 policy.yaml 编辑 — 只读展示
- 不实现跨仓文件搜索 — 归 `EVO-109`
- 不实现 Workspace 页面或 Agent Session — 归 `EVO-104`
- 不注册空的 Resources Tab 或 unavailable 占位 — 真实 Resources 路由与数据归 `EVO-109`

## Dependencies And Blockers

- 硬依赖 `EVO-112-A`（Done，ITERATION-054）— 详情页复用列表、创建和 repo-centric 导航基础
- 硬依赖 `EVO-103-C`（Done，2026-06-26 Iteration 048）— `GET /repos/{id}/file-tree` / `blobs/{sha}` / `commits` 路由已可用
- 硬依赖 `EVO-116`（Done）— 权限边界、Context API 资源边界
- 软依赖 `EVO-102`（Done，policy.yaml parser）— Settings Tab 展示 policy 时调用
- 启动处置：EVO-118-F 与 G-B/C/D 已完成；H 已交付最小 Outbox schema/repository 边界并以 Partial 保留后续接入。用户要求继续产品开发后，本 Story 在 Iteration 061 启动；最终 DEPLOY-01 不作为开发前置条件。

## Governing ADRs, Specs Or Decisions

- [ADR-0004 Git-Centric Storage](../../decisions/ADR-0004-git-centric-storage.md)
- [ADR-0006 Smart HTTP via git subprocess](../../decisions/ADR-0006-smart-http-via-git-subprocess.md)
- [ADR-0008 Repo-centric Interaction Architecture](../../decisions/ADR-0008-repo-centric-interaction-architecture.md)
- [Product Interaction Architecture](../../design/PRODUCT-INTERACTION-ARCHITECTURE.md)
- [DESIGN.md](../../reference/DESIGN.md)：所有颜色 / 字号 / 间距沿用现有 token

## Acceptance Criteria

### 用户故事 BDD 场景

> 作为已登录的 Evolith 开发者，
> 我希望点入仓库详情能看到文件树、文件内容和提交历史，
> 以便验证后端 Repo Context API 的可消费性并预览仓库内容。

```gherkin
Scenario: 用户进入仓库详情（Overview）
  Given 仓库存在且至少有一个 commit
  When 用户从 /repos 列表点击仓库卡片
  Then 浏览器跳转到 /repos/{id}
  And 默认进入 Overview
  And 页面显示 README 或明确的空仓状态
  And 页面显示 default branch、Policy 状态和 latest commit
  And 页面提供可用的 Browse code
  And EVO-104 未完成时不显示失效的 Start Workspace 操作
  And 顶部展示仓库名 + visibility badge + clone URL（可复制）

Scenario: 用户浏览文件树并查看文件内容
  Given 用户在 Files Tab
  When 用户点击 README.md 节点
  Then 右侧切换为 README.md 完整内容（utf-8 文本）
  And 面包屑显示 "Repo Name / README.md"
  And blob 接口的 base64 encoding 在 NUL 字节情况下被识别

Scenario: 用户查看提交历史
  Given 仓库有 1+ commits
  When 用户点击 "Commits" Tab
  Then 显示 commit 列表（最多 50 条）
  And 每条显示 sha 前 7 位 / message 首行 / author / 相对时间
  And 调用 /repos/{id}/commits?limit=50 成功

Scenario: 用户编辑 Settings
  Given 用户有 write 权限
  When 用户在 Settings Tab 修改 description 并保存
  Then 调用 PATCH /repos/{id}
  And UI 展示 "Saved" 状态
  And 返回列表时新值已生效

Scenario: 用户复制 clone URL
  Given 用户在 Files Tab
  When 用户点击 "Copy clone URL" 按钮
  Then 浏览器剪贴板写入 smart HTTP URL
  And 按钮文本短暂切换为 "Copied!"

Scenario: 用户删除仓库
  Given 用户有 write 权限
  When 用户在 Settings Tab 点击 "Delete repo"
  And 在二次确认 Modal 中输入仓库名
  Then 调用 DELETE /repos/{id}
  And 浏览器跳转到 /repos
  And 列表中该仓库不再存在

Scenario: 用户访问不存在的 repo
  Given repo_id 在 tenant 内不存在
  When 用户访问 /repos/{id}
  Then 页面显示 404 占位 + 返回列表链接
  And 不抛错或白屏
```

### 技术验收

- [x] `frontend/src/lib/api/repos.ts` 扩展：`fileTree(tenantId, repoId, ref?)` / `blob(tenantId, repoId, sha)` / `commits(tenantId, repoId, ref?, limit?)`
- [x] `frontend/src/lib/api/repos.ts` 扩展：`update(tenantId, repoId, req)` / `delete(tenantId, repoId)`
- [x] `frontend/src/app/repos/[id]/page.tsx` 实现 Repo 容器，并默认进入 Overview
- [x] `overview` / `files` / `commits` / `settings` 均可通过子路由 deep-link 访问
- [x] `frontend/src/main-spa.tsx` 注册 `/repos/:id` 规范化跳转及四个 Repo 子路由
- [x] i18n 的 Repo Detail 文案统一归口于 `repos.detail.*`，en.json 与 zh-CN.json key parity 通过
- [x] 所有颜色 / 字号 / 间距使用 DESIGN.md token
- [x] `bun run type-check` 0 errors
- [x] `bun run build` 0 errors
- [x] 真实 Headless Chrome 截图：Files Tab（带 README 渲染）/ Commits Tab / Settings Tab / 404 状态，并补充 Overview 与 390px 移动端
- [x] `markdown link check` + `git diff --check` 通过

## Validation Evidence Required

1. `bun run type-check` / `bun run build` 0 errors
2. 手工冒烟：从 /repos 创建仓库 → `git commit --allow-empty` + `git push` 触发 EVO-116 metadata sync → 进 /repos/:id/files 看到 commit 后文件树
3. Playwright 截图 ≥ 4 张
4. `markdown link check` + `git diff --check` 通过
5. 后端 `cargo test --workspace` 不退化（如果前端无后端改动，本项是“已确认未跑”，归口于本 Story 实际启动时的 baseline）

## Current Execution

- 已实现 `/repos/:id` 到 Overview 的规范化、四个可 deep-link 子路由、Repo Context/CRUD 接线、clone URL 复制、完整 Settings、输入仓库名的删除确认、嵌套目录、文本/空/二进制状态、相对时间与有界加载。
- 已通过 `bun run type-check`、`bun run build`、`bun run lint`；Markdown link、治理校验与 `git diff --check` 的最终结果记录在 Iteration 061。
- 真实 Headless Chrome 验证覆盖创建/进入 Repo、README/嵌套目录/空文件/二进制文件、50 条上限的 commits 请求、Settings PATCH/Saved、剪贴板、临时仓库 DELETE 后列表消失、404、桌面与 390px 移动端无横向溢出。截图归档于 `docs/iterations/screenshots/iter-061/`。
- 本 Story 未修改后端，按启动基线未重跑 `cargo test --workspace`；运行态使用真实本地 backend、SQLite 与临时 Git storage 验收，不把前序后端改动计入本 Story。
- 流程偏差：实现代码先于 Iteration 061 治理记录产生；Iteration 061 明确保留该事实并承担后续验证，不能据此伪装为正常的先计划后实施。

## Residual Work Destination

- Code editor / write path / branch switch / diff viewer — 归 `EVO-104`
- Commit / promote 写路径 — 归 `EVO-105`
- First-run 创建 Repo 并进入 Overview — 归 `EVO-120`
- Repo Resources 与跨仓 Discover — 归 `EVO-109`
- File search within repo — 归 `EVO-104` 或后续 EVO
- Policy.yaml 在线编辑 — Phase 5+ 评估
- 文件内容高亮 / 二进制文件预览增强 — `EVO-104` 范围

## Source Snapshot

- Source: Two-Month Plan §3 Week 2 主线；EVO-112-A 后续
- Decision context: EVO-103-C Repo Context API 已 Done；EVO-112-A 创建后跳转到详情占位空白，需立即补
- Prior discussion: 用户 2026-06-29 明确"EVO-112-A 跳转到 `/repos/:id` 不允许假可用；要么立即实现详情，要么跳转到列表并明确告知；优先立即实现"
