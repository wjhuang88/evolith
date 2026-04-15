# Evolith - 智能体开发服务平台

## 项目简介

Evolith 是一个面向智能体开发和运行的服务平台，提供周边支持能力。平台旨在为AI Agent开发者提供工具封装、技能管理和代码复用能力，加速智能体应用的开发和部署。

## 核心价值

- **工具复用**：通过MCP协议提供标准化的工具封装，一次开发多处使用
- **技能共享**：兼容Claude Skills格式的混合型技能系统，支持远程加载和执行
- **代码复用**：面向大模型的代码片段仓库，降低token消耗，提升开发效率
- **生态兼容**：遵循开放标准，与主流AI Agent框架无缝集成

## 功能模块

### 1. MCP Server工具封装（第一期）

提供基于Model Context Protocol的工具封装服务，支持：
- 工具注册与发现
- 统一的工具调用接口
- 权限管理和访问控制

### 2. 智能体技能系统（第一期）

混合型技能系统，特性包括：
- 兼容Claude Skills格式（SKILL.md）
- 服务端代码执行
- 技能版本管理
- 技能市场与分享

### 3. 代码片段仓库（第一期）

面向vibe coding的代码片段管理：
- Markdown + YAML格式描述
- 面向大模型的文档
- 分类和搜索
- 直接引用能力

### 4. 制品仓库（远期规划）

多语言依赖库管理：
- 预编译依赖库
- 面向大模型的API文档
- 版本管理

## 技术栈

| 层级 | 技术选型 |
|------|----------|
| 后端框架 | Rust + Actix-web |
| 前端框架 | Next.js |
| 数据库 | PostgreSQL |
| 缓存 | Redis |
| 对象存储 | MinIO / S3 |
| 容器化 | Docker |

## 文档索引

- [需求文档](./docs/requirements.md)
- [架构设计](./docs/architecture.md)
- [API合约](./docs/api-contract.md)
- [多租户设计](./docs/multi-tenant.md)
- [Skill格式规范](./docs/skill-format.md)
- [代码片段格式规范](./docs/snippet-format.md)
- [技术栈说明](./docs/tech-stack.md)
- [测试](./docs/testing.md)
- [计费](./docs/billing.md)
- [权限](./docs/permissions.md)
- [国际化](./docs/i18n.md)

## 环境要求

### 必需

| 依赖 | 最低版本 | 推荐版本 | 说明 |
|------|----------|----------|------|
| Rust | 1.75 | 1.82+ | `Cargo.toml` 中 `rust-version = "1.75"`，生产 Dockerfile 使用 1.82 |
| Node.js | 18 | 20 LTS | 前端 Dockerfile 使用 `node:20-alpine` |
| npm | 9+ | 10+ | 随 Node.js 安装 |
| Docker | 20.10+ | 24+ | 用于基础设施服务和沙箱执行 |
| Docker Compose | 2.0+ | 2.20+ | V2 插件模式（`docker compose`，非 `docker-compose`） |

### 基础设施服务（Docker 自动管理）

| 服务 | 镜像版本 | 用途 |
|------|----------|------|
| PostgreSQL | 16-alpine | 生产数据库（开发模式可用 SQLite 替代） |
| Redis | 7-alpine | 缓存 |
| MinIO | latest | 对象存储（S3 兼容） |
| Nginx | 1.27-alpine | 生产环境反向代理 |

### 沙箱运行时（技能执行）

| 运行时 | 镜像版本 |
|--------|----------|
| Python | 3.11-alpine |
| Node.js | 20-alpine |

### 开发模式（Lite）

Lite 模式（`./scripts/dev.sh lite`）无需 Docker，使用 SQLite 内存数据库，仅需 Rust 和 Node.js。

## 快速开始

```bash
# 克隆项目
git clone https://github.com/your-org/evolith.git

# 完整模式（需要 Docker）
./scripts/dev.sh start

# 轻量模式（仅需 Rust + Node.js，SQLite 内存数据库）
./scripts/dev.sh lite

# 查看状态
./scripts/dev.sh status

# 停止服务
./scripts/dev.sh stop
```

## 许可证

MIT License
