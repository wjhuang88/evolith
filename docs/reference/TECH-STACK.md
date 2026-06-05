# 技术栈说明

## 1. 概述

本文档详细说明Evolith项目的技术选型和设计决策。

## 2. 后端技术栈

### 2.1 核心框架

| 技术 | 版本 | 用途 | 选择理由 |
|------|------|------|----------|
| Rust | 1.82+ | 核心语言 | 高性能、内存安全、并发友好、零成本抽象 |
| Actix-web | 4.x | Web框架 | 高性能、功能完善、生态成熟、异步支持 |
| Tokio | 1.x | 异步运行时 | Rust事实标准的异步运行时 |

### 2.2 数据层

#### 数据库抽象层设计

```
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                         │
│  (Service Layer - 只依赖 Repository Trait)                  │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    Repository Trait                          │
│  trait Repository { ... }                                   │
│  - 统一的数据访问接口                                        │
│  - 与具体数据库实现解耦                                      │
└─────────────────────────────────────────────────────────────┘
                              │
              ┌───────────────┼───────────────┐
              ▼               ▼               ▼
┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐
│ SQLite/内存模式  │ │   PostgreSQL    │ │     MySQL       │
│ (开发/测试)     │ │   (生产推荐)    │ │    (生产备选)   │
└─────────────────┘ └─────────────────┘ └─────────────────┘
```

MySQL 当前只保留 pool/config 入口，缺少 repository 实现，主服务会拒绝以 MySQL 启动。生产主路径是 PostgreSQL。

#### 数据库配置

当前配置由 `backend/crates/infra/src/config.rs` 读取，嵌套配置统一使用双下划线环境变量。
完整配置清单见 [配置参考](./CONFIG.md)，数据库迁移执行流程见 [数据库迁移 SOP](../sop/DATABASE-MIGRATION.md)。

```bash
# 开发环境 - SQLite 内存数据库
DATABASE__DATABASE_TYPE=sqlite
DATABASE__URL=":memory:"

# 开发环境 - SQLite 文件数据库
DATABASE__DATABASE_TYPE=sqlite
DATABASE__URL="sqlite:dev.db?mode=rwc"

# 生产环境 - PostgreSQL
DATABASE__DATABASE_TYPE=postgres
DATABASE__URL="postgres://user:pass@localhost:5432/evolith"
```

#### Repository Trait 设计

```rust
// domain/repository.rs
use async_trait::async_trait;

#[async_trait]
pub trait ToolRepository: Send + Sync {
    async fn create(&self, tool: NewTool) -> Result<Tool, RepositoryError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Tool>, RepositoryError>;
    async fn find_all(&self, filter: ToolFilter) -> Result<Vec<Tool>, RepositoryError>;
    async fn update(&self, id: Uuid, tool: UpdateTool) -> Result<Tool, RepositoryError>;
    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError>;
}

// 不同数据库的实现
pub struct SqliteToolRepository { /* ... */ }
pub struct PostgresToolRepository { /* ... */ }
pub struct MySqlToolRepository { /* ... */ }

#[async_trait]
impl ToolRepository for SqliteToolRepository { /* ... */ }
#[async_trait]
impl ToolRepository for PostgresToolRepository { /* ... */ }
#[async_trait]
impl ToolRepository for MySqlToolRepository { /* ... */ }
```

#### 数据库切换配置

```toml
# Cargo.toml
[dependencies]
# SQL工具包 - 支持多数据库
sqlx = { version = "0.7", features = [
    "runtime-tokio",
    "tls-rustls",
    "uuid",
    "chrono",
    "json",
] }

# 可选的数据库驱动
sqlx-sqlite = { version = "0.7", optional = true }
sqlx-postgres = { version = "0.7", optional = true }
sqlx-mysql = { version = "0.7", optional = true }

[features]
default = ["sqlite"]
sqlite = ["sqlx/sqlite", "sqlx-sqlite"]
postgres = ["sqlx/postgres", "sqlx-postgres"]
mysql = ["sqlx/mysql", "sqlx-mysql"]
all-databases = ["sqlite", "postgres", "mysql"]
```

