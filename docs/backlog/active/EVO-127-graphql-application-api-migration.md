# EVO-127 — GraphQL Application API Migration

- **类型**：Epic / Architecture / API Contract / Frontend Integration
- **优先级**：P0
- **状态**：Proposed / accepted direction
- **GitHub Issue**：[#26](https://github.com/wjhuang88/evolith/issues/26)
- **决策**：[ADR-0012](../../decisions/ADR-0012-graphql-application-api.md)
- **目标设计**：[GraphQL Application API Migration](../../design/GRAPHQL-APPLICATION-API-MIGRATION.md)
- **影响范围**：backend / frontend / docs / CI

## 目标

将 Evolith 的 browser/application business API 从 REST `/api/v1/*` 迁移为 GraphQL，并让前端使用生成类型的 GraphQL operations；最终删除不再有 consumer 的 REST business handlers 和 handwritten REST DTO/client 层。

GraphQL 不是“所有 HTTP 都统一成一个接口”。Git Smart HTTP、MCP、health/readiness、callbacks/webhooks、future LFS/bundle-uri 和 operational endpoints 保持协议原生。

## 技术选择

Backend：

- `async-graphql` + `async-graphql-actix-web`；
- Actix 继续作为 HTTP server；
- GraphQL resolver 只做 transport/application adapter；
- domain/service/repository/service-git 继续拥有业务与数据边界。

Frontend：

- 保留 TanStack Query 作为 server-state/cache owner；
- `graphql-request` 作为 transport；
- GraphQL Code Generator `client-preset` 生成 operation/input/result types；
- 不引入 Apollo/urql 第二套 cache。

## 子 Story

| Story | 独立结果 | 状态 | 依赖 | GitHub |
|---|---|---|---|---|
| EVO-127-A | GraphQL platform/schema/codegen foundation | Proposed | governance baseline | [#27](https://github.com/wjhuang88/evolith/issues/27) |
| EVO-127-B | Auth/session GraphQL migration | Proposed | A | [#28](https://github.com/wjhuang88/evolith/issues/28) |
| EVO-127-C | Tenant admin/API keys/billing/audit migration | Proposed | A/B | [#29](https://github.com/wjhuang88/evolith/issues/29) |
| EVO-127-D | Repository CRUD GraphQL migration | Proposed | A + EVO-126-B | [#30](https://github.com/wjhuang88/evolith/issues/30) |
| EVO-127-E | Repo Context GraphQL migration | Proposed | A + EVO-126-E | [#31](https://github.com/wjhuang88/evolith/issues/31) |
| EVO-127-F | Tools/Skills/Snippets migration | Proposed | A | [#32](https://github.com/wjhuang88/evolith/issues/32) |
| EVO-127-G | Frontend full cutover + REST business API retirement | Proposed | B-F | [#33](https://github.com/wjhuang88/evolith/issues/33) |

## 完成条件

- Frontend business data access 使用 generated GraphQL operations/types。
- `/api/v1` application/business REST consumers 为零并删除对应 handlers。
- GraphQL schema SDL 与 frontend generated artifacts 有 CI drift gate。
- Auth/RBAC/tenant isolation/CSRF/rate-limit/audit semantics 保持或加强。
- query depth/complexity/batch abuse controls 有负向测试。
- Git Smart HTTP/MCP/health/callback 等 protocol endpoints 无回归且不被 GraphQL 包装。

## 迁移原则

1. 不做 big-bang；按 domain vertical slice 迁移。
2. REST/GraphQL 共存只能是 bounded transition，业务逻辑必须共享 service boundary，不允许复制两套业务实现。
3. 项目未上线，不建立长期 REST compatibility debt；迁移完成即删除旧 business routes。
4. Repo GraphQL 工作等待对应 WalGit service boundary 稳定，避免重复重构。
5. 新业务功能在 EVO-127-A 合入后默认走 GraphQL；协议型功能除外。

## 不做

- 不 GraphQL 化 Git Smart HTTP、MCP、health、Webhook/OAuth/payment callbacks、LFS/bundle-uri。
- 不引入 Federation 或独立 GraphQL 服务。
- 不把 cursor pagination redesign 混入迁移。
- 不把 subscriptions 当作本 Epic 必需条件，除非有独立产品需求。
- 不改变现有业务授权模型。
