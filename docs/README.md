# Evolith 文档地图

本文档定义 Evolith 的工程文档组织和当前关键入口。新增文档前先判断归口，避免把稳定事实、执行顺序、操作流程和历史计划混在一起。

> **2026-09-10 架构改线**：Evolith 已接受 WalGit-backed Git Data Plane 作为长期目标。当前 runtime 仍是 filesystem-backed Git；实现改造归 [EVO-126](backlog/active/EVO-126-walgit-git-data-plane-refactor.md)。当前整体状态以 [Project Status Baseline 2026-09-10](reference/PROJECT-STATUS-BASELINE-2026-09-10.md) 为准，当前激活顺序以 [Current Execution Plan 2026-09](roadmap/CURRENT-EXECUTION-PLAN-2026-09.md) 为准。

## 文档分层

| 目录 | 用途 | Owner 规则 |
|------|------|------------|
| `docs/reference/` | 当前稳定事实、架构、配置、权限和状态/生产基线 | 代码/配置事实变化时更新 |
| `docs/sop/` | 可执行步骤、门禁、验证和失败恢复 | 流程或安全边界变化时更新 |
| `docs/backlog/` | 可执行需求、优先级、验收和状态 | 新问题先进入这里 |
| `docs/iterations/` | 计划基线、执行证据、Review 和 Retro | 开始/结束迭代时更新 |
| `docs/decisions/` | 已接受重大技术和产品取舍 | 重大边界变化时写 ADR |
| `docs/roadmap/` | 阶段顺序和当前/历史执行计划 | 排序或阶段目标变化时新建/更新 owner，不覆写历史计划基线 |
| `docs/proposals/` | 未满足 Backlog DoR 的候选方案 | Agent 不从 Proposal 直接开工 |
| `docs/design/` | 产品交互与实施级设计 | 由 ADR/Backlog 约束；不得单独作为完成证据 |
| `docs/archive/` | 历史快照和非活跃记录 | 不作为默认执行入口 |
| `docs/BOARD.md` | 派生运营视图 | 只反映 owner docs，不自行定义状态 |
| `EVOLUTION.md` | 故障速查和经验写回 | 按 EVOLUTION-FEEDBACK 判断 |

## 当前最高优先级入口

1. [Project Status Baseline 2026-09-10](reference/PROJECT-STATUS-BASELINE-2026-09-10.md) — 当前实现、已完成能力、未完成计划、GIT-DP-01 与目标架构的统一基线。
2. [ADR-0011 WalGit-backed Git Data Plane](decisions/ADR-0011-walgit-backed-git-data-plane.md) — 接受 object-store/WAL data plane；Evolith 保持唯一控制面；ADR-0006 只保留为当前 Alpha/历史实现依据。
3. [WalGit Git Data Plane Refactor Design](design/WALGIT-GIT-DATA-PLANE-REFACTOR.md) — `service-git v2`、Smart HTTP、WAL write、Repo Context、bundle-uri、migration/recovery 的目标设计。
4. [Current Execution Plan 2026-09](roadmap/CURRENT-EXECUTION-PLAN-2026-09.md) — **唯一当前激活顺序 owner**。
5. [EVO-126 Epic](backlog/active/EVO-126-walgit-git-data-plane-refactor.md) — P0 架构重构；A Ready，B~H Proposed；GitHub tracking #12~#20。
6. [Production Readiness Baseline](reference/PRODUCTION-READINESS-BASELINE.md) — SEC/DATA/EVENT 历史 Gate 与 DEPLOY-01；涉及新 data plane 时必须与新 Project Status Baseline 联读。
7. [Security Review](sop/SECURITY-REVIEW.md)、[Release](sop/RELEASE.md)、[Document Check](sop/DOC-CHECK.md) — Git write、storage、迁移与收口强制门禁。

当前执行链：

```text
EVO-126-A
→ EVO-126-B
→ EVO-126-C / E / D（按依赖小批次）
→ EVO-126-F
→ EVO-126-G
→ EVO-126-H / GIT-DP-01 Close
→ EVO-105 / 106 / 107 / 104
→ Experience / Discovery / legacy cleanup
→ EVO-118-E final production convergence
```

EVO-118-G-A / Iteration 056 的 Review/Partial residual 保留并在 release 前处置，不与 EVO-126 混入同一旧 Iteration。

## Root Entrypoints

- [AGENTS.md](../AGENTS.md) — Agent 主启动文档、硬约束和 Task Router；其中 Rust 1.90 由 EVO-126-A 建立；filesystem Git 在 EVO-126-H 前仍是 current runtime fact。
- [CLAUDE.md](../CLAUDE.md) — Claude Code 重定向入口。
- [GEMINI.md](../GEMINI.md) — Gemini CLI 重定向入口。