> 不要使用 `DATABASE_TYPE` / `DATABASE_URL` 表达嵌套配置；这类旧写法不会按当前 `AppConfig` 规则读取。

#### 初始化数据

```rust
// infra/db/seed.rs
pub async fn seed_database(pool: &SqlitePool) -> Result<(), Error> {
    // 仅在开发/测试环境执行
    if std::env::var("ENVIRONMENT").unwrap_or_default() != "production" {
        // 创建测试用户
        sqlx::query!(
            r#"
            INSERT INTO users (id, username, email, password_hash, role)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT DO NOTHING
            "#,
            Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
            "testuser",
            "test@example.com",
            "hashed_password",
            "user"
        )
        .execute(pool)
        .await?;

        // 创建示例工具
        // ...
    }
    Ok(())
}
```

### 2.3 缓存层

| 技术 | 版本 | 用途 | 选择理由 |
|------|------|------|----------|
| Redis | 7.x | 缓存/会话 | 高性能、支持多种数据结构、持久化 |

### 2.4 对象存储

| 技术 | 版本 | 用途 | 选择理由 |
|------|------|------|----------|
| MinIO | 最新 | 对象存储 | S3兼容、可自托管、高性能 |

### 2.5 代码执行

| 运行时 | 版本 | 用途 |
|--------|------|------|
| Python | 3.11 | 技能执行 |
| Node.js | 20 LTS | 技能执行 |
| WASM | - | 高性能执行 |

## 3. 前端技术栈

