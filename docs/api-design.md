# API设计

## 1. API概述

### 1.1 设计原则

- **RESTful风格**：遵循REST架构设计
- **版本控制**：URL路径版本控制 `/api/v1/`
- **统一响应**：标准化响应格式
- **错误处理**：统一错误码和错误信息

### 1.2 基础URL

```
开发环境: http://localhost:8080/api/v1
生产环境: https://api.evolith.io/api/v1
```

### 1.3 通用响应格式

```typescript
// 成功响应
interface SuccessResponse<T> {
  success: true;
  data: T;
  meta?: {
    page?: number;
    per_page?: number;
    total?: number;
  };
}

// 错误响应
interface ErrorResponse {
  success: false;
  error: {
    code: string;
    message: string;
    details?: Record<string, unknown>;
  };
}
```

### 1.4 通用错误码

| 错误码 | HTTP状态 | 描述 |
|--------|----------|------|
| INVALID_REQUEST | 400 | 请求参数无效 |
| UNAUTHORIZED | 401 | 未认证 |
| FORBIDDEN | 403 | 无权限 |
| NOT_FOUND | 404 | 资源不存在 |
| CONFLICT | 409 | 资源冲突 |
| RATE_LIMITED | 429 | 请求过于频繁 |
| INTERNAL_ERROR | 500 | 服务器内部错误 |

## 2. 认证API

### 2.1 用户注册

```
POST /api/v1/auth/register

Request:
{
  "username": "string",      // 3-64字符
  "email": "string",         // 有效邮箱
  "password": "string"       // 至少8字符
}

Response:
{
  "success": true,
  "data": {
    "user": {
      "id": "uuid",
      "username": "string",
      "email": "string",
      "created_at": "datetime"
    },
    "token": "jwt_token"
  }
}
```

### 2.2 用户登录

```
POST /api/v1/auth/login

Request:
{
  "email": "string",
  "password": "string"
}

Response:
{
  "success": true,
  "data": {
    "user": { ... },
    "token": "jwt_token",
    "expires_at": "datetime"
  }
}
```

### 2.3 刷新Token

```
POST /api/v1/auth/refresh

Headers:
  Authorization: Bearer <token>

Response:
{
  "success": true,
  "data": {
    "token": "new_jwt_token",
    "expires_at": "datetime"
  }
}
```

## 3. 工具API

### 3.1 创建工具

```
POST /api/v1/tools

Headers:
  Authorization: Bearer <token>

Request:
{
  "name": "weather-query",
  "description": "Query weather information for a city",
  "input_schema": {
    "type": "object",
    "properties": {
      "city": { "type": "string", "description": "City name" },
      "unit": { "type": "string", "enum": ["celsius", "fahrenheit"] }
    },
    "required": ["city"]
  },
  "output_schema": {
    "type": "object",
    "properties": {
      "temperature": { "type": "number" },
      "condition": { "type": "string" }
    }
  },
  "handler": {
    "type": "http",
    "url": "https://api.weather.example.com",
    "method": "GET",
    "timeout": 5000
  },
  "visibility": "public"  // public | private
}

Response:
{
  "success": true,
  "data": {
    "id": "uuid",
    "name": "weather-query",
    "description": "...",
    "created_at": "datetime"
  }
}
```

### 3.2 获取工具列表

```
GET /api/v1/tools?page=1&per_page=20&search=weather&visibility=public

Headers:
  Authorization: Bearer <token>  (可选)

Response:
{
  "success": true,
  "data": [
    {
      "id": "uuid",
      "name": "weather-query",
      "description": "...",
      "visibility": "public",
      "owner": {
        "id": "uuid",
        "username": "string"
      },
      "created_at": "datetime"
    }
  ],
  "meta": {
    "page": 1,
    "per_page": 20,
    "total": 45
  }
}
```

### 3.3 获取工具详情

```
GET /api/v1/tools/{id}

Response:
{
  "success": true,
  "data": {
    "id": "uuid",
    "name": "weather-query",
    "description": "...",
    "input_schema": { ... },
    "output_schema": { ... },
    "handler": { ... },
    "visibility": "public",
    "owner": { ... },
    "stats": {
      "calls": 1234,
      "success_rate": 0.98
    },
    "created_at": "datetime",
    "updated_at": "datetime"
  }
}
```

### 3.4 更新工具

```
PUT /api/v1/tools/{id}

Headers:
  Authorization: Bearer <token>

Request:
{
  "description": "Updated description",
  "input_schema": { ... },
  "visibility": "private"
}

Response:
{
  "success": true,
  "data": { ... }
}
```

