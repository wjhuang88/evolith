# EVO-127-F — Tools/Skills/Snippets GraphQL migration

- **父 Epic**：[EVO-127](EVO-127-graphql-application-api-migration.md)
- **类型**：Product/API Story
- **优先级**：P1
- **状态**：Proposed
- **GitHub Issue**：[#32](https://github.com/wjhuang88/evolith/issues/32)
- **依赖或阻塞**：EVO-127-A
- **影响范围**：backend / frontend / docs

## 目标

迁移 Tools、Skills、Snippets/CLI Interface application APIs 到 GraphQL，使用 generated operations/types，MCP transport 保持不变。

## Acceptance

- capability read/CRUD GraphQL parity tests 通过。
- filters/pagination 和授权语义保持。
- frontend capability/discovery consumers 不再使用 handwritten REST DTOs。
- `/mcp` API-key/security contract 无变化。

## 不做

MCP transport、capability model、sandbox/runtime redesign。
