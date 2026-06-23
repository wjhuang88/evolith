# Agent Runtime 用户工作空间执行

> 状态：**整合进 [GIT-CENTRIC-PLATFORM](GIT-CENTRIC-PLATFORM.md) — Agent Session 抽象**（2026-06-23 方向调整）
> 原"远期想法"立场不变；本文档保留作为 agent 工作空间方向的背景参考。

## 动机

让用户能够在自己定义的工作空间中运行 Agent，提供隔离的执行环境和资源访问边界，而非仅在平台侧沙箱中运行。

## 调整后定位（2026-06-23）

原 Agent Runtime 设想（用户工作空间 + Agent 运行时 + 资源隔离）已被 [GIT-CENTRIC-PLATFORM](GIT-CENTRIC-PLATFORM.md) 的核心设计吸收：

- **工作空间 = git repo**：用户的 Agent 工作空间就是一个 git repo（Evolith 托管）。
- **Agent 执行环境**：外部 agent engine 提供（用户其他项目）；Evolith 不构建 sandbox。
- **资源隔离**：通过 RBAC + scoped token + repo ACL 实现。
- **工具挂载**：通过 `POST /repos/{id}/files` + policy.yaml 实现；agent 可调用的工具/MCP/CLI 由用户在 repo 内声明。
- **Agent Session 抽象**：[EVO-106 Agent Session API](../backlog/active/EVO-106-agent-session-and-scoped-token.md) + Scoped Token + audit log 已落地。

## 初步能力设想（保留作背景参考）

- 用户工作空间：每个用户/租户拥有独立的工作空间目录，存放代码、配置、凭据等
- Agent 运行时：Agent 在用户工作空间内执行，可访问工作空间内的文件和工具
- 资源隔离：工作空间之间相互隔离，Agent 无法越权访问其他用户的工作空间
- 工具挂载：用户可自定义工作空间内可用的 MCP 工具和 Skill

## 不做（当前阶段）

- 不做通用的 IDE 或编辑器（vibe coding 走 EVO-104 Web UI）
- 不做 Agent 的训练或模型托管（外部 agent engine 提供）
- 不做平台侧沙箱执行（按 ADR-0005 整层删除）

## 前置依赖

- Git-Centric Platform 落地（EVO-100 Phase E'-2 完成；EVO-106 + EVO-107 提供 session + webhook）

## 下一步

- 跟随 [GIT-CENTRIC-PLATFORM](GIT-CENTRIC-PLATFORM.md) Phase E'-2 推进；EVO-106 范围已包含 session/token/audit 基础。
- 多 agent 并行协作 / cross-repo session / workspace bundle 等扩展后续单独评估（独立 EVO）。
