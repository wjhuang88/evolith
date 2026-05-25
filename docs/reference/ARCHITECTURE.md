# 架构设计

## 1. 系统概述

Evolith 采用前后端分离的微服务架构，后端使用 Rust 构建高性能 API 服务，前端当前使用 React + Vite + Bun 构建静态 SPA。EVO-016 之前，前端静态产物可由 Nginx 托管；终局状态会把前端产物嵌入后端发布物，Nginx 只作为可选网关、SSL 终止和反向代理层。

## 2. 整体架构

```
┌─────────────────────────────────────────────────────────────────┐
│                         Client Layer                             │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐  │
│  │   Web App   │  │  MCP Client │  │   Agent SDK (Future)    │  │
│  │React+Vite SPA│ │  (Claude)   │  │   (Multi-language)      │  │
│  └──────┬──────┘  └──────┬──────┘  └────────────┬────────────┘  │
└─────────┼────────────────┼──────────────────────┼────────────────┘
          │                │                      │
          ▼                ▼                      ▼
┌─────────────────────────────────────────────────────────────────┐
│                         Gateway Layer                            │
├─────────────────────────────────────────────────────────────────┤
│  ┌────────────────────────────────────────────────────────────┐ │
│  │            Optional Gateway (Nginx / Platform LB)           │ │
│  │  - SSL Termination                                         │ │
│  │  - Reverse Proxy / Static SPA hosting before EVO-016       │ │
│  │  - Load Balancing                                          │ │
│  └────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
          │
          ▼
┌─────────────────────────────────────────────────────────────────┐
│                       Service Layer                              │
├─────────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────────┐   │
│  │  Tool Service │  │ Skill Service │  │  Snippet Service    │   │
│  │              │  │              │  │                      │   │
│  │ - MCP Server │  │ - Execution  │  │ - Search             │   │
│  │ - Registry   │  │ - Registry   │  │ - Versioning         │   │
│  │ - Discovery  │  │ - Versioning │  │ - Reference          │   │
│  └──────┬───────┘  └──────┬───────┘  └──────────┬───────────┘   │
│         │                 │                      │               │
│         └─────────────────┼──────────────────────┘               │
│                           │                                      │
│                           ▼                                      │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │                    Core Services                            │ │
│  │  - Auth Service (JWT)                                      │ │
│  │  - User Service                                            │ │
│  │  - Notification Service                                    │ │
│  │  - Audit Service                                           │ │
│  └────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
          │
          ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Execution Layer                             │
├─────────────────────────────────────────────────────────────────┤
│  ┌────────────────────────────────────────────────────────────┐ │
│  │              Code Execution Engine                          │ │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │ │
│  │  │  Sandbox     │  │  Sandbox     │  │  Sandbox     │      │ │
│  │  │  (Python)    │  │  (Node.js)   │  │  (Rust/WASM) │      │ │
│  │  └──────────────┘  └──────────────┘  └──────────────┘      │ │
│  └────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
          │
          ▼
┌─────────────────────────────────────────────────────────────────┐
│                       Data Layer                                 │
├─────────────────────────────────────────────────────────────────┤
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────────────┐ │
│  │PostgreSQL│  │  Redis   │  │  MinIO   │  │ Elasticsearch    │ │
│  │          │  │          │  │          │  │  (Future)        │ │
│  │- Users   │  │- Cache   │  │- Files   │  │- Full-text       │ │
│  │- Tools   │  │- Session │  │- Skills  │  │  Search          │ │
│  │- Skills  │  │- Queue   │  │- Snippets│  │                  │ │
│  │- Snippets│  │          │  │          │  │                  │ │
│  └──────────┘  └──────────┘  └──────────┘  └──────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

## 3. 模块设计

### 3.1 后端模块划分

```
backend/
├── Cargo.toml                    # Workspace配置
├── crates/
│   ├── api/                      # API层
│   │   ├── src/
│   │   │   ├── routes/          # 路由定义
│   │   │   ├── handlers/        # 请求处理
│   │   │   ├── middleware/      # 中间件
│   │   │   └── dto/             # 数据传输对象
│   │   └── Cargo.toml
│   │
│   ├── service-tool/             # 工具服务
│   │   ├── src/
│   │   │   ├── registry.rs      # 工具注册
│   │   │   ├── discovery.rs     # 工具发现
│   │   │   ├── executor.rs      # 工具执行
│   │   │   └── mcp.rs           # MCP协议实现
│   │   └── Cargo.toml
│   │
│   ├── service-skill/            # 技能服务
│   │   ├── src/
│   │   │   ├── registry.rs      # 技能注册
│   │   │   ├── parser.rs        # SKILL.md解析
│   │   │   ├── executor.rs      # 代码执行
│   │   │   └── sandbox.rs       # 沙箱管理
│   │   └── Cargo.toml
│   │
│   ├── service-snippet/          # 代码片段服务
│   │   ├── src/
│   │   │   ├── repository.rs    # 片段仓库
│   │   │   ├── search.rs        # 搜索功能
│   │   │   ├── parser.rs        # 格式解析
│   │   │   └── reference.rs     # 引用生成
│   │   └── Cargo.toml
│   │
│   ├── service-auth/             # 认证服务
│   │   ├── src/
│   │   │   ├── jwt.rs           # JWT处理
│   │   │   ├── rbac.rs          # 权限控制
│   │   │   └── session.rs       # 会话管理
│   │   └── Cargo.toml
│   │
│   ├── domain/                   # 领域模型
│   │   ├── src/
│   │   │   ├── tool.rs
│   │   │   ├── skill.rs
│   │   │   ├── snippet.rs
│   │   │   └── user.rs
│   │   └── Cargo.toml
│   │
│   ├── infra/                    # 基础设施
│   │   ├── src/
│   │   │   ├── db/              # 数据库
│   │   │   ├── cache/           # 缓存
│   │   │   ├── storage/         # 对象存储
│   │   │   └── queue/           # 消息队列
│   │   └── Cargo.toml
│   │
│   └── common/                   # 公共模块
│       ├── src/
│       │   ├── error.rs         # 错误处理
│       │   ├── config.rs        # 配置管理
│       │   └── utils.rs         # 工具函数
│       └── Cargo.toml
```

### 3.2 前端模块划分

```
frontend/
├── package.json
├── bun.lock
├── vite.config.ts
├── index.html
├── src/
│   ├── main-spa.tsx             # Vite SPA 入口
│   ├── app/                     # 页面组件，路由由 React Router 装配
│   │   ├── login/
│   │   ├── register/
│   │   ├── join/
│   │   ├── dashboard/
│   │   ├── tools/
│   │   ├── skills/
│   │   └── snippets/
│   │
│   ├── components/               # 组件
│   │   ├── ui/                  # 基础UI组件
│   │   │   ├── Button/
│   │   │   ├── Input/
│   │   │   ├── Card/
│   │   │   └── ...
│   │   ├── layout/              # 布局组件
│   │   │   ├── Header/
│   │   │   ├── Sidebar/
│   │   │   └── Footer/
│   │   ├── features/            # 功能组件
│   │   │   ├── ToolCard/
│   │   │   ├── SkillEditor/
│   │   │   ├── SnippetViewer/
│   │   │   └── ...
│   │   └── common/              # 通用组件
│   │
│   ├── lib/                      # 工具库
│   │   ├── api/                 # API客户端
│   │   ├── auth/                # 认证相关
│   │   └── utils/               # 工具函数
│   │
│   ├── hooks/                    # 自定义Hooks
│   │   ├── useAuth.ts
│   │   ├── useTools.ts
│   │   ├── useSkills.ts
│   │   └── ...
│   │
│   ├── stores/                   # 状态管理 (Zustand)
│   │   ├── authStore.ts
│   │   ├── uiStore.ts
│   │   └── ...
│   │
│   ├── types/                    # 类型定义
│   │   ├── tool.ts
│   │   ├── skill.ts
│   │   ├── snippet.ts
│   │   └── ...
│   │
│   └── styles/                   # 样式
│       ├── globals.css
│       └── themes/
```

## 4. 核心流程

### 4.1 MCP工具调用流程

```
┌─────────┐    ┌─────────┐    ┌─────────────┐    ┌──────────────┐
│  Agent  │───▶│   MCP   │───▶│ Tool Service │───▶│   Executor   │
│  Client │    │ Protocol│    │             │    │              │
└─────────┘    └─────────┘    └─────────────┘    └──────────────┘
     │              │                 │                  │
     │ 1.调用请求   │                 │                  │
     │─────────────▶│                 │                  │
     │              │ 2.解析MCP消息   │                  │
     │              │────────────────▶│                  │
     │              │                 │ 3.查找工具      │
     │              │                 │─────────────────▶│
     │              │                 │                  │
     │              │                 │ 4.执行工具      │
     │              │                 │─────────────────▶│
     │              │                 │                  │
     │              │                 │ 5.返回结果      │
     │              │                 │◀─────────────────│
     │              │ 6.封装响应      │                  │
     │              │◀────────────────│                  │
     │ 7.返回结果   │                 │                  │
     │◀─────────────│                 │                  │
