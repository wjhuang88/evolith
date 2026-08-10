# ADR-0008: Repo-centric Interaction Architecture

## 状态

Accepted（2026-08-08）

## 背景

Evolith 已从 DB-centric Registry 转向 Git-centric AI development platform，但现有页面仍混合两套心智模型：onboarding 创建 Tool，landing 强调 Tools/Skills/Interfaces，Dashboard 混合开发任务与管理数据，Repo 列表尚无完整详情主线。如果继续按旧页面逐个补功能，会让兼容结构反向约束最终产品。

产品的可信价值来自 Repo、受控 Agent 协作、Commit/Promote 和可追溯事件。因此需要先确定整体交互架构，再让页面 Story 按同一主链实施。

## 选项

1. 保留旧 Registry 信息架构，在各页面增加 Repo 入口。
2. Repo 与 Registry 双主线并行，用户自行选择入口。
3. 以 Repo 为唯一产品主线，Skill/CLI/MCP 作为 Repo-derived Discovery，旧页面仅过渡保留。

## 决策

选择选项 3：

- Repo 是所有核心开发流程的一级实体和上下文。
- 首次使用创建或导入 Repo，成功后进入 Repo Overview；不再以创建 Tool 作为 onboarding。
- Dashboard 以 Active work、Review queue、Recent repos 和 Repo health 为中心。
- Repo 默认详情是 Overview，内部组织为 Overview、Files、Commits、Resources、Workspace、Settings；Workspace 只存在于具体 Repo 上下文中。
- Workspace 采用 conversation-first 模型；Agent 计划、事件、artifact、Policy 和 Commit 结果必须可观察。
- Skill、CLI 和 MCP Tool 是由 Repo 内容派生并带 provenance 的能力，不作为与 Repo 并列的内容系统。
- 项目尚未上线，不承担旧 UI 路由兼容义务。Repo Resources / Discover 形成真实替代入口后，直接删除旧 Tools/Skills/Interfaces 页面、路由、导航和用户文案；不建设 Legacy 菜单、迁移向导或长期重定向层。后端同样不建设双写兼容层，按 ADR-0009 / EVO-122 在 Repo-derived 合约承接后删除。
- 登录、邀请、验证和受保护 deep link 使用同一 entry resolver：优先恢复合法 deep link；普通登录按是否存在 Repo 分流到 onboarding 或 Dashboard。
- Settings 采用 `/settings/*` 二级信息架构并从用户菜单进入，不占开发主导航。
- 页面实现以 [Product Interaction Architecture](../design/PRODUCT-INTERACTION-ARCHITECTURE.md) 为产品流程基线，以 [DESIGN.md](../reference/DESIGN.md) 为视觉 token 和组件基线。

## 后果

### 正向

- 首次使用到日常开发形成单一、可解释的业务闭环。
- Repo Detail、Vibe Coding、Discovery 和 Activity 有明确边界。
- Agent 结果与 Git 证据绑定，避免 UI 显示无法验证的成功。
- 不再为历史页面兼容投入主线设计成本。

### 代价与约束

- 现有 onboarding、landing、Dashboard 和 Header 需要分批重构。
- 当前内部链接需要一次性切换到目标路由；因为没有线上调用方，不为旧 UI 路径维护兼容层。
- Workspace 完整闭环仍依赖 EVO-105/106/107 以及可靠事件边界。
- 所有新页面 Story 必须覆盖非正常状态、deep-link 和响应式验证。

## 相关链接

- [Product Interaction Architecture](../design/PRODUCT-INTERACTION-ARCHITECTURE.md)
- [Git-Centric Platform Proposal](../proposals/GIT-CENTRIC-PLATFORM.md)
- [EVO-120 First-run Repo Onboarding](../backlog/active/EVO-120-first-run-repo-onboarding.md)
- [EVO-112-B Repo Detail Read-only](../backlog/active/EVO-112-B-repo-detail-read-only.md)
- [EVO-104 Vibe Coding Web UI](../backlog/active/EVO-104-vibe-coding-web-ui.md)
- [EVO-121 Product Experience Convergence](../backlog/active/EVO-121-product-experience-convergence.md)
- [ADR-0009 No Pre-launch Registry Compatibility](ADR-0009-no-prelaunch-registry-compatibility.md)
