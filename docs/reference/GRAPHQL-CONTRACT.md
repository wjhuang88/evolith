# GraphQL Application API Contract

> Owner: EVO-127 / ADR-0012
>
> Foundation implementation: EVO-127-A / Iteration 070
>
> Canonical executable schema: [`graphql/schema.graphql`](graphql/schema.graphql)

## 1. Scope

`/graphql` is Evolith's canonical **application/business API migration surface**. It does not replace protocol-native endpoints.

The following remain outside GraphQL by design:

- Git Smart HTTP under `/repos/*`;
- MCP under `/mcp`;
- health/readiness endpoints;
- callback/webhook endpoints where an external protocol defines the transport.

During EVO-127-A, existing `/api/v1/*` business routes remain compatibility paths. Domain migration and REST retirement are owned by later EVO-127 slices.

## 2. Endpoint

### `POST /graphql`

- Content type: `application/json`.
- One GraphQL request per HTTP request.
- JSON batch arrays are rejected with HTTP `400` and error code `GRAPHQL_BATCH_NOT_SUPPORTED`.
- Request body limit: **256 KiB**.
- Schema limits:
  - maximum depth: **15**;
  - maximum complexity: **1000**;
  - maximum directives: **20**.

The foundation schema intentionally exposes only a smoke query and a no-op mutation root. Product domain fields are added by later vertical migration stories.

Example:

```graphql
query GraphQLFoundationStatus {
  graphqlFoundation {
    service
    transport
    version
  }
  requestId
}
```

### `GET /graphql`

GET is not a production query transport. It serves GraphiQL only when both conditions are true:

1. Evolith is running in `development` environment; and
2. `GRAPHQL_GRAPHIQL_ENABLED=true` (or equivalent truthy value).

Production returns `404` even if the flag is accidentally enabled.

## 3. Authentication and request context

GraphQL reuses the existing Evolith credential and authorization model; resolvers must not parse credentials themselves.

- Browser session: `evolith_token` httpOnly cookie.
- API client: `Authorization: Bearer <JWT>`.
- API key callers: existing API-key resolution rules, including `X-API-Key` where supported by the shared auth layer.
- Anonymous transport is allowed because the schema may contain public operations; authenticated resolvers remain responsible for authorization.
- If a caller supplies an invalid credential, the request fails closed rather than silently degrading to anonymous.

Resolved identity is inserted as the existing `CurrentUser` principal and copied into GraphQL request context together with the request ID.

## 4. CSRF contract

GraphQL preserves browser CSRF protection without imposing browser state on non-browser API clients.

- A `POST /graphql` carrying the `evolith_token` session cookie **must** also carry the double-submit CSRF pair:
  - `csrf_token` cookie;
  - matching `X-CSRF-Token` header.
- A bearer-only GraphQL request without the session cookie is not required to send a CSRF token.
- This GraphQL-specific rule does not weaken the historical REST CSRF policy for other state-changing routes.

## 5. Request IDs and errors

`X-Request-ID` continues to use the existing request-id middleware. GraphQL errors use standard GraphQL `data` / `errors` semantics rather than the REST `ApiResponse<T>` envelope.

Machine-readable error metadata is exposed through `errors[].extensions`:

```json
{
  "code": "GRAPHQL_ERROR",
  "requestId": "<request-id>",
  "retryable": false
}
```

Rules:

- preserve a resolver-supplied stable `code` when present;
- otherwise use `GRAPHQL_ERROR`;
- always attach `requestId`;
- default `retryable` to `false` unless a resolver explicitly marks a condition retryable.

Transport-level batch rejection uses `GRAPHQL_BATCH_NOT_SUPPORTED`.

## 6. Rate limiting

GraphQL requests use the existing caller-aware rate limiter. Optional GraphQL authentication runs before rate-limit identity selection, so authenticated JWT/API-key callers use their existing caller buckets rather than the anonymous IP bucket.

No GraphQL-specific parallel rate-limit implementation is introduced.

## 7. Schema and generated-client authority

The contract pipeline is:

```text
Rust async-graphql schema
        │
        ├─ exporter: cargo run --locked -p api --bin export_graphql_schema
        ▼
docs/reference/graphql/schema.graphql
        │
        ├─ GraphQL Code Generator client preset
        ▼
frontend/src/graphql/generated/
```

The committed SDL is the executable GraphQL contract artifact. `frontend/src/graphql/operations/` contains authored operations; `frontend/src/graphql/generated/` is generated code and must not be edited by hand.

Formal CI enforces both directions:

1. regenerate the frontend client and fail on generated diff;
2. export the Rust schema and fail when it differs from the committed SDL.

Frontend requests go through `frontend/src/graphql/client.ts`. TanStack Query remains the application cache/server-state layer; `graphql-request` is transport only.

## 8. Foundation non-goals

EVO-127-A does not implement:

- domain-wide GraphQL migration;
- REST deletion;
- subscriptions;
- federation;
- persisted queries;
- cursor-pagination redesign.

Those changes require their own EVO-127 vertical slices and contract updates.
