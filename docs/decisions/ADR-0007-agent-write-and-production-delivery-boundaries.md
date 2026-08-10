# ADR-0007: Agent 写入授权与生产交付边界

## 状态

Accepted（2026-08-07）

## 背景

Evolith 后续的 Commit/Promote、Agent Session 和 Vibe Coding 依赖两个尚未固化的边界：

1. `.evolith/policy.yaml` 如何把默认动作、受保护路径和 Agent scope 组合成可审计的写入结果。
2. Agent Session 凭证如何实现单 Repo 绑定、过期、撤销和最小权限。

生产交付还存在 Embedded Frontend 与独立 Frontend 容器的双口径，需要确定唯一事实交付路径。

## 决策

### 1. PolicyEvaluator 采用 fail-closed 优先级

授权检查和策略评估按以下顺序执行：

1. 身份、Tenant、Repo 和 typed capability 校验；失败返回 401/403，Ref 不得移动。
2. Agent path scope 校验；不在授权路径内直接拒绝（403），不降级为 `require_review`。
3. `block` 是显式拒绝，优先级最高。
4. `protected_paths` 命中强制 `require_review`，即使 `default_action=auto_merge`。
5. 未命中保护规则时才使用 `default_action`（`auto_merge`、`require_review` 或 `block`）。

`commit:<scope>` 不再作为通用写权限；实现必须把 scope 解析为结构化的 Repo/Branch/Path 限制，并通过 PolicyEvaluator 检查。

### 2. Agent Token 使用独立、可撤销的 opaque credential

- 新增 `scoped_tokens` 领域/存储边界，不复用用户 API Key 的管理语义。
- 数据库存储 token hash，不存明文；创建响应只返回一次明文 token。
- Token 绑定一个 `tenant_id`、`repo_id` 和 `agent_session_id`，默认 TTL 24 小时。
- Token 只携带 typed capabilities，例如 `read_repo`、`create_branch`、`commit`、`promote`；禁止 `admin`、`force_push`、`delete_branch`。
- Session 进入 `completed`、`failed` 或 `expired` 后立即撤销；显式撤销必须在所有入口生效。
- Agent 请求使用 `X-Agent-Session`，服务端每次检查 token hash、状态、有效期、Tenant、Repo、Action、Branch/Path scope。

选择 opaque credential 是因为 Agent Token 需要可靠撤销和状态审计；JWT 不作为该凭证的默认载体。

### 3. 生产采用单一 Embedded Frontend 交付

- 根目录 multi-stage build 先构建 Bun/Vite 前端，再编译并嵌入 Rust 后端。
- 生产默认只部署包含前端静态资源的后端应用镜像。
- Nginx 只作为可选 Gateway/SSL/反向代理，不再作为第二个静态前端事实源。
- `/api/v1`、Git Smart HTTP、`/health`、`/assets` 和 SPA fallback 必须由同一交付路径验证。

## 安全边界

| 项目 | 约束 |
|------|------|
| 受保护资产 | Agent Token、Git refs/objects、Tenant/Repo 数据、审计事件 |
| 调用者 | Owner/Admin 用户、Member、API Key、Agent Token、外部 Agent Engine |
| 入口 | Commit/Promote API、Agent Session API、Git 写入路径、Webhook/Event Worker |
| 默认失败行为 | fail closed；拒绝、撤销或进入可重试状态，不返回假成功 |
| 最小负向测试 | 跨租户、过期/撤销 token、无 scope 写入、protected path、force push 和 Ref 不移动 |

## 后果

- EVO-105 可以在不引入模糊字符串权限的前提下实现三态策略。
- EVO-106 需要独立 migration/repository，并增加 token 撤销和 scope 负向测试。
- Webhook/Indexer 必须消费结构化 commit/promote 结果和 Durable Outbox 事件。
- EVO-118-E 的生产 Compose、Dockerfile 和 Smoke Test 以单一 Embedded Frontend 为唯一默认口径。
- 未来若引入 JWT Agent Token 或多 Repo Session，必须另开 ADR，不得悄然改变本决策。

## 相关链接

- [EVO-105 Commit/Promote API](../backlog/active/EVO-105-commit-and-promote-api.md)
- [EVO-106 Agent Session/Scoped Token](../backlog/active/EVO-106-agent-session-and-scoped-token.md)
- [EVO-118-E Production Build](../backlog/active/EVO-118-E-production-build-deployment-convergence.md)
- [Security Review SOP](../sop/SECURITY-REVIEW.md)
