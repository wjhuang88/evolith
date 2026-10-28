# EVO-127-B — Auth/session GraphQL migration

- **父 Epic**：[EVO-127](EVO-127-graphql-application-api-migration.md)
- **类型**：Product/API Security Story
- **优先级**：P0
- **状态**：Proposed
- **GitHub Issue**：[#28](https://github.com/wjhuang88/evolith/issues/28)
- **依赖或阻塞**：EVO-127-A
- **影响范围**：backend / frontend / security / docs

## 目标

把 login/register/refresh/logout/password/email/invitation 等浏览器 auth/session 交互迁到 GraphQL，保持 cookie/JWT/CSRF/Bearer 语义。

## Acceptance

- Browser login/register/refresh/logout E2E 通过；失败 refresh 不循环。
- Public auth mutation 无既有 session cookie 时可用；cookie session 请求维持 CSRF。
- stable auth error codes 进入 GraphQL `extensions.code`。
- frontend auth flows 不再调用 `/api/v1/auth/*`。

## 不做

Tenant admin、repo、MCP/protocol endpoints。
