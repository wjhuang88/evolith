# EVO-100 Git-Centric Platform Foundation

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- [Git-Centric Platform Proposal](../../proposals/GIT-CENTRIC-PLATFORM.md)
- [ADR-0004 Git-Centric Storage](../../decisions/ADR-0004-git-centric-storage.md)
- [ADR-0005 Deprecate Sandbox Runtime](../../decisions/ADR-0005-deprecate-sandbox-runtime.md)

## Summary

- 类型：epic
- 优先级：P0
- 状态：Proposed
- 父 Epic：无
- Epic 完成条件：用户可在 Evolith 创建 git repo、push/pull 代码；与外部 agent engine 集成完成 vibe coding；skill/CLI/MCP 通过文件 pattern 自动索引；sandbox 执行层全部删除，`cargo test --workspace` 全绿

## Problem Or Outcome

将 Evolith 从"DB-centric skill/CLI/MCP registry"重构为"git 托管 + Pages 式衍生能力 + vibe coding 平台"。本 Epic 涵盖后端基础与索引层；前端 UX 单独走 EVO-104。

## Goal And Non-Goals

- **Goal**：完成 git 托管服务（Phase 1）、agent 集成 surface（Phase 2）、skill/CLI/MCP indexer（Phase 3）、sandbox 废弃（Phase 4）的全部后端 Stories。
- **Non-goals**：
  - 不实现 Vibe Coding Web UI（独立 Story EVO-104）。
  - 不实现 SSH / LFS（Phase 5 扩展）。
  - 不构建 LLM loop / agent engine（用户另一项目提供）。
  - 不实现资源级 ACL（Phase 5+）。
  - 不实现对外部 git 托管（GitHub/GitLab）的代理（Phase 6+ 评估）。

## 拆分理由

整个方向调整涉及多个独立验收的工程结果（git service、policy 规范、smart HTTP、commit API、agent session、webhook、indexer、双写适配、sandbox 清理），跨越多层（backend / db / frontend API 契约 / deploy），明显超过 2 天交付窗口，必须按 Epic 拆分。

## 子 Story

| 子 Story | 独立结果 | 状态 | 依赖 | 所属迭代 |
|----------|----------|------|------|----------|
| [EVO-101](EVO-101-git-repos-schema.md) | `git_repos` 表 + 双轨 migration + tenant 创建仓默认能力 | Proposed | 无 | Phase 1 |
| [EVO-102](EVO-102-evolith-policy-yaml.md) | `.evolith/policy.yaml` 规范文档 + 解析器 + 默认值 fallback | Proposed | 无 | Phase 1 |
| [EVO-103](EVO-103-repo-context-and-smart-http.md) | Repo CRUD + Smart HTTP（`info/refs` + `git-upload-pack` + 临时 `git-receive-pack` subprocess）+ Repo Context API（file-tree / blobs / commits / diff） | Proposed | EVO-101 | Phase 1 |
| [EVO-105](EVO-105-commit-and-promote-api.md) | `POST /repos/{id}/commits`（policy 评估 + 三态）+ `POST /repos/{id}/promote` + agent branch 自动命名 | Proposed | EVO-102, EVO-103 | Phase 2 |
| [EVO-106](EVO-106-agent-session-and-scoped-token.md) | `agent_sessions` 表 + `POST /agent-sessions` + scoped token + 权限模型 | Proposed | EVO-103 | Phase 2 |
| [EVO-107](EVO-107-webhook-out.md) | `webhook_deliveries` 表 + push/promote 事件触发 POST + retry 指数退避 + 失败记录 | Proposed | EVO-105, EVO-106 | Phase 2 |
| [EVO-108](EVO-108-skill-cli-mcp-indexer.md) | Indexer 服务监听 git push 事件 → 扫描 `SKILL.md`/`interface.yaml`/`tool.yaml` → 解析 frontmatter → 更新 `skill_index`/`cli_index`/`mcp_tool_index` | Proposed | EVO-103 | Phase 3 |
| [EVO-109](EVO-109-discovery-api-and-pages-ui.md) | `GET /skills?q=`/`GET /cli-interfaces?q=`/`GET /mcp-tools?q=` 跨仓搜索 + Repo 详情页 "此仓包含 X skill/Y CLI/Z MCP tool" 标签页 | Proposed | EVO-108 | Phase 3 |
| [EVO-110](EVO-110-old-table-dual-write.md) | 旧 `skills`/`snippets`/`tools` 表双写适配（写入路径收敛到 `POST /repos/{id}/files`，由 indexer 同步双写）；保留 `GET /skills/{id}` 等 API 兼容 | Proposed | EVO-108 | Phase 3 |
| [EVO-111](EVO-111-deprecate-sandbox-runtime.md) | 删除 service-skill 执行层 + bollard 依赖 + sandbox 镜像 + `SANDBOX__*` 配置 + `POST /skills/{id}/execute`；`ExecutionProvider` 简化为仅 HttpProxy | Proposed | 无（最后执行） | Phase 4 |

