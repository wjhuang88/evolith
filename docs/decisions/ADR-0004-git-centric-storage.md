# ADR-0004: 内容存储从 DB 列迁移到 Git 仓库文件

> **2026-09-10 amendment**：本 ADR 的核心决策继续有效：Git 是代码、版本历史和 Repo 内能力描述的事实源；PostgreSQL/SQLite 保存身份、权限、Repo metadata、索引和事件状态。其“durable Git repository 必须位于 Evolith 应用 persistent filesystem”的实现后果已由 [ADR-0011](ADR-0011-walgit-backed-git-data-plane.md) 更新为 WalGit-backed object-store/WAL data plane。  
> 2026-08-08 amendment：旧表 materialized cache / Indexer 双写迁移策略已被 [ADR-0009](ADR-0009-no-prelaunch-registry-compatibility.md) 部分替代。

## 状态

Accepted / amended by ADR-0009 and ADR-0011

## 背景

Evolith 早期将 skill / cli interface / mcp tool 的内容存储在 PostgreSQL/SQLite 的 TEXT/JSONB 列中：

- `skills.skill_md` TEXT；
- `snippets.content` + `snippets.code` TEXT；
- `tools.input_schema` + `tools.output_schema` JSONB。

这种 DB-centric 模型不符合 Git hosting + Agent/Vibe Coding + Repo-derived capability 的产品方向：

1. 代码与能力应处于同一个 repository/version boundary；
2. Git 原生提供 commit/diff/branch/revert；
3. 一次 commit 可以原子表达跨多个 Skill/MCP/CLI 文件的变更；
4. Agent 应在 Repo context 中操作，而不是绕过 Git 修改数据库内容列。

## 原选项

| 选项 | 优点 | 缺点 |
|------|------|------|
| A. 保持 DB 列存储 | 改动小 | 与 Git-centric 产品方向冲突，无原生版本历史 |
| B. 内容迁移到 Git；DB 保留 metadata/index | Git 成为版本事实源；可派生索引 | 需要 Git service、Indexer 和迁移/一致性边界 |
| C. 所有读取实时扫描 Git、完全无 DB 索引 | schema 简单 | 搜索/发现性能和可运营性差 |

## 决策

采用 **B 的核心方向**：

- `git_repos` 是第一类实体；
- 代码、Commit、Branch、Tag、`.evolith/*`、`SKILL.md`、`interface.yaml`、`tool.yaml` 等内容以 Git repository 为事实源；
- `skill_index` / `cli_index` / `mcp_tool_index` 从 Git 事件与 Repo 内容派生；
- PostgreSQL/SQLite 保存用户、租户、权限、Repo metadata、索引、Agent/Audit/Outbox 等状态；
- 不把 Git object/commit 内容重新搬回数据库作为新的 durable source of truth。

## 后续修订

### ADR-0009：取消未上线 Registry 双写兼容

项目尚未上线，不再建设旧 Registry materialized-cache 双写、回填或长期旧 API 兼容。Repo-derived read/execute 承接后直接删除 legacy runtime/table。

### ADR-0011：Git durable storage 演进为 WalGit-backed object store

ADR-0004 决定“**Git 是内容/历史事实源**”，但不要求这个 Git repository 永久绑定单机应用 filesystem。

2026-09-10 起长期目标为：

```text
Evolith control plane
  -> service-git v2
  -> WalGit Git/WAL/Store primitives
  -> object store = durable Git truth
  -> local bare repo/pack = disposable cache
```

因此：

- 当前 filesystem bare repo 在 EVO-126-H 前仍是 runtime fact；
- 旧 DATA-01/DATA-02 证明的是该 runtime 的单实例 durability/lifecycle 历史基线；
- EVO-126 新增 GIT-DP-01，负责 object-store/WAL migration、readiness、recovery、protocol/context parity 和 cutover；
- Git 的事实源地位不变，只改变 physical durability / serving architecture。

## 保留的不变量

- Skill/MCP/CLI 与 Repo 生命周期、Commit provenance 对齐。
- Agent 写入必须落成真实 Git change/commit/ref publication，不以 DB CRUD 冒充版本控制。
- Capability Index 是可重建派生数据，不与 Git content 争夺事实源。
- Repo rename/product metadata 与 physical Git namespace 应解耦；EVO-126 推荐用稳定 tenant/repository identity 映射 WalGit RepoId。
- 未来任何存储优化不得让 local cache 变成未经声明的第二事实源。

## 后果

### 正面

- Repo、代码、能力描述和 Agent change 共享统一 Git provenance。
- Object-store/WAL 演进可以解决节点与 durable Git data 的强耦合，并支持更清晰的多实例/Agent clone 方向。
- DB 保持擅长的身份、权限、索引、工作流和事件状态，不承担 Git object database。

### 风险

- Git data-plane 架构切换必须处理 existing repo migration、DB/Git 一致性、object-store availability、credentials、cost/retention 与 recovery。
- Indexer 失败仍可能造成派生索引 stale，需要 event + reconcile；不得把 stale index 当作 Git truth。
- 上游 WalGit pre-1.0，需要 exact pin 和 Evolith-owned adapter boundary。

## 相关链接

- [ADR-0011 WalGit-backed Git Data Plane](ADR-0011-walgit-backed-git-data-plane.md)
- [ADR-0009 No Pre-launch Registry Compatibility](ADR-0009-no-prelaunch-registry-compatibility.md)
- [EVO-126 WalGit Git Data Plane](../backlog/active/EVO-126-walgit-git-data-plane-refactor.md)
- [EVO-100 Git-Centric Platform Foundation](../backlog/active/EVO-100-git-centric-platform-foundation.md)
- [EVO-108 Indexer](../backlog/active/EVO-108-skill-cli-mcp-indexer.md)
- [Git-Centric Platform Proposal](../proposals/GIT-CENTRIC-PLATFORM.md)
- [Project Status Baseline](../reference/PROJECT-STATUS-BASELINE-2026-09-10.md)
