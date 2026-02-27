# Evolith API Specification

## Overview

This document defines the API contract between the Evolith frontend (Next.js) and backend (Rust/Actix-web). All APIs follow RESTful conventions and use JSON for request/response bodies.

**Base URL**: `http://localhost:8080/api/v1`

## Authentication

Most endpoints require JWT authentication. Include the token in the `Authorization` header:

```
Authorization: Bearer <token>
```

## Common Types

### ApiResponse<T>
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
  retryable?: boolean;
  details?: Record<string, any>;
}

interface PaginationMeta {
  page: number;
  per_page: number;
  total: number;
  total_pages: number;
}
```

---

## Authentication Endpoints

### POST /auth/register

Register a new user account.

**Request:**
```json
{
  "email": "user@example.com",
  "username": "johndoe",
  "password": "SecurePass123!"
}
```

**Response (201):**
```json
{
  "success": true,
  "data": {
    "token": "eyJhbGciOiJIUzI1NiIs...",
    "expires_at": 1709000000,
    "user": {
      "id": "uuid",
      "email": "user@example.com",
      "username": "johndoe",
      "role": "user"
    }
  }
}
```

**Validation:**
- `email`: Valid email format, max 255 chars
- `username`: 3-50 chars, alphanumeric + underscore
- `password`: 8-128 chars, must contain uppercase, lowercase, digit, special char

---

### POST /auth/login

Authenticate user and get access token.

**Request:**
```json
{
  "email": "user@example.com",
  "password": "SecurePass123!"
}
```

**Response (200):**
```json
{
  "success": true,
  "data": {
    "token": "eyJhbGciOiJIUzI1NiIs...",
    "expires_at": 1709000000,
    "user": {
      "id": "uuid",
      "email": "user@example.com",
      "username": "johndoe",
      "role": "user"
    }
  }
}
```

**Errors:**
- 401: Invalid credentials

---

### POST /auth/logout

Invalidate current session.

**Headers:** `Authorization: Bearer <token>`

**Response (200):**
```json
{
  "success": true,
  "data": { "message": "Logged out successfully" }
}
```

---

### POST /auth/refresh

Refresh access token.

**Headers:** `Authorization: Bearer <token>`

**Response (200):**
```json
{
  "success": true,
  "data": {
    "token": "eyJhbGciOiJIUzI1NiIs...",
    "expires_at": 1709000000
  }
}
```

---

### GET /auth/me

Get current user info.

**Headers:** `Authorization: Bearer <token>`

**Response (200):**
```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "email": "user@example.com",
    "username": "johndoe",
    "role": "user",
    "created_at": "2024-01-01T00:00:00Z"
  }
}
```

---

## MCP Tools Endpoints

### GET /tools

List all available MCP tools.

**Query Parameters:**
- `page` (optional): Page number, default 1
- `per_page` (optional): Items per page, default 20
- `search` (optional): Search by name/description

**Headers:** `Authorization: Bearer <token>`

**Response (200):**
```json
{
  "success": true,
  "data": [
    {
      "id": "uuid",
      "name": "filesystem_read",
      "description": "Read files from the filesystem",
      "category": "file",
      "enabled": true,
      "created_at": "2024-01-01T00:00:00Z"
    }
  ],
  "meta": {
    "page": 1,
    "per_page": 20,
    "total": 100,
    "total_pages": 5
  }
}
```

---

### GET /tools/{id}

Get tool details by ID.

**Headers:** `Authorization: Bearer <token>`

**Response (200):**
```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "name": "filesystem_read",
    "description": "Read files from the filesystem",
    "category": "file",
    "enabled": true,
    "schema": {
      "type": "object",
      "properties": {
        "path": { "type": "string", "description": "File path to read" }
      },
      "required": ["path"]
    },
    "created_at": "2024-01-01T00:00:00Z"
  }
}
```

---

### POST /tools/{id}/execute

Execute a tool.

**Headers:** `Authorization: Bearer <token>`

**Request:**
```json
{
  "parameters": {
    "path": "/path/to/file"
  }
}
```

**Response (200):**
```json
{
  "success": true,
  "data": {
    "result": "file content here",
    "execution_time_ms": 150
  }
}
```

**Errors:**
- 400: Invalid parameters
- 404: Tool not found
- 422: Execution failed

---

## Skills Endpoints

### GET /skills

List all skills.

**Query Parameters:**
- `page` (optional): Page number, default 1
- `per_page` (optional): Items per page, default 20
- `search` (optional): Search by name/description
- `category` (optional): Filter by category

**Headers:** `Authorization: Bearer <token>`

**Response (200):**
```json
{
  "success": true,
  "data": [
    {
      "id": "uuid",
      "name": "code-review",
      "description": "Performs code review on given code",
      "category": "development",
      "version": "1.0.0",
      "author": "Evolith Team",
      "created_at": "2024-01-01T00:00:00Z"
    }
  ],
  "meta": { ... }
}
```

---

### GET /skills/{id}

Get skill details.

**Headers:** `Authorization: Bearer <token>`

**Response (200):**
```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "name": "code-review",
    "description": "Performs code review on given code",
    "category": "development",
    "version": "1.0.0",
    "author": "Evolith Team",
    "content": "# Code Review Skill\n\n## Description\n...",
    "tags": ["review", "code", "quality"],
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-15T00:00:00Z"
  }
}
```

---

### POST /skills/{id}/execute

Execute a skill.

**Headers:** `Authorization: Bearer <token>`

**Request:**
```json
{
  "code": "function hello() { return 'world'; }",
  "language": "javascript"
}
```

**Response (200):**
```json
{
  "success": true,
  "data": {
    "result": {
      "issues": [
        {
          "severity": "warning",
          "message": "Missing semicolon",
          "line": 1
        }
      ]
    },
    "execution_time_ms": 250
  }
}
```

---

### POST /skills

Create a new skill. (Admin only in future)

**Headers:** `Authorization: Bearer <token>`

**Request:**
```json
{
  "name": "my-skill",
  "description": "My custom skill",
  "category": "development",
  "content": "# My Skill\n\n...",
  "tags": ["custom", "utility"]
}
```

**Response (201):**
```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "name": "my-skill",
    ...
  }
}
```

---

## Snippets Endpoints

### GET /snippets

List code snippets.

**Query Parameters:**
- `page`, `per_page`: Pagination
- `search`: Search by title/content
- `language`: Filter by programming language
- `tags`: Filter by tags (comma-separated)

**Headers:** `Authorization: Bearer <token>`

**Response (200):**
```json
{
  "success": true,
  "data": [
    {
      "id": "uuid",
      "title": "React useState Hook",
      "description": "Basic useState example",
      "language": "typescript",
      "code": "const [state, setState] = useState(initial);",
      "tags": ["react", "hooks", "state"],
      "author": "johndoe",
      "created_at": "2024-01-01T00:00:00Z"
    }
  ],
  "meta": { ... }
}
```

---

### GET /snippets/{id}

Get snippet details.

**Headers:** `Authorization: Bearer <token>`

**Response (200):**
```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "title": "React useState Hook",
    "description": "Basic useState example",
    "language": "typescript",
    "code": "const [state, setState] = useState(initial);",
    "tags": ["react", "hooks", "state"],
    "author": "johndoe",
    "usage": "Use this for managing component state",
    "example": "const [count, setCount] = useState(0);",
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-15T00:00:00Z"
  }
}
```

---

### POST /snippets

Create a new snippet.

**Headers:** `Authorization: Bearer <token>`

**Request:**
```json
{
  "title": "My Snippet",
  "description": "A useful code snippet",
  "language": "typescript",
  "code": "console.log('Hello');",
  "tags": ["example", "logging"],
  "usage": "How to use this snippet",
  "example": "Full example code"
}
```

**Response (201):**
```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "title": "My Snippet",
    ...
  }
}
```

---

### PUT /snippets/{id}

Update a snippet.

**Headers:** `Authorization: Bearer <token>`

**Request:**
```json
{
  "title": "Updated Title",
  "description": "Updated description",
  "code": "console.log('Updated');"
}
```

**Response (200):**
```json
{
  "success": true,
  "data": { ... }
}
```

---

### DELETE /snippets/{id}

Delete a snippet.

**Headers:** `Authorization: Bearer <token>`

**Response (204):** No content

---

## Error Codes

| Code | Description | HTTP Status |
|------|-------------|--------------|
| VALIDATION_ERROR | Request validation failed | 400 |
| UNAUTHORIZED | Missing or invalid token | 401 |
| FORBIDDEN | Insufficient permissions | 403 |
| NOT_FOUND | Resource not found | 404 |
| CONFLICT_ERROR | Resource already exists | 409 |
| RATE_LIMIT_ERROR | Too many requests | 429 |
| INTERNAL_ERROR | Server error | 500 |
| EXTERNAL_SERVICE_ERROR | External service failed | 502 |

---

## Version History

- **v1.0.0** (2024-01-01): Initial API contract
