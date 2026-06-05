# EVO-050 Skill/CLI 评分与质量体系

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- [Skill format](../../reference/formats/SKILL-FORMAT.md)
- [CLI interface format](../../reference/formats/CLI-INTERFACE-FORMAT.md)

## Summary

| Field | Value |
| --- | --- |
| ID | EVO-050 |
| Type | feature |
| Status | Proposed |
| Priority | P1 |
| Parent Epic | None recorded |
| Source | 用户反馈 2026-05-29 |

## Problem Or Outcome

平台提供 Skill/CLI 组件的评分能力：用户评分、使用统计、质量评估，支撑生态发现和信任

## Goal And Non-Goals

- Goal: deliver the outcome described by the title and decision context without expanding scope during implementation.
- Non-goals: do not implement adjacent backlog items unless they are listed as hard dependencies in this file.

## Dependencies And Blockers

- No hard blocker recorded in the source backlog.

## Governing ADRs, Specs Or Decisions

- [Skill format](../../reference/formats/SKILL-FORMAT.md)
- [CLI interface format](../../reference/formats/CLI-INTERFACE-FORMAT.md)

## Acceptance Criteria

- 类型：feature
- 优先级：P1
- 状态：Proposed
- 用户价值或技术目标：平台提供 Skill/CLI 组件的评分和质量评估能力，支撑生态中的发现、信任和筛选。用户可对组件评分，平台自动评估质量指标。
- 核心设计：
  1. **用户评分**：1-5 星评分 + 可选文字评价，展示组件平均分和评价数量
  2. **使用统计**：下载量、安装量、调用频次，作为热门度排序信号
  3. **质量评估**（自动计算质量分数，0-100）：
     - **Frontmatter 完整性**（30 分）：必选字段 name+description 满分 20，每多一个可选字段（author/tags/license/compatibility）加 2.5
     - **Description 质量**（20 分）：非空 10 分，长度 > 50 字符 +5，同时包含"做什么"和"何时使用" +5
     - **资源丰富度**（30 分）：有 scripts/ +10，有 references/ +10，有 assets/ +5，有 examples +5
     - **结构合规**（20 分）：name 符合格式 +5，目录名匹配 +5，SKILL.md < 500 行 +5，文件引用正确 +5
  4. **信任信号**：是否经过验证、官方标记、安全扫描通过
  5. **排序/筛选**：列表页支持按评分、下载量、更新时间、质量分数排序
  6. **评分数据模型**：
     - `Rating`：id, user_id, component_id, component_type(skill/cli), score(1-5), comment, created_at
     - `ComponentStats`：component_id, component_type, avg_rating, rating_count, download_count, install_count, call_count, quality_score, updated_at
- 验收标准：
  - [ ] 组件详情页展示平均评分、评价数、下载量、质量分数
  - [ ] 用户可对已使用的组件提交 1-5 星评分和文字评价
  - [ ] 创建/导入时自动计算质量分数（frontmatter 完整性 + description 质量 + 资源丰富度 + 结构合规）
  - [ ] 列表页支持按评分/下载量/更新时间/质量分数排序
  - [ ] SQLite + PostgreSQL 双轨 migration
- 依赖或阻塞：EVO-049 Phase 2（Parser 接线后质量评估维度才准确）
- 影响范围：backend（新增 Rating/ComponentStats 模型、repository、handler）/ frontend（评分 UI、列表排序）/ db migration
- 最小验证方式：评分 API 创建和查询；质量评估对 OpenAI skill-creator 产出合理分数（预期 ≥ 80）
- 不做：不实现安全扫描（远期）；不实现付费推荐/置位（远期市场功能）

## Validation Evidence Required

- Record real command output or manual evidence in the selected iteration before marking this item Done.
- Run checks matching the changed surface from [Testing SOP](../../sop/TESTING.md).

## Residual Work Destination

- Update this file and `docs/backlog/PRODUCT-BACKLOG.md` if scope, status, dependencies or Required Reads change.
- Move completed or deferred execution context to `docs/backlog/archive/<period>/` during backlog compaction.

## Source Snapshot

- Source: 用户反馈 2026-05-29
- Decision context: 平台提供 Skill/CLI 组件的评分能力：用户评分、使用统计、质量评估，支撑生态发现和信任
