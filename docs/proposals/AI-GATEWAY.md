# AI Gateway 模型代理与管理

> 状态：**整合进 [GIT-CENTRIC-PLATFORM](GIT-CENTRIC-PLATFORM.md) — Agent Bridge 层**（2026-06-23 方向调整）
> 原"远期想法"立场不变；本文档保留作为 Agent 与 LLM 集成方向的背景参考。

## 动机

为 AI Agent 提供统一的模型调用入口，屏蔽底层模型差异，提供权限控制和用量管理。

## 调整后定位（2026-06-23）

原 AI Gateway 设想（OpenAI / Anthropic 代理 + 用量统计 + 路由策略）范围收窄为 Agent Bridge 层的**权限 / 配额 / 计费**组件：

- **Agent Bridge 层**（整合进 GIT-CENTRIC-PLATFORM）：[EVO-106 Agent Session API](../backlog/active/EVO-106-agent-session-and-scoped-token.md) + Scoped Token + session 级 audit log + 用量统计钩子。
- **模型代理转发**：不在本提案主路径。Agent engine 是外部项目；Evolith 不直接代理 LLM API。
- **权限控制 / 用量统计**：作为 Agent Session 的 metadata 字段；独立 EVO 评估（不在 EVO-106 MVP 范围）。

## 初步能力设想（保留作背景参考）

- 模型代理转发：统一 API 接口，后端代理到 OpenAI / Anthropic / 本地模型等
- 权限控制：按租户、角色、API Key 控制可访问的模型和调用频率
- 用量统计：Token 消耗、请求次数、费用追踪
- 模型能力管理：模型注册、能力声明（function calling / vision / streaming）、路由策略（fallback / load balance）

## 不做（当前阶段）

- 不替代专业 API Gateway 产品（如 Kong、APISEX）的通用流量管理能力
- 不做模型训练或微调
- 不直接代理 LLM API（外部 agent engine 处理）

## 前置依赖

- Git-Centric Platform 落地（EVO-100 Phase E'-1 + E'-2 + E'-3 完成）
- 计费系统（Stripe）闭环后，可复用用量追踪基础设施

## 下一步

- 跟随 [GIT-CENTRIC-PLATFORM](GIT-CENTRIC-PLATFORM.md) Phase E' 推进；EVO-106 范围已包含 session/token/audit 基础。
- 独立的 LLM API 代理 / 用量计费组件后续单独评估（独立 EVO）。