```

### 4.2 技能执行流程

```
┌─────────┐    ┌─────────────┐    ┌──────────────┐    ┌──────────┐
│  Agent  │───▶│Skill Service│───▶│ Code Sandbox │───▶│  Result  │
│  Client │    │             │    │              │    │ Processor│
└─────────┘    └─────────────┘    └──────────────┘    └──────────┘
     │               │                    │                 │
     │ 1.加载技能    │                    │                 │
     │──────────────▶│                    │                 │
     │               │ 2.解析SKILL.md     │                 │
     │               │───────────────────▶│                 │
     │               │                    │                 │
     │               │ 3.准备执行环境     │                 │
     │               │───────────────────▶│                 │
     │               │                    │                 │
     │ 4.执行请求    │                    │                 │
     │──────────────▶│                    │                 │
     │               │ 5.在沙箱执行代码   │                 │
     │               │───────────────────▶│                 │
     │               │                    │ 6.输出结果      │
     │               │                    │────────────────▶│
     │               │                    │                 │
     │               │ 7.处理和格式化     │                 │
     │               │◀───────────────────│─────────────────│
     │ 8.返回结果    │                    │                 │
     │◀──────────────│                    │                 │
```

### 4.3 代码片段引用流程

```
┌─────────┐    ┌────────────────┐    ┌───────────────┐
│  Agent  │───▶│Snippet Service │───▶│ Code Generator│
│  Client │    │                │    │               │
└─────────┘    └────────────────┘    └───────────────┘
     │                 │                     │
     │ 1.搜索片段      │                     │
     │────────────────▶│                     │
     │                 │ 2.匹配和排序        │
     │                 │                     │
     │ 3.返回列表      │                     │
     │◀────────────────│                     │
     │                 │                     │
     │ 4.请求引用      │                     │
     │────────────────▶│                     │
     │                 │ 5.生成可引用代码    │
     │                 │────────────────────▶│
     │                 │                     │
     │                 │ 6.包含依赖声明      │
     │                 │◀────────────────────│
     │ 7.返回完整代码  │                     │
     │◀────────────────│                     │
