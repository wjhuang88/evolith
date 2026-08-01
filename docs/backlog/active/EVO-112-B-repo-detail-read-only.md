# EVO-112-B Repo Detail Read-only

## Required Reads

- Product Backlog: [`../PRODUCT-BACKLOG.md`](../PRODUCT-BACKLOG.md)
- 父 Epic: [`EVO-100`](EVO-100-git-centric-platform-foundation.md)
- 父 Story: [`EVO-112`](EVO-112-repo-management-ui.md)
- 同级子 Story: [`EVO-112-A`](EVO-112-A-repo-ui-shell.md)（Repo UI Shell，ITERATION-054，列表 + 创建 + 导航 + Dashboard 改版）
- Two-month plan: [`../../roadmap/TWO-MONTH-PLAN-2026-07.md`](../../roadmap/TWO-MONTH-PLAN-2026-07.md)
- Design system: [`../../reference/DESIGN.md`](../../reference/DESIGN.md)
- API contract: [`../../reference/API-CONTRACT.md`](../../reference/API-CONTRACT.md)（§18 Repo Context API）
- ADR-0004 / ADR-0006

## Summary

- 类型：feature / frontend
- 优先级：P0
- 状态：Proposed / paused（等待 EVO-118 S1）
- 父 Epic：`EVO-100`
- 父 Story：`EVO-112`
- Source: Two-Month Plan §3 Week 2 主线；EVO-112-A 创建后立刻需要的"详情入口"。

## Problem Or Outcome

`EVO-112-A` 已完成仓库列表 / 创建 / 主导航 / Dashboard 改版；创建成功暂时返回 `/repos`，不会跳到不存在的详情路由。EVO-118 S1 关闭后，再实现 `/repos/:id` 详情页，只读消费 EVO-103-C Repo Context API，把“能进入仓库、看到文件树与文件内容、看到 commit 历史、看到 settings”作为后续端到端切片。

## Goal And Non-Goals

### Goal

1. `/repos/:id` 详情页 Tab 布局，三个标签：
   - **Files Tab（默认）**：左侧文件树（调用 `GET /repos/{id}/file-tree?ref={default_branch}`）；右侧文件内容查看（点击文件节点调用 `GET /repos/{id}/blobs/{sha}`）；面包屑导航
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

## Dependencies And Blockers

- 硬依赖 `EVO-112-A`（Done，ITERATION-054）— 详情页复用列表、创建和 repo-centric 导航基础
- 硬依赖 `EVO-103-C`（Done，2026-06-26 Iteration 048）— `GET /repos/{id}/file-tree` / `blobs/{sha}` / `commits` 路由已可用
- 硬依赖 `EVO-116`（Done）— 权限边界、Context API 资源边界
- 软依赖 `EVO-102`（Done，policy.yaml parser）— Settings Tab 展示 policy 时调用
- 阻塞：EVO-118 S1（DATA-01 / DEPLOY-01）关闭前不启动实现

## Governing ADRs, Specs Or Decisions

- [ADR-0004 Git-Centric Storage](../../decisions/ADR-0004-git-centric-storage.md)
- [ADR-0006 Smart HTTP via git subprocess](../../decisions/ADR-0006-smart-http-via-git-subprocess.md)
- [DESIGN.md](../../reference/DESIGN.md)：所有颜色 / 字号 / 间距沿用现有 token

## Acceptance Criteria

### 用户故事 BDD 场景

> 作为已登录的 Evolith 开发者，
> 我希望点入仓库详情能看到文件树、文件内容和提交历史，
> 以便验证后端 Repo Context API 的可消费性并预览仓库内容。

```gherkin
Scenario: 用户进入仓库详情（Files Tab）
  Given 仓库存在且至少有一个 commit
  When 用户从 /repos 列表点击仓库卡片
  Then 浏览器跳转到 /repos/{id}
  And 默认进入 Files Tab
  And 左侧文件树显示仓库根目录条目
  And 右侧显示 README.md 或首个文本文件的预览
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

- [ ] `frontend/src/lib/api/repos.ts` 扩展：`fileTree(tenantId, repoId, ref?)` / `blob(tenantId, repoId, sha)` / `commits(tenantId, repoId, ref?, limit?)`
- [ ] `frontend/src/lib/api/repos.ts` 扩展：`update(tenantId, repoId, req)` / `delete(tenantId, repoId)`
- [ ] `frontend/src/app/repos/[id]/page.tsx` 实现 Tab 容器；内部 `<FilesTab>` / `<CommitsTab>` / `<SettingsTab>` 子组件
- [ ] `frontend/src/app/repos/[id]/files/page.tsx` 与 `commits/page.tsx` 与 `settings/page.tsx` 作为子路由，可被 deep-link 访问（如 `/repos/:id/commits`）
- [ ] `frontend/src/main-spa.tsx` 注册 `/repos/:id` / `/repos/:id/files` / `/repos/:id/commits` / `/repos/:id/settings`
- [ ] i18n 同步 `repos.detail.*` / `repos.files.*` / `repos.commits.*` / `repos.settings.*` / `repos.cloneUrl.*` 到 en.json 与 zh-CN.json
- [ ] 所有颜色 / 字号 / 间距使用 DESIGN.md token
- [ ] `bun run type-check` 0 errors
- [ ] `bun run build` 0 errors
- [ ] Playwright 截图：Files Tab（带 README 渲染）/ Commits Tab / Settings Tab / 404 状态
- [ ] `markdown link check` + `git diff --check` 通过

## Validation Evidence Required

1. `bun run type-check` / `bun run build` 0 errors
2. 手工冒烟：从 /repos 创建仓库 → `git commit --allow-empty` + `git push` 触发 EVO-116 metadata sync → 进 /repos/:id/files 看到 commit 后文件树
3. Playwright 截图 ≥ 4 张
4. `markdown link check` + `git diff --check` 通过
5. 后端 `cargo test --workspace` 不退化（如果前端无后端改动，本项是“已确认未跑”，归口于本 Story 实际启动时的 baseline）

## Residual Work Destination

- Code editor / write path / branch switch / diff viewer — 归 `EVO-104`
- Commit / promote 写路径 — 归 `EVO-105`
- File search within repo — 归 `EVO-104` 或后续 EVO
- Policy.yaml 在线编辑 — Phase 5+ 评估
- 文件内容高亮 / 二进制文件预览增强 — `EVO-104` 范围

## Source Snapshot

- Source: Two-Month Plan §3 Week 2 主线；EVO-112-A 后续
- Decision context: EVO-103-C Repo Context API 已 Done；EVO-112-A 创建后跳转到详情占位空白，需立即补
- Prior discussion: 用户 2026-06-29 明确"EVO-112-A 跳转到 `/repos/:id` 不允许假可用；要么立即实现详情，要么跳转到列表并明确告知；优先立即实现"
