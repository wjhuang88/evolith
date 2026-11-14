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

After EVO-127-A, existing `/api/v1/*` business routes remain transitional paths. EVO-127-B progressively migrates auth/session through read models and a shared-service-backed `login` mutation. REST auth handlers and frontend consumers remain authoritative until all auth mutations, stale-session recovery, and browser E2E cutover pass.

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

The foundation schema exposes a smoke query and a no-op mutation root. EVO-127-B adds domain queries `currentSession` / `viewer` and the shared-service-backed `login` mutation without retiring REST auth.

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

### `currentSession` (EVO-127-B first read slice)

`currentSession` is nullable. It returns `null` for anonymous requests and reflects the server-validated `CurrentUser` identity for authenticated callers: `userId`, `tenantId`, `role` and `tenantRole`. It does not expose JWTs, cookies, passwords or API key material. It is a claims-level read model, **not** yet a replacement for `/api/v1/auth/me`, which additionally returns persisted user profile fields.

```graphql
query CurrentSession {
  currentSession {
    userId
    tenantId
    role
    tenantRole
  }
}
```

Invalid supplied credentials still fail at the existing optional-auth middleware; they must never be treated as a valid anonymous session.

### `viewer` (EVO-127-B persisted-profile read slice)

`viewer` is a **non-null, authenticated** query. Unlike `currentSession`'s claims-only projection, it loads the current user's persisted record through Evolith's `UserRepository` and returns only `id`, `email`, `username`, `role`, `tenantId`, `tenantRole` and `emailVerified`. This is the read-model parity target for REST `GET /api/v1/auth/me`; no password hash, verification/reset token or credential-bearing fields enter the GraphQL schema.

```graphql
query CurrentViewer {
  viewer {
    id
    email
    username
    role
    tenantId
    tenantRole
    emailVerified
  }
}
```

- Anonymous query: GraphQL error `extensions.code = UNAUTHORIZED`.
- Supplied invalid Bearer/API key: rejected by the existing optional-auth middleware, before schema execution; never silently returns anonymous data.
- Existing JWT user with absent repository row: `USER_NOT_FOUND`.
- If JWT tenant claim does not match the persisted user's tenant: `UNAUTHORIZED`, with no profile fields returned.
- Repository failure: `INTERNAL_ERROR` without returning internal details. Request ID and `retryable` continue to be enriched by the shared GraphQL error handler.

### `login` mutation (EVO-127-B shared auth service slice)

`login(email: String!, password: String!): GraphQLLoginPayload!` calls the same `service-auth::LoginService` as REST `POST /api/v1/auth/login`. Password verification, JWT signing, role normalization and tenant fallback remain single-source service logic; GraphQL resolvers never read raw JWT cookies or access password-hash fields.

```graphql
mutation Login($email: String!, $password: String!) {
  login(email: $email, password: $password) {
    token
    expiresAt
    user { id email username role tenantId tenantRole emailVerified }
    tenant { id name slug plan }
  }
}
```

- Success returns the JWT for **temporary REST-browser compatibility**; this is sensitive credential material and must not be cached, logged or exposed to other GraphQL operations. `expiresAt` is a **string containing Unix epoch seconds**, not GraphQL Int32.
- The HTTP transport, not the resolver, emits `Set-Cookie` for `evolith_token` (httpOnly) plus `csrf_token` (JavaScript-readable) with `Path=/`, `SameSite=Lax`, expiry and environment-specific `Secure`. REST and GraphQL share the same cookie builder.
- Cookie writes are staged per request and committed only when the **entire GraphQL execution** has no errors. Invalid input returns `VALIDATION_ERROR`, wrong email/password returns `INVALID_CREDENTIALS`; neither writes cookies. At most **one** login attempt (including rejected attempts) is permitted per GraphQL HTTP request, so aliases cannot amplify password verification beyond request rate limits; subsequent aliases return `LOGIN_ALREADY_ATTEMPTED` and the entire response has no Set-Cookie. If a first login alias succeeds before a later field errors, the transport clears partial GraphQL `data` so no signed JWT escapes in a failed response. Any GraphQL request that attempts login also receives `Cache-Control: no-store`. The common GraphQL error envelope adds `requestId` and `retryable`.
- A new browser with **no session cookie** may login without a CSRF header. A request carrying a session cookie still requires matching double-submit CSRF, even for login. Invalid/stale supplied JWTs or API keys remain rejected by the existing optional-auth middleware **before** the mutation executes.
- **Remaining cutover blocker:** a browser with an expired/malformed but still-present JWT cookie cannot yet recover by calling GraphQL `login`; the existing REST login route remains available. Do not migrate frontend login to GraphQL until this recovery path and browser E2E are solved. Also, GraphQL `register`, `refresh`, `logout`, password/email/invitation mutation parity remains outstanding.

Frontend auth consumers still use REST until typed operations plus login/register/refresh/logout cookie parity and E2E are ready. Those mutations, cookie response handling and full browser cutover remain EVO-127-B work; REST business-route retirement remains EVO-127-G.

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
