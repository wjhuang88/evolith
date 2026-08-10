# Evolith Product Interaction Architecture

> Status: Accepted baseline
> Date: 2026-08-08
> Governing decision: [ADR-0008](../decisions/ADR-0008-repo-centric-interaction-architecture.md)
> Visual system: [DESIGN.md](../reference/DESIGN.md)

## Product Value From First Principles

Evolith 的核心不是维护 Tool、Skill 或 Interface 记录，而是让开发者围绕 Git Repository 完成理解、修改、审查和交付。Repo 保存代码与历史；Agent 在受控分支和 Policy 下协作；Commit、Ref、Policy 结果与审计事件共同构成可信结果。

```text
Developer intent
  -> create or import Repo
  -> understand code and policy
  -> browse or start Agent Workspace
  -> produce change
  -> evaluate Policy
  -> Commit
  -> Review/Promote or Auto-merge
  -> Git history records evidence
  -> Repo content becomes discoverable capability
```

## Product Principles

1. **Repo first**：所有核心任务都以 Repo 为上下文，页面不再围绕旧 Registry 数据模型组织。
2. **Agent is observable**：Agent 是可观察、可中断、受 Policy 约束的协作者；计划、事件、产物与结果均可追踪。
3. **Commit is the trust boundary**：成功必须能定位到 Ref、Commit、Policy 结果和审计事件，不能只显示乐观 Toast。
4. **Capabilities are derived**：Skill、CLI、MCP Tool 来自 Repo 内容及其索引，结果必须显示 Repo、Ref、Path、Commit 和更新时间。
5. **Pages serve task state**：页面围绕用户任务和当前状态编排，不为旧菜单或 API 兼容性牺牲主流程。
6. **Every state is designed**：加载、空状态、错误、无权限、成功、重试和 deep-link 刷新均属于验收范围。

## Actors And Primary Jobs

| Actor | Primary job | Product surface |
| --- | --- | --- |
| Anonymous visitor | 判断产品是否解决 Git + Agent 协作问题并注册 | Public entry / auth |
| Developer / Member | 找到 Repo、理解代码、启动/恢复 Session、查看 Commit | Dashboard / Repo / Workspace / Activity |
| Reviewer | 判断变更、Policy 与证据是否足以 Promote | Dashboard review queue / Workspace review / Commit evidence |
| Admin / Owner | 管理成员、凭证、Workspace 默认策略与计费 | Settings；不占开发首屏 |
| Agent / API client | 在 Repo/Branch/Path capability 内产生可审计结果 | Session/API；通过 Activity/Commit 被人观察 |

隐藏入口不是授权。每个 deep link 和 API 都必须按 tenant、Repo、Ref/Path、角色或 Typed Capability 独立校验；无权访问与不存在遵循安全契约，不通过 Dashboard fallback 掩盖。

## Surface Profiles

- **Authenticated product**：安静、紧凑、任务优先。使用列表、表格、分组行和固定工具区；不使用营销 hero、装饰性大卡片、彩色 block 或超大标题。
- **Public entry**：首屏明确展示 Evolith、真实 Repo/Workspace 产品画面和一个主要注册动作；价值说明围绕 Git hosting、controlled Agent work 和 provenance，不使用旧 Registry feature cards。
- **Workspace**：全宽任务面，session stream 为核心，不把 3D/装饰媒体或多层 card 容器引入操作区。
- **Visual tokens**：颜色、间距与组件状态来自 DESIGN.md 和全局 semantic tokens；应用正文 letter spacing 为 0，字号不随 viewport 连续缩放，dark mode 使用同一语义层级。

## First-run Journey

```text
Sign up / Sign in
  -> detect whether the tenant has a Repo
  -> create Repo or import a supported remote Repo
  -> Repo Overview
  -> Browse code or Start Workspace
```