## Reference

- [Project Status Baseline 2026-09-10](reference/PROJECT-STATUS-BASELINE-2026-09-10.md) — 当前整体状态与新架构迁移 Gate。
- [生产就绪与项目完成度基线](reference/PRODUCTION-READINESS-BASELINE.md) — Release Gates 与历史生产证据。
- [架构设计](reference/ARCHITECTURE.md) — 当前代码/运行架构；在 EVO-126-H 前继续描述 filesystem-backed runtime。
- [项目地图](reference/PROJECT-MAP.md) — 代码结构和关键路径。
- [API 合约](reference/API-CONTRACT.md) — HTTP API、认证与错误响应。
- [API Key 与 MCP 授权合约](reference/API-KEY-AUTHORIZATION.md) — API Keys/MCP/scope 安全细化契约。
- [技术栈](reference/TECH-STACK.md) — Manifest/Lockfile 技术版本；Rust 1.90 基线由 EVO-126-A 维护。
- [WalGit Dependency Baseline](reference/WALGIT-DEPENDENCY-BASELINE.md) — exact upstream SHA、允许的 engine crates、禁止的 server boundary、lock/license 与升级流程。
- [配置参考](reference/CONFIG.md) — 当前配置；object-store Git 配置由 EVO-126-F 更新。
- [权限](reference/PERMISSIONS.md) — owner/admin/member 与 Typed Capability。
- [多租户设计](reference/MULTI-TENANT.md) — 租户模型和隔离边界。
- [计费](reference/BILLING.md) — Stripe、订阅和用量设计。
- [测试](reference/TESTING.md) — 测试策略和历史用例状态。
- [前端设计系统](reference/DESIGN.md) — 视觉 Token、组件和页面参考。
- [产品交互架构](design/PRODUCT-INTERACTION-ARCHITECTURE.md) — 目标业务流、Route Ownership Matrix 和页面编排。
- [脚本发布说明](reference/SCRIPTS-RELEASE-NOTES.md) — 脚本行为变更记录；migration/backup/restore 行为变化必须同步。

## Architecture / Design

- [ADR-0011](decisions/ADR-0011-walgit-backed-git-data-plane.md) — WalGit-backed Git Data Plane 决策。
- [WalGit Refactor Design](design/WALGIT-GIT-DATA-PLANE-REFACTOR.md) — 本轮 Git engine 重构实施设计。
- [ADR-0004 Git-Centric Storage](decisions/ADR-0004-git-centric-storage.md) — Git 事实源核心决策；storage 实现后果被 ADR-0011 更新。
- [ADR-0006 Smart HTTP via Git subprocess](decisions/ADR-0006-smart-http-via-git-subprocess.md) — 当前 Alpha/历史实现依据；长期目标由 ADR-0011 supersede。
- [ADR-0007 Agent write boundaries](decisions/ADR-0007-agent-write-and-production-delivery-boundaries.md) — Policy/Scoped Token 边界继续有效。
- [ADR-0008 Repo-centric Interaction](decisions/ADR-0008-repo-centric-interaction-architecture.md) — 产品交互继续有效。
- [ADR-0009 No Pre-launch Compatibility](decisions/ADR-0009-no-prelaunch-registry-compatibility.md) — 不建设旧 Registry 双写兼容。
- [ADR-0010 Final Production Convergence](decisions/ADR-0010-final-production-convergence-after-product-completion.md) — 最终 DEPLOY-01 继续最后执行。
- [决策记录索引](decisions/README.md) — ADR 全量索引与 supersede 关系。

## SOP

- [需求进入与 Backlog 整理](sop/REQUIREMENT-INTAKE.md) — 分流、DoR、Epic/Story 和状态。
- [开始一次迭代](sop/START-ITERATION.md) — Iteration 库存处置和启动。
- [特性迭代工作流](sop/ITERATION-WORKFLOW.md) — XP 循环、DoD、Review、Retro。
- [分阶段结对开发](sop/PAIRING-WORKFLOW.md) — Driver/Navigator。
- [安全敏感变更审查](sop/SECURITY-REVIEW.md) — 授权、Git write、storage、数据耐久性和生产 fail-closed。
- [迭代中需求变更](sop/CHANGE-CONTROL.md) — 变更分类与 product-pivot/replan。
- [本地开发](sop/LOCAL-DEV.md) — Lite/Full 模式和常用命令。
- [新增功能](sop/NEW-FEATURE.md) — 后端、前端、测试、文档落地顺序。
- [API 契约优先](sop/CONTRACT-FIRST.md) — Contract First。
- [数据库迁移](sop/DATABASE-MIGRATION.md) — SQLite/PostgreSQL 双轨。
- [测试与验证](sop/TESTING.md) — 风险匹配验证。
- [任务收口与完成声明](sop/TASK-CLOSURE.md) — Complete/Partial/Blocked。
- [文档一致性检查](sop/DOC-CHECK.md) — 链接、状态、替代和计划基线。
- [发布与部署](sop/RELEASE.md) — Production Gate、恢复和回滚。
- [Git 工作流](sop/GIT-WORKFLOW.md) — 提交规范和分支/PR。
- [经验写回与规则升级](sop/EVOLUTION-FEEDBACK.md) — EVOLUTION 或规则升级。

