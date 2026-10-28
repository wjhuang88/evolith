# Current Execution Plan — 2026-10 GraphQL + Git Data Plane Convergence

> **Status**: Current execution-order owner
> **Effective**: 2026-10-28
> **Supersedes as current-order owner**: `CURRENT-EXECUTION-PLAN-2026-09.md` (retained as historical baseline)
> **Architecture decisions**: ADR-0011 + ADR-0012

## Why this replan exists

EVO-126-A has established the WalGit dependency/toolchain boundary. The product direction now also requires GraphQL to become the canonical browser/application API.

A naive order creates avoidable rework in either direction:

- migrating repository REST -> GraphQL before a stable `service-git v2` boundary would couple schema resolvers to the filesystem implementation and then require another rewrite;
- finishing the entire WalGit data-plane cutover before establishing GraphQL would keep adding/refining REST application handlers that are already scheduled for retirement.

Therefore GraphQL platform/non-Git domains and WalGit engine/domain work converge in dependency order.

## Current facts

- EVO-126-A completed in PR #24 / merge `09316a6ba3a68fb6e7d30bbba6bf5cc563897696`.
- Current Git runtime is still filesystem bare repos + Git subprocess Smart HTTP + gix Repo Context until later EVO-126 cutover Stories.
- Current application API is still REST `/api/v1/*` until EVO-127 Stories land.
- GraphQL is an accepted target, not yet current runtime.
- Protocol-native Git Smart HTTP, MCP, health, callbacks and future LFS/bundle-uri are not GraphQL migration targets.

## Execution sequence

```text
0. Governance closure
   EVO-126-A / Iteration 069 -> Closed / Complete

1. GraphQL platform foundation
   EVO-127-A
   /graphql + schema/security/error/codegen/TanStack Query transport

2. Non-Git GraphQL vertical slices
   EVO-127-B Auth/session
   -> EVO-127-C Tenant admin/API keys/billing/audit
   -> EVO-127-F Tools/Skills/Snippets

3. Stable Git application boundary
   EVO-126-B service-git v2 + WalGit Registry/Store lifecycle

4. Repository application GraphQL
   EVO-127-D Repository CRUD on Evolith-owned service-git boundary

5. Git protocol/context engine work
   EVO-126-C Smart HTTP read
   -> EVO-126-E Repo Context engine parity
   -> EVO-126-D WAL receive-pack
   (C/E/D order may be refined where dependencies permit)

6. Repo Context GraphQL
   EVO-127-E after EVO-126-E

7. Application REST retirement
   EVO-127-G after EVO-127-B/C/D/E/F
   -> frontend business GraphQL-only
   -> remove /api/v1 business handlers and handwritten REST client/DTO layer

8. Git data-plane operational cutover
   EVO-126-F object-store operations/migration/recovery
   -> EVO-126-G bundle-uri
   -> EVO-126-H production default cutover + legacy filesystem Git engine retirement
   -> GIT-DP-01 Closed

9. Agent/Product continuation
   EVO-105 / 106 / 107 / 104
   -> Discovery / Experience / legacy cleanup
   -> EVO-118-E final production convergence
```

## Why EVO-127-A is allowed before EVO-126-B

EVO-127-A is transport/schema/codegen infrastructure only. It does not depend on Git storage and does not migrate repository behavior. Establishing it now prevents new product work from expanding the REST surface.

## Why EVO-127-D waits for EVO-126-B

Repository GraphQL resolvers must target Evolith-owned repository/application types. If they are built directly over today's filesystem helper layout, the schema adapter would immediately need to be rewritten during service-git v2. EVO-126-B is therefore a hard dependency for D.

## Why EVO-127-E waits for EVO-126-E

Repo Context has storage-sensitive tree/blob/commit/diff behavior and bounded-read/error semantics. The GraphQL adapter should be written against the final Evolith Repo Context service boundary, not the old gix/filesystem implementation.

## Protocol exception rule

The phrase “GraphQL migration” means application/business APIs. Do not migrate these solely for uniformity:

- Git Smart HTTP;
- health/liveness/readiness;
- MCP transport;
- payment/OAuth/webhook callbacks;
- static/metrics/operations;
- LFS/bundle-uri/Git protocol endpoints.

Any proposal to move a protocol endpoint behind GraphQL requires its own ADR and interoperability justification.

## Story activation rule

This file owns ordering, not WIP. Every new Iteration still starts through `START-ITERATION.md` inventory and only selects a child Story whose dependencies are satisfied.

GraphQL and EVO-126 may alternate by dependency, but a single Agent session must still obey the one-In-Progress-Story rule.

## Completion gates

### API-GQL-01

Closed when EVO-127-G completes:

- GraphQL is the canonical application API;
- frontend has zero `/api/v1` business calls;
- REST business handlers are removed;
- schema/codegen/security regression gates are durable;
- protocol-native endpoints remain intact.

### GIT-DP-01

Unchanged from EVO-126: closed only after object-store/WAL migration, protocol/context parity, migration/recovery and legacy Git engine cutover.

Neither gate implies production readiness by itself. DEPLOY-01 remains the final release convergence gate.