> 当前实现是 `React + Vite + Bun` 静态 SPA，见
> [ADR-0001](../decisions/ADR-0001-react-vite-bun-frontend.md) 和
> [实施路线图 Phase B](../roadmap/IMPLEMENTATION-ROADMAP.md#phase-b--前端迁移到-react--vite--bunp0)。

### 3.1 核心框架

| 技术 | 版本 | 用途 | 选择理由 |
|------|------|------|----------|
| Vite | 8.x | 构建工具 | 静态 SPA 构建、开发启动快 |
| React | 18.x | UI库 | 组件化、生态成熟、类型支持 |
| TypeScript | 5.x | 语言 | 类型安全、开发体验好 |
| React Router | 7.x | 路由 | SPA 路由和 history fallback |

### 3.2 UI组件

| 技术 | 版本 | 用途 | 选择理由 |
|------|------|------|----------|
| Tailwind CSS | 3.x | 样式 | 快速开发、一致性高、可定制 |
| Radix UI | 最新 | 无样式组件 | 可访问性、键盘导航、无样式设计 |
| Lucide | 最新 | 图标 | 轻量、图标丰富 |

### 3.3 状态管理

| 技术 | 版本 | 用途 | 选择理由 |
|------|------|------|----------|
| Zustand | 4.x | 全局状态 | 轻量、简单易用、TypeScript友好 |
| TanStack Query | 5.x | 服务端状态 | 缓存、自动刷新、乐观更新 |

### 3.4 工具链

| 技术 | 版本 | 用途 |
|------|------|------|
| ESLint | 8.x | 代码检查 |
| Prettier | 3.x | 代码格式化 |
| Bun | 1.x | 包管理和脚本运行 |
| Vitest | 1.x | 单元测试 |
| Playwright | 1.x | E2E测试 |

## 4. DevOps

### 4.1 容器化

| 技术 | 用途 |
|------|------|
| Docker | 容器化 |
| Docker Compose | 本地开发编排 |
| Kubernetes | 生产部署（可选） |

### 4.2 CI/CD

GitHub Actions workflow 由 Iteration 029（EVO-030）建立，文件在
`.github/workflows/ci.yml`。

#### 触发策略

- **Tag-only**：`on: push: tags: ['v*.*.*']`（semver 模式）
- 每次 push 不自动跑 CI；只在打 `vX.Y.Z` tag 时触发
- 契合 release-driven 发布模型；节省 CI 配额
- 切割 release：`git tag v0.1.0 && git push origin v0.1.0`

#### 工作流内容

| 阶段 | 命令 | 用途 |
|------|------|------|
| Frontend install | `bun install --frozen-lockfile` | 锁定依赖版本 |
| Frontend type-check | `bun run type-check` | TypeScript 类型检查 |
| Frontend build | `bun run build` | 产出 `frontend/dist/`（被后端 embed） |
| Frontend lint | `bun run lint` | ESLint |
| Backend format | `cargo fmt --all -- --check` | Rust 格式门禁 |
| Backend check | `cargo check --workspace --all-targets` | 全量编译 |
| Backend clippy | `cargo clippy --workspace --all-targets -- -D warnings` | Lint 严格门禁 |
| Backend test (SQLite) | `cargo test --workspace` | 274 个集成测试（in-memory SQLite） |
| Backend test (PostgreSQL) | `cargo test --workspace` | 同一组测试，PG service 准备给 EVO-053 用 |

#### Service 容器

- `postgres:16-alpine` — 启动 PG 16 服务，供未来 PG 集成测试使用（EVO-053）
- 当前 integration tests 用 SQLite in-memory；PG env 变量已配置但 tests 不读

#### 缓存

- Rust: `Swatinem/rust-cache@v2`，`workspaces: backend -> target`，`shared-key: evolith-rust`
- Bun: `oven-sh/setup-bun@v1` 内置缓存

#### 不做（Deferred）

- **deploy workflow**：部署形态仍可能演化（嵌入式 + 可选 Nginx），不在 Iteration 029 范围
- **PR trigger**：trunk-based 模型，无 PR review 流程
- **PR-only 检查**：tag-only 模型下不区分 PR

## 5. 监控与日志

### 5.1 监控

| 技术 | 用途 |
|------|------|
| Prometheus | 指标收集 |
| Grafana | 可视化仪表盘 |
| AlertManager | 告警管理 |

### 5.2 日志

| 技术 | 用途 |
|------|------|
| tracing | Rust日志框架 |
| OpenTelemetry | 分布式追踪 |
| ELK Stack | 日志聚合（可选） |

## 6. 开发工具

### 6.1 后端

| 工具 | 用途 |
|------|------|
| cargo | 包管理、构建 |
| cargo-watch | 热重载 |
| cargo-nextest | 测试运行器 |
| sqlx-cli | 数据库迁移 |

### 6.2 前端

| 工具 | 用途 |
|------|------|
| Bun | 包管理和脚本运行 |
| Vite | 构建工具和开发服务器 |

## 7. 环境配置

稳定环境变量清单以 [配置参考](./CONFIG.md) 为准。本节只保留技术栈视角下的依赖服务示例。

### 7.1 开发环境

```yaml
# docker-compose.dev.yml
version: '3.8'
services:
  postgres:
    image: postgres:16
    environment:
      POSTGRES_DB: evolith
      POSTGRES_USER: evolith
      POSTGRES_PASSWORD: dev_password
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"

  minio:
    image: minio/minio
    command: server /data --console-address ":9001"
    environment:
      MINIO_ROOT_USER: minioadmin
      MINIO_ROOT_PASSWORD: minioadmin
    ports:
      - "9000:9000"
      - "9001:9001"
    volumes:
      - minio_data:/data

volumes:
  postgres_data:
  minio_data:
```

### 7.2 环境变量

```bash
# .env.development
DATABASE__DATABASE_TYPE=sqlite
DATABASE__URL=":memory:"

# 或者使用文件持久化的 SQLite
# DATABASE__URL="sqlite:dev.db?mode=rwc"

# 使用 PostgreSQL（如果需要）
# DATABASE__DATABASE_TYPE=postgres
# DATABASE__URL="postgres://evolith:dev_password@localhost:5432/evolith"

# Redis
REDIS__URL="redis://localhost:6379"

# Object storage / MinIO
STORAGE__ENDPOINT="localhost:9000"
STORAGE__ACCESS_KEY="minioadmin"
STORAGE__SECRET_KEY="minioadmin"
STORAGE__USE_SSL="false"
STORAGE__BUCKET="evolith"

# JWT
JWT__SECRET="dev_secret_key_change_in_production"
JWT__EXPIRATION="24h"

# 执行沙箱（默认关闭；需要执行 sandbox 时确认 Docker 可用后改为 true）
SANDBOX__ENABLED="false"
SANDBOX__TIMEOUT_SECONDS="30"
SANDBOX__MEMORY_MB="256"

# 环境
ENVIRONMENT="development"
LOG__LEVEL="debug"
```

```bash
# .env.production
DATABASE__DATABASE_TYPE=postgres
DATABASE__URL="postgres://user:password@postgres:5432/evolith"

# Redis (集群)
REDIS__URL="redis://redis-cluster:6379"

# Object storage / MinIO
STORAGE__ENDPOINT="minio:9000"
STORAGE__ACCESS_KEY="${STORAGE_ACCESS_KEY}"
STORAGE__SECRET_KEY="${STORAGE_SECRET_KEY}"
STORAGE__USE_SSL="true"
STORAGE__BUCKET="evolith"

# JWT
JWT__SECRET="${JWT_SECRET}"
JWT__EXPIRATION="24h"

# 执行沙箱（默认关闭；需要执行 sandbox 时确认 Docker 可用后改为 true）
SANDBOX__ENABLED="false"
SANDBOX__TIMEOUT_SECONDS="30"
SANDBOX__MEMORY_MB="256"

# 环境
ENVIRONMENT="production"
LOG__LEVEL="info"
```

## 8. 项目结构

### 8.1 整体结构

```
evolith/
├── backend/                  # Rust后端
│   ├── crates/
│   │   ├── api/             # API层
│   │   ├── service-tool/    # 工具服务
│   │   ├── service-skill/   # 技能服务 (Docker 沙箱)
│   │   ├── service-snippet/ # 片段服务
│   │   ├── service-auth/    # 认证服务
│   │   ├── service-audit/   # 审计日志
│   │   ├── service-payment/ # Stripe 计费
│   │   ├── domain/          # 领域模型
│   │   ├── infra/           # 基础设施
│   │   └── common/          # 公共模块
│   ├── migrations/
│   │   ├── sqlite/          # SQLite 迁移 (开发)
│   │   └── postgres/        # PostgreSQL 迁移 (生产)
│   ├── sandbox/             # 沙箱 Dockerfile (Python, Node.js)
│   ├── Cargo.toml
│   └── Cargo.lock
│
├── frontend/                 # React + Vite + Bun 前端
│   ├── src/
│   │   ├── app/             # 页面 (22 路由)
│   │   ├── components/      # 组件
│   │   ├── lib/             # 工具库 (API client, i18n)
│   │   ├── locales/         # 翻译文件 (zh-CN, en)
│   │   ├── stores/          # 状态 (Zustand)
│   │   ├── types/           # 类型
│   │   └── styles/          # 样式
│   ├── package.json
│   ├── bun.lock
│   └── vite.config.ts
│
├── deploy/                   # Nginx 配置
├── scripts/                  # dev.sh, backup.sh, deploy.sh
├── .github/workflows/        # CI/CD workflow（EVO-030 重建）
├── docs/                     # 文档
├── docker-compose.prod.yml   # 生产 Docker 编排
└── README.md
```

## 9. 数据库切换最佳实践

### 9.1 切换流程

```
开发阶段                          生产阶段
┌──────────────┐               ┌──────────────┐
│ SQLite内存   │               │ PostgreSQL   │
│ DATABASE__URL│               │ DATABASE__URL│
│ =":memory:"  │               │ ="postgres:..│
└──────────────┘               └──────────────┘
       │                              │
       │ 相同的Repository Trait       │
       │ 相同的SQL(通过sqlx抽象)      │
       │                              │
       ▼                              ▼
┌────────────────────────────────────────────┐
│              无缝切换                       │
│  - 代码无需修改                             │
│  - 配置驱动切换                             │
│  - 迁移脚本统一                             │
└────────────────────────────────────────────┘
```

### 9.2 注意事项

1. **SQL兼容性**：SQLite 和 PostgreSQL 使用各自专用的迁移文件（`migrations/sqlite/` 和 `migrations/postgres/`），语法不同（如 `TEXT` vs `UUID`、`datetime('now')` vs `NOW()`）
2. **迁移脚本**：双轨迁移，启动时根据 `DATABASE__DATABASE_TYPE` 选择对应目录
3. **测试覆盖**：在SQLite和目标生产数据库上都运行测试
4. **性能差异**：生产环境需要针对目标数据库进行性能测试