```

## 5. 数据模型

### 5.1 多租户架构

Evolith 采用**共享数据库 + 租户ID**的多租户架构，详见 [多租户设计](./MULTI-TENANT.md)。

**租户识别方式**：
- 子域名：`{tenant}.evolith.io`
- 请求头：`X-Tenant-ID`
- JWT Token 中的 `tenant_id`

### 5.2 核心实体

```rust
// Tenant (租户)
struct Tenant {
    id: Uuid,
    name: String,
    slug: String,              // URL友好标识
    plan: Plan,                 // free, pro, enterprise
    owner_id: Uuid,
    settings: TenantSettings,
    created_at: DateTime,
    updated_at: DateTime,
}

// Tool (工具)
struct Tool {
    id: Uuid,
    tenant_id: Uuid,            // 租户ID
    name: String,
    description: String,
    input_schema: JsonValue,      // JSON Schema
    output_schema: JsonValue,
    handler: HandlerConfig,
    owner_id: Uuid,
    visibility: Visibility,
    created_at: DateTime,
    updated_at: DateTime,
}

// Skill (技能)
struct Skill {
    id: Uuid,
    tenant_id: Uuid,            // 租户ID
    name: String,
    version: String,
    description: String,
    skill_md: String,             // SKILL.md内容
    code_package: Option<String>, // 代码包路径
    runtime: Runtime,
    dependencies: Vec<Dependency>,
    owner_id: Uuid,
    visibility: Visibility,
    created_at: DateTime,
    updated_at: DateTime,
}

