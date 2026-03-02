# 需求文档

## 1. 项目概述

### 1.1 项目背景

随着AI Agent技术的快速发展，开发者需要大量重复性的工具封装、技能定义和代码复用工作。现有方案存在以下问题：
- 工具封装缺乏统一标准，难以跨平台复用
- 技能定义格式各异，无法形成共享生态
- 代码片段缺乏面向大模型的优化，导致token浪费

### 1.2 项目目标

构建一个统一的智能体开发服务平台，提供：
1. 标准化的工具封装和分发能力
2. 兼容主流生态的技能管理系统
3. 面向大模型优化的代码片段仓库

### 1.3 目标用户

| 用户类型 | 描述 | 核心需求 |
|----------|------|----------|
| Agent开发者 | 开发AI Agent应用的工程师 | 工具复用、技能共享 |
| Vibe Coder | 使用AI辅助编程的开发者 | 代码片段、快速集成 |
| 技能创作者 | 编写可复用技能的专家 | 技能发布、版本管理 |
| 企业用户 | 需要私有化部署的组织 | 权限控制、审计日志 |

## 2. 功能需求

### 2.1 MCP Server工具封装

#### 2.1.1 功能描述

提供基于Model Context Protocol的工具封装服务，允许开发者注册、发现和调用远程工具。

#### 2.1.2 功能列表

| 功能ID | 功能名称 | 优先级 | 描述 |
|--------|----------|--------|------|
| F1.1.1 | 工具注册 | P0 | 支持通过API注册新工具 |
| F1.1.2 | 工具发现 | P0 | 支持工具列表查询和搜索 |
| F1.1.3 | 工具调用 | P0 | 支持通过MCP协议调用工具 |
| F1.1.4 | 参数验证 | P0 | 基于JSON Schema的参数校验 |
| F1.1.5 | 权限控制 | P1 | 工具访问权限管理 |
| F1.1.6 | 调用日志 | P1 | 工具调用记录和统计 |
| F1.1.7 | 限流控制 | P2 | API调用频率限制 |

#### 2.1.3 接口规范

```typescript
// 工具注册
POST /api/v1/tools
{
  "name": "weather-query",
  "description": "Query weather information",
  "inputSchema": { /* JSON Schema */ },
  "handler": { /* 执行配置 */ }
}

// 工具发现
GET /api/v1/tools?category=utility&search=weather

// 工具调用（MCP协议）
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "weather-query",
    "arguments": { "city": "Beijing" }
  }
}
```

### 2.2 智能体技能系统

#### 2.2.1 功能描述

提供混合型技能系统，兼容Claude Skills格式，支持SKILL.md指令和服务端代码执行。

#### 2.2.2 技能类型

| 类型 | 描述 | 执行方式 |
|------|------|----------|
| 纯指令型 | 仅包含提示指令 | 客户端注入 |
| 混合型 | 指令 + 可执行代码 | 服务端执行 |

#### 2.2.3 功能列表

| 功能ID | 功能名称 | 优先级 | 描述 |
|--------|----------|--------|------|
| F1.2.1 | 技能创建 | P0 | 创建SKILL.md格式的技能 |
| F1.2.2 | 技能上传 | P0 | 上传技能包（含代码） |
| F1.2.3 | 技能发现 | P0 | 搜索和浏览技能 |
| F1.2.4 | 技能加载 | P0 | 远程加载技能到Agent |
| F1.2.5 | 代码执行 | P0 | 服务端安全执行技能代码 |
| F1.2.6 | 版本管理 | P1 | 技能版本控制和回滚 |
| F1.2.7 | 技能市场 | P1 | 技能发布和共享 |
| F1.2.8 | 依赖管理 | P2 | 技能间依赖关系 |

#### 2.2.4 技能格式规范

```markdown
---
name: data-analyzer
description: Analyze data and generate reports
version: 1.0.0
author: evolith
tags: [data, analysis, report]
execution: server  # 客户端注入 or 服务端执行
entrypoint: analyze.py
runtime: python3.11
dependencies:
  - pandas>=2.0
  - numpy>=1.24
---

# Data Analyzer Skill

This skill analyzes CSV/JSON data and generates insights.

## Usage

Invoke with data file path:
```
/data-analyzer /path/to/data.csv
```

## What it does

1. Load and validate data
2. Generate statistical summary
3. Create visualizations
4. Output insights report
```

### 2.3 代码片段仓库

#### 2.3.1 功能描述

提供面向大模型优化的代码片段管理，支持分类、搜索和直接引用。

#### 2.3.2 功能列表

