# EVO-118-G 运行可靠性与发布门禁接线

- **类型**：Technical / Reliability / Release
- **状态**：Proposed
- **优先级**：P1
- **父 Epic**：[EVO-118](EVO-118-production-readiness-and-security-hardening.md)
- **依赖**：EVO-118-B、EVO-118-E
- **影响范围**：backend / CI / deploy / config / docs / tests

## 工程目标

让已有配置和健康能力真正进入运行链路，避免“配置字段存在、测试函数存在，但生产门禁没有接线”的假完整状态。

## 已确认失败模式

- GitHub CI 只在语义版本 Tag Push 时运行，PR/Main 缺少自动门禁。
- 主服务只挂载匿名 IP limiter，Authenticated/API Key limiter 与单 Key `rate_limit` 未接线。
- readiness 数据库失败时仍返回 HTTP 200，Compose 使用永远 healthy 的 `/health`。
- SMTP 启用但初始化失败时静默降级 ConsoleMailer；敏感链接可能进入日志。
- Redis 降级语义没有区分普通缓存与安全/Session 状态。

## 验收场景

### Scenario 1：PR 不能绕过质量门禁

- **Given** PR 修改 Backend 或 Frontend
- **When** 推送分支或更新 PR
- **Then** fmt/check/clippy/test/type-check/build/lint 自动运行，失败阻止合并

### Scenario 2：Readiness 正确反映依赖

- **Given** 数据库或 Git Storage 不可用
- **When** 调用 `/health/ready`
- **Then** 返回 503 和明确检查结果；恢复后返回 200

### Scenario 3：调用类型限流生效

- **Given** 匿名、JWT、API Key 和不同 Key rate limit
- **When** 超过各自配额
- **Then** 按调用者维度返回 429，不因共享 IP 错误互相影响

### Scenario 4：生产关键依赖 fail closed

- **Given** SMTP 声明启用但配置无效，或承担安全状态的 Redis 不可用
- **When** 服务启动/执行相关流程
- **Then** 启动失败或功能明确不可用，不静默切换为假成功

## 工程要求

- CI 增加 `pull_request` 和 `push main`；Tag 继续承载 release 验证。
- 设置或记录 main Branch Protection 所需检查。
- 区分 liveness/readiness；readiness 检查 DB、Git Storage 和适用关键依赖。
- 将 Authenticated/API Key limiter 与每 Key 计数接入请求链。
- 明确 Redis fallback：普通缓存可降级；Session、rate limit、锁等安全状态不得进程内假分布式。
- 生产 SMTP 初始化失败 fail closed；开发 ConsoleMailer 不记录完整 Token/链接。
- 配置文档、监控和故障响应同步。

## 不做事项

- 不建设完整可观测平台或多区域 SRE 体系。
- 不把所有外部服务都变成启动硬依赖；按事实源/安全状态分类。
- Durable Outbox 归 EVO-118-H。

## 最小验证

- PR workflow 触发与失败检查证据。
- DB/Git Storage 故障下 readiness 503。
- 匿名/JWT/API Key/单 Key 限流集成测试。
- SMTP production 配置失败启动测试与日志脱敏检查。
- Redis 普通缓存降级和安全状态 fail-closed 测试。
- clean production smoke test。

## 解锁内容

解除 REL-01 Gate；让生产发布、回滚和后续 Agent/Indexer 能依赖真实运行门禁。