- 没有 Repo 的用户进入 Repo onboarding，而不是创建 Tool。
- 若 import 后端能力尚未实现，不展示假可用入口；MVP 只提供真实可完成的创建路径。
- 创建成功必须进入 `/repos/:id/overview`，并显示真实 Repo 状态。
- 已有 Repo 的用户跳过 onboarding，进入任务型 Dashboard。
- 未登录用户访问受保护 deep link 时进入 `/login?redirect=<original-path>`；登录成功后优先恢复原 deep link，不被 onboarding 判定覆盖。
- 普通登录没有 deep link 时，入口解析器按 `无 Repo -> onboarding`、`有 Repo -> dashboard` 分流。`/onboarding` 本身必须受认证保护。

## Daily Development Loop

```text
Dashboard
  -> Repositories
  -> Repo Overview
  -> Files or Workspace
  -> Commit result
  -> Review/Promote or Auto-merge
  -> Activity and Git history
```

Dashboard 优先展示 Active work、Review queue、Recent repos 和 Repo health。成员、计费和 API Key 是管理任务，应进入 Settings 或 Workspace 管理区，不占据日常开发主视图。

## Capability Discovery Loop

```text
Discover
  -> search Repo-derived Skill / CLI / MCP capability
  -> inspect provenance
  -> open source blob or owning Repo
  -> use capability in an authorized Workspace
```

Discovery 是 Git 内容的分发和发现层，不是与 Repo 并列的内容管理系统。任何结果都必须保留来源和版本证据。

## Information Architecture

### Primary Navigation

| Entry | Responsibility |
| --- | --- |
| Dashboard | 当前工作、待审查事项、最近 Repo 与健康状态 |
| Repositories | Repo 列表、搜索、创建和导入 |
| Discover | 跨 Repo 发现 Skill、CLI 和 MCP 能力 |
| Activity | Agent Session、Commit、Policy 与审计事件时间线 |
| User menu -> Settings | Tenant、成员、凭证、计费和个人偏好；不占用开发主导航 |

Workspace 只属于具体 Repo，不设置全局 Workspace 入口。项目尚未上线，没有外部 UI 兼容义务；旧 Tools / Skills / Interfaces 页面与路由在 Repo Resources / Discover 可用后直接删除，不建设 Legacy 菜单、退场页或长期重定向层。后端也不建设旧表双写，由 ADR-0009 / EVO-122 在 Repo-derived 合约承接后直接清理。

### Repo Navigation

| Route | Responsibility |
| --- | --- |
| `/repos/:id/overview` | README/空仓状态、default branch、Policy、latest commit、核心操作 |
| `/repos/:id/files` | 文件树、blob 查看、面包屑与 Ref 上下文 |
| `/repos/:id/commits` | Commit 历史与可追溯证据 |
| `/repos/:id/commits/:sha` | Commit message、author/time、parent、changed files、diff 与 Ref 证据 |
| `/repos/:id/resources` | 当前 Repo 派生的 Skill、CLI、MCP 能力 |
| `/repos/:id/workspace/new` | 创建受控 Agent Session |
| `/repos/:id/workspace/:session_id` | Conversation-first Agent Session 与 contextual artifacts |
| `/repos/:id/workspace/:session_id/review` | require_review 的 diff、Policy 解释与 Promote 操作 |
| `/repos/:id/settings` | Repo 元数据、Policy、clone 信息和危险操作 |

`/repos/:id` 应规范化到 Overview。Overview 的首要操作是 **Browse code** 和 **Start Workspace**，而不是直接进入设置或永久三栏 IDE。

### Settings Navigation

| Route | Responsibility |
| --- | --- |
| `/settings/profile` | 个人资料、语言与主题偏好 |
| `/settings/workspace` | Tenant 基本信息与默认策略 |
| `/settings/members` | 成员、角色与邀请 |
| `/settings/api-keys` | API Key、Scoped credential 与轮换状态 |
| `/settings/billing` | 套餐、用量和账单 |

Settings 使用自己的二级导航，不与 Repo 页面混排。由于产品未上线，实施时直接采用目标路径并同步内部链接；不为 `/tenant/*` 和 `/profile` 新建兼容体验。

## Route Ownership Matrix