| 功能ID | 功能名称 | 优先级 | 描述 |
|--------|----------|--------|------|
| F1.3.1 | 片段创建 | P0 | 创建代码片段 |
| F1.3.2 | 片段分类 | P0 | 按语言/框架/用途分类 |
| F1.3.3 | 片段搜索 | P0 | 关键词和语义搜索 |
| F1.3.4 | 片段引用 | P0 | 生成可直接引用的代码 |
| F1.3.5 | Token预估 | P1 | 显示片段的token消耗 |
| F1.3.6 | 依赖声明 | P1 | 声明片段的外部依赖 |
| F1.3.7 | 版本管理 | P2 | 片段版本控制 |

#### 2.3.3 片段格式规范

```markdown
---
id: react-use-debounce
name: useDebounce Hook
language: typescript
framework: react
tags: [hooks, debounce, performance]
dependencies:
  - react@^18.0.0
estimated_tokens: 150
created_at: 2024-01-15
updated_at: 2024-01-15
---

# useDebounce Hook

A React hook for debouncing values. Useful for search inputs,
API calls, and any scenario where you want to delay processing.

## Installation

```bash
npm install react
```

## Usage

```tsx
import { useDebounce } from './useDebounce';

function SearchComponent() {
  const [searchTerm, setSearchTerm] = useState('');
  const debouncedSearch = useDebounce(searchTerm, 300);
  
  useEffect(() => {
    // API call with debounced value
    searchAPI(debouncedSearch);
  }, [debouncedSearch]);
}
```

## Code

```tsx
import { useState, useEffect } from 'react';

export function useDebounce<T>(value: T, delay: number): T {
  const [debouncedValue, setDebouncedValue] = useState<T>(value);

  useEffect(() => {
    const timer = setTimeout(() => {
      setDebouncedValue(value);
    }, delay);

    return () => {
      clearTimeout(timer);
    };
  }, [value, delay]);

  return debouncedValue;
}
```

## API Reference

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| value | T | - | The value to debounce |
| delay | number | - | Delay in milliseconds |

## Returns

The debounced value that updates after the specified delay.
```

### 2.4 制品仓库（远期规划）

#### 2.4.1 功能描述

提供多语言预编译依赖库管理，面向大模型优化文档。

#### 2.4.2 功能列表（远期）

| 功能ID | 功能名称 | 描述 |
|--------|----------|------|
| F1.4.1 | 包管理 | 支持npm/crate/pypi包 |
| F1.4.2 | 预编译 | 提供编译好的二进制 |
| F1.4.3 | LLM文档 | 面向大模型的API文档 |
| F1.4.4 | 版本管理 | 语义化版本控制 |
### 2.5 多租户支持

#### 2.5.1 功能描述

作为 SaaS 平台，Evolith 需要支持多租户架构，多个组织/团队可以独立使用同一套系统，数据相互隔离。

#### 2.5.2 功能列表

| 功能ID | 功能名称 | 优先级 | 描述 |
|--------|----------|--------|------|
| F2.5.1 | 租户创建 | P0 | 注册时创建租户 |
| F2.5.2 | 租户识别 | P0 | 通过子域名/请求头识别租户 |
| F2.5.3 | 数据隔离 | P0 | 基于 tenant_id 的行级隔离 |
| F2.5.4 | 租户配额 | P1 | 基于计划的用户数/资源限制 |
| F2.5.5 | 成员管理 | P1 | 租户内用户邀请/移除 |
| F2.5.6 | 租户设置 | P1 | 租户级配置管理 |

#### 2.5.3 详细设计

见 [多租户设计](./multi-tenant.md) 文档。

### 2.6 国际化 (i18n)

#### 2.6.1 功能描述

Evolith 作为全球化 SaaS 平台，需要支持多语言界面，满足不同地区用户需求。

#### 2.6.2 功能列表

| 功能ID | 功能名称 | 优先级 | 描述 |
|--------|----------|--------|------|
| F2.6.1 | 中文界面 | P0 | 默认中文界面 |
| F2.6.2 | 英文界面 | P0 | 英文界面支持 |
| F2.6.3 | 语言切换 | P0 | 用户可切换语言 |
| F2.6.4 | 语言检测 | P1 | 自动检测用户语言 |
| F2.6.5 | 日期格式 | P1 | 按语言本地化日期 |

#### 2.6.3 技术实现

- 框架：react-i18next
- 翻译文件：locales/{lang}.json
- 语言：zh-CN（默认）、en
详细设计见 [国际化设计](./i18n.md) 文档。

### 2.7 用户与权限系统

#### 2.7.1 功能描述

Evolith 作为 SaaS 平台，需要完整的用户和权限系统来管理租户成员、角色权限和 API Key。

#### 2.7.2 功能列表