// Snippet (代码片段)
struct Snippet {
    id: Uuid,
    tenant_id: Uuid,            // 租户ID
    name: String,
    language: String,
    framework: Option<String>,
    tags: Vec<String>,
    content: String,              // Markdown内容
    code: String,                 // 实际代码
    dependencies: Vec<Dependency>,
    estimated_tokens: u32,
    owner_id: Uuid,
    visibility: Visibility,
    created_at: DateTime,
    updated_at: DateTime,
}

// User (用户)
struct User {
    id: Uuid,
    tenant_id: Uuid,            // 租户ID
    username: String,
    email: String,
    password_hash: String,
    role: Role,                  // owner, admin, member
    created_at: DateTime,
    updated_at: DateTime,
}
```

### 5.3 数据库Schema（多租户）

```sql
-- 租户表
CREATE TABLE tenants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(128) NOT NULL,
    slug VARCHAR(64) UNIQUE NOT NULL,         -- URL友好标识
    plan VARCHAR(32) NOT NULL DEFAULT 'free',  -- free, pro, enterprise
    max_tools INTEGER DEFAULT 10,
    max_skills INTEGER DEFAULT 5,
    max_snippets INTEGER DEFAULT 50,
    max_api_calls_per_day INTEGER DEFAULT 1000,
    owner_id UUID NOT NULL,
    settings JSONB DEFAULT '{}',
    status VARCHAR(32) NOT NULL DEFAULT 'active',  -- active, suspended, deleted
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 用户表
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    username VARCHAR(64) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role VARCHAR(32) NOT NULL DEFAULT 'member',  -- owner, admin, member
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, username),
    UNIQUE(tenant_id, email)
);

-- 工具表
CREATE TABLE tools (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    name VARCHAR(128) NOT NULL,
    description TEXT NOT NULL,
    input_schema JSONB NOT NULL,
    output_schema JSONB,
    handler_config JSONB NOT NULL,
    owner_id UUID NOT NULL REFERENCES users(id),
    visibility VARCHAR(32) NOT NULL DEFAULT 'private',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, name)
);

-- 技能表
CREATE TABLE skills (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    name VARCHAR(128) NOT NULL,
    version VARCHAR(32) NOT NULL,
    description TEXT NOT NULL,
    skill_md TEXT NOT NULL,
    code_package_path TEXT,
    runtime VARCHAR(32) NOT NULL,
    dependencies JSONB DEFAULT '[]',
    owner_id UUID NOT NULL REFERENCES users(id),
    visibility VARCHAR(32) NOT NULL DEFAULT 'private',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, name, version)
);