### 3.5 删除工具

```
DELETE /api/v1/tools/{id}

Headers:
  Authorization: Bearer <token>

Response:
{
  "success": true,
  "data": null
}
```

### 3.6 调用工具（MCP协议）

```
POST /mcp

Headers:
  Content-Type: application/json
  Authorization: Bearer <token>

Request:
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "weather-query",
    "arguments": {
      "city": "Beijing",
      "unit": "celsius"
    }
  }
}

Response:
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "{\"temperature\": 25, \"condition\": \"sunny\"}"
      }
    ]
  }
}
```

## 4. 技能API

### 4.1 创建技能

```
POST /api/v1/skills

Headers:
  Authorization: Bearer <token>
  Content-Type: multipart/form-data

Request (multipart):
{
  "skill_md": "SKILL.md文件内容",
  "code_package": "代码包文件(zip)",
  "config": {
    "runtime": "python3.11",
    "entrypoint": "main.py",
    "dependencies": ["pandas>=2.0"]
  }
}

Response:
{
  "success": true,
  "data": {
    "id": "uuid",
    "name": "data-analyzer",
    "version": "1.0.0",
    "created_at": "datetime"
  }
}
```

### 4.2 获取技能列表

```
GET /api/v1/skills?page=1&per_page=20&search=data&runtime=python

Response:
{
  "success": true,
  "data": [
    {
      "id": "uuid",
      "name": "data-analyzer",
      "version": "1.0.0",
      "description": "...",
      "runtime": "python3.11",
      "visibility": "public",
      "owner": { ... },
      "stats": {
        "downloads": 567,
        "rating": 4.5
      },
      "created_at": "datetime"
    }
  ],
  "meta": { ... }
}
```

### 4.3 获取技能详情

```
GET /api/v1/skills/{id}

Response:
{
  "success": true,
  "data": {
    "id": "uuid",
    "name": "data-analyzer",
    "version": "1.0.0",
    "description": "...",
    "skill_md": "...",
    "runtime": "python3.11",
    "dependencies": [...],
    "visibility": "public",
    "owner": { ... },
    "versions": ["1.0.0", "0.9.0", "0.8.0"],
    "created_at": "datetime"
  }
}
```

### 4.4 加载技能（远程加载）

```
GET /api/v1/skills/{id}/load

Headers:
  Authorization: Bearer <token>

Response:
{
  "success": true,
  "data": {
    "skill_md": "...",
    "metadata": {
      "name": "data-analyzer",
      "description": "...",
      "execution": "server",
      "entrypoint": "main.py"
    },
    "api_endpoint": "/api/v1/skills/{id}/execute"
  }
}
```

### 4.5 执行技能

```
POST /api/v1/skills/{id}/execute

Headers:
  Authorization: Bearer <token>

Request:
{
  "arguments": {
    "input_file": "/data/input.csv",
    "options": {
      "format": "json",
      "include_charts": true
    }
  }
}

Response:
{
  "success": true,
  "data": {
    "output": "分析结果...",
    "files": [
      {
        "name": "report.pdf",
        "url": "/api/v1/files/{file_id}",
        "size": 1024
      }
    ],
    "logs": [
      "Loading data...",
      "Processing...",
      "Done."
    ],
    "execution_time": 2.5
  }
}
```

### 4.6 获取技能版本

```
GET /api/v1/skills/{id}/versions

Response:
{
  "success": true,
  "data": [
    {
      "version": "1.0.0",
      "created_at": "datetime",
      "changelog": "Added new feature"
    },
    {
      "version": "0.9.0",
      "created_at": "datetime",
      "changelog": "Bug fixes"
    }
  ]
}
```

## 5. 代码片段API

### 5.1 创建片段

```
POST /api/v1/snippets

Headers:
  Authorization: Bearer <token>

Request:
{
  "name": "useDebounce Hook",
  "language": "typescript",
  "framework": "react",
  "tags": ["hooks", "debounce", "performance"],
  "content": "# useDebounce Hook\n\n...",
  "code": "export function useDebounce<T>(value: T, delay: number): T { ... }",
  "dependencies": [
    { "name": "react", "version": "^18.0.0" }
  ],
  "visibility": "public"
}

Response:
{
  "success": true,
  "data": {
    "id": "uuid",
    "name": "useDebounce Hook",
    "estimated_tokens": 150,
    "created_at": "datetime"
  }
}
```

### 5.2 搜索片段

