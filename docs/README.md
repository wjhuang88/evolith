# Evolith 文档地图

本文档定义 Evolith 的工程文档组织和当前关键入口。新增文档前先判断归口，避免把稳定事实、执行顺序、操作流程和历史计划混在一起。

## 文档分层

| 目录 | 用途 | Owner 规则 |
|------|------|------------|
| `docs/reference/` | 当前稳定事实、架构、配置、权限和生产就绪基线 | 代码/配置事实变化时更新 |
| `docs/sop/` | 可执行步骤、门禁、验证和失败恢复 | 流程或安全边界变化时更新 |
| `docs/backlog/` | 可执行需求、优先级、验收和状态 | 新问题先进入这里 |
| `docs/iterations/` | 计划基线、执行证据、Review 和 Retro | 开始/结束迭代时更新 |
| `docs/decisions/` | 已接受重大技术/产品取舍 | 重大边界变化时写 ADR |
| `docs/roadmap/` | 阶段顺序和当前执行计划 | 排序/阶段目标变化时更新 |
| `docs/proposals/` | 未满足 Backlog DoR 的候选方案 | Agent 不从 Proposal 直接开工 |
| `docs/archive/` | 历史快照和非活跃记录 | 不作为默认执行入口 |
| `docs/BOARD.md` | 派生运营视图 | 只反映 owner docs，不自行定义状态 |
| `EVOLUTION.md` | 故障速查和经验写回 | 按 EVOLUTION-FEEDBACK 判断 |

## 当前最高优先级入口

1. [Production Readiness Baseline](reference/PRODUCTION-READINESS-BASELINE.md) — 当前成熟度、发布阻断和环境 Gate 的事实 owner；SEC-01/SEC-02 已解除，DATA-01/DEPLOY-01 仍开放。
2. [Production Readiness Plan](roadmap/PRODUCTION-READINESS-PLAN-2026-07.md) — 2026-07-30 后的当前执行顺序 owner。
3. [EVO-118 Epic](backlog/active/EVO-118-production-readiness-and-security-hardening.md) — 生产就绪硬化的可执行 Story 容器；A/B/C Done，下一项为 D。
4. [EVO-118-D Git Durability and Recovery](backlog/active/EVO-118-D-git-storage-durability-and-recovery.md) — 当前下一条 Ready P0，负责关闭 DATA-01。
5. [EVO-118-E Production Build Convergence](backlog/active/EVO-118-E-production-build-deployment-convergence.md) — 随后的 Ready P0，负责关闭 DEPLOY-01。
6. [EVO-118-C HTTP Tool Egress Security](backlog/active/EVO-118-C-http-tool-egress-security.md) — Done；Navigator accepted runtime security，CI #125 全绿，稳定权限/API 契约完成，SEC-02 已解除。
7. [Iteration 052](iterations/ITERATION-052.md) — Closed / Complete；EVO-118-C 的计划基线、执行证据、Review 和 Retro owner。
8. [Permissions](reference/PERMISSIONS.md) 与 [API Contract](reference/API-CONTRACT.md) — Tool create/update 的 Owner/Admin JWT、auth-first、Egress 校验、错误和隐藏语义稳定契约。
9. [Security Review SOP](sop/SECURITY-REVIEW.md) 与 [Release SOP](sop/RELEASE.md) — 后续安全、耐久性和交付 Gate 的强制审查与发布步骤。

> 当前顺序：EVO-118-D → E → F/G/H → EVO-112 → Agent/Vibe/Indexer 主线。原 Two-Month Plan 保留为历史计划基线，不再作为当前激活顺序。

## Root Entrypoints

- [AGENTS.md](../AGENTS.md) — Agent 主启动文档、硬约束和 Task Router。
- [CLAUDE.md](../CLAUDE.md) — Claude Code 重定向入口。
- [GEMINI.md](../GEMINI.md) — Gemini CLI 重定向入口。

## Reference

- [生产就绪与项目完成度基线](reference/PRODUCTION-READINESS-BASELINE.md) — Git Alpha、产品完成度、Release Gates 和环境准入。
- [架构设计](reference/ARCHITECTURE.md) — 模块化单体、事实源、数据边界和目标 Application Services。
- [项目地图](reference/PROJECT-MAP.md) — 代码结构和关键路径。
- [API 合约](reference/API-CONTRACT.md) — 完整 HTTP API、认证和错误响应；包含 Tool 管理与 Egress 稳定契约。
- [API Key 与 MCP 授权合约](reference/API-KEY-AUTHORIZATION.md) — API Keys/MCP/scope 的安全细化合约；相关冲突时优先。
- [技术栈](reference/TECH-STACK.md) — Manifest/Lockfile 技术版本。
- [配置参考](reference/CONFIG.md) — 环境变量和嵌套配置键。
- [权限](reference/PERMISSIONS.md) — 当前 owner/admin/member 角色、Typed capability、Tool 管理边界和 legacy 轮换。
- [多租户设计](reference/MULTI-TENANT.md) — 租户模型和隔离边界。
- [计费](reference/BILLING.md) — Stripe、订阅和用量设计。
- [测试](reference/TESTING.md) — 测试策略、位置和历史用例状态。
- [前端设计系统](reference/DESIGN.md) — 视觉 Token、组件和页面参考。
- [脚本发布说明](reference/SCRIPTS-RELEASE-NOTES.md) — 脚本行为变更记录。