| Surface | Target route | Entry / exit rule | Delivery owner |
| --- | --- | --- | --- |
| Public product entry | `/` | 未登录展示 Git-centric 产品；已登录进入 entry resolver | EVO-121-C |
| Authentication | `/login`, `/register`, verification/recovery/join | 成功后恢复 deep link，否则进入 entry resolver | EVO-121-C |
| First run | `/onboarding` | 仅已认证且 tenant 无 Repo；创建后进入 Repo Overview | EVO-120 |
| Daily home | `/dashboard` | 有 Repo 后默认入口；链接到真实工作与 Repo 状态 | EVO-121-B |
| Repo collection | `/repos`, `/repos/new` | 创建后进入新 Repo Overview | EVO-112-A / EVO-120 |
| Repo read model | `/repos/:id/overview`, `/files`, `/commits`, `/settings` | Repo/tenant/RBAC 校验；404 与 forbidden 不混淆 | EVO-112-B |
| Commit evidence | `/repos/:id/commits/:sha` | 从 Workspace/Activity/Commit list 回到不可歧义的 Git 结果 | EVO-112-C |
| Repo capabilities | `/repos/:id/resources` | 只展示带 Repo/Ref/Path/Commit provenance 的索引结果 | EVO-109 |
| Agent workspace | `/repos/:id/workspace/new`, `/workspace/:session_id`, `/review` | 需要 Agent Session 与受控写入能力；无假入口 | EVO-104 |
| Discovery | `/discover` | 跨 Repo 搜索，结果回到 source blob 或 owning Repo | EVO-109 |
| Activity | `/activity` | Durable event read model；事件可回到 Session/Commit/Repo | EVO-121-E |
| Settings | `/settings/*` | 用户菜单进入；按角色隐藏或禁止管理功能 | EVO-121-D |
| Legacy UI | no target route | 删除旧 Tool/Skill/Interface 页面、主导航和用户文案 | EVO-121-F |

## Page Orchestration Rules

### Entry Resolver

```text
requested protected deep link
  -> no valid session: login(redirect=original)
  -> valid session + authorized target: original deep link
  -> valid session + forbidden target: forbidden state, never dashboard fallback

login without deep link
  -> tenant has no Repo: onboarding
  -> tenant has Repo: dashboard
```

页面不得各自复制这套判断。实现应由单一 route guard / entry resolver 提供，避免 login、join、verification 和 root route 产生不同跳转语义。

### Repo Task Transition

```text
Repo Overview
  -> Browse code -> Files(ref=default_branch)
  -> Inspect history -> Commits(ref=default_branch)
  -> Inspect capability -> Resources(source provenance)
  -> Start work -> Workspace/new -> Workspace/session on agent branch
  -> Configure -> Settings(role-gated)
```

未实现的目标路由不得显示可点击操作或假成功占位。分阶段交付时，Overview 始终提供真实可用的 Browse code；Start Workspace 只在 EVO-104 路由与后端能力可用后出现。

### Commit Result Transition

```text
Workspace result
  -> block: remain in session with policy reason and repair action
  -> require_review: open Workspace/session/review with diff + policy evidence
  -> auto_merge: open /repos/:id/commits/:sha on target Ref
  -> every terminal state: link to Activity evidence
```

### Responsive Shell

- Desktop：固定 top bar + 240-256px side navigation + 单一 page content 区；Repo 内二级导航属于 page header。
- Mobile：top bar + modal navigation drawer；Repo 二级导航使用可横向滚动 tabs 或明确菜单，不复制桌面 sidebar。
- Workspace 在所有视口保持 session stream 为主；artifact 在桌面侧层或主区切换，在移动端使用全屏 sheet。
- 页面内容宽度、table/list overflow、empty/error blocks 必须有稳定约束，动态内容不得推动全局导航位移。

## Page Responsibilities

