# AI Gateway 模型代理与管理

> 状态：远期想法，尚未进入 backlog 排期。

## 动机

为 AI Agent 提供统一的模型调用入口，屏蔽底层模型差异，提供权限控制和用量管理。

## 初步能力设想

- **模型代理转发**：统一 API 接口，后端代理到 OpenAI / Anthropic / 本地模型等
- **权限控制**：按租户、角色、API Key 控制可访问的模型和调用频率
- **用量统计**：Token 消耗、请求次数、费用追踪
- **模型能力管理**：模型注册、能力声明（function calling / vision / streaming）、路由策略（fallback / load balance）

## 不做（当前阶段）

- 不替代专业 API Gateway 产品（如 Kong、APISIX）的通用流量管理能力
- 不做模型训练或微调

## 前置依赖

- 核心平台（MCP 工具、Skill、权限体系）稳定运行
- 计费系统（Stripe）闭环后，可复用用量追踪基础设施

## 下一步

- 待核心功能稳定后，展开技术方案评审
- 定义 AI Gateway 的 API 合约和数据模型
- 进入 `docs/backlog/PRODUCT-BACKLOG.md` 排期
