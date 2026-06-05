# Evolith 文档地图

本文档定义 Evolith 的工程文档组织方式。新增文档前先判断它属于哪一类，避免把操作流程、稳定参考和临时计划混在一起。

## 文档分层

| 目录 | 用途 | 更新时机 |
|------|------|----------|
| `docs/BOARD.md` | 派生运营看板，汇总 Now / Review / Blocked / Next / Later | owner docs 状态变化后同步；不得替代 backlog 或 iteration |
| `docs/reference/` | 稳定事实、架构索引、配置说明、项目地图 | 代码结构或配置边界变化时 |
| `docs/sop/` | 标准操作程序，面向可执行步骤 | 流程、命令或验证方式变化时 |
| `docs/backlog/` | 需求池、故事拆分、优先级和状态 | 新需求进入、状态变化或迭代规划时 |
| `docs/iterations/` | 每轮迭代计划、验证结果和复盘 | 迭代开始和结束时 |
| `docs/decisions/` | 重要技术/产品决策 ADR | 技术栈、架构或产品边界发生重大取舍时 |
| `docs/roadmap/` | 盘点、路线图、阶段检查点 | 阶段规划或汇报口径变化时 |
| `docs/proposals/` | 想法暂存区，尚未满足 Backlog 进入条件的提案（Agent 不从中选取任务） | 晋升到 Backlog 或废弃时 |
| `docs/archive/` | 历史快照、过期路线、压缩后的经验 | 归档时 |
| `EVOLUTION.md` | 故障速查和经验写回 | 按 `docs/sop/EVOLUTION-FEEDBACK.md` 判断后写入 |
| `CLAUDE.md` / `GEMINI.md` | Agent 专用启动重定向 | skill 标准更新或 Agent 入口变化时 |

## 当前入口

### Root Entrypoints

- [AGENTS.md](../AGENTS.md) — Agent 主启动文档和任务路由。
- [CLAUDE.md](../CLAUDE.md) — Claude Code 单行重定向入口。
- [GEMINI.md](../GEMINI.md) — Gemini CLI 单行重定向入口。

### Reference

- [项目地图](reference/PROJECT-MAP.md) — 代码结构、运行入口、关键边界。
- [架构设计](reference/ARCHITECTURE.md) — 系统架构与模块设计。
- [API 合约](reference/API-CONTRACT.md) — HTTP API、认证和错误响应。
- [技术栈](reference/TECH-STACK.md) — 技术选型与依赖说明。
- [配置参考](reference/CONFIG.md) — 环境变量、嵌套配置键和常见误用。
- [权限](reference/PERMISSIONS.md) — 多租户与 RBAC 设计。
- [多租户设计](reference/MULTI-TENANT.md) — 租户模型、成员和隔离边界。
- [计费](reference/BILLING.md) — Stripe、订阅和用量设计。
- [国际化](reference/I18N.md) — 语言资源和 i18n 约定。
- [前端设计系统](reference/DESIGN.md) — Figma 导入后的视觉 token、组件和页面设计参考。
- [测试](reference/TESTING.md) — 测试策略、测试位置和用例状态。
- [脚本发布说明](reference/SCRIPTS-RELEASE-NOTES.md) — 脚本行为变更记录。

### SOP

- [需求进入与 Backlog 整理](sop/REQUIREMENT-INTAKE.md) — 需求分流、Proposal 晋升、DoR 和 backlog 状态。
- [开始一次迭代](sop/START-ITERATION.md) — 选择 Ready story、创建迭代记录、同步 backlog 和验证链接。
- [特性迭代工作流](sop/ITERATION-WORKFLOW.md) — 迭代计划、XP 开发循环、DoD、Review 和 Retro。
- [分阶段结对开发](sop/PAIRING-WORKFLOW.md) — Driver 实现、Navigator 审查和提交前检查的单上下文结对流程。
- [迭代中需求变更](sop/CHANGE-CONTROL.md) — 开发中变更分类、半成品处理和防呆检查。
- [本地开发](sop/LOCAL-DEV.md) — lite/full 模式、常用检查、日志位置。
- [新增功能](sop/NEW-FEATURE.md) — 后端、前端、测试、文档的落地顺序。
- [API 契约优先](sop/CONTRACT-FIRST.md) — 先改合约、再实现和验证的接口变更流程。
- [数据库迁移](sop/DATABASE-MIGRATION.md) — SQLite/PostgreSQL 双轨 migration 和 repository 检查。
- [测试与验证](sop/TESTING.md) — 按变更类型选择验证命令和记录结果。
- [任务收口与完成声明](sop/TASK-CLOSURE.md) — 实施任务的闭环台账、状态同步、残余归口和完成判定。
- [经验写回与规则升级](sop/EVOLUTION-FEEDBACK.md) — 判断何时更新 `EVOLUTION.md`、何时升级为规则或检查。
- [文档一致性检查](sop/DOC-CHECK.md) — 断链、旧术语、提案边界和 ADR 替代关系检查。
- [发布与部署](sop/RELEASE.md) — 发布前检查、构建、验证和回滚。
- [Git 工作流](sop/GIT-WORKFLOW.md) — 提交前检查、提交信息和变更拆分。

