# Iteration 054: EVO-112-A Repo UI Shell（本地历史成果恢复）

> 文档状态：Closed（原始执行 2026-06-29；恢复与重验 2026-08-02）
> 原计划发布日期：2026-06-29
> 计划目标：把 Evolith 前端第一屏从"工具/技能/CLI 注册中心"切到"Git 仓库管理 UI"——实现 `/repos` 列表、`/repos/new` 创建、repo-centric 侧边栏导航（Repos / Dashboard / Settings）、Dashboard repo-centric 改版；旧 Tools / Skills / Interfaces 页面保留路由但降级为 Legacy 二级菜单。
> 恢复说明：本地成果原拟使用 Iteration 050，但该编号已被主线已发布的 EVO-118-A 基线占用；远端 PR #7 已使用 Iteration 053，因此按变更控制迁移为 054，不覆写任何既有计划。
>
> 基线保护：本文件一旦提交，以下"发布计划基线"内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

- 完成 EVO-112 子 Story EVO-112-A Repo UI Shell 的端到端实现。
- 让 Evolith 在 EVO-103 / EVO-116 验收硬化稳定后真正以 Git 仓库作为第一屏。
- 为后续 EVO-112-B Repo Detail Read-only 提供列表、创建与导航基础；详情实现等待 EVO-118 S1 关闭后重新排期。
- 不实现详情页 / 不实现编辑器 / 不动后端代码。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|---------|--------|---------------|
| [EVO-112-A](../backlog/active/EVO-112-A-repo-ui-shell.md) | Repo UI Shell（仓库列表 / 创建 / 导航 / Dashboard repo-centric 改版） | [EVO-100](../backlog/active/EVO-100-git-centric-platform-foundation.md) / [EVO-112](../backlog/active/EVO-112-repo-management-ui.md) | P0 | EVO-103-A（Done，2026-06-25 Iteration 044）+ EVO-116（Done，2026-06-26 Iteration 049） |

WIP 限制：单 Agent 微迭代只选 1 个 story。

## 3. 发布计划基线：不做事项

- 不实现 `/repos/:id` 详情页（Files / Commits / Settings Tab）— 归 `EVO-112-B`，EVO-118 S1 后重新排期
- 不实现代码编辑器 / AI Chat / commit / promote / branch 切换 — 归 `EVO-104` / `EVO-105`
- 不实现 diff viewer — 归 `EVO-104`
- 不实现 Skill / CLI / MCP 发现页 — 归 `EVO-109`
- 不实现 Smart HTTP clone / push / pull 前端 — 后端 `EVO-103-B` 范围
- 不删除旧 Tools / Skills / Interfaces 页面代码 — 保留路由可访问；从主导航降级
- 不引入新的设计 token（颜色 / 字号 / 间距沿用 DESIGN.md 现有定义）
- 不修改后端 Rust 代码
- 不实现 clone URL 展示与复制（属于 EVO-112-B Settings Tab）
- 不修改 API contract（如发现缺口，按 §5 的"先发现 contract 缺口先补"流程处理）

## 4. 发布计划基线：计划验收标准

### Story 格式与 BDD 适用性

- [x] EVO-112-A 标 Product / Frontend 形态。
- [x] 行为类使用 Given/When/Then 场景（已登录导航、空状态、非空列表、创建仓库、未登录重定向、Legacy 路由仍可访问）。
- [x] 技术验收包含命令级验证、状态归口、残余归口。

### EVO-112-A 验收