| 功能ID | 功能名称 | 优先级 | 描述 |
|--------|----------|--------|------|
| F2.7.1 | 用户注册 | P0 | 邮箱注册，自动创建租户 |
| F2.7.2 | 用户登录 | P0 | JWT token 认证 |
| F2.7.3 | 角色管理 | P0 | owner/admin/member/viewer |
| F2.7.4 | 成员邀请 | P1 | 邀请成员加入租户 |
| F2.7.5 | 成员移除 | P1 | 移除租户成员 |
| F2.7.6 | 角色变更 | P1 | 变更成员角色 |
| F2.7.7 | 密码重置 | P1 | 邮箱验证重置 |
| F2.7.8 | API Key | P0 | 创建/管理 API Key |
| F2.7.9 | API Key 使用 | P0 | 替代 JWT 的认证方式 |
| F2.7.10 | 审计日志 | P1 | 记录敏感操作 |

详细设计见 [用户与权限](./permissions.md) 文档。

### 2.8 套餐与计费

#### 2.8.1 功能描述

Evolith 作为付费 SaaS 平台，需要完整的套餐管理和计费系统。

#### 2.8.2 功能列表

| 功能ID | 功能名称 | 优先级 | 描述 |
|--------|----------|--------|------|
| F2.8.1 | 套餐定义 | P0 | Free/Starter/Pro/Enterprise |
| F2.8.2 | 套餐订阅 | P0 | 选择并订阅套餐 |
| F2.8.3 | 套餐升级/降级 | P0 | 变更订阅套餐 |
| F2.8.4 | 支付集成 | P1 | Stripe/支付宝/微信支付 |
| F2.8.5 | 发票管理 | P1 | 生成和管理发票 |
| F2.8.6 | 计费周期 | P0 | 月付/年付 |
| F2.8.7 | 取消订阅 | P1 | 用户取消订阅 |
| F2.8.8 | 使用量统计 | P0 | 实时使用量查询 |
| F2.8.9 | 配额限制 | P0 | 套餐配额检查 |
| F2.8.10 | 超额计费 | P1 | API 调用超配额计费 |

详细设计见 [套餐与计费](./billing.md) 文档。

## 3. 非功能需求


## 3. 非功能需求

### 3.1 性能要求

| 指标 | 要求 | 说明 |
|------|------|------|
| API响应时间 | < 100ms (P95) | 非计算密集型接口 |
| 工具调用延迟 | < 500ms (P95) | 含执行时间 |
| 并发用户数 | 1000+ | 单实例 |
| 代码执行超时 | 30s (默认) | 可配置 |

### 3.2 安全要求

| 要求 | 描述 |
|------|------|
| 代码沙箱 | 技能代码在隔离环境执行 |
| 权限控制 | 基于RBAC的访问控制 |
| 输入验证 | 所有输入进行严格校验 |
| 审计日志 | 记录所有敏感操作 |
| 传输加密 | HTTPS/TLS |

### 3.3 可维护性要求

| 要求 | 描述 |
|------|------|
| 模块化 | 清晰的模块边界和接口 |
| 文档完整 | API文档、架构文档、开发指南 |
| 测试覆盖 | 单元测试覆盖率 > 80% |
| 监控告警 | 完善的监控和告警体系 |
| 日志规范 | 统一的日志格式和级别 |

### 3.4 用户体验要求

| 要求 | 描述 |
|------|------|
| 界面美观 | 现代化UI设计，简洁有设计感 |
| 响应式 | 支持桌面和移动端 |
| 加载速度 | 首屏加载 < 2s |
| 错误提示 | 友好的错误信息和引导 |
| 暗色模式 | 支持明暗主题切换 |

## 4. 约束条件

### 4.1 技术约束

- 后端必须使用 Rust + Actix-web
- 前端必须使用 Next.js
- 采用前后端分离架构
- 支持Docker容器化部署

### 4.2 兼容性约束

- 技能格式兼容Claude Skills (SKILL.md)
- 工具协议兼容MCP (Model Context Protocol)
- 支持主流浏览器 (Chrome, Firefox, Safari, Edge)

### 4.3 时间约束

- 第一期功能开发周期：8周
- MVP发布时间：第6周
- 正式发布时间：第8周

## 5. 验收标准

### 5.1 功能验收

- [ ] 所有P0功能正常工作
- [ ] 通过功能测试用例
- [ ] 无阻塞性Bug

### 5.2 性能验收

- [ ] 满足性能指标要求
- [ ] 通过压力测试

### 5.3 安全验收

- [ ] 通过安全扫描
- [ ] 无高危漏洞

### 5.4 文档验收

- [ ] API文档完整
- [ ] 部署文档完整
- [ ] 用户手册完整