### Backlog / Iterations / Decisions

- [Operating Board](BOARD.md) — 派生运营看板；只汇总 owner doc 状态和 gate，不作为状态源。
- [Product Backlog](backlog/PRODUCT-BACKLOG.md) — compact 需求决策入口；详情见 `docs/backlog/active/` item files 和 `docs/backlog/archive/`。
- [迭代目录](iterations/README.md) — 迭代记录和模板。
- [决策记录](decisions/README.md) — ADR 目录。
- [ADR-0001 前端采用 React + Vite + Bun](decisions/ADR-0001-react-vite-bun-frontend.md) — 前端技术路线决策。
- [ADR-0002 CLI 友好接口替代 Snippet](decisions/ADR-0002-cli-friendly-interface-replaces-snippet.md) — 产品概念迁移决策。

### Roadmap / Proposals

- [工程化路线图](roadmap/ENGINEERING-ROADMAP.md) — 文档治理、流程治理和待补齐能力。
- [实施路线图](roadmap/IMPLEMENTATION-ROADMAP.md) — 阶段顺序、差距盘点和 Backlog / Proposals 归口映射。
- [提案目录](proposals/README.md) — 尚未进入实施的候选方案。
- [Evolith Rust CLI](proposals/RUST-CLI.md) — 本地智能体、skill、tool、CLI interface 管理 CLI。
- [前端嵌入后端](proposals/EMBEDDED-FRONTEND.md) — 前端静态产物打包进后端交付物。
- [Artifact Repository](proposals/ARTIFACT-REPOSITORY.md) — 模型、数据集、prompt、配置等制品管理远期方案。

### 产品与格式规范

| 文档 | 归类 | 说明 |
|------|------|------|
| [需求文档](reference/product/REQUIREMENTS.md) | 产品基线 | 核心需求、功能范围和验收口径 |
| [Skill 格式](reference/formats/SKILL-FORMAT.md) | 格式规范 | Claude Skills 兼容格式 |
| [CLI 友好接口格式](reference/formats/CLI-INTERFACE-FORMAT.md) | 格式规范 | 面向大模型和 CLI 调用的接口描述格式 |
| [Snippet 格式](reference/formats/SNIPPET-FORMAT.md) | 迁移参考 | 旧代码片段格式；新方向见 ADR-0002 |
| [Ideas](proposals/IDEAS.md) | 候选池 | 零散想法和未来方向，成熟后迁移到 `proposals/` 或 `roadmap/` |

## 重构原则

原有根目录文档已按新结构迁移，`docs/` 根目录只保留文档地图。后续按以下规则演进：

1. 不在 `docs/` 根目录新增专题文档。
2. 新需求先进入 `docs/backlog/PRODUCT-BACKLOG.md` 和对应 `docs/backlog/active/` item file，远期想法可先放 `docs/proposals/`。
3. 每轮开发在 `docs/iterations/` 留下计划、验证和复盘。
4. 新增操作流程放 `docs/sop/`。
5. 新增稳定事实或索引放 `docs/reference/`。
6. 重要技术取舍写入 `docs/decisions/`。
7. `docs/proposals/IDEAS.md` 中成熟的条目迁移到 `docs/backlog/`、`docs/proposals/` 或 `docs/roadmap/`。
8. 移动文档时同步更新 README、AGENTS、`docs/README.md` 和文档内相对链接。

## 写文档规则

1. SOP 必须包含触发条件、前置检查、操作步骤、验证和失败恢复。
2. Reference 只写稳定事实，不写临时计划。
3. Roadmap 可以写取舍和阶段目标，但要和当前代码状态分开。
4. Backlog item 必须有优先级、状态和验收标准。
5. 任何脚本行为变更必须同步更新相关 SOP；如果变更影响使用者，也更新 `docs/reference/SCRIPTS-RELEASE-NOTES.md`。
6. 发现新坑时先按 [经验写回与规则升级](sop/EVOLUTION-FEEDBACK.md) 判断，再写入 `EVOLUTION.md` 或升级规则/检查。
7. Agent 参与生成的 Git commit 必须遵守 [Git 工作流](sop/GIT-WORKFLOW.md)，提交信息末尾带 `[model: <name>]`。
