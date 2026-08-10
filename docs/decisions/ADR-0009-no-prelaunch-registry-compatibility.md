# ADR-0009: No Pre-launch Registry Compatibility Layer

## 状态

Accepted（2026-08-08）

部分替代 [ADR-0004](ADR-0004-git-centric-storage.md) 中“旧表作为 materialized cache 并由 Indexer 双写”的迁移策略；不改变 Git 是代码与能力事实源的核心决策。

## 背景

Evolith 尚未上线，没有外部用户、已发布 UI 路由或必须保留的 Registry 数据合约。现有 EVO-110 计划让 Indexer 同时写 `skill_index` / `cli_index` / `mcp_tool_index` 和旧 `skills` / `snippets` / `tools` 表，并建设数据回填与旧 API 回归。

这会永久增加两套写模型的一致性、恢复和测试成本，却不增加最终产品价值。最终产品以 Repo、Ref、Path 和 Commit 作为能力 provenance；旧 DB 行不能继续作为第二事实源。

## 选项

1. 实施 EVO-110 双写、回填和旧 API 兼容，再在未来迁移删除。
2. 旧表只读冻结，新能力写 Git，但长期保留旧 API fallback。
3. 取消双写与迁移兼容；先让 Repo-derived read/execute 合约承接能力，再直接删除旧 UI、API、domain/repository 和表。

## 决策

选择选项 3：

- EVO-110 Dropped，不实现旧表双写、回填脚本、旧 API 行为保持或 deprecation 窗口。
- Repo-derived capability 的稳定身份是 `tenant + repo + ref/commit + path + type`，不是旧表 UUID。
- Discovery/Resources 由 EVO-108/109 的 index/read model 提供。
- MCP Tool 若需要执行，必须从 Repo manifest/index 解析目标与权限，并复用 EVO-118-B Typed Capability 和 EVO-118-C Egress Policy；不得为了复用旧执行路径继续要求 `tools` 行。
- Legacy UI 由 EVO-121-F 删除；Legacy backend 由 EVO-122 分阶段删除。
- 删除表必须使用 SQLite/PostgreSQL 配对 migration，先证明所有 runtime consumer 已切换，并执行跨租户、权限与失败恢复验证。
- 项目开发数据不做产品级迁移向导。若删除前发现真实外部消费者或必须保留的数据，暂停 EVO-122，重新建立 ADR/Story；不能静默恢复双写。

## 约束分类

| 类型 | 结论 |
| --- | --- |
| Hard | Git 是代码/能力事实源；SQLite/PostgreSQL 双数据库；MCP execute 与 tenant/RBAC/Egress 安全边界必须保留 |
| Soft | 未上线阶段不承诺旧 UI/API 兼容；可直接删除内部旧路径 |
| Assumption | 当前无真实外部 consumer 或不可丢弃 Registry 数据；EVO-122 启动时用调用方/数据盘点验证 |

## 后果

### 正向

- 消除长期双写、回填、漂移校验和第二事实源。
- 资源身份与 UI provenance 使用同一 Git 证据模型。
- 开发投入直接服务最终产品，不建设一次性迁移体验。

### 代价与风险

- 不能直接删除旧表：MCP/Skill runtime consumer 必须先切换。
- 清理跨 API、domain、repository、migration 和测试，需要分阶段 Navigator 审查。
- 本地旧开发数据可能被删除；执行前需明确环境与可恢复性，禁止作用于未盘点的生产数据。

## 相关链接

- [ADR-0004 Git-centric Storage](ADR-0004-git-centric-storage.md)
- [ADR-0008 Repo-centric Interaction](ADR-0008-repo-centric-interaction-architecture.md)
- [EVO-110 Dropped dual-write plan](../backlog/archive/2026-Q3/EVO-110-old-table-dual-write.md)
- [EVO-121-F Legacy UI removal](../backlog/active/EVO-121-F-retire-legacy-registry-ui.md)
- [EVO-122 Retire Registry Backend](../backlog/active/EVO-122-retire-prelaunch-registry-backend.md)
