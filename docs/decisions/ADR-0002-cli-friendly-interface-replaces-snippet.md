# ADR-0002: CLI 友好接口替代 Snippet

## 状态

Accepted

## 背景

原 `snippet` 概念偏向代码片段仓库，适合沉淀示例代码和文档引用。但当前大模型工程实践更常见的交付形态，是提供可被模型发现、理解和调用的 CLI 友好接口：稳定命令、机器可读 schema、示例、错误语义和可组合的本地/远程执行入口。

## 决策

停止继续扩展旧 snippet 作为独立主线，后续用 “CLI 友好接口” 替代。

暂定英文命名：`CliInterface`。

该能力关注：

- 命令名称、子命令和参数 schema。
- 面向 LLM 的 usage、examples、error model。
- 输入输出契约，优先 JSON-friendly。
- 与 Rust CLI 子项目复用同一接口描述。
- 可由前端管理，也可由 CLI push/pull/sync。

## 影响

- 现有 snippet API、前端页面和文档需要迁移或废弃。
- `service-snippet` 后续应重命名或演进为 `service-interface` / `service-cli-interface`。
- 原 snippet format 文档进入迁移参考，不再作为新功能目标。
- Backlog 中 snippet update/reference 相关故事转为 Deferred，由 EVO-017 接管。

## 后续任务

1. 定义 `CliInterface` 领域模型和格式文档。
2. 设计 snippet 数据迁移策略：drop / archive / transform。
3. 更新 API 合约和前端导航。
4. 评估是否需要数据库 migration 重命名表，或先兼容旧表新语义。
5. 与 Rust CLI 规划对齐 push/pull/sync 格式。

## 相关链接

- [Product Backlog EVO-017](../backlog/PRODUCT-BACKLOG.md)
- [Rust CLI 规划](../planned/RUST-CLI.md)
