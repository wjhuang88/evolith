# 架构设计

## 1. 系统概述

Evolith采用前后端分离的微服务架构，后端使用Rust构建高性能API服务，前端使用Next.js构建现代化Web界面。

## 2. 整体架构

```
┌─────────────────────────────────────────────────────────────────┐
│                         Client Layer                             │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐  │
│  │   Web App   │  │  MCP Client │  │   Agent SDK (Future)    │  │
│  │  (Next.js)  │  │  (Claude)   │  │   (Multi-language)      │  │
│  └──────┬──────┘  └──────┬──────┘  └────────────┬────────────┘  │
└─────────┼────────────────┼──────────────────────┼────────────────┘
          │                │                      │
          ▼                ▼                      ▼
┌─────────────────────────────────────────────────────────────────┐
│                         Gateway Layer                            │
├─────────────────────────────────────────────────────────────────┤
│  ┌────────────────────────────────────────────────────────────┐ │
│  │                    API Gateway (Nginx)                      │ │
│  │  - SSL Termination                                         │ │
│  │  - Rate Limiting                                           │ │
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
├── src/
│   ├── app/                      # Next.js App Router
│   │   ├── layout.tsx           # 根布局
│   │   ├── page.tsx             # 首页
│   │   ├── (auth)/              # 认证相关页面
│   │   │   ├── login/
│   │   │   └── register/
│   │   ├── tools/               # 工具管理
│   │   │   ├── page.tsx
│   │   │   └── [id]/
│   │   ├── skills/              # 技能管理
│   │   │   ├── page.tsx
│   │   │   ├── [id]/
│   │   │   └── create/
│   │   └── snippets/            # 代码片段
│   │       ├── page.tsx
│   │       ├── [id]/
│   │       └── create/
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

### 5.1 核心实体

```rust
// Tool (工具)
struct Tool {
    id: Uuid,
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
    username: String,
    email: String,
    password_hash: String,
    role: Role,
    created_at: DateTime,
    updated_at: DateTime,
}
```

### 5.2 数据库Schema

```sql
-- 用户表
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(64) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role VARCHAR(32) NOT NULL DEFAULT 'user',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 工具表
CREATE TABLE tools (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(128) UNIQUE NOT NULL,
    description TEXT NOT NULL,
    input_schema JSONB NOT NULL,
    output_schema JSONB,
    handler_config JSONB NOT NULL,
    owner_id UUID NOT NULL REFERENCES users(id),
    visibility VARCHAR(32) NOT NULL DEFAULT 'private',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 技能表
CREATE TABLE skills (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
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
    UNIQUE(name, version)
);

-- 代码片段表
CREATE TABLE snippets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
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

-- 索引
CREATE INDEX idx_tools_owner ON tools(owner_id);
CREATE INDEX idx_skills_owner ON skills(owner_id);
CREATE INDEX idx_snippets_owner ON snippets(owner_id);
CREATE INDEX idx_snippets_tags ON snippets USING GIN(tags);
CREATE INDEX idx_snippets_language ON snippets(language);
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
│  │    "role": "admin",                               │ │
│  │    "exp": 1234567890,                             │ │
│  │    "iat": 1234560000                              │ │
│  │  }                                                 │ │
│  └────────────────────────────────────────────────────┘ │
│                                           │              │
│                                           ▼              │
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
│  │                    Frontend (CDN)                         │  │
│  │  - Static assets served via CDN                          │  │
│  │  - Next.js SSR on edge                                   │  │
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
| Next.js | 框架 | SSR/SSG支持、React生态 |
| TypeScript | 语言 | 类型安全、开发体验好 |
| Tailwind CSS | 样式 | 快速开发、一致性高 |
| Zustand | 状态管理 | 轻量、简单易用 |
| React Query | 数据获取 | 缓存、自动刷新 |

### 8.3 代码执行

| 运行时 | 支持语言 | 隔离方式 |
|--------|----------|----------|
| Python 3.11 | Python | Container |
| Node.js 20 | JavaScript/TypeScript | Container |
| WASM | Rust/Go/etc. | Native Sandbox |
