# EVO-049-B Skill/CLI parser 接线与校验报告

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- [Skill format](../../reference/formats/SKILL-FORMAT.md)
- [CLI interface format](../../reference/formats/CLI-INTERFACE-FORMAT.md)

## Summary

| Field | Value |
| --- | --- |
| ID | EVO-049-B |
| Type | feature |
| Status | Blocked |
| Priority | P0 |
| Parent Epic | EVO-049 |
| Source | EVO-049 split |

## Problem Or Outcome

依赖 EVO-049-A；将 parser 接入创建/更新并输出 blocking error / warning

## Goal And Non-Goals

- Goal: deliver the outcome described by the title and decision context without expanding scope during implementation.
- Non-goals: do not implement adjacent backlog items unless they are listed as hard dependencies in this file.

## Dependencies And Blockers

- 依赖 EVO-049-A；将 parser 接入创建/更新并输出 blocking error / warning
- EVO-049-A must remain Done before activation.

## Governing ADRs, Specs Or Decisions

- [Skill format](../../reference/formats/SKILL-FORMAT.md)
- [CLI interface format](../../reference/formats/CLI-INTERFACE-FORMAT.md)

## Acceptance Criteria

- 类型：feature
- 优先级：P0
- 状态：Blocked
- 父 Epic：EVO-049
- Story 形态：API / Technical
- 用户价值或技术目标：
  - 作为/为了：平台维护者和企业用户创建 Skill/CLI 组件时，需要及时得到结构兼容性反馈。
  - 我希望/需要：create/update/import 路径调用现有 parser，写入结构化字段，并返回 blocking error / warning 校验报告。
  - 以便：不合规组件不会静默进入可用状态，后续制品化和评分体系可复用同一报告。
- 范围：
  - `create_skill` / `update_skill` 接线 `SkillParser`，把 frontmatter 映射到 EVO-049-A 的结构化字段。
  - `create_snippet` / CLI interface update 路径接线 `CliInterfaceParser`。
  - 建立校验报告结构，区分 blocking error 与 warning。
  - 更新 API contract 和最小前端展示边界。
- 不做：
  - 不实现制品打包下载、ZIP/Git 导入或对象存储。
  - 不实现质量评分排序；归 EVO-050。
  - 不实现执行引擎；归 EVO-045。
- 验收标准：
  - [ ] 创建和更新 Skill 时 parser 被调用，结构化字段来自 frontmatter 而不是硬编码默认。
  - [ ] CLI interface 创建/更新能解析 command、inputs、output、examples、error_model。
  - [ ] 校验报告能返回 blocking error 和 warning，并在 API contract 中定义。
  - [ ] 不合规 name/description 返回明确错误；低质量描述只给 warning。
  - [ ] parser、handler 和 API 测试覆盖合法、非法、warning 样例。
- 依赖或阻塞：EVO-049-A 完成并验证。
- 解锁内容：EVO-049 后续制品打包/导入；EVO-050 质量评分；EVO-045 执行引擎。
- 影响范围：backend / frontend / docs
- 最小验证方式：`cargo test -p service-skill`；`cargo test -p service-snippet`；`cargo test -p api`；`cargo test --workspace`；`bun run type-check`（如前端有最小展示）。

## Validation Evidence Required

- Record real command output or manual evidence in the selected iteration before marking this item Done.
- Run checks matching the changed surface from [Testing SOP](../../sop/TESTING.md).

## Residual Work Destination

- Update this file and `docs/backlog/PRODUCT-BACKLOG.md` if scope, status, dependencies or Required Reads change.
- Move completed or deferred execution context to `docs/backlog/archive/<period>/` during backlog compaction.

## Source Snapshot

- Source: EVO-049 split
- Decision context: 依赖 EVO-049-A；将 parser 接入创建/更新并输出 blocking error / warning
