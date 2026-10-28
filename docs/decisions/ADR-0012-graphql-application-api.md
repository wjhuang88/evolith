# ADR-0012: GraphQL as the Canonical Application API

- **Status**: Accepted
- **Date**: 2026-10-28
- **Owners**: EVO-127
- **Supersedes**: the REST-first application API direction in `docs/reference/API-CONTRACT.md`; protocol-native endpoints are explicitly excluded
- **Related**: ADR-0011 WalGit-backed Git Data Plane

## Context

Evolith currently exposes application behavior primarily through `/api/v1/*` REST handlers and the frontend calls those handlers through a handwritten Axios client and handwritten request/response types. The route surface spans auth, tenant members, API keys, billing, audit, repositories, Repo Context, Tools, Skills and Snippets.

This creates repeated DTO/error/pagination plumbing across backend handlers and frontend modules, and makes UI composition depend on endpoint granularity rather than the data graph required by a screen. The project is still pre-launch, so preserving a long-lived public REST compatibility surface has less value than converging on one typed application contract before external consumers stabilize.

At the same time, not every HTTP endpoint is an application API. Git Smart HTTP, MCP, health probes, callbacks/webhooks and future Git protocol extensions have transport contracts that should remain protocol-native.

## Decision

GraphQL becomes Evolith's **canonical application/business API**.

The target application endpoint is `/graphql`, served by the existing Actix application. Existing service/domain crates remain the owners of business rules; GraphQL resolvers are transport adapters and must not become a second domain layer.

The following endpoints remain protocol-native and are not migrated merely for uniformity:

- Git Smart HTTP `/repos/*`;
- `/health`, `/health/live`, `/health/ready`;
- `/mcp`;
- payment/OAuth/webhook callbacks;
- metrics/static/operational endpoints;
- future Git LFS and bundle-uri/protocol endpoints.

### Backend stack

Use `async-graphql` with `async-graphql-actix-web` on the existing Actix server. At the time of this decision, async-graphql 7.2.x supports Actix integration, query complexity/depth limits, custom error extensions and response HTTP headers, and its MSRV is below Evolith's Rust 1.94.1 integration baseline.

The GraphQL root is composed by domain modules rather than one monolithic resolver file. Resolvers receive an Evolith-owned request context containing principal/session identity, request ID and application/service dependencies.

### Frontend stack

Keep TanStack Query as the server-state/cache layer already used by the frontend. Do not introduce Apollo Client or urql as a second normalized cache.

Use:

- `graphql-request` as the GraphQL HTTP transport;
- GraphQL Code Generator `client-preset` for typed operations/results/inputs;
- the generated GraphQL documents from the committed schema contract;
- a thin Evolith transport wrapper that carries credentials, Bearer auth where applicable, CSRF and `X-Request-ID`.

Handwritten REST response DTOs are retired as each domain migrates.

## Contract rules

### Schema ownership

Rust schema composition is the executable source of truth. A generated SDL artifact is committed for review/codegen and CI verifies that SDL and generated frontend artifacts are not stale.

Schema evolution must be reviewed as an API contract change. Because the product is pre-launch, migration Stories may remove obsolete REST contracts rather than maintaining indefinite dual compatibility.

### Error model

Application GraphQL responses use standard GraphQL `{ data, errors }` semantics rather than the REST `ApiResponse<T>` envelope.

Stable machine-readable error metadata is exposed through `errors[].extensions`, including at minimum:

- `code`;
- `requestId` when available;
- `retryable` when meaningful.

Existing domain/security distinctions such as `UNAUTHORIZED`, `FORBIDDEN`, `NOT_FOUND`, `INVALID_INPUT`, `RESOURCE_EXCEEDED` and `TIMEOUT` remain distinguishable.

### Authentication and CSRF

Current cookie/JWT/Bearer authentication semantics remain authoritative.

