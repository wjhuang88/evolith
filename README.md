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

## 快速开始

```bash
# 克隆项目
git clone https://github.com/your-org/evolith.git

# 启动开发环境
docker-compose up -d

# 安装依赖
cd frontend && npm install
cd ../backend && cargo build

# 启动服务
./scripts/dev.sh
```

## 许可证

MIT License
