# EVO-049-A Skill/CLI 规范兼容数据模型基线

> Archived backlog item. Routing index: [2026 Q2 Archive](INDEX.md). Active routing surface: [Product Backlog](../../PRODUCT-BACKLOG.md).

- Type: tech-debt
- Status: Done
- Priority: P0
- Source: EVO-049 split / Iteration 034
- Decision Context: 2026-06-04 完成：migration 006（skills +13 列 / snippets +8 列）；domain/DTO/repository 全量更新；SnippetRepository 新增 update 方法；282 tests passed（+8 新增）

#### Source Detail Snapshot

- 类型：tech-debt
- 优先级：P0
- 状态：Done
- 父 Epic：EVO-049
- Story 形态：Technical
- 用户价值或技术目标：
  - 为了：让 Skill 与 CLI interface 能承载 Agent Skills 规范和结构化 CLI 命令字段。
  - 我希望/需要：完成双数据库 migration、domain model、DTO 和 repository 映射的最小兼容基线。
  - 以便：后续 parser 接线、制品打包、导入和执行引擎不再建立在单文本字段或 legacy snippet 字段上。
- 范围：
  - SQLite 与 PostgreSQL 双轨 migration：为 skills 增加 Agent Skills 兼容字段，为 snippets/CLI interface 增加 command、inputs、output、examples、error_model 等结构化字段。
  - 更新 domain model、Create/Update 请求 DTO、Response DTO 和 SQLite/PostgreSQL repository 映射。
  - 保留 legacy `skill_md`、`language`、`framework`、`code` 等兼容字段，不在本轮删除旧 API。
  - 更新相关 reference 或 API contract 中的数据字段说明。
- 不做：
  - 不接线 parser 到 create/update handler；归 EVO-049-B。
  - 不实现 ZIP/Git 导入、制品下载或对象存储。
  - 不实现 CLI 执行引擎、MCP serverless 或评分体系。
  - 不改前端复杂编辑器，仅保证 API response 字段可承载后续 UI。
- 验收标准：
  - [x] SQLite 与 PostgreSQL migration 字段语义一致，默认值和 nullable 策略明确。
  - [x] `Skill` / `NewSkill` / `UpdateSkill` 与 CLI interface 域模型包含规范兼容字段。
  - [x] API DTO 和 repository 映射能读写新增字段，旧字段兼容不破坏。
  - [x] 双数据库 repository 测试覆盖新增字段的 create/update/read。
  - [x] API contract 或 format reference 记录本轮新增字段和后续 parser 接线边界。
- 依赖或阻塞：无硬依赖；激活前需确认不与 EVO-027/EVO-028 已发布计划基线冲突。
- 解锁内容：EVO-049-B parser 接线与校验报告；EVO-045 CLI 执行引擎的数据路由前置。
- 影响范围：backend / db / docs
- 最小验证方式：`cargo test -p infra`；`cargo test -p domain`（如适用）；`cargo test -p api`；`cargo check --workspace`；`git diff --check`。