| Page | Primary question | Required outcome |
| --- | --- | --- |
| Dashboard | 我现在应该继续什么？ | 恢复工作、处理 review、进入最近 Repo |
| Repositories | 我要在哪个 Repo 工作？ | 找到、创建或导入 Repo |
| Repo Overview | 这个 Repo 当前是什么状态？ | 理解 README、branch、Policy、latest commit 并选择下一步 |
| Files | 代码和内容是什么？ | 在明确 Ref 下浏览文件与 blob |
| Workspace | 我要如何与 Agent 完成变更？ | 观察会话、检查 artifacts、形成并链接到可信 Commit 结果 |
| Activity | 发生了什么？ | 将 Session、Policy、Commit 和审计事件关联起来 |
| Discover | 哪些能力可复用？ | 找到带 provenance 的 Repo-derived capability |

## Page Composition Blueprints

| Page | Region order | Primary command |
| --- | --- | --- |
| Dashboard | Page title -> Active work -> Review queue -> Recent repos -> Repo health | Resume work / Review |
| Repositories | Title + New Repo -> search/filter/sort -> compact list -> pagination/empty | New Repo |
| Repo Overview | Repo identity + Ref/Policy -> README/empty state -> latest Commit -> capability summary | Browse code; Start Workspace when available |
| Files | Repo subnav -> Ref/path toolbar -> tree/drawer + blob -> file metadata | Select file / copy path |
| Commits | Repo subnav -> Ref/filter -> chronological list | Open Commit evidence |
| Commit Evidence | SHA/Ref header -> message/author/parents -> changed files -> diff | Return to Repo/Session |
| Workspace | Session/branch/policy header -> stream -> prompt/action bar -> contextual artifact layer | Send / Stop / Review result |
| Review | Policy result -> diff/files -> checks/evidence -> decision bar | Promote / Return to session |
| Resources | Repo provenance header -> Skill/CLI/MCP grouped lists | Open source blob |
| Discover | Search/filter -> dense cross-Repo results -> provenance detail | Open source / Repo |
| Activity | Filter bar -> chronological event groups -> entity links | Open Session/Commit/Repo |
| Settings | Settings subnav -> one management form/list -> explicit save/error state | Save / Rotate / Invite by page |

Region order is a behavioral constraint, not a pixel-perfect wireframe. Page Stories may refine component placement but must preserve information priority and avoid wrapping each region in nested cards.

## State Models

### Repo

| State | Required UI behavior |
| --- | --- |
| Creating | 创建操作有单一 pending state；禁止重复提交 |
| Empty | Overview 解释没有 Commit，并给出真实 clone/push 或初始化动作 |
| Active | 显示 default Ref、latest Commit、Policy 和真实可用动作 |
| Error | 显示可恢复原因与 retry/reconcile 入口，不假装 empty |
| Deleting | 禁止新写入并显示进行中状态，完成后离开 Repo route |

### Agent Session

| State | Required UI behavior |
| --- | --- |
| Starting | 建立 session/branch，尚未允许输入时显示明确 pending |
| Running | stream 可恢复；Stop 是明确 command，不关闭页面伪装完成 |
| Waiting review | 固定显示 Commit、Policy reason 和 Review 入口 |
| Blocked | 保留 branch/artifacts，显示 policy reason 与修复动作 |
| Failed | 显示最后成功 event、可重试边界和 branch 状态 |
| Completed | 链接到 Commit evidence 和 Activity；不只显示 toast |
| Cancelled | 明确由谁/何时取消，保留已有 artifacts 和审计 |

### Data Fetch

每个独立 section 区分 `loading / empty / error / forbidden / success / stale-refreshing`。并行请求允许部分成功，但失败 section 不得用 `[]`、`0` 或空卡片替代错误；retry 只重试失败边界。

## Workspace Interaction Model

- Session stream 是桌面和移动端默认主区域。
- 文件树通过 rail/drawer 按需打开；文件、diff、commit 作为 contextual artifact 打开。
- Agent 分支使用 `agent/{session_id}/{feature_slug}`，并持续显示当前 Ref 与 Policy 状态。
- `auto_merge`、`require_review`、`block` 必须有持久状态标识和 inline explanation。
- 每个终态都必须给出下一步：查看 Commit、进入 Review/Promote、修复阻塞或重试。

## Current-to-target Gaps

