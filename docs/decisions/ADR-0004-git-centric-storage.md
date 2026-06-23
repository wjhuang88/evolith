# ADR-0004: 内容存储从 DB 列迁移到 Git 仓库文件

## 状态

Accepted

## 背景

Evolith 当前将 skill / cli interface / mcp tool 的内容存储在 PostgreSQL/SQLite 的 TEXT/JSONB 列中：

- `skills.skill_md` TEXT（SKILL.md 完整正文）
- `snippets.content` + `snippets.code` TEXT（CLI 元数据 + 代码）
- `tools.input_schema` + `tools.output_schema` JSONB（MCP tool schema）

这种存储模型在 prototype 阶段（Phase 1-7）便于快速实现，但与 2026-06-23 提出的方向调整不匹配：

1. 用户希望平台重心转向 git 托管；skill/mcp/cli 是 Pages 式衍生能力。
2. Vibe coding 场景下，用户和 agent 在 repo 上下文中编辑代码；DB 列存储破坏了"以仓库为单位"的边界。
3. 没有原生版本历史；`(name, version)` 唯一约束只能追踪单版本。
4. 跨资源原子性差；用户无法把"3 个 skill + 1 个 mcp tool"的变更作为一个 commit 提交。

`docs/proposals/GIT-CENTRIC-PLATFORM.md` 详述了完整方向。

## 选项

| 选项 | 优点 | 缺点 |
|------|------|------|
| **A. 保持 DB 列存储** | 改动小；API 兼容 | 与新方向冲突；无版本历史；迁移成本累积 |
| **B. DB 列迁移到 git 文件 + 指针列** | 与 git-centric 方向一致；版本历史原生；可迁移 | 需 schema 双轨迁移；读路径需 adapter；存量数据迁移风险 |
| **C. 完全废弃 DB 表，所有读取走 git scan** | schema 最干净 | API breaking；性能差（每次全仓 scan） |

## 决策

采用 **B**：DB content 列迁移到 git 仓库文件，DB 保留为 metadata 索引 + git 指针 + materialized cache。

具体设计：

- 新表 `git_repos` 作为第一类实体，无 `resource_type` / `resource_id` 字段（与 GitHub repo 模型对齐）。
- `skill_index` / `cli_index` / `mcp_tool_index` 表从 git push 事件中发现并索引 `SKILL.md` / `interface.yaml` / `tool.yaml` 文件。
- 旧 `skills` / `snippets` / `tools` 表保留为 materialized cache；写入路径收敛到 `POST /repos/{id}/files`；由 indexer 同步双写。
- 迁移期：DB 列保留 nullable，存量数据回填脚本（dump → commit → backfill pointers）。
- 迁移完成后：逐步降低对 DB 列的依赖，DB 列最终废弃（独立 EVO）。

## 后果

### 正面

- Skill/MCP/CLI 与 git repo 生命周期一致；agent 可在 repo 上下文中操作。
- 天然版本历史（git commits）；用户可 revert / diff / branch。
- 跨资源原子性：一次 commit 可同时修改 3 个 skill + 1 个 mcp tool。
- Vibe coding UX 自然支持（编辑器直接编辑 SKILL.md）。
- 与 GitHub Pages 类比心智模型清晰，用户理解成本低。

### 负面

- Schema 改动较大（3 张表加 6+ 列 + 新建 4 张表）。
- 写入路径从直接 CRUD 改为"写文件 + commit"，latency 略高。
- 双写期间需保证 DB 与 git 状态一致；indexer 必须可靠。
- `gix` 服务端能力尚未完全成熟（push 缺）；Phase 1 临时 git CLI subprocess。
- 旧 API（`GET /skills/{id}`）需通过 materialized cache 保持兼容。

### 缓解

- 双轨 schema + dual-read adapter 避免 API breaking。
- 回填脚本带 SHA-256 校验；dry-run 模式。
- Phase 4 评估是否升级为 DB view（消除 materialized cache）。

## 风险

详见 `GIT-CENTRIC-PLATFORM.md` §9。

关键风险：

- 存量数据迁移（mitigated by 回填脚本 + dry-run + SHA-256 校验）。
- Indexer 失效导致 DB cache 与 git 不一致（mitigated by git push 事件 + 定期全量 reconcile）。
- `gix` push 能力缺口（mitigated by 临时 git CLI subprocess + 长期等 `gix-push` 合并）。

## 相关链接

- [Git-Centric Platform Proposal](../proposals/GIT-CENTRIC-PLATFORM.md)
- [ADR-0005 Deprecate Sandbox Runtime](ADR-0005-deprecate-sandbox-runtime.md)
- [EVO-100 Epic: Git-Centric Platform Foundation](../backlog/active/EVO-100-git-centric-platform-foundation.md)
- [EVO-101 git_repos 表](../backlog/active/EVO-101-git-repos-表.md)
- [EVO-108 Indexer 服务](../backlog/active/EVO-108-skill-cli-mcp-indexer.md)
- [EVO-110 旧表双写适配](../backlog/active/EVO-110-旧表双写适配.md)
- [SERVERLESS-RUNTIME.md](../proposals/SERVERLESS-RUNTIME.md)（历史参考）
- [EVOLUTION.md](../../EVOLUTION.md)（2026-06-23 方向变更记录）