A browser request carrying Evolith session cookies to `/graphql` must satisfy the double-submit CSRF contract. This may be stricter than the old REST method-based rule because GraphQL normally uses POST for both queries and mutations; a stricter transport rule is acceptable, weakening CSRF is not.

Public auth mutations with no authenticated session cookie remain callable without an existing CSRF cookie. Bearer-only API clients are not made dependent on browser CSRF cookies.

Auth mutations may set/clear cookies through GraphQL response headers; cookie transport details remain outside domain services.

### Authorization and tenancy

Authentication is resolved before resolver execution and injected into GraphQL context. Tenant/repository authorization remains enforced by Evolith application/service/repository boundaries; schema nesting is not authorization by itself.

GraphQL must not expose WalGit/storage-engine types. Repository fields depend on Evolith-owned application/service types.

### Abuse resistance

Initial GraphQL rollout must:

- disable GraphQL batch requests so one HTTP request cannot bypass request-rate accounting with many operations;
- configure query depth and complexity limits;
- keep existing caller-aware rate limiting at the transport boundary;
- avoid resolver N+1 behavior through batch repository methods or DataLoader where a relation actually requires batching;
- keep GraphiQL/developer tooling disabled or explicitly development-only in production.

Exact depth/complexity thresholds are implementation configuration owned by EVO-127-A and must be justified by tests/representative operations rather than copied from generic defaults.

### Pagination

The migration preserves current pagination/filter behavior first. Introducing Relay cursor pagination across the product is a separate API design change and is not bundled into this transport migration.

## Migration strategy

Use bounded vertical slices, not a big-bang rewrite.

1. EVO-127-A establishes `/graphql`, schema/error/security foundation and frontend codegen/transport.
2. EVO-127-B migrates auth/session.
3. EVO-127-C migrates tenant administration, API keys, billing and audit.
4. EVO-127-D migrates repository management after EVO-126-B stabilizes the service-git engine boundary.
5. EVO-127-E migrates Repo Context after EVO-126-E stabilizes its data-plane boundary.
6. EVO-127-F migrates Tools/Skills/Snippets application APIs while keeping MCP protocol-native.
7. EVO-127-G proves zero application consumers of REST and removes `/api/v1` business handlers/handwritten REST client artifacts.

REST and GraphQL may coexist only as a bounded migration state. New product features should target GraphQL once EVO-127-A lands unless a Story is explicitly protocol-native.

## Consequences

### Positive

- one typed application graph for UI composition;
- generated frontend types reduce contract drift;
- transport no longer determines backend domain boundaries;
- repository/data-plane refactors can stay behind Evolith-owned services while GraphQL schema remains stable;
- pre-launch REST cleanup is explicit rather than becoming permanent compatibility debt.

### Costs and risks

- resolver N+1 and unbounded-query risks require explicit controls;
- auth/CSRF moves from path/method assumptions to a single GraphQL transport and must be regression-tested carefully;
- GraphQL errors use different HTTP semantics from current REST handlers, requiring observability and client error handling updates;
- schema/codegen drift becomes a new CI responsibility;
- partial migration temporarily means two application transports exist.

## Rejected alternatives

### Keep REST as the canonical frontend API

Rejected because the user has selected GraphQL as the target application contract and the project is pre-launch, making this the lowest-cost time to converge.

### Add Apollo Client and keep TanStack Query

Rejected because two client-side server-state/cache systems would create overlapping invalidation and ownership rules. Evolith keeps TanStack Query and uses a lightweight typed GraphQL transport.

### GraphQL-ize Git Smart HTTP/MCP/health/callbacks

Rejected. Those are protocol or operational endpoints, not application CRUD/query contracts; wrapping them in GraphQL would reduce interoperability and add no product value.

### Big-bang replacement of every REST handler in one change

Rejected because auth/security, Git data plane and frontend migration have different failure modes and dependencies. Vertical Stories provide rollback and parity evidence.

## Follow-up

Execution is tracked by EVO-127 / GitHub #26 and child issues #27-#33. This ADR does not itself activate an Iteration.