## Backlog / Board / Iterations

- [Operating Board](BOARD.md) — 当前 Now/Next/Blocked/Done 派生视图。
- [Product Backlog](backlog/PRODUCT-BACKLOG.md) — 当前优先级与 Required Reads。
- [EVO-126](backlog/active/EVO-126-walgit-git-data-plane-refactor.md) — WalGit Git Data Plane Epic。
- [EVO-126-A](backlog/active/EVO-126-A-walgit-dependency-toolchain-boundary.md) — Iteration 069 In Progress；Rust 1.90 + exact WalGit dependency boundary。
- [EVO-118](backlog/active/EVO-118-production-readiness-and-security-hardening.md) — Production Readiness Epic；E 为最终 release gate，G-A 有 residual。
- [EVO-100](backlog/active/EVO-100-git-centric-platform-foundation.md) — Git-centric product foundation；Agent/Discovery 子项等待新 data plane。
- [EVO-121](backlog/active/EVO-121-product-experience-convergence.md) — Experience Convergence；C/D Done，A/B/E/F 待激活。
- [迭代目录](iterations/README.md) — 迭代索引和库存；EVO-126 实施必须新建 Iteration，不改写旧计划。

## Roadmap / Proposals

- [Current Execution Plan 2026-09](roadmap/CURRENT-EXECUTION-PLAN-2026-09.md) — 当前唯一激活顺序 owner。
- [实施路线图](roadmap/IMPLEMENTATION-ROADMAP.md) — 阶段级长期路线；涉及当前启动顺序时服从 2026-09 plan。
- [Production Readiness Plan 2026-07](roadmap/PRODUCTION-READINESS-PLAN-2026-07.md) — 历史执行基线；2026-09 起不再拥有当前激活顺序。
- [Two-Month Phase E' Plan](roadmap/TWO-MONTH-PLAN-2026-07.md) — 2026-06-29 历史计划基线。
- [工程化路线图](roadmap/ENGINEERING-ROADMAP.md) — 治理和工程能力。
- [提案目录](proposals/README.md) — 未成熟候选。
- [Git-Centric Platform Proposal](proposals/GIT-CENTRIC-PLATFORM.md) — 产品方向背景；具体 Git data-plane target 服从 ADR-0011。

## 产品与格式规范

| 文档 | 归类 | 说明 |
|------|------|------|
| [需求文档](reference/product/REQUIREMENTS.md) | 产品基线 | 核心需求和范围 |
| [Policy YAML](reference/formats/POLICY-YAML-FORMAT.md) | 格式规范 | Parser 已有；执行行为仍归 EVO-105 |
| [Skill 格式](reference/formats/SKILL-FORMAT.md) | 格式规范 | Repo 派生能力 |
| [CLI 友好接口格式](reference/formats/CLI-INTERFACE-FORMAT.md) | 格式规范 | Repo 派生能力 |
| [Snippet 格式](reference/formats/SNIPPET-FORMAT.md) | 迁移参考 | legacy，不作为新产品概念 |

## 写文档规则

1. Reference 写当前事实；ADR 写接受的决策；Design 写目标结构；Roadmap 写顺序；Backlog 写可执行验收；Iteration 写计划和证据；Board 只派生。
2. **Current implementation 与 Accepted target 必须显式区分。** EVO-126-H 前不得把 filesystem Git 写成已被 object store 替代。
3. 新安全、数据损坏、事件丢失、Git publication 或生产构建问题必须进入 P0/P1 Backlog，不只留在对话或 PR 评论。
4. 已发布计划不可因改线被覆写；新顺序使用新 Roadmap/Iteration，并保留原基线。
5. 生产就绪声明必须引用最终 data plane 上的 clean build、负向测试、持久化和恢复证据。
6. 修改脚本行为同步 Release Notes；发现新陷阱按 EVOLUTION-FEEDBACK 写回。
7. Agent Git commit 必须遵守 GIT-WORKFLOW，末尾带 `[model: <model-name>]`。
