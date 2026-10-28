# EVO-127-G — Frontend full GraphQL cutover and REST business API retirement

- **父 Epic**：[EVO-127](EVO-127-graphql-application-api-migration.md)
- **类型**：Migration / Cleanup / API Contract Story
- **优先级**：P0
- **状态**：Proposed
- **GitHub Issue**：[#33](https://github.com/wjhuang88/evolith/issues/33)
- **依赖或阻塞**：EVO-127-B/C/D/E/F
- **影响范围**：backend / frontend / docs / CI

## 目标

完成 browser/application GraphQL-only cutover，证明 `/api/v1` business consumers 为零后删除 REST handlers、Axios business client 和过时 REST contract。

## Acceptance

- frontend business code/browser E2E 中 `/api/v1` calls 为零。
- backend `/api/v1` business route inventory 为零；任何剩余 endpoint 必须明确是 protocol/compatibility exception owner。
- schema/codegen drift gate 通过。
- auth/RBAC/tenant isolation/CSRF/rate-limit/audit regression 通过。
- Git Smart HTTP、MCP、health、callbacks 等 protocol-native endpoints 无回归。
- CI 防止重新引入 browser business REST client/path。

## 不做

不 GraphQL 化 protocol-native endpoints；subscriptions/federation 另立需求。