-- 代码片段表
CREATE TABLE snippets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    name VARCHAR(128) NOT NULL,
    language VARCHAR(32) NOT NULL,
    framework VARCHAR(64),
    tags TEXT[] DEFAULT '{}',
    content TEXT NOT NULL,
    code TEXT NOT NULL,
    dependencies JSONB DEFAULT '[]',
    estimated_tokens INTEGER NOT NULL DEFAULT 0,
    owner_id UUID NOT NULL REFERENCES users(id),
    visibility VARCHAR(32) NOT NULL DEFAULT 'private',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 使用统计表
CREATE TABLE usage_stats (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    date DATE NOT NULL,
    api_calls INTEGER DEFAULT 0,
    tool_executions INTEGER DEFAULT 0,
    skill_executions INTEGER DEFAULT 0,
    storage_used_bytes BIGINT DEFAULT 0,
    UNIQUE(tenant_id, date)
);

-- 索引（包含 tenant_id）
CREATE INDEX idx_users_tenant ON users(tenant_id);
CREATE INDEX idx_tools_tenant ON tools(tenant_id);
CREATE INDEX idx_skills_tenant ON skills(tenant_id);
CREATE INDEX idx_snippets_tenant ON snippets(tenant_id);
CREATE INDEX idx_tools_owner ON tools(owner_id);
CREATE INDEX idx_skills_owner ON skills(owner_id);
CREATE INDEX idx_snippets_owner ON snippets(owner_id);
CREATE INDEX idx_snippets_tags ON snippets USING GIN(tags);
CREATE INDEX idx_snippets_language ON snippets(language);
CREATE INDEX idx_usage_stats_tenant_date ON usage_stats(tenant_id, date);
```

## 6. 安全设计

### 6.1 代码执行沙箱

```
┌─────────────────────────────────────────────────────┐
│                   Sandbox Architecture               │
├─────────────────────────────────────────────────────┤
│                                                      │
│  ┌──────────────────────────────────────────────┐  │
│  │           Resource Limits                     │  │
│  │  - CPU: 0.5 cores                            │  │
│  │  - Memory: 256MB                              │  │
│  │  - Time: 30s                                  │  │
│  │  - Network: Disabled                          │  │
│  │  - Filesystem: Read-only + /tmp              │  │
│  └──────────────────────────────────────────────┘  │
│                                                      │
│  ┌──────────────────────────────────────────────┐  │
│  │           Isolation                           │  │
│  │  - Namespace isolation (PID, NET, MNT)       │  │
│  │  - Seccomp filters                           │  │
│  │  - Capability dropping                       │  │
│  └──────────────────────────────────────────────┘  │
│                                                      │
│  ┌──────────────────────────────────────────────┐  │
│  │           Monitoring                          │  │
│  │  - Resource usage tracking                    │  │
│  │  - Timeout enforcement                       │  │
│  │  - OOM detection                             │  │
│  └──────────────────────────────────────────────┘  │
│                                                      │
└─────────────────────────────────────────────────────┘
```

### 6.2 认证授权

```
┌─────────────────────────────────────────────────────────┐
│                    Authentication Flow                   │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  ┌──────────┐     ┌──────────┐     ┌──────────────────┐ │
│  │  Client  │────▶│  Login   │────▶│  JWT Generation  │ │
│  └──────────┘     └──────────┘     └──────────────────┘ │
│                                           │              │
│                                           ▼              │
│  ┌────────────────────────────────────────────────────┐ │
│  │              JWT Payload                            │ │
│  │  {                                                 │ │
│  │    "sub": "user_id",                              │ │
│  │    "tenant_id": "tenant_uuid",                    │ │
│  │    "role": "admin",                               │ │
│  │    "exp": 1234567890,                             │ │
│  │    "iat": 1234560000                              │ │
│  │  }                                                 │ │
│  └────────────────────────────────────────────────────┘ │
│                                           │              │
│  **Token 传递方式**: httpOnly cookie (`evolith_token`)    │
│  **CSRF 防护**: double-submit cookie (`csrf_token`)      │
│                                           │              │
│  ┌────────────────────────────────────────────────────┐ │
│  │              Authorization                          │ │
│  │  - RBAC: role-based access control                │ │
│  │  - Resource ownership: user can modify own items  │ │
│  │  - Public/Private visibility                      │ │
│  └────────────────────────────────────────────────────┘ │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