```
GET /api/v1/snippets/search?q=debounce&language=typescript&framework=react

Response:
{
  "success": true,
  "data": [
    {
      "id": "uuid",
      "name": "useDebounce Hook",
      "language": "typescript",
      "framework": "react",
      "tags": ["hooks", "debounce"],
      "estimated_tokens": 150,
      "relevance": 0.95
    }
  ],
  "meta": {
    "query": "debounce",
    "total": 5
  }
}
```

### 5.3 获取片段详情

```
GET /api/v1/snippets/{id}

Response:
{
  "success": true,
  "data": {
    "id": "uuid",
    "name": "useDebounce Hook",
    "language": "typescript",
    "framework": "react",
    "tags": [...],
    "content": "...",
    "code": "...",
    "dependencies": [...],
    "estimated_tokens": 150,
    "owner": { ... },
    "created_at": "datetime"
  }
}
```

### 5.4 获取可引用代码

```
GET /api/v1/snippets/{id}/reference?format=import

Response:
{
  "success": true,
  "data": {
    "code": "import { useDebounce } from '@evolith/snippets/use-debounce';\n\n...",
    "installation": "npm install @evolith/snippets",
    "dependencies": [
      { "name": "react", "version": "^18.0.0", "install": "npm install react" }
    ],
    "usage_example": "const debouncedValue = useDebounce(value, 300);"
  }
}
```

### 5.5 按语言/框架列表

```
GET /api/v1/snippets?language=typescript&framework=react&page=1

Response:
{
  "success": true,
  "data": [...],
  "meta": {
    "page": 1,
    "per_page": 20,
    "total": 45,
    "filters": {
      "language": "typescript",
      "framework": "react"
    }
  }
}
```

## 6. 用户API

### 6.1 获取当前用户

```
GET /api/v1/users/me

Headers:
  Authorization: Bearer <token>

Response:
{
  "success": true,
  "data": {
    "id": "uuid",
    "username": "developer",
    "email": "dev@example.com",
    "role": "user",
    "stats": {
      "tools": 5,
      "skills": 3,
      "snippets": 12
    },
    "created_at": "datetime"
  }
}
```

### 6.2 获取用户的资源

```
GET /api/v1/users/me/{resource_type}?page=1

resource_type: tools | skills | snippets

Response:
{
  "success": true,
  "data": [...],
  "meta": { ... }
}
```

## 7. MCP协议端点

### 7.1 MCP Server端点

```
POST /mcp

支持的标准MCP方法：
- initialize        初始化连接
- tools/list        列出可用工具
- tools/call        调用工具
- resources/list    列出资源
- resources/read    读取资源
- prompts/list      列出提示模板
- prompts/get       获取提示模板
```

### 7.2 MCP Initialize

```
Request:
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "initialize",
  "params": {
    "protocolVersion": "2024-11-05",
    "capabilities": {},
    "clientInfo": {
      "name": "claude-code",
      "version": "1.0.0"
    }
  }
}

Response:
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "protocolVersion": "2024-11-05",
    "capabilities": {
      "tools": {}
    },
    "serverInfo": {
      "name": "evolith",
      "version": "1.0.0"
    }
  }
}
```

### 7.3 MCP Tools List

```
Request:
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "tools/list"
}

Response:
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": {
    "tools": [
      {
        "name": "weather-query",
        "description": "Query weather information",
        "inputSchema": {
          "type": "object",
          "properties": {
            "city": { "type": "string" }
          },
          "required": ["city"]
        }
      }
    ]
  }
}
```

## 8. 分页与过滤

### 8.1 分页参数

| 参数 | 默认值 | 最大值 | 描述 |
|------|--------|--------|------|
| page | 1 | - | 页码 |
| per_page | 20 | 100 | 每页数量 |

### 8.2 排序参数

| 参数 | 描述 |
|------|------|
| sort | 排序字段 (created_at, name, downloads等) |
| order | 排序方向 (asc, desc) |

### 8.3 过滤参数

| 资源 | 可过滤字段 |
|------|------------|
| tools | visibility, owner_id |
| skills | visibility, runtime, owner_id |
| snippets | visibility, language, framework, tags |

## 9. 限流策略

| 端点类型 | 限制 | 窗口 |
|----------|------|------|
| 认证 | 10次 | 1分钟 |
| 读操作 | 100次 | 1分钟 |
| 写操作 | 30次 | 1分钟 |
| 工具执行 | 60次 | 1分钟 |
| 技能执行 | 20次 | 1分钟 |
