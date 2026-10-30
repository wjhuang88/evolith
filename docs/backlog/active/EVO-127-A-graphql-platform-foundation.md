# EVO-127-A — GraphQL platform/schema/codegen foundation

- **父 Epic**：[EVO-127](EVO-127-graphql-application-api-migration.md)
- **类型**：Technical / API Contract / Platform
- **优先级**：P0
- **状态**：Review / Iteration 070 / exact-head gates pending
- **GitHub Issue**：[#27](https://github.com/wjhuang88/evolith/issues/27)
- **依赖或阻塞**：ADR-0012 / EVO-127 governance baseline 已合并；START-ITERATION inventory 已完成
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

- [ ] Rust 1.94.1 `fmt/check/clippy/test --locked` 通过（focused bootstrap 已通过；正式 exact-head workspace CI 待 PR）。
- [x] Frontend codegen/type-check/lint 通过；build 与完整回归由正式 exact-head CI 再证明。
- [x] schema SDL 和 generated artifacts drift gate 已进入正式 `ci.yml`。
- [x] depth/complexity/batch abuse negative tests 已实现；bootstrap focused tests 通过。
- [x] cookie+CSRF 与 bearer-only request behavior 有真实 `/graphql` Actix integration tests；正式 workspace gate 待 PR。
- [x] GraphiQL 只允许 development 明确启用，production 默认关闭。
- [ ] `/api/v1` 现有业务路由在本 Story 保持行为不变（实现未改这些 routes；完整 regression 由 exact-head CI 证明）。

## 已落地证据

- `/graphql`：single-request only，batch HTTP 400 `GRAPHQL_BATCH_NOT_SUPPORTED`。
- 资源上限：body 256 KiB、depth 15、complexity 1000、directives 20。
- error extensions：`code` / `requestId` / `retryable`。
- credential boundary：global RBAC 将 `/graphql` 作为 optional-auth transport；专用 middleware 复用现有 JWT/API-key resolution，解析后再进入 caller-aware rate limiter。
- CSRF：只有携带 `evolith_token` session cookie 的 GraphQL POST 强制 double-submit；bearer-only/no-session-cookie 不依赖 browser CSRF。
- canonical SDL：`docs/reference/graphql/schema.graphql`。
- authored operation：`frontend/src/graphql/operations/`；generated client：`frontend/src/graphql/generated/`。
- frontend transport：same-origin `/graphql`、`credentials: include`、Authorization/CSRF/X-Request-ID；TanStack Query 继续作为 cache owner。
- typed smoke call：`frontend/src/graphql/foundation.ts` 使用 generated `GraphQlFoundationStatusDocument`。
- bootstrap run #6 (`40723836561`) success；bot generated-artifact commit `5b1f66a`。
- 临时 bootstrap workflow 已删除，永久 drift gate 已并入 `.github/workflows/ci.yml`。

## 最小验证

- backend GraphQL request integration tests；
- schema export deterministic check；
- frontend Codegen + TypeScript build；
- existing full CI/durability gates。

## 不做

业务域迁移、REST 删除、subscriptions、federation、cursor pagination redesign。

## 执行归口

- **Iteration**：[Iteration 070](../../iterations/ITERATION-070.md)
- **激活日期**：2026-10-28
- **当前阶段**：Navigator Review；Driver/focused gates 完成，等待 PR exact-head full gates 后才能进入 Done/closure。
