# EVO-009 SKILL.md 与 CLI interface frontmatter parser

> Archived backlog item. Routing index: [2026 Q2 Archive](INDEX.md). Active routing surface: [Product Backlog](../../PRODUCT-BACKLOG.md).

- Type: feature
- Status: Done
- Priority: P1
- Source: 格式规范 / EVO-017 / Iteration 010
- Decision Context: CLI interface parser 已有基线；SKILL.md parser 已实现

#### Source Detail Snapshot

- 类型：feature
- 优先级：P1
- 状态：Done
- 用户价值或技术目标：让 SKILL.md parser 从 stub 升级为真正解析 YAML frontmatter + Markdown body 的生产级解析器，为 Skill 上传校验（EVO-027/028）和 CLI interface 创建打基础。CLI interface parser 已有基线（Iteration 002），本次补齐 SKILL.md 侧。
- 范围：
  - 重写 `service-skill/src/parser.rs`：解析 SKILL.md 的 YAML frontmatter，提取 `name`、`description`、`version`、`type`、`execution`、`runtime`、`entrypoint`、`timeout`、`memory`、`dependencies`、`tags`、`author` 等字段。
  - 扩展 `SkillMetadata` 结构体，覆盖 SKILL-FORMAT.md 中定义的字段。
  - 实现校验逻辑：`name` 必填且符合命名规则（小写字母/数字/连字符，不首尾连字符，无连续连字符）；`description` 必填且 1-1024 字符；`version` 如存在须为合法 semver。
  - Markdown body 提取为独立字段。
  - 单元测试覆盖：合法 SKILL.md、缺少 frontmatter、缺少必填字段、name 格式非法、description 过长。
  - CLI interface parser 如有遗漏字段也一并补齐（当前已较完整）。
- 不做：
  - 不实现 Skill 包目录扫描（只解析单个 SKILL.md 文本）。
  - 不实现 Skill 导入管线（EVO-027）。
  - 不实现版本管理和校验报告模型（EVO-028）。
  - 不实现描述质量评分（EVO-029）。
  - 不修改 Skill API handler 或数据库 schema。
- 验收标准：
  - [x] `SkillParser::parse()` 能解析包含完整 frontmatter 的 SKILL.md 文本，返回 `SkillMetadata` + body。
  - [x] `name` 校验：必填、1-64 字符、小写字母/数字/连字符、不首尾连字符、无连续连字符。
  - [x] `description` 校验：必填、1-1024 字符。
  - [x] 缺少 frontmatter 或缺少必填字段返回 `ValidationError`。
  - [x] 可选字段（version、author、tags、dependencies 等）缺失时不报错，使用合理默认值。
  - [x] `cargo test -p service-skill` 通过，parser 单元测试 >= 5 个。
  - [x] `cargo check --workspace` 无错误。
  - [x] `cargo clippy --workspace` 无错误。
- 技术备注：
  - CLI interface parser（`service-snippet/src/parser.rs`）已有成熟的 frontmatter 拆分 + YAML 解析 + 校验模式，SKILL.md parser 应复用相同模式。
  - `serde_yaml` 已在 `service-skill/Cargo.toml` 中。
  - SKILL.md 格式规范见 `docs/reference/formats/SKILL-FORMAT.md`。
  - 当前 `SkillMetadata` 只有 5 个字段（name/description/version/author/runtime），需要扩展。
- 依赖：无外部依赖阻塞。EVO-017（CLI interface 概念迁移）已完成。
- 影响范围：backend
- 最小验证方式：`cargo test -p service-skill`；手工构造 SKILL.md 文本验证解析和校验。