## Dependencies And Blockers

- 依赖 ADR-0004 + ADR-0005 决策生效。
- 依赖 `gix` crate 与 `bollard` 升级链路（参考 EVO-061~076 全量依赖迁移，2026-06-06 已完成）。
- EVO-103 smart HTTP push 阶段临时依赖 `git` CLI subprocess；`gix-push` 合入后移除（独立 EVO 跟踪）。
- EVO-105 / EVO-106 / EVO-107 软依赖 EVO-104 UX 决策 U-04（agent branch 命名）。

## Acceptance Criteria（Epic 父项）

- [ ] Phase 1 完成：用户可在 Web UI 创建 repo；`git clone http://.../repos/{id}` 完整可用；`.evolith/policy.yaml` 解析生效
- [ ] Phase 2 完成：vibe coding 三栏 UI MVP 可用；agent session 创建/完成/审计全链路贯通
- [ ] Phase 3 完成：push 一个含 `SKILL.md` 的 commit → `skill_index` 在 5s 内更新；`GET /skills?q=` 跨仓搜索可用；旧 API 兼容
- [ ] Phase 4 完成：`cargo test --workspace` 全绿；`cargo clippy --workspace --all-targets -- -D warnings` 0 errors；前端 bundle 中无 sandbox 相关依赖
- [ ] 文档同步：`PRODUCT-BACKLOG.md` / `IMPLEMENTATION-ROADMAP.md` / `EVOLUTION.md` / `BOARD.md` / `docs/README.md` 反映新方向；旧项 Dropped/Superseded 标注完整

## Validation Evidence Required

- 每个子 Story 收口记录到对应 `docs/iterations/ITERATION-NNN.md`。
- Phase 1~4 收口记录到 EVOLUTION.md（重大决策与残余归口）。
- 外部 agent engine 联调 end-to-end 截图（mock agent 也可）。

## Residual Work Destination

- 子 Story Dropped 时同步本文件子项表；Epic 完成条件可能变化需记录。
- Phase 5+ 扩展项（SSH / LFS / 跨仓搜索 / 资源级 ACL）独立评估后另开 EVO。
- 与本 Epic 相关但未列入的子议题（如 git hooks / protected branches）另开 EVO。

## Source Snapshot

- Source: 用户反馈 2026-06-23（开发目标变更）
- Decision context: 从 "Skill Registry" 转向 "Git 托管 + Vibe Coding 平台"；skill/CLI/MCP 是 Pages 式衍生能力；详见 `docs/proposals/GIT-CENTRIC-PLATFORM.md`
- Prior discussion: 2026-06-23 多轮分析确认 MVP 范围（仓库浏览 + 直推直合，无 PR/MR UI）+ 仓库模型（每 skill/cli/mcp 独立小仓已弃用，改为通用 git 托管 + 文件 pattern 索引）
