# EVO-127-E — Repo Context GraphQL migration

- **父 Epic**：[EVO-127](EVO-127-graphql-application-api-migration.md)
- **类型**：Product/API Story
- **优先级**：P0
- **状态**：Proposed
- **GitHub Issue**：[#31](https://github.com/wjhuang88/evolith/issues/31)
- **依赖或阻塞**：EVO-127-A + EVO-126-E
- **影响范围**：backend / frontend / service-git / docs

## 目标

迁移 tree/blob/commit/diff Repo Context application reads 到 GraphQL，并让 Repo Detail/Commit Evidence 使用 generated operations。

## Acceptance

- tree/blob/commit/diff representative parity tests 通过。
- `INVALID_INPUT` / `RESOURCE_EXCEEDED` / `TIMEOUT` 仍可通过 GraphQL error extensions 区分。
- tenant/repo authorization 保持。
- list/detail resolver 不产生未受控 N+1/reopen loop。
- frontend Repo Context REST calls 为零。

## 不做

Git fetch/push protocol；Repo Context engine cutover 仍归 EVO-126-E。
