# Evolith — AI-Native Git Platform

> AI 原生 Git 平台 | Where AI Ships Code

Evolith 是面向 AI 时代的开发平台，将 Git 仓库托管、AI 智能体协作和 Agent 技能发布融为一体。不是在传统平台上加 AI，而是为 AI Agent 从零构建。

## 为什么选择 Evolith？

- **AI 原生架构**：为 AI Agent 协作从零设计，不是事后补丁
- **Git 兼容**：标准 Git 协议，现有仓库零迁移成本接入
- **Vibe Coding**：AI 智能体直接在仓库中编码、审查和提交
- **技能生态**：从代码仓库自动索引 Skill / MCP 工具 / CLI 接口，Agent 自动发现
- **企业级安全**：RBAC 权限、审计日志、多租户隔离、CSRF 防护

## 核心能力

### Git 仓库托管

AI 原生的 Git 托管服务，每个仓库开箱即用：
- 标准 Git 协议，支持 clone / push / pull
- 仓库级策略文件（`.evolith/policy.yaml`）控制 Agent 权限
- 直推直合工作流，适配 AI 协作场景
- 可见性控制（public / private）

### Vibe Coding（在线 AI 编程）

与 AI 智能体一起编码、审查和交付：
- Agent 直接向仓库提交代码，无需复制粘贴
- 基于仓库上下文的智能代码生成
- 在线代码编辑器，支持实时预览
- 提交策略控制（auto-merge / require-review / block）

### Agent 技能中心

从任意仓库自动索引和发布 AI Agent 能力：
- Skill（`.evolith/SKILL.md`）— Claude Skills 兼容格式
- MCP 工具（`.evolith/tool.yaml`）— Model Context Protocol 标准
- CLI 接口（`.evolith/interface.yaml`）— 面向大模型的命令描述
- Agent 自动发现并使用——就像 AI 领域的 npm

## 技术栈

| 层级 | 技术选型 |
|------|----------|
| 后端框架 | Rust + Actix-web |
| 前端框架 | React + Vite + Bun 静态 SPA |
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
| Node.js | 18 | 20 LTS | Vite/React 工具链运行时 |
| Bun | 1.3 | 1.3.14+ | 前端包管理、锁文件和脚本执行 |
| Docker | 20.10+ | 24+ | 可选，用于完整模式的基础设施服务 |
| Docker Compose | 2.0+ | 2.20+ | V2 插件模式（`docker compose`，非 `docker-compose`） |

### 基础设施服务（Docker 自动管理）

| 服务 | 镜像版本 | 用途 |
|------|----------|------|
| PostgreSQL | 16-alpine | 生产数据库（开发模式可用 SQLite 替代） |
| Redis | 7-alpine | 缓存 |
| MinIO | latest | 对象存储（S3 兼容） |
| Nginx | 1.27-alpine | 生产环境反向代理 |

### 开发模式（Lite）

Lite 模式（`./scripts/dev.sh lite`）无需 Docker，使用 SQLite 内存数据库，仅需 Rust 和 Bun。

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
