# EVO-049 Skill/CLI 生态兼容与行业标准对齐

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- [Skill format](../../reference/formats/SKILL-FORMAT.md)
- [CLI interface format](../../reference/formats/CLI-INTERFACE-FORMAT.md)

## Summary

| Field | Value |
| --- | --- |
| ID | EVO-049 |
| Type | feature |
| Status | Proposed |
| Priority | P0 |
| Parent Epic | None recorded |
| Source | 用户反馈 2026-05-29 |

## Problem Or Outcome

Epic；不直接进迭代，先执行 EVO-049-A/B 子 Story

## Goal And Non-Goals

- Goal: deliver the outcome described by the title and decision context without expanding scope during implementation.
- Non-goals: do not implement adjacent backlog items unless they are listed as hard dependencies in this file.

## Dependencies And Blockers

- No hard blocker recorded in the source backlog.

## Governing ADRs, Specs Or Decisions

- [Skill format](../../reference/formats/SKILL-FORMAT.md)
- [CLI interface format](../../reference/formats/CLI-INTERFACE-FORMAT.md)

## Acceptance Criteria

- 类型：feature / Epic
- 优先级：P0
- 状态：Proposed
- 父 Epic：无
- Story 形态：Epic
- 用户价值或技术目标：让 Evolith 平台的 Skill 和 CLI 完全兼容 Agent Skills 行业规范（agentskills.io），使任何支持该规范的 Agent 工具（Claude Codex、OpenAI Codex 等）都能使用 Evolith 上的 Skill 和 CLI，成为大生态的一部分。

#### 子 Story

| 子 Story | 独立结果 | 状态 | 依赖 | 所属迭代 |
|----------|----------|------|------|----------|
| EVO-049-A | Skill/CLI 规范兼容数据模型基线 | Done | 无 | Iteration 034 |
| EVO-049-B | Skill/CLI parser 接线与校验报告 | Blocked | EVO-049-A | - |
| 后续切片 | 制品打包、导入、前端编辑体验 | Proposed | EVO-049-B / EVO-020 | - |

#### 行业标准兼容性现状

Evolith SKILL-FORMAT.md 在 Agent Skills 规范 6 个标准字段（name/description/license/compatibility/metadata/allowed-tools）上完全匹配。主要差距：

| 项目 | 当前状态 | 目标 |
|------|----------|------|
| `version`/`author`/`tags` | 顶层 frontmatter 字段（非标准） | 移入 `metadata` 下，顶层保留兼容别名 |
| `when_to_use` | 缺失 | 新增，Claude Code 扩展字段 |
| `arguments` | 缺失 | 新增，支持 `$0`/`$name` 位置参数替换 |
| 目录结构（scripts/references/assets） | 格式文档定义了，但存储只存 `skill_md` 单文本 | 支持完整目录打包和对象存储 |
| Frontmatter 解析器 | 已实现（`service-skill/parser.rs`、`service-snippet/parser.rs`）但未接线到 handler | 在创建/导入/更新流程中调用 |
| CLI 结构化字段 | 格式文档定义了（command/inputs/output/error_model），但模型只有 name/language/code/content | 数据模型新增所有 CLI 格式字段 |
| 制品打包/下载 | 无 | ZIP 打包下载 API，外部工具可直接使用 |

#### 实施子任务与顺序

**Phase 1：数据模型升级（前后端对齐）**

1. **DB migration — skills 表新增列**
   - SQLite + PostgreSQL 双轨 migration
   - 新增列：`author TEXT`、`tags JSON`、`skill_type TEXT DEFAULT 'instruction'`、`execution TEXT DEFAULT 'client'`、`entrypoint TEXT`、`timeout INTEGER DEFAULT 30`、`memory_mb INTEGER DEFAULT 256`、`permissions JSON`、`license TEXT`、`compatibility TEXT`、`disable_model_invocation BOOLEAN DEFAULT 0`、`user_invocable BOOLEAN DEFAULT 1`、`argument_hint TEXT`、`skill_package_path TEXT`
   - 现有 `skill_md` 列保留，存放完整 SKILL.md 原文；新列存放解析后的结构化字段

2. **DB migration — snippets 表新增列（CLI 字段）**
   - 新增列：`version TEXT`、`summary TEXT`、`command TEXT`、`subcommands JSON`、`inputs JSON`、`output JSON`、`examples JSON`、`error_model JSON`
   - 现有 `language`/`framework`/`code` 列保留兼容；`command` 替代 `code` 的语义角色

3. **Domain model 更新**
   - `Skill` 结构体：新增所有 migration 列对应的 Rust 字段
   - `NewSkill`/`UpdateSkill`：同步新增可选字段
   - `Snippet` 结构体：新增 CLI Interface 字段（version/summary/command/subcommands/inputs/output/examples/error_model）
   - `NewSnippet`/`UpdateSnippet`：同步新增

4. **DTO 更新**
   - `CreateSkillRequest`/`SkillResponse`：新增 author/tags/skill_type/execution/entrypoint/timeout/memory_mb/permissions/license/compatibility 等字段
   - `CreateSnippetRequest`/`SnippetResponse`：新增 version/summary/command/subcommands/inputs/output/examples/error_model
   - `SkillResponse`：移除硬编码空字符串的 `category`/`tags`，改为真实字段映射