| Current state | Target state | Owner |
| --- | --- | --- |
| Onboarding 创建旧 Tool | 首次创建/导入 Repo 并进入 Overview | EVO-120 |
| Dashboard 混合成员、计费、API Key 与资源统计 | Active work / Review queue / Recent repos / Repo health | EVO-121-B |
| Repo 列表链接到未注册详情路由 | Overview-first Repo Detail | EVO-112-B |
| Commit 只有列表、无稳定结果详情 | Commit evidence deep link | EVO-112-C |
| Landing 与 auth 成功路径仍围绕旧产品或固定跳 Dashboard | Git-centric public entry + single entry resolver | EVO-121-C |
| Tenant/Profile 管理页分散且占主导航 | `/settings/*` 二级信息架构 | EVO-121-D |
| Vibe Coding 尚无页面 | Repo 内 conversation-first Workspace | EVO-104 |
| Skill/CLI/MCP 是独立 legacy 页面 | 带 provenance 的 Discover 与 Repo Resources | EVO-109 |
| Activity 没有可靠事件 read model 或页面 | Outbox + Worker 驱动的可追溯时间线 | EVO-118-H / EVO-121-E |
| Header 仍提供 Legacy disclosure 且旧 CRUD 页面可达 | 目标导航不含 legacy 内容，旧页面代码删除 | EVO-121-F |
| EVO-110 计划旧表双写/回填/API 兼容 | Repo-derived read/execute 承接后直接删除 legacy runtime/table | ADR-0009 / EVO-122 |

## Delivery Order

```text
EVO-118-F / G / H close lifecycle, runtime and event gates
  -> EVO-120 / EVO-112-B / EVO-112-C Repo onboarding, detail and commit evidence
  -> EVO-121-C / EVO-121-D public/auth entry and Settings IA
  -> EVO-105 / EVO-106 / EVO-107 / EVO-104 controlled Agent write loop and Workspace
  -> EVO-121-E / EVO-121-B durable Activity and task-first Dashboard
  -> EVO-108 / EVO-109 Repo-derived Discovery and Resources
  -> EVO-121-A / EVO-121-F final App Shell and legacy UI removal
  -> EVO-111 / EVO-122 remove Sandbox and legacy Registry backend
  -> EVO-118-E final production build, deployment and release smoke
```

该顺序只表达产品依赖，不自动激活 Iteration。每个 Story 仍须通过 START-ITERATION。

## Page Delivery Gates

每个新页面或页面重构必须验证：

- 正常、loading、empty、error、forbidden、retry 和 success 状态；
- 直接打开 deep link 与浏览器刷新；
- 桌面和移动视口无重叠、截断或布局跳动；
- 键盘焦点、可访问名称和主要操作顺序；
- API 失败时不返回假成功，写操作可恢复或给出明确残余；
- 所有 Git 结果显示 Ref / Commit 等真实证据；
- `bun run type-check`、`bun run build` 以及 Playwright 关键流程截图。

## Development-ready Completion Criteria

整体交互方案只有在以下条件全部满足时才算实现完成：

- Route Ownership Matrix 每个目标 surface 的 owner Story 为 Done，或明确 Deferred/Dropped 且不破坏主闭环；
- login、join、verification、root 与 protected deep link 共用同一 entry resolver 行为；
- 从 first run、daily development、commit result 到 capability discovery 的每条主链都通过真实 API/事件数据验证；
- 不存在指向未注册路由的按钮、用空数组吞掉错误的假 empty state，或没有 Ref/Commit 证据的成功反馈；
- 用户可见导航、页面和 i18n 不再把 Tool/Skill/Interface CRUD 描述为产品主线；
- 桌面与移动 Playwright 截图覆盖主流程以及 loading/empty/error/forbidden/retry 状态。

## Explicitly Out Of MVP

- 实时多人协作；
- 浏览器终端；
- Live Preview sandbox；
- 完整 IDE/LSP 体验；
- Capability 评分、评论或独立 marketplace；
- 为旧 Registry 页面重做主流程、保持 UI 信息架构兼容或建设迁移向导。
