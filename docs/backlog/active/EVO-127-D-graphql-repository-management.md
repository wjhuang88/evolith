# EVO-127-D — Repository management GraphQL migration

- **父 Epic**：[EVO-127](EVO-127-graphql-application-api-migration.md)
- **类型**：Product/API Story
- **优先级**：P0
- **状态**：Proposed
- **GitHub Issue**：[#30](https://github.com/wjhuang88/evolith/issues/30)
- **依赖或阻塞**：EVO-127-A + EVO-126-B
- **影响范围**：backend / frontend / service-git / docs

## 目标

在 `service-git v2` application boundary 稳定后迁移 repository list/get/create/delete 到 GraphQL，并让 Repo list/create/detail UI 停止调用 REST。

## Acceptance

- tenant isolation/not-found/forbidden negative tests 通过。
- create/delete lifecycle 与现有 compensation/reconcile 语义一致。
- GraphQL/API public types 不暴露 `walgit_*` types。
- frontend repo management REST calls 为零。

## 不做

Git Smart HTTP 保持协议原生；Repo Context 归 EVO-127-E；storage cutover 仍归 EVO-126。