5. **Repository 实现**
   - 4 个 repo 实现（SQLite skill/snippet + PostgreSQL skill/snippet）的 SQL 和映射同步更新

**Phase 2：Parser 接线与校验**

6. **Skill handler 接线 SkillParser**
   - `create_skill`：接收 content → `SkillParser::parse()` → 提取 metadata 字段 → 存入结构化列 + `skill_md` 存原文
   - `update_skill`：同上
   - `load_skill`/`execute_skill`：使用 `entrypoint` + `skill_package_path` 定位代码，不再把 `skill_md` 当代码执行

7. **Snippet handler 接线 CliInterfaceParser**
   - `create_snippet`：接收 content → `CliInterfaceParser::parse()` → 提取 metadata → 存入结构化列
   - `update_snippet`：实现（当前 501）

8. **Frontmatter 校验**
   - 创建和导入时校验：name 格式（小写+连字符，≤64字符，无连续/首尾连字符）、description 非空且 ≤1024字符、name 无 XML 标签、name 无保留词（anthropic/claude）
   - 对齐 OpenAI `quick_validate.py` 规则
   - 区分 blocking error 和 warning

**Phase 3：制品打包与导入**

9. **Skill 制品打包/下载 API**
   - `GET /api/v1/skills/{id}/package` → 返回 ZIP（SKILL.md + scripts/ + references/ + assets/ + src/）
   - ZIP 内目录名 = skill name，符合 Agent Skills 规范
   - 对象存储（MinIO/S3）存放 ZIP，DB 记录 `skill_package_path`

10. **Skill 导入 API**
    - `POST /api/v1/skills/import` — 接受 ZIP 上传或 Git URL
    - 解压/clone → 校验目录结构（必须含 SKILL.md）→ 解析 frontmatter → 存储
    - 返回导入报告（成功/校验问题/资源清单）

11. **CLI 制品打包/下载 API**
    - `GET /api/v1/snippets/{id}/package` → 返回 CLI Interface 描述文件（YAML frontmatter + Markdown body）

12. **CLI 导入 API**
    - `POST /api/v1/snippets/import` — 接受标准 CLI Interface 文件或 URL

**Phase 4：前端升级**

13. **Skill 创建/编辑表单升级**
    - 支持 runtime 选择（Python/Node/Wasm）
    - 支持 dependencies 编辑
    - 支持 SKILL.md 编辑器（带 frontmatter 高亮）
    - 支持 scripts/references/assets 文件管理（上传/预览）
    - 导入向导（ZIP 上传 / Git URL）

14. **CLI 创建/编辑表单升级**
    - 支持 command 输入
    - 支持 inputs schema 构建器（参数名、类型、必填、默认值、枚举值）
    - 支持 output schema 编辑
    - 支持 examples 编辑（command + input + output）
    - 支持 error_model 编辑（code + message + retryable）

#### 验收标准

- [ ] Skills 表包含 Agent Skills 规范所有标准字段的结构化列
- [ ] 创建 Skill 时 frontmatter 被解析并存入结构化字段（不再只存原文）
- [ ] Frontmatter 校验覆盖 name/description/license/compatibility 全部规则
- [ ] Skill 打包下载 API 返回符合 Agent Skills 规范的 ZIP
- [ ] 从 GitHub 导入 OpenAI skill-creator 和 cli-creator 成功
- [ ] Evolith 导出的 Skill 可被 Claude Codex / OpenAI Codex 正确识别
- [ ] CLI 数据模型包含 command、inputs、output、error_model、examples 结构化字段
- [ ] CLI 创建表单支持结构化命令定义（不只是代码编辑器）
- [ ] SQLite + PostgreSQL 双轨 migration 和 repository 同步更新
- [ ] `cargo test --workspace` 通过；`bun run build` 通过

- 依赖或阻塞：无（可与 EVO-046 并行推进 Phase 1/2）
- 影响范围：backend（domain model、DTO、service、handler、migration、storage）/ frontend（创建/编辑表单）/ docs（格式规范更新）
- 最小验证方式：导入 OpenAI skills 仓库中的 skill-creator 和 cli-creator 成功；导出 Skill 可被外部工具解析；全部测试通过
- 不做：不实现 Skill 执行环境迁移（EVO-046）；不实现 CLI 执行引擎（EVO-045）；不实现评分体系（EVO-050）

#### 依赖关系

```
EVO-049 Phase 1（数据模型）→ Phase 2（Parser 接线）→ Phase 3（打包/导入）→ Phase 4（前端）
                                                        ↗
                        EVO-046（Skill 制品化）           │
                        EVO-050（评分体系）────── 依赖 Phase 2 完成
```

## Validation Evidence Required

- Record real command output or manual evidence in the selected iteration before marking this item Done.
- Run checks matching the changed surface from [Testing SOP](../../sop/TESTING.md).

## Residual Work Destination

- Update this file and `docs/backlog/PRODUCT-BACKLOG.md` if scope, status, dependencies or Required Reads change.
- Move completed or deferred execution context to `docs/backlog/archive/<period>/` during backlog compaction.

## Source Snapshot

- Source: 用户反馈 2026-05-29
- Decision context: Epic；不直接进迭代，先执行 EVO-049-A/B 子 Story