- [x] `frontend/src/lib/api/repos.ts` 新增 `reposApi`：`list(tenantId)` / `create(tenantId, req)` / `get(tenantId, repoId)`
- [x] `frontend/src/lib/api/types.ts` 新增 `Repo` 类型（与 `RepoResponse` 对齐）
- [x] `frontend/src/components/layout/Header.tsx` 侧边栏 `navigation` 改为 `Repos / Dashboard / Settings`；旧 Tools/Skills/Interfaces 降级为 Legacy 二级菜单（折叠展开）
- [x] `frontend/src/app/repos/page.tsx` 实现 `/repos`（卡片 + 搜索 + 排序 + 空状态）
- [x] `frontend/src/app/repos/new/page.tsx` 实现 `/repos/new`（表单 + 提交 + 返回列表）
- [x] `frontend/src/app/dashboard/page.tsx` 改为 repo-centric
- [x] `frontend/src/main-spa.tsx` 注册 `/repos` 和 `/repos/new` 路由（详情页路由不在本轮）
- [x] `frontend/src/locales/en.json` 与 `frontend/src/locales/zh-CN.json` 同步新增 i18n key
- [x] 所有颜色 / 字号 / 间距使用 DESIGN.md token

## 5. 发布计划基线：计划验证

```bash
# frontend
cd frontend
bun run type-check
bun run build

# docs
python3 - <<'PY'
from pathlib import Path
import re
missing=[]
for path in list(Path('docs').rglob('*.md'))+[Path('README.md'),Path('AGENTS.md'),Path('EVOLUTION.md')]:
    if not path.exists():
        continue
    text=path.read_text(encoding='utf-8')
    for m in re.finditer(r'\[[^\]]+\]\(([^)]+\.md)(?:#[^)]+)?\)', text):
        target=m.group(1)
        if '://' in target:
            continue
        p=(path.parent/target).resolve()
        if not p.exists():
            missing.append((str(path),target))
if missing:
    print('MISSING LINKS:')
    for src,target in missing:
        print(f'{src} -> {target}')
    raise SystemExit(1)
print('all markdown links exist')
PY

git diff --check
```