## SOP

- [需求进入与 Backlog 整理](sop/REQUIREMENT-INTAKE.md) — 分流、DoR、Epic/Story 和状态。
- [开始一次迭代](sop/START-ITERATION.md) — Iteration 库存处置和启动。
- [特性迭代工作流](sop/ITERATION-WORKFLOW.md) — XP 循环、DoD、Review、Retro。
- [分阶段结对开发](sop/PAIRING-WORKFLOW.md) — Driver/Navigator。
- [安全敏感变更审查](sop/SECURITY-REVIEW.md) — 授权、SSRF、Git 写入、数据耐久性和生产 fail-closed。
- [迭代中需求变更](sop/CHANGE-CONTROL.md) — 变更分类和半成品处置。
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

## Backlog / Iterations / Decisions

- [Operating Board](BOARD.md) — 当前 Now/Done/Blocked/Next/Later 派生视图。
- [Product Backlog](backlog/PRODUCT-BACKLOG.md) — 当前优先级与 Required Reads。
- [EVO-118](backlog/active/EVO-118-production-readiness-and-security-hardening.md) — Production Readiness Epic；A/B/C Done，D/E Ready。
- [EVO-118-B](backlog/active/EVO-118-B-api-key-mcp-authorization-hardening.md) — Done；Iteration 051 和 PR #3 的历史安全闭环记录，SEC-01 已解除。
- [EVO-118-C](backlog/active/EVO-118-C-http-tool-egress-security.md) — Done / Complete；负责 SEC-02 HTTP Tool SSRF / Egress Security，稳定契约和关闭证据已同步。
- [Iteration 052](iterations/ITERATION-052.md) — Closed / Complete；记录 exact-head CI、Navigator remediation、稳定契约与治理关闭。
- [迭代目录](iterations/README.md) — 迭代索引和库存；当前没有 Active / Review Iteration。
- [决策记录](decisions/README.md) — ADR 目录。
- [ADR-0004 Git-Centric Storage](decisions/ADR-0004-git-centric-storage.md) — Git 事实源方向。
- [ADR-0005 Deprecate Sandbox](decisions/ADR-0005-deprecate-sandbox-runtime.md) — legacy Sandbox 删除。
- [ADR-0006 Smart HTTP via Git subprocess](decisions/ADR-0006-smart-http-via-git-subprocess.md) — Git 协议实现边界。

## Roadmap / Proposals

- [实施路线图](roadmap/IMPLEMENTATION-ROADMAP.md) — 当前阶段和严格启动顺序。
- [Production Readiness Plan](roadmap/PRODUCTION-READINESS-PLAN-2026-07.md) — 当前激活计划。
- [Two-Month Phase E' Plan](roadmap/TWO-MONTH-PLAN-2026-07.md) — 2026-06-29 历史计划基线；当前已被 Readiness Plan 替代用于激活排序。
- [工程化路线图](roadmap/ENGINEERING-ROADMAP.md) — 治理和工程能力。
- [提案目录](proposals/README.md) — 未成熟候选。
- [Git-Centric Platform Proposal](proposals/GIT-CENTRIC-PLATFORM.md) — 产品方向背景。

## 产品与格式规范

| 文档 | 归类 | 说明 |
|------|------|------|
| [需求文档](reference/product/REQUIREMENTS.md) | 产品基线 | 核心需求和范围 |
| [Policy YAML](reference/formats/POLICY-YAML-FORMAT.md) | 格式规范 | Parser 已有；执行行为仍归 EVO-105 |
| [Skill 格式](reference/formats/SKILL-FORMAT.md) | 格式规范 | Repo 派生能力 |
| [CLI 友好接口格式](reference/formats/CLI-INTERFACE-FORMAT.md) | 格式规范 | Repo 派生能力 |
| [Snippet 格式](reference/formats/SNIPPET-FORMAT.md) | 迁移参考 | legacy，不作为新产品概念 |

## 写文档规则

1. Reference 写当前事实；Roadmap 写顺序；Backlog 写可执行验收；Iteration 写计划和证据；Board 只派生。
2. 新安全、数据损坏、事件丢失或生产构建问题必须进入 P0/P1 Backlog，不只留在对话或 PR 评论。
3. 已发布计划不可因改线被覆写；新顺序使用新 Roadmap/Iteration，并保留原基线。
4. 新 SOP 必须包含触发、前置、步骤、验证和失败恢复，并接入 AGENTS Task Router。
5. 生产就绪声明必须引用实际 clean build、负向测试、持久化和恢复证据。
6. 修改脚本行为同步 Release Notes；发现新陷阱按 EVOLUTION-FEEDBACK 写回。
7. Agent Git commit 必须遵守 GIT-WORKFLOW，末尾带 `[model: <model-name>]`。