## 7. 部署架构

```
┌─────────────────────────────────────────────────────────────────┐
│                        Production Environment                    │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │                    Load Balancer                          │  │
│  │                    (Nginx / Cloud LB)                     │  │
│  └──────────────────────────────────────────────────────────┘  │
│                            │                                     │
│            ┌───────────────┼───────────────┐                   │
│            ▼               ▼               ▼                   │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐           │
│  │   Backend    │ │   Backend    │ │   Backend    │           │
│  │  Instance 1  │ │  Instance 2  │ │  Instance N  │           │
│  └──────┬───────┘ └──────┬───────┘ └──────┬───────┘           │
│         │                │                │                    │
│         └────────────────┼────────────────┘                    │
│                          │                                      │
│  ┌───────────────────────┼───────────────────────────────────┐│
│  │                       │         Shared Services            ││
│  │  ┌──────────┐  ┌──────┴─────┐  ┌──────────┐  ┌─────────┐ ││
│  │  │PostgreSQL│  │   Redis    │  │  MinIO   │  │ Sandbox │ ││
│  │  │(Primary) │  │  Cluster   │  │ Cluster  │  │ Workers │ ││
│  │  └──────────┘  └────────────┘  └──────────┘  └─────────┘ ││
│  │       │                                                    ││
│  │  ┌──────────┐                                             ││
│  │  │PostgreSQL│                                             ││
│  │  │(Replica) │                                             ││
│  │  └──────────┘                                             ││
│  └───────────────────────────────────────────────────────────┘│
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │                    Frontend Assets                        │  │
│  │  - Static SPA assets served by Nginx/CDN before EVO-016   │  │
│  │  - Embedded into backend release artifact at final state  │  │
│  └──────────────────────────────────────────────────────────┘  │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

## 8. 技术选型说明

### 8.1 后端技术栈

| 技术 | 用途 | 选择理由 |
|------|------|----------|
| Rust | 核心语言 | 高性能、内存安全、并发友好 |
| Actix-web | Web框架 | 高性能、功能完善、生态成熟 |
| SQLx | 数据库访问 | 编译时SQL检查、异步支持 |
| Redis | 缓存 | 高性能、支持多种数据结构 |
| MinIO | 对象存储 | S3兼容、可自托管 |

### 8.2 前端技术栈

| 技术 | 用途 | 选择理由 |
|------|------|----------|
| React | UI 框架 | 生态成熟、组件模型稳定 |
| Vite | 构建工具 | 静态 SPA 构建快，部署产物简单 |
| Bun | 包管理与脚本运行 | 与当前前端迁移目标一致，锁文件为 `bun.lock` |
| TypeScript | 语言 | 类型安全、开发体验好 |
| Tailwind CSS | 样式 | 快速开发、一致性高 |
| Zustand | 状态管理 | 轻量、简单易用 |
| TanStack Query | 数据获取 | 缓存、自动刷新 |

### 8.3 代码执行

| 运行时 | 支持语言 | 隔离方式 |
|--------|----------|----------|
| Python 3.11 | Python | Container |
| Node.js 20 | JavaScript/TypeScript | Container |
| WASM | Rust/Go/etc. | Native Sandbox |
## 9. API契约优先开发

Evolith采用契约优先(Contract-First)的API开发模式，确保前后端接口的一致性。

执行步骤、DoD 和变更记录要求见 [API 契约优先 SOP](../sop/CONTRACT-FIRST.md)。本节保留架构层面的契约边界说明。

### 9.1 契约文件

API契约定义在 `docs/reference/API-CONTRACT.md` 中，包含：
- 所有API端点的请求/响应格式
- 数据类型定义
- 错误码说明
- 认证方式

### 9.2 开发流程

```
1. 编写API契约 (API-CONTRACT.md)
   ↓
