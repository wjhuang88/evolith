# EVO-127-A — GraphQL platform/schema/codegen foundation

- **父 Epic**：[EVO-127](EVO-127-graphql-application-api-migration.md)
- **类型**：Technical / API Contract / Platform
- **优先级**：P0
- **状态**：Ready after current iteration inventory closure
- **GitHub Issue**：[#27](https://github.com/wjhuang88/evolith/issues/27)
- **依赖或阻塞**：ADR-0012 / EVO-127 governance baseline merged；当前非终态 Iteration 必须先按 START-ITERATION 处置
- **解锁内容**：EVO-127-B/C/F；为 D/E 提供 GraphQL 基础
- **影响范围**：backend / frontend / docs / CI

## 工程目标

建立单一 `/graphql` application endpoint、schema/error/security foundation、committed SDL 与 frontend codegen/typed transport，使后续各业务域可以按 vertical slice 迁移；本 Story 不做大规模 domain cutover。

## Scope

- Backend 引入 `async-graphql` + `async-graphql-actix-web`，Actix 继续作为 server。
- Root Query/Mutation、request context、stable error extensions。
- depth/complexity limits、batch disabled、request/rate-limit/security integration。
- browser cookie + CSRF / bearer-only auth transport contract。
- committed schema SDL + deterministic export/drift check。
- Frontend 引入 `graphql-request`、GraphQL Code Generator client preset；继续使用 TanStack Query。
- typed GraphQL transport wrapper：credentials/auth/CSRF/request-id。
- 只提供最小 schema smoke operation，不迁移完整业务域。

## Acceptance

- [ ] Rust 1.94.1 `fmt/check/clippy/test --locked` 通过。
- [ ] Frontend codegen/type-check/build/lint 通过。
- [ ] schema SDL 和 generated artifacts drift gate 可重复失败/通过。
- [ ] depth/complexity/batch abuse negative tests 通过。
- [ ] cookie+CSRF 与 bearer-only integration tests 通过。
- [ ] GraphiQL 只允许 development 明确启用，production 默认关闭。
- [ ] `/api/v1` 现有业务路由在本 Story 保持行为不变。

## 最小验证

- backend GraphQL request integration tests；
- schema export deterministic check；
- frontend Codegen + TypeScript build；
- existing full CI/durability gates。

## 不做

业务域迁移、REST 删除、subscriptions、federation、cursor pagination redesign。
