# EVO-029 Skill 专业描述与发现质量提升

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)

## Summary

| Field | Value |
| --- | --- |
| ID | EVO-029 |
| Type | feature |
| Status | Proposed |
| Priority | P2 |
| Parent Epic | None recorded |
| Source | Agent Skills spec |

## Problem Or Outcome

提升 description、触发关键词、兼容性、资源索引和搜索排序质量

## Goal And Non-Goals

- Goal: deliver the outcome described by the title and decision context without expanding scope during implementation.
- Non-goals: do not implement adjacent backlog items unless they are listed as hard dependencies in this file.

## Dependencies And Blockers

- No hard blocker recorded in the source backlog.

## Governing ADRs, Specs Or Decisions

- None recorded beyond the product backlog and item file.

## Acceptance Criteria

- 类型：feature
- 优先级：P2
- 状态：Proposed
- 用户价值或技术目标：让技能描述更适合智能体自动发现和选择，提升企业知识资产的可检索性。
- 验收标准：
  - [ ] 创建和导入时对 description 给出专业度评分或检查项：能力、触发场景、关键词、边界条件。
  - [ ] 列表和搜索优先使用 name、description、tags、compatibility 和 metadata 中的发现信号。
  - [ ] 前端新建页提供符合 Agent Skills 建议的描述模板，不再使用泛泛示例。
  - [ ] 低质量描述不会阻塞保存，但必须在导入报告或编辑页提示。
- 依赖或阻塞：EVO-028 的校验报告模型。
- 影响范围：backend / frontend / docs
- 最小验证方式：描述质量检查单测；前端新建页文案检查；搜索结果基本回归。

## Validation Evidence Required

- Record real command output or manual evidence in the selected iteration before marking this item Done.
- Run checks matching the changed surface from [Testing SOP](../../sop/TESTING.md).

## Residual Work Destination

- Update this file and `docs/backlog/PRODUCT-BACKLOG.md` if scope, status, dependencies or Required Reads change.
- Move completed or deferred execution context to `docs/backlog/archive/<period>/` during backlog compaction.

## Source Snapshot

- Source: Agent Skills spec
- Decision context: 提升 description、触发关键词、兼容性、资源索引和搜索排序质量
