# EVO-102 `.evolith/policy.yaml` 规范与解析器

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- [Git-Centric Platform Proposal](../../proposals/GIT-CENTRIC-PLATFORM.md)
- 父 Epic: [EVO-100](EVO-100-git-centric-platform-foundation.md)

## Summary

- 类型：feature / governance
- 优先级：P1
- 状态：Proposed
- 父 Epic: EVO-100

## Problem Or Outcome

定义每仓库 `.evolith/policy.yaml` 的规范格式 + Rust 解析器，承载直推直合三态（auto_merge / require_review / block）、protected_paths 与 agent scope 配置。提供 typed struct 给后续 EVO-105 commit policy 评估使用。

## Goal And Non-Goals

- **Goal**：
  1. 写 `.evolith/policy.yaml` 规范文档（独立 `docs/reference/formats/POLICY-YAML-FORMAT.md` 或类似位置）
  2. 实现 Rust 解析器（`serde_norway` 或 `serde_yaml`；参考 EVO-076 已迁至 serde_norway）
  3. 与 `git_repos.auto_merge` / `require_review` 默认值合并（policy 优先，缺失字段用 DB 默认）
- **Non-goals**：
  - 不实现 policy 评估逻辑（EVO-105 范围）
  - 不实现 policy 远程拉取 / 链式继承
  - 不实现 policy 变更审计
  - 不实现 policy 可视化编辑 UI（EVO-104 范围）

## Dependencies And Blockers

- 无硬依赖；可与 EVO-103 同迭代推进
- 间接依赖 `serde_norway` 已升级（EVO-076 Done）

## Governing ADRs, Specs Or Decisions

- [ADR-0004 Git-Centric Storage](../../decisions/ADR-0004-git-centric-storage.md)

## Acceptance Criteria

- [ ] 规范文档定义完整 schema：`version`、`default_action`、`protected_paths[]`、`agents[]`（含 `name`、`scopes[]`、`auto_merge`）
- [ ] `default_action` 枚举：`auto_merge` | `require_review` | `block`
- [ ] `scopes` 枚举：`read` | `commit:<path-glob>` | `promote`
- [ ] `path-glob` 支持 `*`、`**`、`?`（与 gitignore 类似语义）
- [ ] Rust 解析器实现为 `EvolithPolicy::parse(&str) -> Result<EvolithPolicy, AppError>`
- [ ] 缺失 `policy.yaml` 时使用 `git_repos.auto_merge` / `require_review` 默认值（fallback 逻辑测试覆盖）
- [ ] 单元测试覆盖：3 种 default_action、protected_paths 命中/未命中、多 agent scope、glob 边界
- [ ] 规范文档给出 SKILL.md / CLI/MCP 文件作为 protected_paths 的推荐配置
- [ ] 规范文档包含向后兼容策略（`version` 字段缺失时按 v1 处理）

## Validation Evidence Required

- `cargo test -p domain` 或新建 policy 模块测试通过
- 规范文档链接到本 item file Required Reads
- 至少 10 个单元测试用例覆盖 3 种 action × 多种 scope 组合

## Residual Work Destination

- 远程 policy（从一个 repo 引用另一个）的支持后续 EVO 评估
- policy 变更通知 / 审计后续 EVO 评估
- 可视化编辑 UI 在 EVO-104 Vibe Coding UI 中实现

## Source Snapshot

- Source: 用户反馈 2026-06-23
- Decision context: `.evolith/policy.yaml` 是 repo 级别策略文件，类似 GitHub Pages 的 `Settings → Pages → Source` 配置
- 草案示例：
  ```yaml
  version: 1
  default_action: require_review
  protected_paths:
    - SKILL.md
    - cli/interface.yaml
    - mcp/tool.yaml
  agents:
    - name: "code-reviewer"
      scopes: ["read", "commit:src/"]
      auto_merge: true
  ```
