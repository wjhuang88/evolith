# Evolith - 企业级 AI Agent Harness 平台

## 项目简介

Evolith 的定位是构建企业级 AI Agent Harness 平台：为企业内部智能体提供可治理、可审计、可复用、可集成的工具、技能、CLI 友好接口和运行支撑能力。

平台不是单纯的代码片段仓库或演示型控制台，而是面向企业落地的 agent 能力承载层，重点解决工具接入、权限边界、执行安全、接口契约、团队复用和运维交付问题。

## 核心价值

- **工具治理**：通过 MCP 协议提供标准化工具封装、权限控制和审计边界
- **技能复用**：兼容 Claude Skills 格式的混合型技能系统，支持远程加载和沙箱执行
- **CLI 友好接口**：用机器可读 schema、usage、examples 和错误语义替代旧 snippet 主线
- **企业交付**：面向多租户、RBAC、审计、部署、运行时配置和后续 CLI 同步能力设计

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

### 3. CLI 友好接口仓库（第一期）

面向大模型和本地 CLI 调用的接口描述管理：
- 稳定命令、子命令和参数 schema
- 面向大模型的 usage、examples、error model
- JSON-friendly 输入输出契约
- 可由前端管理，也可被后续 Rust CLI push / pull / sync

### 4. 制品仓库（远期规划）

多语言依赖库管理：
- 预编译依赖库
- 面向大模型的API文档
- 版本管理

## 技术栈

| 层级 | 技术选型 |
|------|----------|
| 后端框架 | Rust + Actix-web |
| 前端框架 | 当前 Next.js；P0 迁移目标为 React + Vite + Bun 静态 SPA |
| 数据库 | PostgreSQL |
| 缓存 | Redis |
| 对象存储 | MinIO / S3 |
| 容器化 | Docker |

## 文档索引

- [文档地图](./docs/README.md)
- [项目地图](./docs/reference/PROJECT-MAP.md)
- [本地开发 SOP](./docs/sop/LOCAL-DEV.md)
- [新增功能 SOP](./docs/sop/NEW-FEATURE.md)
- [发布与部署 SOP](./docs/sop/RELEASE.md)
- [Git 工作流 SOP](./docs/sop/GIT-WORKFLOW.md)
- [需求文档](./docs/reference/product/REQUIREMENTS.md)
- [架构设计](./docs/reference/ARCHITECTURE.md)
- [API合约](./docs/reference/API-CONTRACT.md)
- [多租户设计](./docs/reference/MULTI-TENANT.md)
- [Skill格式规范](./docs/reference/formats/SKILL-FORMAT.md)
- [CLI 友好接口格式规范](./docs/reference/formats/CLI-INTERFACE-FORMAT.md)
- [代码片段格式规范](./docs/reference/formats/SNIPPET-FORMAT.md)（legacy 迁移参考）
- [技术栈说明](./docs/reference/TECH-STACK.md)
- [测试](./docs/reference/TESTING.md)
- [计费](./docs/reference/BILLING.md)
- [权限](./docs/reference/PERMISSIONS.md)
- [国际化](./docs/reference/I18N.md)
- [工程化路线图](./docs/roadmap/ENGINEERING-ROADMAP.md)
- [经验积累](./EVOLUTION.md)

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