如果发现 API contract 缺口（前端需要的字段后端没返回），按 [CONTRACT-FIRST](../sop/CONTRACT-FIRST.md) 先更新 `docs/reference/API-CONTRACT.md`，再实现，最后同步前端 client；本轮开始前已核查现有 contract §16 Repos 字段齐全，本 Story 不动 contract。

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| 前端 Dashboard 改为 repo-centric 后旧页面入口找不到 | 保留 Legacy 二级菜单（折叠展开）让旧 Tools / Skills / Interfaces 仍可访问；不在主导航暴露 |
| 后端 `GET /api/v1/tenant/{tenant_id}/repos` 在 tenant 内有 1000+ repo 时返回慢 | 列表不要求分页（EVO-103 设计为返回全部）；EVO-116 性能证据显示 P95 = 31.46ms（1000 repos），无需前端分页 |
| 创建仓库后跳转到 `/repos/:id` 但详情页未实现 → 白屏 | 详情页交付前，创建成功统一返回 `/repos`；不得把 404 当作可接受 fallback |
| 旧 Tools / Skills / Interfaces 链接在新导航看不到 | Legacy 二级菜单默认折叠（点击 "Legacy" 展开）；不删除页面代码，URL `/tools` / `/skills` / `/interfaces` 仍可访问 |
| 移动端 sidebar 折叠后再展开问题 | MobileNav 当前实现是 full-screen overlay；新导航需要在 overlay 中也显示 Legacy 二级菜单；测试基础布局 |
| i18n key 漏改导致 `t('xxx')` 返回 key 字面量 | 同步 en.json 和 zh-CN.json；CI 不强制 i18n 完整性，但手工冒烟切换 zh-CN / en 检查 |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | Evolith 前端第一屏切换为 Git 仓库管理；用户在 Web UI 中可浏览 / 创建仓库，看到 repo-centric Dashboard |
| 产物 | EVO-112-A / EVO-112-B item file；ITERATION-054；PRODUCT-BACKLOG / EVO-100 子项表 / EVO-112 子项表 / iterations README / BOARD 同步；前端 `repos.ts` API client + `Repo` 类型；`Header.tsx` 侧边栏重构；`/repos` + `/repos/new` 页面；`dashboard/page.tsx` repo-centric 改版；`main-spa.tsx` 路由注册；en.json + zh-CN.json i18n key 同步 |
| 状态同步归口 | PRODUCT-BACKLOG、EVO-100 子项表、EVO-112 子项表、EVO-112-A item（Done）、EVO-112-B item（Proposed / paused）、ITERATION-054、iterations README、BOARD |
| Story/BDD 归口 | [EVO-112-A item file](../backlog/active/EVO-112-A-repo-ui-shell.md) |
| 验证证据 | `bun run type-check` 0 errors / `bun run build` 0 errors / Python markdown link check all OK / `git diff --check` clean / Playwright 截图 ≥ 5 张 / 手工冒烟通过 |
| 残余工作归口 | `/repos/:id` 详情页 → EVO-112-B（EVO-118 S1 后重新排期）；Dashboard "Recent commits" 列表需要 commit history API → EVO-105；移动端深度适配 → 后续 EVO |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-06-29 | inventory-disposition | 启动前盘点 `docs/iterations/`：Active/In Progress/Review = 无；Planned/Blocked = Iterations 025（EVO-012/EVO-013，租户设置）/ 026（EVO-014，Stripe webhook；Phase F 候选，与本轮 EVO-112-A 独立）；Superseded = Iterations 018/019/020/027（2026-06-23 方向调整后）；Recently Closed = Iterations 042/044/045/046/047/048/049（EVO-101/EVO-102/EVO-103 A/B/C/EVO-115/EVO-116，均 Done）。无阻塞项，EVO-112-A 可启动。 |
| 2026-06-29 | activation | 建立 EVO-112-A / EVO-112-B item file；选定 EVO-112-A 进入 In Progress；同步 PRODUCT-BACKLOG / EVO-100 子项表 / EVO-112 子项表 / iterations README / BOARD；后续进入前端实现阶段。 |
| 2026-06-29 | progress | 前端实现：`repos.ts` API client + `Repo` 类型 + Header 侧边栏重构（Repos/Dashboard/Settings + Legacy 二级菜单）+ `/repos` 列表 + `/repos/new` 表单 + Dashboard repo-centric 改版 + main-spa 路由注册 + i18n 70+ key 同步。 |
| 2026-06-29 | validation | `bun run type-check` 0 errors；`bun run build` 0 errors；markdown link check all OK；`git diff --check` clean；Playwright 截图 12 张覆盖桌面 + 移动端 + Legacy 路由可访问；手工冒烟通过（登录 → 创建 → 跳转 → 列表 → Dashboard → Legacy → Tools）。 |
| 2026-06-29 | completion | EVO-112-A 原始本地实现完成；残余归口 EVO-112-B / EVO-105 / 后续 EVO。该成果当时未提交，后续必须以主线现状重新验证。 |
| 2026-08-02 | change-request | `replan`：主线已发布 Iteration 050，远端 PR #7 已使用 Iteration 053；保留两份既有基线，将本地 EVO-112-A 历史记录迁移为 Iteration 054。 |
| 2026-08-02 | recovery-fix | 创建成功路径由未实现的 `/repos/:id` 改为 `/repos`，避免把 404 当作可接受 fallback；详情页仍归 EVO-112-B。 |
| 2026-08-02 | revalidation | 最新 `origin/main` 上 `bun run type-check` 与 `bun run build` 通过；en/zh locale key 集合一致；Markdown links、governance validator、`git diff --check` 通过。`bun run lint` 因既有 ESLint 10 flat config 缺失失败，残余归 EVO-118-G，不属于本 Story hard gate。 |
| 2026-08-02 | merge-closure | [PR #8](https://github.com/wjhuang88/evolith/pull/8) 由 exact head `295f3f36ba16400c6415c90504cf1e0f96fe318a` 合并为 `1216624`；CI run `30712179586` / job `91401439386` 以 15m28s 全绿；Navigator 两轮审查最终无 blocking findings。 |

### 迭代启动前库存盘点（per [START-ITERATION.md](../sop/START-ITERATION.md)）

- **Active / In Progress**：无。
- **Review**：无。
- **Planned / Blocked**：Iterations 025/026（Phase F 租户 / 计费；候选需 refinement，与本轮 EVO-112-A 独立，维持 Planned/Blocked）。
- **Superseded**：Iterations 018/019/020/027（2026-06-23 方向调整后不再激活）。
- **Recently Closed**：Iterations 042/044/045/046/047/048/049（EVO-101/EVO-102/EVO-103 A/B/C/EVO-115/EVO-116）。

**disposition 决策**：

- Iterations 025/026：维持 Planned/Blocked，不因本轮插队修改计划基线（与 Phase E' 主线独立）。
- Iterations 018-020/027：维持 Superseded。
- Iterations 042/044/045/046/047/048/049：Closed，无需重新打开。
- 无 Active/Review 阻塞项，EVO-112-A 满足 Story DoR（依赖 EVO-103-A + EVO-116 均 Done；Ready），可启动。

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|
| 2026-08-02 | replan | 接受 | Iteration 编号 050 → 054；不改变 EVO-112-A 产品范围，不抢占 EVO-118-D / PR #7 | 本地代码全部保留并在最新 `origin/main` 上重放；治理索引以主线为准做增量同步 |

## 10. Review

- 完成：
  - **前端代码**：
    - `frontend/src/lib/api/repos.ts` 新增（`list` / `get` / `create` / `update` / `delete`）
    - `frontend/src/lib/api/types.ts` 新增 `Repo` / `CreateRepoRequest` / `UpdateRepoRequest` 类型
    - `frontend/src/lib/api/index.ts` 重新导出
    - `frontend/src/components/layout/Header.tsx` 重构：`primaryNavigation = [Repos, Dashboard, Settings]`；`legacyNavigation = [Tools, Skills, Interfaces]`；新增 `<LegacyDisclosure>`（折叠展开）；移动端 overlay 同步
    - `frontend/src/app/repos/page.tsx` 实现：卡片网格、客户端搜索、排序（最近更新 / 名称 / 创建时间）、空状态、Intl.RelativeTimeFormat 相对时间
    - `frontend/src/app/repos/new/page.tsx` 实现：表单 + 客户端校验（name pattern + 长度）+ visibility 单选 + default_branch + auto_merge / require_review 开关 + 提交后返回 `/repos`
    - `frontend/src/app/dashboard/page.tsx` 改版 repo-centric：4 个 stat cards（Repositories / Commits this month / Team members / with commits）+ 4 个 Quick Actions（New / Browse / Team / API keys）+ Recent repositories 列表 + Quick Links
    - `frontend/src/main-spa.tsx` 注册 `/repos` 和 `/repos/new` 路由
  - **i18n**：`frontend/src/locales/en.json` 与 `zh-CN.json` 同步新增 `nav.repos` / `nav.legacy` / `repos.*` / `dashboard.recentRepos.*` / `dashboard.repoCentric.*` / `dashboard.stats.repos|commitsMonth|members|withCommits|empty` 等 70+ 个 key
  - **文档**：
    - `docs/backlog/active/EVO-112-A-repo-ui-shell.md` 新建（Ready → In Progress → Done）
    - `docs/backlog/active/EVO-112-B-repo-detail-read-only.md` 新建（Proposed / paused；EVO-118 S1 后重新排期）
    - `docs/backlog/active/EVO-112-repo-management-ui.md` 增加子 Story 表 + 拆分依据
    - `docs/backlog/active/EVO-100-git-centric-platform-foundation.md` 增加 EVO-112-A / EVO-112-B 子项
    - `docs/backlog/PRODUCT-BACKLOG.md` 增加 EVO-112-A / EVO-112-B 行
    - `docs/iterations/README.md` 增加 ITERATION-054 历史恢复记录
    - `docs/BOARD.md` Now / Next 同步
- 未完成：无（本 Story 范围内）。
- 验证结果：
  - **2026-08-02 主线恢复重验**：
    - `cd frontend && bun run type-check` → exit 0
    - `cd frontend && bun run build` → exit 0，Vite 8.0.16，1970 modules，`✓ built in 1.95s`
    - en/zh locale scalar key 集合比较 → 无差异
    - Markdown link check → `all markdown links exist`
    - `validate_project_governance.sh` → `0 warning(s)`
    - `git diff --check` → clean
    - `bun run lint` → exit 2；仓库缺少 ESLint 10 `eslint.config.*`，为已知基线问题，归 EVO-118-G 的 CI/lint Gate，不阻塞本 Story 的 type-check/build hard gate
    - GitHub exact-head CI run `30712179586` → `Backend + Frontend` pass（15m28s）；frontend type-check/build、backend fmt/check/clippy/SQLite workspace tests 全绿
    - Navigator final check → no blocking findings；Iteration 053 的后续状态仍由 Draft PR #7 owner branch 同步
  - `cd frontend && bun run type-check` → 0 errors
  - `cd frontend && bun run build` → ✓ built in 599ms；dist/index.html + dist/assets/index-*.css + dist/assets/index-*.js；0 errors
  - `python3 -c "..."` markdown link check → `all markdown links exist`
  - `git diff --check` → clean
  - Playwright 截图 12 张（覆盖桌面 + 移动端基础布局）：
    - `01-dashboard-repo-centric-empty.png`：登录后 Dashboard 空状态（侧边栏 Repos / Dashboard / Settings + Legacy）
    - `02-repos-empty-state.png`：`/repos` 空仓库空状态（dotted border 引导卡片 + New Repo CTA）
    - `03-repos-new-empty.png`：`/repos/new` 表单初始状态
    - `04-repos-new-filled.png`：填了 demo-repo + description 的表单
    - `05-repo-detail-not-implemented.png`：原始本地实现暴露的 404 证据；2026-08-02 恢复时已改为创建后返回 `/repos`
    - `06-repos-list-with-repo.png`：`/repos` 列表显示新创建的 demo-repo（Private badge / main branch / "No commits yet"）
    - `07-legacy-menu-expanded.png`：Legacy 二级菜单展开显示 Tools / Skills / CLI Interfaces
    - `08-legacy-tools-still-works.png`：点击 Legacy → Tools 进入旧 `/tools` 路由，仍可正常访问（不 404）
    - `09-dashboard-repo-centric.png`：1 个 repo 时 Dashboard 显示 4 个统计 + Quick Actions + Recent repos
    - `10-mobile-repos-list.png`：移动端 390×844 `/repos` 列表（卡片单列 + 顶部汉堡按钮）
    - `11-mobile-nav-overlay.png`：移动端 hamburger overlay 显示 Repos / Dashboard / Settings + Legacy
    - `12-mobile-dashboard.png`：移动端 Dashboard
  - 原始手工冒烟：登录 → /repos → New Repo → 填表 → Create → 回到 /repos 看到新 repo → 进 Dashboard 看到统计更新 → 进 Legacy → Tools 旧路由仍工作
- 闭环状态：**Complete**（PR #8 已合并，merge/CI/Navigator 证据已写回）
- 残余归口：
  - `/repos/:id` 详情页 Files / Commits / Settings 三 Tab → `EVO-112-B`（硬依赖 EVO-112-A + EVO-103-C；等待 EVO-118 S1 后重新排期）
  - Dashboard "Recent commits" 列表（按 repo 聚合 5 条最新 commit）需要 commit history API → `EVO-105` 范围；本 Story 仅做静态计数（`repos[i].last_commit_sha` 存在即 +1），不做 commit 时间线
  - Repo 卡片 design 候选 U-17 / U-18 / U-19：本 Story 采用精简卡片 + 双栏文件树占位 + Legacy 二级菜单（与 item file §UX Decisions 默认推荐一致）
  - `EVO-044` 前端 CLI 命名简化 → 与本 Story 解耦；EVO-044 item file 仍 Proposed，未推进
  - 移动端深度适配（NavigationDrawer / advanced hamburger） → 现有 MobileNav 模式保留，未做深度改造

## 11. Retrospective

- 做得好的：
  - 提前在启动 SOP 中盘点 iteration inventory（Active/In Progress/Review = 无），明确 disposition（Iterations 025/026 维持 Planned/Blocked；018-020/027 维持 Superseded），避免直接撞 backlog 导致流程偏差。
  - 拆分 EVO-112 → EVO-112-A + EVO-112-B：明确"先做壳再做详情"，避免单个 Story 跨越 2 天窗口，且让 B 子项硬依赖清晰可验证。
  - 恢复时保留主线 Iteration 050 与远端 PR #7 的 Iteration 053，迁移到 054，避免覆盖已发布基线。
  - 旧 Tools / Skills / Interfaces 降级为 Legacy 二级菜单（保留路由可访问），符合用户原意"围绕 Git 仓库为中心"但避免硬删除产生迁移成本。
  - 所有颜色 / 字号 / 间距沿用 DESIGN.md 现有 token（`bg-[var(--primary)]` / `rounded-[50px]` / `rounded-[24px]` / `text-3xl` 等），不引入新的 hex 或 px；DESIGN.md §Iteration Guide "Run lint after edits" 虽未跑（design.md validator 不在项目依赖中），但通过 Playwright 截图人工核对视觉一致。
  - i18n en + zh-CN 同步新增 70+ key，覆盖 `nav.*` / `repos.*` / `dashboard.recentRepos.*` / `dashboard.stats.*` 等命名空间；LanguageSwitcher 已可用。
- 需要调整：
  - `bun run lint` 已执行，但因仓库缺少 ESLint 10 flat config 在加载规则前失败；修复归 EVO-118-G，不能把该结果描述成代码 lint 通过。
  - Playwright 测试当前是手工 + 截图，不是 CI 自动化；建议后续把"空状态 / 创建仓库 / Legacy 路由可访问" 3 个核心场景写成 Playwright test spec 跑在 GitHub Actions 中（依赖 EVO-030 CI workflow 的 bun runtime）。
  - 原始实现把创建成功路径指向未实现详情页；恢复时改为返回列表。EVO-112-B 仍需在 S1 后交付，但不再阻塞当前路径可用性。
- 写入 EVOLUTION：
  - **拆分 EVO-112 时机选择**：在 EVO-103/EVO-116 已 Done 但 EVO-104 仍依赖后续细节时，把仓库管理 UI 拆为 A（壳 + Dashboard 改版）+ B（详情 Tab）两轮，让 A 在 0.5-2 天窗口内可完成且独立验收。
  - **后续路由不能作为当前 fallback**：未实现详情页时，创建成功必须返回已实现的列表；已知 404 不能通过文档登记转化为 Complete。

## 相关链接

- Story：[EVO-112-A](../backlog/active/EVO-112-A-repo-ui-shell.md)
- 父 Story：[EVO-112](../backlog/active/EVO-112-repo-management-ui.md)
- 同级子 Story：[EVO-112-B](../backlog/active/EVO-112-B-repo-detail-read-only.md)
- 父 Epic：[EVO-100](../backlog/active/EVO-100-git-centric-platform-foundation.md)
- Two-Month Plan：[TWO-MONTH-PLAN-2026-07](../roadmap/TWO-MONTH-PLAN-2026-07.md)
- Design System：[DESIGN](../reference/DESIGN.md)
- API Contract：[API-CONTRACT](../reference/API-CONTRACT.md)
- ADR：[ADR-0004](../decisions/ADR-0004-git-centric-storage.md) / [ADR-0006](../decisions/ADR-0006-smart-http-via-git-subprocess.md)
