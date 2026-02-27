# 开发计划 - 已完成

> 最后更新: 2026-02-27

## 完成记录

### 2026-02-27

#### 文档完成

- [x] **DOC-001** 项目概述文档
  - 创建 `docs/README.md`
  - 项目简介、核心价值、功能模块概述

- [x] **DOC-002** 需求文档
  - 创建 `docs/requirements.md`
  - 功能需求详细说明
  - 非功能需求
  - 验收标准

- [x] **DOC-003** 架构设计文档
  - 创建 `docs/architecture.md`
  - 整体架构图
  - 模块划分
  - 核心流程
  - 数据模型
  - 安全设计
  - 部署架构

- [x] **DOC-004** API设计文档
  - 创建 `docs/api-design.md`
  - RESTful API规范
  - MCP协议端点
  - 请求/响应格式
  - 错误码定义

- [x] **DOC-005** Skill格式规范
  - 创建 `docs/skill-format.md`
  - SKILL.md格式定义
  - 代码规范
  - 打包格式

- [x] **DOC-006** 代码片段格式规范
  - 创建 `docs/snippet-format.md`
  - Markdown + YAML格式
  - 分类体系
  - 引用格式

- [x] **DOC-007** 技术栈说明
  - 创建 `docs/tech-stack.md`
  - 后端技术栈
  - 前端技术栈
  - 数据库抽象层设计（SQLite开发/PostgreSQL生产无缝切换）
  - 开发/生产环境配置

- [x] **DOC-008** 开发计划
  - 创建 `plans/todo.md`
  - 创建 `plans/done.md`
  - 里程碑规划
  - 任务分解

---

## 设计决策记录

### DDR-001: 技能类型选择

**日期**: 2026-02-27

**决策**: 采用混合型技能系统

**理由**:
- 纯指令型（Claude兼容）+ 服务端代码执行
- 兼顾生态兼容性和功能扩展性
- 代码在服务端执行，安全性更好

**影响**:
- 需要实现代码执行沙箱
- 需要支持多运行时（Python、Node.js、WASM）

---

### DDR-002: 数据库架构

**日期**: 2026-02-27

**决策**: 采用数据库抽象层，支持开发/生产环境无缝切换

**理由**:
- 开发时使用SQLite内存数据库，快速迭代
- 生产环境使用PostgreSQL/MySQL，稳定可靠
- 通过Repository Trait抽象，代码无需修改

**实现**:
```rust
trait Repository { ... }

impl Repository for SqliteRepository { ... }  // 开发
impl Repository for PostgresRepository { ... } // 生产
```

**配置切换**:
```bash
# 开发
DATABASE_TYPE=sqlite DATABASE_URL=":memory:"

# 生产
DATABASE_TYPE=postgres DATABASE_URL="postgres://..."
```

**影响**:
- 需要维护多数据库适配器
- 迁移脚本需要兼容多数据库
- 测试需要在两种数据库上运行

---

### DDR-003: 代码片段Token预估

**日期**: 2026-02-27

**决策**: 在片段元数据中包含estimated_tokens字段

**理由**:
- 帮助用户预估token消耗
- 支持按token大小筛选
- 优化LLM上下文使用

**计算规则**:
```
estimated_tokens = code_tokens + (doc_tokens * 0.5) + (example_tokens * 0.3)
```

---

### DDR-004: 制品仓库延期

**日期**: 2026-02-27

**决策**: 制品仓库作为远期目标，不在第一期实现

**理由**:
- 降低第一期复杂度
- 聚焦核心功能
- 待用户反馈后再优化

**远期规划**:
- 多语言包管理
- 预编译依赖库
- 面向大模型的API文档

---

## 风险记录

### RISK-001: 代码执行安全

**等级**: 高

**描述**: 技能代码执行存在安全风险

**缓解措施**:
- 使用容器隔离
- 资源限制（CPU、内存、时间）
- 禁止网络访问（可配置）
- 文件系统只读

**状态**: 待实现

---

### RISK-002: 数据库兼容性

**等级**: 中

**描述**: SQLite和PostgreSQL的SQL语法差异可能导致问题

**缓解措施**:
- 使用sqlx的参数化查询
- 避免数据库特定语法
- 在两种数据库上运行测试

**状态**: 待验证

---

## 依赖清单

### 后端依赖

| 依赖 | 版本 | 用途 |
|------|------|------|
| actix-web | 4.x | Web框架 |
| sqlx | 0.7 | 数据库 |
| tokio | 1.x | 异步运行时 |
| serde | 1.x | 序列化 |
| tracing | 0.1 | 日志 |
| jsonwebtoken | 9.x | JWT |
| uuid | 1.x | UUID |
| chrono | 0.4 | 时间 |

### 前端依赖

| 依赖 | 版本 | 用途 |
|------|------|------|
| next | 14.x | 框架 |
| react | 18.x | UI库 |
| typescript | 5.x | 语言 |
| tailwindcss | 3.x | 样式 |
| zustand | 4.x | 状态管理 |
| @tanstack/query | 5.x | 数据获取 |

---

## 下一步行动

1. **开始Phase 1**: 核心框架
   - 实现配置管理
   - 实现错误处理
   - 实现日志系统
   - 实现认证中间件
   - 完善UI组件库

2. **技术验证**
   - 验证Rust后端编译
   - 验证Next.js前端构建
   - 验证数据库连接

3. **功能开发**
   - 开始MCP工具系统开发
   - 开始技能系统开发
   - 开始代码片段系统开发
