# EVO-122 Retire Pre-launch Registry Backend

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- [ADR-0009 No Pre-launch Registry Compatibility](../../decisions/ADR-0009-no-prelaunch-registry-compatibility.md)
- [ADR-0004 Git-centric Storage](../../decisions/ADR-0004-git-centric-storage.md)
- [EVO-108 Indexer](EVO-108-skill-cli-mcp-indexer.md)
- [EVO-109 Discovery](EVO-109-discovery-api-and-pages-ui.md)
- [EVO-111 Sandbox Removal](EVO-111-deprecate-sandbox-runtime.md)
- [Security Review](../../sop/SECURITY-REVIEW.md)
- [Database Migration](../../sop/DATABASE-MIGRATION.md)

## Summary

| Field | Value |
| --- | --- |
| ID | EVO-122 |
| Type | epic / backend + db |
| Status | Proposed / paused |
| Priority | P1 |
| Parent Epic | EVO-100 |
| Source | ADR-0009 replacement for Dropped EVO-110 |

## Problem Or Outcome

取消 EVO-110 双写后，旧 `tools` / `skills` / `snippets` 仍被 API、domain、repository 和部分执行路径引用。直接删表会破坏 MCP execute 和运行时；继续保留则让“Git 是事实源”只停留在文档。

本 Epic 先建立 Repo-derived 执行合约，再删除 legacy surface，最后通过双数据库 migration 删除旧表。

## Goal And Non-goals

- Goal：所有 capability 读取/执行都由 Repo + Ref/Commit + Path 解析；旧 CRUD/API/domain/repository/table 完全退出 runtime。
- Non-goals：不回填旧数据、不保持旧 UUID、不提供旧 API redirect/deprecation window、不建设双写。

## Child Stories

| 子 Story | 独立结果 | 状态 | 依赖 | 所属迭代 |
| --- | --- | --- | --- | --- |
| [EVO-122-A](EVO-122-A-repo-derived-mcp-execution.md) | Repo-derived MCP execution contract | Proposed / paused | EVO-108/109, EVO-118-B/C | - |
| [EVO-122-B](EVO-122-B-remove-legacy-registry-runtime.md) | 删除旧 CRUD/API/domain/repository runtime | Proposed / paused | EVO-122-A, EVO-111, EVO-121-F | - |
| [EVO-122-C](EVO-122-C-drop-legacy-registry-tables.md) | SQLite/PostgreSQL 删除旧表 | Proposed / paused | EVO-122-B | - |

## Threat And Failure Model

| 项目 | 内容 |
| --- | --- |
| 受保护资产 | Tenant Repo、Git manifest、MCP input/output、凭证、审计、现有 DB 数据 |
| 调用者 | Member/Admin/Owner、API Key、Agent Token、MCP client |
| 入口 | Repo-derived execute API、MCP tools/call、migration、worker/indexer |
| 信任边界 | Caller -> API -> Repo/index -> executor -> egress；DB migration -> runtime |
| 失败模式 | 跨 tenant manifest、Ref/path 越界、旧 consumer 漏删、删表后 runtime crash、Egress 回退、部分数据库迁移 |
| 安全默认 | 未解析/无权限/不一致即拒绝；旧 consumer inventory 非零时禁止 drop |
| 验证证据 | 权限负向矩阵、consumer inventory、SQLite/PostgreSQL migration、全量 tests、Navigator |

## Epic Completion Criteria

- [ ] 三个子 Story Done，EVO-110 保持 Dropped。
- [ ] API Contract 不再发布旧 Registry CRUD 为目标产品合约。
- [ ] runtime 搜索无 legacy Tool/Skill/Snippet repository consumer。
- [ ] SQLite/PostgreSQL fresh migration 与 upgrade migration 均不含旧表。
- [ ] Repo-derived MCP execute 通过 typed capability、tenant hiding、Ref/Path scope、Egress 和 audit 负向测试。

## Residual Work Destination

- 新 capability 类型或执行协议独立进入 EVO-108/109 后续 Story；不恢复通用旧 Registry。

## Source Snapshot

- EVO-110 历史方案保留在原 item，状态 Dropped。
- 当前代码事实：legacy routes/domain/repositories/table 仍存在，不能在文档切片中声称已删除。
