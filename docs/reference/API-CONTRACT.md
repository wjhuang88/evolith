# Evolith API Contract

**Base URL:** `http://localhost:8080`

This document describes the complete REST API contract for Evolith, matching the current handler implementations. All endpoints, request/response shapes, validation rules, query parameters, error codes, and authentication requirements are documented. Stub or not-yet-implemented endpoints are marked with ⚠️.

---

## Table of Contents

1. [Common Types](#common-types)
2. [Authentication](#authentication)
3. [Common Headers](#common-headers)
4. [Error Codes](#error-codes)
5. [Rate Limiting](#rate-limiting)
6. [Health](#health)
7. [MCP Endpoints](#mcp-endpoints)
8. [Auth](#auth)
9. [Tools](#tools)
10. [Skills](#skills)
11. [Snippets](#snippets)
12. [CLI Interfaces](#cli-interfaces)
13. [Members](#members)
14. [API Keys](#api-keys)
15. [Billing](#billing)
16. [Repos](#repos)
17. [Smart HTTP (Git Protocol)](#smart-http-eVO-103)
18. [Repo Context API](#repo-context-eVO-103-c)
19. [API key scope](#api-key-scope-eVO-116)
20. [Push metadata sync](#push-metadata-sync-eVO-116)
21. [Audit Logs](#audit-logs)
22. [Version History](#version-history)

---

## Common Types

```typescript
interface ApiResponse<T> {
  success: boolean;
  data?: T;
  meta?: PaginationMeta;
  error?: ErrorInfo;
}

interface ErrorInfo {
  code: string;
  message: string;
}

interface PaginationMeta {
  page: number;
  per_page: number;
  total: number;
}
```

---

## Authentication

Evolith supports two authentication methods: **httpOnly cookies** (recommended for browsers) and **Authorization header** (for API clients).

### Cookie-Based Authentication (Browser)

On successful login, register, or token refresh, the server sets two cookies:

| Cookie | Name | httpOnly | SameSite | Secure | Purpose |
|--------|------|----------|----------|--------|---------|
| JWT Token | `evolith_token` | ✅ Yes | Lax | Production only | Contains the JWT token. Automatically sent with every request. |
| CSRF Token | `csrf_token` | ❌ No | Lax | Production only | CSRF protection token. Readable by JavaScript. |

**Cookie Lifecycle:**
- **Set on**: `POST /api/v1/auth/login`, `POST /api/v1/auth/register`, `POST /api/v1/auth/refresh`
- **Cleared on**: `POST /api/v1/auth/logout` (both cookies set to `max_age=0`)
- **Expiration**: Same as JWT token lifetime (configured via `JWT__EXPIRATION`, default 24h)

### CSRF Protection

State-changing requests (`POST`, `PUT`, `PATCH`, `DELETE`) require CSRF validation when using cookie authentication. This uses the **double-submit cookie pattern**:

1. Read the `csrf_token` cookie value via JavaScript
2. Include it as the `X-CSRF-Token` header on every state-changing request

**CSRF-exempt paths** (no `X-CSRF-Token` required):
- `/health`, `/health/live`, `/health/ready`
- `/mcp`
- `/api/v1/auth/login`, `/api/v1/auth/register`
- `/api/v1/auth/forgot-password`, `/api/v1/auth/reset-password`, `/api/v1/auth/verify-email`
- `/api/v1/invitations/accept`, `/api/v1/tenant/{tenant_id}/members/join`
- `/repos/{repo_id}/git-upload-pack` (POST), `/repos/{repo_id}/git-receive-pack` (POST) — standard git clients do not send CSRF tokens

**CSRF error response:**
```json
{
  "success": false,
  "error": {
    "code": "CSRF_ERROR",
    "message": "CSRF token missing from header"
  }
}
```

| CSRF Error | HTTP Status | Description |
|-----------|-------------|-------------|
| Missing cookie | 403 | `csrf_token` cookie not present |
| Missing header | 403 | `X-CSRF-Token` header not sent |
| Token mismatch | 403 | Header value doesn't match cookie value |

### Header-Based Authentication (API Clients)

API clients can use the `Authorization: Bearer <token>` header instead of cookies. When using header-based auth, CSRF validation is still required for state-changing requests if a `csrf_token` cookie is present.

### Frontend Integration Example

```typescript
// Axios configuration for cookie-based auth
const client = axios.create({
  baseURL: 'http://localhost:8080',
  withCredentials: true,  // Required for cookie authentication
});

// Add CSRF token to state-changing requests
client.interceptors.request.use((config) => {
  const csrfToken = document.cookie
    .split('; ')
    .find(row => row.startsWith('csrf_token='))
    ?.split('=')[1];
  if (csrfToken && ['post', 'put', 'patch', 'delete'].includes(config.method ?? '')) {
    config.headers['X-CSRF-Token'] = csrfToken;
  }
  return config;
});
```

---

## Common Headers

### Request Headers

| Header | Type | Required | Description |
|--------|------|----------|-------------|
| `Authorization` | string | Conditional | `Bearer <JWT token>` for authenticated endpoints (alternative to cookie auth) |
| `X-API-Key` | string | Conditional | API key for MCP endpoints |
| `X-CSRF-Token` | string | Conditional | Required on `POST`/`PUT`/`PATCH`/`DELETE` when using cookie authentication. Value must match the `csrf_token` cookie. |
| `X-Request-ID` | string | No | Client-provided request ID. If absent, server generates one (UUID v4) |
| `Content-Type` | string | Yes (POST/PUT/PATCH) | `application/json` |

### Response Headers

| Header | Type | Always Present | Description |
|--------|------|----------------|-------------|
| `X-Request-ID` | string | Yes | Request tracking ID (echoed from request or server-generated). Use for log correlation and support requests. |

### Response Cookies

| Cookie | Set On | Description |
|--------|--------|-------------|
| `evolith_token` | Login, Register, Refresh | httpOnly JWT cookie |
| `csrf_token` | Login, Register, Refresh | Non-httpOnly CSRF token |

---

## Error Codes

| Code               | Description                                 | HTTP Status |
|--------------------|---------------------------------------------|-------------|
| VALIDATION_ERROR   | Request validation failed                   | 400         |
| INVALID_CREDENTIALS| Wrong email or password                     | 401         |
| UNAUTHORIZED       | Missing or invalid JWT                      | 401         |
| FORBIDDEN          | Insufficient permissions                    | 403         |
| NOT_FOUND          | Resource not found                          | 404         |
| EMAIL_EXISTS       | Email already registered                    | 409         |
| DUPLICATE          | Resource already exists                     | 409         |
| WEAK_PASSWORD      | Password doesn't meet strength requirements | 400         |
| INVALID_ID         | UUID format error                           | 400         |
| INVALID_RUNTIME    | Unknown runtime string                      | 400         |
| INVALID_ROLE       | Role must be admin or member                | 400         |
| INVALID_INPUT      | Repo Context API received malformed input (e.g. invalid blob SHA hex) | 400 |
| RESOURCE_EXCEEDED  | Repo Context API refused a request that exceeded a size or entry limit (blob > 1 MiB, file-tree > 5000 entries, diff > 5000 entries) | 413 |
| TIMEOUT            | A blocking gix read exceeded `CONTEXT_BLOCKING_TIMEOUT` (5s) | 504 |
| ALREADY_INVITED    | Email already has pending invitation        | 400         |
| PLAN_NOT_FOUND     | Invalid plan ID                             | 400         |
| REPO_EXISTS        | Repo with that name already exists in tenant | 409         |
| DATABASE_ERROR     | Database operation failed                   | 500         |
| INTERNAL_ERROR     | Server error                                | 500         |
| NOT_IMPLEMENTED    | Feature not yet available                   | 501         |
| USER_NOT_FOUND     | User not found                              | 404         |
| CSRF_ERROR         | CSRF token missing or mismatch              | 403         |

---

## Rate Limiting

All endpoints are rate-limited by client IP address using a token bucket algorithm.

| Tier | Default Limit | Applies To |
|------|--------------|------------|
| Unauthenticated | 30 requests/minute | Requests without valid JWT or API key |
| Authenticated | 300 requests/minute | Requests with valid JWT |
| API Key | 1000 requests/minute | Requests with valid API key |

**Rate limit exceeded response:**

```
HTTP/1.1 429 Too Many Requests
```

The rate limits are configurable via environment variables:
- `RATE_LIMIT__UNAUTHENTICATED_RPM` (default: 30)
- `RATE_LIMIT__AUTHENTICATED_RPM` (default: 300)
- `RATE_LIMIT__API_KEY_RPM` (default: 1000)

> **Note:** Current implementation applies IP-based rate limiting globally via `actix-governor`. Per-user and per-api-key rate limiting tiers are planned for future phases.

---

## Health

### `GET /health`

- **Auth:** None
- **Description:** Basic health check. Returns healthy status if the server is running.

**Response:**
```json
{
  "status": "healthy",
  "version": "0.1.0"
}
```

---

### `GET /health/live`

- **Auth:** None
- **Description:** Kubernetes-style liveness probe. Returns OK if the process is alive.

**Response:**
```json
{
  "status": "ok",
  "version": "0.1.0"
}
```

---

### `GET /health/ready`

- **Auth:** None
- **Description:** Kubernetes-style readiness probe. Checks database connectivity. Returns `"healthy"` if all checks pass, `"degraded"` if any check fails.

**Response:**
```typescript
interface ReadinessResponse {
  status: string;        // "healthy" | "degraded"
  version: string;       // semver from Cargo.toml
  checks: {
    database: boolean;   // true if DB query succeeds
  };
}
```

Example (healthy):
```json
{
  "status": "healthy",
  "version": "0.1.0",
  "checks": {
    "database": true
  }
}
```

Example (degraded):
```json
{
  "status": "degraded",
  "version": "0.1.0",
  "checks": {
    "database": false
  }
}
```

---

## MCP Endpoints

### `POST /mcp`

- **Auth:** `tools/call` requires an API Key (`Authorization: Bearer <key>` or `X-API-Key`). Protocol initialization and public tool discovery may be requested without a key.
- **Description:** JSON-RPC 2.0 endpoint for MCP operations

**Request:**
JSON-RPC 2.0 object.
Supported methods: `initialize`, `tools/list`, `tools/call`, `resources/list`, `resources/read`, `prompts/list`, `ping`

`tools/call` never executes a tool anonymously, even when that tool is visible in public discovery.

**Response:**
JSON-RPC 2.0 response object.

---

### `GET /mcp/tools`

- **Auth:** API Key optional
- **Description:** List available MCP tools (REST format)

**Response:**
```typescript
ApiResponse<{
  tools: McpTool[];
}>

interface McpTool {
  name: string;
  description: string;
  input_schema: object;
}
```

---

## Auth

### `POST /api/v1/auth/register`

- **Auth:** None
- **Description:** Register a new user and tenant. Sets `evolith_token` and `csrf_token` cookies.

**Request:**
```typescript
interface RegisterRequest {
  email: string;        // email format
  username: string;     // 3-50 chars
  password: string;     // 8-128 chars, Argon2 strength
  tenant_name?: string; // optional, defaults to "{username}'s Team"
  tenant_slug?: string; // optional, auto-generated from username
}
```

**Response:**
```typescript
ApiResponse<AuthResponseData>
```

---

### `POST /api/v1/auth/login`

- **Auth:** None
- **Description:** Login with email and password. Sets `evolith_token` and `csrf_token` cookies.

**Request:**
```typescript
interface LoginRequest {
  email: string;     // email format
  password: string;  // min 1 char
}
```

**Response:**
```typescript
ApiResponse<AuthResponseData>
```

---

### `POST /api/v1/auth/logout`

- **Auth:** JWT
- **Description:** Logout current user. Clears `evolith_token` and `csrf_token` cookies.

**Request:** _No body_

**Response:**
```typescript
ApiResponse<MessageResponse>
```

---

### `POST /api/v1/auth/refresh`

- **Auth:** JWT
- **Description:** Refresh JWT token. Sets new `evolith_token` and `csrf_token` cookies.

**Request:** _No body_

**Response:**
```typescript
ApiResponse<TokenResponse>
```

---

### `GET /api/v1/auth/me`

- **Auth:** JWT
- **Description:** Get current user info

**Response:**
```typescript
ApiResponse<UserInfo>
```

---

### `PATCH /api/v1/auth/profile`

- **Auth:** JWT
- **Description:** Update user profile

**Request:**
```typescript
interface UpdateProfileRequest {
  username?: string;
  email?: string;
}
```

**Response:**
```typescript
ApiResponse<UserInfo>
```

---

### `POST /api/v1/auth/change-password`

- **Auth:** JWT
- **Description:** Change password

**Request:**
```typescript
interface ChangePasswordRequest {
  old_password: string;  // min 1 char
  new_password: string;  // 8-128 chars
}
```

**Response:**
```typescript
ApiResponse<MessageResponse>
```

---

### `POST /api/v1/auth/send-verify`

- **Auth:** None
- **Description:** Send verification email. Always returns success for valid requests to avoid email enumeration. If the email exists and is not yet verified, a verification token is generated and sent via the configured mailer.

**Request:**
```typescript
interface SendVerifyEmailRequest {
  email: string;
}
```

**Response:**
```typescript
ApiResponse<MessageResponse>
```

**Errors:**
- `VALIDATION_ERROR` — email format invalid

---

### `POST /api/v1/auth/verify-email`

- **Auth:** None
- **Description:** Verify email address using a valid verification token.

**Request:**
```typescript
interface VerifyEmailRequest {
  token: string;
}
```

**Response:**
```typescript
ApiResponse<MessageResponse>
```

**Errors:**
- `INVALID_TOKEN` — token does not exist or is invalid

---

### `POST /api/v1/auth/forgot-password`

- **Auth:** None
- **Description:** Send a password reset link if the account exists. Always returns success for valid requests to avoid email enumeration.

**Request:**
```typescript
interface ForgotPasswordRequest {
  email: string;
}
```

**Response:**
```typescript
ApiResponse<MessageResponse>
```

---

### `POST /api/v1/auth/reset-password`

- **Auth:** None
- **Description:** Reset the password using a valid, unexpired reset token.

**Request:**
```typescript
interface ResetPasswordRequest {
  token: string;
  password: string;
}
```

**Response:**
```typescript
ApiResponse<MessageResponse>
```

**Errors:**
- `INVALID_TOKEN` - token does not exist or is invalid
- `TOKEN_EXPIRED` - token has expired
- `WEAK_PASSWORD` - replacement password does not meet policy

---

### Auth Types

```typescript
interface AuthResponseData {
  token: string;
  expires_at: number;   // Unix timestamp
  user: UserInfo;
  tenant: TenantInfo;
}

interface UserInfo {
  id: string;           // UUID
  email: string;
  username: string;
  role: string;         // "user" | "admin"
  tenant_id: string;    // UUID
  tenant_role: string;  // "owner" | "admin" | "member"
  email_verified: boolean;
}

interface TenantInfo {
  id: string;           // UUID
  name: string;
  slug: string;
  plan: string;         // "free" | "starter" | "pro" | "enterprise"
}

interface TokenResponse {
  token: string;
  expires_at: number;
}

interface MessageResponse {
  message: string;
}
```

---

## Tools

### Stable Tool management boundary

`POST /api/v1/tools` and `PUT /api/v1/tools/{id}` are security-sensitive management endpoints.

- Only a same-tenant `owner` or `admin` authenticated with JWT may create or update a Tool.
- A `member` or any API Key caller receives `403 FORBIDDEN`, including an API Key with `execute` capability.
- Authorization runs before JSON/DTO deserialization. An unauthorized caller therefore receives 403 even when the request body is malformed.
- HTTP handler configuration is validated before persistence and before any target connection.
- `DELETE /api/v1/tools/{id}` intentionally retains its existing same-tenant authenticated legacy behavior and is documented separately; the create/update role restriction must not be inferred for delete.

### HTTP Tool handler validation

When `type` is `"http"`:

- `handler_url` is required and must use `https` in the production policy;
- URL credentials/userinfo are forbidden;
- metadata hostnames, loopback, private, link-local, multicast, documentation, benchmark, reserved, mapped-private and other non-public targets are rejected;
- every resolved A/AAAA record must be approved; mixed public/private results fail closed;
- the approved address set is pinned to the connection and automatic redirects/system proxies are disabled;
- redirects are handled explicitly, bounded, and each hop repeats URL, DNS and IP validation;
- `handler_method` may be `GET`, `POST`, `PUT`, `DELETE`, `PATCH`, `HEAD`, or `OPTIONS` (case-insensitive);
- `handler_timeout`, when present, must be in `1..=30000` milliseconds.

Invalid or disallowed handler configuration returns `400 VALIDATION_ERROR` with a stable, redacted message. The response does not expose the full URL, resolved addresses, internal hostnames, or lower-level reqwest diagnostics.

### `GET /api/v1/tools`

- **Auth:** JWT
- **Description:** List tools for current tenant

**Query Parameters:**
| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `page` | number | 1 | Page number |
| `per_page` | number | 20 | Items per page |
| `search` | string | — | Optional search filter |

**Response:**
```typescript
ApiResponse<ToolListResponse>

interface ToolListResponse {
  tools: ToolResponse[];
  page: number;
  per_page: number;
  total: number;
}
```

---

### `POST /api/v1/tools`

- **Auth:** Same-tenant Owner/Admin JWT only
- **Authorization order:** Authentication and role checks run before request-body deserialization.
- **Description:** Create a new Tool. HTTP Tool configuration must pass the stable egress validation above.

**Request:**
```typescript
interface CreateToolRequest {
  name: string;             // 1-128 chars
  description: string;      // min 1 char
  input_schema: object;     // JSON Schema
  type?: "http" | "function"; // default: "function"
  handler_url?: string;     // required for type="http"; production requires https
  handler_method?: string;  // GET|POST|PUT|DELETE|PATCH|HEAD|OPTIONS
  handler_timeout?: number; // 1..=30000 ms
  is_public?: boolean;      // default: false
}
```

**Response:** `201 Created`

```typescript
ApiResponse<ToolResponse>
```

**Errors:**

- `403 FORBIDDEN` — caller is a Member or authenticated with an API Key. This is returned before parsing the body, including for malformed JSON.
- `400 VALIDATION_ERROR` — DTO validation fails, handler type/method/timeout is unsupported, or the HTTP target is invalid/disallowed.
- `500 DATABASE_ERROR` / `500 INTERNAL_ERROR` — persistence or unexpected server failure.

---

### `GET /api/v1/tools/{id}`

- **Auth:** JWT
- **Description:** Get a same-tenant Tool by ID. A missing or foreign Tool returns `404 NOT_FOUND`.

**Response:**
```typescript
ApiResponse<ToolResponse>
```

---

### `PUT /api/v1/tools/{id}`

- **Auth:** Same-tenant Owner/Admin JWT only
- **Authorization order:** Authentication and role checks run before request-body deserialization and before Tool lookup.
- **Description:** Partially update a Tool. Handler fields are merged with the existing handler and the resulting configuration is revalidated before persistence.

**Request:**
```typescript
interface UpdateToolRequest {
  name?: string;
  description?: string;
  input_schema?: object;
  type?: "http" | "function";
  handler_url?: string;
  handler_method?: string;
  handler_timeout?: number; // 1..=30000 ms
  is_public?: boolean;
}
```

**Response:**
```typescript
ApiResponse<ToolResponse>
```

**Errors:**

- `403 FORBIDDEN` — caller is a Member or authenticated with an API Key. This is returned before parsing the body or looking up the Tool.
- `404 NOT_FOUND` — Tool is missing or belongs to another tenant. Both cases return the same `Tool not found or access denied` response after the Owner/Admin JWT gate passes.
- `400 VALIDATION_ERROR` — DTO validation fails or the resulting handler configuration is invalid/disallowed.
- `500 DATABASE_ERROR` / `500 INTERNAL_ERROR` — persistence or unexpected server failure.

---

### `DELETE /api/v1/tools/{id}`

- **Auth:** Authenticated same-tenant caller (legacy behavior; not changed by EVO-118-C)
- **Description:** Delete a Tool. Missing and foreign Tools return `404 NOT_FOUND`.

> This endpoint does not currently share the Owner/Admin-only create/update gate. Any future role tightening requires a separate implementation and contract change.

**Response:**
```typescript
ApiResponse<MessageResponse>
```

---

### Tool Types

```typescript
interface ToolResponse {
  id: string;           // UUID
  name: string;
  description: string;
  category: string;     // always "custom"
  schema: object;       // JSON Schema
  handler: {
    handler_type: string;
    url?: string;
    method?: string;
    timeout?: number;
  };
  is_public: boolean;
  owner_id: string;     // UUID
  tenant_id: string;    // UUID
  created_at: string;   // ISO 8601
  updated_at: string;   // ISO 8601
}
```

---

## Skills

### `GET /api/v1/skills`

- **Auth:** JWT
- **Description:** List skills

**Query Parameters:**
| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `search` | string | — | Optional search filter |
| `is_public` | boolean | — | Filter by visibility |
| `owner_id` | string | — | Filter by owner UUID |
| `page` | number | 1 | Page number |
| `per_page` | number | 20 | Items per page |

**Response:**
```typescript
ApiResponse<{
  skills: SkillResponse[];
  page: number;
  per_page: number;
  total: number;
}>
```

---

### `POST /api/v1/skills`

- **Auth:** JWT
- **Description:** Create a new skill from direct `SKILL.md` content. Multi-source creation is planned under EVO-027.

**Request:**
```typescript
interface CreateSkillRequest {
  name: string;           // 1-128 chars
  version: string;        // 1-32 chars (semver)
  description: string;    // min 1 char
  content: string;        // min 1 char, SKILL.md format
  runtime: string;        // "python311"|"python"|"py" | "node20"|"node"|"javascript"|"js" | "wasm"|"webassembly"
  dependencies?: DependencyInput[];  // default: []
  is_public?: boolean;    // default: false
}

interface DependencyInput {
  name: string;
  version: string;
}
```

**Response:**
```typescript
ApiResponse<SkillResponse>
```

---

### Planned Skill Import Endpoints ⚠️ Not implemented

These endpoints define the target contract for EVO-027 and EVO-028. They are not available in the current service.

| Endpoint | Description |
|----------|-------------|
| `POST /api/v1/skills/import/zip` | Upload a ZIP package containing a standard Skill directory |
| `POST /api/v1/skills/import/git` | Import or preview a Skill from a Git repository, branch/tag/commit, and optional subdirectory |
| `POST /api/v1/skills/import/skillhub` | Sync a Skill from SkillHub or a compatible registry |
| `POST /api/v1/skills/validate` | Validate direct `SKILL.md` content or a staged package without creating a Skill |
| `GET /api/v1/skills/{id}/versions` | List versions for a Skill |
| `GET /api/v1/skills/{id}/versions/{version}` | Get one version and its validation report |
| `POST /api/v1/skills/{id}/versions/{version}/default` | Set the default version |
| `POST /api/v1/skills/{id}/versions/{version}/rollback` | Roll back the default version pointer |

Planned validation baseline:

- A Skill package must contain `SKILL.md`.
- `SKILL.md` must contain YAML frontmatter and Markdown body.
- Frontmatter must include `name` and `description`.
- `name` must be 1-64 characters, lowercase alphanumeric or hyphen only, must not start or end with a hyphen, must not contain consecutive hyphens, and must match the Skill directory name.
- `description` must be 1-1024 characters and should explain both capability and activation conditions.
- `scripts/`, `references/`, `assets/`, `license`, `compatibility`, `metadata`, and `allowed-tools` must be parsed for compatibility even if Evolith does not execute or enforce all of them initially.

Planned import response:

```typescript
interface SkillImportReport {
  source_type: 'manual' | 'zip' | 'git' | 'skillhub';
  source_ref?: string;
  skill_id?: string;
  skill_name?: string;
  version?: string;
  status: 'accepted' | 'accepted_with_warnings' | 'rejected' | 'draft';
  files: string[];
  errors: SkillValidationIssue[];
  warnings: SkillValidationIssue[];
}

interface SkillValidationIssue {
  code: string;
  message: string;
  path?: string;
  line?: number;
}
```

---

### `GET /api/v1/skills/{id}`

- **Auth:** JWT
- **Description:** Get skill by ID

**Response:**
```typescript
ApiResponse<SkillResponse>
```

---

### `PUT /api/v1/skills/{id}`

- **Auth:** JWT
- **Description:** Update skill. Supports partial updates — only provided fields are changed. Tenant isolation enforced (cross-tenant skill returns 404). Owner or admin can update.

**Request:**
```typescript
interface UpdateSkillRequest {
  name?: string;
  version?: string;
  description?: string;
  content?: string;
  runtime?: string;
  dependencies?: DependencyInput[];
  is_public?: boolean;
}
```

**Response:**
```typescript
ApiResponse<SkillResponse>
```

**Errors:**
- `NOT_FOUND` — skill does not exist or belongs to another tenant
- `FORBIDDEN` — user is not the owner or admin
- `INVALID_RUNTIME` — unknown runtime string

---

### `DELETE /api/v1/skills/{id}`

- **Auth:** JWT
- **Description:** Delete skill

**Response:**
```typescript
ApiResponse<MessageResponse>
```

---

### `GET /api/v1/skills/{id}/load`

- **Auth:** JWT
- **Description:** Load skill for execution (returns minimal info for runtime)

**Response:**
```typescript
ApiResponse<SkillLoadResponse>

interface SkillLoadResponse {
  id: string;
  name: string;
  version: string;
  content: string;
  runtime: string;
}
```

---

### `POST /api/v1/skills/{id}/execute`

- **Auth:** JWT
- **Status:** Legacy / pending removal by [ADR-0005](../decisions/ADR-0005-deprecate-sandbox-runtime.md) and [EVO-111](../backlog/active/EVO-111-deprecate-sandbox-runtime.md).
- **Description:** Execute a skill in a sandboxed Docker container. This is existing compatibility behavior only; Git-Centric Platform work must not add new dependencies on this route. Requires sandbox to be enabled on the server.
- **RBAC:** Authenticated users can execute skills belonging to their tenant or public skills.

**Request:**
```typescript
interface ExecuteSkillRequest {
  parameters?: object;   // Runtime parameters passed as JSON env vars (default: {})
  code?: string;         // Optional: override skill's stored code for ad-hoc execution
}
```

**Response (200 — execution completed):**
```typescript
ApiResponse<SkillExecutionResponse>

interface SkillExecutionResponse {
  status: string;           // "success" | "error" | "timeout"
  skill_id: string;         // UUID
  skill_name: string;
  runtime: string;          // "python311" | "node20"
  output: string;           // stdout from execution
  errors: string;           // stderr from execution
  exit_code: number;        // Process exit code (-1 if killed)
  execution_time_ms: number;
  timed_out: boolean;
}
```

**Error Responses:**
- `404` — Skill not found
- `400` — Sandbox disabled or unsupported runtime
- `500` — Docker execution failure

**Example:**
```bash
curl -X POST http://localhost:8080/api/v1/skills/{id}/execute \
  -H "Content-Type: application/json" \
  -H "X-CSRF-Token: {csrf_token}" \
  --cookie "evolith_token={jwt}" \
  -d '{"parameters": {"name": "world"}, "code": "print(\"Hello, \" + __import__(\"os\").environ.get(\"PARAM_name\", \"default\"))"}'
```

**Notes:**
- Parameters are injected as environment variables prefixed with `PARAM_` (e.g., `parameters.name` → `PARAM_name`)
- Execution is subject to sandbox resource limits (CPU, memory, timeout, network, PID count)
- Output is truncated at `max_output_bytes` (default 10MB)
- Container is automatically removed after execution

---

### Skill Types

```typescript
interface SkillResponse {
  id: string;
  name: string;
  version: string;
  description: string;
  content: string;       // SKILL.md content
  runtime: string;       // "python311" | "node20" | "wasm"
  dependencies: DependencyResponse[];
  is_public: boolean;
  category: string;      // always ""
  tags: string[];        // always []
  owner_id: string;      // UUID
  tenant_id: string;     // UUID
  created_at: string;    // ISO 8601
  updated_at: string;    // ISO 8601
}

interface DependencyResponse {
  name: string;
  version: string;
}
```

---

## Snippets

> Deprecated planning note: snippet remains in the current implementation and contract for compatibility,
> but new product work should target CLI-friendly interfaces instead. See
> [ADR-0002](../decisions/ADR-0002-cli-friendly-interface-replaces-snippet.md) and backlog item EVO-017.

### `GET /api/v1/snippets`

- **Auth:** JWT
- **Description:** List all snippets for tenant

**Response:**
```typescript
ApiResponse<{
  snippets: SnippetResponse[];
  total: number;
}>
```

---

### `POST /api/v1/snippets`

- **Auth:** JWT
- **Description:** Create a new snippet

**Request:**
```typescript
interface CreateSnippetRequest {
  name: string;           // 1-128 chars
  language: string;       // 1-32 chars
  framework?: string;
  tags?: string[];        // default: []
  content: string;        // min 1 char, markdown documentation
  code: string;           // min 1 char
  dependencies?: SnippetDependencyInput[];  // default: []
  estimated_tokens?: number;
  is_public?: boolean;    // default: false
}

interface SnippetDependencyInput {
  name: string;
  version: string;
  required?: boolean;     // default: false
}
```

**Response:**
```typescript
ApiResponse<SnippetResponse>
```

---

### `GET /api/v1/snippets/{id}`

- **Auth:** JWT
- **Description:** Get snippet by ID

**Response:**
```typescript
ApiResponse<SnippetResponse>
```

---

### `PUT /api/v1/snippets/{id}` ⚠️ Not implemented (501)

- **Auth:** JWT
- **Description:** Update snippet

**Request:**
```typescript
interface UpdateSnippetRequest {
  name?: string;
  language?: string;
  framework?: string;
  tags?: string[];
  content?: string;
  code?: string;
  dependencies?: SnippetDependencyInput[];
  estimated_tokens?: number;
  is_public?: boolean;
}
```

**Response:** 501 NOT_IMPLEMENTED

---

### `DELETE /api/v1/snippets/{id}`

- **Auth:** JWT
- **Description:** Delete snippet

**Response:**
```typescript
ApiResponse<MessageResponse>
```

---

### `GET /api/v1/snippets/search`

- **Auth:** JWT
- **Description:** Search snippets

**Query Parameters:**
| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `q` | string | — | Search query |
| `language` | string | — | Filter by language |
| `framework` | string | — | Filter by framework |

**Response:**
```typescript
ApiResponse<{
  snippets: SnippetResponse[];
  total: number;
}>
```

---

### `GET /api/v1/snippets/{id}/reference`

- **Auth:** JWT
- **Description:** Get LLM-optimized snippet reference

**Response:**
```typescript
ApiResponse<SnippetReferenceResponse>

interface SnippetReferenceResponse {
  name: string;
  language: string;
  framework?: string;
  content: string;
  code: string;
  estimated_tokens: number;
}
```

---

### Snippet Types

```typescript
interface SnippetResponse {
  id: string;
  title: string;          // mapped from domain 'name'
  description: string;    // always ""
  language: string;
  framework?: string;
  tags: string[];
  content: string;        // markdown documentation
  code: string;
  dependencies: SnippetDependencyResponse[];
  estimated_tokens: number;
  is_public: boolean;
  owner_id: string;       // UUID
  tenant_id: string;      // UUID
  created_at: string;     // ISO 8601
  updated_at: string;     // ISO 8601
}

interface SnippetDependencyResponse {
  name: string;
  version: string;
  required: boolean;
}
```

---

## CLI Interfaces

> Planned replacement concept for Snippets. Current backend routes still use
> `/api/v1/snippets` for compatibility; new product language, documents, and
> parser logic use `CliInterface`. Route-level rename is intentionally deferred
> until the migration can cover backend, frontend, database compatibility, and
> release notes in one story.

### Compatibility Strategy

| Phase | API Path | Status | Notes |
|-------|----------|--------|-------|
| Current | `/api/v1/snippets` | Implemented compatibility path | Stores existing rows and can carry CLI interface metadata in `content` / `code` while migration proceeds |
| Planned | `/api/v1/cli-interfaces` | Not implemented | New canonical path after repository, DTO, frontend, and migration strategy are ready |

### `CliInterface` Document Shape

CLI interface documents use YAML frontmatter + Markdown body. The canonical
format is documented in [CLI 友好接口格式规范](formats/CLI-INTERFACE-FORMAT.md).

```typescript
interface CliInterfaceDocument {
  metadata: CliInterfaceMetadata;
  body: string; // Markdown usage, notes, constraints
}

interface CliInterfaceMetadata {
  name: string;
  version: string;
  summary: string;
  command: string;
  subcommands?: string[];
  tags?: string[];
  visibility?: "private" | "tenant" | "public";
  inputs?: CliInterfaceParameter[];
  output?: CliInterfaceOutput;
  examples?: CliInterfaceExample[];
  error_model?: CliInterfaceError[];
}

interface CliInterfaceParameter {
  name: string;
  type: "string" | "number" | "boolean" | "enum" | "object" | "array";
  required?: boolean;
  description?: string;
  values?: string[];
  default?: unknown;
}

interface CliInterfaceOutput {
  type: string;
  description?: string;
}

interface CliInterfaceExample {
  title: string;
  command: string;
  input?: unknown;
  output?: unknown;
}

interface CliInterfaceError {
  code: string;
  message: string;
  retryable?: boolean;
}
```

### Legacy Mapping

| Snippet Field | CLI Interface Meaning | Current Handling |
|---------------|-----------------------|------------------|
| `name` / `title` | `metadata.name` | Keep as stable interface name |
| `content` | Markdown body | Keep as usage/notes during compatibility phase |
| `code` | `metadata.command` or example command | Stop treating as insertable source code for new work |
| `language` | tag or runtime hint | Keep for compatibility; no longer canonical taxonomy |
| `framework` | tag | Keep for compatibility |
| `dependencies` | future runtime/dependency metadata | Keep as legacy field until migration story |

---

## Members

### `GET /api/v1/tenant/{tenant_id}/members`

- **Auth:** JWT
- **Description:** List members of a tenant

**Response:**
```typescript
ApiResponse<MemberListResponse>

interface MemberListResponse {
  members: MemberInfo[];
  total: number;
}
```

---

### `POST /api/v1/tenant/{tenant_id}/members/invite`

- **Auth:** JWT (admin only)
- **Description:** Invite a member and send an invitation email when mailer is configured. The response also includes the relative join URL for admin copy/paste fallback.

**Request:**
```typescript
interface InviteMemberRequest {
  email: string;    // email format
  role: string;     // "admin" | "member"
  message?: string;
}
```

**Response:**
```typescript
ApiResponse<InviteMemberResponse>

interface InviteMemberResponse {
  id: string;
  email: string;
  role: string;
  invite_url: string;    // "/join?token=..."
  expires_at: string;
}
```

---

### `GET /api/v1/tenant/{tenant_id}/members/invitations`

- **Auth:** JWT
- **Description:** List pending invitations

**Response:**
```typescript
ApiResponse<InvitationListResponse>

interface InvitationListResponse {
  invitations: InvitationInfo[];
  total: number;
}

interface InvitationInfo {
  id: string;
  email: string;
  role: string;
  status: string;        // "pending" | "expired"
  invited_by: string;    // UUID
  expires_at: string;    // ISO 8601
  created_at: string;    // ISO 8601
}
```

---

### `DELETE /api/v1/tenant/{tenant_id}/members/{member_id}`

- **Auth:** JWT (admin only)
- **Description:** Remove member from tenant

**Response:**
```typescript
ApiResponse<MessageResponse>
```

---

### `POST /api/v1/invitations/accept`

- **Auth:** None
- **Description:** Accept invitation and create the invited account. This is the canonical public endpoint used by the SPA join page.

**Request:**
```typescript
interface AcceptInviteRequest {
  token: string;
  password: string;  // 8-128 chars
  username: string;  // 3-50 chars
}
```

**Response:**
```typescript
ApiResponse<AuthResponseData>
```

**Errors:**
- `INVALID_TOKEN` — token does not exist
- `TOKEN_USED` — invitation already accepted
- `TOKEN_EXPIRED` — invitation expired
- `EMAIL_EXISTS` — the invited email already has an account
- `WEAK_PASSWORD` — password does not meet password policy

---

### `POST /api/v1/tenant/{tenant_id}/members/join`

- **Auth:** None
- **Description:** Compatibility alias for accepting invitations. Prefer `POST /api/v1/invitations/accept` because the token already identifies the tenant.

**Request:** same as `POST /api/v1/invitations/accept`

**Response:** same as `POST /api/v1/invitations/accept`

---

### Member Types

```typescript
interface MemberInfo {
  id: string;
  email: string;
  username: string;
  full_name?: string;
  role: string;
  tenant_role: string;
  status: string;
  joined_at?: string;     // ISO 8601
  last_login_at?: string; // ISO 8601
  avatar_url?: string;
}
```

---

## API Keys

### `GET /api/v1/tenant/{tenant_id}/api-keys`

- **Auth:** JWT
- **Description:** List API keys for tenant

**Response:**
```typescript
ApiResponse<ApiKeyListResponse>

interface ApiKeyListResponse {
  keys: ApiKeyListItem[];
  total: number;
}

interface ApiKeyListItem {
  id: string;
  name: string;
  key_prefix: string;     // e.g. "evo_sk_a1b2c3d4..."
  permissions: string[];
  expires_at?: string;    // ISO 8601
  rate_limit: number;
  status: string;         // "active" | "revoked" | "expired"
  request_count: number;
  last_used_at?: string;  // ISO 8601
  created_at: string;     // ISO 8601
}
```

---

### `POST /api/v1/tenant/{tenant_id}/api-keys`

- **Auth:** JWT
- **Description:** Create API key

**Request:**
```typescript
interface CreateApiKeyRequest {
  name: string;            // 1-128 chars
  permissions?: string[];  // ["read","write","admin"], default: ["read"]
  expires_in_days?: number;
  rate_limit?: number;     // per hour
}
```

**Response:**
```typescript
ApiResponse<ApiKeyResponse>

interface ApiKeyResponse {
  id: string;
  name: string;
  key?: string;           // ⚠️ Only shown ONCE on creation
  key_prefix: string;
  permissions: string[];
  expires_at?: string;    // ISO 8601
  rate_limit: number;
  status: string;         // "active" | "revoked" | "expired"
  created_at: string;     // ISO 8601
}
```

---

### `DELETE /api/v1/tenant/{tenant_id}/api-keys/{key_id}`

- **Auth:** JWT
- **Description:** Revoke API key

**Request (optional body):**
```typescript
interface RevokeApiKeyRequest {
  reason?: string;
}
```

**Response:**
```typescript
ApiResponse<MessageResponse>
```

---

## Billing

> All billing endpoints currently return stub/mock data. Full Stripe integration is not yet
> implemented. The frontend billing page is gated behind a `BILLING_ENABLED` flag and renders a
> "待计费" placeholder until the backend service-payment routes are wired up
> (see EVO-055 / Iteration 035). Until then, do not consume billing endpoints from production paths.

### `GET /api/v1/tenant/{tenant_id}/billing/plans`

- **Auth:** Tenant access
- **Description:** List available plans

**Response:**
```typescript
ApiResponse<PlanListResponse>

interface PlanListResponse {
  plans: PlanInfo[];
}

interface PlanInfo {
  id: string;
  name: string;
  display_name: string;
  description?: string;
  monthly_price: number;
  yearly_price?: number;
  max_users: number;
  max_tools: number;
  max_skills: number;
  max_snippets: number;
  max_api_calls_per_month: number;
  max_storage_mb: number;
  features: object;
  is_builtin: boolean;
}
```

---

### `GET /api/v1/tenant/{tenant_id}/billing/subscription`

- **Auth:** JWT
- **Description:** Get current subscription

**Response:**
```typescript
ApiResponse<GetSubscriptionResponse>

interface GetSubscriptionResponse {
  subscription?: SubscriptionInfo;
}

interface SubscriptionInfo {
  id: string;
  plan: PlanInfo;
  status: string;
  billing_cycle: string;
  current_period_start: string;  // ISO 8601
  current_period_end: string;    // ISO 8601
  trial_end_at?: string;         // ISO 8601
  cancel_at_period_end: boolean;
}
```

---

### `POST /api/v1/tenant/{tenant_id}/billing/subscription/checkout`

- **Auth:** JWT
- **Description:** Create checkout session

**Request:**
```typescript
interface CreateSubscriptionRequest {
  plan_id: string;
  billing_cycle?: string;       // "monthly" | "yearly"
  payment_method_id?: string;
}
```

**Response:**
```typescript
ApiResponse<{
  checkout_url: string;
  session_id: string;
  plan: PlanInfo;
  billing_cycle: string;
}>
```

---

### `POST /api/v1/tenant/{tenant_id}/billing/subscription/manage`

- **Auth:** JWT
- **Description:** Manage subscription

**Response:**
```typescript
ApiResponse<{
  management_url: string;
  message: string;
}>
```

---

### `GET /api/v1/tenant/{tenant_id}/billing/usage`

- **Auth:** JWT
- **Description:** Get usage for current billing period

**Response:**
```typescript
ApiResponse<UsageResponse>

interface UsageResponse {
  period: {
    start: string;          // ISO 8601
    end: string;            // ISO 8601
    remaining_days: number;
  };
  resources: ResourceUsage[];
}

interface ResourceUsage {
  resource_type: string;
  used: number;
  limit: number;
  percent: number;
  remaining?: number;
  overage?: number;
}
```

---

## Repos

> Stable as of EVO-116. The Repo CRUD endpoints (EVO-103-A), Smart HTTP
> git protocol endpoints (EVO-103-B-1/B-2/EVO-115), and Repo Context read
> endpoints (EVO-103-C) are documented together here because the path
> hierarchy shares a single `/repos/{id}` namespace.
>
> **Auth scope (EVO-116)**: every route accepts either a JWT session
> cookie or an API key. API keys must carry a permission that matches
> the route's scope (see [API key scope](#api-key-scope-eVO-116)).
> When an API key lacks the required permission, the response is
> `403 FORBIDDEN` with `code: "FORBIDDEN"`.

### `POST /api/v1/tenant/{tenant_id}/repos`

- **Auth:** JWT or API key with `repo:write` / `write` / `admin` / `commit` / `commit:*`
- **CSRF:** required when using cookie auth
- **Description:** Create a new git repo in the tenant. The server
  `gix::init_bare`s the repo on disk and (if `seed_template: true`)
  writes `.evolith/agents.yaml`, `.evolith/policy.yaml`, and
  `README.md` at the default branch.

**Request:**
```typescript
interface CreateRepoRequest {
  name: string;              // 1-128 chars, unique per tenant
  description?: string;
  default_branch?: string;   // default "main"
  visibility?: "public" | "private";  // default "private"
  auto_merge?: boolean;      // default false
  require_review?: boolean;  // default true
  seed_template?: boolean;   // default false
}
```

**Response:** `201 Created` with `ApiResponse<RepoResponse>` (see [Repo Types](#repo-types-eVO-103)).

**Errors:**
- `400 VALIDATION_ERROR` — invalid name
- `403 FORBIDDEN` — cross-tenant or API key lacks `repo:write`
- `409 REPO_EXISTS` — a repo with this name already exists in the tenant

---

### `GET /api/v1/tenant/{tenant_id}/repos`

- **Auth:** JWT or API key with `repo:read` / `write` / `admin` / `commit` / `commit:*`
- **CSRF:** not required (GET)
- **Description:** List all repos for the tenant.

**Response:**
```typescript
ApiResponse<RepoListResponse>
interface RepoListResponse {
  repos: RepoResponse[];
  total: number;
}
```

---

### `GET /api/v1/tenant/{tenant_id}/repos/{repo_id}`

- **Auth:** JWT or API key with `repo:read` (or any superset)
- **CSRF:** not required (GET)
- **Description:** Get a single repo by id.

**Response:** `ApiResponse<RepoResponse>`.

**Errors:**
- `403 FORBIDDEN` — cross-tenant access
- `404 NOT_FOUND` — repo does not exist

---

### `PATCH /api/v1/tenant/{tenant_id}/repos/{repo_id}`

- **Auth:** JWT or API key with `repo:write` / `write` / `admin` / `commit` / `commit:*`
- **CSRF:** required when using cookie auth
- **Description:** Update repo metadata (name, description, default_branch,
  visibility, auto_merge, require_review).

**Request:** `UpdateRepoRequest` (all fields optional).

**Response:** `ApiResponse<RepoResponse>`.

---

### `DELETE /api/v1/tenant/{tenant_id}/repos/{repo_id}`

- **Auth:** JWT or API key with `repo:write` / `write` / `admin` / `commit` / `commit:*`
- **CSRF:** required when using cookie auth
- **Description:** Delete the repo (DB row + on-disk bare repo).
- **Response:** `204 No Content`.

---

### Repo Types <a id="repo-types-eVO-103"></a>

```typescript
interface RepoResponse {
  id: string;                    // UUID
  tenant_id: string;             // UUID
  name: string;
  description: string;
  default_branch: string;
  storage_path: string;          // "{tenant_id}/{repo_id}.git" (mirror of disk path)
  visibility: "public" | "private";
  auto_merge: boolean;
  require_review: boolean;
  last_commit_sha: string | null;    // updated after each successful push to default branch
  last_committed_at: string | null;  // ISO 8601
  created_at: string;            // ISO 8601
  updated_at: string;            // ISO 8601
}
```

---

## Smart HTTP (Git Protocol) <a id="smart-http-eVO-103"></a>

> Standard `git` protocol over HTTP. Standard clients (CLI, libgit2,
> Tower, SourceTree) talk to these endpoints without modification.
> Implemented via the `git` subprocess per [ADR-0006](../decisions/ADR-0006-smart-http-via-git-subprocess.md).

### Authentication

- **Basic-Auth**: `Authorization: Basic base64("any:API_KEY")` — git
  clients send the password field as the API key. The first
  unauthorized response includes `WWW-Authenticate: Basic realm="evolith"`
  so the client retries with URL credentials.
- **X-API-Key header**: `X-API-Key: evo_sk_…` for non-git HTTP clients.
- **JWT cookie**: the SPA does not normally use these endpoints.

### Permission matrix

| Endpoint                                | Required API key scope |
|-----------------------------------------|------------------------|
| `GET /repos/{id}/info/refs`             | `repo:read` (or write/admin) |
| `POST /repos/{id}/git-upload-pack`      | `repo:read` (or write/admin) |
| `POST /repos/{id}/git-receive-pack`     | `repo:write` / `write` / `admin` / `commit` / `commit:*` |

A read-only API key calling `git-receive-pack` receives
`403 Forbidden` ("API key lacks required repository permission") without
the git subprocess being spawned.

### `GET /repos/{repo_id}/info/refs?service=git-upload-pack|git-receive-pack`

- **Auth:** JWT or API key (scope per matrix above)
- **CSRF:** not required (GET)
- **Response:** `Content-Type: application/x-git-{upload,receive}-pack-advertisement` with Smart HTTP pkt-line advertisement.

### `POST /repos/{repo_id}/git-upload-pack`

- **Auth:** JWT or API key with `repo:read` (or write/admin)
- **CSRF:** exempt (standard git clients do not send CSRF)
- **Description:** Streaming clone/fetch endpoint. Pipes request body
  to `git upload-pack --stateless-rpc <path>` stdin and streams stdout
  to the response. Subprocess timeout: 30s.

### `POST /repos/{repo_id}/git-receive-pack`

- **Auth:** JWT or API key with `repo:write` / `write` / `admin` / `commit` / `commit:*`
- **CSRF:** exempt (standard git clients do not send CSRF)
- **Description:** Streaming push endpoint. Pipes request body to
  `git receive-pack --stateless-rpc <path>` stdin and streams stdout
  to the response. Subprocess timeout: 30s. After a successful push
  to the default branch, the server updates
  `git_repos.last_commit_sha` / `last_committed_at` in the background
  (see [Push metadata sync](#push-metadata-sync-eVO-116)); a failure
  to update metadata is logged but does not fail the push.

### `Content-Type` advertisement types

- `application/x-git-upload-pack-advertisement` (refs advertisement)
- `application/x-git-upload-pack-result` (pack data stream)
- `application/x-git-receive-pack-advertisement`
- `application/x-git-receive-pack-result`

### Subprocess safety

- Fixed `Command::new("git")` with an explicit argument vector; no shell.
- `repo_path` always comes from `service_git::repo_path(base, tenant_id, repo_id)` — never from URL.
- 30-second wall-clock timeout via `tokio::time::timeout` + `kill_on_drop(true)` + stderr drain.
- `git_service` parameter is enum-validated (`git-upload-pack` | `git-receive-pack`); other values are rejected.

---

## Repo Context API <a id="repo-context-eVO-103-c"></a>

> Read-only git queries over the bare repo. Backed by the `gix` crate.
> Implemented in `service-git` and exposed via `repo_context_handlers`.

### Authentication and scope

- **Auth:** JWT or API key with `repo:read` (or any superset).
- An `execute`-only or unrelated API key receives `403 FORBIDDEN`; the
  repo path and ref resolution error are not leaked.

### Resource bounds (EVO-116)

| Resource                | Limit                        | On exceed |
|-------------------------|------------------------------|-----------|
| Blob response size      | `BLOB_MAX_BYTES` = 1 MiB     | `413 RESOURCE_EXCEEDED` (object header checked before blob body load) |
| File-tree entries       | `FILE_TREE_MAX_ENTRIES` = 5000 | `413 RESOURCE_EXCEEDED` |
| Diff entries            | `DIFF_MAX_ENTRIES` = 5000     | `413 RESOURCE_EXCEEDED` |
| Blocking gix read       | `CONTEXT_BLOCKING_TIMEOUT` = 5s | `504 TIMEOUT` (returns control to client) |
| Commits limit (existing)| clamped to `[1, 100]`        | clamp only |

### Error mapping

| gix failure                     | HTTP status | Code                |
|---------------------------------|-------------|---------------------|
| Ref not found (file-tree, diff) | 404         | `NOT_FOUND`         |
| Invalid blob SHA hex            | 400         | `INVALID_INPUT`     |
| Empty blob SHA                  | 400         | `INVALID_INPUT`     |
| Blob SHA not found              | 404         | `NOT_FOUND`         |
| Resource bound exceeded         | 413         | `RESOURCE_EXCEEDED` |
| Blocking gix read timeout       | 504         | `TIMEOUT`           |
| Any other gix read error        | 500         | `INTERNAL_ERROR`    |

### `GET /api/v1/tenant/{tenant_id}/repos/{repo_id}/file-tree?ref={ref}`

- **Auth:** JWT or API key with `repo:read`
- **CSRF:** not required (GET)
- **Description:** Breadth-first recursive listing of the tree at `ref`.
  Default `ref` is `repo.default_branch`.

**Response:**
```typescript
ApiResponse<FileTreeResponse>
interface FileTreeResponse {
  entries: FileTreeEntryDto[];
}
interface FileTreeEntryDto {
  name: string;        // path relative to repo root
  kind: "blob" | "tree";
  oid: string;         // 40-char hex
  is_tree: boolean;
}
```

---

### `GET /api/v1/tenant/{tenant_id}/repos/{repo_id}/blobs/{sha}`

- **Auth:** JWT or API key with `repo:read`
- **CSRF:** not required (GET)
- **Description:** Return blob content at `sha`. The server checks the
  object header first; if its size exceeds `BLOB_MAX_BYTES`, it returns
  `413` before loading the blob body via `gix::find_blob`.

**Response:**
```typescript
ApiResponse<BlobResponse>
interface BlobResponse {
  content: string;     // utf-8 text or base64 (see encoding)
  size: number;        // bytes
  encoding: "utf-8" | "base64";
}
```

The server chooses `base64` for blobs containing NUL bytes, `utf-8` otherwise.

**Errors:**
- `400 INVALID_INPUT` — `sha` is empty or not a 40-char hex string
- `404 NOT_FOUND` — blob does not exist in the repo
- `413 RESOURCE_EXCEEDED` — blob size > `BLOB_MAX_BYTES`

---

### `GET /api/v1/tenant/{tenant_id}/repos/{repo_id}/commits?ref={ref}&limit={n}`

- **Auth:** JWT or API key with `repo:read`
- **CSRF:** not required (GET)
- **Description:** Walk ancestors of `ref` (default = default branch),
  newest first, up to `limit` (default 50, clamped to 100).

**Response:**
```typescript
ApiResponse<CommitListResponse>
interface CommitListResponse {
  commits: CommitDto[];
}
interface CommitDto {
  sha: string;
  author_name: string;
  author_email: string;
  message: string;
  timestamp: number;     // unix seconds
}
```

---

### `GET /api/v1/tenant/{tenant_id}/repos/{repo_id}/diff?base={ref}&head={ref}`

- **Auth:** JWT or API key with `repo:read`
- **CSRF:** not required (GET)
- **Description:** Tree diff between `base` and `head`. Refs may be
  branch shorthand (`main`) or fully-qualified (`refs/heads/main`)
  or commit SHAs. `gix` tree change iteration is bounded by
  `DIFF_MAX_ENTRIES`.

**Response:**
```typescript
ApiResponse<DiffResponse>
interface DiffResponse {
  entries: DiffEntryDto[];
}
interface DiffEntryDto {
  path: string;
  change_type: "added" | "deleted" | "modified";
  old_oid: string;
  new_oid: string;
}
```

---

## API key scope <a id="api-key-scope-eVO-116"></a>

> Single source of truth: `backend/crates/api/src/middleware/api_key_scope.rs`.

| Permission token                     | Read | Write | Manage keys |
|--------------------------------------|:----:|:-----:|:-----------:|
| `read` / `repo:read`                 |  ✓   |       |             |
| `write` / `repo:write`               |  ✓   |  ✓    |             |
| `admin`                              |  ✓   |  ✓    |             |
| `commit` / `commit:*`                |  ✓   |  ✓    |             |
| `execute`                            |      |       |             |
| (empty / unrelated)                  |      |       |             |

**Notes:**
- **API keys are NEVER allowed to manage other API keys** regardless
  of permissions. The `/api-keys` routes are JWT-only; an API key
  caller receives `403 FORBIDDEN`.
- Read-only keys can `git clone` / `git fetch` (Smart HTTP read
  endpoints) and can call the Repo Context `GET` endpoints.
- Read-only keys CANNOT call `git push` (`git-receive-pack`) and
  CANNOT call Repo CRUD `POST/PATCH/DELETE`.

---

## Push metadata sync <a id="push-metadata-sync-eVO-116"></a>

> After a successful `git push` to the default branch via
> `POST /repos/{repo_id}/git-receive-pack`, the server resolves
> `refs/heads/{default_branch}` via gix and updates
> `git_repos.last_commit_sha` and `last_committed_at`.

- This runs after the git subprocess reports a successful receive-pack
  result in the same detached streaming task. The metadata write is
  bounded by `CONTEXT_BLOCKING_TIMEOUT`; failure is logged and not sent
  as a git protocol failure.
- Pushes to a non-default branch do NOT update default-branch metadata
  (per-branch metadata is future work).
- If the metadata update fails (e.g. resolve error, DB write error,
  timeout), the failure is logged at WARN level and the push response
  is NOT affected. The repo continues to function; the next successful
  push to the default branch will reconcile.
- Implementation: `git_smart_http_handlers::update_repo_metadata_after_push`.

---

## Audit Logs

### `GET /api/v1/tenant/{tenant_id}/audit-logs`

- **Auth:** JWT (admin only)
- **Description:** List audit logs

**Query Parameters:**
| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `limit` | number | 50 | Max results |
| `offset` | number | 0 | Offset for pagination |

**Response:**
```typescript
ApiResponse<AuditLogListResponse>

interface AuditLogListResponse {
  logs: AuditLogResponse[];
  total: number;
}

interface AuditLogResponse {
  id: string;
  tenant_id?: string;     // UUID
  user_id?: string;       // UUID
  action: string;
  resource_type?: string;
  resource_id?: string;
  details: object;
  ip_address?: string;
  user_agent?: string;
  created_at: string;     // ISO 8601
}
```

---

### `GET /api/v1/tenant/{tenant_id}/audit-logs/{log_id}` ⚠️ Not implemented (501)

- **Auth:** JWT (admin only)
- **Description:** Get audit log by ID

**Response:** 501 NOT_IMPLEMENTED

---

## Version History

- **2026-08-01:** EVO-118-C / Iteration 052 — documented the same-tenant Owner/Admin JWT-only Tool create/update boundary, auth-before-deserialization and auth-before-lookup ordering, HTTPS/public-target egress validation, supported methods, `1..=30000 ms` timeout, stable redacted `400`, auth-first `403`, foreign/missing update hiding, and the unchanged legacy delete behavior.
- **2026-06-26:** EVO-116 / Iteration 049 — added Repos, Smart HTTP, Repo Context API, API key scope, and Push metadata sync sections; documented `INVALID_INPUT`, `RESOURCE_EXCEEDED`, `TIMEOUT`, `REPO_EXISTS` error codes; added `/repos/{id}/git-upload-pack` and `/repos/{id}/git-receive-pack` to CSRF-exempt paths; fixed Audit Logs section placement during acceptance remediation. `cargo test --workspace` + `cargo clippy --workspace --all-targets -- -D warnings` green.
- **2026-05-27:** Removed ⚠️ markers from `send-verify`, `verify-email` (implemented in Iteration 009) and `PUT /skills/{id}` (implemented in Iteration 010). Added response shapes and error codes. Synced with EVO-040 / Iteration 021.
- **2026-03-15:** Phase 6 updates — Added Authentication section documenting cookie-based auth (httpOnly `evolith_token` + `csrf_token`), CSRF protection (double-submit cookie pattern), `X-CSRF-Token` header requirement. Added `CSRF_ERROR` to error codes. Updated login/register/logout/refresh descriptions to mention cookie behavior. Updated Table of Contents.
- **2026-03-15:** Phase 3 updates — Added `/health/live` and `/health/ready` endpoints. Added Common Headers section (`X-Request-ID`). Added Rate Limiting section (429 responses). Updated Table of Contents. Health endpoint now returns `status`+`version` directly (not wrapped in `ApiResponse`).
- **2026-03-14:** Full rewrite for production contract. All endpoints, DTOs, validation, and error codes updated to match handler implementations (Phase 0 complete). Stub endpoints marked with ⚠️. Table of Contents and consistent TypeScript-style formatting added.
- **2024-01-10:** Initial prototype contract (obsolete).
