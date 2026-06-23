# EVO-105 Commit API + 直推直合 + Promote API

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- [Git-Centric Platform Proposal](../../proposals/GIT-CENTRIC-PLATFORM.md)
- 父 Epic: [EVO-100](EVO-100-git-centric-platform-foundation.md)
- 依赖: [EVO-102](EVO-102-evolith-policy-yaml.md), [EVO-103](EVO-103-repo-context-and-smart-http.md)

## Summary

- 类型：feature / api
- 优先级：P0
- 状态：Proposed
- 父 Epic: EVO-100

## Problem Or Outcome

提供 `POST /repos/{id}/commits` 让用户/agent 通过 API 提交 commit，自动应用 `.evolith/policy.yaml` 直推直合策略；提供 `POST /repos/{id}/promote` 让用户手动合并分支到 main。形成"agent commit → policy 评估 → auto_merge 或 require_review"的闭环。

## Goal And Non-Goals

- **Goal**：
  1. Commit API：接收 `{branch, message, files: [{path, content, op}]}`；创建 commit
  2. Policy 评估：读取 `.evolith/policy.yaml`，按 default_action + protected_paths + agents 决定 commit 落点
  3. 三态执行：
     - `auto_merge`：直接更新 default_branch 并指回新 oid
     - `require_review`：commit 到分支 `agent/{session-id}/{feature}` 等待 promote
     - `block`：返回 4xx 拒绝
  4. Promote API：合并分支到 default_branch（fast-forward 或 merge commit）
- **Non-goals**：
  - 不实现 conflict 解决（Phase 5；U-08）
  - 不实现 3-way merge（Phase 5）
  - 不实现 protected branch 规则（Phase 5+）
  - 不实现 code review approval flow（Phase 5+）

## Dependencies And Blockers

- 依赖 EVO-102 policy 解析器
- 依赖 EVO-103 smart HTTP（commit 走 API 而非 git CLI）
- 软依赖 EVO-104 UX U-04 决策（agent branch 命名约定）

## Governing ADRs, Specs Or Decisions

- [ADR-0004 Git-Centric Storage](../../decisions/ADR-0004-git-centric-storage.md)
- 待补：policy 评估与 default_action 行为细节（写在 EVO-102 规范文档）

## Acceptance Criteria

- [ ] `POST /repos/{id}/commits` 实现：参数校验 → file ops 应用（write/modify/delete）→ policy 评估 → commit 创建
- [ ] 三态执行单元测试覆盖：
  - default_action=auto_merge → commit 到 default_branch 并指回
  - default_action=require_review → commit 到 `agent/{session-id}/{feature}` 分支
  - default_action=block → 返回 403，commit 拒绝
- [ ] protected_paths 命中测试：用户/agent 修改 `SKILL.md` 且 default_action=auto_merge 时强制 require_review
- [ ] agents[].scopes 评估：agent 修改未授权路径返回 403
- [ ] `POST /repos/{id}/promote` 实现：fast-forward 时直接指回；非 fast-forward 时创建 merge commit（默认策略）；返回新 default_branch oid
- [ ] agent branch 自动命名遵循 `agent/{session_id}/{feature_slug}`（slug 由 LLM 或用户提供）
- [ ] audit log 记录：commit / promote / block 全部事件，含 actor (user_id 或 session_id)、policy 决策、commit oid
- [ ] `cargo test --workspace` 与 `cargo clippy --workspace --all-targets -- -D warnings` 全绿

## Validation Evidence Required

- E2E 测试：mock agent commit → policy auto_merge → commit 在 main；mock agent commit → policy require_review → commit 在分支等待 promote
- 单元测试覆盖 3 种 action × 多 protected_path × 多 agent scope 组合（至少 15 个用例）
- audit log 记录可查询（admin endpoint 或 DB 查询）

## Residual Work Destination

- 3-way merge（Phase 5）
- protected branches（Phase 5+）
- code review approval（Phase 5+）
- EVO-104 UX U-05 三态视觉反馈是后端责任，UI 实现走 EVO-104

## Source Snapshot

- Source: 用户反馈 2026-06-23
- Decision context: 直推直合 + Pages 式 agent branch 是 vibe coding MVP 核心
- 心智模型：类比 GitHub PR 但无 UI；agent commit + policy 自动判定落点