2. 前端根据契约生成TypeScript类型
   ↓
3. 后端实现API端点
   ↓
4. 前后端独立测试
   ↓
5. 集成测试验证
```

### 9.3 前端类型生成

后端API契约可使用工具自动生成前端TypeScript类型：

```typescript
// 从API契约生成 types/api.ts
export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  meta?: PaginationMeta;
  error?: ErrorInfo;
}

export interface LoginRequest {
  email: string;
  password: string;
}

export interface AuthResponse {
  token: string;
  expires_at: number;
  user: UserInfo;
}
```

### 9.4 契约版本管理

- 契约文件使用语义化版本
- 重大变更需要更新主版本号
- 向前兼容的变更可更新次版本号
- 保持契约文档与实际实现同步

## 10. 环境配置

完整环境变量清单和常见误用见 [配置参考](./CONFIG.md)。本节只保留架构部署视角下的最小示例。

### 10.1 开发环境

使用内存数据库(SQLite)进行本地开发：

```bash
DATABASE__DATABASE_TYPE=sqlite
DATABASE__URL=:memory:
ENVIRONMENT=development
```

### 10.2 生产环境

切换到PostgreSQL：

```bash
DATABASE__DATABASE_TYPE=postgres
DATABASE__URL=postgresql://user:pass@host:5432/evolith
ENVIRONMENT=production
```

### 10.3 环境变量

| 变量 | 说明 | 默认值 |
|------|------|--------|
| `DATABASE__URL` | 数据库连接串 | `:memory:` |
| `DATABASE__DATABASE_TYPE` | 数据库类型 | `sqlite` |
| `JWT__SECRET` | JWT密钥 | (开发专用) |
| `JWT__EXPIRATION` | Token过期时间 | `24h` |
| `LOG__LEVEL` | 日志级别 | `info` |
| `ENVIRONMENT` | 环境 | `development` |
| `APP__PUBLIC_URL` | 对外访问入口，用于邮件链接等绝对 URL | `http://localhost:3001` |
| `CORS__ALLOWED_ORIGIN` | 允许的前端域名 | `http://localhost:3001` |
| `CSRF__ENABLED` | CSRF 保护开关 | `true` |
| `SANDBOX__ENABLED` | 沙箱执行器开关 | `true` |
| `SANDBOX__TIMEOUT_SECONDS` | 沙箱执行超时 | `30` |
| `SANDBOX__MEMORY_MB` | 沙箱内存限制 | `256` |
| `RATE_LIMIT__UNAUTHENTICATED_RPM` | 未认证请求限流 | `30` |
| `RATE_LIMIT__AUTHENTICATED_RPM` | 已认证请求限流 | `300` |
| `RATE_LIMIT__API_KEY_RPM` | API key 请求限流 | `1000` |
| `SMTP__HOST` | SMTP 邮件服务器 | (无) |
| `SMTP__PORT` | SMTP 端口 | `587` |
| `SMTP__FROM_ADDRESS` | 发件人地址 | (无) |
| `SMTP__FROM_NAME` | 发件人名称 | `Evolith` |

## 11. 相关文档

- [多租户设计](./MULTI-TENANT.md) - **多租户架构详细设计**
- [API契约](./API-CONTRACT.md) - 前后端接口详细定义
- [Skill格式](./formats/SKILL-FORMAT.md) - 技能定义规范
- [代码片段格式](./formats/SNIPPET-FORMAT.md) - 片段定义规范
- [技术栈](./TECH-STACK.md) - 技术选型详情
