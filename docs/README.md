# Evolith 文档地图

本文档定义 Evolith 的工程文档组织方式。新增文档前先判断它属于哪一类，避免把操作流程、稳定参考和临时计划混在一起。

## 文档分层

| 目录 | 用途 | 更新时机 |
|------|------|----------|
| `docs/reference/` | 稳定事实、架构索引、配置说明、项目地图 | 代码结构或配置边界变化时 |
| `docs/sop/` | 标准操作程序，面向可执行步骤 | 流程、命令或验证方式变化时 |
| `docs/backlog/` | 需求池、故事拆分、优先级和状态 | 新需求进入、状态变化或迭代规划时 |
| `docs/iterations/` | 每轮迭代计划、验证结果和复盘 | 迭代开始和结束时 |
| `docs/decisions/` | 重要技术/产品决策 ADR | 技术栈、架构或产品边界发生重大取舍时 |
| `docs/roadmap/` | 盘点、路线图、阶段检查点 | 阶段规划或汇报口径变化时 |
| `docs/planned/` | 已讨论但尚未实施的方案 | 方案进入实施前或废弃时 |
| `docs/archive/` | 历史快照、过期路线、压缩后的经验 | 归档时 |
| `EVOLUTION.md` | 故障速查和经验写回 | 发现新陷阱或被纠正时 |

## 当前入口

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
- [测试](reference/TESTING.md) — 测试策略、测试位置和用例状态。
- [脚本发布说明](reference/SCRIPTS-RELEASE-NOTES.md) — 脚本行为变更记录。

### SOP

- [特性迭代工作流](sop/ITERATION-WORKFLOW.md) — backlog、DoR/DoD、迭代计划和 XP 开发循环。
- [本地开发](sop/LOCAL-DEV.md) — lite/full 模式、常用检查、日志位置。
- [新增功能](sop/NEW-FEATURE.md) — 后端、前端、测试、文档的落地顺序。
- [API 契约优先](sop/CONTRACT-FIRST.md) — 先改合约、再实现和验证的接口变更流程。
- [数据库迁移](sop/DATABASE-MIGRATION.md) — SQLite/PostgreSQL 双轨 migration 和 repository 检查。
- [测试与验证](sop/TESTING.md) — 按变更类型选择验证命令和记录结果。
- [发布与部署](sop/RELEASE.md) — 发布前检查、构建、验证和回滚。
- [Git 工作流](sop/GIT-WORKFLOW.md) — 提交前检查、提交信息和变更拆分。

### Backlog / Iterations / Decisions

- [Product Backlog](backlog/PRODUCT-BACKLOG.md) — 统一需求池和优先级。
- [迭代目录](iterations/README.md) — 迭代记录和模板。
- [决策记录](decisions/README.md) — ADR 目录。
- [ADR-0001 前端采用 React + Vite + Bun](decisions/ADR-0001-react-vite-bun-frontend.md) — 前端技术路线决策。
- [ADR-0002 CLI 友好接口替代 Snippet](decisions/ADR-0002-cli-friendly-interface-replaces-snippet.md) — 产品概念迁移决策。

### Roadmap / Planned

- [工程化路线图](roadmap/ENGINEERING-ROADMAP.md) — 文档治理、流程治理和待补齐能力。
- [开发计划](roadmap/DEVELOPMENT-PLAN.md) — 需求差距、前端迁移和阶段计划。
- [规划中事项](planned/README.md) — 尚未进入实施的候选方案。
- [Evolith Rust CLI](planned/RUST-CLI.md) — 本地智能体、skill、tool、CLI interface 管理 CLI。
- [前端嵌入后端](planned/EMBEDDED-FRONTEND.md) — 前端静态产物打包进后端交付物。
- [Artifact Repository](planned/ARTIFACT-REPOSITORY.md) — 模型、数据集、prompt、配置等制品管理远期方案。

### 产品与格式规范

| 文档 | 归类 | 说明 |
|------|------|------|
| [需求文档](reference/product/REQUIREMENTS.md) | 产品基线 | 核心需求、功能范围和验收口径 |
| [Skill 格式](reference/formats/SKILL-FORMAT.md) | 格式规范 | Claude Skills 兼容格式 |
| [Snippet 格式](reference/formats/SNIPPET-FORMAT.md) | 迁移参考 | 旧代码片段格式；新方向见 ADR-0002 |
| [Ideas](planned/IDEAS.md) | 候选池 | 零散想法和未来方向，成熟后迁移到 `planned/` 或 `roadmap/` |

## 重构原则

原有根目录文档已按新结构迁移，`docs/` 根目录只保留文档地图。后续按以下规则演进：

1. 不在 `docs/` 根目录新增专题文档。
2. 新需求先进入 `docs/backlog/PRODUCT-BACKLOG.md`，远期想法可先放 `docs/planned/`。
3. 每轮开发在 `docs/iterations/` 留下计划、验证和复盘。
4. 新增操作流程放 `docs/sop/`。
5. 新增稳定事实或索引放 `docs/reference/`。
6. 重要技术取舍写入 `docs/decisions/`。
7. `docs/planned/IDEAS.md` 中成熟的条目迁移到 `docs/backlog/`、`docs/planned/` 或 `docs/roadmap/`。
8. 移动文档时同步更新 README、AGENTS、`docs/README.md` 和文档内相对链接。

## 写文档规则

1. SOP 必须包含触发条件、前置检查、操作步骤、验证和失败恢复。
2. Reference 只写稳定事实，不写临时计划。
3. Roadmap 可以写取舍和阶段目标，但要和当前代码状态分开。
4. Backlog item 必须有优先级、状态和验收标准。
5. 任何脚本行为变更必须同步更新相关 SOP；如果变更影响使用者，也更新 `docs/reference/SCRIPTS-RELEASE-NOTES.md`。
6. 发现新坑时写入 `EVOLUTION.md`，不要只留在对话里。
7. Agent 参与生成的 Git commit 必须遵守 [Git 工作流](sop/GIT-WORKFLOW.md)，提交信息末尾带 `[model: <name>]`。
