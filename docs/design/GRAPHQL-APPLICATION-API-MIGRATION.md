# GraphQL Application API Migration Design

> Owner: EVO-127 / GitHub #26
> Decision: ADR-0012
> Status: Accepted target design; runtime remains REST-first until implementation Stories land.

## 1. Goal

Make GraphQL the canonical Evolith application API for browser/product interactions while keeping protocol-native HTTP surfaces intact.

Target topology:

```text
React/Vite
  -> generated GraphQL operations
  -> TanStack Query
  -> Evolith GraphQL transport (/graphql)
  -> async-graphql / Actix
  -> Evolith application/service boundaries
  -> domain + repositories + service-git

Protocol-native clients
  -> Git Smart HTTP / MCP / health / callbacks / future LFS/bundle
  -> dedicated Actix protocol handlers
```

GraphQL is an API composition layer, not a replacement for services, repositories or protocol handlers.

## 2. Current state

Current browser data access is centered on `frontend/src/lib/api/client.ts` and domain REST modules. The backend route tree exposes:

- `/api/v1/auth/*`;
- global Tools/Skills/Snippets application endpoints;
- `/api/v1/tenant/{tenant_id}/members`;
- API Keys / billing / audit;
- repository CRUD and Repo Context;
- invitation/join flows.

Separate existing protocol surfaces include `/repos/*` Smart HTTP, `/mcp` and health endpoints.

The migration must not interpret “all interfaces” as “all HTTP traffic”. Application business contracts move to GraphQL; standardized protocols remain standardized protocols.

## 3. Backend module shape

Recommended API crate layout:

```text
backend/crates/api/src/graphql/
  mod.rs
  schema.rs
  context.rs
  error.rs
  query.rs
  mutation.rs
  auth.rs
  tenant.rs
  repository.rs
  repo_context.rs
  capabilities.rs
  admin.rs
```

The root `Query` and `Mutation` compose domain-specific GraphQL objects. Domain resolvers are intentionally thin:

```text
GraphQL resolver
  -> parse/validate GraphQL input
  -> read Principal/Tenant context
  -> call Evolith service/application API
  -> map Evolith result/error to GraphQL type/error extension
```

Resolvers must not:

- open databases directly when an existing repository/service owns the operation;
- parse JWT/API keys independently from established auth middleware/context;
- perform WalGit/object-store operations directly;
- duplicate Policy/RBAC decisions in ad-hoc field code.

## 4. Schema conventions

### IDs

Expose UUID-backed identifiers as a GraphQL UUID scalar or a consistently documented `ID`. Do not expose filesystem paths, WalGit RepoId internals or database surrogate details as API identity.

### Tenancy

Use explicit tenant scoping in application operations. Candidate query shape:

```graphql
query RepoList($tenantId: UUID!) {
  tenant(id: $tenantId) {
    repositories { ... }
  }
}
```

Mutations keep tenant identity explicit in the mutation input or parent scope. Schema shape never replaces authorization; services verify membership/role.

### Mutations

Use action-oriented mutations and payload types, for example:

```graphql
mutation CreateRepository($input: CreateRepositoryInput!) {
  createRepository(input: $input) {
    repository { id name status }
  }
}
```

Expected domain failures may still surface as GraphQL errors with stable extension codes. Do not recreate the REST `success/data/error` envelope around every field.

### Errors

GraphQL error extension contract:

```json
{
  "message": "Repository not found",
  "extensions": {
    "code": "NOT_FOUND",
    "requestId": "...",
    "retryable": false
  }
}
```

Transport-level failures such as malformed JSON, request-size rejection or global rate limiting may continue to use appropriate HTTP status codes before GraphQL execution.

## 5. Request context and security

`GraphQLContext` should contain Evolith-owned values, not transport library internals:

```rust
pub struct GraphQLRequestContext {
    pub request_id: Uuid,
    pub principal: Option<Principal>,
    // application/service handles
}
```

Authentication middleware resolves browser cookie or Bearer identity before schema execution where practical. Public operations still receive an unauthenticated context.

### CSRF

Because queries and mutations commonly use HTTP POST, old “state-changing HTTP verb” logic cannot be copied blindly.

Target rule:

- browser request with Evolith session cookie -> require valid double-submit CSRF header for `/graphql`;
- unauthenticated public auth request with no session cookie -> allowed;
- bearer-only request with no browser session cookie -> no browser CSRF dependency.

The frontend GraphQL transport always attaches `X-CSRF-Token` when the cookie exists, so authenticated queries and mutations satisfy the same transport rule.

### Rate/cost controls

The existing caller-aware request limiter remains outside schema execution. In addition:

- batch requests disabled initially;
- request body size bounded;
- schema depth/complexity limit configured;
- expensive list fields expose explicit pagination bounds;
- representative expensive queries have negative tests;
- production GraphiQL is off.

## 6. Frontend architecture

Keep the existing TanStack Query ownership model:

```text
React component
 -> queryOptions/useQuery/useMutation
 -> generated TypedDocumentNode
 -> graphql-request based Evolith fetcher
 -> /graphql
```

Do not add Apollo normalized cache or urql document cache while TanStack Query remains authoritative.

Recommended structure:

```text
frontend/src/graphql/
  client.ts
  operations/
    auth.graphql
    tenant.graphql
    repositories.graphql
    repo-context.graphql
    capabilities.graphql
  generated/
    graphql.ts
    gql.ts
```

Codegen owns operation input/result types. Domain UI code imports generated types/documents rather than duplicating REST DTOs.

Transport responsibilities:

- `credentials: include`;
- optional Bearer header during the bounded legacy token transition;
- CSRF header when cookie is available;
- `X-Request-ID`;
- normalized network/GraphQL error helper for UI error states.

## 7. Schema/codegen pipeline

Backend schema code is executable source of truth. CI should run an export command/test such as:

```text
cargo run -p api --bin export-graphql-schema
```

(or an equivalent deterministic test/helper) and compare the result with a committed SDL artifact.

Frontend Codegen consumes the committed SDL plus operation documents. CI then runs codegen and fails if generated output differs from the repository.

This gives reviewable schema diffs without requiring a running backend during frontend build.

## 8. Migration map

| Current domain | GraphQL Story | Dependency |
|---|---|---|
| platform/schema/client | EVO-127-A | none after governance/iteration selection |
| auth/session/invitations | EVO-127-B | A |
| members/API keys/billing/audit | EVO-127-C | A/B |
| repository CRUD | EVO-127-D | A + EVO-126-B |
| Repo Context | EVO-127-E | A + EVO-126-E |
| tools/skills/snippets/CLI app views | EVO-127-F | A |
| REST retirement | EVO-127-G | B-F |

WalGit protocol Stories remain owned by EVO-126. GraphQL does not become a second owner for Git transport semantics.

## 9. REST retirement policy

During migration, the same domain may have REST and GraphQL adapters calling the same underlying service. This coexistence is temporary and should not result in dual business logic implementations.

Before deleting a REST domain:

1. search frontend and tests for its paths/imports;
2. migrate browser E2E to GraphQL operations;
3. prove auth/RBAC/error parity;
4. remove handler registration and unused DTO/client code;
5. update API contract/reference.

Final EVO-127-G adds a CI/static check that prevents new `/api/v1` browser business calls from returning.

## 10. Observability

Preserve `X-Request-ID` and structured tracing. GraphQL spans should record operation name/type and top-level field, but never raw secrets/variables.

Metrics should distinguish:

- GraphQL transport request count/latency;
- operation errors by stable extension code;
- complexity/depth rejection;
- resolver/service latency for known expensive domains.

Do not log full query variables when they may contain password, API key or token inputs.

## 11. Explicit non-goals

Initial migration does not introduce:

- GraphQL Federation;
- a standalone GraphQL microservice;
- Apollo/urql client cache;
- subscriptions without a concrete product need;
- Relay cursor pagination as a cross-product redesign;
- GraphQL wrappers for Git Smart HTTP, MCP, health, callbacks, LFS or bundle-uri;
- a new authorization model.

## 12. Completion

EVO-127 completes only when the frontend application has zero legacy REST business calls, `/api/v1` business handlers are retired, schema/codegen CI is authoritative, security semantics are regression-tested, and protocol-native endpoints remain unaffected.
